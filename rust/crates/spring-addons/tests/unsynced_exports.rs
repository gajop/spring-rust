/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use spring_addons::reexports::{All, Spring};
use spring_addons::{AddonContext, UnsyncedAddon, UnsyncedHandler};
use std::cell::Cell;

#[derive(Default)]
pub struct State(Cell<f32>);
struct Probe;
impl UnsyncedAddon<State, All> for Probe {
    fn name(&self) -> &'static str {
        "probe"
    }
    fn update(&self, ctx: &AddonContext<'_, State, All>, delta: f32) {
        ctx.global().0.set(delta);
    }
}
fn setup(handler: &mut UnsyncedHandler<State, All>, _: &Spring<'_, All>) {
    handler.add(Box::new(Probe));
}
spring_addons::export_rules_unsynced_addons! {
    state: State, setup: setup, scratch: 4096, environment: All,
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_update_reaches_unsynced_addons() {
    use spring_addons::reexports::{__sys as sys, NATIVE_API_VERSION};
    let interface = sys::NativeInterface::default();
    let [major, minor, patch] = NATIVE_API_VERSION;
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
    let mut result = sys::UpdateResult::default();
    unsafe {
        Update(
            &interface,
            initialized.moduleData,
            &sys::UpdateQuery { deltaSeconds: 0.25 },
            &mut result,
        );
    }
    assert!(result.error.is_null());
    with_global(|global| assert_eq!(global.0.get(), 0.25));
    let mut result = sys::ShutdownResult::default();
    unsafe {
        Shutdown(
            &interface,
            initialized.moduleData,
            &sys::ShutdownQuery::default(),
            &mut result,
        );
    }
    assert!(result.error.is_null());
}
