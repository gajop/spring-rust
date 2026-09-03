//! Shared Core-Wasm wire and variable-input lowering helpers.
//!
//! These helpers define the canonical generated wire layout used by the
//! variable-input, variable-output, and dynamic-output renderers. Keeping the
//! layout decisions in one module prevents transport classes from drifting.

use std::collections::BTreeMap;

use crate::model::{FieldModel, RecordModel, SemanticType};

pub(crate) fn direct_type(ty: &SemanticType) -> bool {
    matches!(
        ty,
        SemanticType::Scalar { .. } | SemanticType::Enum { .. } | SemanticType::Handle { .. }
    )
}

/// True for `option<string>`: a presence flag followed by the same
/// pointer/length pair a plain string input uses.
pub(crate) fn optional_string(field: &FieldModel) -> bool {
    matches!(&field.ty, SemanticType::Option { inner } if matches!(inner.as_ref(), SemanticType::String))
        && presence_field(field).is_some()
}

pub(crate) fn input_field_supported(
    field: &FieldModel,
    records: &BTreeMap<String, RecordModel>,
) -> bool {
    if direct_type(&field.ty) || fixed_wire_field(field, records) || optional_string(field) {
        return true;
    }
    match &field.ty {
        SemanticType::String => true,
        SemanticType::Bytes => count_field(field).is_some(),
        SemanticType::List { element } => {
            count_field(field).is_some() && input_list_element_supported(element, records)
        }
        _ => false,
    }
}

fn input_list_element_supported(
    ty: &SemanticType,
    records: &BTreeMap<String, RecordModel>,
) -> bool {
    match ty {
        SemanticType::String => true,
        SemanticType::Scalar { name } if !matches!(name.as_str(), "bool" | "isize" | "usize") => {
            true
        }
        SemanticType::Enum { .. } => true,
        SemanticType::Record { .. } => fixed_wire_type(ty, records),
        _ => false,
    }
}

pub(crate) fn borrowed_list_element(ty: &SemanticType) -> bool {
    match ty {
        SemanticType::Scalar { name } => {
            matches!(name.as_str(), "i32" | "u32" | "f32" | "i64" | "u64" | "f64")
        }
        SemanticType::Enum { .. } => true,
        _ => false,
    }
}

pub(crate) fn borrowed_element_bytes(ty: &SemanticType) -> u32 {
    match ty {
        SemanticType::Scalar { name } => match name.as_str() {
            "i64" | "u64" | "f64" => 8,
            "i32" | "u32" | "f32" => 4,
            _ => unreachable!("non-borrowable scalar element"),
        },
        SemanticType::Enum { .. } => 4,
        _ => unreachable!("non-borrowable variable list element"),
    }
}

pub(crate) fn variable_type(ty: &SemanticType) -> bool {
    matches!(
        ty,
        SemanticType::String | SemanticType::Bytes | SemanticType::List { .. }
    )
}

/// Field-aware fixed-wire test. An `option<T>` is fixed-size on the wire -- a
/// u32 presence flag followed by an always-reserved payload -- but only the
/// field carries the presence metadata, so options can only be recognised here.
pub(crate) fn fixed_wire_field(
    field: &FieldModel,
    records: &BTreeMap<String, RecordModel>,
) -> bool {
    match &field.ty {
        SemanticType::Option { inner } => {
            presence_field(field).is_some() && fixed_wire_type(inner, records)
        }
        other => fixed_wire_type(other, records),
    }
}

pub(crate) fn fixed_wire_type(ty: &SemanticType, records: &BTreeMap<String, RecordModel>) -> bool {
    match ty {
        SemanticType::Scalar { .. } | SemanticType::Enum { .. } | SemanticType::Handle { .. } => {
            true
        }
        SemanticType::FixedArray { element, length } => {
            *length <= 64 && fixed_wire_type(element, records)
        }
        SemanticType::Record { name } => records.get(name).is_some_and(|record| {
            record
                .fields
                .iter()
                .all(|field| fixed_wire_field(field, records))
        }),
        _ => false,
    }
}

pub(crate) fn presence_field(field: &FieldModel) -> Option<String> {
    field.metadata.iter().find_map(|metadata| {
        metadata
            .strip_prefix("presence-field:")
            .map(ToString::to_string)
    })
}

/// True when a record needs an explicit trailing alignment on the wire because
/// one of its fields is an option. Records without options keep the historical
/// encoding untouched.
pub(crate) fn record_has_option(
    ty: &SemanticType,
    records: &BTreeMap<String, RecordModel>,
) -> bool {
    match ty {
        SemanticType::Record { name } => records.get(name).is_some_and(|record| {
            record.fields.iter().any(|field| {
                matches!(field.ty, SemanticType::Option { .. })
                    || record_has_option(&field.ty, records)
            })
        }),
        SemanticType::FixedArray { element, .. } => record_has_option(element, records),
        _ => false,
    }
}

pub(crate) fn fixed_wire_layout(
    ty: &SemanticType,
    records: &BTreeMap<String, RecordModel>,
) -> Option<(u32, u32)> {
    match ty {
        SemanticType::Scalar { name } => match name.as_str() {
            "i64" | "u64" | "isize" | "usize" | "f64" => Some((8, 8)),
            _ => Some((4, 4)),
        },
        SemanticType::Enum { .. } => Some((4, 4)),
        SemanticType::Handle { .. } => Some((8, 8)),
        SemanticType::FixedArray { element, length } => {
            let (bytes, alignment) = fixed_wire_layout(element, records)?;
            Some((bytes.checked_mul(u32::try_from(*length).ok()?)?, alignment))
        }
        SemanticType::Record { name } => layout_fields(&records.get(name)?.fields, records),
        // Presence flag, then the payload at its own alignment, then the pair
        // padded so an array/record of options keeps a constant stride.
        SemanticType::Option { inner } => {
            let (payload_bytes, payload_alignment) = fixed_wire_layout(inner, records)?;
            let alignment = payload_alignment.max(4);
            let payload_offset = align_up(4, payload_alignment);
            Some((
                align_up(payload_offset.checked_add(payload_bytes)?, alignment),
                alignment,
            ))
        }
        _ => None,
    }
}

pub(crate) fn layout_fields(
    fields: &[FieldModel],
    records: &BTreeMap<String, RecordModel>,
) -> Option<(u32, u32)> {
    let mut bytes = 0u32;
    let mut alignment = 1u32;
    for field in fields {
        let (field_bytes, field_alignment) = fixed_wire_layout(&field.ty, records)?;
        bytes = align_up(bytes, field_alignment).checked_add(field_bytes)?;
        alignment = alignment.max(field_alignment);
    }
    Some((align_up(bytes, alignment), alignment))
}

fn input_descriptor_field_layout(
    field: &FieldModel,
    records: &BTreeMap<String, RecordModel>,
) -> Option<(u32, u32)> {
    if variable_type(&field.ty) {
        Some((8, 4))
    } else if optional_string(field) {
        // Presence flag plus the always-reserved pointer/length pair.
        Some((12, 4))
    } else {
        fixed_wire_layout(&field.ty, records)
    }
}

pub(crate) fn input_descriptor_layout(
    fields: &[FieldModel],
    records: &BTreeMap<String, RecordModel>,
) -> Option<(u32, u32)> {
    let mut bytes = 0u32;
    let mut alignment = 1u32;
    let mut any = false;
    for field in fields.iter().filter(|field| !direct_type(&field.ty)) {
        let (field_bytes, field_alignment) = input_descriptor_field_layout(field, records)?;
        bytes = align_up(bytes, field_alignment).checked_add(field_bytes)?;
        alignment = alignment.max(field_alignment);
        any = true;
    }
    any.then(|| (align_up(bytes, alignment), alignment))
}

#[derive(Clone, Copy)]
pub(crate) struct OutputLayout {
    pub(crate) descriptor_bytes: u32,
    pub(crate) fixed_offset: u32,
    pub(crate) fixed_bytes: u32,
    pub(crate) fixed_alignment: u32,
}

pub(crate) fn align_up(value: u32, alignment: u32) -> u32 {
    (value + alignment - 1) & !(alignment - 1)
}

pub(crate) fn render_input_variable(
    field: &FieldModel,
    records: &BTreeMap<String, RecordModel>,
) -> String {
    let stem = sanitize(&field.name);
    let mut output = format!(
        "    std::uint32_t {stem}InputPointer = 0;\n    std::uint32_t {stem}InputCount = 0;\n    if (!reader.U32({stem}InputPointer) || !reader.U32({stem}InputCount)) {{\n        slots[0].i32 = static_cast<std::int32_t>(Status::InvalidArgument);\n        return nullptr;\n    }}\n"
    );
    match &field.ty {
        // The descriptor pair locates the blob; the string inside it is
        // length-prefixed, exactly as the guest owned wrappers encode it and as
        // the dynamic-input host bindings decode it. Reading the blob as raw
        // bytes instead swallowed the four-byte prefix into the value, so every
        // lookup keyed on the string missed.
        SemanticType::String => output.push_str(&format!(
            "    std::span<const std::uint8_t> {stem}InputWire;\n    if (!state->memory.View({stem}InputPointer, {stem}InputCount, {stem}InputWire)) {{ slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds); return nullptr; }}\n    WireReader {stem}InputReader({stem}InputWire);\n    std::string {stem}InputStorage;\n    {{ std::uint32_t coreLength = 0; if (!{stem}InputReader.U32(coreLength)) {{ slots[0].i32 = static_cast<std::int32_t>(Status::InvalidArgument); return nullptr; }} std::span<const std::uint8_t> coreBytes; if (!{stem}InputReader.Bytes(coreLength, coreBytes)) {{ slots[0].i32 = static_cast<std::int32_t>(Status::InvalidArgument); return nullptr; }} {stem}InputStorage.assign(reinterpret_cast<const char*>(coreBytes.data()), coreBytes.size()); }}\n    query.{field_name} = {stem}InputStorage.c_str();\n",
            field_name = field.name,
        )),
        SemanticType::Bytes => {
            let count = count_field(field).expect("eligible bytes input count");
            output.push_str(&format!(
                "    if ({stem}InputCount == 0) {{\n        query.{field_name} = nullptr;\n    }} else {{\n        std::span<const std::uint8_t> {stem}InputBytes;\n        if (!state->memory.View({stem}InputPointer, {stem}InputCount, {stem}InputBytes)) {{ slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds); return nullptr; }}\n        query.{field_name} = reinterpret_cast<std::remove_reference_t<decltype(query.{field_name})>>({stem}InputBytes.data());\n    }}\n    if (!AssignCoreCount({stem}InputCount, query.{count})) {{ slots[0].i32 = static_cast<std::int32_t>(Status::InvalidArgument); return nullptr; }}\n",
                field_name = field.name,
            ));
        }
        SemanticType::List { element } if matches!(element.as_ref(), SemanticType::String) => {
            let count = count_field(field).expect("eligible string list input count");
            output.push_str(&format!(
                "    const std::uint64_t {stem}InputDescriptorBytes = static_cast<std::uint64_t>({stem}InputCount) * 8u;\n    if ({stem}InputDescriptorBytes > std::numeric_limits<std::size_t>::max()) {{ slots[0].i32 = static_cast<std::int32_t>(Status::InvalidArgument); return nullptr; }}\n    std::span<const std::uint8_t> {stem}InputWire;\n    if (!state->memory.View({stem}InputPointer, static_cast<std::size_t>({stem}InputDescriptorBytes), {stem}InputWire)) {{ slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds); return nullptr; }}\n    WireReader {stem}InputReader({stem}InputWire);\n    std::vector<std::string> {stem}InputStrings;\n    std::vector<const char*> {stem}InputPointers;\n    {stem}InputStrings.reserve({stem}InputCount);\n    for (std::uint32_t coreIndex = 0; coreIndex < {stem}InputCount; ++coreIndex) {{\n        std::uint32_t itemPointer = 0, itemLength = 0;\n        if (!{stem}InputReader.U32(itemPointer) || !{stem}InputReader.U32(itemLength)) {{ slots[0].i32 = static_cast<std::int32_t>(Status::InvalidArgument); return nullptr; }}\n        std::span<const std::uint8_t> itemBytes;\n        if (!state->memory.View(itemPointer, itemLength, itemBytes)) {{ slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds); return nullptr; }}\n        {stem}InputStrings.emplace_back(reinterpret_cast<const char*>(itemBytes.data()), itemBytes.size());\n    }}\n    {stem}InputPointers.reserve({stem}InputStrings.size());\n    for (const auto& item : {stem}InputStrings) {stem}InputPointers.push_back(item.c_str());\n    query.{field_name} = {stem}InputPointers.data();\n    if (!AssignCoreCount({stem}InputCount, query.{count})) {{ slots[0].i32 = static_cast<std::int32_t>(Status::InvalidArgument); return nullptr; }}\n",
                field_name = field.name,
            ));
        }
        SemanticType::List { element } if borrowed_list_element(element) => {
            let count = count_field(field).expect("eligible borrowed list input count");
            let cpp = native_cpp_type(element);
            let bytes = borrowed_element_bytes(element);
            let alignment = bytes.min(8);
            output.push_str(&format!(
                "    if ({stem}InputCount == 0) {{\n        query.{field_name} = nullptr;\n    }} else {{\n        if constexpr (std::endian::native != std::endian::little) {{ slots[0].i32 = static_cast<std::int32_t>(Status::NotAvailable); return nullptr; }}\n        const std::uint64_t {stem}InputBytes64 = static_cast<std::uint64_t>({stem}InputCount) * {bytes}u;\n        if ({stem}InputBytes64 > std::numeric_limits<std::size_t>::max() || ({stem}InputPointer % {alignment}u) != 0u) {{ slots[0].i32 = static_cast<std::int32_t>(Status::InvalidArgument); return nullptr; }}\n        std::span<const std::uint8_t> {stem}InputBytes;\n        if (!state->memory.View({stem}InputPointer, static_cast<std::size_t>({stem}InputBytes64), {stem}InputBytes)) {{ slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds); return nullptr; }}\n        static_assert(sizeof({cpp}) == {bytes}u, \"generated Core borrowed/native element width mismatch\");\n        query.{field_name} = reinterpret_cast<std::remove_reference_t<decltype(query.{field_name})>>({stem}InputBytes.data());\n    }}\n    if (!AssignCoreCount({stem}InputCount, query.{count})) {{ slots[0].i32 = static_cast<std::int32_t>(Status::InvalidArgument); return nullptr; }}\n",
                field_name = field.name,
            ));
        }
        SemanticType::List { element } => {
            let count = count_field(field).expect("eligible adapted list input count");
            let (element_bytes, element_alignment) = fixed_wire_layout(element, records).expect("eligible list input element");
            let cpp = native_cpp_type(element);
            let item_read = render_wire_read(element, "item", records, &format!("{stem}InputReader"), 2);
            output.push_str(&format!(
                "    const std::uint64_t {stem}InputBytes64 = static_cast<std::uint64_t>({stem}InputCount) * {element_bytes}u;\n    if ({stem}InputBytes64 > std::numeric_limits<std::size_t>::max()) {{ slots[0].i32 = static_cast<std::int32_t>(Status::InvalidArgument); return nullptr; }}\n    std::span<const std::uint8_t> {stem}InputWire;\n    if (!state->memory.View({stem}InputPointer, static_cast<std::size_t>({stem}InputBytes64), {stem}InputWire)) {{ slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds); return nullptr; }}\n    WireReader {stem}InputReader({stem}InputWire);\n    std::vector<{cpp}> {stem}InputStorage;\n    {stem}InputStorage.reserve({stem}InputCount);\n    for (std::uint32_t coreIndex = 0; coreIndex < {stem}InputCount; ++coreIndex) {{\n        {cpp} item{{}};\n{item_read}        {stem}InputStorage.push_back(item);\n    }}\n    if (!{stem}InputReader.Finish({element_alignment}u)) {{ slots[0].i32 = static_cast<std::int32_t>(Status::InvalidArgument); return nullptr; }}\n    query.{field_name} = {stem}InputStorage.data();\n    if (!AssignCoreCount({stem}InputCount, query.{count})) {{ slots[0].i32 = static_cast<std::int32_t>(Status::InvalidArgument); return nullptr; }}\n",
                field_name = field.name,
            ));
        }
        _ => unreachable!(),
    }
    output
}

pub(crate) fn native_cast(destination: &str, value: &str) -> String {
    format!(
        "static_cast<std::remove_cv_t<std::remove_reference_t<decltype({destination})>>>({value})"
    )
}

pub(crate) fn render_wire_read_field(
    field: &FieldModel,
    destination: &str,
    owner: &str,
    records: &BTreeMap<String, RecordModel>,
    reader: &str,
    indent: usize,
) -> String {
    let pad = "    ".repeat(indent);
    match &field.ty {
        SemanticType::Option { inner } => {
            let presence = presence_field(field).expect("eligible option has presence metadata");
            // Scope the temporary: one function can read several options.
            let mut output = format!(
                "{pad}{{\n{pad}    bool corePresent = false;\n{pad}    if (!{reader}.Bool(corePresent)) return Trap(\"generated Core option presence underflow\");\n{pad}    {owner}.{presence} = corePresent;\n{pad}}}\n"
            );
            output.push_str(&render_wire_read(
                inner,
                destination,
                records,
                reader,
                indent,
            ));
            output
        }
        other => render_wire_read(other, destination, records, reader, indent),
    }
}

pub(crate) fn render_wire_read(
    ty: &SemanticType,
    destination: &str,
    records: &BTreeMap<String, RecordModel>,
    reader: &str,
    indent: usize,
) -> String {
    let pad = "    ".repeat(indent);
    match ty {
        SemanticType::Scalar { name } => match name.as_str() {
            "bool" => format!(
                "{pad}if (!{reader}.Bool({destination})) return Trap(\"generated Core wire underflow\");\n"
            ),
            "f32" => format!(
                "{pad}if (!{reader}.F32({destination})) return Trap(\"generated Core wire underflow\");\n"
            ),
            "f64" => format!(
                "{pad}if (!{reader}.F64({destination})) return Trap(\"generated Core wire underflow\");\n"
            ),
            "i64" | "isize" => scalar_read(reader, "I64", "std::int64_t", destination, &pad),
            "u64" | "usize" => scalar_read(reader, "U64", "std::uint64_t", destination, &pad),
            "i8" | "i16" | "i32" => scalar_read(reader, "I32", "std::int32_t", destination, &pad),
            _ => scalar_read(reader, "U32", "std::uint32_t", destination, &pad),
        },
        SemanticType::Enum { .. } => scalar_read(reader, "I32", "std::int32_t", destination, &pad),
        SemanticType::Handle { .. } => {
            scalar_read(reader, "U64", "std::uint64_t", destination, &pad)
        }
        SemanticType::Record { name } => {
            let mut output = records[name]
                .fields
                .iter()
                .map(|field| {
                    render_wire_read_field(
                        field,
                        &format!("{destination}.{}", field.name),
                        destination,
                        records,
                        reader,
                        indent,
                    )
                })
                .collect::<String>();
            if record_has_option(ty, records) {
                let (_, alignment) =
                    fixed_wire_layout(ty, records).expect("option record has a fixed layout");
                output.push_str(&format!(
                    "{pad}if (!{reader}.Align({alignment}u)) return Trap(\"generated Core option record alignment underflow\");\n"
                ));
            }
            output
        }
        SemanticType::FixedArray { element, length } => {
            let index = format!("coreReadIndex{indent}");
            let nested = render_wire_read(
                element,
                &format!("{destination}[{index}]"),
                records,
                reader,
                indent + 1,
            );
            format!(
                "{pad}for (std::size_t {index} = 0; {index} < {length}u; ++{index}) {{\n{nested}{pad}}}\n"
            )
        }
        _ => unreachable!(),
    }
}

fn scalar_read(reader: &str, method: &str, raw: &str, destination: &str, pad: &str) -> String {
    format!(
        "{pad}{{ {raw} coreRaw = 0; if (!{reader}.{method}(coreRaw)) return Trap(\"generated Core wire underflow\"); {destination} = {cast}; }}\n",
        cast = native_cast(destination, "coreRaw")
    )
}

pub(crate) fn render_wire_write_field(
    field: &FieldModel,
    value: &str,
    owner: &str,
    records: &BTreeMap<String, RecordModel>,
    writer: &str,
    indent: usize,
) -> String {
    let pad = "    ".repeat(indent);
    match &field.ty {
        SemanticType::Option { inner } => {
            let presence = presence_field(field).expect("eligible option has presence metadata");
            // The payload slot is always written so the record keeps a fixed
            // stride; an absent option writes a zeroed value of the same shape.
            let native = format!("std::remove_cv_t<std::remove_reference_t<decltype({value})>>");
            let mut output = format!(
                "{pad}if (!{writer}.Bool({owner}.{presence})) return Trap(\"generated Core option overflow\");\n{pad}if ({owner}.{presence}) {{\n"
            );
            output.push_str(&render_wire_write(
                inner,
                value,
                records,
                writer,
                indent + 1,
            ));
            output.push_str(&format!(
                "{pad}}} else {{\n{pad}    {native} coreAbsent{{}};\n"
            ));
            output.push_str(&render_wire_write(
                inner,
                "coreAbsent",
                records,
                writer,
                indent + 1,
            ));
            output.push_str(&format!("{pad}}}\n"));
            output
        }
        other => render_wire_write(other, value, records, writer, indent),
    }
}

pub(crate) fn render_wire_write(
    ty: &SemanticType,
    value: &str,
    records: &BTreeMap<String, RecordModel>,
    writer: &str,
    indent: usize,
) -> String {
    let pad = "    ".repeat(indent);
    match ty {
        SemanticType::Scalar { name } => {
            let method = match name.as_str() {
                "bool" => "Bool",
                "f32" => "F32",
                "f64" => "F64",
                "i64" | "isize" => "I64",
                "u64" | "usize" => "U64",
                "i8" | "i16" | "i32" => "I32",
                _ => "U32",
            };
            format!(
                "{pad}if (!{writer}.{method}({value})) return Trap(\"generated Core wire overflow\");\n"
            )
        }
        SemanticType::Enum { .. } => format!(
            "{pad}if (!{writer}.I32(static_cast<std::int32_t>({value}))) return Trap(\"generated Core wire overflow\");\n"
        ),
        SemanticType::Handle { .. } => format!(
            "{pad}if (!{writer}.U64(static_cast<std::uint64_t>({value}))) return Trap(\"generated Core wire overflow\");\n"
        ),
        SemanticType::Record { name } => {
            let mut output = records[name]
                .fields
                .iter()
                .map(|field| {
                    render_wire_write_field(
                        field,
                        &format!("{value}.{}", field.name),
                        value,
                        records,
                        writer,
                        indent,
                    )
                })
                .collect::<String>();
            if record_has_option(ty, records) {
                let (_, alignment) =
                    fixed_wire_layout(ty, records).expect("option record has a fixed layout");
                output.push_str(&format!(
                    "{pad}if (!{writer}.Align({alignment}u)) return Trap(\"generated Core option record alignment overflow\");\n"
                ));
            }
            output
        }
        SemanticType::FixedArray { element, length } => {
            let index = format!("coreWriteIndex{indent}");
            let nested = render_wire_write(
                element,
                &format!("{value}[{index}]"),
                records,
                writer,
                indent + 1,
            );
            format!(
                "{pad}for (std::size_t {index} = 0; {index} < {length}u; ++{index}) {{\n{nested}{pad}}}\n"
            )
        }
        _ => unreachable!(),
    }
}

pub(crate) fn native_cpp_type(ty: &SemanticType) -> String {
    match ty {
        SemanticType::Scalar { name } => match name.as_str() {
            "i8" => "std::int8_t".to_owned(),
            "i16" => "std::int16_t".to_owned(),
            "i32" => "std::int32_t".to_owned(),
            "i64" => "std::int64_t".to_owned(),
            "u8" | "char" => "std::uint8_t".to_owned(),
            "u16" => "std::uint16_t".to_owned(),
            "u32" => "std::uint32_t".to_owned(),
            "u64" => "std::uint64_t".to_owned(),
            "f32" => "float".to_owned(),
            "f64" => "double".to_owned(),
            _ => unreachable!("unsupported list scalar type"),
        },
        SemanticType::Enum { name } | SemanticType::Record { name } => name.clone(),
        _ => unreachable!("unsupported variable list element C++ type"),
    }
}

pub(crate) fn count_field(field: &FieldModel) -> Option<String> {
    field.metadata.iter().find_map(|metadata| {
        metadata
            .strip_prefix("count-field:")
            .map(ToString::to_string)
    })
}

pub(crate) fn sanitize(value: &str) -> String {
    value
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}
