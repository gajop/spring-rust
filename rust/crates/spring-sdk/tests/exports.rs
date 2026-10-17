/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

// Compile every generated callin macro on both targets. Tests below exercise
// the actual C exports and module boundary, not just the Rust slot methods.
use spring_sdk::{Menu, Result, Spring, module::Module};
use std::cell::Cell;
struct Fixture {
    calls: Cell<u32>,
}
impl Module<Menu> for Fixture {
    fn initialize(_: &mut Spring<'_, Menu>) -> Result<Self> {
        Ok(Self {
            calls: Cell::new(0),
        })
    }
}
fn handler<Q, R: Default>(module: &Fixture, _: &mut Spring<'_, Menu>, _: &Q) -> Result<R> {
    module.calls.set(module.calls.get() + 1);
    Ok(R::default())
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
            Ok(())
        })
        .unwrap()
    });
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
