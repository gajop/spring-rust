#![allow(clippy::all)]

pub(crate) mod bindings {
    wit_bindgen::generate!({
        path: "wit",
        world: "rules-unsynced",
    });
}

use std::sync::atomic::{AtomicBool, Ordering};

use bindings::exports::recoil::spring_api::callins_rules_unsynced::{
    GameFrameQuery, GameFrameResult, Guest, RecvFromSyncedQuery, SpringError, UpdateQuery,
    UpdateResult,
};
use bindings::recoil::spring_api::{messages, profiling, units_info, units_query};

static RAN: AtomicBool = AtomicBool::new(false);

fn scale() -> f64 {
    option_env!("SPRING_BENCHMARK_SCALE")
        .and_then(|value| value.parse::<f64>().ok())
        .map(|value| value.clamp(0.0001, 1.0))
        .unwrap_or(1.0)
}

fn count(value: usize) -> usize {
    ((value as f64 * scale()).round() as usize).max(1)
}

fn timer_micros() -> Result<u64, SpringError> {
    profiling::get_timer_micros(0u8).map_err(|error| SpringError { code: error.code })
}

fn memory_snapshot() -> Result<(f64, f64), SpringError> {
    let value =
        profiling::get_lua_mem_usage(0u8).map_err(|error| SpringError { code: error.code })?;
    Ok((
        value.global_alloced_kb as f64 * 1024.0,
        value.global_allocs_k as f64 * 1000.0,
    ))
}

fn send_row(row: &str) {
    let _ = messages::send_lua_rules_msg(&format!("WASM_BENCH|{row}"));
}

fn run_memory() -> Result<(), SpringError> {
    let units = units_query::get_team_units(0).map_err(|error| SpringError { code: error.code })?;
    let unit_id = *units.first().ok_or(SpringError { code: 1 })?;

    let (before_bytes, before_allocs) = memory_snapshot()?;
    let small_start = timer_micros()?;
    for _ in 0..count(100_000) {
        let _ = units_info::get_unit_position(
            unit_id,
            units_info::GetUnitPositionOptions {
                mid_pos: false,
                aim_pos: false,
            },
        );
    }
    let small_elapsed = timer_micros()?.saturating_sub(small_start);
    let (small_bytes, small_allocs) = memory_snapshot()?;
    send_row(&format!(
        "{{\"backend\":\"wasm\",\"test\":\"mem_per_call_small\",\"status\":\"pass\",\"bytes\":{:.3},\"allocations\":{:.3},\"elapsedMs\":{:.3},\"scale\":{}}}",
        (small_bytes - before_bytes).max(0.0),
        (small_allocs - before_allocs).max(0.0),
        small_elapsed as f64 / 1000.0,
        scale()
    ));

    let (before_bytes, before_allocs) = memory_snapshot()?;
    let list_start = timer_micros()?;
    for _ in 0..count(1_000) {
        let _ = units_query::get_team_units(0);
    }
    let list_elapsed = timer_micros()?.saturating_sub(list_start);
    let (list_bytes, list_allocs) = memory_snapshot()?;
    send_row(&format!(
        "{{\"backend\":\"wasm\",\"test\":\"mem_per_call_list\",\"status\":\"pass\",\"bytes\":{:.3},\"allocations\":{:.3},\"elapsedMs\":{:.3},\"scale\":{}}}",
        (list_bytes - before_bytes).max(0.0),
        (list_allocs - before_allocs).max(0.0),
        list_elapsed as f64 / 1000.0,
        scale()
    ));

    let gc_start = timer_micros()?;
    let gc_kb =
        profiling::get_synced_gc_info(true).map_err(|error| SpringError { code: error.code })?;
    let gc_elapsed = timer_micros()?.saturating_sub(gc_start);
    send_row(&format!(
        "{{\"backend\":\"wasm\",\"test\":\"gc_pause\",\"status\":\"pass\",\"gcKB\":{gc_kb:.3},\"totalPauseMs\":{:.3},\"scale\":{}}}",
        gc_elapsed as f64 / 1000.0,
        scale()
    ));

    let mut frame_times = Vec::new();
    let frame_count = count(5_000);
    for _ in 0..frame_count {
        let start = timer_micros()?;
        for unit in units.iter().take(count(1_000)) {
            let _ = units_info::get_unit_def_id(*unit);
            let _ = units_info::get_unit_position(
                *unit,
                units_info::GetUnitPositionOptions {
                    mid_pos: false,
                    aim_pos: false,
                },
            );
        }
        frame_times.push(timer_micros()?.saturating_sub(start) as f64 / 1000.0);
    }
    frame_times.sort_by(|left, right| left.total_cmp(right));
    let p99_index = ((frame_times.len() as f64 * 0.99).ceil() as usize)
        .saturating_sub(1)
        .min(frame_times.len() - 1);
    send_row(&format!(
        "{{\"backend\":\"wasm\",\"test\":\"frame_spike\",\"status\":\"pass\",\"worstMs\":{:.3},\"p99Ms\":{:.3},\"frames\":{},\"scale\":{}}}",
        frame_times[frame_times.len() - 1],
        frame_times[p99_index],
        frame_count,
        scale()
    ));

    let (steady_bytes, _) = memory_snapshot()?;
    send_row(&format!(
        "{{\"backend\":\"wasm\",\"test\":\"mem_growth\",\"status\":\"pass\",\"peakBytes\":{:.3},\"steadyBytes\":{:.3},\"scale\":{}}}",
        steady_bytes.max(before_bytes),
        steady_bytes,
        scale()
    ));
    send_row("{\"backend\":\"wasm\",\"test\":\"complete\",\"status\":\"pass\"}");
    Ok(())
}

struct BenchmarkUnsyncedGuest;

impl Guest for BenchmarkUnsyncedGuest {
    fn game_frame(_query: GameFrameQuery) -> Result<GameFrameResult, SpringError> {
        Ok(GameFrameResult { unused: 0 })
    }

    fn game_frame_post(_query: GameFrameQuery) -> Result<GameFrameResult, SpringError> {
        Ok(GameFrameResult { unused: 0 })
    }

    fn recv_from_synced(_query: RecvFromSyncedQuery) -> Result<(), SpringError> {
        Ok(())
    }

    fn update(_query: UpdateQuery) -> Result<UpdateResult, SpringError> {
        if !RAN.load(Ordering::Acquire) {
            let has_units = units_query::get_team_units(0)
                .map(|units| !units.is_empty())
                .unwrap_or(false);
            if has_units {
                RAN.store(true, Ordering::Release);
                if let Err(error) = run_memory() {
                    send_row(&format!(
                        "{{\"backend\":\"wasm\",\"test\":\"complete\",\"status\":\"error\",\"code\":{}}}",
                        error.code
                    ));
                }
            }
        }
        Ok(UpdateResult { unused: 0 })
    }
}

bindings::export!(BenchmarkUnsyncedGuest with_types_in bindings);
