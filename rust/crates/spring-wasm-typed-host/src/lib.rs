//! Wasmtime host reaching the Component Model through the typed Rust API
//! instead of the dynamic C API.
//!
//! Same WIT, same wit-bindgen, same componentize step, same `.wasm` bytes as
//! the C API path: only the host differs, so the two columns of the benchmark
//! table are a like-for-like comparison of the transport.

pub(crate) mod bindings {
    wasmtime::component::bindgen!({
        path: "wit",
        world: "benchmark-rules-synced",
    });
}

/// Second world for the unsynced guest.  `with` reuses the synced world's
/// import types so the Host traits are implemented once, not twice; only the
/// exported callin interface differs between the two.
pub(crate) mod unsynced_bindings {
    wasmtime::component::bindgen!({
        path: "wit",
        world: "benchmark-rules-unsynced",
        with: {
            "recoil:spring-api/messages": crate::bindings::recoil::spring_api::messages,
            "recoil:spring-api/profiling": crate::bindings::recoil::spring_api::profiling,
            "recoil:spring-api/units-info": crate::bindings::recoil::spring_api::units_info,
            "recoil:spring-api/units-query": crate::bindings::recoil::spring_api::units_query,
        },
    });
}

/// Third world for the draw profile.  Reuses the shared messages and profiling
/// import types; gfx and the callin interface are its own.
pub(crate) mod ui_bindings {
    wasmtime::component::bindgen!({
        path: "wit",
        world: "benchmark-ui",
        with: {
            "recoil:spring-api/messages": crate::bindings::recoil::spring_api::messages,
            "recoil:spring-api/profiling": crate::bindings::recoil::spring_api::profiling,
        },
    });
}

mod ffi;
mod host;

use core::ffi::{c_char, c_void};

use wasmtime::Result;
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};

pub struct HostState {
    /// Opaque `NativeInterface*` owned by the engine.
    native: *mut c_void,
    /// Borrowed from the engine, which keeps it alive for the process.
    shims: *const ffi::ShimTable,
    /// The guest's `callback-1` export, resolved after instantiation. Held
    /// here because the heightmap callback has to re-enter the guest from
    /// inside a host call, where only the store is reachable.
    callback: Option<wasmtime::component::TypedFunc<(u32,), ()>>,
}

impl HostState {
    #[inline]
    fn shims(&self) -> &ffi::ShimTable {
        // The engine passes a pointer to a static table and outlives the host.
        unsafe { &*self.shims }
    }
}

// The engine drives every guest call from the sim thread; the pointer is not
// shared across threads.
unsafe impl Send for HostState {}

/// The two guests export different callin interfaces.  `None` means the guest
/// exports none at all, which is what the callin_unimplemented row measures;
/// requiring the export would make that guest fail to load instead.
enum Callins {
    Synced(bindings::BenchmarkRulesSynced),
    Unsynced(unsynced_bindings::BenchmarkRulesUnsynced),
    Ui(ui_bindings::BenchmarkUi),
    None,
}

/// Mirrors `SpringTypedWorld` in rts/WasmInterface/WasmTypedHost.h.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum World {
    RulesSynced,
    RulesUnsynced,
    Ui,
}

impl World {
    fn from_raw(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::RulesSynced),
            1 => Some(Self::RulesUnsynced),
            2 => Some(Self::Ui),
            _ => None,
        }
    }
}

pub struct TypedHost {
    store: Store<HostState>,
    instance: Callins,
}

/// Mirrors rts/WasmInterface/WasmRuntime.cpp so both hosts measure the same
/// Cranelift and trap configuration.
fn engine() -> Result<Engine> {
    let mut config = Config::new();
    config.consume_fuel(false);
    config.wasm_component_model(true);
    config.signals_based_traps(true);
    config.wasm_threads(false);
    config.wasm_relaxed_simd(false);
    config.relaxed_simd_deterministic(true);
    config.cranelift_nan_canonicalization(true);
    config.wasm_multi_value(true);
    config.wasm_bulk_memory(true);
    Engine::new(&config)
}

impl TypedHost {
    fn new(
        component_bytes: &[u8],
        native: *mut c_void,
        shims: *const ffi::ShimTable,
        world: World,
    ) -> Result<Self> {
        let engine = engine()?;
        let component = Component::from_binary(&engine, component_bytes)?;
        let mut linker = Linker::<HostState>::new(&engine);
        // define_unknown_imports_as_traps works at instance granularity: an
        // interface this host implements only part of still counts as unknown,
        // and defining it wholesale collides with the real definitions.  So
        // trap everything first and let the 16 implemented callouts shadow
        // their traps.  Anything the guest calls that is not implemented here
        // still traps, which is louder than a silent default.
        linker.allow_shadowing(true);
        linker.define_unknown_imports_as_traps(&component)?;
        // Per interface rather than the world-level add_to_linker, because
        // terrain-control has to be defined by hand: defining an instance
        // replaces it wholesale, so the hand-written one must carry every
        // function the guest imports from it, not just the callback.
        use bindings::recoil::spring_api as api;
        api::messages::add_to_linker::<_, HasHostState>(&mut linker, |state| state)?;
        api::profiling::add_to_linker::<_, HasHostState>(&mut linker, |state| state)?;
        api::rules_params::add_to_linker::<_, HasHostState>(&mut linker, |state| state)?;
        api::terrain::add_to_linker::<_, HasHostState>(&mut linker, |state| state)?;
        api::unit_control::add_to_linker::<_, HasHostState>(&mut linker, |state| state)?;
        api::unit_defs::add_to_linker::<_, HasHostState>(&mut linker, |state| state)?;
        api::units_commands::add_to_linker::<_, HasHostState>(&mut linker, |state| state)?;
        api::units_info::add_to_linker::<_, HasHostState>(&mut linker, |state| state)?;
        api::units_query::add_to_linker::<_, HasHostState>(&mut linker, |state| state)?;
        define_terrain_control(&mut linker)?;
        define_gfx(&mut linker)?;
        linker.allow_shadowing(false);
        let mut store = Store::new(&engine, HostState { native, shims, callback: None });
        let raw = linker.instantiate(&mut store, &component)?;
        let instance = match world {
            World::RulesSynced => bindings::BenchmarkRulesSynced::new(&mut store, &raw)
                .map(Callins::Synced)
                .unwrap_or(Callins::None),
            World::RulesUnsynced => {
                unsynced_bindings::BenchmarkRulesUnsynced::new(&mut store, &raw)
                    .map(Callins::Unsynced)
                    .unwrap_or(Callins::None)
            }
            World::Ui => ui_bindings::BenchmarkUi::new(&mut store, &raw)
                .map(Callins::Ui)
                .unwrap_or(Callins::None),
        };
        // Absent for guests that export no callback, which is fine: only the
        // heightmap rows use it.
        let callback = raw
            .get_typed_func::<(u32,), ()>(&mut store, "callback-1")
            .ok();
        store.data_mut().callback = callback;
        Ok(Self { store, instance })
    }
}

type TerrainError = bindings::recoil::spring_api::terrain_control::SpringError;

/// Defines every terrain-control function the guest imports.  set-height-map-func
/// cannot go through bindgen's Host trait, which hands out only `&mut HostState`,
/// because re-entering the guest needs the store.
fn define_terrain_control(linker: &mut Linker<HostState>) -> Result<()> {
    let mut instance = linker.instance("recoil:spring-api/terrain-control@1.0.0")?;
    instance.func_wrap(
        "level-height-map",
        |mut store: wasmtime::StoreContextMut<'_, HostState>,
         (x1, z1, x2, z2, height): (f32, f32, f32, f32, f32)|
         -> Result<(core::result::Result<bool, TerrainError>,)> {
            let (native, shims) = (store.data().native, store.data().shims);
            let mut out = false;
            let code = unsafe {
                ((*shims).terrain_control_level_height_map)(
                    native, x1, z1, x2, z2, height, &mut out,
                )
            };
            Ok((if code == 0 { Ok(out) } else { Err(TerrainError { code }) },))
        },
    )?;
    instance.func_wrap(
        "set-height-map",
        |mut store: wasmtime::StoreContextMut<'_, HostState>,
         (x, z, height, terraform): (f32, f32, f32, f32)|
         -> Result<(core::result::Result<bool, TerrainError>,)> {
            let (native, shims) = (store.data().native, store.data().shims);
            let mut out = false;
            let code = unsafe {
                ((*shims).terrain_control_set_height_map)(native, x, z, height, terraform, &mut out)
            };
            Ok((if code == 0 { Ok(out) } else { Err(TerrainError { code }) },))
        },
    )?;
    instance.func_wrap(
        "set-height-map-func",
        |mut store: wasmtime::StoreContextMut<'_, HostState>,
         (_callback, user_data): (u32, u32)|
         -> Result<(core::result::Result<bool, TerrainError>,)> {
            let native = store.data().native;
            let shims = store.data().shims;
            let Some(guest) = store.data().callback else {
                return Ok((Err(TerrainError { code: -1 }),));
            };
            // The engine calls this back once, between opening and closing the
            // heightmap edit session; every set-height-map the guest makes
            // inside it lands in that session.
            let mut invoke = || {
                if guest.call(&mut store, (user_data,)).is_ok() {
                    let _ = guest.post_return(&mut store);
                }
            };
            let (thunk, context) = ffi::as_trampoline(&mut invoke);
            let mut out = false;
            let code = unsafe {
                ((*shims).terrain_control_set_height_map_func)(native, thunk, context, &mut out)
            };
            Ok((if code == 0 {
                Ok(out)
            } else {
                Err(TerrainError { code })
            },))
        },
    )?;
    Ok(())
}

type GfxError = ui_bindings::recoil::spring_api::gfx::SpringError;

/// Both gfx functions the UI guest imports.  begin-end re-enters the guest the
/// same way set-height-map-func does, so neither can go through bindgen's Host
/// trait, and defining an instance replaces it wholesale.
fn define_gfx(linker: &mut Linker<HostState>) -> Result<()> {
    let mut instance = linker.instance("recoil:spring-api/gfx@1.0.0")?;
    instance.func_wrap(
        "vertex",
        |store: wasmtime::StoreContextMut<'_, HostState>,
         (x, y, z, w, count): (f32, f32, f32, f32, u32)|
         -> Result<(core::result::Result<(), GfxError>,)> {
            let (native, shims) = (store.data().native, store.data().shims);
            let code = unsafe { ((*shims).gfx_vertex)(native, x, y, z, w, count) };
            Ok((if code == 0 { Ok(()) } else { Err(GfxError { code }) },))
        },
    )?;
    instance.func_wrap(
        "begin-end",
        |mut store: wasmtime::StoreContextMut<'_, HostState>,
         (primitive, _callback, user_data): (u32, u32, u32)|
         -> Result<(core::result::Result<(), GfxError>,)> {
            let native = store.data().native;
            let shims = store.data().shims;
            let Some(guest) = store.data().callback else {
                return Ok((Err(GfxError { code: -1 }),));
            };
            // The engine calls back once, between glBegin and glEnd; every
            // vertex the guest emits inside it lands in that primitive.
            let mut invoke = || {
                if guest.call(&mut store, (user_data,)).is_ok() {
                    let _ = guest.post_return(&mut store);
                }
            };
            let (thunk, context) = ffi::as_trampoline(&mut invoke);
            let code = unsafe { ((*shims).gfx_begin_end)(native, primitive, thunk, context) };
            Ok((if code == 0 { Ok(()) } else { Err(GfxError { code }) },))
        },
    )?;
    Ok(())
}

struct HasHostState;

impl wasmtime::component::HasData for HasHostState {
    type Data<'a> = &'a mut HostState;
}

// ---------------------------------------------------------------- C ABI

/// Returned to the engine on failure; freed with `spring_wasm_typed_string_free`.
fn set_error(error_out: *mut *mut c_char, error: &wasmtime::Error) {
    if error_out.is_null() {
        return;
    }
    let text = format!("{error:#}");
    let mut bytes = text.into_bytes();
    bytes.push(0);
    bytes.shrink_to_fit();
    let pointer = bytes.as_mut_ptr() as *mut c_char;
    core::mem::forget(bytes);
    unsafe { *error_out = pointer };
}

#[no_mangle]
pub extern "C" fn spring_wasm_typed_string_free(pointer: *mut c_char) {
    if pointer.is_null() {
        return;
    }
    unsafe {
        let mut len = 0usize;
        while *pointer.add(len) != 0 {
            len += 1;
        }
        drop(Vec::from_raw_parts(pointer as *mut u8, len + 1, len + 1));
    }
}

#[no_mangle]
pub unsafe extern "C" fn spring_wasm_typed_host_new(
    component_bytes: *const u8,
    component_len: usize,
    native: *mut c_void,
    shims: *const ffi::ShimTable,
    world: u8,
    error_out: *mut *mut c_char,
) -> *mut TypedHost {
    if component_bytes.is_null() || component_len == 0 || shims.is_null() {
        set_error(error_out, &wasmtime::Error::msg("empty component or missing shim table"));
        return core::ptr::null_mut();
    }
    let Some(world) = World::from_raw(world) else {
        set_error(error_out, &wasmtime::Error::msg("unknown world selector"));
        return core::ptr::null_mut();
    };
    let bytes = core::slice::from_raw_parts(component_bytes, component_len);
    match TypedHost::new(bytes, native, shims, world) {
        Ok(host) => Box::into_raw(Box::new(host)),
        Err(error) => {
            set_error(error_out, &error);
            core::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn spring_wasm_typed_host_free(host: *mut TypedHost) {
    if !host.is_null() {
        drop(Box::from_raw(host));
    }
}

/// Callin entry points. Each returns 0 on success, -1 on a host-side failure
/// with `error_out` set, or the guest's Spring error code. A guest that does
/// not export the callin returns 0 without dispatching, which is what the
/// callin_unimplemented row measures.
macro_rules! finish {
    ($error_out:expr, $result:expr) => {
        match $result {
            Ok(Ok(_)) => 0,
            Ok(Err(error)) => error.code,
            Err(error) => {
                set_error($error_out, &error);
                -1
            }
        }
    };
}

use bindings::exports::recoil::spring_api::callins_rules_synced as sync_callins;
use unsynced_bindings::exports::recoil::spring_api::callins_rules_unsynced as unsync_callins;
use ui_bindings::exports::recoil::spring_api::callins_ui as ui_callins;

#[no_mangle]
pub unsafe extern "C" fn spring_wasm_typed_callin_game_frame(
    host: *mut TypedHost,
    game_frame: i32,
    error_out: *mut *mut c_char,
) -> i32 {
    let Some(host) = host.as_mut() else { return -1 };
    match &host.instance {
        Callins::Synced(bindings) => finish!(
            error_out,
            bindings
                .recoil_spring_api_callins_rules_synced()
                .call_game_frame(&mut host.store, sync_callins::GameFrameQuery { game_frame })
        ),
        Callins::Unsynced(bindings) => finish!(
            error_out,
            bindings
                .recoil_spring_api_callins_rules_unsynced()
                .call_game_frame(&mut host.store, unsync_callins::GameFrameQuery { game_frame })
        ),
        Callins::Ui(_) | Callins::None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn spring_wasm_typed_callin_game_frame_post(
    host: *mut TypedHost,
    game_frame: i32,
    error_out: *mut *mut c_char,
) -> i32 {
    let Some(host) = host.as_mut() else { return -1 };
    match &host.instance {
        Callins::Synced(bindings) => finish!(
            error_out,
            bindings
                .recoil_spring_api_callins_rules_synced()
                .call_game_frame_post(&mut host.store, sync_callins::GameFrameQuery { game_frame })
        ),
        Callins::Unsynced(bindings) => finish!(
            error_out,
            bindings
                .recoil_spring_api_callins_rules_unsynced()
                .call_game_frame_post(
                    &mut host.store,
                    unsync_callins::GameFrameQuery { game_frame }
                )
        ),
        Callins::Ui(_) | Callins::None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn spring_wasm_typed_callin_update(
    host: *mut TypedHost,
    delta_seconds: f32,
    error_out: *mut *mut c_char,
) -> i32 {
    let Some(host) = host.as_mut() else { return -1 };
    match &host.instance {
        Callins::Synced(bindings) => finish!(
            error_out,
            bindings
                .recoil_spring_api_callins_rules_synced()
                .call_update(&mut host.store, sync_callins::UpdateQuery { delta_seconds })
        ),
        Callins::Unsynced(bindings) => finish!(
            error_out,
            bindings
                .recoil_spring_api_callins_rules_unsynced()
                .call_update(&mut host.store, unsync_callins::UpdateQuery { delta_seconds })
        ),
        Callins::Ui(_) | Callins::None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn spring_wasm_typed_callin_unit_created(
    host: *mut TypedHost,
    unit_id: i32,
    unit_def_id: i32,
    unit_team: i32,
    builder_id: i32,
    error_out: *mut *mut c_char,
) -> i32 {
    let Some(host) = host.as_mut() else { return -1 };
    // Synced-only in Lua and here.
    let Callins::Synced(bindings) = &host.instance else { return 0 };
    finish!(
        error_out,
        bindings
            .recoil_spring_api_callins_rules_synced()
            .call_unit_created(
                &mut host.store,
                sync_callins::UnitCreatedQuery {
                    unit_id,
                    unit_def_id,
                    unit_team,
                    builder_id,
                },
            )
    )
}

#[no_mangle]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C" fn spring_wasm_typed_callin_unit_pre_damaged(
    host: *mut TypedHost,
    unit_id: i32,
    unit_def_id: i32,
    unit_team: i32,
    damage: f32,
    paralyzer: bool,
    weapon_def_id: i32,
    projectile_id: i32,
    attacker_id: i32,
    attacker_def_id: i32,
    attacker_team: i32,
    out_new_damage: *mut f32,
    out_impulse_mult: *mut f32,
    error_out: *mut *mut c_char,
) -> i32 {
    let Some(host) = host.as_mut() else { return -1 };
    let Callins::Synced(bindings) = &host.instance else { return 0 };
    let query = sync_callins::UnitDamagedQuery {
        unit_id,
        unit_def_id,
        unit_team,
        damage,
        paralyzer,
        weapon_def_id,
        projectile_id,
        attacker_id,
        attacker_def_id,
        attacker_team,
    };
    match bindings
        .recoil_spring_api_callins_rules_synced()
        .call_unit_pre_damaged(&mut host.store, query)
    {
        Ok(Ok(result)) => {
            if !out_new_damage.is_null() {
                *out_new_damage = result.new_damage;
            }
            if !out_impulse_mult.is_null() {
                *out_impulse_mult = result.impulse_mult;
            }
            0
        }
        Ok(Err(error)) => error.code,
        Err(error) => {
            set_error(error_out, &error);
            -1
        }
    }
}

#[no_mangle]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C" fn spring_wasm_typed_callin_allow_unit_creation(
    host: *mut TypedHost,
    unit_def_id: i32,
    builder_id: i32,
    builder_team: i32,
    has_build_info: bool,
    build_pos_x: f32,
    build_pos_y: f32,
    build_pos_z: f32,
    build_facing: i32,
    out_allow: *mut bool,
    out_drop_order: *mut bool,
    error_out: *mut *mut c_char,
) -> i32 {
    let Some(host) = host.as_mut() else { return -1 };
    let Callins::Synced(bindings) = &host.instance else { return 0 };
    let query = sync_callins::AllowUnitCreationQuery {
        unit_def_id,
        builder_id,
        builder_team,
        has_build_info,
        build_pos: sync_callins::Float3 {
            x: build_pos_x,
            y: build_pos_y,
            z: build_pos_z,
        },
        build_facing,
    };
    match bindings
        .recoil_spring_api_callins_rules_synced()
        .call_allow_unit_creation(&mut host.store, query)
    {
        Ok(Ok(result)) => {
            if !out_allow.is_null() {
                *out_allow = result.allow;
            }
            if !out_drop_order.is_null() {
                *out_drop_order = result.drop_order;
            }
            0
        }
        Ok(Err(error)) => error.code,
        Err(error) => {
            set_error(error_out, &error);
            -1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn spring_wasm_typed_callin_draw_world(
    host: *mut TypedHost,
    error_out: *mut *mut c_char,
) -> i32 {
    let Some(host) = host.as_mut() else { return -1 };
    let Callins::Ui(bindings) = &host.instance else {
        return 0;
    };
    let query = ui_callins::SimpleCallinQuery { unused: 0 };
    finish!(
        error_out,
        bindings
            .recoil_spring_api_callins_ui()
            .call_draw_world(&mut host.store, query)
    )
}
