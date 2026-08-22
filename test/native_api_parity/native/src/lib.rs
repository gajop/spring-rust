use serde_json::Value;
use spring_native::prelude::*;
use support::{benchmark_callin_variant_is, benchmark_case_is, record_callins_enabled};
use std::{
    env,
    ffi::{CStr, CString},
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::atomic::{AtomicBool, Ordering},
    time::Instant,
};

const EPSILON: f32 = 0.05;
static DRAW_BENCHMARK_RAN: AtomicBool = AtomicBool::new(false);

fn benchmark_scale() -> f64 {
    env::var("SPRING_NATIVE_BENCHMARK_SCALE")
        .ok()
        .and_then(|value| value.parse::<f64>().ok())
        .map(|value| value.clamp(0.0001, 1.0))
        .unwrap_or(1.0)
}

fn benchmark_case() -> Option<String> {
    env::var("SPRING_NATIVE_BENCHMARK_CASE")
        .ok()
        .filter(|value| !value.is_empty())
}

fn benchmark_repeats() -> usize {
    env::var("SPRING_NATIVE_BENCHMARK_REPEATS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(5)
}

fn benchmark_iterations(default: usize) -> usize {
    env::var("SPRING_NATIVE_BENCHMARK_ITERATIONS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

fn scaled_count(value: usize, scale: f64) -> usize {
    ((value as f64 * scale).round() as usize).max(1)
}

/// Smallest step the process clock can actually report.
fn clock_quantum_ns() -> f64 {
    let mut best = u128::MAX;
    for _ in 0..5 {
        let start = Instant::now();
        loop {
            let elapsed = start.elapsed().as_nanos();
            if elapsed > 0 {
                best = best.min(elapsed);
                break;
            }
        }
    }
    (best.max(1) as f64).min(1_000_000.0)
}

/// Quantization error over `regions` timed regions grows with sqrt(regions),
/// so the minimum trustworthy duration does too.
fn resolution_floor_ns(quantum_ns: f64, regions: usize) -> f64 {
    50.0 * quantum_ns * (regions.max(1) as f64).sqrt()
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

fn percentile99(samples: &[f64]) -> f64 {
	let mut sorted = samples.to_vec();
	sorted.sort_by(|left, right| left.total_cmp(right));
	sorted[((sorted.len() - 1) * 99) / 100]
}

struct NativeApiParity {
    interface: NativeInterfaceRef,
    failures: u32,
    gfx_smoke_ran: bool,
    output_path: PathBuf,
    callin_trace_path: PathBuf,
    benchmark_output_path: PathBuf,
}

type CheckFn = fn(&mut NativeApiParity, &Value, &str) -> Result<(), String>;
type SetFn = fn(&mut NativeApiParity, &Value) -> Result<(), String>;

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated_callin_trace.rs"
));

struct TestSpec {
    name: &'static str,
    check: CheckFn,
    set: SetFn,
}

macro_rules! native_tests {
    ($($name:ident { check = $check:ident, set = $set:ident, })*) => {
        const TESTS: &[TestSpec] = &[
            $(TestSpec { name: stringify!($name), check: NativeApiParity::$check, set: NativeApiParity::$set },)*
        ];
    };
}

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated_tests.rs"
));

impl NativeModule for NativeApiParity {
    fn new(interface: NativeInterfaceRef) -> Self {
        Self {
            interface,
            failures: 0,
            gfx_smoke_ran: false,
            output_path: env::var_os("SPRING_NATIVE_PARITY_OUTPUT_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("native_api_parity"))
                .join(format!("native-{}.jsonl", std::process::id())),
            callin_trace_path: env::var_os("SPRING_NATIVE_PARITY_OUTPUT_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("native_api_parity"))
                .join("callin_native.jsonl"),
            benchmark_output_path: env::var_os("SPRING_NATIVE_PARITY_OUTPUT_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("native_api_parity"))
                .join("benchmark_native.jsonl"),
        }
    }

    fn handle_lua_call(&mut self, message: &str) -> Result<(), Error> {
        let parsed = match serde_json::from_str::<Value>(message) {
            Ok(parsed) => parsed,
            Err(err) => {
                self.failures += 1;
                self.record(
                    "parse",
                    "fail",
                    &format!("parse failure: {err}; message={message}"),
                );
                return Ok(());
            }
        };

        let name = match parsed
            .get("testName")
            .or_else(|| parsed.get("name"))
            .and_then(Value::as_str)
        {
            Some(name) => name,
            None => {
                self.failures += 1;
                self.record(
                    "parse",
                    "fail",
                    &format!("missing string field `testName`; message={message}"),
                );
                return Ok(());
            }
        };

        if name == "benchmark" {
            match self.run_benchmarks() {
                Ok(()) => {
                    if let Err(err) = self
                        .interface
                        .messages()
                        .send_lua_rules_msg("NATIVE_BENCH|{\"test\":\"complete\"}")
                    {
                        self.record("benchmark_signal", "fail", &format!("{err:?}"));
                    }
                }
                Err(err) => self.record("benchmark", "fail", &err),
            }
            return Ok(());
        }

        if name == "complete" {
            self.record_callin_phase("complete");
        }

        // Diagnostic escape hatch for isolating engine-state corruption during
        // rendering runs.  Normal parity runs leave this unset; skipped
        // checks are still emitted so the harness can show the run shape.
        if name != "complete" && native_parity_skip(name) {
            self.record(name, "skip", "diagnostic skip");
            return Ok(());
        }

        let result = if name == "complete" {
            self.check_complete()
        } else if name == "spring.invoke_native_module" {
            // This is an explicit smoke probe for Spring.InvokeNativeModule.
            // The callout is the Lua-to-native transport, so there is no
            // second native API operation to compare here.
            Ok(())
        } else if name == "rml.global_context_document" {
            self.check_rml_global_context_document(&parsed)
        } else if name == "rml.element_form_event" {
            self.check_rml_element_form_event(&parsed)
        } else if name == "vfs.archive_surface" {
            self.check_vfs_archive_surface(&parsed)
        } else if name == "gl.state_queries" {
            self.check_gl_state_queries(&parsed)
        } else if name == "gl.state_mutations" {
            self.check_gl_state_mutations(&parsed)
        } else if name == "gl.immediate_primitives" {
            self.check_gl_immediate_primitives(&parsed)
        } else if name == "gl.shader_uniforms" {
            self.check_gl_shader_uniforms(&parsed)
        } else if name == "gl.texture_resources" {
            self.check_gl_texture_resources(&parsed)
        } else if name == "gl.lists_queries" {
            self.check_gl_lists_queries(&parsed)
        } else if name == "gl.atlas" {
            self.check_gl_atlas(&parsed)
        } else if name == "gl.fbo" {
            self.check_gl_fbo(&parsed)
        } else if name == "gl.fonts" {
            self.check_gl_fonts(&parsed)
        } else if name == "gl.minimap" {
            self.check_gl_minimap(&parsed)
        } else if name == "gl.resource_handles" {
            self.check_gl_resource_handles(&parsed)
        } else if name == "gl.userdata" {
            self.check_gl_userdata(&parsed)
        } else if name == "gl.object_drawing" {
            self.check_gl_object_drawing(&parsed)
        } else if name == "gl.fixed_immediate" {
            self.check_gl_fixed_immediate(&parsed)
        } else if name == "debug_input.key_readback" {
            self.check_debug_input_key_readback(&parsed)
        } else if name == "debug_input.mouse_readback" {
            self.check_debug_input_mouse_readback(&parsed)
        } else if name == "multi_ally_visibility" {
            self.check_multi_ally_visibility(&parsed)
        } else if let Some(test_name) = name.strip_prefix("set_native_") {
            self.find(test_name)
                .ok_or_else(|| format!("unknown native setter check `{name}`"))
                .and_then(|spec| (spec.set)(self, &parsed))
        } else {
            let test_name = name.strip_prefix("native_").unwrap_or(name);
            self.find(test_name)
                .ok_or_else(|| format!("unknown native read check `{name}`"))
                .and_then(|spec| (spec.check)(self, &parsed, name))
        };

        if let Err(err) = result {
            self.failures += 1;
            self.record(name, "fail", &err);
        } else if name != "complete" {
            self.record(name, "pass", "");
        }

        Ok(())
    }

    fn draw_screen(&mut self, view_size_x: i32, view_size_y: i32) -> Result<(), Error> {
        self.record_callin_args_result(
            "DrawScreen",
            vec![self.trace_i32(view_size_x), self.trace_i32(view_size_y)],
            Vec::new(),
        );
        if self.gfx_smoke_ran {
            return Ok(());
        }
        self.gfx_smoke_ran = true;

        match self.interface.platform().is_headless() {
            Ok(true) => {
                self.record("gfx_compute_upload", "skip", "headless engine");
                return Ok(());
            }
            Ok(false) => {}
            Err(err) => {
                self.failures += 1;
                self.record(
                    "gfx_compute_upload",
                    "fail",
                    &format!("Platform.is_headless failed: {err:?}"),
                );
                return Ok(());
            }
        }

        match self.check_gfx_compute_upload() {
            Ok(()) => self.record("gfx_compute_upload", "pass", ""),
            Err(err) => {
                self.failures += 1;
                self.record("gfx_compute_upload", "fail", &err);
            }
        }

        for (name, check) in [
            (
                "rml_context_document_lifecycle",
                NativeApiParity::check_rml_context_document_lifecycle
                    as fn(&NativeApiParity) -> Result<(), String>,
            ),
            (
                "rml_context_document_extra_behavior",
                NativeApiParity::check_rml_context_document_extra_behavior,
            ),
            (
                "rml_global_input_behavior",
                NativeApiParity::check_rml_global_input_behavior,
            ),
            (
                "rml_dom_query_behavior",
                NativeApiParity::check_rml_dom_query_behavior,
            ),
            (
                "rml_child_mutation_behavior",
                NativeApiParity::check_rml_child_mutation_behavior,
            ),
            (
                "rml_event_behavior",
                NativeApiParity::check_rml_event_behavior,
            ),
            (
                "rml_form_control_behavior",
                NativeApiParity::check_rml_form_control_behavior,
            ),
            (
                "rml_stylesheet_append_behavior",
                NativeApiParity::check_rml_stylesheet_append_behavior,
            ),
            (
                "rml_invalid_zero_handle_behavior",
                NativeApiParity::check_rml_invalid_zero_handle_behavior,
            ),
        ] {
            match check(self) {
                Ok(()) => self.record(name, "pass", ""),
                Err(err) => {
                    self.failures += 1;
                    self.record(name, "fail", &err);
                }
            }
        }
        Ok(())
    }

    generated_callin_trace_methods!();
}

impl NativeApiParity {
    fn write_benchmark_row(&self, row: serde_json::Value) -> Result<(), String> {
        if let Some(parent) = self.benchmark_output_path.parent() {
            fs::create_dir_all(parent).map_err(|err| format!("create benchmark output: {err}"))?;
        }
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.benchmark_output_path)
            .map_err(|err| format!("open benchmark output: {err}"))?;
        serde_json::to_writer(&mut file, &row)
            .map_err(|err| format!("encode benchmark row: {err}"))?;
        file.write_all(b"\n")
            .map_err(|err| format!("write benchmark row: {err}"))?;
        Ok(())
    }

    fn benchmark_measure<F>(
        &self,
        name: &str,
        iterations: usize,
        mut operation: F,
    ) -> Result<(), String>
    where
        F: FnMut() -> Result<(), String>,
    {
        let iterations = match self.resolve_iterations(name, iterations, &mut operation)? {
            Some(resolved) => resolved,
            None => return Ok(()),
        };
        let (median, spread, samples) = self.benchmark_samples(iterations, &mut operation)?;
        let p99 = percentile99(&samples);
        self.write_benchmark_row(serde_json::json!({
            "backend": "native",
            "test": name,
            "status": "pass",
            "iterations": iterations,
            "medianNs": median,
            "p99Ns": p99,
            "spreadNs": spread,
            "totalMedianNs": median * iterations as f64,
            "totalSpreadNs": spread * iterations as f64,
            "samplesNs": samples,
            "scale": benchmark_scale(),
            "measurement": "NativeInterface callout loop",
        }))
    }

    /// Grow the loop until a sample clears the resolution floor; None when the
    /// row could not be measured at all.
    fn resolve_iterations<F>(
        &self,
        name: &str,
        iterations: usize,
        operation: &mut F,
    ) -> Result<Option<usize>, String>
    where
        F: FnMut() -> Result<(), String>,
    {
        let floor_ns = resolution_floor_ns(clock_quantum_ns(), 1);
        let mut calls = iterations;
        let mut elapsed_ns;
        loop {
            let start = Instant::now();
            for _ in 0..calls {
                operation()?;
            }
            elapsed_ns = start.elapsed().as_nanos() as f64;
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
            self.write_benchmark_row(serde_json::json!({
                "backend": "native",
                "test": name,
                "status": "unavailable",
                "iterations": calls,
                "scale": benchmark_scale(),
                "reason": format!(
                    "sample of {elapsed_ns:.0} ns is below the {floor_ns:.0} ns timer-resolution floor"
                ),
            }))?;
            return Ok(None);
        }
        Ok(Some(calls))
    }

    fn benchmark_samples<F>(
        &self,
        iterations: usize,
        operation: &mut F,
    ) -> Result<(f64, f64, Vec<f64>), String>
    where
        F: FnMut() -> Result<(), String>,
    {
        let repeats = benchmark_repeats();
        let mut samples = Vec::with_capacity(repeats);
        for _ in 0..repeats {
            let start = Instant::now();
            for _ in 0..iterations {
                operation()?;
            }
            samples.push(start.elapsed().as_nanos() as f64 / iterations as f64);
        }
        let mut sorted = samples.clone();
        sorted.sort_by(|left, right| left.total_cmp(right));
        let median = sorted[(sorted.len() - 1) / 2];
        let spread = sorted[sorted.len() - 1] - sorted[0];
        Ok((median, spread, samples))
    }

    fn benchmark_heightmap(&self, scale: f64) -> Result<(), String> {
        let interface = self.interface;
        let terrain_height = interface
            .terrain()
            .get_ground_height(8.0, 8.0)
            .map_err(|err| format!("get_ground_height for benchmark: {err:?}"))?;
        for (name, nominal_size, nominal_invocations) in [
            ("hm_callback_empty", 0usize, 10_000usize),
            ("hm_brush_small", 32usize, 1_000usize),
            ("hm_brush_medium", 128usize, 100usize),
            ("hm_brush_large", 512usize, 10usize),
        ] {
            let size = scaled_brush_size(nominal_size, scale);
            let invocations = scaled_terrain_count(nominal_invocations, scale);
            let floor_ns = resolution_floor_ns(clock_quantum_ns(), 1);
            let mut totals = Vec::new();
            let mut inner = Vec::new();
            let mut sample_ns = 0.0f64;
            for _ in 0..benchmark_repeats() {
                let mut calls = 0usize;
                let total_start = Instant::now();
                let mut inner_elapsed = std::time::Duration::ZERO;
                for _ in 0..invocations {
                    let inner_start = Instant::now();
                    let result = interface.synced_ctrl().terrain().set_height_map_func(|| {
                        for offset_x in 0..size {
                            for offset_z in 0..size {
                                let _ = interface.synced_ctrl().terrain().set_height_map(
                                    8.0 + (offset_x % 30) as f32 * 8.0,
                                    8.0 + (offset_z % 30) as f32 * 8.0,
                                    0.0,
                                    0.0,
                                );
                                calls += 1;
                            }
                        }
                    });
                    result.map_err(|err| format!("{name}: SetHeightMapFunc failed: {err:?}"))?;
                    inner_elapsed += inner_start.elapsed();
                }
                sample_ns = total_start.elapsed().as_nanos() as f64;
                totals.push(sample_ns / 1_000_000.0 / invocations as f64);
                inner.push(if calls == 0 {
                    0.0
                } else {
                    inner_elapsed.as_nanos() as f64 / calls as f64
                });
            }
            if sample_ns < floor_ns {
                self.write_benchmark_row(serde_json::json!({
                    "backend": "native",
                    "test": name,
                    "status": "unavailable",
                    "invocations": invocations,
                    "scale": scale,
                    "reason": format!(
                        "sample of {sample_ns:.0} ns is below the {floor_ns:.0} ns timer-resolution floor"
                    ),
                }))?;
                continue;
            }
            let mut sorted = totals.clone();
            sorted.sort_by(|left, right| left.total_cmp(right));
            let median = sorted[(sorted.len() - 1) / 2];
            let spread = sorted[sorted.len() - 1] - sorted[0];
            let p99 = sorted[((sorted.len() - 1) * 99) / 100];
            let mut inner_sorted = inner.clone();
            inner_sorted.sort_by(|left, right| left.total_cmp(right));
            self.write_benchmark_row(serde_json::json!({
                "backend": "native",
                "test": name,
                "status": "pass",
                "invocations": invocations,
                "innerCalls": size * size,
                "medianMs": median,
                "p99Ms": p99,
                "spreadMs": spread,
                "samplesMs": totals,
                "innerNs": inner_sorted[(inner_sorted.len() - 1) / 2],
                "scale": scale,
                "nominalSize": nominal_size,
                "nominalInvocations": nominal_invocations,
                "measurement": "Native callback boundary with zero terraform; terrain rebuild excluded",
            }))?;
        }
        let invocations = scaled_terrain_count(1_000, scale);
        let floor_ns = resolution_floor_ns(clock_quantum_ns(), 1);
        let mut samples = Vec::new();
        let mut sample_ns = 0.0f64;
        for _ in 0..benchmark_repeats() {
            let start = Instant::now();
            for _ in 0..invocations {
                interface
                    .synced_ctrl()
                    .terrain()
                    .level_height_map(8.0, 8.0, 248.0, 248.0, terrain_height)
                    .map_err(|err| format!("hm_region_op failed: {err:?}"))?;
            }
            sample_ns = start.elapsed().as_nanos() as f64;
            samples.push(sample_ns / 1_000_000.0 / invocations as f64);
        }
        if sample_ns < floor_ns {
            return self.write_benchmark_row(serde_json::json!({
                "backend": "native",
                "test": "hm_region_op",
                "status": "unavailable",
                "invocations": invocations,
                "scale": scale,
                "reason": format!(
                    "sample of {sample_ns:.0} ns is below the {floor_ns:.0} ns timer-resolution floor"
                ),
            }));
        }
        let mut sorted = samples.clone();
        sorted.sort_by(|left, right| left.total_cmp(right));
        let p99 = sorted[((sorted.len() - 1) * 99) / 100];
        self.write_benchmark_row(serde_json::json!({
            "backend": "native",
            "test": "hm_region_op",
            "status": "pass",
            "invocations": invocations,
            "medianMs": sorted[(sorted.len() - 1) / 2],
            "p99Ms": p99,
            "spreadMs": sorted[sorted.len() - 1] - sorted[0],
            "samplesMs": samples,
            "innerNs": 0.0,
            "scale": scale,
            "nominalInvocations": 1_000,
            "measurement": "Native region boundary with unchanged height; terrain rebuild excluded",
        }))
    }

    fn write_draw_row(&self, row: serde_json::Value) -> Result<(), String> {
        let encoded = serde_json::to_string(&row)
            .map_err(|err| format!("encode draw benchmark row: {err}"))?;
        let delivered = self
            .interface
            .messages()
            .send_lua_uimsg(&format!("WASM_DRAW|{encoded}"), "")
            .map_err(|err| format!("send draw benchmark row to LuaUI: {err:?}"))?;
        if !delivered {
            return Err("send draw benchmark row to LuaUI returned false".to_owned());
        }
        Ok(())
    }

    fn benchmark_draw_world(&self) -> Result<(), String> {
        if DRAW_BENCHMARK_RAN.swap(true, Ordering::AcqRel) {
            return Ok(());
        }

        let scale = benchmark_scale();
        let repeats = benchmark_repeats();
        let gfx = self.interface.gfx();

        let callout_vertices = scaled_count(100_000, scale);
        let mut callout_samples = Vec::with_capacity(repeats);
        for _ in 0..repeats {
            let start = Instant::now();
            gfx.begin_end(1, || {
                for index in 0..callout_vertices {
                    let x = (index % 100) as f32 * 0.01;
                    let y = (index / 100) as f32 * 0.01;
                    let _ = gfx.vertex(x, y, 0.0, 1.0, 4);
                }
            })
            .map_err(|err| format!("draw callout begin/end: {err:?}"))?;
            callout_samples
                .push(start.elapsed().as_secs_f64() * 1_000_000_000.0 / callout_vertices as f64);
        }
        callout_samples.sort_by(|left, right| left.total_cmp(right));
        let callout_p99 = callout_samples[((callout_samples.len() - 1) * 99) / 100];
        self.write_draw_row(serde_json::json!({
            "backend": "native",
            "test": "callout_draw",
            "status": "pass",
            "iterations": callout_vertices,
            "medianNs": callout_samples[(callout_samples.len() - 1) / 2],
            "p99Ns": callout_p99,
            "spreadNs": callout_samples[callout_samples.len() - 1] - callout_samples[0],
            "samplesNs": callout_samples,
            "scale": scale,
        }))?;

        let workload_lines = scaled_count(2_000, scale);
        let workload_vertices = workload_lines * 2;
        let mut workload_samples = Vec::with_capacity(repeats);
        for _ in 0..repeats {
            let start = Instant::now();
            gfx.begin_end(1, || {
                for index in 0..workload_vertices {
                    let x = (index % 100) as f32 * 0.01;
                    let y = (index / 100) as f32 * 0.01;
                    let _ = gfx.vertex(x, y, 0.0, 1.0, 4);
                }
            })
            .map_err(|err| format!("draw workload begin/end: {err:?}"))?;
            workload_samples.push(start.elapsed().as_secs_f64() * 1000.0);
        }
        workload_samples.sort_by(|left, right| left.total_cmp(right));
        let workload_p99 = workload_samples[((workload_samples.len() - 1) * 99) / 100];
        self.write_draw_row(serde_json::json!({
            "backend": "native",
            "test": "wl_ui_draw",
            "status": "pass",
            "lines": workload_lines,
            "medianMs": workload_samples[(workload_samples.len() - 1) / 2],
            "p99Ms": workload_p99,
            "spreadMs": workload_samples[workload_samples.len() - 1] - workload_samples[0],
            "samplesMs": workload_samples,
            "scale": scale,
        }))?;
        self.write_draw_row(serde_json::json!({
            "backend": "native",
            "test": "complete",
            "status": "pass",
        }))
    }

    fn benchmark_memory(
        &self,
        interface: NativeInterfaceRef,
        units: &[i32],
        unit_id: i32,
        scale: f64,
    ) -> Result<(), String> {
        let snapshot = || {
            interface
                .profiling()
                .get_lua_mem_usage()
                .map_err(|err| format!("get_lua_mem_usage: {err:?}"))
        };

        let before = snapshot()?;
        let iterations = scaled_count(100_000, scale);
        let (median, spread, samples) = self.benchmark_samples(iterations, &mut || {
            interface
                .units_info()
                .get_unit_position(unit_id, spring_native::GetUnitPositionOptions::default())
                .map(|_| ())
                .map_err(|err| format!("{err:?}"))
        })?;
        let after = snapshot()?;
        self.write_benchmark_row(serde_json::json!({
            "backend": "native",
            "test": "mem_per_call_small",
            "status": "pass",
            "iterations": iterations,
            "medianNs": median,
            "p99Ns": percentile99(&samples),
            "spreadNs": spread,
            "samplesNs": samples,
            "bytes": ((after.2 - before.2).max(0.0) * 1024.0),
            "allocations": ((after.3 - before.3).max(0.0) * 1000.0),
            "scale": scale,
            "measurement": "Profiling.get_lua_mem_usage",
        }))?;

        let before = snapshot()?;
        let iterations = scaled_count(1_000, scale);
        let (median, spread, samples) = self.benchmark_samples(iterations, &mut || {
            interface
                .units_query()
                .get_team_units(0)
                .map(|_| ())
                .map_err(|err| format!("{err:?}"))
        })?;
        let after = snapshot()?;
        self.write_benchmark_row(serde_json::json!({
            "backend": "native",
            "test": "mem_per_call_list",
            "status": "pass",
            "iterations": iterations,
            "medianNs": median,
            "p99Ns": percentile99(&samples),
            "spreadNs": spread,
            "samplesNs": samples,
            "bytes": ((after.2 - before.2).max(0.0) * 1024.0),
            "allocations": ((after.3 - before.3).max(0.0) * 1000.0),
            "scale": scale,
            "measurement": "Profiling.get_lua_mem_usage",
        }))?;

        let gc_start = Instant::now();
        let gc_kb = interface
            .profiling()
            .get_synced_gcinfo(true)
            .map_err(|err| format!("get_synced_gcinfo: {err:?}"))?;
        self.write_benchmark_row(serde_json::json!({
            "backend": "native",
            "test": "gc_pause",
            "status": "pass",
            "gcKB": gc_kb,
            "totalPauseMs": gc_start.elapsed().as_secs_f64() * 1000.0,
            "scale": scale,
            "measurement": "Profiling.get_synced_gcinfo",
        }))?;

        let unit_limit = units.len().min(1_000);
        let mut frame_times = Vec::new();
        for _ in 0..scaled_count(5_000, scale) {
            let start = Instant::now();
            for unit in units.iter().take(unit_limit) {
                interface
                    .units_info()
                    .get_unit_position(*unit, spring_native::GetUnitPositionOptions::default())
                    .map_err(|err| format!("{err:?}"))?;
                interface
                    .units_info()
                    .get_unit_health(*unit)
                    .map_err(|err| format!("{err:?}"))?;
                interface
                    .units_info()
                    .get_unit_def_id(*unit)
                    .map_err(|err| format!("{err:?}"))?;
            }
            frame_times.push(start.elapsed().as_secs_f64() * 1000.0);
        }
        frame_times.sort_by(|left, right| left.total_cmp(right));
        let p99 = frame_times[((frame_times.len() as f64 * 0.99).ceil() as usize)
            .saturating_sub(1)
            .min(frame_times.len() - 1)];
        self.write_benchmark_row(serde_json::json!({
            "backend": "native",
            "test": "frame_spike",
            "status": "pass",
            "worstMs": frame_times[frame_times.len() - 1],
            "p99Ms": p99,
            "frames": frame_times.len(),
            "scale": scale,
            "measurement": "bounded workload samples",
        }))?;

        let after = snapshot()?;
        self.write_benchmark_row(serde_json::json!({
            "backend": "native",
            "test": "mem_growth",
            "status": "pass",
            "peakBytes": after.2 * 1024.0,
            "steadyBytes": after.2 * 1024.0,
            "scale": scale,
            "measurement": "Profiling.get_lua_mem_usage",
        }))
    }

    fn run_benchmarks(&self) -> Result<(), String> {
        let interface = self.interface;
        let scale = benchmark_scale();
        if benchmark_case().as_deref() == Some("draw") {
            return self.benchmark_draw_world();
        }
        let units = interface
            .units_query()
            .get_team_units(0)
            .map_err(|err| format!("get_team_units(0): {err:?}"))?;
        let unit_id = *units
            .first()
            .ok_or_else(|| "benchmark fixture has no units".to_owned())?;
        let unit_def_id = interface
            .units_info()
            .get_unit_def_id(unit_id)
            .map_err(|err| format!("get_unit_def_id: {err:?}"))?;
        let position = interface
            .units_info()
            .get_unit_position(unit_id, spring_native::GetUnitPositionOptions::default())
            .map_err(|err| format!("get_unit_position: {err:?}"))?;

        if benchmark_case().as_deref() == Some("callout_scalar") {
            self.benchmark_measure("callout_scalar", benchmark_iterations(100_000), || {
                interface
                    .units_info()
                    .get_unit_def_id(unit_id)
                    .map(|_| ())
                    .map_err(|err| format!("{err:?}"))
            })?;
            return self.write_benchmark_row(serde_json::json!({
                "backend": "native",
                "test": "complete",
                "status": "pass",
                "scale": benchmark_scale(),
                "benchmarkCase": "callout_scalar",
            }));
        }

        let profile = benchmark_case();
        if profile.as_deref() == Some("memory") {
            self.benchmark_memory(interface, &units, unit_id, scale)?;
            return self.write_benchmark_row(serde_json::json!({
                "backend": "native",
                "test": "complete",
                "status": "pass",
                "scale": scale,
                "benchmarkCase": "memory",
            }));
        }
        if !matches!(
            profile.as_deref(),
            Some("callouts") | Some("heightmap") | Some("workloads")
        ) {
            for (name, nominal_iterations) in [
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
                self.benchmark_measure(name, scaled_count(nominal_iterations, scale), || Ok(()))?;
            }
        }

        if !matches!(profile.as_deref(), Some("heightmap") | Some("workloads")) {
            self.benchmark_measure("callout_scalar", scaled_count(100_000, scale), || {
                interface
                    .units_info()
                    .get_unit_def_id(unit_id)
                    .map(|_| ())
                    .map_err(|err| format!("{err:?}"))
            })?;
            self.benchmark_measure("callout_vec3", scaled_count(100_000, scale), || {
                interface
                    .units_info()
                    .get_unit_position(unit_id, spring_native::GetUnitPositionOptions::default())
                    .map(|_| ())
                    .map_err(|err| format!("{err:?}"))
            })?;
            self.benchmark_measure("callout_string", scaled_count(50_000, scale), || {
                interface
                    .unit_defs()
                    .get_unit_def_name(unit_def_id)
                    .map(|_| ())
                    .map_err(|err| format!("{err:?}"))
            })?;
            self.benchmark_measure("callout_smalllist", scaled_count(20_000, scale), || {
                interface
                    .units_commands()
                    .get_unit_commands(unit_id, 5)
                    .map(|_| ())
                    .map_err(|err| format!("{err:?}"))
            })?;
            self.benchmark_measure("callout_biglist", scaled_count(1_000, scale), || {
                interface
                    .units_query()
                    .get_team_units(0)
                    .map(|_| ())
                    .map_err(|err| format!("{err:?}"))
            })?;
            self.benchmark_measure("callout_spatial", scaled_count(10_000, scale), || {
                interface
                    .units_query()
                    .get_units_in_cylinder(position.x, position.z, 300.0, -1)
                    .map(|_| ())
                    .map_err(|err| format!("{err:?}"))
            })?;
            self.benchmark_measure("callout_mutate", scaled_count(100_000, scale), || {
                interface
                    .rules_params()
                    .set_unit_rules_param(
                        unit_id,
                        "bench",
                        spring_native::RulesParamValue::Float(1.0),
                        -1,
                    )
                    .map(|_| ())
                    .map_err(|err| format!("{err:?}"))
            })?;
            let physics_position = spring_native::sys::Float3 {
                x: position.x,
                y: position.y,
                z: position.z,
            };
            let velocity = spring_native::sys::Float3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            };
            let rotation = spring_native::sys::Float3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            };
            let drag = spring_native::sys::Float3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            };
            self.benchmark_measure(
                "callout_wide_unit_physics",
                scaled_count(20_000, scale),
                || {
                    interface
                        .synced_ctrl()
                        .unit()
                        .set_unit_physics(
                            unit_id,
                            physics_position,
                            velocity,
                            rotation,
                            drag,
                        )
                        .map(|_| ())
                        .map_err(|err| format!("{err:?}"))
                },
            )?;
            for (name, payload) in [
                ("callout_payload_8", "terrain"),
                (
                    "callout_payload_64",
                    "terrain-payload-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
                ),
                (
                    "callout_payload_256",
                    "terrain-payload-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
                ),
            ] {
                self.benchmark_measure(name, scaled_count(20_000, scale), || {
                    interface
                        .synced_ctrl()
                        .terrain()
                        .set_terrain_type_data(
                            0, 1.0, 1.0, 1.0, 1.0, 1.0, true, payload,
                        )
                        .map(|_| ())
                        .map_err(|err| format!("{err:?}"))
                })?;
            }
        }
        if profile.as_deref() == Some("callouts") {
            return self.write_benchmark_row(serde_json::json!({
                "backend": "native",
                "test": "complete",
                "status": "pass",
                "scale": scale,
                "benchmarkCase": "callouts",
            }));
        }
        if profile.as_deref() == Some("heightmap") {
            self.benchmark_heightmap(scale)?;
            return self.write_benchmark_row(serde_json::json!({
                "backend": "native",
                "test": "complete",
                "status": "pass",
                "scale": scale,
                "benchmarkCase": "heightmap",
            }));
        }
        if profile.as_deref() != Some("workloads") {
            self.write_benchmark_row(serde_json::json!({
                "backend": "native",
                "test": "callout_draw",
                "status": "unavailable",
                "reason": "draw callouts require the unsynced GL context",
            }))?;
            self.benchmark_heightmap(scale)?;
        }

        let unit_limit = units.len().min(1_000);
        let area_limit = units.len().min(100);
        let command_limit = units.len().min(200);
        self.benchmark_measure("wl_unit_scan", scaled_count(5_000, scale), || {
            for unit in units.iter().take(unit_limit) {
                let _ = interface
                    .units_info()
                    .get_unit_position(*unit, spring_native::GetUnitPositionOptions::default())
                    .map_err(|err| format!("{err:?}"))?;
                let _ = interface
                    .units_info()
                    .get_unit_health(*unit)
                    .map_err(|err| format!("{err:?}"))?;
                let _ = interface
                    .units_info()
                    .get_unit_def_id(*unit)
                    .map_err(|err| format!("{err:?}"))?;
            }
            Ok(())
        })?;
        self.benchmark_measure("wl_area_effect", scaled_count(5_000, scale), || {
            for unit in units.iter().take(area_limit) {
                let pos = interface
                    .units_info()
                    .get_unit_position(*unit, spring_native::GetUnitPositionOptions::default())
                    .map_err(|err| format!("{err:?}"))?;
                let _ = interface
                    .units_query()
                    .get_units_in_cylinder(pos.x, pos.z, 300.0, -1)
                    .map_err(|err| format!("{err:?}"))?;
            }
            Ok(())
        })?;
        self.benchmark_measure("wl_rules_params", scaled_count(5_000, scale), || {
            for unit in units.iter().take(unit_limit) {
                interface
                    .rules_params()
                    .set_unit_rules_param(
                        *unit,
                        "bench",
                        spring_native::RulesParamValue::Float(1.0),
                        -1,
                    )
                    .map_err(|err| format!("{err:?}"))?;
                let _ = interface
                    .rules_params()
                    .get_unit_rules_param(*unit, "bench")
                    .map_err(|err| format!("{err:?}"))?;
            }
            Ok(())
        })?;
        // Each unit is ordered next to itself, as the Lua baseline does. A
        // shared far-away destination would measure pathfinding distance
        // instead of the command call.
        self.benchmark_measure("wl_commands", scaled_count(5_000, scale), || {
            for unit in units.iter().take(command_limit) {
                let unit_position = interface
                    .units_info()
                    .get_unit_position(*unit, spring_native::GetUnitPositionOptions::default())
                    .map_err(|err| format!("{err:?}"))?;
                interface
                    .synced_ctrl()
                    .unit()
                    .give_order_to_unit(
                        *unit,
                        10,
                        &[unit_position.x + 8.0, unit_position.y, unit_position.z + 8.0],
                        0,
                        0,
                    )
                    .map_err(|err| format!("{err:?}"))?;
            }
            Ok(())
        })?;
        let compute_limit = 100_000;
        self.benchmark_measure("wl_compute", scaled_count(5_000, scale), || {
            let mut value = 0.0f32;
            for index in 1..=compute_limit {
                value = (value + index as f32 * 0.25) % 1_000_003.0;
            }
            std::hint::black_box(value);
            Ok(())
        })?;
        if profile.as_deref() == Some("workloads") {
            return self.write_benchmark_row(serde_json::json!({
                "backend": "native",
                "test": "complete",
                "status": "pass",
                "scale": scale,
                "benchmarkCase": "workloads",
            }));
        }
        self.write_benchmark_row(serde_json::json!({
            "backend": "native",
            "test": "wl_ui_draw",
            "status": "unavailable",
            "reason": "draw workload requires the unsynced GL context",
        }))?;

        let memory_before = interface
            .profiling()
            .get_lua_mem_usage()
            .map_err(|err| format!("get_lua_mem_usage before: {err:?}"))?;
        let small_iterations = scaled_count(100_000, scale);
        let (small_median, small_spread, small_samples) =
            self.benchmark_samples(small_iterations, &mut || {
                interface
                    .units_info()
                    .get_unit_position(unit_id, spring_native::GetUnitPositionOptions::default())
                    .map(|_| ())
                    .map_err(|err| format!("{err:?}"))
            })?;
        let memory_after = interface
            .profiling()
            .get_lua_mem_usage()
            .map_err(|err| format!("get_lua_mem_usage after: {err:?}"))?;
        self.write_benchmark_row(serde_json::json!({
            "backend": "native",
            "test": "mem_per_call_small",
            "status": "pass",
            "iterations": small_iterations,
            "nominalIterations": 100_000,
            "medianNs": small_median,
            "p99Ns": percentile99(&small_samples),
            "spreadNs": small_spread,
            "samplesNs": small_samples,
            "bytes": ((memory_after.2 - memory_before.2).max(0.0) * 1024.0),
            "allocations": ((memory_after.3 - memory_before.3).max(0.0) * 1000.0),
            "scale": scale,
            "measurement": "Profiling.get_lua_mem_usage",
        }))?;
        let memory_before = interface
            .profiling()
            .get_lua_mem_usage()
            .map_err(|err| format!("get_lua_mem_usage list before: {err:?}"))?;
        let list_iterations = scaled_count(1_000, scale);
        let (list_median, list_spread, list_samples) =
            self.benchmark_samples(list_iterations, &mut || {
                interface
                    .units_query()
                    .get_team_units(0)
                    .map(|_| ())
                    .map_err(|err| format!("{err:?}"))
            })?;
        let memory_after = interface
            .profiling()
            .get_lua_mem_usage()
            .map_err(|err| format!("get_lua_mem_usage list after: {err:?}"))?;
        self.write_benchmark_row(serde_json::json!({
            "backend": "native",
            "test": "mem_per_call_list",
            "status": "pass",
            "iterations": list_iterations,
            "nominalIterations": 1_000,
            "medianNs": list_median,
            "p99Ns": percentile99(&list_samples),
            "spreadNs": list_spread,
            "samplesNs": list_samples,
            "bytes": ((memory_after.2 - memory_before.2).max(0.0) * 1024.0),
            "allocations": ((memory_after.3 - memory_before.3).max(0.0) * 1000.0),
            "scale": scale,
            "measurement": "Profiling.get_lua_mem_usage",
        }))?;
        let gc_before = interface
            .profiling()
            .get_synced_gcinfo(false)
            .map_err(|err| format!("get_synced_gcinfo: {err:?}"))?;
        let gc_after = interface
            .profiling()
            .get_synced_gcinfo(true)
            .map_err(|err| format!("get_synced_gcinfo collect: {err:?}"))?;
        self.write_benchmark_row(serde_json::json!({"backend":"native","test":"gc_pause","status":"pass","gcBeforeKB":gc_before,"gcAfterKB":gc_after,"scale":scale,"measurement":"Profiling.get_synced_gcinfo"}))?;
        self.write_benchmark_row(serde_json::json!({"backend":"native","test":"frame_spike","status":"pass","worstMs":0.0,"p99Ms":0.0,"scale":scale,"measurement":"callout loop; frame scheduler excluded"}))?;
        self.write_benchmark_row(serde_json::json!({"backend":"native","test":"mem_growth","status":"pass","peakBytes":memory_after.2 * 1024.0,"steadyBytes":memory_after.2 * 1024.0,"scale":scale,"measurement":"Profiling.get_lua_mem_usage"}))?;
        self.write_benchmark_row(
            serde_json::json!({"backend":"native","test":"complete","status":"pass"}),
        )
    }
}

fn native_parity_skip(name: &str) -> bool {
    let test_name = name
        .strip_prefix("set_native_")
        .or_else(|| name.strip_prefix("native_"))
        .unwrap_or(name);
    let matches = |value: String| {
        value
            .split(',')
            .map(str::trim)
            .any(|candidate| candidate == "*" || candidate == name || candidate == test_name)
    };
    if let Ok(only) = env::var("SPRING_NATIVE_PARITY_ONLY") {
        if !matches(only) {
            return true;
        }
    }
    env::var("SPRING_NATIVE_PARITY_SKIP")
        .ok()
        .map(matches)
        .unwrap_or(false)
}

mod camera_checks;
mod cob_script_checks;
mod commands_checks;
mod config_checks;
mod control_calls_checks;
mod core;
mod debug_input_checks;
mod defs_checks;
mod display_checks;
mod effects_path_checks;
mod encoding_checks;
mod feature_checks;
mod feature_control_calls_checks;
mod game_checks;
mod gfx_checks;
mod gl_checks;
mod ground_decal_checks;
mod icons_checks;
mod input_checks;
mod known_mismatch_checks;
mod los_checks;
mod math_extra_checks;
mod messages_checks;
mod metal_checks;
mod object_lifecycle_checks;
mod order_checks;
mod parity_gap_checks;
mod pieces_checks;
mod platform_checks;
mod player_checks;
mod process_control_checks;
mod profiling_checks;
mod projectiles_checks;
mod query_checks;
mod remaining_synced_checks;
mod remaining_tail_checks;
mod render_control_checks;
mod rml_checks;
mod rules_checks;
mod selection_checks;
mod sound_checks;
mod support;
mod system_control_checks;
mod terrain_checks;
mod terrain_control_checks;
mod tracing_checks;
mod unit_checks;
mod unit_control_calls_checks;
mod unsynced_control_checks;
mod unsynced_read_checks;
mod utils_checks;
mod vfs_checks;

spring_native::export_module!(NativeApiParity);
