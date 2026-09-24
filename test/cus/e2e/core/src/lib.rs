/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use spring::cus::core_module::{CoreCusCallResult, CoreCusModule};
use spring::cus::host::CusHost;
use spring::cus::{Piece, ScriptCapabilities, TaskDefinition, UnitCtx, UnitScript};
use spring::{DefId, TeamId, UnitId};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicI32, Ordering};

const CAPS: ScriptCapabilities =
    ScriptCapabilities::new(ScriptCapabilities::CREATE | ScriptCapabilities::QUERY_WEAPON);
static PENDING_UNIT: AtomicI32 = AtomicI32::new(-1);
static ATTACHED_UNIT: AtomicI32 = AtomicI32::new(-1);

fn record(message: &str) {
    let _ = spring::messages::log("cus-e2e", 1, message);
}

struct E2EScript {
    resumed: Rc<RefCell<bool>>,
}

impl UnitScript for E2EScript {
    fn new(_ctx: &mut spring::cus::InitCtx<'_>) -> Self {
        Self {
            resumed: Rc::new(RefCell::new(false)),
        }
    }

    fn create(&mut self, ctx: &UnitCtx) {
        record("CUS_E2E|core|create");
        ctx.spawn(TaskDefinition::with_state(
            "rust-cus-e2e-next-frame",
            Rc::clone(&self.resumed),
            resume_next_frame,
        ));
    }

    // Pieces resolve only once the Create task has run, like a script that
    // looks them up asynchronously; the attach-time refresh must not warn.
    fn query_weapon(&mut self, _ctx: &UnitCtx, _weapon: spring::cus::WeaponId) -> Piece {
        if *self.resumed.borrow() {
            Piece(0)
        } else {
            Piece(-1)
        }
    }

    fn call_named(
        &mut self,
        _ctx: &UnitCtx,
        name: &str,
        arguments: &[f32],
        returns: &mut [f32],
    ) -> Option<usize> {
        if name != "e2e_named" {
            return None;
        }
        let (Some(argument), Some(output)) = (arguments.first(), returns.first_mut()) else {
            return Some(0);
        };
        *output = *argument + 1.0;
        Some(1)
    }
}

async fn resume_next_frame(state: Rc<RefCell<bool>>, ctx: UnitCtx) {
    ctx.next_frame().await;
    *state.borrow_mut() = true;
    ctx.move_now(Piece(0), spring::cus::Axis::X, 1.0);
}

/// A game module that keeps its own state next to the scripts and hands the
/// CUS exports to `CusHost`.
#[derive(Default)]
struct CusE2ECore {
    host: CusHost<E2EScript>,
    task_logged: bool,
}

impl CusE2ECore {
    // Attach, then create a unit while still holding the CUS state: the nested
    // UnitCreated has to defer its work, and the engine must still deliver
    // this script's Create exactly once, after GameFrame returns.
    fn attach_pending(&mut self) {
        let unit_id = PENDING_UNIT.swap(-1, Ordering::Relaxed);
        if unit_id < 0 {
            return;
        }
        let unit = UnitId(unit_id);
        match self.host.attach(unit, CAPS, E2EScript::new) {
            Ok(()) => {
                ATTACHED_UNIT.store(unit_id, Ordering::Relaxed);
                record(&format!("CUS_E2E|core|attached|unit={unit_id}"));
            }
            Err(_) => record("CUS_E2E|core|attach-error"),
        }
        let second = spring::create_unit(
            spring::UnitDefRef {
                name: "native_api_test_unit",
                id: -1,
            },
            spring::Float3 {
                x: 160.0,
                y: 0.0,
                z: 160.0,
            },
            0,
            spring::CreateUnitOptions {
                unit_id: -1,
                builder_id: -1,
                ..Default::default()
            },
        );
        if second.is_err() {
            record("CUS_E2E|core|second-unit-error");
        }
    }

    fn resumed(&mut self, unit: UnitId) -> bool {
        self.host
            .with_script(unit, |_, script| *script.resumed.borrow())
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
        self.host
            .cus_invoke(instance_id, call, float_arguments, integer_arguments, result)
    }

    fn cus_call_named(
        &mut self,
        instance_id: u32,
        function_name: &str,
        arguments: &[f32],
        return_values: &mut [f32],
        found: &mut bool,
    ) -> Option<usize> {
        let count =
            self.host
                .cus_call_named(instance_id, function_name, arguments, return_values, found);
        if *found {
            record("CUS_E2E|core|named|found=1|success=1|value=4");
        }
        count
    }

    fn cus_tick(&mut self, frame: u32) {
        if frame == 3 {
            record(&format!("RNG|core|{}", synced_random_draws()));
            spring::log_info!(
                "cus-e2e",
                "CUS_E2E|core|sdk|frame={}|fight={}",
                spring::game::frame().unwrap_or(0),
                spring::cmd::FIGHT
            );
        }
        self.host.cus_tick(frame);
        let unit = ATTACHED_UNIT.load(Ordering::Relaxed);
        if unit >= 0 && !self.task_logged && self.resumed(UnitId(unit)) {
            self.task_logged = true;
            record(&format!("CUS_E2E|core|tick|frame={frame}|task_resumed=1"));
        }
    }

    fn cus_detach(&mut self, instance_id: u32) {
        record("CUS_E2E|core|detach");
        self.host.cus_detach(instance_id);
    }
}

/// Mirrors the fixture gadget's `math.random` calls after the same reseed.
fn synced_random_draws() -> String {
    const SCALE: f32 = 16_777_216.0;
    spring::random::synced::seed(12345);
    let draws = [
        (spring::random::synced::float() * SCALE) as i64,
        spring::random::synced::up_to(10) as i64,
        spring::random::synced::int(3, 7) as i64,
        spring::random::synced::int(-5, 5) as i64,
        (spring::random::synced::float() * SCALE) as i64,
    ];
    draws.map(|draw| draw.to_string()).join("|")
}

/// Rules code calling its own module's unit script: through the engine and
/// directly through `with_cus_module`.
fn game_frame(frame: i32) {
    if frame == 1 && with_cus_module(|module| module.attach_pending()).is_none() {
        record("CUS_E2E|core|attach-busy");
    }
    if frame != 4 {
        return;
    }
    let points = [[128.0, 128.0], [160.0, 96.0], [300.0, 40.0]];
    let batched = spring::ground_heights(&points);
    let single: Vec<_> = points
        .iter()
        .map(|[x, z]| spring::get_ground_height(*x, *z))
        .collect();
    let matches = matches!(&batched, Ok(heights)
        if heights.len() == points.len()
            && heights.iter().zip(&single).all(|(a, b)| b.as_ref().is_ok_and(|b| a == b)));
    record(&format!("CUS_E2E|core|ground-heights|matches={}", matches as u8));
    check_sdk_helpers();

    let unit = ATTACHED_UNIT.load(Ordering::Relaxed);
    if unit < 0 {
        return;
    }
    match spring::typed::call_unit_script(unit, "e2e_named", &[3.0], 1) {
        Ok(result) => record(&format!(
            "CUS_E2E|core|self-engine|found={}|success={}|value={}",
            result.function_found as u8,
            result.success as u8,
            result.ret_values.first().copied().unwrap_or(-1.0)
        )),
        Err(error) => record(&format!("CUS_E2E|core|self-engine|error={error:?}")),
    }
    let mut values = [0.0f32; 1];
    let direct = with_cus_module(|module| {
        module
            .host
            .call(UnitId(unit), "e2e_named", &[3.0], &mut values)
    });
    record(&format!(
        "CUS_E2E|core|self-direct|available={}|found={}|value={}",
        direct.is_some() as u8,
        direct.flatten().is_some() as u8,
        values[0]
    ));
}

spring::unit_kinds! {
    enum Kind {
        TestUnit = "native_api_test_unit",
        Missing = "rust_cus_e2e_missing_def",
    }
}

fn check_sdk_helpers() {
    use spring::ResultExt;

    let def = Kind::TestUnit.def();
    let kinds = def.is_some()
        && Kind::Missing.def().is_none()
        && def.and_then(Kind::from_def) == Some(Kind::TestUnit);
    let param = def.and_then(|def| {
        spring::kinds::unit_def_custom_param::<String>(def, "native_api_parity_unit")
    });
    let number = def.and_then(|def| {
        spring::kinds::unit_def_custom_param::<f32>(def, "native_api_parity_unit")
    });
    // Two failures at one call site log once.
    for _ in 0..2 {
        let _ = spring::get_unit_def_id(-5).log_err_once("CUS_E2E|core|log-once");
    }
    let camera = spring::CameraState::new(
        "spring",
        spring::Float3::new(0.0, 100.0, 0.0),
        spring::Float3::new(0.0, -0.90149933, -0.4335693),
    );
    // MyCube's hand-written camera; its direction is only nearly unit length.
    let camera_ok = (camera.up.y - 0.4335693).abs() < 1e-3
        && (camera.up.z + 0.90149933).abs() < 1e-3
        && (camera.right.x - 1.0).abs() < 1e-5;
    record(&format!(
        "CUS_E2E|core|sdk-helpers|kinds={}|param={}|number={}|camera={}",
        kinds as u8,
        (param.as_deref() == Some("unit_custom_value")) as u8,
        number.is_none() as u8,
        camera_ok as u8
    ));
}

fn unit_created(unit: UnitId, _def: DefId, _team: TeamId, _builder: UnitId) {
    if PENDING_UNIT
        .compare_exchange(-1, unit.0, Ordering::Relaxed, Ordering::Relaxed)
        .is_ok()
        && ATTACHED_UNIT.load(Ordering::Relaxed) < 0
    {
        return;
    }
    // The second unit, created while GameFrame holds the CUS state.
    PENDING_UNIT.store(-1, Ordering::Relaxed);
    let ran_now = with_cus_module_or_defer(move |module| {
        record(&format!(
            "CUS_E2E|core|deferred-ran|unit={}|attached={}",
            unit.0,
            module.host.is_attached(UnitId(ATTACHED_UNIT.load(Ordering::Relaxed))) as u8
        ));
    });
    record(&format!("CUS_E2E|core|unit-created|deferred={}", (!ran_now) as u8));
}

fn handle_lua_msg(_player: i32, _script: i32, _mode: i32, data: &[u8]) {
    if data == b"cus-e2e-fault" {
        record("CUS_E2E|core|fault");
        core::arch::wasm32::unreachable();
    }
}

spring::export_core_cus!(CusE2ECore);
spring::export_callin_scratch!(4096);
spring::export_handle_lua_msg!(handle_lua_msg);
spring::export_environment_mask!(spring::rules_synced::ENVIRONMENT_MASK);
spring::export_unit_created!(unit_created);
spring::export_game_frame!(game_frame);
