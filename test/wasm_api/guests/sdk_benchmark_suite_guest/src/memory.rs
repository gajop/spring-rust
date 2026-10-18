/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use crate::Engine;
use spring_sdk as spring;

use crate::common;

fn memory_snapshot(spring: &Engine<'_>) -> spring::Result<(f64, f64)> {
    let value = spring.profiling().get_lua_mem_usage()?;
    Ok((
        value.global_alloced_kb as f64 * 1024.0,
        value.global_allocs_k as f64 * 1_000.0,
    ))
}

pub fn run(spring: &Engine<'_>) -> spring::Result<()> {
    let units = spring.units().get_team_units(0)?;
    let unit_id = *units
        .first()
        .ok_or(spring::ApiError::new(spring::ErrorCode::NotFound as i32))?;

    let (before_bytes, before_allocs) = memory_snapshot(spring)?;
    let small_start = common::timer_micros(spring)?;
    for _ in 0..common::count(100_000) {
        let _ = spring.units_info().get_unit_position(
            unit_id,
            spring::units_info::GetUnitPositionOptions::default(),
        );
    }
    let small_elapsed = common::timer_micros(spring)?.saturating_sub(small_start);
    let (small_bytes, small_allocs) = memory_snapshot(spring)?;
    common::send_row(
        spring,
        &format!(
            "{{\"backend\":\"wasm_core\",\"test\":\"mem_per_call_small\",\"status\":\"pass\",\"bytes\":{:.3},\"allocations\":{:.3},\"elapsedMs\":{:.3},\"scale\":{}}}",
            (small_bytes - before_bytes).max(0.0),
            (small_allocs - before_allocs).max(0.0),
            small_elapsed as f64 / 1_000.0,
            common::scale()
        ),
    );

    let (before_list_bytes, before_list_allocs) = memory_snapshot(spring)?;
    let list_start = common::timer_micros(spring)?;
    for _ in 0..common::count(1_000) {
        let _ = spring.units().get_team_units(0);
    }
    let list_elapsed = common::timer_micros(spring)?.saturating_sub(list_start);
    let (list_bytes, list_allocs) = memory_snapshot(spring)?;
    common::send_row(
        spring,
        &format!(
            "{{\"backend\":\"wasm_core\",\"test\":\"mem_per_call_list\",\"status\":\"pass\",\"bytes\":{:.3},\"allocations\":{:.3},\"elapsedMs\":{:.3},\"scale\":{}}}",
            (list_bytes - before_list_bytes).max(0.0),
            (list_allocs - before_list_allocs).max(0.0),
            list_elapsed as f64 / 1_000.0,
            common::scale()
        ),
    );

    let gc_start = common::timer_micros(spring)?;
    let gc_kb = spring.profiling().get_synced_gc_info(true)?;
    let gc_elapsed = common::timer_micros(spring)?.saturating_sub(gc_start);
    common::send_row(
        spring,
        &format!(
            "{{\"backend\":\"wasm_core\",\"test\":\"gc_pause\",\"status\":\"pass\",\"gcKB\":{gc_kb:.3},\"totalPauseMs\":{:.3},\"scale\":{}}}",
            gc_elapsed as f64 / 1_000.0,
            common::scale()
        ),
    );

    let mut frame_times = Vec::new();
    let frame_count = common::count(5_000);
    for _ in 0..frame_count {
        let start = common::timer_micros(spring)?;
        for unit in units.iter().take(common::count(1_000)) {
            let unit_id = *unit;
            let _ = spring.units_info().get_unit_def_id(unit_id);
            let _ = spring.units_info().get_unit_position(
                unit_id,
                spring::units_info::GetUnitPositionOptions::default(),
            );
        }
        frame_times.push(common::timer_micros(spring)?.saturating_sub(start) as f64 / 1_000.0);
    }
    frame_times.sort_by(|left, right| left.total_cmp(right));
    let p99_index = ((frame_times.len() as f64 * 0.99).ceil() as usize)
        .saturating_sub(1)
        .min(frame_times.len() - 1);
    common::send_row(
        spring,
        &format!(
            "{{\"backend\":\"wasm_core\",\"test\":\"frame_spike\",\"status\":\"pass\",\"worstMs\":{:.3},\"p99Ms\":{:.3},\"frames\":{},\"scale\":{}}}",
            frame_times[frame_times.len() - 1],
            frame_times[p99_index],
            frame_count,
            common::scale()
        ),
    );

    let (steady_bytes, _) = memory_snapshot(spring)?;
    common::send_row(
        spring,
        &format!(
            "{{\"backend\":\"wasm_core\",\"test\":\"mem_growth\",\"status\":\"pass\",\"peakBytes\":{:.3},\"steadyBytes\":{:.3},\"scale\":{}}}",
            steady_bytes.max(before_bytes),
            steady_bytes,
            common::scale()
        ),
    );
    common::send_complete(spring, "memory");
    Ok(())
}
