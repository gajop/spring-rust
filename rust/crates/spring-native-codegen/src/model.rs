//! Transport-neutral description of the Spring NativeInterface.
//!
//! The legacy native renderer still lives in `lib.rs`, but all new transports
//! use this model.  Keeping the model in the generator crate lets native
//! bindings remain backwards compatible while WIT, host adapters, SDKs and
//! signature artifacts are generated from the same semantic inventory.

use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use crate::{lua_loader::LuaLoaderMatrix, ApiSpec, CType, Primitive};

/// An execution environment in which a module instance may run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Environment {
    RulesSynced,
    RulesUnsynced,
    GaiaSynced,
    GaiaUnsynced,
    Ui,
}

impl Environment {
    pub const GADGETS: [Self; 4] = [
        Self::RulesSynced,
        Self::RulesUnsynced,
        Self::GaiaSynced,
        Self::GaiaUnsynced,
    ];

    pub const ALL: [Self; 5] = [
        Self::RulesSynced,
        Self::RulesUnsynced,
        Self::GaiaSynced,
        Self::GaiaUnsynced,
        Self::Ui,
    ];

    pub const fn is_synced(self) -> bool {
        matches!(self, Self::RulesSynced | Self::GaiaSynced)
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RulesSynced => "rules-synced",
            Self::RulesUnsynced => "rules-unsynced",
            Self::GaiaSynced => "gaia-synced",
            Self::GaiaUnsynced => "gaia-unsynced",
            Self::Ui => "ui",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "rules-synced" => Some(Self::RulesSynced),
            "rules-unsynced" => Some(Self::RulesUnsynced),
            "gaia-synced" => Some(Self::GaiaSynced),
            "gaia-unsynced" => Some(Self::GaiaUnsynced),
            "ui" => Some(Self::Ui),
            _ => None,
        }
    }
}

/// How a semantic type is transported.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum SemanticType {
    Scalar {
        name: String,
    },
    Enum {
        name: String,
    },
    Record {
        name: String,
    },
    String,
    Bytes,
    List {
        element: Box<SemanticType>,
    },
    FixedArray {
        element: Box<SemanticType>,
        length: u64,
    },
    Option {
        inner: Box<SemanticType>,
    },
    Result {
        ok: Option<Box<SemanticType>>,
        error: Option<Box<SemanticType>>,
    },
    Handle {
        family: String,
    },
    Callback {
        name: String,
    },
    Pointer {
        pointee: Box<SemanticType>,
        mutable: bool,
    },
    Unknown {
        spelling: String,
    },
}

impl SemanticType {
    pub fn is_variable_size(&self) -> bool {
        match self {
            Self::String | Self::Bytes | Self::List { .. } => true,
            Self::FixedArray { element, .. }
            | Self::Option { inner: element }
            | Self::Pointer {
                pointee: element, ..
            } => element.is_variable_size(),
            Self::Result { ok, error } => {
                ok.as_deref().is_some_and(Self::is_variable_size)
                    || error.as_deref().is_some_and(Self::is_variable_size)
            }
            Self::Record { .. }
            | Self::Scalar { .. }
            | Self::Enum { .. }
            | Self::Handle { .. }
            | Self::Callback { .. }
            | Self::Unknown { .. } => false,
        }
    }

    pub fn is_unsupported(&self) -> bool {
        match self {
            Self::Unknown { .. } => true,
            Self::List { element }
            | Self::FixedArray { element, .. }
            | Self::Option { inner: element }
            | Self::Pointer {
                pointee: element, ..
            } => element.is_unsupported(),
            Self::Result { ok, error } => {
                ok.as_deref().is_some_and(Self::is_unsupported)
                    || error.as_deref().is_some_and(Self::is_unsupported)
            }
            _ => false,
        }
    }
}

/// Status of a C declaration's lowering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LoweringStatus {
    Automatic,
    Annotated,
    Manual,
    Unsupported,
}

/// A field in a semantic record or query/result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldModel {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: SemanticType,
    pub status: LoweringStatus,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<String>,
}

/// A record definition used by an API function.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordModel {
    pub name: String,
    pub fields: Vec<FieldModel>,
    pub status: LoweringStatus,
}

/// An enum definition used by an API function.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnumModel {
    pub name: String,
    pub variants: BTreeMap<String, i64>,
}

/// A normalized NativeInterface callout.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionModel {
    pub name: String,
    pub query: String,
    pub result: String,
    pub inputs: Vec<FieldModel>,
    pub outputs: Vec<FieldModel>,
    pub environments: BTreeSet<Environment>,
    pub mutating: bool,
    /// Results whose meaning depends on ally/LOS/UI visibility context.
    #[serde(default)]
    pub visibility_sensitive: bool,
    pub status: LoweringStatus,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
}

/// One coherent API module.  Its `name` is the stable C header/module name,
/// while `interface_version` is the independently versioned Wasm interface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiModule {
    pub name: String,
    pub interface_version: String,
    pub functions: Vec<FunctionModel>,
    pub records: Vec<RecordModel>,
    pub enums: Vec<EnumModel>,
}

/// The complete normalized API model for a generation run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiModel {
    pub model_version: u32,
    pub native_api_version: Option<[u32; 3]>,
    pub modules: Vec<ApiModule>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub callins: Vec<CallinModel>,
}

/// Canonical event/callin metadata shared by native and Wasm dispatch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallinModel {
    pub name: String,
    pub query: String,
    pub result: String,
    pub environments: BTreeSet<Environment>,
    pub aggregation: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub flags: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationMode {
    Report,
    Strict,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelSummary {
    pub modules: usize,
    pub functions: usize,
    pub automatic: usize,
    pub annotated: usize,
    pub manual: usize,
    pub unsupported: usize,
    pub per_environment: BTreeMap<String, usize>,
}

impl ApiModel {
    pub fn summary(&self) -> ModelSummary {
        let mut summary = ModelSummary {
            modules: self.modules.len(),
            ..ModelSummary::default()
        };

        for module in &self.modules {
            for function in &module.functions {
                summary.functions += 1;
                match function.status {
                    LoweringStatus::Automatic => summary.automatic += 1,
                    LoweringStatus::Annotated => summary.annotated += 1,
                    LoweringStatus::Manual => summary.manual += 1,
                    LoweringStatus::Unsupported => summary.unsupported += 1,
                }
                for environment in &function.environments {
                    *summary
                        .per_environment
                        .entry(environment.as_str().to_string())
                        .or_default() += 1;
                }
            }
        }

        summary
    }

    /// Validate invariants shared by all transports.
    pub fn validate(&self, mode: ValidationMode) -> anyhow::Result<()> {
        let mut errors = Vec::new();

        for module in &self.modules {
            let mut names = BTreeSet::new();
            for function in &module.functions {
                if !names.insert(&function.name) {
                    errors.push(format!(
                        "{}: duplicate function {}",
                        module.name, function.name
                    ));
                }
                if function.query.is_empty() || function.result.is_empty() {
                    errors.push(format!(
                        "{}::{}: missing query/result record",
                        module.name, function.name
                    ));
                }
                if function.environments.is_empty() {
                    errors.push(format!(
                        "{}::{}: no execution environment assigned",
                        module.name, function.name
                    ));
                }
                if function.mutating
                    && function
                        .environments
                        .iter()
                        .any(|environment| !environment.is_synced())
                {
                    errors.push(format!(
                        "{}::{}: mutating function is available in an unsynced or UI environment",
                        module.name, function.name
                    ));
                }
                let reviewed_variable_result = function.name == "CallCOBScript"
                    || function
                        .notes
                        .iter()
                        .any(|note| note.contains("reviewed mutating variable-size"));
                if function.mutating
                    && function
                        .outputs
                        .iter()
                        .any(|field| field.ty.is_variable_size())
                    && !reviewed_variable_result
                {
                    errors.push(format!(
                        "{}::{}: mutating function has a variable-size result; annotate a reviewed exception",
                        module.name, function.name
                    ));
                }
                if mode == ValidationMode::Strict && function.status == LoweringStatus::Unsupported
                {
                    errors.push(format!(
                        "{}::{}: unsupported semantic lowering",
                        module.name, function.name
                    ));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(errors.join("\n")))
        }
    }

    pub fn to_pretty_json(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string_pretty(self)? + "\n")
    }
}

/// Convert the legacy Clang model into the transport-neutral model.
pub(crate) fn from_legacy_spec(
    spec: &ApiSpec,
    module_name: &str,
    header: &Path,
    lua_loaders: &LuaLoaderMatrix,
) -> ApiModule {
    let mut records = Vec::new();
    for record in spec.all_structs.values() {
        // NativeInterface headers include platform headers transitively.  Do
        // not expose implementation-detail declarations such as glibc's
        // `__fsid_t` in a transport model: they are not part of any API
        // signature, and their layout is neither portable nor a supported
        // Wasm value shape.
        if is_private_record_name(&record.name) {
            continue;
        }
        let fields = normalize_record_fields(record);
        let status = fields
            .iter()
            .map(|field| field.status)
            .max_by_key(lowering_rank)
            .unwrap_or(LoweringStatus::Automatic);
        records.push(RecordModel {
            name: record.name.clone(),
            fields,
            status,
        });
    }
    records.sort_by(|left, right| left.name.cmp(&right.name));

    let mut enums = spec
        .enums
        .values()
        .map(|enum_def| EnumModel {
            name: enum_def.name.clone(),
            variants: enum_def.variants.clone(),
        })
        .collect::<Vec<_>>();
    enums.sort_by(|left, right| left.name.cmp(&right.name));

    let mut functions = Vec::new();
    for function in &spec.api.functions {
        let query = spec.structs.get(&function.query);
        let result = spec.structs.get(&function.result);
        let mut inputs = query
            .map(|record| normalize_fields(&record.fields))
            .unwrap_or_default();
        let mut outputs = result
            .map(|record| normalize_result_fields(&record.fields))
            .unwrap_or_default();
        normalize_known_string_shapes(module_name, &function.name, &mut inputs, &mut outputs);
        let mut notes = Vec::new();
        let mut status = combine_status(&inputs, &outputs);
        if query.is_none() || result.is_none() {
            status = LoweringStatus::Unsupported;
            notes.push("missing query/result record".to_string());
        }
        if inputs.iter().any(|field| field.ty.is_unsupported())
            || outputs.iter().any(|field| field.ty.is_unsupported())
        {
            status = LoweringStatus::Unsupported;
            notes.push("one or more fields require a manual lowering".to_string());
        }
        let is_callback_function = function.name.contains("Callback")
            || (function.name.contains("EventListener")
                && function.name != "ContextRemoveEventListener"
                && function.name != "ElementRemoveEventListener");
        if is_callback_function {
            notes.push("callback or re-entry semantics require a reviewed adapter".to_string());
            if status == LoweringStatus::Automatic {
                status = LoweringStatus::Manual;
            }
        }
        if function.name == "CallCOBScript" {
            notes.push("reviewed mutating variable-size-result exception".to_string());
        }

        // Normalize mutates its Float3 input in place.  The native result only
        // carries the length, but the Wasm contract must return the mutated value
        // as owned data rather than silently losing the mutation at the pointer
        // boundary.  Keep this explicit in the shared model; the host adapter is
        // intentionally manual so it can apply the native operation first.
        if module_name == "math_extra" && function.name == "Normalize" {
            outputs.push(FieldModel {
                name: "vec".to_string(),
                ty: SemanticType::Record {
                    name: "Float3".to_string(),
                },
                status: LoweringStatus::Manual,
                metadata: vec!["mutated-input:vec".to_string()],
            });
            status = LoweringStatus::Manual;
            notes.push("returns the mutated Float3 input as owned data".to_string());
        }

        let mutating = is_mutating(module_name, &function.name);
        let visibility_sensitive = visibility_sensitive_module(module_name);
        functions.push(FunctionModel {
            name: function.name.clone(),
            query: function.query.clone(),
            result: function.result.clone(),
            inputs,
            outputs,
            environments: lua_loaders.environments_for_function(
                header,
                module_name,
                &function.name,
                mutating,
                visibility_sensitive,
            ),
            mutating,
            visibility_sensitive,
            status,
            notes,
        });
    }

    functions.sort_by(|left, right| left.name.cmp(&right.name));
    ApiModule {
        name: module_name.to_string(),
        interface_version: "1.0".to_string(),
        functions,
        records,
        enums,
    }
}

fn is_private_record_name(name: &str) -> bool {
    name.starts_with("__")
}

fn normalize_fields(fields: &[crate::FieldDef]) -> Vec<FieldModel> {
    let mut result = Vec::new();
    let mut index = 0;
    while index < fields.len() {
        let field = &fields[index];
        if let Some(next) = fields.get(index + 1) {
            if is_presence_pair(field, next) {
                result.push(FieldModel {
                    name: field.name.clone(),
                    ty: SemanticType::Option {
                        inner: Box::new(semantic_type(&field.ty)),
                    },
                    status: LoweringStatus::Automatic,
                    metadata: annotation_metadata(
                        field,
                        vec![format!("presence-field:{}", next.name)],
                    ),
                });
                index += 2;
                continue;
            }
            if is_count_field(next) && !is_char_pointer_with_semantic_size(field, next) {
                // An explicit list annotation takes precedence over the
                // structural pointer inference.  This matters for pointer
                // arrays whose pointee is itself a pointer (for example
                // `const char**`) and for parallel arrays that share one
                // count field.
                // A string/bytes annotation is also authoritative: a
                // `const char* name` followed by a `size` field is a sized
                // string record, not a pointer/count byte list.  Consuming
                // the size here would silently erase the size field from the
                // semantic record.
                let list_type = annotated_list_type(field).or_else(|| {
                    if has_string_or_bytes_annotation(field) {
                        None
                    } else {
                        list_type_for(&field.ty)
                    }
                });
                if let Some(list_type) = list_type {
                    result.push(FieldModel {
                        name: field.name.clone(),
                        ty: list_type,
                        status: LoweringStatus::Annotated,
                        metadata: annotation_metadata(
                            field,
                            vec![format!("count-field:{}", next.name)],
                        ),
                    });
                    index += 2;
                    continue;
                }
            }
        }

        let ty = annotated_semantic_type(field);
        let status = if has_manual_annotation(field) {
            LoweringStatus::Manual
        } else if matches!(field.ty, CType::Pointer { .. })
            && field
                .annotations
                .iter()
                .any(|annotation| annotation.starts_with("spring.wasm.record:"))
        {
            // A record annotation describes the pointee shape, but it does
            // not make the native pointer itself safe to pass through an
            // automatically generated adapter.  The host must allocate and
            // own the pointee explicitly.
            LoweringStatus::Manual
        } else if ty.is_unsupported() {
            LoweringStatus::Unsupported
        } else if matches!(ty, SemanticType::Pointer { .. }) {
            LoweringStatus::Manual
        } else {
            LoweringStatus::Automatic
        };
        result.push(FieldModel {
            name: field.name.clone(),
            ty,
            status,
            metadata: annotation_metadata(field, Vec::new()),
        });
        index += 1;
    }
    result
}

fn annotated_semantic_type(field: &crate::FieldDef) -> SemanticType {
    if field
        .annotations
        .iter()
        .any(|annotation| annotation == "spring.wasm.string")
    {
        return SemanticType::String;
    }
    if field
        .annotations
        .iter()
        .any(|annotation| annotation == "spring.wasm.bytes")
    {
        return SemanticType::Bytes;
    }
    if let Some(annotation) = field
        .annotations
        .iter()
        .find(|annotation| annotation.starts_with("spring.wasm.handle:"))
    {
        return SemanticType::Handle {
            family: annotation
                .strip_prefix("spring.wasm.handle:")
                .unwrap_or("native-handle")
                .to_string(),
        };
    }
    if let Some(annotation) = field
        .annotations
        .iter()
        .find(|annotation| annotation.starts_with("spring.wasm.record:"))
    {
        return SemanticType::Record {
            name: annotation
                .strip_prefix("spring.wasm.record:")
                .unwrap_or("NativeRecord")
                .to_string(),
        };
    }
    if let Some(annotation) = field
        .annotations
        .iter()
        .find(|annotation| annotation.starts_with("spring.wasm.list:"))
    {
        let mut parts = annotation.split(':');
        let _ = parts.next();
        let element = parts.next().unwrap_or("u8");
        let element_type = match element {
            "string" => SemanticType::String,
            "bytes" => SemanticType::Bytes,
            scalar => SemanticType::Scalar {
                name: scalar.to_string(),
            },
        };
        return SemanticType::List {
            element: Box::new(element_type),
        };
    }
    if field
        .annotations
        .iter()
        .any(|annotation| annotation == "spring.wasm.callback")
    {
        return SemanticType::Callback {
            name: "annotated-callback".to_string(),
        };
    }
    semantic_type(&field.ty)
}

fn annotated_list_type(field: &crate::FieldDef) -> Option<SemanticType> {
    field
        .annotations
        .iter()
        .find(|annotation| annotation.starts_with("spring.wasm.list:"))
        .map(|annotation| {
            let mut parts = annotation.split(':');
            let _ = parts.next();
            let element = parts.next().unwrap_or("u8");
            let element_type = match element {
                "string" => SemanticType::String,
                "bytes" => SemanticType::Bytes,
                scalar => SemanticType::Scalar {
                    name: scalar.to_string(),
                },
            };
            SemanticType::List {
                element: Box::new(element_type),
            }
        })
}

fn has_manual_annotation(field: &crate::FieldDef) -> bool {
    field
        .annotations
        .iter()
        .any(|annotation| annotation.starts_with("spring.wasm.manual:"))
}

fn has_string_or_bytes_annotation(field: &crate::FieldDef) -> bool {
    field
        .annotations
        .iter()
        .any(|annotation| annotation == "spring.wasm.string" || annotation == "spring.wasm.bytes")
}

fn annotation_metadata(field: &crate::FieldDef, mut metadata: Vec<String>) -> Vec<String> {
    metadata.extend(field.annotations.iter().cloned());
    for annotation in &field.annotations {
        if let Some(count) = annotation
            .strip_prefix("spring.wasm.list:")
            .and_then(|value| value.split(':').nth(1))
        {
            metadata.push(format!("count-field:{count}"));
        }
    }
    metadata
}

fn normalize_result_fields(fields: &[crate::FieldDef]) -> Vec<FieldModel> {
    let mut normalized = normalize_fields(
        &fields
            .iter()
            .filter(|field| field.name != "error")
            .cloned()
            .collect::<Vec<_>>(),
    );
    // ReadFileAsString has a C string plus an explicit byte length.  The
    // length is ownership/ABI metadata, not a second semantic return value;
    // the native Rust facade exposes this result as Option<String>.  Keep the
    // special case narrow so ordinary `const char*`/length byte buffers remain
    // modelled as lists.
    if fields.iter().any(|field| field.name == "content")
        && fields.iter().any(|field| field.name == "contentLength")
    {
        normalized.retain(|field| field.name != "contentLength");
        if let Some(content) = normalized.iter_mut().find(|field| field.name == "content") {
            content.ty = SemanticType::String;
            content.metadata.retain(|metadata| {
                !metadata.starts_with("count-field:") && !metadata.starts_with("presence-field:")
            });
        }
    }
    normalized
}

fn normalize_known_string_shapes(
    module: &str,
    function: &str,
    inputs: &mut Vec<FieldModel>,
    _outputs: &mut Vec<FieldModel>,
) {
    // GfxLoadFont(path, size, outlineWidth, outlineWeight) contains a string
    // followed by a real font size.  Keep both as semantic inputs even when a
    // compiler spelling makes the generic pointer/count recognizer classify
    // that pair as byte data.
    if module == "gfx" && function == "LoadFont" {
        if let Some(path) = inputs.iter_mut().find(|field| field.name == "path") {
            path.ty = SemanticType::String;
            path.metadata.retain(|metadata| {
                !metadata.starts_with("count-field:") && !metadata.starts_with("presence-field:")
            });
        }
        if !inputs.iter().any(|field| field.name == "size") {
            let insertion_index = inputs
                .iter()
                .position(|field| field.name == "outlineWidth")
                .unwrap_or(inputs.len());
            inputs.insert(
                insertion_index,
                FieldModel {
                    name: "size".to_string(),
                    ty: SemanticType::Scalar {
                        name: "i32".to_string(),
                    },
                    status: LoweringStatus::Automatic,
                    metadata: Vec::new(),
                },
            );
        }
    }
}

/// Native result structs carry an `Error*` transport field.  It is represented
/// by the function-level `result<..., spring-error>` in WIT and must not be
/// mistaken for a pointer/count list when the raw record is emitted.
fn normalize_record_fields(record: &crate::StructDef) -> Vec<FieldModel> {
	// RulesParamValue is a tagged C union.  Clang exposes the discriminant but
	// not the anonymous union members as ordinary record fields, while the
	// native ABI still requires the active payload to cross the Wasm boundary.
	// Keep the C layout unchanged and describe all union arms in the transport
	// model; the generated host adapter selects the arm from `type`.
	if record.name == "RulesParamValue" {
		return vec![
			FieldModel {
				name: "type".to_string(),
				ty: SemanticType::Enum {
					name: "RulesParamType".to_string(),
				},
				status: LoweringStatus::Automatic,
				metadata: Vec::new(),
			},
			FieldModel {
				name: "boolValue".to_string(),
				ty: SemanticType::Scalar {
					name: "bool".to_string(),
				},
				status: LoweringStatus::Automatic,
				metadata: Vec::new(),
			},
			FieldModel {
				name: "floatValue".to_string(),
				ty: SemanticType::Scalar {
					name: "f32".to_string(),
				},
				status: LoweringStatus::Automatic,
				metadata: Vec::new(),
			},
			FieldModel {
				name: "stringValue".to_string(),
				ty: SemanticType::String,
				status: LoweringStatus::Automatic,
				metadata: Vec::new(),
			},
		];
	}

	normalize_fields(
        &record
            .fields
            .iter()
            .filter(|field| field.name != "error")
            .cloned()
            .collect::<Vec<_>>(),
    )
}

fn semantic_type(ty: &CType) -> SemanticType {
    match ty {
        CType::Primitive(primitive) => SemanticType::Scalar {
            name: primitive_name(*primitive).to_string(),
        },
        CType::Enum(name) => SemanticType::Enum { name: name.clone() },
        CType::Record(name) => {
            if looks_like_handle(name) {
                SemanticType::Handle {
                    family: name.clone(),
                }
            } else {
                SemanticType::Record { name: name.clone() }
            }
        }
        CType::Pointer { pointee, is_const } => {
            if matches!(pointee.as_ref(), CType::Primitive(Primitive::Char)) {
                SemanticType::String
            } else {
                SemanticType::Pointer {
                    pointee: Box::new(semantic_type(pointee)),
                    mutable: !is_const,
                }
            }
        }
        CType::FnPtr => SemanticType::Callback {
            name: "native-callback".to_string(),
        },
        CType::Array { element, length } => SemanticType::FixedArray {
            element: Box::new(semantic_type(element)),
            length: *length,
        },
        CType::Unknown(spelling) => SemanticType::Unknown {
            spelling: spelling.clone(),
        },
    }
}

fn list_type_for(ty: &CType) -> Option<SemanticType> {
    let CType::Pointer { pointee, .. } = ty else {
        return None;
    };
    if matches!(pointee.as_ref(), CType::Primitive(Primitive::Char)) {
        return Some(SemanticType::Bytes);
    }
    Some(SemanticType::List {
        element: Box::new(semantic_type(pointee)),
    })
}

fn is_presence_pair(value: &crate::FieldDef, presence: &crate::FieldDef) -> bool {
    if !matches!(presence.ty, CType::Primitive(Primitive::Bool)) {
        return false;
    }
    let Some(suffix) = presence.name.strip_prefix("has") else {
        return false;
    };
    suffix.eq_ignore_ascii_case(&value.name)
        || value
            .name
            .strip_suffix("Value")
            .is_some_and(|prefix| suffix.eq_ignore_ascii_case(prefix))
}

fn is_count_field(field: &crate::FieldDef) -> bool {
    matches!(
        field.ty,
        CType::Primitive(Primitive::U32)
            | CType::Primitive(Primitive::U64)
            | CType::Primitive(Primitive::I32)
    ) && (field.name.eq_ignore_ascii_case("count")
        || field.name.ends_with("Count")
        || field.name.ends_with("Length")
        || field.name.ends_with("Bytes")
        || field.name.eq_ignore_ascii_case("size")
        || field.name.ends_with("Size")
        || field.name.starts_with("num"))
}

// A count-looking field after an unannotated C string can still be a real
// semantic argument (GfxLoadFont(path, size, ...) and
// SetMapShadingTexture(texName, num) are examples). Treat pointer/count as a
// byte-list lowering only when the API explicitly annotated the pointer as a
// list or bytes value. The annotation is the unambiguous escape hatch for
// APIs that genuinely use a `const char*` byte buffer.
fn is_char_pointer_with_semantic_size(value: &crate::FieldDef, _size: &crate::FieldDef) -> bool {
    matches!(
        &value.ty,
        CType::Pointer {
            pointee,
            ..
        } if matches!(pointee.as_ref(), CType::Primitive(Primitive::Char))
    ) && !value.annotations.iter().any(|annotation| {
        annotation.starts_with("spring.wasm.list:") || annotation == "spring.wasm.bytes"
    })
}

fn combine_status(inputs: &[FieldModel], outputs: &[FieldModel]) -> LoweringStatus {
    inputs
        .iter()
        .chain(outputs)
        .map(|field| field.status)
        .max_by_key(lowering_rank)
        .unwrap_or(LoweringStatus::Automatic)
}

fn lowering_rank(status: &LoweringStatus) -> u8 {
    match status {
        LoweringStatus::Automatic => 0,
        LoweringStatus::Annotated => 1,
        LoweringStatus::Manual => 2,
        LoweringStatus::Unsupported => 3,
    }
}

fn primitive_name(primitive: Primitive) -> &'static str {
    match primitive {
        Primitive::I8 => "i8",
        Primitive::I16 => "i16",
        Primitive::I32 => "i32",
        Primitive::I64 => "i64",
        Primitive::U32 => "u32",
        Primitive::U16 => "u16",
        Primitive::U8 => "u8",
        Primitive::U64 => "u64",
        Primitive::Bool => "bool",
        Primitive::F32 => "f32",
        Primitive::Char => "char",
        Primitive::Void => "void",
    }
}

fn looks_like_handle(name: &str) -> bool {
    name.ends_with("Handle") || name.ends_with("Resource")
}

fn is_mutating(module: &str, function: &str) -> bool {
    // `mutating` is the deterministic simulation policy bit, not a generic
    // "the C++ function changes some process state" bit.  Local camera,
    // renderer, audio, VFS, input and UI controls are intentionally exposed by
    // unsynced Lua loaders and must not be mislabeled as synced mutations.
    let synced_control_module = matches!(
        module,
        "team_control"
            | "unit_control"
            | "feature_control"
            | "terrain_control"
            | "projectile_control"
            | "effects_control"
            | "game_config"
            | "cob_script"
    );
    if synced_control_module {
        return !function.starts_with("Get")
            && !function.starts_with("Is")
            && !function.starts_with("Has")
            && !function.starts_with("Can")
            && !function.starts_with("Valid")
            && !function.starts_with("Find")
            && !function.starts_with("Test")
            && function != "GetFactoryBuggerOff";
    }

    match module {
        "move_ctrl" => {
            !function.starts_with("Get")
                && !function.starts_with("Is")
                && !function.starts_with("Has")
                && !function.starts_with("Can")
                && !function.starts_with("Test")
        }
        "rules_params" => function.starts_with("Set"),
        "units_commands" => function.starts_with("GiveOrder"),
        "metal_map" => function == "SetMetalAmount",
        "path_finder" => matches!(
            function,
            "DeletePath" | "SetPathNodeCost" | "SetPathNodeCosts"
        ),
        _ => false,
    }
}

fn visibility_sensitive_module(module: &str) -> bool {
    matches!(
        module,
        "units_query"
            | "units_info"
            | "features"
            | "projectiles"
            | "unsynced_read"
            | "unit_rendering"
            | "los"
            | "rules_params"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FieldDef, Primitive};

    fn field(name: &str, ty: CType) -> FieldModel {
        let raw = FieldDef {
            name: name.to_string(),
            ty,
            annotations: Vec::new(),
        };
        normalize_fields(&[raw]).remove(0)
    }

    #[test]
    fn lowers_scalar_string_fixed_array_and_callback_patterns() {
        assert_eq!(
            field("value", CType::Primitive(Primitive::I32)).ty,
            SemanticType::Scalar {
                name: "i32".to_string()
            }
        );
        assert_eq!(
            field(
                "text",
                CType::Pointer {
                    pointee: Box::new(CType::Primitive(Primitive::Char)),
                    is_const: true,
                }
            )
            .ty,
            SemanticType::String
        );
        assert_eq!(
            field(
                "values",
                CType::Array {
                    element: Box::new(CType::Primitive(Primitive::F32)),
                    length: 4,
                }
            )
            .ty,
            SemanticType::FixedArray {
                element: Box::new(SemanticType::Scalar {
                    name: "f32".to_string(),
                }),
                length: 4,
            }
        );
        assert!(matches!(
            field("callback", CType::FnPtr).ty,
            SemanticType::Callback { .. }
        ));
    }

    #[test]
    fn lowers_lists_and_options_using_adjacent_metadata() {
        let fields = normalize_fields(&[
            FieldDef {
                name: "data".to_string(),
                ty: CType::Pointer {
                    pointee: Box::new(CType::Primitive(Primitive::U8)),
                    is_const: true,
                },
                annotations: Vec::new(),
            },
            FieldDef {
                name: "dataLength".to_string(),
                ty: CType::Primitive(Primitive::U32),
                annotations: Vec::new(),
            },
            FieldDef {
                name: "health".to_string(),
                ty: CType::Primitive(Primitive::F32),
                annotations: Vec::new(),
            },
            FieldDef {
                name: "hasHealth".to_string(),
                ty: CType::Primitive(Primitive::Bool),
                annotations: Vec::new(),
            },
        ]);
        assert!(matches!(fields[0].ty, SemanticType::List { .. }));
        assert!(matches!(fields[1].ty, SemanticType::Option { .. }));
    }

    #[test]
    fn lowers_byte_buffers_with_size_metadata() {
        let fields = normalize_fields(&[
            FieldDef {
                name: "data".to_string(),
                ty: CType::Pointer {
                    pointee: Box::new(CType::Primitive(Primitive::U8)),
                    is_const: true,
                },
                annotations: Vec::new(),
            },
            FieldDef {
                name: "dataSize".to_string(),
                ty: CType::Primitive(Primitive::U32),
                annotations: Vec::new(),
            },
        ]);
        assert_eq!(fields.len(), 1);
        assert_eq!(
            fields[0].ty,
            SemanticType::List {
                element: Box::new(SemanticType::Scalar {
                    name: "u8".to_string(),
                }),
            }
        );
        assert!(fields[0]
            .metadata
            .contains(&"count-field:dataSize".to_string()));
    }

    #[test]
    fn adjacent_clang_annotations_override_ambiguous_layouts() {
        let field = FieldDef {
            name: "opaque".to_string(),
            ty: CType::Pointer {
                pointee: Box::new(CType::Primitive(Primitive::U8)),
                is_const: true,
            },
            annotations: vec!["spring.wasm.handle:texture".to_string()],
        };
        let lowered = normalize_fields(&[field]).remove(0);
        assert_eq!(
            lowered.ty,
            SemanticType::Handle {
                family: "texture".to_string()
            }
        );
        assert!(lowered
            .metadata
            .contains(&"spring.wasm.handle:texture".to_string()));
    }

    #[test]
    fn environment_mapping_uses_loader_and_mutation_policy() {
        let loaders = LuaLoaderMatrix::default();
        let header = Path::new("rts/NativeInterface/api/SyncedCtrl.h");
        let environments =
            loaders.environments_for_function(header, "unit_control", "CreateUnit", true, false);
        assert!(environments.contains(&Environment::RulesSynced));
        assert!(!environments.contains(&Environment::RulesUnsynced));
        assert!(!environments.contains(&Environment::Ui));

        let read_environments = loaders.environments_for_function(
            Path::new("rts/NativeInterface/api/UnitsQuery.h"),
            "units_query",
            "GetTeamUnitCount",
            false,
            true,
        );
        assert!(read_environments.contains(&Environment::RulesUnsynced));
        assert!(read_environments.contains(&Environment::Ui));
    }

    #[test]
    fn excludes_reserved_transitive_header_records_from_transport_models() {
        assert!(is_private_record_name("__fsid_t"));
        assert!(!is_private_record_name("PublicApiRecord"));

        let mut all_structs = std::collections::HashMap::new();
        all_structs.insert(
            "__fsid_t".to_string(),
            crate::StructDef {
                name: "__fsid_t".to_string(),
                fields: vec![crate::FieldDef {
                    name: "__val".to_string(),
                    ty: CType::Array {
                        element: Box::new(CType::Primitive(Primitive::I32)),
                        length: 2,
                    },
                    annotations: Vec::new(),
                }],
            },
        );
        all_structs.insert(
            "PublicApiRecord".to_string(),
            crate::StructDef {
                name: "PublicApiRecord".to_string(),
                fields: vec![crate::FieldDef {
                    name: "value".to_string(),
                    ty: CType::Primitive(Primitive::I32),
                    annotations: Vec::new(),
                }],
            },
        );

        let module = from_legacy_spec(
            &crate::ApiSpec {
                structs: std::collections::HashMap::new(),
                all_structs,
                enums: std::collections::HashMap::new(),
                api: crate::ApiDef {
                    functions: Vec::new(),
                },
            },
            "test",
            Path::new("rts/NativeInterface/api/Common.h"),
            &LuaLoaderMatrix::default(),
        );
        assert_eq!(module.records.len(), 1);
        assert_eq!(module.records[0].name, "PublicApiRecord");
    }
}
