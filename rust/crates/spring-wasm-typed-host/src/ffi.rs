//! The table of C++ shims each callout calls.
//!
//! One shim per callout, taking POD arguments and writing POD results.  This
//! is the whole point of the experiment: the dynamic C API path lowers a
//! `wasmtime_component_val_t` into a `WasmValue` tree and then lowers that
//! into the native struct, and the two trees cost 646 of `callout_vec3`'s
//! 881 ns.  Here Wasmtime hands the typed argument straight to the shim.
//!
//! The engine loads this crate as a `cdylib` and passes the table in, rather
//! than the crate importing engine symbols by name, so the library resolves
//! standalone under `dlopen`.  It is the same shape as `NativeInterface`
//! itself.
//!
//! Every shim returns 0 on success or a Spring error code.  Borrowed outputs
//! (strings, lists) point into a per-call scratch buffer owned by the C++
//! side and stay valid until the next shim call on the same thread, which is
//! sufficient because guest calls are synchronous and single-threaded and the
//! Rust side copies before returning to the guest.

use core::ffi::{c_char, c_void};

/// Mirrors the engine-side `SpringTypedCommand`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SpringTypedCommand {
    pub cmd_id: i32,
    pub tag: i32,
    pub ai_command_id: i32,
    pub time_out: f32,
    pub param_offset: u32,
    pub param_count: u32,
    pub options: u8,
}

/// Mirrors the engine-side `SpringTypedShimTable`.  Field order is load-bearing.
#[repr(C)]
pub struct ShimTable {
    pub messages_send_lua_rules_msg: unsafe extern "C" fn(
        native: *mut c_void,
        message: *const c_char,
        message_len: usize,
        out: *mut bool,
    ) -> i32,

    pub profiling_get_timer_micros:
        unsafe extern "C" fn(native: *mut c_void, unused: u8, out: *mut u64) -> i32,

    pub rules_params_get_unit_rules_param: unsafe extern "C" fn(
        native: *mut c_void,
        unit_id: i32,
        name: *const c_char,
        name_len: usize,
        out_type: *mut i32,
        out_bool: *mut bool,
        out_float: *mut f32,
        out_string: *mut *const c_char,
        out_string_len: *mut usize,
        out_los: *mut i32,
        out_exists: *mut bool,
    ) -> i32,

    pub rules_params_set_unit_rules_param: unsafe extern "C" fn(
        native: *mut c_void,
        unit_id: i32,
        name: *const c_char,
        name_len: usize,
        value_type: i32,
        value_bool: bool,
        value_float: f32,
        value_string: *const c_char,
        value_string_len: usize,
        los: i32,
        out: *mut bool,
    ) -> i32,

    pub terrain_get_ground_orig_height:
        unsafe extern "C" fn(native: *mut c_void, x: f32, z: f32, out: *mut f32) -> i32,

    pub terrain_control_level_height_map: unsafe extern "C" fn(
        native: *mut c_void,
        x1: f32,
        z1: f32,
        x2: f32,
        z2: f32,
        height: f32,
        out: *mut bool,
    ) -> i32,

    pub terrain_control_set_height_map: unsafe extern "C" fn(
        native: *mut c_void,
        x: f32,
        z: f32,
        height: f32,
        terraform: f32,
        out: *mut bool,
    ) -> i32,

    pub unit_control_give_order_to_unit: unsafe extern "C" fn(
        native: *mut c_void,
        unit_id: i32,
        cmd_id: i32,
        params: *const f32,
        params_len: usize,
        options: u32,
        timeout: i32,
        out: *mut bool,
    ) -> i32,

    pub unit_defs_get_unit_def_name: unsafe extern "C" fn(
        native: *mut c_void,
        unit_def_id: i32,
        out: *mut *const c_char,
        out_len: *mut usize,
    ) -> i32,

    pub units_commands_get_unit_commands: unsafe extern "C" fn(
        native: *mut c_void,
        unit_id: i32,
        max_commands: u32,
        out_commands: *mut *const SpringTypedCommand,
        out_command_count: *mut usize,
        out_params: *mut *const f32,
        out_param_count: *mut usize,
    ) -> i32,

    pub units_info_get_unit_def_id:
        unsafe extern "C" fn(native: *mut c_void, unit_id: i32, out: *mut i32) -> i32,

    /// Writes health, max-health, paralyze-damage, capture-progress,
    /// build-progress into `out`.
    pub units_info_get_unit_health:
        unsafe extern "C" fn(native: *mut c_void, unit_id: i32, out: *mut f32) -> i32,

    /// Writes x, y, z into `out`.
    pub units_info_get_unit_position: unsafe extern "C" fn(
        native: *mut c_void,
        unit_id: i32,
        mid_pos: bool,
        aim_pos: bool,
        out: *mut f32,
    ) -> i32,

    pub units_query_get_team_units: unsafe extern "C" fn(
        native: *mut c_void,
        team_id: i32,
        out: *mut *const i32,
        out_len: *mut usize,
    ) -> i32,

    pub units_query_get_units_in_cylinder: unsafe extern "C" fn(
        native: *mut c_void,
        x: f32,
        z: f32,
        radius: f32,
        allegiance: i32,
        out: *mut *const i32,
        out_len: *mut usize,
    ) -> i32,

    /// Takes a thunk plus context rather than a callback index: the engine
    /// invokes the thunk, and the Rust side re-enters the guest from it.
    pub terrain_control_set_height_map_func: unsafe extern "C" fn(
        native: *mut c_void,
        trampoline: unsafe extern "C" fn(*mut c_void),
        trampoline_context: *mut c_void,
        out: *mut bool,
    ) -> i32,

    /// Writes the eight KB/allocation counters into `out`.
    pub profiling_get_lua_mem_usage:
        unsafe extern "C" fn(native: *mut c_void, unused: u8, out: *mut f32) -> i32,

    pub profiling_get_synced_gc_info:
        unsafe extern "C" fn(native: *mut c_void, collect: bool, out: *mut f32) -> i32,
    pub messages_send_lua_ui_msg: unsafe extern "C" fn(
        native: *mut c_void,
        message: *const c_char,
        message_len: usize,
        mode: *const c_char,
        mode_len: usize,
        out: *mut bool,
    ) -> i32,
    pub gfx_vertex: unsafe extern "C" fn(
        native: *mut c_void,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
        count: u32,
    ) -> i32,
    pub gfx_begin_end: unsafe extern "C" fn(
        native: *mut c_void,
        primitive: u32,
        trampoline: unsafe extern "C" fn(*mut c_void),
        trampoline_context: *mut c_void,
    ) -> i32,
}

/// Erases a closure to the C thunk signature the native API expects.
/// Monomorphised per closure so the context pointer needs no vtable.
pub unsafe extern "C" fn trampoline<F: FnMut()>(data: *mut c_void) {
    if data.is_null() {
        return;
    }
    (*(data as *mut F))();
}

pub fn as_trampoline<F: FnMut()>(
    closure: &mut F,
) -> (unsafe extern "C" fn(*mut c_void), *mut c_void) {
    (trampoline::<F>, closure as *mut F as *mut c_void)
}
