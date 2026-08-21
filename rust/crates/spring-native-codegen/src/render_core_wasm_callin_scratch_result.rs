//! Result-shape classification shared by host and guest scratch-callin generation.

use std::collections::BTreeMap;

use crate::model::{CallinModel, FieldModel, RecordModel, SemanticType};

const MAX_PACKED_RESULT_LEAVES: usize = 2;

#[derive(Clone, Copy)]
pub(super) struct ResultRecord<'a> {
    record: &'a RecordModel,
    ignored: bool,
}

pub(super) fn for_callin<'a>(callin: &CallinModel, record: &'a RecordModel) -> ResultRecord<'a> {
    ResultRecord {
        record,
        ignored: callin.aggregation == "ignore",
    }
}

#[derive(Clone)]
pub(super) struct PackedLeaf {
    pub(super) ty: SemanticType,
    pub(super) cpp_path: String,
}

#[derive(Clone)]
pub(super) enum ResultShape {
    Empty,
    Direct(FieldModel),
    PackedFixed(Vec<PackedLeaf>),
}

pub(super) fn classify(
    result: ResultRecord<'_>,
    records: &BTreeMap<String, RecordModel>,
) -> Option<ResultShape> {
    // The Core dispatcher always passes a null result sink for notification
    // callins. Their native result records are therefore not part of the guest
    // ABI, even when those records contain opaque/native-only fields.
    if result.ignored {
        return Some(ResultShape::Empty);
    }

    let record = result.record;
    if record.fields.is_empty() {
        return Some(ResultShape::Empty);
    }
    if record.fields.len() == 1 && direct_type(&record.fields[0].ty) {
        return Some(ResultShape::Direct(record.fields[0].clone()));
    }

    let leaves = flatten_fields(&record.fields, records)?;
    if leaves.is_empty()
        || leaves.len() > MAX_PACKED_RESULT_LEAVES
        || !leaves.iter().all(|leaf| packed32_type(&leaf.ty))
    {
        return None;
    }
    Some(ResultShape::PackedFixed(leaves))
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

fn flatten_fields(
    fields: &[FieldModel],
    records: &BTreeMap<String, RecordModel>,
) -> Option<Vec<PackedLeaf>> {
    let mut output = Vec::new();
    for field in fields {
        if !flatten_type(&field.ty, &field.name, records, &mut output) {
            return None;
        }
    }
    Some(output)
}

fn flatten_type(
    ty: &SemanticType,
    cpp_path: &str,
    records: &BTreeMap<String, RecordModel>,
    output: &mut Vec<PackedLeaf>,
) -> bool {
    if direct_type(ty) {
        output.push(PackedLeaf {
            ty: ty.clone(),
            cpp_path: cpp_path.to_owned(),
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
                if !flatten_type(element, &format!("{cpp_path}[{index}]"), records, output) {
                    return false;
                }
            }
            true
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use crate::model::{CallinModel, FieldModel, LoweringStatus, RecordModel, SemanticType};

    use super::{classify, for_callin, ResultShape};

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
        let result = opaque_result();
        assert!(matches!(
            classify(for_callin(&callin("ignore"), &result), &BTreeMap::new()),
            Some(ResultShape::Empty)
        ));
    }

    #[test]
    fn observed_callin_still_rejects_opaque_result() {
        let result = opaque_result();
        assert!(classify(for_callin(&callin("first"), &result), &BTreeMap::new()).is_none());
    }
}
