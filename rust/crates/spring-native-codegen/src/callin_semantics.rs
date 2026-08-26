//! Semantic overrides for callin records whose C spelling is intentionally
//! more ambiguous than their engine contract.

use anyhow::{Result, anyhow};

use crate::model::{ApiModel, LoweringStatus, SemanticType};

const COUNTED_BYTE_CALLINS: &[(&str, &str, &str)] = &[
    ("HandleLuaCall", "message", "messageLength"),
    ("RecvFromSynced", "message", "messageLength"),
];

pub fn normalize(model: &mut ApiModel) -> Result<()> {
    for &(callin_name, value_name, count_name) in COUNTED_BYTE_CALLINS {
        let Some(query_name) = model
            .callins
            .iter()
            .find(|callin| callin.name == callin_name)
            .map(|callin| callin.query.clone())
        else {
            continue;
        };

        let mut normalized = 0usize;
        for module in &mut model.modules {
            for record in &mut module.records {
                if record.name != query_name {
                    continue;
                }
                normalize_counted_bytes(record, value_name, count_name)
                    .map_err(|reason| anyhow!("{callin_name} query {query_name}: {reason}"))?;
                normalized += 1;
            }
        }
        if normalized == 0 {
            return Err(anyhow!(
                "{callin_name} references missing counted-byte query record {query_name}"
            ));
        }
    }
    Ok(())
}

fn normalize_counted_bytes(
    record: &mut crate::model::RecordModel,
    value_name: &str,
    count_name: &str,
) -> std::result::Result<(), String> {
    let value_index = record
        .fields
        .iter()
        .position(|field| field.name == value_name)
        .ok_or_else(|| format!("missing value field {value_name}"))?;
    let count_index = record
        .fields
        .iter()
        .position(|field| field.name == count_name)
        .ok_or_else(|| format!("missing count field {count_name}"))?;
    if value_index == count_index {
        return Err("value and count fields unexpectedly alias".to_owned());
    }

    {
        let value = &mut record.fields[value_index];
        value.ty = SemanticType::List {
            element: Box::new(SemanticType::Scalar {
                name: "u8".to_owned(),
            }),
        };
        value.status = LoweringStatus::Annotated;
        value.metadata.retain(|metadata| {
            !metadata.starts_with("count-field:") && !metadata.starts_with("presence-field:")
        });
        value.metadata.push(format!("count-field:{count_name}"));
        value.metadata.push("callin-counted-bytes".to_owned());
    }
    record.fields.remove(count_index);
    if record.status == LoweringStatus::Automatic {
        record.status = LoweringStatus::Annotated;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ApiModule, CallinModel, Environment, FieldModel, RecordModel};
    use std::collections::BTreeSet;

    #[test]
    fn counted_message_preserves_embedded_bytes_semantics() {
        let query = RecordModel {
            name: "RecvFromSyncedQuery".to_owned(),
            fields: vec![
                FieldModel {
                    name: "message".to_owned(),
                    ty: SemanticType::String,
                    status: LoweringStatus::Automatic,
                    metadata: Vec::new(),
                },
                FieldModel {
                    name: "messageLength".to_owned(),
                    ty: SemanticType::Scalar {
                        name: "u32".to_owned(),
                    },
                    status: LoweringStatus::Automatic,
                    metadata: Vec::new(),
                },
            ],
            status: LoweringStatus::Automatic,
        };
        let mut model = ApiModel {
            model_version: 1,
            native_api_version: None,
            modules: vec![ApiModule {
                name: "test".to_owned(),
                interface_version: "1.0".to_owned(),
                functions: Vec::new(),
                records: vec![query],
                enums: Vec::new(),
            }],
            callins: vec![CallinModel {
                name: "RecvFromSynced".to_owned(),
                query: "RecvFromSyncedQuery".to_owned(),
                result: "RecvFromSyncedResult".to_owned(),
                environments: BTreeSet::from([Environment::RulesUnsynced]),
                aggregation: "ignore".to_owned(),
                aliases: Vec::new(),
                flags: Vec::new(),
            }],
        };

        normalize(&mut model).unwrap();
        let fields = &model.modules[0].records[0].fields;
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].name, "message");
        assert_eq!(
            fields[0].ty,
            SemanticType::List {
                element: Box::new(SemanticType::Scalar {
                    name: "u8".to_owned(),
                }),
            }
        );
        assert!(
            fields[0]
                .metadata
                .iter()
                .any(|metadata| metadata == "count-field:messageLength")
        );
    }
}
