//! Deterministic normalized signature artifacts and comparison.
//!
//! The signature gate deliberately has three independently produced inputs:
//! the semantic model, the emitted WIT, and the native/Lua source extractors.
//! Keeping the source representations in the artifact makes the CI result
//! inspectable without turning the artifact into a second copy of `model.json`.

use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    process::Command,
};

use crate::{
    model::{ApiModel, Environment, FunctionModel, LoweringStatus, SemanticType},
    render_wit,
};

/// A compact, transport-facing field signature.  Types are WIT spellings,
/// which gives the model and the parsed WIT a common semantic vocabulary
/// without serializing the complete model graph twice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignatureField {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelSignature {
    pub module: String,
    pub name: String,
    pub inputs: Vec<SignatureField>,
    pub result: String,
    pub environments: BTreeSet<Environment>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WitSignature {
    pub interface: String,
    pub name: String,
    pub inputs: Vec<SignatureField>,
    pub result: String,
    pub environments: BTreeSet<Environment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeParameter {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeSignature {
    pub module: String,
    pub name: String,
    pub params: Vec<NativeParameter>,
    pub return_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuaParameter {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuaFunction {
    pub name: String,
    pub params: Vec<LuaParameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuaMatch {
    pub lua_name: String,
    pub native_module: Option<String>,
    pub native_name: Option<String>,
    pub params_match: bool,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuaSignatures {
    pub documented: Vec<LuaFunction>,
    pub registered: Vec<String>,
    pub matches: Vec<LuaMatch>,
    pub unmatched: Vec<String>,
    pub explicit_exclusions: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SourceSignatures {
    native: Vec<NativeSignature>,
    lua: LuaSignatures,
}

/// The checked signature artifact.  `model` is a normalized function list,
/// not an embedded `ApiModel`; `wit`, `native`, and `lua` are independently
/// extracted source representations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureArtifact {
    pub format: String,
    pub model: Vec<ModelSignature>,
    pub wit: Vec<WitSignature>,
    pub native: Vec<NativeSignature>,
    pub lua: LuaSignatures,
}

/// Kept for callers of the old model-to-model helper.  It is intentionally no
/// longer the artifact written by the generator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacySignatureArtifact {
    pub format: String,
    pub model: ApiModel,
}

pub fn artifact(model: ApiModel) -> LegacySignatureArtifact {
    LegacySignatureArtifact {
        format: "recoil.spring.semantic-signature.v1".to_string(),
        model,
    }
}

/// Parse the emitted WIT and native Rust, combine them with the Lua extractor,
/// and fail generation on any independent representation drift.
pub fn round_trip(model: &ApiModel, root: &Path, output: &Path) -> Result<SignatureArtifact> {
    let model_signatures = model_signatures(model);
    let wit_signatures = parse_wit_signatures(model, &output.join("wit"))?;
    let source = extract_native_and_lua(root)?;
    let mut errors = Vec::new();

    compare_wit(model, &model_signatures, &wit_signatures, &mut errors);
    compare_native(model, &source.native, &mut errors);
    compare_lua(model, &source.lua, &mut errors);

    if !errors.is_empty() {
        bail!(
            "independent API signature gate failed:\n{}",
            errors.join("\n")
        );
    }

    Ok(SignatureArtifact {
        format: "recoil.spring.semantic-signature.v2".to_string(),
        model: model_signatures,
        wit: wit_signatures,
        native: source.native,
        lua: source.lua,
    })
}

fn model_signatures(model: &ApiModel) -> Vec<ModelSignature> {
    let mut signatures = model
        .modules
        .iter()
        .flat_map(|module| {
            module.functions.iter().map(|function| ModelSignature {
                module: module.name.clone(),
                name: function.name.clone(),
                inputs: function
                    .inputs
                    .iter()
                    .map(|field| SignatureField {
                        name: render_wit::wit_identifier(&field.name),
                        ty: render_wit::wit_type(&field.ty),
                    })
                    .collect(),
                result: result_type(function),
                environments: function.environments.clone(),
            })
        })
        .collect::<Vec<_>>();
    signatures.sort_by(|left, right| (&left.module, &left.name).cmp(&(&right.module, &right.name)));
    signatures
}

fn result_type(function: &FunctionModel) -> String {
    match function.outputs.as_slice() {
        [] => "_".to_string(),
        [field] => render_wit::wit_type(&field.ty),
        _ => format!("{}-value", render_wit::wit_identifier(&function.name)),
    }
}

fn parse_wit_signatures(model: &ApiModel, wit_dir: &Path) -> Result<Vec<WitSignature>> {
    let expected_interfaces = model
        .modules
        .iter()
        .flat_map(render_wit::interface_variants)
        .map(|(name, _)| render_wit::wit_identifier(&name))
        .collect::<BTreeSet<_>>();
    let world_environments = parse_world_imports(wit_dir)?;
    let mut parsed = Vec::new();
    let mut seen_interfaces = BTreeSet::new();

    for entry in fs::read_dir(wit_dir)
        .with_context(|| format!("reading generated WIT directory {}", wit_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("wit") {
            continue;
        }
        let content = fs::read_to_string(&path)
            .with_context(|| format!("reading emitted WIT {}", path.display()))?;
        if content.contains("world ") {
            continue;
        }
        let Some(interface) = parse_named_declaration(&content, "interface") else {
            continue;
        };
        // Callin interfaces are a separate engine-to-guest inventory.  They
        // are checked by the callin registry and are not NativeInterface
        // callout signatures.
        if interface == "callins" || interface.starts_with("callins-") {
            continue;
        }
        if !expected_interfaces.contains(&interface) {
            bail!(
                "unexpected emitted callout interface {interface} in {}",
                path.display()
            );
        }
        if !seen_interfaces.insert(interface.clone()) {
            bail!("duplicate emitted callout interface {interface}");
        }
        let environments = world_environments
            .get(&interface)
            .cloned()
            .unwrap_or_default();
        parsed.extend(parse_interface_functions(
            &content,
            &interface,
            environments,
        )?);
    }

    for interface in expected_interfaces {
        if !seen_interfaces.contains(&interface) {
            bail!("missing emitted callout interface {interface}");
        }
    }
    parsed
        .sort_by(|left, right| (&left.interface, &left.name).cmp(&(&right.interface, &right.name)));
    Ok(parsed)
}

fn parse_world_imports(wit_dir: &Path) -> Result<BTreeMap<String, BTreeSet<Environment>>> {
    let mut imports = BTreeMap::<String, BTreeSet<Environment>>::new();
    for environment in Environment::ALL {
        let path = wit_dir.join(format!("{}.wit", environment.as_str()));
        let content = fs::read_to_string(&path)
            .with_context(|| format!("reading generated world {}", path.display()))?;
        let mut world_imports = Vec::new();
        for line in content.lines() {
            let line = line.trim();
            if let Some(import) = line.strip_prefix("import ") {
                if let Some(name) = import.strip_suffix(';') {
                    world_imports.push(name.trim().trim_start_matches('%').to_string());
                }
            }
        }
        for interface in world_imports {
            imports.entry(interface).or_default().insert(environment);
        }
    }
    Ok(imports)
}

fn parse_named_declaration(content: &str, kind: &str) -> Option<String> {
    content.lines().find_map(|line| {
        let line = line.trim();
        let prefix = format!("{kind} ");
        line.strip_prefix(&prefix)
            .and_then(|rest| rest.split_whitespace().next())
            .map(|name| name.trim_start_matches('%').to_string())
    })
}

fn parse_interface_functions(
    content: &str,
    interface: &str,
    environments: BTreeSet<Environment>,
) -> Result<Vec<WitSignature>> {
    let mut signatures = Vec::new();
    for line in content.lines() {
        let line = line.trim();
        let Some(colon) = line.find(": func(") else {
            continue;
        };
        let name = line[..colon].trim().trim_start_matches('%').to_string();
        let rest = &line[colon + ": func(".len()..];
        let Some(close) = rest.find(") -> result<") else {
            bail!("malformed WIT function {interface}::{name}");
        };
        let inputs = parse_signature_fields(&rest[..close])
            .with_context(|| format!("parsing WIT inputs {interface}::{name}"))?;
        let result = rest[close + ") -> result<".len()..]
            .strip_suffix(">;")
            .ok_or_else(|| anyhow!("malformed WIT result {interface}::{name}"))?
            .trim()
            .to_string();
        let result = split_top_level(&result, ',')
            .first()
            .cloned()
            .ok_or_else(|| anyhow!("empty WIT result {interface}::{name}"))?;
        signatures.push(WitSignature {
            interface: interface.to_string(),
            name,
            inputs,
            result,
            environments: environments.clone(),
        });
    }
    if signatures.is_empty() {
        bail!("emitted interface {interface} contains zero functions");
    }
    Ok(signatures)
}

fn parse_signature_fields(value: &str) -> Result<Vec<SignatureField>> {
    if value.trim().is_empty() {
        return Ok(Vec::new());
    }
    split_top_level(value, ',')
        .into_iter()
        .map(|field| {
            let (name, ty) = field
                .split_once(':')
                .ok_or_else(|| anyhow!("malformed WIT field {field}"))?;
            Ok(SignatureField {
                // Preserve WIT's `%` escape.  It is part of the emitted
                // transport spelling and is therefore useful to the
                // round-trip check (for example `%type` and `%func`).
                name: name.trim().to_string(),
                ty: ty.trim().to_string(),
            })
        })
        .collect()
}

fn split_top_level(value: &str, separator: char) -> Vec<String> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut depth = 0i32;
    for (index, character) in value.char_indices() {
        match character {
            '<' | '(' | '[' | '{' => depth += 1,
            '>' | ')' | ']' | '}' => depth = (depth - 1).max(0),
            character if character == separator && depth == 0 => {
                let part = value[start..index].trim();
                if !part.is_empty() {
                    parts.push(part.to_string());
                }
                start = index + character.len_utf8();
            }
            _ => {}
        }
    }
    let tail = value[start..].trim();
    if !tail.is_empty() {
        parts.push(tail.to_string());
    }
    parts
}

fn extract_native_and_lua(root: &Path) -> Result<SourceSignatures> {
    let script = root.join("rts/wasm/extract_signature_sources.py");
    let output = Command::new("python3")
        .arg(&script)
        .arg("--root")
        .arg(root)
        .current_dir(root)
        .output()
        .with_context(|| format!("running {}", script.display()))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!(
            "{} failed with {}: {}",
            script.display(),
            output.status,
            stderr.trim()
        );
    }
    serde_json::from_slice(&output.stdout).context("parsing independent native/Lua signature data")
}

fn compare_wit(
    model: &ApiModel,
    model_signatures: &[ModelSignature],
    wit_signatures: &[WitSignature],
    errors: &mut Vec<String>,
) {
    let mut expected = BTreeMap::new();
    for module in &model.modules {
        for (interface, variant) in render_wit::interface_variants(module) {
            for function in &variant.functions {
                let Some(model_signature) = model_signatures.iter().find(|signature| {
                    signature.module == module.name && signature.name == function.name
                }) else {
                    errors.push(format!(
                        "WIT model signature missing {}::{}",
                        module.name, function.name
                    ));
                    continue;
                };
                expected.insert(
                    (
                        render_wit::wit_identifier(&interface),
                        render_wit::wit_identifier(&function.name),
                    ),
                    (model_signature.clone(), function.mutating),
                );
            }
        }
    }

    let mut actual = BTreeMap::new();
    for signature in wit_signatures {
        if actual
            .insert(
                (signature.interface.clone(), signature.name.clone()),
                signature,
            )
            .is_some()
        {
            errors.push(format!(
                "WIT emitted duplicate function {}::{}",
                signature.interface, signature.name
            ));
        }
    }

    for ((interface, name), (expected_signature, mutating)) in &expected {
        let Some(actual_signature) = actual.get(&(interface.clone(), name.clone())) else {
            errors.push(format!("WIT missing function {interface}::{name}"));
            continue;
        };
        if actual_signature.inputs != expected_signature.inputs {
            errors.push(format!("WIT input mismatch {interface}::{name}"));
        }
        if actual_signature.result != expected_signature.result {
            errors.push(format!("WIT result mismatch {interface}::{name}"));
        }
        if actual_signature.environments != expected_signature.environments {
            errors.push(format!("WIT environment mismatch {interface}::{name}"));
        }
        if *mutating
            && actual_signature
                .environments
                .iter()
                .any(|environment| !environment.is_synced())
        {
            errors.push(format!(
                "WIT mutating function in unsynced/UI world {interface}::{name}"
            ));
        }
    }
    for (interface, name) in actual.keys() {
        if !expected.contains_key(&(interface.clone(), name.clone())) {
            errors.push(format!("WIT extra function {interface}::{name}"));
        }
    }
}

fn compare_native(model: &ApiModel, native: &[NativeSignature], errors: &mut Vec<String>) {
    let mut by_name = BTreeMap::<(String, String), Vec<&NativeSignature>>::new();
    for signature in native {
        by_name
            .entry((
                signature.module.clone(),
                canonical_identifier(&signature.name),
            ))
            .or_default()
            .push(signature);
    }

    for module in &model.modules {
        for function in &module.functions {
            if native_signature_excluded(&module.name, &function.name) {
                continue;
            }
            let key = (module.name.clone(), canonical_identifier(&function.name));
            let Some(matches) = by_name.get(&key) else {
                errors.push(format!(
                    "native Rust missing {}::{}",
                    module.name, function.name
                ));
                continue;
            };
            if matches.len() != 1 {
                errors.push(format!(
                    "native Rust has {} signatures for {}::{}",
                    matches.len(),
                    module.name,
                    function.name
                ));
                continue;
            }
            let signature = matches[0];

            // Manual functions are still required to exist in the extracted
            // native surface, but their reviewed adapter is intentionally not
            // required to have the generated method's ordinary ABI shape.
            // This is the explicit boundary between the independent source
            // inventory check and the automatic signature comparison.
            if function.status == LoweringStatus::Manual {
                continue;
            }

            // `_unused` is a native ABI padding byte.  The generated Rust
            // facade intentionally omits it from public methods, while WIT
            // retains it as an explicit transport field for reproducibility.
            let expected_inputs = function
                .inputs
                .iter()
                .filter(|field| field.name != "_unused")
                .collect::<Vec<_>>();
            let mut expected_index = 0;
            let mut native_index = 0;
            while expected_index < expected_inputs.len() {
                let field = expected_inputs[expected_index];
                let Some(parameter) = signature.params.get(native_index) else {
                    errors.push(format!(
                        "native input count mismatch {}::{} (model={}, native={})",
                        module.name,
                        function.name,
                        expected_inputs.len(),
                        signature.params.len()
                    ));
                    break;
                };
                if canonical_identifier(&parameter.name) != canonical_identifier(&field.name) {
                    errors.push(format!(
                        "native input name mismatch {}::{} (model={}, native={})",
                        module.name, function.name, field.name, parameter.name
                    ));
                }

                let pointer_count_lowering = is_pointer_count_lowering(
                    &field.ty,
                    parameter,
                    signature.params.get(native_index + 1),
                );
                if !pointer_count_lowering && !native_type_matches(&field.ty, &parameter.ty) {
                    errors.push(format!(
                        "native input type mismatch {}::{}::{} (model={}, native={})",
                        module.name,
                        function.name,
                        field.name,
                        render_wit::wit_type(&field.ty),
                        parameter.ty
                    ));
                }
                native_index += 1 + usize::from(pointer_count_lowering);
                expected_index += 1;
            }
            if native_index != signature.params.len() {
                errors.push(format!(
                    "native input count mismatch {}::{} (model={}, native={})",
                    module.name,
                    function.name,
                    expected_inputs.len(),
                    signature.params.len()
                ));
            }

            let native_outputs = native_result_types(&signature.return_type);
            let mut expected_outputs = Vec::new();
            for field in &function.outputs {
                if field
                    .metadata
                    .iter()
                    .any(|metadata| metadata.starts_with("mutated-input:"))
                {
                    continue;
                }
                expected_outputs.push(&field.ty);
            }
            let mut native_output_index = 0;
            for expected in &expected_outputs {
                let Some(actual) = native_outputs.get(native_output_index) else {
                    errors.push(format!(
                        "native output count mismatch {}::{} (model={}, native={})",
                        module.name,
                        function.name,
                        expected_outputs.len(),
                        native_outputs.len()
                    ));
                    break;
                };

                // Native result records encode nullable values as `(value,
                // has_value)`, while the generated Rust facade exposes the
                // semantic `Option<value>`.  Collapse that ABI pair only when
                // the flag is actually a bool and the value matches the
                // Option's inner type.
                let presence_pair = match expected {
                    SemanticType::Option { inner }
                        if native_type_matches(inner, actual)
                            && native_outputs
                                .get(native_output_index + 1)
                                .is_some_and(|value| is_native_bool(value)) =>
                    {
                        true
                    }
                    _ => false,
                };
                if presence_pair {
                    native_output_index += 2;
                    continue;
                }
                if !native_type_matches(expected, actual) {
                    errors.push(format!(
                        "native output type mismatch {}::{} (model={}, native={})",
                        module.name,
                        function.name,
                        render_wit::wit_type(expected),
                        actual
                    ));
                }
                native_output_index += 1;
            }
            if native_output_index != native_outputs.len() {
                errors.push(format!(
                    "native output count mismatch {}::{} (model={}, native={})",
                    module.name,
                    function.name,
                    expected_outputs.len(),
                    native_outputs.len()
                ));
            }

            for field in &function.outputs {
                let Some(metadata) = field
                    .metadata
                    .iter()
                    .find(|metadata| metadata.starts_with("mutated-input:"))
                else {
                    continue;
                };
                let input_name = metadata.trim_start_matches("mutated-input:");
                let Some(parameter) = signature.params.iter().find(|parameter| {
                    canonical_identifier(&parameter.name) == canonical_identifier(input_name)
                }) else {
                    errors.push(format!(
                        "native mutated output input missing {}::{}::{}",
                        module.name, function.name, input_name
                    ));
                    continue;
                };
                if !parameter.ty.contains("&mut") {
                    errors.push(format!(
                        "native mutated output is not borrowed mutably {}::{}::{}",
                        module.name, function.name, input_name
                    ));
                }
                if !native_type_matches(&field.ty, &parameter.ty) {
                    errors.push(format!(
                        "native mutated output type mismatch {}::{}::{}",
                        module.name, function.name, input_name
                    ));
                }
            }
        }
    }
}

fn native_signature_excluded(module: &str, function: &str) -> bool {
    matches!(
        (module, function),
        (
            "rml_ui",
            "ContextRemoveEventListener"
                | "ElementRemoveEventListener"
                | "DataModelBindEvent"
                | "DataModelUnbindEvent"
        )
    )
}

fn canonical_identifier(value: &str) -> String {
    value
        .trim_start_matches("r#")
        .to_ascii_lowercase()
        .chars()
        .filter(|character| *character != '_')
        .collect()
}

fn is_native_bool(value: &str) -> bool {
    matches!(value.replace(' ', "").as_str(), "bool" | "sys::bool")
}

fn is_pointer_count_lowering(
    expected: &SemanticType,
    parameter: &NativeParameter,
    following: Option<&NativeParameter>,
) -> bool {
    let is_list = matches!(expected, SemanticType::List { .. } | SemanticType::Bytes);
    let is_pointer = parameter.ty.contains('*')
        || parameter.ty.starts_with("&sys::")
        || parameter.ty.starts_with("&mutsys::");
    let is_count = following.is_some_and(|parameter| {
        let name = canonical_identifier(&parameter.name);
        name == "count"
            || name.ends_with("count")
            || name.ends_with("length")
            || name.ends_with("size")
            || name.ends_with("bytes")
    });
    is_list && is_pointer && is_count
}

fn native_result_types(return_type: &str) -> Vec<String> {
    let value = return_type.trim();
    let value = value
        .strip_prefix("Result<")
        .and_then(|value| value.strip_suffix('>'))
        .and_then(|value| split_top_level(value, ',').first().cloned())
        .unwrap_or_else(|| value.to_string());
    let value = value.trim();
    if value == "()" || value.is_empty() {
        return Vec::new();
    }
    if value.starts_with('(') && value.ends_with(')') {
        return split_top_level(&value[1..value.len() - 1], ',');
    }
    vec![value.to_string()]
}

fn native_type_matches(expected: &SemanticType, actual: &str) -> bool {
    let actual = actual.replace(' ', "");
    match expected {
        SemanticType::Scalar { name } => {
            let name = match name.as_str() {
                "char" => "u8",
                "void" => "()",
                value => value,
            };
            actual == name || actual == format!("sys::{name}")
        }
        SemanticType::Enum { name } | SemanticType::Record { name } => {
            actual.ends_with(name) || actual.ends_with(&format!("sys::{name}"))
        }
        // The C ABI represents nullable output strings as `const char*` and
        // the native Rust wrapper correctly exposes `Option<String>`.  WIT's
        // semantic model currently treats those documented string results as
        // required strings; accept the owned nullable representation here
        // while still rejecting all unrelated types.
        SemanticType::String => matches!(actual.as_str(), "&str" | "String" | "Option<String>"),
        SemanticType::Bytes => actual.contains("[u8]") || actual.contains("Vec<u8>"),
        SemanticType::List { element } | SemanticType::FixedArray { element, .. } => {
            let inner = actual
                .strip_prefix("&[")
                .and_then(|value| value.strip_suffix(']'))
                .or_else(|| {
                    actual
                        .strip_prefix("Vec<")
                        .and_then(|value| value.strip_suffix('>'))
                })
                .or_else(|| {
                    actual
                        .strip_prefix('[')
                        .and_then(|value| value.split_once(';').map(|(element, _)| element))
                        .map(str::trim)
                });
            inner.is_some_and(|inner| native_type_matches(element, inner))
        }
        SemanticType::Option { inner } => actual
            .strip_prefix("Option<")
            .and_then(|value| value.strip_suffix('>'))
            .is_some_and(|value| native_type_matches(inner, value)),
        SemanticType::Result { .. } => actual.contains("Result<"),
        SemanticType::Handle { .. } => actual == "u64" || actual.ends_with("Handle"),
        SemanticType::Callback { .. } => {
            actual == "u32" || actual.contains("Callback") || actual.contains("Fn")
        }
        SemanticType::Pointer { .. } | SemanticType::Unknown { .. } => {
            actual == "u32" || actual.contains('*')
        }
    }
}

fn compare_lua(model: &ApiModel, lua: &LuaSignatures, errors: &mut Vec<String>) {
    let registered = lua.registered.iter().collect::<BTreeSet<_>>();
    for function in &lua.documented {
        if !registered.contains(&function.name) {
            errors.push(format!(
                "Lua documented function is not registered: {}",
                function.name
            ));
        }
    }

    for unmatched in &lua.unmatched {
        if !lua.explicit_exclusions.contains_key(unmatched) {
            errors.push(format!("Lua function has no native signature: {unmatched}"));
        }
    }
    for matched in &lua.matches {
        if !matched.params_match && matched.native_name.is_some() {
            errors.push(format!(
                "Lua/native parameter mismatch {}: {}",
                matched.lua_name, matched.detail
            ));
        }
        let Some(native_name) = &matched.native_name else {
            continue;
        };
        let lua_name = matched
            .lua_name
            .rsplit('.')
            .next()
            .unwrap_or(&matched.lua_name);
        let normalized = canonical_identifier(lua_name);
        let candidates = model
            .modules
            .iter()
            .flat_map(|module| {
                module
                    .functions
                    .iter()
                    .map(move |function| (module, function))
            })
            .filter(|(_, function)| canonical_identifier(&function.name) == normalized)
            .collect::<Vec<_>>();
        if candidates.is_empty() {
            errors.push(format!(
                "Lua/native function is absent from the semantic model: {} -> {}",
                matched.lua_name, native_name
            ));
        } else if candidates
            .iter()
            .any(|(_, function)| function.name != lua_name)
        {
            errors.push(format!(
                "Lua/model function spelling mismatch: {} vs {}",
                matched.lua_name, candidates[0].1.name
            ));
        }
    }
}

pub fn compare(left: &ApiModel, right: &ApiModel) -> Result<()> {
    let mut errors = Vec::new();
    if left.model_version != right.model_version {
        errors.push(format!(
            "model version mismatch: {} != {}",
            left.model_version, right.model_version
        ));
    }
    if left.native_api_version != right.native_api_version {
        errors.push("native API version mismatch".to_string());
    }
    for left_module in &left.modules {
        let Some(right_module) = right
            .modules
            .iter()
            .find(|module| module.name == left_module.name)
        else {
            errors.push(format!("missing module {}", left_module.name));
            continue;
        };
        if left_module.interface_version != right_module.interface_version {
            errors.push(format!("{} interface version mismatch", left_module.name));
        }
        for left_function in &left_module.functions {
            let Some(right_function) = right_module
                .functions
                .iter()
                .find(|function| function.name == left_function.name)
            else {
                errors.push(format!(
                    "{} missing function {}",
                    left_module.name, left_function.name
                ));
                continue;
            };
            if left_function.query != right_function.query {
                errors.push(format!(
                    "{}::{} query mismatch",
                    left_module.name, left_function.name
                ));
            }
            if left_function.result != right_function.result {
                errors.push(format!(
                    "{}::{} result mismatch",
                    left_module.name, left_function.name
                ));
            }
            if left_function.inputs != right_function.inputs {
                errors.push(format!(
                    "{}::{} input mismatch",
                    left_module.name, left_function.name
                ));
            }
            if left_function.outputs != right_function.outputs {
                errors.push(format!(
                    "{}::{} output mismatch",
                    left_module.name, left_function.name
                ));
            }
            if left_function.environments != right_function.environments {
                errors.push(format!(
                    "{}::{} environment mismatch",
                    left_module.name, left_function.name
                ));
            }
            if left_function.mutating != right_function.mutating {
                errors.push(format!(
                    "{}::{} mutation marker mismatch",
                    left_module.name, left_function.name
                ));
            }
            if left_function.visibility_sensitive != right_function.visibility_sensitive {
                errors.push(format!(
                    "{}::{} visibility marker mismatch",
                    left_module.name, left_function.name
                ));
            }
            if left_function.status != right_function.status {
                errors.push(format!(
                    "{}::{} lowering status mismatch",
                    left_module.name, left_function.name
                ));
            }
        }
        for right_function in &right_module.functions {
            if !left_module
                .functions
                .iter()
                .any(|function| function.name == right_function.name)
            {
                errors.push(format!(
                    "{} extra function {}",
                    left_module.name, right_function.name
                ));
            }
        }
        for left_record in &left_module.records {
            let Some(right_record) = right_module
                .records
                .iter()
                .find(|record| record.name == left_record.name)
            else {
                errors.push(format!(
                    "{} missing record {}",
                    left_module.name, left_record.name
                ));
                continue;
            };
            if left_record != right_record {
                errors.push(format!(
                    "{}::{} record mismatch",
                    left_module.name, left_record.name
                ));
            }
        }
        for right_record in &right_module.records {
            if !left_module
                .records
                .iter()
                .any(|record| record.name == right_record.name)
            {
                errors.push(format!(
                    "{} extra record {}",
                    left_module.name, right_record.name
                ));
            }
        }
        for left_enum in &left_module.enums {
            let Some(right_enum) = right_module
                .enums
                .iter()
                .find(|item| item.name == left_enum.name)
            else {
                errors.push(format!(
                    "{} missing enum {}",
                    left_module.name, left_enum.name
                ));
                continue;
            };
            if left_enum != right_enum {
                errors.push(format!(
                    "{}::{} enum mismatch",
                    left_module.name, left_enum.name
                ));
            }
        }
        for right_enum in &right_module.enums {
            if !left_module
                .enums
                .iter()
                .any(|item| item.name == right_enum.name)
            {
                errors.push(format!(
                    "{} extra enum {}",
                    left_module.name, right_enum.name
                ));
            }
        }
    }
    for right_module in &right.modules {
        if !left
            .modules
            .iter()
            .any(|module| module.name == right_module.name)
        {
            errors.push(format!("extra module {}", right_module.name));
        }
    }
    if left.callins != right.callins {
        errors.push("callin inventory mismatch".to_string());
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(anyhow!(errors.join("\n")))
    }
}

/// Return only the functions and callins available in one world. This is used
/// by the per-environment parity gate. The UI model can still be generated for
/// schema inspection, but the runtime deliberately does not load that world
/// until the visibility parity phase is complete.
pub fn for_environment(model: &ApiModel, environment: Environment) -> ApiModel {
    let mut result = model.clone();
    result.modules = result
        .modules
        .into_iter()
        .filter_map(|mut module| {
            module
                .functions
                .retain(|function| function.environments.contains(&environment));
            (!module.functions.is_empty()).then_some(module)
        })
        .collect();
    result
        .callins
        .retain(|callin| callin.environments.contains(&environment));
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        ApiModule, CallinModel, FieldModel, FunctionModel, LoweringStatus, SemanticType,
    };
    use std::collections::BTreeSet;

    fn function(name: &str, environment: Environment) -> FunctionModel {
        FunctionModel {
            name: name.to_string(),
            query: format!("{name}Query"),
            result: format!("{name}Result"),
            inputs: vec![FieldModel {
                name: "value".to_string(),
                ty: SemanticType::Scalar {
                    name: "i32".to_string(),
                },
                status: LoweringStatus::Automatic,
                metadata: Vec::new(),
            }],
            outputs: Vec::new(),
            environments: BTreeSet::from([environment]),
            mutating: false,
            visibility_sensitive: false,
            status: LoweringStatus::Automatic,
            notes: Vec::new(),
        }
    }

    fn model() -> ApiModel {
        ApiModel {
            model_version: 1,
            native_api_version: Some([1, 5, 0]),
            modules: vec![ApiModule {
                name: "synthetic".to_string(),
                interface_version: "1.0".to_string(),
                functions: vec![function("Call", Environment::RulesSynced)],
                records: Vec::new(),
                enums: Vec::new(),
            }],
            callins: vec![CallinModel {
                name: "GameFrame".to_string(),
                query: "GameFrameQuery".to_string(),
                result: "GameFrameResult".to_string(),
                environments: BTreeSet::from([Environment::RulesSynced]),
                aggregation: "ignore".to_string(),
                aliases: Vec::new(),
                flags: Vec::new(),
            }],
        }
    }

    #[test]
    fn compare_rejects_extra_functions_and_semantic_markers() {
        let left = model();
        let mut right = left.clone();
        right.modules[0]
            .functions
            .push(function("Extra", Environment::RulesSynced));
        let error = compare(&left, &right).unwrap_err().to_string();
        assert!(error.contains("extra function Extra"));

        right.modules[0].functions[0].mutating = true;
        let error = compare(&left, &right).unwrap_err().to_string();
        assert!(error.contains("mutation marker mismatch"));
    }

    #[test]
    fn compare_checks_records_callins_and_versions() {
        let left = model();
        let mut right = left.clone();
        right.model_version = 2;
        right.callins[0].aggregation = "first".to_string();
        let error = compare(&left, &right).unwrap_err().to_string();
        assert!(error.contains("model version mismatch"));
        assert!(error.contains("callin inventory mismatch"));
    }

    #[test]
    fn for_environment_filters_callins_as_well_as_functions() {
        let mut source = model();
        source.modules[0]
            .functions
            .push(function("Unsynced", Environment::RulesUnsynced));
        source.callins.push(CallinModel {
            name: "Update".to_string(),
            query: "UpdateQuery".to_string(),
            result: "UpdateResult".to_string(),
            environments: BTreeSet::from([Environment::RulesUnsynced]),
            aggregation: "ignore".to_string(),
            aliases: Vec::new(),
            flags: Vec::new(),
        });
        let synced = for_environment(&source, Environment::RulesSynced);
        assert_eq!(synced.modules[0].functions.len(), 1);
        assert_eq!(synced.callins.len(), 1);
        assert_eq!(synced.callins[0].name, "GameFrame");
    }
}
