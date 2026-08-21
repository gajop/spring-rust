//! Eligibility and ABI model for direct generated Core-Wasm callins.

use heck::ToSnakeCase;
use std::collections::BTreeMap;

use crate::model::{ApiModel, CallinModel, FieldModel, RecordModel, SemanticType};

const MAX_FLATTENED_PARAMS: usize = 32;
const MAX_PACKED_RESULT_LEAVES: usize = 2;

const SPECIALIZED_CALLINS: &[&str] = &[
    "AddConsoleLine",
    "AllowUnitCreation",
    "CommandNotify",
    "DrawWorld",
    "GameFrame",
    "GameFramePost",
    "UnitCreated",
    "UnitPreDamaged",
    "Update",
];

#[derive(Clone)]
pub(super) struct ExpandedCallin<'a> {
    pub(super) name: String,
    pub(super) callin: &'a CallinModel,
    pub(super) ordinal: u16,
}

#[derive(Clone)]
pub(super) struct FlatLeaf {
    pub(super) ty: SemanticType,
    pub(super) cpp_path: String,
    pub(super) rust_name: String,
}

#[derive(Clone)]
pub(super) enum ResultAbi {
    Empty,
    Direct { field: FieldModel },
    PackedFixed { leaves: Vec<FlatLeaf> },
    String { field: FieldModel },
}

#[derive(Clone)]
pub(super) struct ExecutableCallin<'a> {
    pub(super) entry: ExpandedCallin<'a>,
    pub(super) query_leaves: Vec<FlatLeaf>,
    pub(super) result: ResultAbi,
}

pub(super) fn executable_callin<'a>(
    entry: ExpandedCallin<'a>,
    records: &BTreeMap<String, RecordModel>,
) -> Option<ExecutableCallin<'a>> {
    if SPECIALIZED_CALLINS.contains(&entry.name.as_str()) {
        return None;
    }
    let query = records.get(&entry.callin.query)?;

    let query_leaves = flatten_fields(&query.fields, records)?;
    if query_leaves.len() > MAX_FLATTENED_PARAMS {
        return None;
    }

    // Ignore aggregation is notification semantics. Core dispatch always invokes
    // these callins with a null result sink, so requiring or exporting the native
    // result shape would add ABI work that can never be observed.
    let result = if entry.callin.aggregation == "ignore" {
        ResultAbi::Empty
    } else {
        let result = records.get(&entry.callin.result)?;
        if result.fields.is_empty() {
            ResultAbi::Empty
        } else if result.fields.len() == 1
            && matches!(result.fields[0].ty, SemanticType::String)
            && entry.callin.aggregation == "first-non-empty"
        {
            ResultAbi::String {
                field: result.fields[0].clone(),
            }
        } else if result.fields.len() == 1 && direct_type(&result.fields[0].ty) {
            ResultAbi::Direct {
                field: result.fields[0].clone(),
            }
        } else {
            let leaves = flatten_fields(&result.fields, records)?;
            if leaves.is_empty()
                || leaves.len() > MAX_PACKED_RESULT_LEAVES
                || !leaves.iter().all(|leaf| packed32_type(&leaf.ty))
            {
                return None;
            }
            ResultAbi::PackedFixed { leaves }
        }
    };

    Some(ExecutableCallin {
        entry,
        query_leaves,
        result,
    })
}

pub(super) fn expanded_callins(model: &ApiModel) -> Vec<ExpandedCallin<'_>> {
    let mut entries = Vec::new();
    // Keep this byte-for-byte in the same semantic ordering as
    // render_host::render_callin_registry: canonical callin, then its aliases.
    // WasmCoreCallin is the generated registry index + 1; sorting here makes
    // aliases address unrelated export slots.
    for callin in &model.callins {
        entries.push((callin.name.clone(), callin));
        for alias in &callin.aliases {
            entries.push((alias.clone(), callin));
        }
    }
    entries
        .into_iter()
        .enumerate()
        .map(|(index, (name, callin))| ExpandedCallin {
            name,
            callin,
            ordinal: u16::try_from(index + 1).expect("Core callin count exceeds u16"),
        })
        .collect()
}

fn flatten_fields(
    fields: &[FieldModel],
    records: &BTreeMap<String, RecordModel>,
) -> Option<Vec<FlatLeaf>> {
    let mut output = Vec::new();
    for field in fields {
        if !flatten_type(
            &field.ty,
            &field.name,
            &field.name.to_snake_case(),
            records,
            &mut output,
        ) {
            return None;
        }
    }
    Some(output)
}

fn flatten_type(
    ty: &SemanticType,
    cpp_path: &str,
    rust_name: &str,
    records: &BTreeMap<String, RecordModel>,
    output: &mut Vec<FlatLeaf>,
) -> bool {
    if direct_type(ty) {
        output.push(FlatLeaf {
            ty: ty.clone(),
            cpp_path: cpp_path.to_owned(),
            rust_name: rust_ident(rust_name),
        });
        return true;
    }
    match ty {
        SemanticType::Record { name } => {
            let Some(record) = records.get(name) else {
                return false;
            };
            for field in &record.fields {
                if !flatten_type(
                    &field.ty,
                    &format!("{cpp_path}.{}", field.name),
                    &format!("{rust_name}_{}", field.name.to_snake_case()),
                    records,
                    output,
                ) {
                    return false;
                }
            }
            true
        }
        SemanticType::FixedArray { element, length } => {
            let Ok(length) = usize::try_from(*length) else {
                return false;
            };
            for index in 0..length {
                if !flatten_type(
                    element,
                    &format!("{cpp_path}[{index}]"),
                    &format!("{rust_name}_{index}"),
                    records,
                    output,
                ) {
                    return false;
                }
            }
            true
        }
        _ => false,
    }
}

pub(super) fn direct_type(ty: &SemanticType) -> bool {
    matches!(
        ty,
        SemanticType::Scalar { .. } | SemanticType::Enum { .. } | SemanticType::Handle { .. }
    )
}

fn packed32_type(ty: &SemanticType) -> bool {
    match ty {
        SemanticType::Scalar { name } => matches!(
            name.as_str(),
            "bool" | "i8" | "i16" | "i32" | "u8" | "char" | "u16" | "u32" | "f32"
        ),
        SemanticType::Enum { .. } => true,
        _ => false,
    }
}

fn rust_ident(value: &str) -> String {
    match value {
        "move" | "type" | "match" | "loop" | "ref" | "self" | "crate" | "super" => {
            format!("r#{value}")
        }
        _ => value.to_owned(),
    }
}

pub(super) fn record_index(model: &ApiModel) -> BTreeMap<String, RecordModel> {
    let mut records = BTreeMap::new();
    for module in &model.modules {
        for record in &module.records {
            records
                .entry(record.name.clone())
                .or_insert_with(|| record.clone());
        }
    }
    records
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use crate::model::{CallinModel, FieldModel, LoweringStatus, RecordModel, SemanticType};

    use super::{executable_callin, ExpandedCallin, ResultAbi};

    fn scalar_query() -> RecordModel {
        RecordModel {
            name: "Query".to_owned(),
            fields: vec![FieldModel {
                name: "value".to_owned(),
                ty: SemanticType::Scalar {
                    name: "i32".to_owned(),
                },
                status: LoweringStatus::Automatic,
                metadata: Vec::new(),
            }],
            status: LoweringStatus::Automatic,
        }
    }

    fn opaque_result() -> RecordModel {
        RecordModel {
            name: "OpaqueResult".to_owned(),
            fields: vec![FieldModel {
                name: "moduleData".to_owned(),
                ty: SemanticType::Pointer {
                    pointee: Box::new(SemanticType::Scalar {
                        name: "u8".to_owned(),
                    }),
                    mutable: true,
                },
                status: LoweringStatus::Manual,
                metadata: Vec::new(),
            }],
            status: LoweringStatus::Manual,
        }
    }

    fn callin(aggregation: &str) -> CallinModel {
        CallinModel {
            name: "Notification".to_owned(),
            query: "Query".to_owned(),
            result: "OpaqueResult".to_owned(),
            environments: BTreeSet::new(),
            aggregation: aggregation.to_owned(),
            aliases: Vec::new(),
            flags: Vec::new(),
        }
    }

    #[test]
    fn ignored_callin_does_not_require_native_result_lowering() {
        let callin = callin("ignore");
        let mut records = BTreeMap::new();
        records.insert("Query".to_owned(), scalar_query());
        records.insert("OpaqueResult".to_owned(), opaque_result());
        let executable = executable_callin(
            ExpandedCallin {
                name: callin.name.clone(),
                callin: &callin,
                ordinal: 1,
            },
            &records,
        )
        .expect("ignored direct callin should be executable");
        assert!(matches!(executable.result, ResultAbi::Empty));
    }

    #[test]
    fn observed_callin_still_requires_native_result_lowering() {
        let callin = callin("first");
        let mut records = BTreeMap::new();
        records.insert("Query".to_owned(), scalar_query());
        records.insert("OpaqueResult".to_owned(), opaque_result());
        assert!(executable_callin(
            ExpandedCallin {
                name: callin.name.clone(),
                callin: &callin,
                ordinal: 1,
            },
            &records,
        )
        .is_none());
    }
}
