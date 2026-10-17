/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use spring::cus::host::{CusCallResult, CusModule};
use spring_addons::GadgetHandler;
use spring_addons::reexports::{self as spring, All, Result, Spring};
use std::cell::Cell;

#[derive(Default)]
pub struct Global;
#[derive(Default)]
pub struct Scripts {
    tick: Cell<u32>,
    detached: Cell<u32>,
}
impl CusModule<All> for Scripts {
    fn cus_invoke(
        &self,
        _: &Spring<'_, All>,
        _: u32,
        _: u32,
        _: &[f32],
        _: &[i32],
        _: &mut CusCallResult<'_>,
    ) -> Result<bool> {
        Ok(false)
    }
    fn cus_call_named(
        &self,
        _: &Spring<'_, All>,
        _: u32,
        _: &str,
        _: &[f32],
        _: &mut [f32],
        _: &mut bool,
    ) -> Result<Option<usize>> {
        Ok(None)
    }
    fn cus_tick(&self, spring: &Spring<'_, All>, frame: u32) -> Result<()> {
        self.tick.set(frame);
        // A custom host needs ordinary engine access during tick, as it does
        // during invoke. Queue work must execute in the same addon context.
        assert_eq!(spring.game().get_game_frame()?.low16, frame);
        with_addon_context(|ctx| {
            ctx.commands().run(|spring| {
                assert_eq!(spring.game().get_game_frame().unwrap().low16, 42);
            })
        });
        Ok(())
    }
    fn cus_detach(&self, spring: &Spring<'_, All>, instance: u32) -> Result<()> {
        assert_eq!(spring.game().get_game_frame()?.low16, self.tick.get());
        self.detached.set(instance);
        Ok(())
    }
}
fn setup(_: &mut GadgetHandler<Global, All>, _: &Spring<'_, All>) {}
spring_addons::export_rules_gadgets! {
    state: Global, setup: setup, scratch: 4096, environment: All, cus: Scripts,
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn custom_cus_tick_and_detach_receive_the_active_spring() {
    use spring::__sys as sys;
    unsafe extern "C" fn frame(
        _: *const sys::GetGameFrameQuery,
        out: *mut sys::GetGameFrameResult,
    ) {
        unsafe {
            (*out).low16 = 42;
        }
    }
    let game = sys::GameApi {
        GetGameFrame: Some(frame),
        ..Default::default()
    };
    let interface = sys::NativeInterface {
        game: &game,
        ..Default::default()
    };
    let [major, minor, patch] = spring::NATIVE_API_VERSION;
    let query = sys::InitializeNativeModuleQuery {
        hostVersionMajor: major,
        hostVersionMinor: minor,
        hostVersionPatch: patch,
    };
    let mut initialized = sys::InitializeNativeModuleResult::default();
    unsafe {
        InitializeNativeModule(&interface, &query, &mut initialized);
    }
    assert!(initialized.error.is_null());
    let mut tick = sys::CusTickResult::default();
    unsafe {
        RustCUSTick(
            &interface,
            initialized.moduleData,
            &sys::CusTickQuery { frame: 42 },
            &mut tick,
        );
    }
    assert!(tick.error.is_null());
    let mut detach = sys::CusDetachResult::default();
    unsafe {
        RustCUSDetach(
            &interface,
            initialized.moduleData,
            &sys::CusDetachQuery { instanceID: 7 },
            &mut detach,
        );
    }
    assert!(detach.error.is_null());
    with_cus_module(|host, _| {
        assert_eq!((host.tick.get(), host.detached.get()), (42, 7));
        Ok(())
    })
    .unwrap();
    let mut shutdown = sys::ShutdownResult::default();
    unsafe {
        Shutdown(
            &interface,
            initialized.moduleData,
            &sys::ShutdownQuery::default(),
            &mut shutdown,
        );
    }
    assert!(shutdown.error.is_null());
}
