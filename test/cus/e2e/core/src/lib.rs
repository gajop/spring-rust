/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use spring::cus::core_module::{CoreCusCallResult, CoreCusModule};
use spring::cus::wasm::WasmCus;
use spring::cus::{
    CusHandle, CusInstance, CusRegistry, Piece, ScriptCapabilities, TaskDefinition, UnitCtx,
    UnitScript, UnitScriptCall,
};
use spring::{DefId, TeamId, UnitId};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicI32, Ordering};

const INSTANCE_ID: u32 = 0xC05E_0002;
const CAPS: ScriptCapabilities =
    ScriptCapabilities::new(ScriptCapabilities::CREATE | ScriptCapabilities::QUERY_WEAPON);
static PENDING_UNIT: AtomicI32 = AtomicI32::new(-1);

fn record(message: &str) {
    let _ = spring::messages::log("cus-e2e", 1, message);
}

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
    fn new(_ctx: &mut spring::cus::InitCtx<'_>) -> Self {
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
    ctx.move_now(Piece(0), spring::cus::Axis::X, 1.0);
}

#[derive(Default)]
struct CusE2ECore {
    registry: CusRegistry<E2EScript>,
    handle: Option<CusHandle>,
    task_logged: bool,
}

impl CusE2ECore {
    fn attach_pending(&mut self) {
        if self.handle.is_some() {
            return;
        }
        let unit_id = PENDING_UNIT.swap(-1, Ordering::Relaxed);
        if unit_id < 0 {
            return;
        }
        let unit = UnitId(unit_id);
        let host = WasmCus::new(unit, INSTANCE_ID);
        let engine = Rc::new(RefCell::new(host.engine()));
        let instance = CusInstance::attach(unit, E2EScript::default(), Rc::clone(&engine));
        let handle = self.registry.attach(instance);
        self.handle = Some(handle);
        match WasmCus::attach(unit, INSTANCE_ID, CAPS) {
            Ok(_) => record(&format!("CUS_E2E|core|attached|unit={unit_id}")),
            Err(_) => {
                record("CUS_E2E|core|attach-error");
                self.handle = None;
                let _ = self.registry.detach(handle);
            }
        }
    }

    fn resumed(&mut self) -> bool {
        self.handle
            .and_then(|handle| {
                self.registry.with(handle, |instance| {
                    instance.with_state(|script| *script.resumed.borrow())
                })
            })
            .unwrap_or(false)
    }
}

impl CoreCusModule for CusE2ECore {
    fn cus_invoke(
        &mut self,
        instance_id: u32,
        call: u32,
        float_arguments: &[f32],
        integer_arguments: &[i32],
        result: &mut CoreCusCallResult<'_>,
    ) -> bool {
        if instance_id != INSTANCE_ID {
            return false;
        }
        let Some(call) = UnitScriptCall::from_u32(call) else {
            return false;
        };
        let Some(handle) = self.handle else {
            return false;
        };
        let mut call_result = spring::cus::UnitScriptCallResult::default();
        let handled = self
            .registry
            .with(handle, |instance| {
                instance.invoke(call, float_arguments, integer_arguments, &mut call_result)
            })
            .unwrap_or(false);
        if handled && call == UnitScriptCall::Create {
            record("CUS_E2E|core|create");
        }
        result.int_value = call_result.int_value;
        result.float_value = call_result.float_value;
        result.bool_value = call_result.bool_value;
        result.complete = call_result.complete;
        result.int_count = call_result.int_values.len().min(result.int_values.len());
        result.int_values[..result.int_count]
            .copy_from_slice(&call_result.int_values[..result.int_count]);
        handled
    }

    fn cus_call_named(
        &mut self,
        instance_id: u32,
        function_name: &str,
        arguments: &[f32],
        return_values: &mut [f32],
        found: &mut bool,
    ) -> Option<usize> {
        if instance_id != INSTANCE_ID || function_name != "e2e_named" {
            return None;
        }
        *found = true;
        if let (Some(argument), Some(output)) = (arguments.first(), return_values.first_mut()) {
            *output = *argument + 1.0;
        }
        record("CUS_E2E|core|named|found=1|success=1|value=4");
        Some(1)
    }

    fn cus_tick(&mut self, frame: u32) {
        self.attach_pending();
        if let Some(handle) = self.handle {
            self.registry.tick(frame as u64);
            if !self.task_logged && self.resumed() {
                self.task_logged = true;
                record(&format!("CUS_E2E|core|tick|frame={frame}|task_resumed=1"));
            }
            let _ = handle;
        }
    }

    fn cus_detach(&mut self, instance_id: u32) {
        if instance_id != INSTANCE_ID {
            return;
        }
        record("CUS_E2E|core|detach");
        if let Some(handle) = self.handle.take() {
            let _ = self.registry.detach(handle);
        }
    }
}

fn unit_created(unit: UnitId, _def: DefId, _team: TeamId, _builder: UnitId) {
    let _ = PENDING_UNIT.compare_exchange(-1, unit.0, Ordering::Relaxed, Ordering::Relaxed);
}

spring::export_core_cus!(CusE2ECore);
spring::export_environment_mask!(spring::rules_synced::ENVIRONMENT_MASK);
spring::export_unit_created!(unit_created);
