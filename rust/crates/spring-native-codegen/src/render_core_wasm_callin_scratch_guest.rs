//! Guest-side generation for shared-scratch Core callins.

use heck::{ToKebabCase, ToSnakeCase};
use std::collections::BTreeMap;

use crate::model::{ApiModel, FieldModel, RecordModel, SemanticType};

use super::{executable, record_index, result, ScratchCallin};

#[path = "render_core_wasm_callin_scratch_guest_wire.rs"]
mod wire;

pub(super) fn render_rust(model: &ApiModel) -> String {
    let records = record_index(model);
    let callins = executable(model, &records);
    let mut macros = String::new();
    for callin in &callins {
        macros.push_str(&render_rust_macro(callin, &records));
        macros.push('\n');
    }
    wire::render(&macros, callins.len())
}

fn render_rust_macro(
    callin: &ScratchCallin<'_>,
    records: &BTreeMap<String, RecordModel>,
) -> String {
    let macro_name = format!("export_{}", callin.entry.name.to_snake_case());
    let function_name = format!(
        "__spring_wasm_core_callin_{}",
        callin.entry.name.to_snake_case()
    );
    let export_name = format!("spring:callin/{}", callin.entry.name.to_kebab_case());
    let mut declarations = String::new();
    let mut args = Vec::new();
    render_rust_fields(
        &callin.query.fields,
        "",
        records,
        &mut declarations,
        &mut args,
    );
    let handler = format!("$handler({})", args.join(", "));
    let shape = result::classify(callin.result, records).expect("executable scratch result");
    let (result_type, result_expr, result_doc) = match shape {
        result::ResultShape::Empty => (String::new(), format!("{handler};"), ""),
        result::ResultShape::Direct(field) => (
            format!(" -> {}", rust_raw_type(&field.ty)),
            rust_return_expr(&field.ty, &handler),
            "",
        ),
        result::ResultShape::PackedFixed(leaves) => {
            let packed = leaves
                .iter()
                .enumerate()
                .map(|(index, leaf)| {
                    let value = if leaves.len() == 1 {
                        "value".to_owned()
                    } else {
                        format!("value.{index}")
                    };
                    let encoded = rust_pack32_expr(&leaf.ty, &value);
                    if index == 0 {
                        format!("({encoded} as u64)")
                    } else {
                        format!("(({encoded} as u64) << {}u32)", index * 32)
                    }
                })
                .collect::<Vec<_>>()
                .join(" | ");
            (
                " -> i64".to_owned(),
                format!("let value = {handler};\n            ({packed}) as i64"),
                if leaves.len() == 2 {
                    " The handler returns a tuple matching flattened result declaration order."
                } else {
                    " The handler returns the single flattened result leaf."
                },
            )
        }
    };

    format!(
        "/// Export `{native}` through the shared-scratch Core ABI.{result_doc}\n#[macro_export]\nmacro_rules! {macro_name} {{\n    ($handler:path) => {{\n        #[cfg(target_arch = \"wasm32\")]\n        #[export_name = \"{export_name}\"]\n        pub extern \"C\" fn {function_name}(used_bytes: i32){result_type} {{\n            let bytes = match $crate::__spring_core_callin_scratch_view(used_bytes) {{ Some(value) => value, None => core::arch::wasm32::unreachable() }};\n            let mut reader = $crate::generated::__core_callin_scratch_wire::Reader::new(bytes);\n{declarations}            if !reader.finish() {{ core::arch::wasm32::unreachable(); }}\n            {result_expr}\n        }}\n    }};\n}}\n",
        native = callin.entry.name,
        result_doc = result_doc,
        macro_name = macro_name,
        export_name = export_name,
        function_name = function_name,
        result_type = result_type,
        declarations = declarations,
        result_expr = result_expr,
    )
}

fn render_rust_fields(
    fields: &[FieldModel],
    prefix: &str,
    records: &BTreeMap<String, RecordModel>,
    declarations: &mut String,
    args: &mut Vec<String>,
) {
    let implicit = super::implicit_count_fields(fields);
    for field in fields
        .iter()
        .filter(|field| !implicit.contains(&field.name))
    {
        render_rust_type(
            &field.ty,
            field,
            &format!("{prefix}{}", field.name.to_snake_case()),
            records,
            declarations,
            args,
        );
    }
}

fn render_rust_type(
    ty: &SemanticType,
    field: &FieldModel,
    name: &str,
    records: &BTreeMap<String, RecordModel>,
    declarations: &mut String,
    args: &mut Vec<String>,
) {
    match ty {
        SemanticType::Scalar { name: scalar_name } => {
            let read = rust_reader_method(scalar_name);
            declarations.push_str(&format!(
                "            let {name} = match reader.{read}() {{ Some(value) => value, None => core::arch::wasm32::unreachable() }};\n"
            ));
            args.push(rust_handler_arg(ty, name));
        }
        SemanticType::Enum { .. } => {
            declarations.push_str(&format!(
                "            let {name} = match reader.i32() {{ Some(value) => value, None => core::arch::wasm32::unreachable() }};\n"
            ));
            args.push(name.to_owned());
        }
        SemanticType::Handle { .. } => {
            declarations.push_str(&format!(
                "            let {name} = match reader.u64() {{ Some(value) => value, None => core::arch::wasm32::unreachable() }};\n"
            ));
            args.push(format!("{name} as u64"));
        }
        SemanticType::String | SemanticType::Bytes => {
            declarations.push_str(&format!(
                "            let {name} = match reader.bytes() {{ Some(value) => value, None => core::arch::wasm32::unreachable() }};\n"
            ));
            args.push(name.to_owned());
        }
        SemanticType::List { element } => match element.as_ref() {
            SemanticType::Record { name: record_name } => {
                let record = &records[record_name];
                let frames = format!("{name}_frames");
                let frame = format!("{name}_frame");
                let mut item_declarations = String::new();
                let mut item_args = Vec::new();
                render_rust_fields(
                    &record.fields,
                    &format!("{name}_item_"),
                    records,
                    &mut item_declarations,
                    &mut item_args,
                );
                let item = rust_tuple(&item_args);
                declarations.push_str(&format!(
                    "            let mut {frames} = match reader.framed_list() {{ Some(value) => value, None => core::arch::wasm32::unreachable() }};\n            let {name} = core::iter::from_fn(move || {{\n                if {frames}.remaining() == 0 {{ return None; }}\n                let {frame} = match {frames}.next_frame() {{ Some(value) => value, None => core::arch::wasm32::unreachable() }};\n                let mut reader = $crate::generated::__core_callin_scratch_wire::Reader::new({frame});\n{item_declarations}                if !reader.finish() {{ core::arch::wasm32::unreachable(); }}\n                Some({item})\n            }});\n"
                ));
                args.push(name.to_owned());
            }
            _ => {
                let rust = rust_list_type(element);
                declarations.push_str(&format!(
                    "            let {name}: &[{rust}] = match reader.slice::<{rust}>() {{ Some(value) => value, None => core::arch::wasm32::unreachable() }};\n"
                ));
                args.push(name.to_owned());
            }
        },
        SemanticType::Record { name: record_name } => {
            let record = &records[record_name];
            render_rust_fields(
                &record.fields,
                &format!("{name}_"),
                records,
                declarations,
                args,
            );
        }
        SemanticType::FixedArray { element, length } => {
            for index in 0..*length {
                render_rust_type(
                    element,
                    field,
                    &format!("{name}_{index}"),
                    records,
                    declarations,
                    args,
                );
            }
        }
        _ => unreachable!(),
    }
}

fn rust_tuple(args: &[String]) -> String {
    match args {
        [] => "()".to_owned(),
        [only] => format!("({only},)"),
        _ => format!("({})", args.join(", ")),
    }
}

fn rust_reader_method(name: &str) -> &'static str {
    match name {
        "bool" => "boolean",
        "f32" => "f32",
        "f64" => "f64",
        "i64" | "isize" => "i64",
        "u64" | "usize" => "u64",
        "i8" | "i16" | "i32" => "i32",
        "u8" | "char" | "u16" | "u32" => "u32",
        _ => "i32",
    }
}

fn rust_handler_arg(ty: &SemanticType, name: &str) -> String {
    match ty {
        SemanticType::Scalar { name: scalar } => match scalar.as_str() {
            "bool" | "f32" | "f64" | "i32" | "i64" => name.to_owned(),
            "i8" => format!("{name} as i8"),
            "i16" => format!("{name} as i16"),
            "u8" | "char" => format!("{name} as u8"),
            "u16" => format!("{name} as u16"),
            "u32" => format!("{name} as u32"),
            "u64" => format!("{name} as u64"),
            "isize" => format!("{name} as isize"),
            "usize" => format!("{name} as usize"),
            _ => name.to_owned(),
        },
        _ => name.to_owned(),
    }
}

fn rust_raw_type(ty: &SemanticType) -> &'static str {
    match ty {
        SemanticType::Scalar { name } => match name.as_str() {
            "f32" => "f32",
            "f64" => "f64",
            "i64" | "u64" | "isize" | "usize" => "i64",
            _ => "i32",
        },
        SemanticType::Enum { .. } => "i32",
        SemanticType::Handle { .. } => "i64",
        _ => unreachable!(),
    }
}

fn rust_return_expr(ty: &SemanticType, value: &str) -> String {
    match ty {
        SemanticType::Scalar { name } => match name.as_str() {
            "bool" => format!("if {value} {{ 1 }} else {{ 0 }}"),
            "f32" | "f64" | "i32" | "i64" => value.to_owned(),
            "i8" | "i16" | "u8" | "char" | "u16" | "u32" => format!("{value} as i32"),
            "u64" | "isize" | "usize" => format!("{value} as i64"),
            _ => value.to_owned(),
        },
        SemanticType::Enum { .. } => format!("{value} as i32"),
        SemanticType::Handle { .. } => format!("{value} as i64"),
        _ => unreachable!(),
    }
}

fn rust_pack32_expr(ty: &SemanticType, value: &str) -> String {
    match ty {
        SemanticType::Scalar { name } if name == "bool" => {
            format!("if {value} {{ 1u32 }} else {{ 0u32 }}")
        }
        SemanticType::Scalar { name } if name == "f32" => format!("{value}.to_bits()"),
        SemanticType::Scalar { .. } | SemanticType::Enum { .. } => format!("{value} as u32"),
        _ => unreachable!(),
    }
}

fn rust_list_type(ty: &SemanticType) -> String {
    match ty {
        SemanticType::Scalar { name } => match name.as_str() {
            "i8" => "i8",
            "i16" => "i16",
            "i32" => "i32",
            "i64" => "i64",
            "u8" | "char" => "u8",
            "u16" => "u16",
            "u32" => "u32",
            "u64" => "u64",
            "f32" => "f32",
            "f64" => "f64",
            other => panic!("unsupported scratch list scalar {other}"),
        }
        .to_owned(),
        SemanticType::Enum { .. } => "i32".to_owned(),
        _ => unreachable!(),
    }
}
