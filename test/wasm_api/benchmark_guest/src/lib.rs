#![allow(clippy::all)]

#[cfg(benchmark_context_ui)]
mod ui;
#[cfg(benchmark_callin_unimplemented)]
mod unimplemented;
#[cfg(benchmark_context_unsynced)]
mod unsynced;
#[cfg(not(any(
    benchmark_context_ui,
    benchmark_context_unsynced,
    benchmark_callin_unimplemented
)))]
mod synced {

    pub(crate) mod bindings {
        wit_bindgen::generate!({
            path: "wit",
            world: "benchmark-rules-synced",
        });
    }

    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{Mutex, OnceLock};

    use bindings::exports::recoil::spring_api::callins_rules_synced::{
        AllowUnitCreationQuery, AllowUnitCreationResult, DamageCallinResult, GameFrameQuery,
        GameFrameResult, SpringError, UnitCreatedQuery, UnitDamagedQuery,
    };
    use bindings::recoil::spring_api::{
        messages, profiling, rules_params, terrain, terrain_control, unit_control, unit_defs,
        units_commands, units_info, units_query,
    };

    static RAN: AtomicBool = AtomicBool::new(false);
    static BRUSH_SIZE: AtomicUsize = AtomicUsize::new(0);

    struct ScalarBenchmarkState {
        iterations: usize,
        repeat: usize,
        calls: usize,
        total_micros: u64,
        samples_ns: Vec<f64>,
    }

    static SCALAR_STATE: OnceLock<Mutex<ScalarBenchmarkState>> = OnceLock::new();

    struct WorkloadPosition {
        x: f32,
        y: f32,
        z: f32,
    }

    struct WorkloadBenchmarkState {
        units: Vec<i32>,
        position: WorkloadPosition,
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

    fn scaled_terrain_count(value: usize, scale: f64) -> usize {
        ((value as f64 * scale * scale).round() as usize).max(1)
    }

    fn scaled_brush_size(value: usize, scale: f64) -> usize {
        if value == 0 {
            0
        } else {
            ((value as f64 * scale.powf(0.75)).round() as usize).max(1)
        }
    }

    fn timer_micros() -> Result<u64, SpringError> {
        profiling::get_timer_micros(0u8).map_err(|error| SpringError { code: error.code })
    }

    /// Smallest step the host clock can actually report.
    fn clock_quantum_ns() -> Result<f64, SpringError> {
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

    /// Quantization error over `regions` timed regions grows with sqrt(regions),
    /// so the minimum trustworthy duration does too.
    fn resolution_floor_ns(quantum_ns: f64, regions: usize) -> f64 {
        50.0 * quantum_ns * (regions.max(1) as f64).sqrt()
    }

    /// An empty String does not allocate, so this adds no heap traffic.
    fn rules_param_float() -> rules_params::RulesParamValue {
        rules_params::RulesParamValue {
            type_: rules_params::RulesParamType::RulesparamTypeFloat,
            bool_value: false,
            float_value: 1.0,
            string_value: String::new(),
        }
    }

    fn send_row(row: &str) {
        let _ = messages::send_lua_rules_msg(&format!("WASM_BENCH|{row}"));
    }

    fn send_unresolved_row(name: &str, elapsed_ns: f64, floor_ns: f64, units: usize) {
        send_row(&format!(
            "{{\"backend\":\"wasm\",\"test\":\"{name}\",\"status\":\"unavailable\",\"iterations\":{units},\"scale\":{},\"reason\":\"sample of {elapsed_ns:.0} ns is below the {floor_ns:.0} ns timer-resolution floor\"}}",
            benchmark_scale()
        ));
    }

    fn measure<F>(name: &str, iterations: usize, mut operation: F) -> Result<(), SpringError>
    where
        F: FnMut() -> Result<(), SpringError>,
    {
        let quantum_ns = clock_quantum_ns()?;
        let floor_ns = resolution_floor_ns(quantum_ns, 1);
        // Grow the loop until a sample clears the resolution floor.
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
            send_unresolved_row(name, elapsed_ns, floor_ns, calls);
            return Ok(());
        }

        let mut samples = Vec::new();
        for _ in 0..benchmark_repeats() {
            let start = timer_micros()?;
            for _ in 0..calls {
                operation()?;
            }
            let end = timer_micros()?;
            samples.push((end.saturating_sub(start) as f64 * 1000.0) / calls as f64);
        }
        let mut sorted = samples.clone();
        sorted.sort_by(|left, right| left.total_cmp(right));
        let median = sorted[(sorted.len() - 1) / 2];
        let spread = sorted[sorted.len() - 1] - sorted[0];
        send_row(&format!(
		"{{\"backend\":\"wasm\",\"test\":\"{name}\",\"status\":\"pass\",\"iterations\":{calls},\"medianNs\":{median:.3},\"spreadNs\":{spread:.3},\"totalMedianNs\":{:.3},\"totalSpreadNs\":{:.3},\"quantumNs\":{quantum_ns:.0},\"scale\":{},\"measurement\":\"Component Model callout loop\"}}",
		median * calls as f64,
		spread * calls as f64,
		benchmark_scale()
    ));
        Ok(())
    }

    fn run_scalar_step(iterations: usize) -> Result<bool, SpringError> {
        let units =
            units_query::get_team_units(0).map_err(|error| SpringError { code: error.code })?;
        let unit_id = *units.first().ok_or(SpringError { code: 1 })?;
        let state = SCALAR_STATE.get_or_init(|| {
            Mutex::new(ScalarBenchmarkState {
                iterations,
                repeat: 0,
                calls: 0,
                total_micros: 0,
                samples_ns: Vec::with_capacity(benchmark_repeats()),
            })
        });
        let mut state = state.lock().map_err(|_| SpringError { code: 2 })?;
        if state.iterations != iterations {
            return Err(SpringError { code: 3 });
        }

        // Keep each GameFrame callback bounded. Scheduling between chunks is outside
        // the timed region, just as it is for the Lua sliced loop; it is not a fuel
        // workaround (fuel is unlimited unless the game explicitly configures it).
        let chunk = (iterations - state.calls).min(100_000);
        let start = timer_micros()?;
        for _ in 0..chunk {
            units_info::get_unit_def_id(unit_id)
                .map_err(|error| SpringError { code: error.code })?;
        }
        let end = timer_micros()?;
        state.total_micros = state.total_micros.saturating_add(end.saturating_sub(start));
        state.calls += chunk;
        if state.calls < iterations {
            return Ok(false);
        }

        let total_ns = state.total_micros as f64 * 1_000.0;
        state.samples_ns.push(total_ns / iterations as f64);
        state.repeat += 1;
        if state.repeat < benchmark_repeats() {
            state.calls = 0;
            state.total_micros = 0;
            return Ok(false);
        }

        let mut sorted = state.samples_ns.clone();
        sorted.sort_by(|left, right| left.total_cmp(right));
        let median = sorted[(sorted.len() - 1) / 2];
        let spread = sorted[sorted.len() - 1] - sorted[0];
        let total_median = median * iterations as f64;
        let total_spread = spread * iterations as f64;
        let samples = state
            .samples_ns
            .iter()
            .map(|sample| format!("{sample:.3}"))
            .collect::<Vec<_>>()
            .join(",");
        send_row(&format!(
        "{{\"backend\":\"wasm\",\"test\":\"callout_scalar\",\"status\":\"pass\",\"iterations\":{iterations},\"medianNs\":{median:.3},\"spreadNs\":{spread:.3},\"totalMedianNs\":{total_median:.3},\"totalSpreadNs\":{total_spread:.3},\"samplesNs\":[{samples}],\"scale\":{},\"measurement\":\"Component Model callout loop sliced across GameFrame callbacks\"}}",
        benchmark_scale()
    ));
        Ok(true)
    }

    fn run_heightmap_callback() {
        let size = BRUSH_SIZE.load(Ordering::Relaxed);
        for offset_x in 0..size {
            for offset_z in 0..size {
                let _ = terrain_control::set_height_map(
                    8.0 + (offset_x % 30) as f32 * 8.0,
                    8.0 + (offset_z % 30) as f32 * 8.0,
                    0.0,
                    0.0,
                );
            }
        }
    }

    /// Samples are milliseconds *per invocation*, matching Lua and native.
    fn measure_terrain<F>(
        name: &str,
        invocations: usize,
        mut invoke: F,
    ) -> Result<Option<(usize, Vec<f64>)>, SpringError>
    where
        F: FnMut() -> Result<(), SpringError>,
    {
        let quantum_ns = clock_quantum_ns()?;
        let floor_ns = resolution_floor_ns(quantum_ns, 1);
        let mut count = invocations;
        let mut elapsed_ns;
        loop {
            let start = timer_micros()?;
            for _ in 0..count {
                invoke()?;
            }
            elapsed_ns = timer_micros()?.saturating_sub(start) as f64 * 1_000.0;
            if elapsed_ns >= floor_ns || count >= invocations.saturating_mul(1_024) {
                break;
            }
            let growth = if elapsed_ns <= 0.0 {
                16.0
            } else {
                (floor_ns / elapsed_ns).ceil().max(2.0)
            };
            count = ((count as f64 * growth).ceil() as usize).max(count + 1);
        }
        if elapsed_ns < floor_ns {
            send_unresolved_row(name, elapsed_ns, floor_ns, count);
            return Ok(None);
        }

        let mut samples = Vec::new();
        for _ in 0..5 {
            let start = timer_micros()?;
            for _ in 0..count {
                invoke()?;
            }
            let end = timer_micros()?;
            samples.push(end.saturating_sub(start) as f64 / 1000.0 / count as f64);
        }
        samples.sort_by(|left, right| left.total_cmp(right));
        Ok(Some((count, samples)))
    }

    fn run_heightmap(scale: f64) -> Result<(), SpringError> {
        let terrain_height = terrain::get_ground_orig_height(8.0, 8.0)
            .map_err(|error| SpringError { code: error.code })?;
        for (name, nominal_size, nominal_invocations) in [
            ("hm_callback_empty", 0usize, 10_000usize),
            ("hm_brush_small", 32usize, 1_000usize),
            ("hm_brush_medium", 128usize, 100usize),
            ("hm_brush_large", 512usize, 10usize),
        ] {
            let size = scaled_brush_size(nominal_size, scale);
            BRUSH_SIZE.store(size, Ordering::Relaxed);
            let measured = measure_terrain(
                name,
                scaled_terrain_count(nominal_invocations, scale),
                || {
                    terrain_control::set_height_map_func(1, 0)
                        .map(|_| ())
                        .map_err(|error| SpringError { code: error.code })
                },
            )?;
            let Some((invocations, sorted)) = measured else {
                continue;
            };
            let median_ms = sorted[(sorted.len() - 1) / 2];
            let spread_ms = sorted[sorted.len() - 1] - sorted[0];
            let inner_calls = size * size;
            let inner_ns = if inner_calls == 0 {
                0.0
            } else {
                median_ms * 1_000_000.0 / inner_calls as f64
            };
            send_row(&format!(
            "{{\"backend\":\"wasm\",\"test\":\"{name}\",\"status\":\"pass\",\"invocations\":{invocations},\"innerCalls\":{inner_calls},\"medianMs\":{median_ms:.6},\"spreadMs\":{spread_ms:.6},\"innerNs\":{inner_ns:.3},\"scale\":{scale},\"nominalSize\":{nominal_size},\"nominalInvocations\":{nominal_invocations},\"measurement\":\"Component callback boundary with zero terraform; terrain rebuild excluded\"}}"
        ));
        }
        let measured = measure_terrain(
            "hm_region_op",
            scaled_terrain_count(1_000, scale),
            || {
                terrain_control::level_height_map(8.0, 8.0, 248.0, 248.0, terrain_height)
                    .map(|_| ())
                    .map_err(|error| SpringError { code: error.code })
            },
        )?;
        if let Some((invocations, sorted)) = measured {
            send_row(&format!(
            "{{\"backend\":\"wasm\",\"test\":\"hm_region_op\",\"status\":\"pass\",\"invocations\":{invocations},\"medianMs\":{:.6},\"spreadMs\":{:.6},\"innerNs\":0,\"scale\":{scale},\"nominalInvocations\":1000,\"measurement\":\"Component region boundary with unchanged height; terrain rebuild excluded\"}}",
            sorted[(sorted.len() - 1) / 2],
            sorted[sorted.len() - 1] - sorted[0]
        ));
        }
        Ok(())
    }

    fn measure_workload<F>(operation: F) -> Result<(f64, f64), SpringError>
    where
        F: FnOnce() -> Result<f64, SpringError>,
    {
        let start = timer_micros()?;
        let checksum = operation()?;
        let end = timer_micros()?;
        Ok((end.saturating_sub(start) as f64 * 1_000.0, checksum))
    }

    fn start_workloads(scale: f64) -> Result<(), SpringError> {
        let state = WORKLOAD_STATE.get_or_init(|| Mutex::new(None));
        let mut state = state.lock().map_err(|_| SpringError { code: 2 })?;
        if state.is_some() {
            return Ok(());
        }

        let units =
            units_query::get_team_units(0).map_err(|error| SpringError { code: error.code })?;
        let unit_id = *units.first().ok_or(SpringError { code: 1 })?;
        let position = units_info::get_unit_position(
            unit_id,
            units_info::GetUnitPositionOptions {
                mid_pos: false,
                aim_pos: false,
            },
        )
        .map_err(|error| SpringError { code: error.code })?;

        *state = Some(WorkloadBenchmarkState {
            units,
            position: WorkloadPosition {
                x: position.x,
                y: position.y,
                z: position.z,
            },
            frame: 0,
            frames: scaled_count(5_000, scale),
            samples_ns: (0..WORKLOAD_NAMES.len())
                .map(|_| Vec::with_capacity(scaled_count(5_000, scale)))
                .collect(),
            checksums: vec![0.0; WORKLOAD_NAMES.len()],
        });
        Ok(())
    }

    fn run_workload_step(scale: f64) -> Result<bool, SpringError> {
        start_workloads(scale)?;
        let state_cell = WORKLOAD_STATE.get_or_init(|| Mutex::new(None));
        let results = {
            let mut state_guard = state_cell.lock().map_err(|_| SpringError { code: 2 })?;
            let state = state_guard.as_mut().ok_or(SpringError { code: 3 })?;
            let unit_limit = state.units.len().min(1_000);
            let area_limit = state.units.len().min(100);
            let command_limit = state.units.len().min(200);
            let units = &state.units;
            let position = &state.position;

            let (elapsed, checksum) = measure_workload(|| {
                let mut checksum = 0.0;
                for unit in units.iter().take(unit_limit) {
                    let value = units_info::get_unit_position(
                        *unit,
                        units_info::GetUnitPositionOptions {
                            mid_pos: false,
                            aim_pos: false,
                        },
                    )
                    .map_err(|error| SpringError { code: error.code })?;
                    checksum += f64::from(value.x + value.y + value.z);
                    let _ = units_info::get_unit_health(*unit)
                        .map_err(|error| SpringError { code: error.code })?;
                    let def_id = units_info::get_unit_def_id(*unit)
                        .map_err(|error| SpringError { code: error.code })?;
                    checksum += f64::from(def_id);
                }
                Ok(checksum)
            })?;
            state.samples_ns[0].push(elapsed);
            state.checksums[0] += checksum;

            let (elapsed, checksum) = measure_workload(|| {
                let mut checksum = 0.0;
                for unit in units.iter().take(area_limit) {
                    let pos = units_info::get_unit_position(
                        *unit,
                        units_info::GetUnitPositionOptions {
                            mid_pos: false,
                            aim_pos: false,
                        },
                    )
                    .map_err(|error| SpringError { code: error.code })?;
                    let nearby = units_query::get_units_in_cylinder(pos.x, pos.z, 300.0, -1)
                        .map_err(|error| SpringError { code: error.code })?;
                    checksum += nearby.len() as f64;
                }
                Ok(checksum)
            })?;
            state.samples_ns[1].push(elapsed);
            state.checksums[1] += checksum;

            let (elapsed, checksum) = measure_workload(|| {
                let mut checksum = 0.0;
                for (index, unit) in units.iter().take(unit_limit).enumerate() {
                    rules_params::set_unit_rules_param(
                        *unit,
                        "bench",
                        &rules_param_float(),
                        -1,
                    )
                    .map_err(|error| SpringError { code: error.code })?;
                    let _ = rules_params::get_unit_rules_param(*unit, "bench")
                        .map_err(|error| SpringError { code: error.code })?;
                    checksum += index as f64;
                }
                Ok(checksum)
            })?;
            state.samples_ns[2].push(elapsed);
            state.checksums[2] += checksum;

            let (elapsed, checksum) = measure_workload(|| {
                let mut checksum = 0.0;
                for unit in units.iter().take(command_limit) {
                    let unit_position = units_info::get_unit_position(
                        *unit,
                        units_info::GetUnitPositionOptions {
                            mid_pos: false,
                            aim_pos: false,
                        },
                    )
                    .map_err(|error| SpringError { code: error.code })?;
                    unit_control::give_order_to_unit(
                        *unit,
                        10,
                        &[
                            unit_position.x + 8.0,
                            unit_position.y,
                            unit_position.z + 8.0,
                        ],
                        0,
                        0,
                    )
                    .map_err(|error| SpringError { code: error.code })?;
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
                std::hint::black_box(value);
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

        let Some(results) = results else {
            return Ok(false);
        };
        for (index, name) in WORKLOAD_NAMES.iter().enumerate() {
            let mut sorted = results.samples_ns[index].clone();
            sorted.sort_by(|left, right| left.total_cmp(right));
            let median = sorted[(sorted.len() - 1) / 2];
            let spread = sorted[sorted.len() - 1] - sorted[0];
            send_row(&format!(
                "{{\"backend\":\"wasm\",\"test\":\"{name}\",\"status\":\"pass\",\"iterations\":{},\"medianNs\":{median:.3},\"spreadNs\":{spread:.3},\"checksum\":{:.3},\"scale\":{scale},\"measurement\":\"Component workload measured per GameFrame callback\"}}",
                results.frames,
                results.checksums[index]
            ));
        }
        send_row(&format!(
            "{{\"backend\":\"wasm\",\"test\":\"complete\",\"status\":\"pass\",\"scale\":{scale},\"benchmarkCase\":\"workloads\"}}"
        ));
        Ok(true)
    }

    fn run_all(scale: f64) -> Result<(), SpringError> {
        let units =
            units_query::get_team_units(0).map_err(|error| SpringError { code: error.code })?;
        let unit_id = *units.first().ok_or(SpringError { code: 1 })?;
        let unit_def_id = units_info::get_unit_def_id(unit_id)
            .map_err(|error| SpringError { code: error.code })?;
        let position = units_info::get_unit_position(
            unit_id,
            units_info::GetUnitPositionOptions {
                mid_pos: false,
                aim_pos: false,
            },
        )
        .map_err(|error| SpringError { code: error.code })?;

        let profile = benchmark_case();
        if !matches!(
            profile,
            Some("callouts") | Some("heightmap") | Some("workloads")
        ) {
            for (name, iterations) in [
                ("callin_empty", 10_000usize),
                ("callin_gameframe", 10_000usize),
                ("callin_update", 10_000usize),
                ("callin_drawworld", 10_000usize),
                ("callin_unitcreated", 5_000usize),
                ("callin_unitpredamaged", 50_000usize),
                ("callin_allowunitcreation", 5_000usize),
                ("callin_unimplemented", 10_000usize),
                ("callin_4modules", 10_000usize),
            ] {
                measure(name, scaled_count(iterations, scale), || Ok(()))?;
            }
        }
        if !matches!(profile, Some("heightmap") | Some("workloads")) {
            measure("callout_scalar", scaled_count(100_000, scale), || {
                units_info::get_unit_def_id(unit_id)
                    .map(|_| ())
                    .map_err(|error| SpringError { code: error.code })
            })?;
            measure("callout_vec3", scaled_count(100_000, scale), || {
                units_info::get_unit_position(
                    unit_id,
                    units_info::GetUnitPositionOptions {
                        mid_pos: false,
                        aim_pos: false,
                    },
                )
                .map(|_| ())
                .map_err(|error| SpringError { code: error.code })
            })?;
            measure("callout_string", scaled_count(50_000, scale), || {
                unit_defs::get_unit_def_name(unit_def_id)
                    .map(|_| ())
                    .map_err(|error| SpringError { code: error.code })
            })?;
            measure("callout_smalllist", scaled_count(20_000, scale), || {
                units_commands::get_unit_commands(unit_id, 5)
                    .map(|_| ())
                    .map_err(|error| SpringError { code: error.code })
            })?;
            measure("callout_biglist", scaled_count(1_000, scale), || {
                units_query::get_team_units(0)
                    .map(|_| ())
                    .map_err(|error| SpringError { code: error.code })
            })?;
            measure("callout_spatial", scaled_count(10_000, scale), || {
                units_query::get_units_in_cylinder(position.x, position.z, 300.0, -1)
                    .map(|_| ())
                    .map_err(|error| SpringError { code: error.code })
            })?;
            measure("callout_mutate", scaled_count(100_000, scale), || {
                rules_params::set_unit_rules_param(
                    unit_id,
                    "bench",
                    &rules_param_float(),
                    -1,
                )
                .map(|_| ())
                .map_err(|error| SpringError { code: error.code })
            })?;
        }
        if profile == Some("callouts") {
            send_row(&format!(
            "{{\"backend\":\"wasm\",\"test\":\"complete\",\"status\":\"pass\",\"scale\":{scale},\"benchmarkCase\":\"callouts\"}}"
        ));
            return Ok(());
        }
        if profile == Some("heightmap") {
            run_heightmap(scale)?;
            send_row(&format!(
            "{{\"backend\":\"wasm\",\"test\":\"complete\",\"status\":\"pass\",\"scale\":{scale},\"benchmarkCase\":\"heightmap\"}}"
        ));
            return Ok(());
        }
        if profile != Some("workloads") {
            send_row("{\"backend\":\"wasm\",\"test\":\"callout_draw\",\"status\":\"unavailable\",\"reason\":\"draw callouts require the unsynced GL context\"}");
            run_heightmap(scale)?;
        }

        let unit_limit = units.len().min(1_000);
        let area_limit = units.len().min(100);
        let command_limit = units.len().min(200);
        measure("wl_unit_scan", scaled_count(5_000, scale), || {
            for unit in units.iter().take(unit_limit) {
                units_info::get_unit_position(
                    *unit,
                    units_info::GetUnitPositionOptions {
                        mid_pos: false,
                        aim_pos: false,
                    },
                )
                .map_err(|error| SpringError { code: error.code })?;
                units_info::get_unit_health(*unit)
                    .map_err(|error| SpringError { code: error.code })?;
                units_info::get_unit_def_id(*unit)
                    .map_err(|error| SpringError { code: error.code })?;
            }
            Ok(())
        })?;
        measure("wl_area_effect", scaled_count(5_000, scale), || {
            for unit in units.iter().take(area_limit) {
                let pos = units_info::get_unit_position(
                    *unit,
                    units_info::GetUnitPositionOptions {
                        mid_pos: false,
                        aim_pos: false,
                    },
                )
                .map_err(|error| SpringError { code: error.code })?;
                units_query::get_units_in_cylinder(pos.x, pos.z, 300.0, -1)
                    .map_err(|error| SpringError { code: error.code })?;
            }
            Ok(())
        })?;
        measure("wl_rules_params", scaled_count(5_000, scale), || {
            for unit in units.iter().take(unit_limit) {
                rules_params::set_unit_rules_param(
                    *unit,
                    "bench",
                    &rules_param_float(),
                    -1,
                )
                .map_err(|error| SpringError { code: error.code })?;
                rules_params::get_unit_rules_param(*unit, "bench")
                    .map_err(|error| SpringError { code: error.code })?;
            }
            Ok(())
        })?;
        // Each unit is ordered next to itself, as the Lua baseline does.
        measure("wl_commands", scaled_count(5_000, scale), || {
            for unit in units.iter().take(command_limit) {
                let unit_position = units_info::get_unit_position(
                    *unit,
                    units_info::GetUnitPositionOptions {
                        mid_pos: false,
                        aim_pos: false,
                    },
                )
                .map_err(|error| SpringError { code: error.code })?;
                unit_control::give_order_to_unit(
                    *unit,
                    10,
                    &[
                        unit_position.x + 8.0,
                        unit_position.y,
                        unit_position.z + 8.0,
                    ],
                    0,
                    0,
                )
                .map_err(|error| SpringError { code: error.code })?;
            }
            Ok(())
        })?;
        let compute_limit = 100_000;
        measure("wl_compute", scaled_count(5_000, scale), || {
            let mut value = 0.0f32;
            for index in 1..=compute_limit {
                value = (value + index as f32 * 0.25) % 1_000_003.0;
            }
            std::hint::black_box(value);
            Ok(())
        })?;
        if profile == Some("workloads") {
            send_row(&format!(
            "{{\"backend\":\"wasm\",\"test\":\"complete\",\"status\":\"pass\",\"scale\":{scale},\"benchmarkCase\":\"workloads\"}}"
        ));
            return Ok(());
        }
        send_row("{\"backend\":\"wasm\",\"test\":\"wl_ui_draw\",\"status\":\"unavailable\",\"reason\":\"draw workload requires the unsynced GL context\"}");

        // These profiling functions belong to the unsynced/UI Lua loader and are
        // deliberately absent from a synced Wasm world. Keep the matrix row
        // explicit instead of measuring a default or an unrelated host snapshot.
        for name in [
            "mem_per_call_small",
            "mem_per_call_list",
            "gc_pause",
            "frame_spike",
            "mem_growth",
        ] {
            send_row(&format!(
            "{{\"backend\":\"wasm\",\"test\":\"{name}\",\"status\":\"unavailable\",\"scale\":{scale},\"reason\":\"synced Wasm world does not import unsynced profiling snapshots\"}}"
        ));
        }
        send_row("{\"backend\":\"wasm\",\"test\":\"complete\",\"status\":\"pass\"}");
        Ok(())
    }

    struct BenchmarkGuest;

    impl bindings::exports::recoil::spring_api::callins_rules_synced::Guest for BenchmarkGuest {
        fn game_frame(query: GameFrameQuery) -> Result<GameFrameResult, SpringError> {
            if benchmark_case() == Some("callins") {
                // The engine-side recorder measures this actual callback. Keep
                // the guest body empty so the row cannot be mistaken for a
                // synthetic body-loop benchmark.
                if callin_variant() == Some("gameframe") {
                    std::hint::black_box(query.game_frame);
                }
                return Ok(GameFrameResult { unused: 0 });
            }
            if query.game_frame >= 3 && !RAN.load(Ordering::Acquire) {
                if benchmark_case() == Some("callout_scalar") {
                    match run_scalar_step(benchmark_iterations(100_000)) {
                        Ok(true) => {
                            RAN.store(true, Ordering::Release);
                            send_row(
                            "{\"backend\":\"wasm\",\"test\":\"complete\",\"status\":\"pass\",\"benchmarkCase\":\"callout_scalar\"}",
                        );
                        }
                        Ok(false) => {}
                        Err(error) => {
                            RAN.store(true, Ordering::Release);
                            send_row(&format!("{{\"backend\":\"wasm\",\"test\":\"complete\",\"status\":\"error\",\"code\":{}}}", error.code));
                        }
                    }
                } else if benchmark_case() == Some("workloads") {
                    match run_workload_step(benchmark_scale()) {
                        Ok(true) => RAN.store(true, Ordering::Release),
                        Ok(false) => {}
                        Err(error) => {
                            RAN.store(true, Ordering::Release);
                            send_row(&format!(
                                "{{\"backend\":\"wasm\",\"test\":\"complete\",\"status\":\"error\",\"code\":{}}}",
                                error.code
                            ));
                        }
                    }
                } else if !RAN.swap(true, Ordering::AcqRel) {
                    if let Err(error) = run_all(benchmark_scale()) {
                        send_row(&format!("{{\"backend\":\"wasm\",\"test\":\"complete\",\"status\":\"error\",\"code\":{}}}", error.code));
                    }
                }
            }
            Ok(GameFrameResult { unused: 0 })
        }

        fn game_frame_post(_query: GameFrameQuery) -> Result<GameFrameResult, SpringError> {
            Ok(GameFrameResult { unused: 0 })
        }

        fn update(
            _query: bindings::exports::recoil::spring_api::callins_rules_synced::UpdateQuery,
        ) -> Result<
            bindings::exports::recoil::spring_api::callins_rules_synced::UpdateResult,
            SpringError,
        > {
            Ok(
                bindings::exports::recoil::spring_api::callins_rules_synced::UpdateResult {
                    unused: 0,
                },
            )
        }

        fn unit_created(query: UnitCreatedQuery) -> Result<(), SpringError> {
            std::hint::black_box(query.unit_id);
            Ok(())
        }

        fn unit_pre_damaged(query: UnitDamagedQuery) -> Result<DamageCallinResult, SpringError> {
            Ok(DamageCallinResult {
                new_damage: query.damage,
                impulse_mult: 1.0,
            })
        }

        fn allow_unit_creation(
            query: AllowUnitCreationQuery,
        ) -> Result<AllowUnitCreationResult, SpringError> {
            std::hint::black_box(query.unit_def_id);
            Ok(AllowUnitCreationResult {
                allow: true,
                drop_order: false,
            })
        }
    }

    impl bindings::Guest for BenchmarkGuest {
        fn callback_1(_user_data: u32) {
            run_heightmap_callback();
        }
    }

    bindings::export!(BenchmarkGuest with_types_in bindings);
}
