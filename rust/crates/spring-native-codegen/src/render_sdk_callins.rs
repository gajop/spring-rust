/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

//! Reuse the tested Core callin codecs behind ordinary Rust handler arguments
//! and return values, with a fresh Spring scope on both targets.
use crate::model::{ApiModel, FieldModel, RecordModel, SemanticType};
use anyhow::{Context, Result, bail};
use heck::ToSnakeCase;
use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use quote::ToTokens;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use syn::parse::Parser;

fn ident(name: &str) -> String {
    let name = name.to_snake_case();
    if matches!(
        name.as_str(),
        "type"
            | "ref"
            | "in"
            | "loop"
            | "move"
            | "box"
            | "self"
            | "match"
            | "async"
            | "mod"
            | "use"
    ) {
        format!("{name}_")
    } else {
        name
    }
}
struct Builder<'a> {
    records: BTreeMap<String, &'a RecordModel>,
    model: &'a ApiModel,
}
impl Builder<'_> {
    fn record(&self, name: &str, args: &mut VecDeque<String>) -> Result<String> {
        let record = self
            .records
            .get(name)
            .with_context(|| format!("record {name}"))?;
        let counts = record
            .fields
            .iter()
            .flat_map(|f| {
                f.metadata.iter().filter_map(|m| {
                    m.strip_prefix("count-field:")
                        .map(|count| (count.to_owned(), ident(&f.name)))
                })
            })
            .collect::<BTreeMap<_, _>>();
        let mut setup = String::new();
        let mut fields = Vec::new();
        for field in record
            .fields
            .iter()
            .filter(|f| !counts.contains_key(&f.name))
        {
            let n = ident(&field.name);
            let value = self.value(&field.ty, args)?;
            setup.push_str(&format!("let {n}={value};"));
            fields.push(n.to_string());
        }
        for field in record
            .fields
            .iter()
            .filter(|f| counts.contains_key(&f.name))
        {
            let n = ident(&field.name);
            setup.push_str(&format!("let {n}={}.len().try_into().map_err(|_|$crate::ApiError::new($crate::ErrorCode::OutOfBounds as i32))?;",counts[&field.name]));
            fields.push(n);
        }
        Ok(format!(
            "{{{setup} $crate::types::callins::{name} {{{}}}}}",
            fields.join(",")
        ))
    }
    fn leaves(&self, fields: &[FieldModel]) -> usize {
        let counts = fields
            .iter()
            .flat_map(|f| {
                f.metadata
                    .iter()
                    .filter_map(|m| m.strip_prefix("count-field:"))
            })
            .collect::<BTreeSet<_>>();
        fields
            .iter()
            .filter(|f| !counts.contains(f.name.as_str()))
            .map(|f| self.leaf_count(&f.ty))
            .sum()
    }
    fn leaf_count(&self, ty: &SemanticType) -> usize {
        match ty {
            SemanticType::Record { name } => self.leaves(&self.records[name].fields),
            SemanticType::FixedArray { element, length } => {
                self.leaf_count(element) * *length as usize
            }
            _ => 1,
        }
    }
    fn value(&self, ty: &SemanticType, args: &mut VecDeque<String>) -> Result<String> {
        Ok(match ty {
            SemanticType::Record { name } => self.record(name, args)?,
            SemanticType::FixedArray { element, length } => format!(
                "vec![{}]",
                (0..*length)
                    .map(|_| self.value(element, args))
                    .collect::<Result<Vec<_>>>()?
                    .join(",")
            ),
            _ => {
                let arg = args.pop_front().context("callin argument exhausted")?;
                match ty {
                    SemanticType::String => {
                        format!("$crate::module::decode_string(({arg}).to_vec())")
                    }
                    SemanticType::Bytes => format!("({arg}).to_vec()"),
                    SemanticType::List { element } => {
                        if let SemanticType::Record { name } = element.as_ref() {
                            let count = self.leaves(&self.records[name].fields);
                            let vars = (0..count).map(|i| format!("item{i}")).collect::<Vec<_>>();
                            let mut items = vars.iter().cloned().collect();
                            let body = self.record(name, &mut items)?;
                            format!(
                                "({arg}).map(|({},)|-> $crate::Result<_> {{Ok({body})}}).collect::<$crate::Result<Vec<_>>>()?",
                                vars.join(",")
                            )
                        } else {
                            format!("({arg}).to_vec()")
                        }
                    }
                    SemanticType::Enum { name } => {
                        let enumeration = self
                            .model
                            .modules
                            .iter()
                            .flat_map(|m| &m.enums)
                            .find(|e| &e.name == name)
                            .context("enum")?;
                        use heck::ToUpperCamelCase;
                        let cases = enumeration
                            .variants
                            .iter()
                            .map(|(n, v)| {
                                format!(
                                    "{v}=>$crate::types::callins::{name}::{},",
                                    n.to_lowercase().to_upper_camel_case()
                                )
                            })
                            .collect::<String>();
                        format!(
                            "match {arg} as i64 {{{cases} _=>return Err($crate::ApiError::new($crate::ErrorCode::InvalidArgument as i32))}}"
                        )
                    }
                    SemanticType::Scalar { .. } | SemanticType::Handle { .. } => arg,
                    _ => bail!("callin input {ty:?}"),
                }
            }
        })
    }
    /// Hide transport-only envelope/count fields; retain semantic records such
    /// as commands and vectors. Nested query envelopes are flattened too.
    fn arguments<'a>(&'a self, record: &str, prefix: &str) -> Vec<(String, &'a SemanticType)> {
        let record = self.records[record];
        let counts = record
            .fields
            .iter()
            .flat_map(|f| {
                f.metadata
                    .iter()
                    .filter_map(|m| m.strip_prefix("count-field:"))
            })
            .collect::<BTreeSet<_>>();
        record
            .fields
            .iter()
            .filter(|f| {
                f.name != "_unused" && !counts.contains(f.name.as_str())
                // These two legacy string queries carry an unannotated byte
                // length. Rust strings already preserve their exact length.
                && !(f.name == "messageLength" && record.fields.iter()
                    .any(|f| f.name == "message" && matches!(f.ty, SemanticType::String)))
            })
            .flat_map(|f| {
                let access = format!("{prefix}.{}", ident(&f.name));
                if let SemanticType::Record { name } = &f.ty
                    && name.ends_with("Query")
                {
                    return self.arguments(name, &access);
                }
                vec![(access, &f.ty)]
            })
            .collect()
    }
    fn public_type(&self, ty: &SemanticType) -> Result<String> {
        Ok(match ty {
            SemanticType::Scalar { name } if name == "char" => "u8".into(),
            SemanticType::Scalar { name } => name.clone(),
            SemanticType::Handle { .. } => "u64".into(),
            SemanticType::Record { name } if name == "Float3" => "$crate::Float3".into(),
            SemanticType::Record { name } | SemanticType::Enum { name } => {
                format!("$crate::types::callins::{name}")
            }
            SemanticType::String => "String".into(),
            SemanticType::Bytes => "Vec<u8>".into(),
            SemanticType::List { element } | SemanticType::FixedArray { element, .. } => {
                format!("Vec<{}>", self.public_type(element)?)
            }
            _ => bail!("public callin type {ty:?}"),
        })
    }
    fn parameter(&self, access: &str, ty: &SemanticType) -> Result<(String, String)> {
        Ok(match ty {
            SemanticType::String => ("&str".into(), format!("{access}.as_str()")),
            SemanticType::Bytes => ("&[u8]".into(), format!("{access}.as_slice()")),
            SemanticType::List { element } | SemanticType::FixedArray { element, .. } => (
                format!("&[{}]", self.public_type(element)?),
                format!("{access}.as_slice()"),
            ),
            SemanticType::Record { name } if name != "Float3" => {
                (format!("&{}", self.public_type(ty)?), format!("&{access}"))
            }
            _ => (self.public_type(ty)?, access.into()),
        })
    }
    fn public_result(&self, record: &RecordModel) -> Result<String> {
        let types = record
            .fields
            .iter()
            .map(|f| self.public_type(&f.ty))
            .collect::<Result<Vec<_>>>()?;
        Ok(match types.as_slice() {
            [] => "()".into(),
            [only] => only.clone(),
            _ => format!("({})", types.join(", ")),
        })
    }
    fn pack_result(&self, record: &RecordModel) -> String {
        let fields = record
            .fields
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let access = if record.fields.len() == 1 {
                    "returned".into()
                } else {
                    format!("returned.{i}")
                };
                format!("{}:{access}", ident(&f.name))
            })
            .collect::<Vec<_>>()
            .join(",");
        let unit = if record.fields.is_empty() {
            "let ()=returned;"
        } else {
            ""
        };
        format!(
            "{{{unit}$crate::types::callins::{}{{{fields}}}}}",
            record.name
        )
    }
    fn result_leaves(&self, ty: &SemanticType, access: &str) -> Result<Vec<String>> {
        Ok(match ty {
            SemanticType::Record { name } => self.records[name]
                .fields
                .iter()
                .map(|f| self.result_leaves(&f.ty, &format!("{access}.{}", ident(&f.name))))
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .flatten()
                .collect(),
            SemanticType::FixedArray { element, length } => (0..*length)
                .map(|i| self.result_leaves(element, &format!("{access}[{i}]")))
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .flatten()
                .collect(),
            SemanticType::Scalar { .. } | SemanticType::Handle { .. } => vec![access.into()],
            SemanticType::Enum { .. } => vec![format!("{access} as i32")],
            SemanticType::String => vec![access.into()],
            _ => bail!("callin result {ty:?}"),
        })
    }
}

fn replace_handler(
    tokens: TokenStream,
    replacement: &mut impl FnMut(TokenStream) -> Result<TokenStream>,
) -> Result<TokenStream> {
    let tokens = tokens.into_iter().collect::<Vec<_>>();
    let mut result = TokenStream::new();
    let mut i = 0;
    while i < tokens.len() {
        if i + 2 < tokens.len()
            && matches!(&tokens[i],TokenTree::Punct(p) if p.as_char()=='$')
            && matches!(&tokens[i+1],TokenTree::Ident(n) if n=="handler")
            && let TokenTree::Group(g) = &tokens[i + 2]
            && g.delimiter() == Delimiter::Parenthesis
        {
            result.extend(replacement(g.stream())?);
            i += 3;
            continue;
        }
        let token = match &tokens[i] {
            TokenTree::Group(g) => TokenTree::Group(Group::new(
                g.delimiter(),
                replace_handler(g.stream(), replacement)?,
            )),
            other => other.clone(),
        };
        result.extend([token]);
        i += 1;
    }
    Ok(result)
}

pub fn render(model: &ApiModel) -> Result<(String, String, String)> {
    let builder = Builder {
        records: model
            .modules
            .iter()
            .flat_map(|m| &m.records)
            .map(|r| (r.name.clone(), r))
            .collect(),
        model,
    };
    let sources = [
        crate::render_core_wasm_callin_exec::render_scoped_rust(model),
        crate::render_core_wasm_callin_scratch::render_rust(model),
        specialized_macros().into(),
    ];
    let mut output = String::from(
        "/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */\n// @generated by spring-api-codegen; do not edit.\n",
    );
    let mut emitted = BTreeSet::new();
    let keyword = regex::Regex::new(r"\btype\b").unwrap();
    for source in sources {
        let source = keyword.replace_all(&source, "type_");
        for item in syn::parse_file(&source)?.items {
            let syn::Item::Macro(item) = item else {
                continue;
            };
            let Some(name) = item.ident else { continue };
            let macro_name = name.to_string();
            let Some((callin, native_name)) = model
                .callins
                .iter()
                .flat_map(|c| {
                    std::iter::once((c, c.name.as_str()))
                        .chain(c.aliases.iter().map(move |alias| (c, alias.as_str())))
                })
                .find(|(_, name)| macro_name == format!("export_{}", name.to_snake_case()))
            else {
                continue;
            };
            if matches!(callin.name.as_str(), "InitializeNativeModule" | "Shutdown") {
                continue;
            }
            let string_result = callin.aggregation == "first-non-empty";
            let result_record = builder.records[&callin.result];
            let environment_mask = callin
                .environments
                .iter()
                .map(|e| 1u32 << (*e as u32))
                .sum::<u32>();
            let arguments = builder
                .arguments(&callin.query, "query")
                .iter()
                .map(|(access, ty)| builder.parameter(access, ty))
                .collect::<Result<Vec<_>>>()?;
            let parameters = arguments
                .iter()
                .map(|(ty, _)| ty.clone())
                .collect::<Vec<_>>();
            let invoke_args = arguments
                .iter()
                .map(|(_, value)| format!(",{value}"))
                .collect::<String>();
            let result_type = builder.public_result(result_record)?;
            let pack_result = builder.pack_result(result_record);
            let query_name = if arguments.is_empty() { "_" } else { "query" };
            let mut replace = |tokens: TokenStream| -> Result<TokenStream> {
                let args =
                    syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated
                        .parse2(tokens.clone())
                        .with_context(|| format!("{native_name} handler args: {tokens}"))?;
                let mut args = args
                    .iter()
                    .map(|e| e.to_token_stream().to_string())
                    .collect::<VecDeque<_>>();
                let buffer = if string_result {
                    Some(args.pop_back().context("string output buffer")?)
                } else {
                    None
                };
                let query = builder.record(&callin.query, &mut args)?;
                if !args.is_empty() {
                    bail!("{native_name}: {} leftover query args", args.len());
                }
                let invoke = format!(
                    "let value:$crate::types::callins::{result_type}=$crate::module::wasm_boundary($slot.with(|slot|slot.invoke_for_environment::<{environment_mask},_>(|module,spring| {{let {query_name}={query}; let returned=$handler(module,spring{invoke_args})?; Ok({pack_result})}}))); ",
                    result_type = callin.result
                );
                let expr = if let Some(buffer) = buffer {
                    let field = ident(&result_record.fields[0].name);
                    format!(
                        "{{{invoke} let bytes=value.{field}.as_bytes(); if bytes.contains(&0) || bytes.len()>({buffer}).len() {{usize::MAX}} else {{({buffer})[..bytes.len()].copy_from_slice(bytes); bytes.len()}}}}"
                    )
                } else {
                    let leaves = if callin.aggregation == "ignore" {
                        Vec::new()
                    } else {
                        result_record
                            .fields
                            .iter()
                            .map(|f| {
                                builder.result_leaves(&f.ty, &format!("value.{}", ident(&f.name)))
                            })
                            .collect::<Result<Vec<_>>>()?
                            .into_iter()
                            .flatten()
                            .collect::<Vec<_>>()
                    };
                    let value = match leaves.as_slice() {
                        [] => "{let _=value;}".into(),
                        [only] => only.clone(),
                        _ => format!("({})", leaves.join(",")),
                    };
                    format!("{{{invoke}{value}}}")
                };
                expr.parse()
                    .map_err(|error| anyhow::anyhow!("callin replacement: {error}"))
            };
            let transformed = replace_handler(item.mac.tokens, &mut replace)?
                .to_string()
                .replace("$ handler : path", "$ slot : ident , $ handler : path")
                .replace(
                    "$ crate :: generated :: __core_callin_scratch_wire",
                    "$ crate :: __callin_wire",
                );
            // Add native C exports to the same macro expansion. String-buffer
            // capacity is only a Wasm transport concern.
            let native = format!(
                "#[cfg(not(target_arch=\"wasm32\"))] #[unsafe(no_mangle)] pub unsafe extern \"C\" fn {native_name}(_interface:*const $crate::__sys::NativeInterface,data:*mut core::ffi::c_void,query:*const $crate::__sys::{query},result:*mut $crate::__sys::{result}) {{ let Some(result)=(unsafe {{result.as_mut()}}) else {{return;}}; *result=Default::default(); result.error=$crate::module::error_pointer(unsafe {{$slot.with_host(|slot| {{slot.validate_identity(data)?; slot.invoke_for_environment::<{environment_mask},_>(|module,spring| {{let {query_name}=unsafe {{$crate::__native_callins::decode_{decode}(query)}}?;let returned=$handler(module,spring{invoke_args})?;let value={pack_result};$crate::__native_callins::encode_{encode}(&value,result)}})}})}}); }}",
                query = callin.query,
                result = callin.result,
                decode = ident(&callin.query),
                encode = ident(&callin.result)
            );
            // Insert native export before the closing expansion, preserving the
            // original transport macro's pattern and generated codec body.
            let mut tokens = transformed
                .parse::<TokenStream>()
                .map_err(|error| anyhow::anyhow!("callin tokens: {error}"))?
                .into_iter()
                .collect::<Vec<_>>();
            let Some(TokenTree::Group(expansion)) = tokens.get_mut(3) else {
                bail!("callin macro expansion {macro_name}")
            };
            let mut body = expansion.stream();
            body.extend(
                native
                    .parse::<TokenStream>()
                    .map_err(|error| anyhow::anyhow!("native callin tokens: {error}"))?,
            );
            *expansion = Group::new(Delimiter::Brace, body);
            let signature = parameters
                .iter()
                .map(|p| format!(", {p}"))
                .collect::<String>();
            output.push_str(&format!("/// Shared scoped `{native_name}` handler: `(&Module, &mut Spring{signature}) -> Result<{result_type}>`.\n#[macro_export]\nmacro_rules! {macro_name} {{ {tokens} }}\n",tokens=tokens.into_iter().collect::<TokenStream>()));
            emitted.insert(native_name.to_owned());
        }
    }
    let missing = model
        .callins
        .iter()
        .filter(|c| {
            !matches!(c.name.as_str(), "InitializeNativeModule" | "Shutdown")
                && !emitted.contains(&c.name)
        })
        .map(|c| c.name.clone())
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        bail!("scoped callin adapters missing: {}", missing.join(", "));
    }
    let mut smoke = String::from(
        "/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */\n// @generated by spring-api-codegen; do not edit.\n",
    );
    for environment in crate::model::Environment::ALL {
        if environment == crate::model::Environment::Menu {
            continue;
        }
        let ty = format!("{environment:?}");
        let slot = format!("MODULE_{}", ty.to_snake_case().to_uppercase());
        smoke.push_str(&format!("#[cfg(target_arch=\"wasm32\")] std::thread_local!{{static {slot}:spring_sdk::module::ModuleSlot<spring_sdk::{ty},Fixture>=const{{spring_sdk::module::ModuleSlot::new()}};}}\n#[cfg(not(target_arch=\"wasm32\"))] #[allow(dead_code)] static {slot}:spring_sdk::module::NativeSlot<spring_sdk::{ty},Fixture>=const{{spring_sdk::module::NativeSlot::new()}};\n"));
    }
    for name in &emitted {
        let callin = model
            .callins
            .iter()
            .find(|c| &c.name == name || c.aliases.contains(name))
            .unwrap();
        let capacity = if callin.aggregation == "first-non-empty" {
            ",4096"
        } else {
            ""
        };
        let arguments = builder
            .arguments(&callin.query, "query")
            .iter()
            .map(|(access, ty)| builder.parameter(access, ty))
            .collect::<Result<Vec<_>>>()?;
        let parameters = arguments
            .iter()
            .enumerate()
            .map(|(i, (ty, _))| format!(",_arg{i}:{}", ty.replace("$crate", "spring_sdk")))
            .collect::<String>();
        let result = builder
            .public_result(builder.records[&callin.result])?
            .replace("$crate", "spring_sdk");
        let environment = if callin
            .environments
            .contains(&crate::model::Environment::Menu)
        {
            crate::model::Environment::Menu
        } else {
            *callin.environments.first().context("callin environment")?
        };
        let environment = format!("{environment:?}");
        let slot = if environment == "Menu" {
            "MODULE".into()
        } else {
            format!("MODULE_{}", environment.to_snake_case().to_uppercase())
        };
        let handler = format!("handler_{}", name.to_snake_case());
        let hook = match callin.name.as_str() {
            "Update" => "record_update(module,_arg0);",
            "ActivateMenu" => "record_message(module,_arg0);",
            _ => "",
        };
        let value = match callin.name.as_str() {
            "AllowDraw" => "true",
            "ScreenTooltip" => "String::from(\"tooltip 日本語\")",
            _ if result == "()" => "()",
            _ => "Default::default()",
        };
        smoke.push_str(&format!(
            "fn {handler}(module:&Fixture,_spring:&mut spring_sdk::Spring<'_,spring_sdk::{environment}>{parameters})->spring_sdk::Result<{result}> {{record_call(module);{hook}Ok({value})}}\n\
             spring_sdk::export_{}!({slot},{handler}{capacity});\n",
            name.to_snake_case()
        ));
    }
    let coverage = serde_json::json!({"generated":emitted,"remaining_specialized":missing});
    Ok((
        spring_native_codegen::render_sdk::formatted(&output)?,
        serde_json::to_string_pretty(&coverage)? + "\n",
        smoke,
    ))
}

fn specialized_macros() -> &'static str {
    r#"
#[macro_export]
macro_rules! export_draw_world {
    ($handler:path)=>{
        #[cfg(target_arch="wasm32")]
        #[unsafe(export_name="spring:callin/draw-world")]
        pub extern "C" fn __spring_wasm_core_callin_draw_world() {
            $handler(0u8);
        }
    };
}
#[macro_export]
macro_rules! export_allow_unit_creation {
    ($handler:path)=>{
        #[cfg(target_arch="wasm32")]
        #[unsafe(export_name="spring:callin/allow-unit-creation")]
        pub extern "C" fn __spring_sdk_allow_unit_creation(unit_def_id:i32,builder_id:i32,builder_team:i32,has_build_info:i32,build_x:f32,build_y:f32,build_z:f32,build_facing:i32)->i32 {
            let (allow,drop_order)=$handler(unit_def_id,builder_id,builder_team,has_build_info!=0,build_x,build_y,build_z,build_facing);
            (allow as i32)|((drop_order as i32)<<1)
        }
    };
}
#[macro_export]
macro_rules! export_add_console_line {
    ($handler:path)=>{
        #[cfg(target_arch="wasm32")]
        #[unsafe(export_name="spring:callin/add-console-line")]
        pub extern "C" fn __spring_sdk_add_console_line(used_bytes:i32)->i64 {
            let bytes=__spring_core_callin_scratch_view(used_bytes).unwrap_or_else(||core::arch::wasm32::unreachable());
            let (message,section,level)=$crate::module::console_payload(bytes).unwrap_or_else(|_|core::arch::wasm32::unreachable());
            ($handler(message,section,level)) as i64
        }
    };
}
#[macro_export]
macro_rules! export_command_notify {
    ($handler:path)=>{
        #[cfg(target_arch="wasm32")]
        #[unsafe(export_name="spring:callin/command-notify")]
        pub extern "C" fn __spring_sdk_command_notify(used_bytes:i32)->i64 {
            let bytes=__spring_core_callin_scratch_view(used_bytes).unwrap_or_else(||core::arch::wasm32::unreachable());
            let (id,time_out,page_index,tag,options,params)=$crate::module::command_payload(bytes).unwrap_or_else(|_|core::arch::wasm32::unreachable());
            ($handler(id,time_out,page_index,tag,options,&params)) as i64
        }
    };
}
"#
}
