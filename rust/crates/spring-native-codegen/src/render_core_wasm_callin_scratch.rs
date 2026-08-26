//! Generated Core callins for variable-size engine -> guest queries.
//!
//! Shared-scratch callins use one guest-owned region, no per-event host
//! allocation and exactly one host -> guest call. Host serialization, guest
//! parsing, eligibility and result-shape logic live in focused submodules.

use std::collections::BTreeSet;

use crate::model::ApiModel;

#[path = "render_core_wasm_callin_scratch_guest.rs"]
mod guest;
#[path = "render_core_wasm_callin_scratch_host.rs"]
mod host;
#[path = "render_core_wasm_callin_scratch_result.rs"]
mod result;
#[path = "render_core_wasm_callin_scratch_model.rs"]
mod scratch_model;

// Guest generation is a descendant module and uses these shared model helpers
// through the parent namespace. Keep the public renderer surface itself tiny.
use scratch_model::{ScratchCallin, executable, implicit_count_fields, record_index};

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
    let records = record_index(model);
    executable(model, &records)
        .into_iter()
        .map(|callin| callin.entry.callin.name.clone())
        .collect()
}
