/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use spring::{
    UnitId,
    cus::{
        Axis, InitCtx, Piece, ScriptCapabilities, TaskDefinition, UnitCtx, UnitScript,
        UnitScriptCall,
    },
};
use spring_native::cus::{CusHandle, CusInstance, CusRegistry, NativeCusCallResult};
use spring_native::prelude::*;
use std::cell::RefCell;
use std::env;
use std::ffi::{CStr, CString};
use std::fs::OpenOptions;
use std::io::Write;
use std::mem::MaybeUninit;
use std::rc::Rc;

const INSTANCE_ID: u32 = 0xC05E_0001;
const CAPS: ScriptCapabilities =
    ScriptCapabilities::new(ScriptCapabilities::CREATE | ScriptCapabilities::QUERY_WEAPON);

struct E2EScript {
    created: Rc<RefCell<bool>>,
    resumed: Rc<RefCell<bool>>,
}

impl Default for E2EScript {
    fn default() -> Self {
        Self {
            created: Rc::new(RefCell::new(false)),
            resumed: Rc::new(RefCell::new(false)),
        }
    }
}

impl UnitScript for E2EScript {
    fn new(_ctx: &mut InitCtx<'_>) -> Self {
        Self::default()
    }

    fn create(&mut self, ctx: &UnitCtx) {
        *self.created.borrow_mut() = true;
        ctx.spawn(TaskDefinition::with_state(
            "rust-cus-e2e-next-frame",
            Rc::clone(&self.resumed),
            resume_next_frame,
        ));
    }

    fn query_weapon(&mut self, _ctx: &UnitCtx, _weapon: spring::cus::WeaponId) -> Piece {
        Piece(7)
    }
}

async fn resume_next_frame(state: Rc<RefCell<bool>>, ctx: UnitCtx) {
    ctx.next_frame().await;
    *state.borrow_mut() = true;
    ctx.move_now(Piece(0), Axis::X, 1.0);
}

struct NamedResult {
    found: bool,
    success: bool,
    values: Vec<f32>,
}

struct CusE2ENative {
    interface: NativeInterfaceRef,
    registry: CusRegistry<E2EScript>,
    handle: Option<CusHandle>,
    unit: Option<i32>,
    mode: String,
    output: Option<std::path::PathBuf>,
    named_attempted: bool,
    ready_to_quit: bool,
    quit_requested: bool,
    fault_sent: bool,
    unsynced_random_checked: bool,
}

impl CusE2ENative {
    fn mode_is_core(&self) -> bool {
        self.mode == "core" || self.mode_is_core_fault()
    }

    // The Core module traps on a rules message while its CUS script is still
    // attached; the engine must drop the module and keep simulating.
    fn mode_is_core_fault(&self) -> bool {
        self.mode == "core-fault"
    }

    fn record(&self, event: &str) {
        let Some(path) = self.output.as_ref() else {
            return;
        };
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
            let _ = writeln!(file, "native|{event}");
        }
    }

    fn named_call(&self, unit: i32) -> Result<NamedResult, String> {
        let interface = unsafe { &*self.interface.as_ptr() };
        let synced = unsafe {
            interface
                .syncedCtrl
                .as_ref()
                .ok_or_else(|| "synced control API is unavailable".to_string())?
        };
        let unit_script = unsafe {
            synced
                .unitScript
                .as_ref()
                .ok_or_else(|| "unit-script API is unavailable".to_string())?
        };
        let call = unit_script
            .CallUnitScript
            .ok_or_else(|| "CallUnitScript is unavailable".to_string())?;
        let name = CString::new("e2e_named").map_err(|err| err.to_string())?;
        let arguments = [3.0f32];
        let query = sys::CallUnitScriptQuery {
            unitID: unit,
            functionName: name.as_ptr(),
            args: arguments.as_ptr(),
            argCount: arguments.len() as u32,
            retCapacity: 8,
        };
        let mut raw_result = MaybeUninit::<sys::CallUnitScriptResult>::zeroed();
        unsafe { call(&query, raw_result.as_mut_ptr()) };
        let raw_result = unsafe { raw_result.assume_init() };
        if !raw_result.error.is_null() {
            let error = unsafe { &*raw_result.error };
            let message = if error.message.is_null() {
                "unknown engine error".to_string()
            } else {
                unsafe { CStr::from_ptr(error.message) }
                    .to_string_lossy()
                    .into_owned()
            };
            return Err(format!("CallUnitScript failed: {message}"));
        }
        let values = if raw_result.retValues.is_null() || raw_result.retCount == 0 {
            Vec::new()
        } else {
            unsafe {
                std::slice::from_raw_parts(raw_result.retValues, raw_result.retCount as usize)
                    .to_vec()
            }
        };
        Ok(NamedResult {
            found: raw_result.functionFound,
            success: raw_result.success,
            values,
        })
    }

    fn script_resumed(&mut self) -> bool {
        self.handle
            .and_then(|handle| {
                self.registry.with(handle, |instance| {
                    instance.with_state(|script| *script.resumed.borrow())
                })
            })
            .unwrap_or(false)
    }
}

impl NativeModule for CusE2ENative {
    fn new(interface: NativeInterfaceRef) -> Self {
        Self {
            interface,
            registry: CusRegistry::default(),
            handle: None,
            unit: None,
            mode: env::var("SPRING_CUS_E2E_MODE").unwrap_or_else(|_| "native".to_string()),
            output: env::var_os("SPRING_CUS_E2E_OUTPUT").map(Into::into),
            named_attempted: false,
            ready_to_quit: false,
            quit_requested: false,
            fault_sent: false,
            unsynced_random_checked: false,
        }
    }

    fn unit_created(
        &mut self,
        unit_id: i32,
        _unit_def_id: i32,
        _unit_team: i32,
        _builder_id: i32,
    ) -> Result<(), Error> {
        if self.unit.is_some() {
            return Ok(());
        }
        self.unit = Some(unit_id);
        if self.mode_is_core() {
            self.record(&format!("core-unit-created|unit={unit_id}"));
            return Ok(());
        }

        let unit = UnitId(unit_id);
        let engine = Rc::new(RefCell::new(self.interface.cus().engine(unit, INSTANCE_ID)));
        let instance = CusInstance::attach(unit, E2EScript::default(), Rc::clone(&engine));
        let handle = self.registry.attach(instance);
        self.handle = Some(handle);
        self.record(&format!("attached|unit={unit_id}|instance={INSTANCE_ID}"));
        match self.interface.cus().attach(unit, INSTANCE_ID, CAPS)? {
            true => Ok(()),
            false => Err(Error::new(8, "native CUS attach returned false")),
        }
    }

    fn cus_invoke(
        &mut self,
        instance_id: u32,
        call: u32,
        float_arguments: &[f32],
        integer_arguments: &[i32],
        result: &mut NativeCusCallResult<'_>,
    ) -> Result<bool, Error> {
        if instance_id != INSTANCE_ID {
            return Ok(false);
        }
        let Some(call) = UnitScriptCall::from_u32(call) else {
            return Ok(false);
        };
        let Some(handle) = self.handle else {
            return Ok(false);
        };
        let mut call_result = spring::cus::UnitScriptCallResult::default();
        let handled = self
            .registry
            .with(handle, |instance| {
                instance.invoke(call, float_arguments, integer_arguments, &mut call_result)
            })
            .unwrap_or(false);
        if handled && call == UnitScriptCall::Create {
            self.record("create");
        }
        result.int_value = call_result.int_value;
        result.float_value = call_result.float_value;
        result.bool_value = call_result.bool_value;
        result.complete = call_result.complete;
        result.int_count = call_result.int_values.len().min(result.int_values.len());
        result.int_values[..result.int_count]
            .copy_from_slice(&call_result.int_values[..result.int_count]);
        Ok(handled)
    }

    fn cus_call_named(
        &mut self,
        instance_id: u32,
        function_name: &str,
        arguments: &[f32],
        return_values: &mut [f32],
        found: &mut bool,
    ) -> Result<Option<usize>, Error> {
        if instance_id != INSTANCE_ID || function_name != "e2e_named" {
            return Ok(None);
        }
        *found = true;
        if let Some(value) = arguments.first().copied()
            && let Some(output) = return_values.first_mut()
        {
            *output = value + 1.0;
        }
        self.record("named|found=1|success=1|value=4");
        Ok(Some(1))
    }

    fn cus_tick(&mut self, frame: u32) -> Result<(), Error> {
        let Some(handle) = self.handle else {
            return Ok(());
        };
        let Some(unit) = self.unit else {
            return Ok(());
        };
        self.registry.tick(frame as u64);
        let resumed = self.script_resumed();
        self.record(&format!(
            "tick|frame={frame}|task_resumed={}",
            resumed as u8
        ));
        if resumed && !self.mode_is_core() && !self.named_attempted {
            self.named_attempted = true;
            match self.named_call(unit) {
                Ok(result) => {
                    let value = result.values.first().copied().unwrap_or_default();
                    self.record(&format!(
                        "named|found={}|success={}|value={value}",
                        result.found as u8, result.success as u8
                    ));
                }
                Err(error) => self.record(&format!("named-error|{error}")),
            }
        }
        if resumed {
            self.ready_to_quit = true;
        }
        let _ = handle;
        Ok(())
    }

    fn cus_detach(&mut self, instance_id: u32) -> Result<(), Error> {
        if instance_id == INSTANCE_ID {
            self.record("detach");
            if let Some(handle) = self.handle.take() {
                let _ = self.registry.detach(handle);
            }
        }
        Ok(())
    }

    fn game_frame(&mut self, frame: i32) -> Result<(), Error> {
        if frame == 1 && !self.mode_is_core() {
            // Mirrors the fixture gadget's `math.random` calls after the same
            // reseed; draw early because the native run quits at frame 2.
            const SCALE: f32 = 16_777_216.0;
            let synced = self.interface.synced_ctrl();
            let random = synced.random();
            random.set_seed(12345)?;
            let draws = [
                (random.next_float()? * SCALE) as i64,
                random.next_int_up_to(10)? as i64,
                random.next_int(3, 7)? as i64,
                random.next_int(-5, 5)? as i64,
                (random.next_float()? * SCALE) as i64,
            ];
            let draws = draws.map(|draw| draw.to_string()).join("|");
            self.record(&format!("RNG|native|{draws}"));
        }
        let Some(unit) = self.unit else {
            return Ok(());
        };
        if self.mode_is_core() && !self.named_attempted && frame >= 5 {
            self.named_attempted = true;
            match self.named_call(unit) {
                Ok(result) => {
                    let value = result.values.first().copied().unwrap_or_default();
                    self.record(&format!(
                        "core-named|found={}|success={}|value={value}",
                        result.found as u8, result.success as u8
                    ));
                    if result.found
                        && result.success
                        && (value - 4.0).abs() < f32::EPSILON
                        && !self.mode_is_core_fault()
                    {
                        self.ready_to_quit = true;
                    }
                }
                Err(error) => self.record(&format!("core-named-error|{error}")),
            }
        }
        if self.mode_is_core_fault() && self.named_attempted && !self.fault_sent {
            self.fault_sent = true;
            match self.interface.messages().send_lua_rules_msg("cus-e2e-fault") {
                Ok(_) => self.record(&format!("core-fault-sent|frame={frame}")),
                Err(error) => self.record(&format!("core-fault-error|{error:?}")),
            }
        }
        if self.mode_is_core_fault() && self.fault_sent && frame >= 90 {
            self.record(&format!("core-fault-survived|frame={frame}"));
            self.ready_to_quit = true;
        }
        if self.ready_to_quit && !self.quit_requested {
            self.quit_requested = true;
            self.record(&format!("quit|frame={frame}"));
            if let Err(error) = self.interface.system_control().quit() {
                self.record(&format!("quit-error|{error:?}"));
            }
        }
        Ok(())
    }

    fn update(&mut self, _delta_seconds: f32) -> Result<(), Error> {
        if self.unsynced_random_checked {
            return Ok(());
        }
        self.unsynced_random_checked = true;

        // Unsynced draws cannot be compared with Lua's; check the range and
        // that reseeding reproduces the sequence.
        let unsynced = self.interface.unsynced_ctrl();
        let random = unsynced.random();
        let draw = || -> Result<(f32, i32, i32), Error> {
            random.set_seed(4242)?;
            Ok((
                random.next_float()?,
                random.next_int_up_to(6)?,
                random.next_int(-3, 3)?,
            ))
        };
        let first = draw()?;
        let second = draw()?;
        let in_range = (0.0..1.0).contains(&first.0)
            && (1..=6).contains(&first.1)
            && (-3..=3).contains(&first.2);
        self.record(&format!(
            "RNG|native-unsynced|in_range={}|reseed_repeats={}",
            in_range as u8,
            (first == second) as u8
        ));

        // Hiding the built-in interface is idempotent and reversible.
        const ALL: u32 = (1 << 12) - 1;
        let hidden = unsynced.set_default_interface_visible(ALL, false)?;
        let hidden_again = unsynced.set_default_interface_visible(ALL, false)?;
        let shown = unsynced.set_default_interface_visible(ALL, true)?;
        let final_hidden = unsynced.set_default_interface_visible(ALL, false)?;
        self.record(&format!(
            "UI|default-interface|hidden={hidden}|again={hidden_again}|shown={shown}|final={final_hidden}"
        ));
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), Error> {
        self.record("shutdown");
        Ok(())
    }
}

spring_native::export_module!(CusE2ENative);
