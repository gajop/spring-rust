//! Executable Core callin coverage report.
//!
//! The planner describes every shape; this report describes what has an actual
//! production implementation path. Keep the distinction explicit so planned
//! ABI support is never mistaken for tested/executable coverage.

use anyhow::bail;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

use crate::model::ApiModel;
use crate::render_core_wasm_callin_exec;
use crate::render_core_wasm_callin_scratch;
use crate::render_core_wasm_callins::{self, CallinPlan, QueryStrategy, ResultStrategy};

const MAX_FLATTENED_PARAMS: u32 = 32;
const MAX_PACKED_RESULT_VALUES: u32 = 2;

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

#[derive(Serialize)]
struct Entry {
    name: String,
    query: String,
    result: String,
    aggregation: String,
    environments: Vec<String>,
    class: Option<String>,
    verified: bool,
    reason: Option<String>,
}

#[derive(Serialize)]
struct Report {
    total: usize,
    executable_total: usize,
    verified_total: usize,
    by_class: BTreeMap<String, usize>,
    executable: Vec<Entry>,
    pending: Vec<Entry>,
}

pub fn render_json(model: &ApiModel) -> anyhow::Result<String> {
    let plan = render_core_wasm_callins::plan_report(model);
    let (direct, scratch) = generator_inventory(model)?;

    let mut executable = Vec::new();
    let mut pending = Vec::new();
    let mut by_class = BTreeMap::<String, usize>::new();

    for callin in plan.callins {
        let class = executable_class(&callin, &direct, &scratch);
        let reason = class
            .is_none()
            .then(|| pending_reason(&callin, &direct, &scratch));
        let entry = Entry {
            name: callin.name.clone(),
            query: callin.query,
            result: callin.result,
            aggregation: callin.aggregation,
            environments: callin.environments,
            class: class.map(ToOwned::to_owned),
            verified: false,
            reason,
        };
        if let Some(class) = class {
            *by_class.entry(class.to_owned()).or_default() += 1;
            executable.push(entry);
        } else {
            pending.push(entry);
        }
    }

    let report = Report {
        total: executable.len() + pending.len(),
        executable_total: executable.len(),
        verified_total: 0,
        by_class,
        executable,
        pending,
    };
    Ok(serde_json::to_string_pretty(&report)? + "\n")
}

pub fn coverage_errors(model: &ApiModel) -> anyhow::Result<Vec<String>> {
    let plan = render_core_wasm_callins::plan_report(model);
    let (direct, scratch) = generator_inventory(model)?;
    Ok(plan
        .callins
        .iter()
        .filter(|callin| executable_class(callin, &direct, &scratch).is_none())
        .map(|callin| {
            format!(
                "{}: {}",
                callin.name,
                pending_reason(callin, &direct, &scratch)
            )
        })
        .collect())
}

fn generator_inventory(model: &ApiModel) -> anyhow::Result<(BTreeSet<String>, BTreeSet<String>)> {
    let direct = render_core_wasm_callin_exec::executable_canonical_names(model);
    let scratch = render_core_wasm_callin_scratch::executable_canonical_names(model);
    let overlap = direct.intersection(&scratch).cloned().collect::<Vec<_>>();
    if !overlap.is_empty() {
        bail!(
            "Core callins selected by both direct and scratch generators: {}",
            overlap.join(", ")
        );
    }
    Ok((direct, scratch))
}

fn fixed_result_packable(callin: &CallinPlan) -> bool {
    callin.result_strategy == ResultStrategy::FixedWire
        && callin
            .packed_result_values
            .is_some_and(|count| count > 0 && count <= MAX_PACKED_RESULT_VALUES)
}

fn executable_class(
    callin: &CallinPlan,
    direct: &BTreeSet<String>,
    scratch: &BTreeSet<String>,
) -> Option<&'static str> {
    if SPECIALIZED_CALLINS.contains(&callin.name.as_str()) {
        return Some("specialized");
    }

    if scratch.contains(&callin.name) {
        if callin.aggregation == "ignore" {
            return Some("generated-shared-scratch-ignored-result");
        }
        return Some(if fixed_result_packable(callin) {
            "generated-shared-scratch-packed-result"
        } else {
            "generated-shared-scratch"
        });
    }

    // From here on, a class is executable only when the direct/fixed generator
    // selected the canonical callin. Planner shape metadata labels that emitted
    // ABI but no longer independently grants executable coverage.
    if !direct.contains(&callin.name) {
        return None;
    }

    if callin.aggregation == "ignore" {
        return Some(if callin.query_strategy == QueryStrategy::FixedWire {
            "generated-fixed-flattened-ignored-result"
        } else {
            "generated-direct-ignored-result"
        });
    }

    if callin.aggregation == "first-non-empty" && callin.result == "StringCallinResult" {
        return Some("generated-first-non-empty-string");
    }

    let fixed_query = callin.query_strategy == QueryStrategy::FixedWire;
    if fixed_result_packable(callin) {
        return Some(if fixed_query {
            "generated-fixed-flattened-packed-result"
        } else {
            "generated-packed-result"
        });
    }

    Some(if fixed_query {
        "generated-fixed-flattened"
    } else {
        "generated-direct"
    })
}

fn pending_reason(
    callin: &CallinPlan,
    direct: &BTreeSet<String>,
    scratch: &BTreeSet<String>,
) -> String {
    debug_assert!(!direct.contains(&callin.name));
    debug_assert!(!scratch.contains(&callin.name));

    if matches!(callin.query_strategy, QueryStrategy::Manual) {
        return "manual/opaque callin query lowering required".to_owned();
    }
    if callin.aggregation != "ignore" && matches!(callin.result_strategy, ResultStrategy::Manual) {
        return "manual/opaque callin result lowering required".to_owned();
    }
    if matches!(callin.query_strategy, QueryStrategy::VariableScratch) {
        if !callin.scratch_query_eligible {
            return "variable query needs explicit option/complex-list/retained adapter".to_owned();
        }
        if callin.result_strategy == ResultStrategy::FixedWire && !fixed_result_packable(callin) {
            return "scratch query fixed result exceeds the packed return ABI".to_owned();
        }
        return "planner accepts variable scratch shape but generated scratch model rejected it"
            .to_owned();
    }
    if callin.query_strategy == QueryStrategy::FixedWire {
        if callin.numeric_flat_query_values.is_none() {
            return "fixed query is not representable by the native scalar flattener".to_owned();
        }
        if callin
            .numeric_flat_query_values
            .is_some_and(|count| count > MAX_FLATTENED_PARAMS)
        {
            return format!(
                "fixed numeric query has more than {MAX_FLATTENED_PARAMS} scalar leaves; use reviewed wire lowering"
            );
        }
    }
    if matches!(callin.result_strategy, ResultStrategy::FixedWire) {
        if callin.packed_result_values.is_none() {
            return "fixed result contains a non-32-bit leaf and needs explicit return lowering"
                .to_owned();
        }
        if callin
            .packed_result_values
            .is_some_and(|count| count > MAX_PACKED_RESULT_VALUES)
        {
            return format!(
                "fixed result has more than {MAX_PACKED_RESULT_VALUES} 32-bit leaves; use guest scratch or multi-value lowering"
            );
        }
    }
    "planner accepts direct/fixed shape but generated direct model rejected it".to_owned()
}
