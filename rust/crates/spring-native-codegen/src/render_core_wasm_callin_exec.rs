//! Executable Core-Wasm callin bindings for allocation-free scalar/fixed
//! queries and compact results.
//!
//! Query records are recursively flattened into Core scalar parameters. This
//! covers direct records and fixed numeric records/arrays without guest scratch
//! or memory traffic. Small fixed results with one or two 32-bit leaves are
//! packed into one i64. `first-non-empty` string results use one packed `(ptr,
//! len)` i64; the host validates and copies that guest memory immediately so no
//! guest pointer survives the call. Other variable/manual shapes stay in their
//! dedicated lowering classes.

use std::collections::BTreeSet;

use crate::model::ApiModel;

#[path = "render_core_wasm_callin_exec_model.rs"]
mod exec_model;
#[path = "render_core_wasm_callin_exec_guest.rs"]
mod guest;
#[path = "render_core_wasm_callin_exec_host.rs"]
mod host;

pub fn render_header(model: &ApiModel) -> String {
    host::render_header(model)
}

pub fn render_cpp(model: &ApiModel) -> String {
    host::render_cpp(model)
}

pub fn render_rust(model: &ApiModel) -> String {
    guest::render_rust(model)
}

pub(crate) fn executable_canonical_names(model: &ApiModel) -> BTreeSet<String> {
    let records = exec_model::record_index(model);
    exec_model::expanded_callins(model)
        .into_iter()
        .filter(|entry| entry.name == entry.callin.name)
        .filter_map(|entry| exec_model::executable_callin(entry, &records))
        .map(|callin| callin.entry.callin.name.clone())
        .collect()
}
