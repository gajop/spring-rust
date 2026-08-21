//! Scratch-callin selection and query-shape eligibility.

use std::collections::{BTreeMap, BTreeSet};

use crate::model::{ApiModel, CallinModel, FieldModel, RecordModel, SemanticType};
use crate::render_core_wasm_callins::{self as callin_plan, QueryStrategy, ResultStrategy};

use super::result;

const SPECIALIZED: &[&str] = &["AddConsoleLine", "CommandNotify"];

#[derive(Clone)]
pub(super) struct Expanded<'a> {
    pub(super) name: String,
    pub(super) callin: &'a CallinModel,
    pub(super) ordinal: u16,
}

#[derive(Clone)]
pub(super) struct ScratchCallin<'a> {
    pub(super) entry: Expanded<'a>,
    pub(super) query: &'a RecordModel,
    pub(super) result: result::ResultRecord<'a>,
}

pub(super) fn executable<'a>(
    model: &'a ApiModel,
    records: &'a BTreeMap<String, RecordModel>,
) -> Vec<ScratchCallin<'a>> {
    let plan = callin_plan_index(model);
    expanded(model)
        .into_iter()
        .filter_map(|entry| {
            if SPECIALIZED.contains(&entry.name.as_str()) {
                return None;
            }
            let callin_plan = plan.get(&entry.callin.name)?;
            if callin_plan.query_strategy != QueryStrategy::VariableScratch
                || (entry.callin.aggregation != "ignore"
                    && !matches!(
                        callin_plan.result_strategy,
                        ResultStrategy::Empty | ResultStrategy::Direct | ResultStrategy::FixedWire
                    ))
            {
                return None;
            }
            let query = records.get(&entry.callin.query)?;
            let result_record = records.get(&entry.callin.result)?;
            let result = result::for_callin(entry.callin, result_record);
            if !record_supported(query, records) || result::classify(result, records).is_none() {
                return None;
            }
            Some(ScratchCallin {
                entry,
                query,
                result,
            })
        })
        .collect()
}

pub(super) fn expanded(model: &ApiModel) -> Vec<Expanded<'_>> {
    let mut entries = Vec::new();
    // Match render_host::render_callin_registry exactly: canonical entry first,
    // followed immediately by its aliases. WasmCoreCallin is that registry
    // index + 1, so sorting here silently dispatches aliases to wrong slots.
    for callin in &model.callins {
        entries.push((callin.name.clone(), callin));
        for alias in &callin.aliases {
            entries.push((alias.clone(), callin));
        }
    }
    entries
        .into_iter()
        .enumerate()
        .map(|(index, (name, callin))| Expanded {
            name,
            callin,
            ordinal: u16::try_from(index + 1).expect("Core callin count exceeds u16"),
        })
        .collect()
}

fn record_supported(record: &RecordModel, records: &BTreeMap<String, RecordModel>) -> bool {
    let implicit = implicit_count_fields(&record.fields);
    record.fields.iter().all(|field| {
        implicit.contains(&field.name) || scratch_type_supported(&field.ty, field, records)
    })
}

fn scratch_type_supported(
    ty: &SemanticType,
    field: &FieldModel,
    records: &BTreeMap<String, RecordModel>,
) -> bool {
    match ty {
        SemanticType::Scalar { .. } | SemanticType::Enum { .. } | SemanticType::Handle { .. } => {
            true
        }
        SemanticType::String => true,
        SemanticType::Bytes => count_field(field).is_some(),
        SemanticType::List { element } => {
            count_field(field).is_some() && list_element_supported(element, records)
        }
        SemanticType::Record { name } => records
            .get(name)
            .is_some_and(|record| record_supported(record, records)),
        SemanticType::FixedArray { element, .. } => fixed_type_supported(element, records),
        SemanticType::Option { .. }
        | SemanticType::Result { .. }
        | SemanticType::Pointer { .. }
        | SemanticType::Callback { .. }
        | SemanticType::Unknown { .. } => false,
    }
}

fn fixed_type_supported(ty: &SemanticType, records: &BTreeMap<String, RecordModel>) -> bool {
    match ty {
        SemanticType::Scalar { .. } | SemanticType::Enum { .. } | SemanticType::Handle { .. } => {
            true
        }
        SemanticType::Record { name } => records
            .get(name)
            .is_some_and(|record| record_supported(record, records)),
        SemanticType::FixedArray { element, .. } => fixed_type_supported(element, records),
        _ => false,
    }
}

fn list_element_supported(ty: &SemanticType, records: &BTreeMap<String, RecordModel>) -> bool {
    match ty {
        SemanticType::Scalar { name } => !matches!(name.as_str(), "bool" | "isize" | "usize"),
        SemanticType::Enum { .. } => true,
        SemanticType::Record { name } => records
            .get(name)
            .is_some_and(|record| framed_record_supported(record, records)),
        _ => false,
    }
}

fn framed_record_supported(record: &RecordModel, records: &BTreeMap<String, RecordModel>) -> bool {
    let implicit = implicit_count_fields(&record.fields);
    record.fields.iter().all(|field| {
        implicit.contains(&field.name) || framed_record_type_supported(&field.ty, field, records)
    })
}

fn framed_record_type_supported(
    ty: &SemanticType,
    field: &FieldModel,
    records: &BTreeMap<String, RecordModel>,
) -> bool {
    match ty {
        SemanticType::Scalar { .. } | SemanticType::Enum { .. } | SemanticType::Handle { .. } => {
            true
        }
        SemanticType::String => true,
        SemanticType::Bytes => count_field(field).is_some(),
        SemanticType::List { element } => {
            (count_field(field).is_some()
                && matches!(
                    element.as_ref(),
                    SemanticType::Scalar { name }
                        if !matches!(name.as_str(), "bool" | "isize" | "usize")
                ))
                || (count_field(field).is_some()
                    && matches!(element.as_ref(), SemanticType::Enum { .. }))
        }
        SemanticType::Record { name } => records
            .get(name)
            .is_some_and(|record| framed_record_supported(record, records)),
        SemanticType::FixedArray { element, .. } => fixed_type_supported(element, records),
        SemanticType::Option { .. }
        | SemanticType::Result { .. }
        | SemanticType::Pointer { .. }
        | SemanticType::Callback { .. }
        | SemanticType::Unknown { .. } => false,
    }
}

pub(super) fn implicit_count_fields(fields: &[FieldModel]) -> BTreeSet<String> {
    fields.iter().filter_map(count_field).collect()
}

pub(super) fn count_field(field: &FieldModel) -> Option<String> {
    field
        .metadata
        .iter()
        .find_map(|metadata| metadata.strip_prefix("count-field:").map(ToOwned::to_owned))
}

fn callin_plan_index(model: &ApiModel) -> BTreeMap<String, callin_plan::CallinPlan> {
    callin_plan::plan_report(model)
        .callins
        .into_iter()
        .map(|plan| (plan.name.clone(), plan))
        .collect()
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
