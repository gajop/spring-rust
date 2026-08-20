use std::hint::black_box;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};

use spring_wasm_core as spring;

static RAN: AtomicBool = AtomicBool::new(false);
static DRAW_RAN: AtomicBool = AtomicBool::new(false);

struct WorkloadBenchmarkState {
    units: Vec<i32>,
    frame: usize,
    frames: usize,
    samples_ns: Vec<Vec<f64>>,
    checksums: Vec<f64>,
}

struct WorkloadResults {
    frames: usize,
    samples_ns: Vec<Vec<f64>>,
    checksums: Vec<f64>,
}

static WORKLOAD_STATE: OnceLock<Mutex<Option<WorkloadBenchmarkState>>> = OnceLock::new();

const WORKLOAD_NAMES: [&str; 5] = [
    "wl_unit_scan",
    "wl_area_effect",
    "wl_rules_params",
    "wl_commands",
    "wl_compute",
];

fn benchmark_scale() -> f64 {
    option_env!("SPRING_BENCHMARK_SCALE")
        .and_then(|value| value.parse::<f64>().ok())
        .map(|value| value.clamp(0.0001, 1.0))
        .unwrap_or(1.0)
}

fn benchmark_case() -> Option<&'static str> {
    option_env!("SPRING_BENCHMARK_CASE").filter(|value| !value.is_empty())
}

fn benchmark_repeats() -> usize {
    option_env!("SPRING_BENCHMARK_REPEATS")
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(5)
}

fn benchmark_iterations(default: usize) -> usize {
    option_env!("SPRING_BENCHMARK_ITERATIONS")
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

fn callin_variant() -> Option<&'static str> {
    option_env!("SPRING_BENCHMARK_CALLIN_VARIANT").filter(|value| !value.is_empty())
}

fn scaled_count(value: usize, scale: f64) -> usize {
    ((value as f64 * scale).round() as usize).max(1)
}

fn timer_micros() -> spring::Result<u64> {
    spring::get_timer_micros()
}

fn clock_quantum_ns() -> spring::Result<f64> {
    let mut best = u64::MAX;
    for _ in 0..5 {
        let start = timer_micros()?;
        let mut now = start;
        while now == start {
            now = timer_micros()?;
        }
        best = best.min(now - start);
    }
    Ok(best.max(1) as f64 * 1_000.0)
}

fn resolution_floor_ns(quantum_ns: f64, regions: usize) -> f64 {
    50.0 * quantum_ns * (regions.max(1) as f64).sqrt()
}

fn send_row(row: &str) {
    let _ = spring::send_lua_rules_msg(&format!("WASM_BENCH|{row}"));
}

fn send_draw_row(row: &str) {
    let _ = spring::send_lua_ui_msg(&format!("WASM_DRAW|{row}"), "");
}

fn send_unavailable(name: &str, reason: &str) {
    send_row(&format!(
        "{{\"backend\":\"wasm_core\",\"test\":\"{name}\",\"status\":\"unavailable\",\"scale\":{},\"reason\":\"{reason}\"}}",
        benchmark_scale()
    ));
}

fn send_draw_unavailable(name: &str, reason: &str) {
    send_draw_row(&format!(
        "{{\"backend\":\"wasm_core\",\"test\":\"{name}\",\"status\":\"unavailable\",\"scale\":{},\"reason\":\"{reason}\"}}",
        benchmark_scale()
    ));
}

fn send_complete(case: &str) {
    send_row(&format!(
        "{{\"backend\":\"wasm_core\",\"test\":\"complete\",\"status\":\"pass\",\"scale\":{},\"benchmarkCase\":\"{case}\"}}",
        benchmark_scale()
    ));
}

fn measure<F>(name: &str, iterations: usize, mut operation: F) -> spring::Result<()>
where
    F: FnMut() -> spring::Result<()>,
{
    let quantum_ns = clock_quantum_ns()?;
    let floor_ns = resolution_floor_ns(quantum_ns, 1);
    let mut calls = iterations;
    let mut elapsed_ns;
    loop {
        let start = timer_micros()?;
        for _ in 0..calls {
            operation()?;
        }
        elapsed_ns = timer_micros()?.saturating_sub(start) as f64 * 1_000.0;
        if elapsed_ns >= floor_ns || calls >= iterations.saturating_mul(1_024) {
            break;
        }
        let growth = if elapsed_ns <= 0.0 {
            16.0
        } else {
            (floor_ns / elapsed_ns).ceil().max(2.0)
        };
        calls = ((calls as f64 * growth).ceil() as usize).max(calls + 1);
    }
    if elapsed_ns < floor_ns {
        send_row(&format!(
            "{{\"backend\":\"wasm_core\",\"test\":\"{name}\",\"status\":\"unavailable\",\"iterations\":{calls},\"scale\":{},\"reason\":\"sample of {elapsed_ns:.0} ns is below the {floor_ns:.0} ns timer-resolution floor\"}}",
            benchmark_scale()
        ));
        return Ok(());
    }

    let mut samples = Vec::with_capacity(benchmark_repeats());
    for _ in 0..benchmark_repeats() {
        let start = timer_micros()?;
        for _ in 0..calls {
            operation()?;
        }
        let end = timer_micros()?;
        samples.push((end.saturating_sub(start) as f64 * 1_000.0) / calls as f64);
    }
    let mut sorted = samples.clone();
    sorted.sort_by(|left, right| left.total_cmp(right));
    let median = sorted[(sorted.len() - 1) / 2];
    let spread = sorted[sorted.len() - 1] - sorted[0];
    send_row(&format!(
        "{{\"backend\":\"wasm_core\",\"test\":\"{name}\",\"status\":\"pass\",\"iterations\":{calls},\"medianNs\":{median:.3},\"spreadNs\":{spread:.3},\"totalMedianNs\":{:.3},\"totalSpreadNs\":{:.3},\"quantumNs\":{quantum_ns:.0},\"scale\":{},\"measurement\":\"Core Wasm callout loop\"}}",
        median * calls as f64,
        spread * calls as f64,
        benchmark_scale()
    ));
    Ok(())
}

fn owned_cylinder(x: f32, z: f32, radius: f32, allegiance: i32) -> spring::Result<Vec<i32>> {
    let mut empty: [i32; 0] = [];
    let first = spring::get_units_in_cylinder_into(x, z, radius, allegiance, &mut empty)?;
    let required = match first {
        spring::BufferFill::Complete(count) | spring::BufferFill::Insufficient { required: count } => count,
    };
    let mut units = vec![0i32; required];
    for _ in 0..3 {
        match spring::get_units_in_cylinder_into(x, z, radius, allegiance, &mut units)? {
            spring::BufferFill::Complete(count) => {
                units.truncate(count);
                return Ok(units);
            }
            spring::BufferFill::Insufficient { required } => units.resize(required, 0),
        }
    }
    Err(spring::ApiError::new(spring::ErrorCode::BufferOverflow as i32))
}

fn run_callouts(scale: f64, scalar_only: bool) -> spring::Result<()> {
    let units = spring::get_team_units(0)?;
    let unit_id = *units.first().ok_or(spring::ApiError::new(1))?;
    let unit_def_id = spring::get_unit_def_id(unit_id)?;
    let position = spring::get_unit_position(unit_id, false, false)?;

    measure(
        "callout_scalar",
        if scalar_only {
            benchmark_iterations(100_000)
        } else {
            scaled_count(100_000, scale)
        },
        || spring::get_unit_def_id(unit_id).map(|value| black_box(value)).map(|_| ()),
    )?;
    if scalar_only {
        send_complete("callout_scalar");
        return Ok(());
    }

    measure("callout_vec3", scaled_count(100_000, scale), || {
        spring::get_unit_position(unit_id, false, false)
            .map(|value| black_box(value))
            .map(|_| ())
    })?;
    measure("callout_string", scaled_count(50_000, scale), || {
        let bytes = spring::get_unit_def_name_bytes(unit_def_id)?;
        let name = String::from_utf8(bytes)
            .map_err(|_| spring::ApiError::new(spring::ErrorCode::Internal as i32))?;
        black_box(name);
        Ok(())
    })?;
    measure("callout_smalllist", scaled_count(20_000, scale), || {
        spring::get_unit_commands(unit_id, 5)
            .map(|value| black_box(value))
            .map(|_| ())
    })?;
    measure("callout_biglist", scaled_count(1_000, scale), || {
        spring::get_team_units(0)
            .map(|value| black_box(value))
            .map(|_| ())
    })?;
    measure("callout_spatial", scaled_count(10_000, scale), || {
        owned_cylinder(position[0], position[2], 300.0, -1)
            .map(|value| black_box(value))
            .map(|_| ())
    })?;
    measure("callout_mutate", scaled_count(100_000, scale), || {
        spring::set_unit_rules_param_f32(unit_id, "bench", 1.0, -1)
            .map(|value| black_box(value))
            .map(|_| ())
    })?;
    send_complete("callouts");
    Ok(())
}

fn measure_workload<F>(operation: F) -> spring::Result<(f64, f64)>
where
    F: FnOnce() -> spring::Result<f64>,
{
    let start = timer_micros()?;
    let checksum = operation()?;
    let end = timer_micros()?;
    Ok((end.saturating_sub(start) as f64 * 1_000.0, checksum))
}

fn start_workloads(scale: f64) -> spring::Result<()> {
    let cell = WORKLOAD_STATE.get_or_init(|| Mutex::new(None));
    let mut guard = cell
        .lock()
        .map_err(|_| spring::ApiError::new(spring::ErrorCode::InvalidState as i32))?;
    if guard.is_some() {
        return Ok(());
    }
    let units = spring::get_team_units(0)?;
    if units.is_empty() {
        return Err(spring::ApiError::new(spring::ErrorCode::NotFound as i32));
    }
    let frames = scaled_count(5_000, scale);
    *guard = Some(WorkloadBenchmarkState {
        units,
        frame: 0,
        frames,
        samples_ns: (0..WORKLOAD_NAMES.len())
            .map(|_| Vec::with_capacity(frames))
            .collect(),
        checksums: vec![0.0; WORKLOAD_NAMES.len()],
    });
    Ok(())
}

fn run_workload_step(scale: f64) -> spring::Result<bool> {
    start_workloads(scale)?;
    let cell = WORKLOAD_STATE.get_or_init(|| Mutex::new(None));
    let completed = {
        let mut guard = cell
            .lock()
            .map_err(|_| spring::ApiError::new(spring::ErrorCode::InvalidState as i32))?;
        let state = guard
            .as_mut()
            .ok_or(spring::ApiError::new(spring::ErrorCode::InvalidState as i32))?;
        let unit_limit = state.units.len().min(1_000);
        let area_limit = state.units.len().min(100);
        let command_limit = state.units.len().min(200);
        let units = &state.units;

        let (elapsed, checksum) = measure_workload(|| {
            let mut checksum = 0.0;
            for unit in units.iter().take(unit_limit) {
                let position = spring::get_unit_position(*unit, false, false)?;
                checksum += f64::from(position[0] + position[1] + position[2]);
                let _ = spring::get_unit_health(*unit)?;
                checksum += f64::from(spring::get_unit_def_id(*unit)?);
            }
            Ok(checksum)
        })?;
        state.samples_ns[0].push(elapsed);
        state.checksums[0] += checksum;

        let (elapsed, checksum) = measure_workload(|| {
            let mut checksum = 0.0;
            for unit in units.iter().take(area_limit) {
                let position = spring::get_unit_position(*unit, false, false)?;
                let nearby = owned_cylinder(position[0], position[2], 300.0, -1)?;
                checksum += nearby.len() as f64;
            }
            Ok(checksum)
        })?;
        state.samples_ns[1].push(elapsed);
        state.checksums[1] += checksum;

        let (elapsed, checksum) = measure_workload(|| {
            let mut checksum = 0.0;
            for (index, unit) in units.iter().take(unit_limit).enumerate() {
                spring::set_unit_rules_param_f32(*unit, "bench", 1.0, -1)?;
                let _ = spring::get_unit_rules_param_f32(*unit, "bench")?;
                checksum += index as f64;
            }
            Ok(checksum)
        })?;
        state.samples_ns[2].push(elapsed);
        state.checksums[2] += checksum;

        let (elapsed, checksum) = measure_workload(|| {
            let mut checksum = 0.0;
            for unit in units.iter().take(command_limit) {
                let position = spring::get_unit_position(*unit, false, false)?;
                spring::give_order_to_unit(
                    *unit,
                    10,
                    &[position[0] + 8.0, position[1], position[2] + 8.0],
                    0,
                    0,
                )?;
                checksum += 1.0;
            }
            Ok(checksum)
        })?;
        state.samples_ns[3].push(elapsed);
        state.checksums[3] += checksum;

        let (elapsed, checksum) = measure_workload(|| {
            let mut value = 0.0f32;
            for index in 1..=100_000 {
                value = (value + index as f32 * 0.25) % 1_000_003.0;
            }
            black_box(value);
            Ok(f64::from(value))
        })?;
        state.samples_ns[4].push(elapsed);
        state.checksums[4] += checksum;

        state.frame += 1;
        if state.frame < state.frames {
            None
        } else {
            Some(WorkloadResults {
                frames: state.frames,
                samples_ns: std::mem::take(&mut state.samples_ns),
                checksums: state.checksums.clone(),
            })
        }
    };

    let Some(results) = completed else {
        return Ok(false);
    };
    for (index, name) in WORKLOAD_NAMES.iter().enumerate() {
        let mut sorted = results.samples_ns[index].clone();
        sorted.sort_by(|left, right| left.total_cmp(right));
        let median = sorted[(sorted.len() - 1) / 2];
        let spread = sorted[sorted.len() - 1] - sorted[0];
        send_row(&format!(
            "{{\"backend\":\"wasm_core\",\"test\":\"{name}\",\"status\":\"pass\",\"iterations\":{},\"medianNs\":{median:.3},\"spreadNs\":{spread:.3},\"checksum\":{:.3},\"scale\":{scale},\"measurement\":\"Core Wasm workload measured per GameFrame callback\"}}",
            results.frames,
            results.checksums[index]
        ));
    }
    send_complete("workloads");
    Ok(true)
}

fn send_profile_unavailable(profile: &str) {
    let tests: &[&str] = match profile {
        "heightmap" => &[
            "hm_callback_empty",
            "hm_brush_small",
            "hm_brush_medium",
            "hm_brush_large",
            "hm_region_op",
        ],
        "memory" => &[
            "mem_per_call_small",
            "mem_per_call_list",
            "gc_pause",
            "frame_spike",
            "mem_growth",
        ],
        _ => &[],
    };
    for name in tests {
        send_unavailable(
            name,
            "Core benchmark ABI for this profile is not implemented yet; row intentionally left empty",
        );
    }
    send_complete(profile);
}

fn run_synced_once(frame: i32) {
    if frame < 3 || RAN.swap(true, Ordering::AcqRel) {
        return;
    }
    let result = match benchmark_case() {
        Some("callout_scalar") => run_callouts(benchmark_scale(), true),
        Some("callouts") => run_callouts(benchmark_scale(), false),
        Some("heightmap") => {
            send_profile_unavailable("heightmap");
            Ok(())
        }
        Some("callins") => Ok(()),
        Some(other) => {
            send_profile_unavailable(other);
            Ok(())
        }
        None => run_callouts(benchmark_scale(), false),
    };
    if let Err(error) = result {
        send_row(&format!(
            "{{\"backend\":\"wasm_core\",\"test\":\"complete\",\"status\":\"error\",\"code\":{}}}",
            error.code
        ));
    }
}

fn on_game_frame(frame: i32) {
    if benchmark_case() == Some("callins") {
        if callin_variant() == Some("gameframe") {
            black_box(frame);
        }
        return;
    }
    if benchmark_case() == Some("workloads") {
        if frame < 3 || RAN.load(Ordering::Acquire) {
            return;
        }
        match run_workload_step(benchmark_scale()) {
            Ok(true) => RAN.store(true, Ordering::Release),
            Ok(false) => {}
            Err(error) => {
                RAN.store(true, Ordering::Release);
                send_row(&format!(
                    "{{\"backend\":\"wasm_core\",\"test\":\"complete\",\"status\":\"error\",\"code\":{}}}",
                    error.code
                ));
            }
        }
        return;
    }
    run_synced_once(frame);
}

#[cfg(not(benchmark_callin_unimplemented))]
spring::export_game_frame!(on_game_frame);

fn on_game_frame_post(_frame: i32) {}
spring::export_game_frame_post!(on_game_frame_post);

fn on_update(delta_seconds: f32) {
    if benchmark_case() == Some("callins") {
        black_box(delta_seconds);
        return;
    }
    if benchmark_case() == Some("memory") && !RAN.swap(true, Ordering::AcqRel) {
        send_profile_unavailable("memory");
    }
}
spring::export_update!(on_update);

fn on_unit_created(unit_id: i32, unit_def_id: i32, unit_team: i32, builder_id: i32) {
    black_box((unit_id, unit_def_id, unit_team, builder_id));
}
spring::export_unit_created!(on_unit_created);

fn on_unit_pre_damaged(
    unit_id: i32,
    unit_def_id: i32,
    unit_team: i32,
    damage: f32,
    paralyzer: bool,
    weapon_def_id: i32,
    projectile_id: i32,
    attacker_id: i32,
    attacker_def_id: i32,
    attacker_team: i32,
) -> spring::DamageResult {
    black_box((
        unit_id,
        unit_def_id,
        unit_team,
        paralyzer,
        weapon_def_id,
        projectile_id,
        attacker_id,
        attacker_def_id,
        attacker_team,
    ));
    spring::DamageResult::unchanged(damage)
}
spring::export_unit_pre_damaged!(on_unit_pre_damaged);

fn on_allow_unit_creation(
    unit_def_id: i32,
    builder_id: i32,
    builder_team: i32,
    has_build_info: bool,
    build_pos: [f32; 3],
    build_facing: i32,
) -> spring::AllowUnitCreationResult {
    black_box((
        unit_def_id,
        builder_id,
        builder_team,
        has_build_info,
        build_pos,
        build_facing,
    ));
    spring::AllowUnitCreationResult::ALLOW
}
spring::export_allow_unit_creation!(on_allow_unit_creation);

fn on_draw_world() {
    if benchmark_case() != Some("draw") || DRAW_RAN.swap(true, Ordering::AcqRel) {
        return;
    }
    send_draw_unavailable(
        "callout_draw",
        "Core Gfx BeginEnd callback re-entry ABI is not implemented yet",
    );
    send_draw_unavailable(
        "wl_ui_draw",
        "Core Gfx BeginEnd callback re-entry ABI is not implemented yet",
    );
    send_draw_row(
        "{\"backend\":\"wasm_core\",\"test\":\"complete\",\"status\":\"pass\"}",
    );
}
spring::export_draw_world!(on_draw_world);
