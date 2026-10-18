/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use super::*;
use spring_native_sys as sys;
use std::{
    cell::{Cell, RefCell},
    ffi::CStr,
    ptr,
};

#[test]
fn native_callin_errors_include_the_code_in_the_message() {
    assert!(module::error_pointer(Ok(())).is_null());
    for code in [4, 5, 999, -7] {
        let pointer = module::error_pointer(Err(ApiError::new(code)));
        let error = unsafe { &*pointer };
        assert_eq!(error.code, code);
        assert_eq!(
            unsafe { CStr::from_ptr(error.message) }.to_str().unwrap(),
            format!("spring-sdk module callin failed (error code {code})")
        );
    }
}

#[test]
fn shader_params_preserve_stage_options_and_initialize_uniforms_on_native() {
    thread_local! { static BOUND: RefCell<Vec<u32>> = const { RefCell::new(Vec::new()) }; }
    unsafe extern "C" fn create(
        query: *const sys::GfxCreateShaderQuery,
        result: *mut sys::GfxCreateShaderResult,
    ) {
        let query = unsafe { &*query };
        assert_eq!(
            unsafe { CStr::from_ptr(query.vertex) }.to_str().unwrap(),
            "vertex"
        );
        assert!(query.options.hasGeoOutputVerts);
        assert_eq!(query.options.geoOutputVerts, 6);
        let failed = unsafe { CStr::from_ptr(query.fragment) }.to_bytes() == b"fail";
        unsafe {
            (*result).shaderID = if failed { 0 } else { 17 };
        }
    }
    unsafe extern "C" fn bind(query: *const sys::GfxShaderQuery, _: *mut sys::GfxUseShaderResult) {
        BOUND.with(|bound| bound.borrow_mut().push(unsafe { (*query).shaderID }));
    }
    unsafe extern "C" fn location(
        query: *const sys::GfxUniformLocationQuery,
        result: *mut sys::GfxUniformLocationResult,
    ) {
        let query = unsafe { &*query };
        assert_eq!(query.shaderID, 17);
        let name = unsafe { CStr::from_ptr(query.name) }.to_bytes();
        unsafe {
            (*result).location = if name == b"sampler" { 2 } else { 3 };
        }
    }
    unsafe extern "C" fn integer(
        query: *const sys::GfxUniformIntQuery,
        _: *mut sys::GfxEmptyResult,
    ) {
        let query = unsafe { &*query };
        assert_eq!(
            (query.location, query.count, query.values),
            (2, 1, [4, 0, 0, 0])
        );
    }
    unsafe extern "C" fn float(
        query: *const sys::GfxUniformFloatQuery,
        _: *mut sys::GfxEmptyResult,
    ) {
        let query = unsafe { &*query };
        assert_eq!(
            (query.location, query.count, query.values),
            (3, 1, [0.5, 0.0, 0.0, 0.0])
        );
    }
    let gfx = sys::GfxApi {
        CreateShader: Some(create),
        UseShader: Some(bind),
        GetUniformLocation: Some(location),
        UniformInt: Some(integer),
        Uniform: Some(float),
        ..Default::default()
    };
    let interface = sys::NativeInterface {
        gfx: &gfx,
        ..Default::default()
    };
    let context = unsafe { ModuleContext::<Ui>::from_native(&interface) }.unwrap();
    context
        .with_spring(|spring| {
            let params = ShaderCreateParams {
                vertex: "vertex",
                fragment: "fragment",
                geo_output_verts: Some(6),
                uniform_ints: &[ShaderUniformInt {
                    name: "sampler",
                    value: 4,
                }],
                uniform_floats: &[ShaderUniformFloat {
                    name: "scale",
                    value: 0.5,
                }],
                ..Default::default()
            };
            assert_eq!(spring.gfx().compile_shader(params).unwrap().shader_id, 17);
            assert_eq!(
                spring
                    .gfx()
                    .compile_shader(ShaderCreateParams {
                        fragment: "fail",
                        ..params
                    })
                    .unwrap_err()
                    .code,
                ErrorCode::OperationFailed as i32
            );
        })
        .unwrap();
    BOUND.with(|bound| assert_eq!(&*bound.borrow(), &[17, 0]));
}

#[test]
fn graphics_counted_arrays_accept_short_inputs_and_zero_fill_native_capacity() {
    unsafe extern "C" fn uniform(
        query: *const sys::GfxUniformFloatQuery,
        _: *mut sys::GfxEmptyResult,
    ) {
        let query = unsafe { &*query };
        assert_eq!(query.count, 2);
        assert_eq!(query.values, [1280.0, 720.0, 0.0, 0.0]);
    }
    unsafe extern "C" fn uniform_int(
        query: *const sys::GfxUniformIntQuery,
        _: *mut sys::GfxEmptyResult,
    ) {
        let query = unsafe { &*query };
        assert_eq!(query.count, 1);
        assert_eq!(query.values, [7, 0, 0, 0]);
    }
    unsafe extern "C" fn clear(query: *const sys::GfxClearQuery, _: *mut sys::GfxEmptyResult) {
        let query = unsafe { &*query };
        assert_eq!(query.count, 1);
        assert_eq!(query.values, [1.0, 0.0, 0.0, 0.0]);
    }
    let gfx = sys::GfxApi {
        Uniform: Some(uniform),
        UniformInt: Some(uniform_int),
        Clear: Some(clear),
        ..Default::default()
    };
    let interface = sys::NativeInterface {
        gfx: &gfx,
        ..Default::default()
    };
    let context = unsafe { ModuleContext::<Ui>::from_native(&interface) }.unwrap();
    context
        .with_spring(|spring| {
            spring.gfx().uniform(3, &[1280.0, 720.0], 2).unwrap();
            spring.gfx().uniform_int(4, &[7], 1).unwrap();
            spring
                .gfx()
                .clear(spring_sdk_internal::gl::DEPTH_BUFFER_BIT, &[1.0], 1)
                .unwrap();
            assert!(spring.gfx().uniform(3, &[1.0; 5], 5).is_err());
            assert!(spring.gfx().uniform_int(4, &[1; 5], 5).is_err());
            assert!(spring.gfx().clear(0, &[1.0; 5], 5).is_err());
        })
        .unwrap();
}

#[test]
fn local_random_preserves_pcg_sequence_and_seeds_each_module_only_once() {
    thread_local! { static SEEDS: Cell<u32> = const { Cell::new(0) }; }
    unsafe extern "C" fn seed(query: *const sys::NextIntQuery, result: *mut sys::NextIntResult) {
        let query = unsafe { &*query };
        assert_eq!((query.lower, query.upper), (i32::MIN, i32::MAX));
        SEEDS.with(|count| count.set(count.get() + 1));
        unsafe {
            (*result).value = 42;
        }
    }
    let random = sys::UnsyncedRandomApi {
        NextInt: Some(seed),
        ..Default::default()
    };
    let ctrl = sys::UnsyncedCtrlApi {
        random: &random,
        ..Default::default()
    };
    let interface = sys::NativeInterface {
        unsyncedCtrl: &ctrl,
        ..Default::default()
    };
    let first = unsafe { ModuleContext::<Ui>::from_native(&interface) }.unwrap();
    let second = unsafe { ModuleContext::<Ui>::from_native(&interface) }.unwrap();
    let mut expected = pcg::Pcg32::seeded(42);
    first
        .with_spring(|spring| {
            assert_eq!(
                spring.unsynced_random().int(-5, 9).unwrap(),
                expected.int(-5, 9)
            );
            assert_eq!(spring.unsynced_random().float().unwrap(), expected.float());
            assert_eq!(
                spring.unsynced_random().up_to(20).unwrap(),
                expected.int(1, 20)
            );
        })
        .unwrap();
    second
        .with_spring(|spring| {
            assert_eq!(
                spring.unsynced_random().float().unwrap(),
                pcg::Pcg32::seeded(42).float()
            );
            spring.unsynced_random().seed(7).unwrap();
            assert_eq!(
                spring.unsynced_random().float().unwrap(),
                pcg::Pcg32::seeded(7).float()
            );
        })
        .unwrap();
    first
        .with_spring(|spring| {
            assert_eq!(spring.unsynced_random().float().unwrap(), expected.float());
        })
        .unwrap();
    SEEDS.with(|count| assert_eq!(count.get(), 2));
}

#[test]
fn default_unit_creation_does_not_request_an_existing_unit_id() {
    unsafe extern "C" fn create(
        query: *const sys::CreateUnitQuery,
        result: *mut sys::CreateUnitResult,
    ) {
        let query = unsafe { &*query };
        assert_eq!(query.options.unitID, -1);
        assert_eq!(query.options.builderID, -1);
        unsafe {
            (*result).unitID = 25;
        }
    }
    let unit_api = sys::UnitControlApi {
        CreateUnit: Some(create),
        ..Default::default()
    };
    let api = sys::SyncedCtrlApi {
        unit: &unit_api,
        ..Default::default()
    };
    let interface = sys::NativeInterface {
        syncedCtrl: &api,
        ..Default::default()
    };
    let context = unsafe { ModuleContext::<All>::from_native(&interface) }.unwrap();
    context
        .with_spring(|spring| {
            let def = unit_control::DefRef {
                name: String::new(),
                id: 1,
            };
            assert_eq!(
                spring
                    .unit_control()
                    .create_unit(
                        &def,
                        Float3::ZERO,
                        0,
                        0,
                        unit_control::CreateUnitOptions::default()
                    )
                    .unwrap(),
                25
            );
        })
        .unwrap();
}

thread_local! {
    static SCRATCH:RefCell<Vec<i32>>=const {RefCell::new(Vec::new())};
    static GENERATION:Cell<i32>=const {Cell::new(0)};
    static CONTEXT:Cell<*const ModuleContext<Ui>>=const {Cell::new(ptr::null())};
    static REENTER:Cell<bool>=const {Cell::new(false)};
    static EVENTS:RefCell<Vec<&'static str>>=const {RefCell::new(Vec::new())};
    static STRINGS:RefCell<Vec<(String,String,bool)>>=const {RefCell::new(Vec::new())};
}

unsafe extern "C" fn all_units(
    _: *const sys::GetAllUnitsQuery,
    result: *mut sys::GetAllUnitsResult,
) {
    SCRATCH.with(|scratch| {
        let mut scratch = scratch.borrow_mut();
        let generation = GENERATION.with(|n| {
            n.set(n.get() + 10);
            n.get()
        });
        *scratch = vec![generation + 1, generation + 2];
        unsafe {
            *result = sys::GetAllUnitsResult {
                error: ptr::null(),
                units: scratch.as_mut_ptr(),
                count: 2,
            };
        }
    });
}

unsafe extern "C" fn frame(_: *const sys::GetGameFrameQuery, result: *mut sys::GetGameFrameResult) {
    EVENTS.with(|e| e.borrow_mut().push("outer host call"));
    if REENTER.with(|v| v.replace(false)) {
        CONTEXT.with(|context| {
            let context = unsafe { &*context.get() };
            context
                .with_spring(|spring| {
                    EVENTS.with(|e| e.borrow_mut().push("nested callin"));
                    assert_eq!(spring.units().get_all_units().unwrap().len(), 2);
                })
                .unwrap();
        });
    }
    EVENTS.with(|e| e.borrow_mut().push("host call returns"));
    unsafe {
        *result = sys::GetGameFrameResult {
            error: ptr::null(),
            low16: 42,
            high16: 3,
        };
    }
}

unsafe extern "C" fn set_string(
    query: *const sys::SetConfigStringQuery,
    result: *mut sys::SetConfigStringResult,
) {
    let query = unsafe { &*query };
    let key = unsafe { CStr::from_ptr(query.key) }
        .to_str()
        .unwrap()
        .to_owned();
    let value = unsafe { CStr::from_ptr(query.value) }
        .to_str()
        .unwrap()
        .to_owned();
    STRINGS.with(|s| s.borrow_mut().push((key, value, query.useOverlay)));
    unsafe {
        *result = sys::SetConfigStringResult {
            error: ptr::null(),
            success: true,
        };
    }
}

fn tables() -> (sys::GameApi, sys::UnitsQueryApi, sys::ConfigApi) {
    let game = sys::GameApi {
        GetGameFrame: Some(frame),
        ..Default::default()
    };
    let units = sys::UnitsQueryApi {
        GetAllUnits: Some(all_units),
        ..Default::default()
    };
    let config = sys::ConfigApi {
        SetConfigString: Some(set_string),
        ..Default::default()
    };
    (game, units, config)
}

#[test]
fn results_own_scratch_and_owned_snapshots_can_cross_threads() {
    let (game, units, config) = tables();
    let interface = sys::NativeInterface {
        game: &game,
        unitsQuery: &units,
        config: &config,
        ..Default::default()
    };
    let context = unsafe { ModuleContext::<Ui>::from_native(&interface) }.unwrap();
    GENERATION.with(|n| n.set(0));
    let first = context
        .with_spring(|spring| spring.units().get_all_units())
        .unwrap()
        .unwrap();
    let second = context
        .with_spring(|spring| spring.units().get_all_units())
        .unwrap()
        .unwrap();
    assert_eq!(first, [11, 12]);
    assert_eq!(second, [21, 22]);
    assert_eq!(std::thread::spawn(move || first).join().unwrap(), [11, 12]);
}

#[test]
fn host_reentry_is_immediate_and_does_not_lock_the_context() {
    let (game, units, config) = tables();
    let interface = sys::NativeInterface {
        game: &game,
        unitsQuery: &units,
        config: &config,
        ..Default::default()
    };
    let context = unsafe { ModuleContext::<Ui>::from_native(&interface) }.unwrap();
    CONTEXT.with(|c| c.set(&context));
    REENTER.with(|v| v.set(true));
    EVENTS.with(|e| e.borrow_mut().clear());
    let frame = context
        .with_spring(|spring| {
            let units = spring.units();
            let game = spring.game();
            let frame = game.get_game_frame();
            assert_eq!(units.get_all_units().unwrap().len(), 2);
            frame
        })
        .unwrap()
        .unwrap();
    CONTEXT.with(|c| c.set(ptr::null()));
    assert_eq!((frame.low16, frame.high16), (42, 3));
    EVENTS.with(|e| {
        assert_eq!(
            *e.borrow(),
            ["outer host call", "nested callin", "host call returns"]
        )
    });
}

#[test]
fn strings_are_owned_for_the_call_and_interior_nul_is_rejected() {
    let (game, units, config) = tables();
    let interface = sys::NativeInterface {
        game: &game,
        unitsQuery: &units,
        config: &config,
        ..Default::default()
    };
    let context = unsafe { ModuleContext::<Menu>::from_native(&interface) }.unwrap();
    STRINGS.with(|s| s.borrow_mut().clear());
    context
        .with_spring(|s| s.config().set_config_string("key", "日本語", true))
        .unwrap()
        .unwrap();
    let error = context
        .with_spring(|s| s.config().set_config_string("key\0tail", "value", false))
        .unwrap()
        .unwrap_err();
    assert_eq!(error.code, ErrorCode::InvalidArgument as i32);
    STRINGS.with(|s| assert_eq!(*s.borrow(), [("key".into(), "日本語".into(), true)]));
}

#[test]
fn missing_native_table_and_function_are_explicit_errors() {
    let interface = sys::NativeInterface::default();
    let context = unsafe { ModuleContext::<Ui>::from_native(&interface) }.unwrap();
    assert_eq!(
        context
            .with_spring(|s| s.game().get_game_frame())
            .unwrap()
            .unwrap_err()
            .code,
        ErrorCode::NotAvailable as i32
    );
    let game = sys::GameApi::default();
    let interface = sys::NativeInterface {
        game: &game,
        ..Default::default()
    };
    let context = unsafe { ModuleContext::<Ui>::from_native(&interface) }.unwrap();
    assert_eq!(
        context
            .with_spring(|s| s.game().get_game_frame())
            .unwrap()
            .unwrap_err()
            .code,
        ErrorCode::NotAvailable as i32
    );
}

#[test]
fn foreign_host_thread_is_rejected_before_calling_the_table() {
    let interface = sys::NativeInterface::default();
    let context = unsafe { ModuleContext::<Ui>::from_native(&interface) }.unwrap();
    let address = &context as *const ModuleContext<Ui> as usize;
    std::thread::scope(|threads| {
        // Deliberately simulate a foreign host entering from the wrong thread.
        // Safe Rust cannot send/share this context in the first place.
        threads
            .spawn(move || {
                let context = unsafe { &*(address as *const ModuleContext<Ui>) };
                let error = context
                    .with_spring(|_| panic!("off-thread scope entered"))
                    .unwrap_err();
                assert_eq!(error.code, ErrorCode::InvalidState as i32);
            })
            .join()
            .unwrap();
    });
}

// Matching pass cases ensure compile-fail doctests use actual supported APIs.
#[allow(dead_code)]
fn allowed_calls(synced: &mut Spring<'_, RulesSynced>, ui: &mut Spring<'_, Ui>) {
    let _ = synced
        .unit_control()
        .destroy_unit(1, unit_control::DestroyUnitOptions::default());
    let _ = ui.gfx().get_matrix_data(0);
}

thread_local! {
    static SYNC_MODE:Cell<u8>=const {Cell::new(0)};
    static HOST_CLEANED:Cell<bool>=const {Cell::new(false)};
    static ACTIVE_SYNC:Cell<(sys::NativeCallback,usize)>=const {Cell::new((None,0))};
    static REGISTRATION:RefCell<Option<Registration>>=const {RefCell::new(None)};
    static REJECT_REGISTRATION:Cell<bool>=const {Cell::new(false)};
}

unsafe extern "C" fn call_as_team(
    query: *const sys::CallAsTeamQuery,
    result: *mut sys::CallAsTeamResult,
) {
    let query = unsafe { &*query };
    let callback = query.callback.unwrap();
    let data = query.userData as usize;
    HOST_CLEANED.with(|v| v.set(false));
    if SYNC_MODE.with(|v| v.get()) == 1 {
        std::thread::scope(|threads| {
            threads
                .spawn(move || unsafe { callback(data as *mut _) })
                .join()
                .unwrap();
        });
    } else {
        ACTIVE_SYNC.with(|v| v.set((Some(callback), data)));
        unsafe { callback(data as *mut _) };
        ACTIVE_SYNC.with(|v| v.set((None, 0)));
    }
    HOST_CLEANED.with(|v| v.set(true));
    unsafe {
        *result = sys::CallAsTeamResult {
            error: ptr::null(),
            success: true,
        };
    }
}

#[test]
fn synchronous_callback_panic_waits_for_host_cleanup() {
    let control = sys::SystemControlApi {
        CallAsTeam: Some(call_as_team),
        ..Default::default()
    };
    let interface = sys::NativeInterface {
        systemControl: &control,
        ..Default::default()
    };
    let context = unsafe { ModuleContext::<Ui>::from_native(&interface) }.unwrap();
    SYNC_MODE.with(|v| v.set(0));
    let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = context.with_spring(|spring| {
            spring
                .system_control()
                .call_as_team(0, |_| panic!("callback panic"))
        });
    }));
    assert!(panic.is_err());
    HOST_CLEANED.with(|v| assert!(v.get()));
}

#[test]
fn synchronous_callback_reentry_and_foreign_thread_fail_without_aliasing() {
    let control = sys::SystemControlApi {
        CallAsTeam: Some(call_as_team),
        ..Default::default()
    };
    let interface = sys::NativeInterface {
        systemControl: &control,
        ..Default::default()
    };
    let context = unsafe { ModuleContext::<Ui>::from_native(&interface) }.unwrap();
    SYNC_MODE.with(|v| v.set(0));
    let error = context
        .with_spring(|spring| {
            spring.system_control().call_as_team(0, |_| {
                ACTIVE_SYNC.with(|v| {
                    let (callback, data) = v.get();
                    unsafe { callback.unwrap()(data as *mut _) };
                });
            })
        })
        .unwrap()
        .unwrap_err();
    assert_eq!(error.code, ErrorCode::InvalidState as i32);
    SYNC_MODE.with(|v| v.set(1));
    let called = std::sync::Arc::new(TestCell::new(false));
    let observed = std::sync::Arc::clone(&called);
    let error = context
        .with_spring(|spring| {
            spring
                .system_control()
                .call_as_team(0, move |_| observed.set(true))
        })
        .unwrap()
        .unwrap_err();
    assert_eq!(error.code, ErrorCode::InvalidState as i32);
    assert!(!called.get());
    SYNC_MODE.with(|v| v.set(0));
}

#[derive(Clone)]
struct Registration {
    callback: sys::RmlDataEventCallback,
    destroy: sys::NativeCallback,
    data: usize,
    schema: Vec<u8>,
}
unsafe extern "C" fn bind_event(
    query: *const sys::RmlDataModelBindEventQuery,
    result: *mut sys::RmlDataModelBindEventResult,
) {
    let query = unsafe { &*query };
    let schema = unsafe { crate::native::read_slice(query.fieldTypes, query.fieldCount as usize) }
        .unwrap()
        .to_vec();
    let registration = Registration {
        callback: query.callback,
        destroy: query.destroyCallback,
        data: query.userData as usize,
        schema,
    };
    if REJECT_REGISTRATION.with(|v| v.get()) {
        unsafe { registration.destroy.unwrap()(registration.data as *mut _) };
        unsafe {
            *result = sys::RmlDataModelBindEventResult {
                error: ptr::null(),
                eventHandle: 0,
                success: false,
            };
        }
    } else {
        REGISTRATION.with(|r| *r.borrow_mut() = Some(registration));
        unsafe {
            *result = sys::RmlDataModelBindEventResult {
                error: ptr::null(),
                eventHandle: 77,
                success: true,
            };
        }
    }
}

fn fire_data_event() {
    let registration = REGISTRATION.with(|r| r.borrow().as_ref().unwrap().clone());
    assert_eq!(registration.schema, [3, 1]);
    let text = std::ffi::CString::new("owned event").unwrap();
    let values = [
        sys::RmlDataValue {
            type_: 3,
            stringValue: text.as_ptr(),
            ..Default::default()
        },
        sys::RmlDataValue {
            type_: 1,
            intValue: 7,
            ..Default::default()
        },
    ];
    let args = sys::RmlDataEventArgs {
        eventHandle: 77,
        targetElementHandle: 99,
        values: values.as_ptr(),
        count: 2,
    };
    unsafe { registration.callback.unwrap()(registration.data as *mut _, &args) };
}

fn destroy_registration() {
    let registration = REGISTRATION.with(|r| r.borrow_mut().take());
    if let Some(registration) = registration {
        unsafe { registration.destroy.unwrap()(registration.data as *mut _) };
    }
}

struct Capture(std::sync::Arc<TestCell<usize>>);
impl Drop for Capture {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

#[test]
fn retained_events_own_values_and_context_drop_cancels_the_callback() {
    let rml = sys::RmlUiApi {
        DataModelBindEvent: Some(bind_event),
        ..Default::default()
    };
    let interface = sys::NativeInterface {
        rmlUi: &rml,
        ..Default::default()
    };
    let context = unsafe { ModuleContext::<Menu>::from_native(&interface) }.unwrap();
    REJECT_REGISTRATION.with(|v| v.set(false));
    let events = std::sync::Arc::new(TestRefCell::new(Vec::new()));
    let observed = std::sync::Arc::clone(&events);
    let drops = std::sync::Arc::new(TestCell::new(0));
    let capture = Capture(std::sync::Arc::clone(&drops));
    let registration = context
        .with_spring(|spring| {
            spring
                .rml_ui()
                .data_model_bind_event(1, "event", &[3, 1], move |_, event| {
                    let _keep = &capture;
                    observed.borrow_mut().push(event);
                    Ok(())
                })
        })
        .unwrap()
        .unwrap();
    assert!(registration.success);
    fire_data_event();
    assert_eq!(
        events.borrow()[0].values,
        [
            callbacks::Value::String("owned event".into()),
            callbacks::Value::Int(7)
        ]
    );
    assert_eq!(events.borrow()[0].target_element_handle, 99);
    drop(context);
    assert_eq!(drops.get(), 1);
    fire_data_event(); // The host still owns its ID, but the SDK owner is gone.
    assert_eq!(events.borrow().len(), 1);
    destroy_registration();
    assert_eq!(drops.get(), 1);
}

#[test]
fn failed_registration_and_self_destruction_drop_captures_once() {
    let rml = sys::RmlUiApi {
        DataModelBindEvent: Some(bind_event),
        ..Default::default()
    };
    let interface = sys::NativeInterface {
        rmlUi: &rml,
        ..Default::default()
    };
    let context = unsafe { ModuleContext::<Menu>::from_native(&interface) }.unwrap();
    let drops = std::sync::Arc::new(TestCell::new(0));
    let capture = Capture(std::sync::Arc::clone(&drops));
    REJECT_REGISTRATION.with(|v| v.set(true));
    let registration = context
        .with_spring(|spring| {
            spring
                .rml_ui()
                .data_model_bind_event(1, "event", &[3, 1], move |_, _| {
                    let _keep = &capture;
                    Ok(())
                })
        })
        .unwrap()
        .unwrap();
    assert!(!registration.success);
    assert_eq!(drops.get(), 1);
    REJECT_REGISTRATION.with(|v| v.set(false));
    let capture = Capture(std::sync::Arc::clone(&drops));
    let observed = std::sync::Arc::clone(&drops);
    context
        .with_spring(|spring| {
            spring
                .rml_ui()
                .data_model_bind_event(1, "event", &[3, 1], move |_, _| {
                    let _keep = &capture;
                    destroy_registration();
                    assert_eq!(observed.get(), 1);
                    Ok(())
                })
        })
        .unwrap()
        .unwrap();
    fire_data_event();
    assert_eq!(drops.get(), 2);
}

#[test]
fn retained_callback_resource_conflict_is_reported_at_the_scope_boundary() {
    let rml = sys::RmlUiApi {
        DataModelBindEvent: Some(bind_event),
        ..Default::default()
    };
    let interface = sys::NativeInterface {
        rmlUi: &rml,
        ..Default::default()
    };
    let context = unsafe { ModuleContext::<Menu>::from_native(&interface) }.unwrap();
    REJECT_REGISTRATION.with(|v| v.set(false));
    let state = std::sync::Arc::new(TestRefCell::new(0));
    context
        .with_spring(|spring| {
            spring
                .rml_ui()
                .data_model_bind_event(1, "event", &[3, 1], move |_, _| {
                    let mut state = state
                        .try_borrow_mut()
                        .map_err(|_| ApiError::new(ErrorCode::InvalidState as i32))?;
                    fire_data_event();
                    *state += 1;
                    Ok(())
                })
        })
        .unwrap()
        .unwrap();
    fire_data_event();
    let error = context.with_spring(|_| ()).unwrap_err();
    assert_eq!(error.code, ErrorCode::InvalidState as i32);
    destroy_registration();
}

#[test]
fn same_retained_callback_reenters_immediately_with_shared_state() {
    let rml = sys::RmlUiApi {
        DataModelBindEvent: Some(bind_event),
        ..Default::default()
    };
    let interface = sys::NativeInterface {
        rmlUi: &rml,
        ..Default::default()
    };
    let context = unsafe { ModuleContext::<Menu>::from_native(&interface) }.unwrap();
    REJECT_REGISTRATION.with(|v| v.set(false));
    let nested = std::sync::Arc::new(TestCell::new(false));
    let order = std::sync::Arc::new(TestRefCell::new(Vec::new()));
    let observed = std::sync::Arc::clone(&order);
    context
        .with_spring(|spring| {
            spring
                .rml_ui()
                .data_model_bind_event(1, "event", &[3, 1], move |_, _| {
                    if nested.replace(true) {
                        observed.borrow_mut().push("nested");
                    } else {
                        observed.borrow_mut().push("outer starts");
                        fire_data_event();
                        observed.borrow_mut().push("outer finishes");
                    }
                    Ok(())
                })
        })
        .unwrap()
        .unwrap();
    fire_data_event();
    assert_eq!(
        *order.borrow(),
        ["outer starts", "nested", "outer finishes"]
    );
    context.with_spring(|_| ()).unwrap();
    destroy_registration();
}

#[test]
fn retained_callback_from_foreign_host_thread_is_rejected_and_reported() {
    let rml = sys::RmlUiApi {
        DataModelBindEvent: Some(bind_event),
        ..Default::default()
    };
    let interface = sys::NativeInterface {
        rmlUi: &rml,
        ..Default::default()
    };
    let context = unsafe { ModuleContext::<Menu>::from_native(&interface) }.unwrap();
    REJECT_REGISTRATION.with(|v| v.set(false));
    let called = std::sync::Arc::new(TestCell::new(false));
    let observed = std::sync::Arc::clone(&called);
    context
        .with_spring(|spring| {
            spring
                .rml_ui()
                .data_model_bind_event(1, "event", &[3, 1], move |_, _| {
                    observed.set(true);
                    Ok(())
                })
        })
        .unwrap()
        .unwrap();
    let registration = REGISTRATION.with(|r| r.borrow().as_ref().unwrap().clone());
    std::thread::scope(|threads| {
        threads
            .spawn(move || unsafe {
                registration.callback.unwrap()(registration.data as *mut _, ptr::null())
            })
            .join()
            .unwrap();
    });
    assert!(!called.get());
    assert_eq!(
        context.with_spring(|_| ()).unwrap_err().code,
        ErrorCode::InvalidState as i32
    );
    destroy_registration();
}

thread_local! {
    static MODEL_INT:Cell<i32>=const {Cell::new(0)};
    static ROW_WRITES:Cell<usize>=const {Cell::new(0)};
}
unsafe extern "C" fn create_model(
    _: *const sys::RmlContextCreateDataModelQuery,
    result: *mut sys::RmlContextOpenDataModelResult,
) {
    unsafe {
        *result = sys::RmlContextOpenDataModelResult {
            error: ptr::null(),
            dataModelHandle: 10,
            success: true,
        };
    }
}
unsafe extern "C" fn remove_model(
    _: *const sys::RmlContextStringQuery,
    result: *mut sys::RmlContextBoolResult,
) {
    unsafe {
        *result = Default::default();
        (*result).success = true;
    }
}
unsafe extern "C" fn bind_int(
    query: *const sys::RmlDataModelBindIntQuery,
    result: *mut sys::RmlDataModelBindResult,
) {
    MODEL_INT.with(|value| value.set(unsafe { (*query).initialValue }));
    unsafe {
        *result = sys::RmlDataModelBindResult {
            error: ptr::null(),
            variableHandle: 11,
            success: true,
        };
    }
}
unsafe extern "C" fn get_int(
    _: *const sys::RmlDataModelVariableHandleQuery,
    result: *mut sys::RmlDataModelGetIntResult,
) {
    unsafe {
        *result = sys::RmlDataModelGetIntResult {
            error: ptr::null(),
            value: MODEL_INT.with(Cell::get),
            success: true,
        };
    }
}
unsafe extern "C" fn set_int(
    query: *const sys::RmlDataModelVariableIntQuery,
    result: *mut sys::RmlElementBoolResult,
) {
    MODEL_INT.with(|value| value.set(unsafe { (*query).value }));
    unsafe {
        *result = Default::default();
        (*result).success = true;
    }
}
unsafe extern "C" fn bind_rows(
    _: *const sys::RmlDataModelBindRowsQuery,
    result: *mut sys::RmlDataModelRowsResult,
) {
    unsafe {
        *result = sys::RmlDataModelRowsResult {
            error: ptr::null(),
            rowsHandle: 12,
            success: true,
        };
    }
}
unsafe extern "C" fn set_rows(
    _: *const sys::RmlDataModelSetRowsQuery,
    result: *mut sys::RmlElementBoolResult,
) {
    ROW_WRITES.with(|count| count.set(count.get() + 1));
    unsafe {
        *result = Default::default();
        (*result).success = true;
    }
}

#[test]
fn typed_rml_bindings_validate_owner_schema_and_model_lifetime() {
    use rml::{DataModel, Value, ValueType};
    let ui = sys::RmlUiApi {
        ContextCreateDataModel: Some(create_model),
        ContextRemoveDataModel: Some(remove_model),
        DataModelBindInt: Some(bind_int),
        DataModelGetInt: Some(get_int),
        DataModelSetInt: Some(set_int),
        DataModelBindRows: Some(bind_rows),
        DataModelSetRows: Some(set_rows),
        ..Default::default()
    };
    let interface = sys::NativeInterface {
        rmlUi: &ui,
        ..Default::default()
    };
    let context = unsafe { ModuleContext::<Menu>::from_native(&interface) }.unwrap();
    let other = unsafe { ModuleContext::<Menu>::from_native(&interface) }.unwrap();
    let (model, variable, rows) = context
        .with_spring(|spring| {
            let model = DataModel::create(spring, 1, "model").unwrap();
            let variable = model.bind(spring, "count", 3i32).unwrap();
            let rows = model
                .rows(
                    spring,
                    "players",
                    &[("name", ValueType::String), ("score", ValueType::Int)],
                )
                .unwrap();
            (model, variable, rows)
        })
        .unwrap();
    context
        .with_spring(|spring| {
            assert_eq!(variable.get(spring).unwrap(), 3);
            variable.set(spring, 7).unwrap();
            assert_eq!(variable.get(spring).unwrap(), 7);
            ROW_WRITES.with(|count| count.set(0));
            assert!(
                rows.set(spring, vec![Value::String("alice".into())])
                    .is_err()
            );
            assert!(
                rows.set(
                    spring,
                    vec![Value::Int(1), Value::String("wrong order".into())]
                )
                .is_err()
            );
            ROW_WRITES.with(|count| assert_eq!(count.get(), 0));
            rows.set(spring, vec![Value::String("alice".into()), Value::Int(3)])
                .unwrap();
            ROW_WRITES.with(|count| assert_eq!(count.get(), 1));
        })
        .unwrap();
    assert!(
        other
            .with_spring(|spring| variable.get(spring))
            .unwrap()
            .is_err()
    );
    context
        .with_spring(|spring| model.close(spring))
        .unwrap()
        .unwrap();
    assert!(
        context
            .with_spring(|spring| variable.get(spring))
            .unwrap()
            .is_err()
    );
    assert!(
        context
            .with_spring(|spring| rows.set(spring, vec![]))
            .unwrap()
            .is_err()
    );
}

struct SharedModule {
    calls: Cell<u32>,
}
impl module::Module<Menu> for SharedModule {
    fn initialize(_: &mut Spring<'_, Menu>) -> Result<Self> {
        Ok(Self {
            calls: Cell::new(0),
        })
    }
}
#[test]
fn module_slot_reenters_without_a_module_wide_borrow_and_contains_panics() {
    let interface = sys::NativeInterface::default();
    let slot = module::ModuleSlot::<Menu, SharedModule>::new();
    let identity = unsafe { slot.initialize_native(&interface) }.unwrap();
    slot.validate_identity(identity).unwrap();
    slot.invoke(|module, _| {
        module.calls.set(1);
        slot.invoke(|nested, _| {
            assert!(core::ptr::eq(module, nested));
            assert_eq!(nested.calls.get(), 1);
            nested.calls.set(2);
            Ok(())
        })?;
        assert_eq!(module.calls.get(), 2);
        Ok(())
    })
    .unwrap();
    assert!(slot.invoke::<()>(|_, _| panic!("callin panic")).is_err());
    assert!(slot.invoke(|_, _| Ok(())).is_err());
    slot.shutdown().unwrap();
    assert!(slot.validate_identity(identity).is_err());
}

#[test]
fn all_environment_combines_render_and_simulation_access() {
    fn capabilities<
        E: capabilities::Game
            + capabilities::Render
            + capabilities::Synced
            + capabilities::Local
            + capabilities::Session
            + capabilities::Common
            + capabilities::Launch
            + capabilities::Downloads
            + capabilities::Replay
            + capabilities::World,
    >() {
    }
    capabilities::<All>();
    assert_eq!(All::MASK, 127);
}

struct TestCell<T>(std::sync::Mutex<T>);
impl<T: Copy> TestCell<T> {
    fn new(value: T) -> Self {
        Self(std::sync::Mutex::new(value))
    }
    fn get(&self) -> T {
        *self.0.lock().unwrap()
    }
    fn set(&self, value: T) {
        *self.0.lock().unwrap() = value;
    }
    fn replace(&self, value: T) -> T {
        std::mem::replace(&mut *self.0.lock().unwrap(), value)
    }
}
struct TestRefCell<T>(std::sync::Mutex<T>);
impl<T> TestRefCell<T> {
    fn new(value: T) -> Self {
        Self(std::sync::Mutex::new(value))
    }
    fn borrow(&self) -> std::sync::MutexGuard<'_, T> {
        self.0.lock().unwrap()
    }
    fn borrow_mut(&self) -> std::sync::MutexGuard<'_, T> {
        self.0.lock().unwrap()
    }
    fn try_borrow_mut(&self) -> std::sync::TryLockResult<std::sync::MutexGuard<'_, T>> {
        self.0.try_lock()
    }
}

#[test]
fn native_module_and_retained_callbacks_survive_loading_thread_handoff() {
    use crate::module::{Module, NativeSlot};
    use std::sync::atomic::{AtomicUsize, Ordering};
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    struct App {
        count: Cell<usize>,
        reference: ModuleRef<All>,
    }
    impl Module<All> for App {
        fn initialize(spring: &mut Spring<'_, All>) -> Result<Self> {
            spring
                .rml_ui()
                .data_model_bind_event(1, "event", &[3, 1], |spring, _| {
                    CALLS.fetch_add(1, Ordering::Relaxed);
                    // A fresh Spring scope uses the engine interface after handoff.
                    assert_eq!(spring.units().get_all_units()?.len(), 2);
                    Ok(())
                })?;
            Ok(Self {
                count: Cell::new(0),
                reference: spring.module_ref(),
            })
        }
    }
    CALLS.store(0, Ordering::Relaxed);
    let (_, units, _) = tables();
    let rml = sys::RmlUiApi {
        DataModelBindEvent: Some(bind_event),
        ..Default::default()
    };
    let interface = sys::NativeInterface {
        unitsQuery: &units,
        rmlUi: &rml,
        ..Default::default()
    };
    let address = &interface as *const sys::NativeInterface as usize;
    let slot = NativeSlot::<All, App>::new();
    let (identity, registration) = std::thread::scope(|threads| {
        threads
            .spawn(|| {
                REJECT_REGISTRATION.with(|v| v.set(false));
                unsafe {
                    slot.with_host(|slot| {
                        let identity = slot.initialize_native(address as *const _).unwrap();
                        let registration = REGISTRATION.with(|r| r.borrow_mut().take().unwrap());
                        (identity as usize, registration)
                    })
                }
            })
            .join()
            .unwrap()
    });
    // Safe user access on an unadmitted thread cannot transfer the module.
    let refused = std::thread::scope(|threads| {
        threads
            .spawn(|| std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| slot.with(|_| ()))))
            .join()
            .unwrap()
    });
    assert!(refused.is_err());
    // Only the real host entry is allowed to transfer thread ownership.
    unsafe {
        slot.with_host(|slot| {
            slot.validate_identity(identity as *mut _).unwrap();
            slot.invoke(|module, spring| {
                module.count.set(module.count.get() + 1);
                module.reference.with_spring(|spring| {
                    assert_eq!(spring.units().get_all_units()?.len(), 2);
                    Ok(())
                })?;
                assert_eq!(spring.units().get_all_units()?.len(), 2);
                Ok(())
            })
            .unwrap();
        })
    };
    REGISTRATION.with(|r| *r.borrow_mut() = Some(registration));
    fire_data_event();
    assert_eq!(CALLS.load(Ordering::Relaxed), 1);
    slot.with(|slot| {
        slot.invoke(|app, _| {
            assert_eq!(app.count.get(), 1);
            Ok(())
        })
        .unwrap()
    });
    unsafe { slot.with_host(|slot| slot.shutdown().unwrap()) };
    fire_data_event();
    assert_eq!(CALLS.load(Ordering::Relaxed), 1);
    destroy_registration();
    // Bindings are transferable data; every operation still needs Spring.
    fn send<T: Send>() {}
    send::<crate::rml::DataModel>();
    send::<crate::rml::Variable<String>>();
}

#[test]
fn queued_order_copies_inputs_and_preserves_the_engine_result() {
    thread_local! { static CALLED: Cell<bool> = const { Cell::new(false) }; }
    unsafe extern "C" fn order(
        query: *const sys::GiveOrderToUnitQuery,
        result: *mut sys::GiveOrderToUnitResult,
    ) {
        let query = unsafe { &*query };
        assert_eq!(
            (query.unitID, query.cmdID, query.options, query.timeout),
            (12, 10, 4, -1)
        );
        assert_eq!(
            unsafe { core::slice::from_raw_parts(query.params, query.paramCount as usize) },
            [2.0, 3.0]
        );
        CALLED.with(|called| called.set(true));
        unsafe {
            (*result).success = false;
        }
    }
    let unit = sys::UnitControlApi {
        GiveOrderToUnit: Some(order),
        ..Default::default()
    };
    let ctrl = sys::SyncedCtrlApi {
        unit: &unit,
        ..Default::default()
    };
    let interface = sys::NativeInterface {
        syncedCtrl: &ctrl,
        ..Default::default()
    };
    let context = unsafe { ModuleContext::<RulesSynced>::from_native(&interface) }.unwrap();
    let queue = commands::Queue::<RulesSynced>::new();
    let result = std::sync::Arc::new(std::sync::Mutex::new(None));
    let returned = result.clone();
    let mut params = vec![2.0, 3.0];
    queue
        .give_order_to_unit(12, 10, &params, 4, -1)
        .then(move |_, value| {
            *returned.lock().unwrap() = Some(value);
        });
    params.fill(99.0);
    drop(params);
    CALLED.with(|called| assert!(!called.get()));
    context.with_spring(|spring| queue.flush(spring)).unwrap();
    assert_eq!(*result.lock().unwrap(), Some(Ok(false)));
    CALLED.with(|called| assert!(called.get()));
}

#[test]
fn queued_commands_keep_fifo_continuations_and_immediate_reentry() {
    use crate::{
        commands::Queue,
        module::{Module, NativeSlot},
    };
    static ORDER: std::sync::Mutex<Vec<&'static str>> = std::sync::Mutex::new(Vec::new());
    struct App {
        queue: Queue<All>,
    }
    impl Module<All> for App {
        fn initialize(_: &mut Spring<'_, All>) -> Result<Self> {
            Ok(Self {
                queue: Queue::new(),
            })
        }
    }
    static SLOT: NativeSlot<All, App> = NativeSlot::new();
    let interface = sys::NativeInterface::default();
    unsafe { SLOT.with_host(|slot| slot.initialize_native(&interface).unwrap()) };
    SLOT.with(|slot| {
        slot.invoke(|app, spring| {
            let mut outer = app.queue.scope(spring);
            app.queue
                .queue(|_| {
                    ORDER.lock().unwrap().push("first");
                    SLOT.with(|slot| {
                        slot.invoke(|app, spring| {
                            ORDER.lock().unwrap().push("nested callin");
                            let _nested = app.queue.scope(spring);
                            app.queue.run(|_| ORDER.lock().unwrap().push("appended"));
                            Ok(())
                        })
                        .unwrap()
                    });
                    7
                })
                .then(|_, value| {
                    assert_eq!(value, 7);
                    ORDER.lock().unwrap().push("continuation");
                });
            {
                let _nested = app.queue.scope(outer.spring());
                app.queue.run(|_| ORDER.lock().unwrap().push("second"));
            }
            assert!(ORDER.lock().unwrap().is_empty());
            ORDER.lock().unwrap().push("delayed action");
            Ok(())
        })
        .unwrap()
    });
    assert_eq!(
        *ORDER.lock().unwrap(),
        [
            "delayed action",
            "first",
            "nested callin",
            "continuation",
            "second",
            "appended"
        ]
    );
    unsafe { SLOT.with_host(|slot| slot.shutdown().unwrap()) };
}

#[test]
fn stored_module_identity_only_opens_scopes_inside_its_own_live_callin() {
    let (game, units, config) = tables();
    let interface = sys::NativeInterface {
        game: &game,
        unitsQuery: &units,
        config: &config,
        ..Default::default()
    };
    let context = unsafe { ModuleContext::<All>::from_native(&interface) }.unwrap();
    let reference = context.with_spring(|spring| spring.module_ref()).unwrap();
    assert_eq!(
        reference
            .with_spring::<()>(|_| panic!("outside a callin"))
            .unwrap_err()
            .code,
        ErrorCode::InvalidState as i32
    );
    context
        .with_spring(|spring| {
            let helper = spring.reborrow();
            assert_eq!(helper.units().get_all_units()?.len(), 2);
            reference.with_spring(|spring| {
                assert_eq!(spring.game().get_game_frame()?.low16, 42);
                Ok(())
            })?;
            std::thread::scope(|threads| {
                let worker_reference = reference.clone();
                let error = threads
                    .spawn(move || worker_reference.with_spring::<()>(|_| panic!("worker scope")))
                    .join()
                    .unwrap()
                    .unwrap_err();
                assert_eq!(error.code, ErrorCode::InvalidState as i32);
            });
            Ok::<_, ApiError>(())
        })
        .unwrap()
        .unwrap();
    let other = unsafe { ModuleContext::<All>::from_native(&interface) }.unwrap();
    other
        .with_spring(|_| {
            assert_eq!(
                reference
                    .with_spring::<()>(|_| panic!("different module"))
                    .unwrap_err()
                    .code,
                ErrorCode::InvalidState as i32
            );
        })
        .unwrap();
    drop(other);
    drop(context);
    assert_eq!(
        reference
            .with_spring::<()>(|_| panic!("dead module"))
            .unwrap_err()
            .code,
        ErrorCode::InvalidState as i32
    );
}

unsafe extern "C" fn pointer_delta(
    query: *const sys::RmlContextHandleQuery,
    result: *mut sys::RmlContextPointerDeltaResult,
) {
    let status = unsafe { (*query).contextHandle as i32 };
    unsafe {
        *result = sys::RmlContextPointerDeltaResult {
            error: ptr::null(),
            deltaX: -7,
            deltaY: 11,
            status,
        };
    }
}

#[test]
fn pointer_capture_preserves_deltas_and_rejects_unknown_status() {
    use rml::PointerCaptureStatus::{Active, Cancelled, None, Released};
    let rml = sys::RmlUiApi {
        ContextTakePointerCaptureDelta: Some(pointer_delta),
        ..Default::default()
    };
    let interface = sys::NativeInterface {
        rmlUi: &rml,
        ..Default::default()
    };
    let context = unsafe { ModuleContext::<All>::from_native(&interface) }.unwrap();
    context
        .with_spring(|spring| {
            for (raw, status) in [None, Active, Released, Cancelled].into_iter().enumerate() {
                let delta = spring
                    .rml_ui()
                    .take_pointer_capture_delta(raw as u64)
                    .unwrap();
                assert_eq!(
                    delta,
                    rml::PointerCaptureDelta {
                        delta_x: -7,
                        delta_y: 11,
                        status
                    }
                );
            }
            assert_eq!(
                spring
                    .rml_ui()
                    .take_pointer_capture_delta(4)
                    .unwrap_err()
                    .code,
                ErrorCode::InvalidState as i32
            );
        })
        .unwrap();
}
