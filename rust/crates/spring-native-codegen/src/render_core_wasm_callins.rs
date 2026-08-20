//! Core-WebAssembly callin planning from the canonical Callins.def model.
//!
//! This is diagnostic/implementation metadata, not an executable registry.
//! It keeps callin ABI decisions tied to the same semantic records used by WIT
//! and the native adapters while making variable-scratch/manual gaps explicit.

use serde::Serialize;
use std::collections::BTreeMap;

use crate::model::{ApiModel, CallinModel, FieldModel, RecordModel, SemanticType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum QueryStrategy {
    Empty,
    Direct,
    FixedWire,
    VariableScratch,
    Manual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResultStrategy {
    Empty,
    Direct,
    FixedWire,
    Manual,
}

#[derive(Debug, Clone, Serialize)]
pub struct CallinPlan {
    pub name: String,
    pub query: String,
    pub result: String,
    pub aggregation: String,
    pub environments: Vec<String>,
    pub query_strategy: QueryStrategy,
    pub result_strategy: ResultStrategy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_query_bytes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_result_bytes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric_flat_query_values: Option<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CallinPlanReport {
    pub callins: Vec<CallinPlan>,
    pub by_query_strategy: BTreeMap<String, usize>,
    pub by_result_strategy: BTreeMap<String, usize>,
}

pub fn render_json(model: &ApiModel) -> anyhow::Result<String> {
    let records = record_index(model);
    let mut callins = model
        .callins
        .iter()
        .map(|callin| plan_callin(callin, &records))
        .collect::<Vec<_>>();
    callins.sort_by(|left, right| left.name.cmp(&right.name));

    let mut by_query_strategy = BTreeMap::new();
    let mut by_result_strategy = BTreeMap::new();
    for callin in &callins {
        *by_query_strategy
            .entry(query_strategy_name(callin.query_strategy).to_owned())
            .or_default() += 1;
        *by_result_strategy
            .entry(result_strategy_name(callin.result_strategy).to_owned())
            .or_default() += 1;
    }

    Ok(serde_json::to_string_pretty(&CallinPlanReport {
        callins,
        by_query_strategy,
        by_result_strategy,
    })? + "\n")
}

fn plan_callin(callin: &CallinModel, records: &BTreeMap<String, RecordModel>) -> CallinPlan {
    let query = records.get(&callin.query);
    let result = records.get(&callin.result);
    let mut notes = Vec::new();

    let (query_strategy, fixed_query_bytes, numeric_flat_query_values) = match query {
        None => {
            notes.push("missing query record".to_owned());
            (QueryStrategy::Manual, None, None)
        }
        Some(record) if record.fields.is_empty() => (QueryStrategy::Empty, Some(0), Some(0)),
        Some(record) if record.fields.iter().any(|field| contains_string_list(&field.ty, records)) => {
            notes.push("list<string> requires reviewed flat scratch lowering".to_owned());
            (QueryStrategy::Manual, None, None)
        }
        Some(record) if record.fields.iter().any(|field| manual_type(&field.ty, records)) => {
            notes.push("query contains pointer/callback/unknown or unsupported nested type".to_owned());
            (QueryStrategy::Manual, None, None)
        }
        Some(record) if record.fields.iter().any(|field| variable_type(&field.ty, records)) => {
            notes.push("variable payload should use one cached guest callin scratch region".to_owned());
            (QueryStrategy::VariableScratch, None, None)
        }
        Some(record) => {
            let layout = layout_fields(&record.fields, records).map(|value| value.0);
            let flat = flatten_numeric_fields(&record.fields, records);
            if record.fields.iter().all(|field| direct_type(&field.ty)) {
                (QueryStrategy::Direct, layout, flat)
            } else {
                if flat.is_some() {
                    notes.push(
                        "fixed numeric record is a flattening candidate; benchmark before choosing a large signature"
                            .to_owned(),
                    );
                }
                (QueryStrategy::FixedWire, layout, flat)
            }
        }
    };

    let (result_strategy, fixed_result_bytes) = match result {
        None => {
            notes.push("missing result record".to_owned());
            (ResultStrategy::Manual, None)
        }
        Some(record) if record.fields.is_empty() => (ResultStrategy::Empty, Some(0)),
        Some(record)
            if record
                .fields
                .iter()
                .any(|field| manual_type(&field.ty, records) || variable_type(&field.ty, records)) =>
        {
            notes.push("variable/manual result needs reviewed guest-to-host return lowering".to_owned());
            (ResultStrategy::Manual, None)
        }
        Some(record) if record.fields.len() == 1 && direct_type(&record.fields[0].ty) => {
            (ResultStrategy::Direct, layout_fields(&record.fields, records).map(|value| value.0))
        }
        Some(record) => (
            ResultStrategy::FixedWire,
            layout_fields(&record.fields, records).map(|value| value.0),
        ),
    };

    CallinPlan {
        name: callin.name.clone(),
        query: callin.query.clone(),
        result: callin.result.clone(),
        aggregation: callin.aggregation.clone(),
        environments: callin
            .environments
            .iter()
            .map(|environment| environment.as_str().to_owned())
            .collect(),
        query_strategy,
        result_strategy,
        fixed_query_bytes,
        fixed_result_bytes,
        numeric_flat_query_values,
        notes,
    }
}

fn direct_type(ty: &SemanticType) -> bool {
    matches!(
        ty,
        SemanticType::Scalar { .. } | SemanticType::Enum { .. } | SemanticType::Handle { .. }
    )
}

fn variable_type(ty: &SemanticType, records: &BTreeMap<String, RecordModel>) -> bool {
    match ty {
        SemanticType::String | SemanticType::Bytes | SemanticType::List { .. } => true,
        SemanticType::FixedArray { element, .. } | SemanticType::Option { inner: element } => {
            variable_type(element, records)
        }
        SemanticType::Record { name } => records
            .get(name)
            .is_some_and(|record| record.fields.iter().any(|field| variable_type(&field.ty, records))),
        SemanticType::Result { ok, error } => {
            ok.as_deref().is_some_and(|ty| variable_type(ty, records))
                || error.as_deref().is_some_and(|ty| variable_type(ty, records))
        }
        _ => false,
    }
}

fn manual_type(ty: &SemanticType, records: &BTreeMap<String, RecordModel>) -> bool {
    match ty {
        SemanticType::Pointer { .. } | SemanticType::Callback { .. } | SemanticType::Unknown { .. } => true,
        SemanticType::FixedArray { element, .. } | SemanticType::Option { inner: element } => {
            manual_type(element, records)
        }
        SemanticType::List { element } => manual_type(element, records),
        SemanticType::Record { name } => records
            .get(name)
            .is_none_or(|record| record.fields.iter().any(|field| manual_type(&field.ty, records))),
        SemanticType::Result { ok, error } => {
            ok.as_deref().is_some_and(|ty| manual_type(ty, records))
                || error.as_deref().is_some_and(|ty| manual_type(ty, records))
        }
        _ => false,
    }
}

fn contains_string_list(ty: &SemanticType, records: &BTreeMap<String, RecordModel>) -> bool {
    match ty {
        SemanticType::List { element } => {
            matches!(element.as_ref(), SemanticType::String)
                || contains_string_list(element, records)
        }
        SemanticType::FixedArray { element, .. } | SemanticType::Option { inner: element } => {
            contains_string_list(element, records)
        }
        SemanticType::Record { name } => records.get(name).is_some_and(|record| {
            record
                .fields
                .iter()
                .any(|field| contains_string_list(&field.ty, records))
        }),
        SemanticType::Result { ok, error } => {
            ok.as_deref().is_some_and(|ty| contains_string_list(ty, records))
                || error
                    .as_deref()
                    .is_some_and(|ty| contains_string_list(ty, records))
        }
        SemanticType::Pointer { pointee, .. } => contains_string_list(pointee, records),
        _ => false,
    }
}

fn flatten_numeric_fields(
    fields: &[FieldModel],
    records: &BTreeMap<String, RecordModel>,
) -> Option<u32> {
    fields.iter().try_fold(0u32, |count, field| {
        count.checked_add(flatten_numeric_type(&field.ty, records)?)
    })
}

fn flatten_numeric_type(
    ty: &SemanticType,
    records: &BTreeMap<String, RecordModel>,
) -> Option<u32> {
    if direct_type(ty) {
        return Some(1);
    }
    match ty {
        SemanticType::FixedArray { element, length } => {
            flatten_numeric_type(element, records)?
                .checked_mul(u32::try_from(*length).ok()?)
        }
        SemanticType::Record { name } => flatten_numeric_fields(&records.get(name)?.fields, records),
        SemanticType::Option { inner } => 1u32.checked_add(flatten_numeric_type(inner, records)?),
        _ => None,
    }
}

fn layout_fields(fields: &[FieldModel], records: &BTreeMap<String, RecordModel>) -> Option<(u32, u32)> {
    let mut bytes = 0u32;
    let mut alignment = 1u32;
    for field in fields {
        let (field_bytes, field_alignment) = fixed_layout(&field.ty, records)?;
        bytes = align_up(bytes, field_alignment).checked_add(field_bytes)?;
        alignment = alignment.max(field_alignment);
    }
    Some((align_up(bytes, alignment), alignment))
}

fn fixed_layout(ty: &SemanticType, records: &BTreeMap<String, RecordModel>) -> Option<(u32, u32)> {
    match ty {
        SemanticType::Scalar { name } => match name.as_str() {
            "i64" | "u64" | "isize" | "usize" | "f64" => Some((8, 8)),
            _ => Some((4, 4)),
        },
        SemanticType::Enum { .. } => Some((4, 4)),
        SemanticType::Handle { .. } => Some((8, 8)),
        SemanticType::FixedArray { element, length } => {
            let (bytes, alignment) = fixed_layout(element, records)?;
            Some((bytes.checked_mul(u32::try_from(*length).ok()?)?, alignment))
        }
        SemanticType::Record { name } => layout_fields(&records.get(name)?.fields, records),
        SemanticType::Option { inner } => {
            let (payload_bytes, payload_alignment) = fixed_layout(inner, records)?;
            let alignment = payload_alignment.max(4);
            let payload_offset = align_up(4, payload_alignment);
            Some((align_up(payload_offset.checked_add(payload_bytes)?, alignment), alignment))
        }
        _ => None,
    }
}

fn align_up(value: u32, alignment: u32) -> u32 {
    (value + alignment - 1) & !(alignment - 1)
}

fn record_index(model: &ApiModel) -> BTreeMap<String, RecordModel> {
    let mut records = BTreeMap::new();
    for module in &model.modules {
        for record in &module.records {
            records.entry(record.name.clone()).or_insert_with(|| record.clone());
        }
    }
    records
}

fn query_strategy_name(strategy: QueryStrategy) -> &'static str {
    match strategy {
        QueryStrategy::Empty => "empty",
        QueryStrategy::Direct => "direct",
        QueryStrategy::FixedWire => "fixed-wire",
        QueryStrategy::VariableScratch => "variable-scratch",
        QueryStrategy::Manual => "manual",
    }
}

fn result_strategy_name(strategy: ResultStrategy) -> &'static str {
    match strategy {
        ResultStrategy::Empty => "empty",
        ResultStrategy::Direct => "direct",
        ResultStrategy::FixedWire => "fixed-wire",
        ResultStrategy::Manual => "manual",
    }
}
