/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

// Compile every generated callin macro on both targets. Tests below exercise
// the actual C exports and module boundary, not just the Rust slot methods.
use spring_sdk::{Environment, Menu, Result, Spring, module::Module};
use std::cell::{Cell, RefCell};
struct Fixture {
    calls: Cell<u32>,
    delta: Cell<f32>,
    message: RefCell<String>,
}
impl<E: Environment> Module<E> for Fixture {
    fn initialize(_: &mut Spring<'_, E>) -> Result<Self> {
        Ok(Self {
            calls: Cell::new(0),
            delta: Cell::new(0.0),
            message: RefCell::new(String::new()),
        })
    }
}
fn record_call(module: &Fixture) {
    module.calls.set(module.calls.get() + 1);
}
fn record_update(module: &Fixture, delta: f32) {
    module.delta.set(delta);
}
fn record_message(module: &Fixture, message: &str) {
    *module.message.borrow_mut() = message.to_owned();
}
spring_sdk::export_module!(MODULE, Menu, Fixture);
#[cfg(target_arch = "wasm32")]
spring_sdk::export_callin_scratch!(65536);

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../rts/wasm/generated/sdk/scoped_callin_exports_test.rs"
));

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_exports_validate_identity_and_dispatch() {
    use spring_sdk::__sys as sys;
    let interface = sys::NativeInterface::default();
    let [major, minor, patch] = spring_sdk::NATIVE_API_VERSION;
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
    assert!(!initialized.moduleData.is_null());
    let update = sys::UpdateQuery { deltaSeconds: 0.02 };
    let mut result = sys::UpdateResult::default();
    unsafe {
        Update(&interface, initialized.moduleData, &update, &mut result);
    }
    assert!(result.error.is_null());
    MODULE.with(|slot| {
        slot.invoke(|module, _| {
            assert_eq!(module.calls.get(), 1);
            assert_eq!(module.delta.get(), 0.02);
            Ok(())
        })
        .unwrap()
    });
    let text = c"message 日本語";
    let query = sys::ActivateMenuQuery {
        message: text.as_ptr(),
        messageLength: text.to_bytes().len() as u32,
    };
    let mut activated = sys::ActivateMenuResult::default();
    unsafe {
        ActivateMenu(&interface, initialized.moduleData, &query, &mut activated);
    }
    assert!(activated.error.is_null());
    MODULE.with(|slot| {
        slot.invoke(|module, _| {
            assert_eq!(&*module.message.borrow(), "message 日本語");
            Ok(())
        })
        .unwrap()
    });
    let mut allowed = sys::BoolCallinResult::default();
    unsafe {
        AllowDraw(
            &interface,
            initialized.moduleData,
            &sys::SimpleCallinQuery::default(),
            &mut allowed,
        );
    }
    assert!(allowed.error.is_null());
    assert!(allowed.value);
    let mut tooltip = sys::StringCallinResult::default();
    unsafe {
        GetTooltip(
            &interface,
            initialized.moduleData,
            &sys::ScreenPositionQuery::default(),
            &mut tooltip,
        );
    }
    assert!(tooltip.error.is_null());
    assert_eq!(
        unsafe { std::ffi::CStr::from_ptr(tooltip.value) }
            .to_str()
            .unwrap(),
        "tooltip 日本語"
    );
    unsafe {
        Update(&interface, core::ptr::null_mut(), &update, &mut result);
    }
    assert!(!result.error.is_null());
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
