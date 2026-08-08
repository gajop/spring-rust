use serde_json::Value;
use spring_native::prelude::*;
use std::{
    env,
    ffi::{CStr, CString},
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

const EPSILON: f32 = 0.05;

struct NativeApiParity {
    interface: NativeInterfaceRef,
    failures: u32,
    gfx_smoke_ran: bool,
    output_path: PathBuf,
    callin_trace_path: PathBuf,
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
