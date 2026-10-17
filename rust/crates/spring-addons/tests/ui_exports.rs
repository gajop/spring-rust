/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use spring_addons::reexports::{All, Spring};
use spring_addons::{AddonContext, EventResult, KeyEvent, Widget, WidgetHandler};
use std::cell::Cell;

#[derive(Default)]
pub struct State(Cell<u32>);
struct Probe;
impl Widget<State, All> for Probe {
    fn name(&self) -> &'static str {
        "probe"
    }
    fn key_press(&self, ctx: &AddonContext<'_, State, All>, event: &KeyEvent<'_>) -> EventResult {
        assert_eq!(event.key_code, 13);
        assert_eq!(event.label, b"Return");
        ctx.global().0.set(ctx.global().0.get() + 1);
        EventResult::Handled
    }
}
fn setup(handler: &mut WidgetHandler<State, All>, _: &Spring<'_, All>) {
    handler.add(Box::new(Probe));
}
spring_addons::export_ui_widgets! {
    state: State, setup: setup, scratch: 4096, environment: All,
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_key_result_and_label_reach_the_widget() {
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
    let query = sys::KeyPressQuery {
        keyCode: 13,
        label: c"Return".as_ptr(),
        ..Default::default()
    };
    let mut result = sys::BoolCallinResult::default();
    unsafe {
        KeyPress(&interface, initialized.moduleData, &query, &mut result);
    }
    assert!(result.error.is_null());
    assert!(result.value);
    with_global(|global| assert_eq!(global.0.get(), 1));
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
