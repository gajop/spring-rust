use serde_json::Value;
use spring_native::prelude::*;
use std::{
    ffi::{CStr, CString},
    env,
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
}

type CheckFn = fn(&mut NativeApiParity, &Value, &str) -> Result<(), String>;
type SetFn = fn(&mut NativeApiParity, &Value) -> Result<(), String>;

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

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_tests.rs"));

impl NativeModule for NativeApiParity {
    fn new(interface: NativeInterfaceRef) -> Self {
        Self {
            interface,
            failures: 0,
            gfx_smoke_ran: false,
            output_path: env::var_os("SPRING_NATIVE_PARITY_OUTPUT_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("native_api_parity"))
                .join("native.jsonl"),
        }
    }

    fn handle_lua_call(&mut self, message: &str) -> Result<(), Error> {
        let parsed = match serde_json::from_str::<Value>(message) {
            Ok(parsed) => parsed,
            Err(err) => {
                self.failures += 1;
                self.record("parse", "fail", &format!("parse failure: {err}; message={message}"));
                return Ok(());
            }
        };

        let name = match parsed.get("name").and_then(Value::as_str) {
            Some(name) => name,
            None => {
                self.failures += 1;
                self.record("parse", "fail", &format!("missing string field `name`; message={message}"));
                return Ok(());
            }
        };

        let result = if name == "complete" {
            self.check_complete()
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

    fn draw_screen(&mut self) -> Result<(), Error> {
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
                self.record("gfx_compute_upload", "fail", &format!("Platform.is_headless failed: {err:?}"));
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
        Ok(())
    }
}

mod core;
mod game_checks;
mod query_checks;
mod metal_checks;
mod rules_checks;
mod unit_checks;
mod feature_checks;
mod terrain_checks;
mod los_checks;
mod defs_checks;
mod utils_checks;
mod commands_checks;
mod config_checks;
mod camera_checks;
mod display_checks;
mod input_checks;
mod icons_checks;
mod math_extra_checks;
mod platform_checks;
mod player_checks;
mod pieces_checks;
mod system_control_checks;
mod messages_checks;
mod selection_checks;
mod sound_checks;
mod profiling_checks;
mod projectiles_checks;
mod unsynced_read_checks;
mod vfs_checks;
mod known_mismatch_checks;
mod gfx_checks;
mod support;

spring_native::export_module!(NativeApiParity);
