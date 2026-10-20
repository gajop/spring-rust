/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */
#[cfg(not(target_arch = "wasm32"))]
use spring_addons::reexports as spring;
use spring_addons::reexports::{
    All, Spring,
    jobs::{JobId, JobResult},
};
use spring_addons::{AddonContext, Widget, WidgetHandler};
use std::{
    cell::Cell,
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
};

#[derive(Default)]
pub struct State {
    value: Arc<AtomicU32>,
    updates: Cell<u32>,
}
struct Probe;
impl Widget<State, All> for Probe {
    fn name(&self) -> &'static str {
        "job probe"
    }
    fn init(&self, ctx: &AddonContext<'_, State, All>) {
        ctx.spring().jobs().run_next_tick(DOUBLE, &[3]).unwrap();
    }
    fn update(&self, ctx: &AddonContext<'_, State, All>, _: f32) {
        let previous = ctx.global().updates.get();
        assert_eq!(
            ctx.global().value.load(Ordering::SeqCst),
            if previous == 0 { 0 } else { 6 }
        );
        ctx.global().updates.set(previous + 1);
    }
    fn job_result(&self, ctx: &AddonContext<'_, State, All>, id: JobId, result: JobResult<'_>) {
        assert_eq!(id, JobId(1));
        assert_eq!(result, Ok(&[6][..]));
        assert_eq!(ctx.global().value.load(Ordering::SeqCst), 0);
        with_addon_context(|nested| assert_eq!(nested.global().updates.get(), 1));
        let value = ctx.global().value.clone();
        ctx.commands().run(move |spring| {
            spring
                .module_ref()
                .with_spring(|_| {
                    value.store(6, Ordering::SeqCst);
                    Ok(())
                })
                .unwrap();
        });
        assert_eq!(
            ctx.global().value.load(Ordering::SeqCst),
            0,
            "queued command ran before result callback returned"
        );
    }
}
fn double(input: &[u8]) -> Vec<u8> {
    input.iter().map(|value| value * 2).collect()
}
fn setup(handler: &mut WidgetHandler<State, All>, _: &Spring<'_, All>) {
    handler.add(Box::new(Probe));
}
spring_addons::export_ui_widgets! { state: State, setup: setup, scratch: 4096, environment: All, }
spring_addons::export_jobs!({ DOUBLE => ("double", double) });

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_job_results_use_the_addon_context_and_drain_commands_before_update() {
    use spring::{__sys as sys, NATIVE_API_VERSION};
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
    for _ in 0..2 {
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
    }
    with_global(|global| {
        assert_eq!(global.value.load(Ordering::SeqCst), 6);
        assert_eq!(global.updates.get(), 2);
    });
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
