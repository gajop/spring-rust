//! The generated host traits, implemented by calling the C++ shims.

use core::ffi::c_char;

use crate::bindings::recoil::spring_api as api;
use crate::ffi;
use crate::HostState;

/// Every interface declares its own `spring-error`, so the code has to be
/// rebuilt per interface rather than shared.
macro_rules! err {
    ($module:path, $code:expr) => {{
        use $module as m;
        m::SpringError { code: $code }
    }};
}

/// `0` means success; anything else is a Spring error code.
macro_rules! check {
    ($module:path, $code:expr, $ok:expr) => {{
        let code = $code;
        if code == 0 {
            Ok($ok)
        } else {
            Err(err!($module, code))
        }
    }};
}

unsafe fn borrowed_str<'a>(pointer: *const c_char, len: usize) -> &'a str {
    if pointer.is_null() || len == 0 {
        return "";
    }
    let bytes = core::slice::from_raw_parts(pointer as *const u8, len);
    core::str::from_utf8(bytes).unwrap_or("")
}

unsafe fn borrowed_slice<'a, T>(pointer: *const T, len: usize) -> &'a [T] {
    if pointer.is_null() || len == 0 {
        return &[];
    }
    core::slice::from_raw_parts(pointer, len)
}

impl api::messages::Host for HostState {
    fn send_lua_rules_msg(&mut self, message: String) -> Result<bool, api::messages::SpringError> {
        let mut out = false;
        let code = unsafe {
            (self.shims().messages_send_lua_rules_msg)(
                self.native,
                message.as_ptr() as *const c_char,
                message.len(),
                &mut out,
            )
        };
        check!(api::messages, code, out)
    }
}

impl api::profiling::Host for HostState {
    fn get_timer_micros(&mut self, unused: u8) -> Result<u64, api::profiling::SpringError> {
        let mut out = 0u64;
        let code = unsafe {
            (self.shims().profiling_get_timer_micros)(self.native, unused, &mut out)
        };
        check!(api::profiling, code, out)
    }

    fn get_lua_mem_usage(
        &mut self,
        unused: u8,
    ) -> Result<api::profiling::GetLuaMemUsageValue, api::profiling::SpringError> {
        let mut out = [0.0f32; 8];
        let code = unsafe {
            (self.shims().profiling_get_lua_mem_usage)(self.native, unused, out.as_mut_ptr())
        };
        check!(
            api::profiling,
            code,
            api::profiling::GetLuaMemUsageValue {
                handle_alloced_kb: out[0],
                handle_allocs_k: out[1],
                global_alloced_kb: out[2],
                global_allocs_k: out[3],
                unsynced_alloced_kb: out[4],
                unsynced_allocs_k: out[5],
                synced_alloced_kb: out[6],
                synced_allocs_k: out[7],
            }
        )
    }

    fn get_synced_gc_info(&mut self, collect: bool) -> Result<f32, api::profiling::SpringError> {
        let mut out = 0.0f32;
        let code =
            unsafe { (self.shims().profiling_get_synced_gc_info)(self.native, collect, &mut out) };
        check!(api::profiling, code, out)
    }
}

impl api::rules_params::Host for HostState {
    fn get_unit_rules_param(
        &mut self,
        unit_id: i32,
        param_name: String,
    ) -> Result<api::rules_params::GetUnitRulesParamValue, api::rules_params::SpringError> {
        let mut value_type = 0i32;
        let mut bool_value = false;
        let mut float_value = 0.0f32;
        let mut string_pointer: *const c_char = core::ptr::null();
        let mut string_len = 0usize;
        let mut los = 0i32;
        let mut exists = false;
        let code = unsafe {
            (self.shims().rules_params_get_unit_rules_param)(
                self.native,
                unit_id,
                param_name.as_ptr() as *const c_char,
                param_name.len(),
                &mut value_type,
                &mut bool_value,
                &mut float_value,
                &mut string_pointer,
                &mut string_len,
                &mut los,
                &mut exists,
            )
        };
        if code != 0 {
            return Err(err!(api::rules_params, code));
        }
        let string_value = unsafe { borrowed_str(string_pointer, string_len) }.to_owned();
        Ok(api::rules_params::GetUnitRulesParamValue {
            value: api::rules_params::RulesParamValue {
                type_: rules_param_type(value_type),
                bool_value,
                float_value,
                string_value,
            },
            los,
            exists,
        })
    }

    fn set_unit_rules_param(
        &mut self,
        unit_id: i32,
        param_name: String,
        value: api::rules_params::RulesParamValue,
        los: i32,
    ) -> Result<bool, api::rules_params::SpringError> {
        let mut out = false;
        let code = unsafe {
            (self.shims().rules_params_set_unit_rules_param)(
                self.native,
                unit_id,
                param_name.as_ptr() as *const c_char,
                param_name.len(),
                rules_param_type_code(value.type_),
                value.bool_value,
                value.float_value,
                value.string_value.as_ptr() as *const c_char,
                value.string_value.len(),
                los,
                &mut out,
            )
        };
        check!(api::rules_params, code, out)
    }
}

fn rules_param_type(code: i32) -> api::rules_params::RulesParamType {
    match code {
        0 => api::rules_params::RulesParamType::RulesparamTypeBool,
        2 => api::rules_params::RulesParamType::RulesparamTypeString,
        _ => api::rules_params::RulesParamType::RulesparamTypeFloat,
    }
}

fn rules_param_type_code(value: api::rules_params::RulesParamType) -> i32 {
    match value {
        api::rules_params::RulesParamType::RulesparamTypeBool => 0,
        api::rules_params::RulesParamType::RulesparamTypeFloat => 1,
        api::rules_params::RulesParamType::RulesparamTypeString => 2,
    }
}

impl api::terrain::Host for HostState {
    fn get_ground_orig_height(
        &mut self,
        x: f32,
        z: f32,
    ) -> Result<f32, api::terrain::SpringError> {
        let mut out = 0.0f32;
        let code = unsafe {
            (self.shims().terrain_get_ground_orig_height)(self.native, x, z, &mut out)
        };
        check!(api::terrain, code, out)
    }
}

impl api::terrain_control::Host for HostState {
    fn level_height_map(
        &mut self,
        x1: f32,
        z1: f32,
        x2: f32,
        z2: f32,
        height: f32,
    ) -> Result<bool, api::terrain_control::SpringError> {
        let mut out = false;
        let code = unsafe {
            (self.shims().terrain_control_level_height_map)(
                self.native,
                x1,
                z1,
                x2,
                z2,
                height,
                &mut out,
            )
        };
        check!(api::terrain_control, code, out)
    }

    fn set_height_map(
        &mut self,
        x: f32,
        z: f32,
        height: f32,
        terraform: f32,
    ) -> Result<bool, api::terrain_control::SpringError> {
        let mut out = false;
        let code = unsafe {
            (self.shims().terrain_control_set_height_map)(
                self.native,
                x,
                z,
                height,
                terraform,
                &mut out,
            )
        };
        check!(api::terrain_control, code, out)
    }

    fn set_height_map_func(
        &mut self,
        _callback: u32,
        _user_data: u32,
    ) -> Result<bool, api::terrain_control::SpringError> {
        // Unreachable: the linker shadows this with define_set_height_map_func,
        // which gets the store it needs to re-enter the guest. The trait still
        // has to be satisfied, so this stays as a loud fallback.
        Err(err!(api::terrain_control, -1))
    }
}

impl api::unit_control::Host for HostState {
    fn give_order_to_unit(
        &mut self,
        unit_id: i32,
        cmd_id: i32,
        params: Vec<f32>,
        options: u32,
        timeout: i32,
    ) -> Result<bool, api::unit_control::SpringError> {
        let mut out = false;
        let code = unsafe {
            (self.shims().unit_control_give_order_to_unit)(
                self.native,
                unit_id,
                cmd_id,
                params.as_ptr(),
                params.len(),
                options,
                timeout,
                &mut out,
            )
        };
        check!(api::unit_control, code, out)
    }
}

impl api::unit_defs::Host for HostState {
    fn get_unit_def_name(
        &mut self,
        unit_def_id: i32,
    ) -> Result<String, api::unit_defs::SpringError> {
        let mut pointer: *const c_char = core::ptr::null();
        let mut len = 0usize;
        let code = unsafe {
            (self.shims().unit_defs_get_unit_def_name)(
                self.native,
                unit_def_id,
                &mut pointer,
                &mut len,
            )
        };
        if code != 0 {
            return Err(err!(api::unit_defs, code));
        }
        Ok(unsafe { borrowed_str(pointer, len) }.to_owned())
    }
}

impl api::units_commands::Host for HostState {
    fn get_unit_commands(
        &mut self,
        unit_id: i32,
        max_commands: u32,
    ) -> Result<Vec<api::units_commands::CommandFfi>, api::units_commands::SpringError> {
        let mut commands: *const ffi::SpringTypedCommand = core::ptr::null();
        let mut command_count = 0usize;
        let mut params: *const f32 = core::ptr::null();
        let mut param_count = 0usize;
        let code = unsafe {
            (self.shims().units_commands_get_unit_commands)(
                self.native,
                unit_id,
                max_commands,
                &mut commands,
                &mut command_count,
                &mut params,
                &mut param_count,
            )
        };
        if code != 0 {
            return Err(err!(api::units_commands, code));
        }
        let commands = unsafe { borrowed_slice(commands, command_count) };
        let params = unsafe { borrowed_slice(params, param_count) };
        Ok(commands
            .iter()
            .map(|command| {
                let start = command.param_offset as usize;
                let end = start.saturating_add(command.param_count as usize);
                api::units_commands::CommandFfi {
                    cmd_id: command.cmd_id,
                    options: command.options,
                    tag: command.tag,
                    ai_command_id: command.ai_command_id,
                    time_out: command.time_out,
                    params: params.get(start..end).unwrap_or(&[]).to_vec(),
                }
            })
            .collect())
    }
}

impl api::units_info::Host for HostState {
    fn get_unit_def_id(&mut self, unit_id: i32) -> Result<i32, api::units_info::SpringError> {
        let mut out = 0i32;
        let code = unsafe {
            (self.shims().units_info_get_unit_def_id)(self.native, unit_id, &mut out)
        };
        check!(api::units_info, code, out)
    }

    fn get_unit_health(
        &mut self,
        unit_id: i32,
    ) -> Result<api::units_info::UnitHealth, api::units_info::SpringError> {
        let mut out = [0.0f32; 5];
        let code = unsafe {
            (self.shims().units_info_get_unit_health)(self.native, unit_id, out.as_mut_ptr())
        };
        check!(
            api::units_info,
            code,
            api::units_info::UnitHealth {
                health: out[0],
                max_health: out[1],
                paralyze_damage: out[2],
                capture_progress: out[3],
                build_progress: out[4],
            }
        )
    }

    fn get_unit_position(
        &mut self,
        unit_id: i32,
        options: api::units_info::GetUnitPositionOptions,
    ) -> Result<api::units_info::Float3, api::units_info::SpringError> {
        let mut out = [0.0f32; 3];
        let code = unsafe {
            (self.shims().units_info_get_unit_position)(
                self.native,
                unit_id,
                options.mid_pos,
                options.aim_pos,
                out.as_mut_ptr(),
            )
        };
        check!(
            api::units_info,
            code,
            api::units_info::Float3 {
                x: out[0],
                y: out[1],
                z: out[2],
            }
        )
    }
}

impl api::units_query::Host for HostState {
    fn get_team_units(&mut self, team_id: i32) -> Result<Vec<i32>, api::units_query::SpringError> {
        let mut pointer: *const i32 = core::ptr::null();
        let mut len = 0usize;
        let code = unsafe {
            (self.shims().units_query_get_team_units)(
                self.native,
                team_id,
                &mut pointer,
                &mut len,
            )
        };
        if code != 0 {
            return Err(err!(api::units_query, code));
        }
        Ok(unsafe { borrowed_slice(pointer, len) }.to_vec())
    }

    fn get_units_in_cylinder(
        &mut self,
        x: f32,
        z: f32,
        radius: f32,
        allegiance: i32,
    ) -> Result<Vec<i32>, api::units_query::SpringError> {
        let mut pointer: *const i32 = core::ptr::null();
        let mut len = 0usize;
        let code = unsafe {
            (self.shims().units_query_get_units_in_cylinder)(
                self.native,
                x,
                z,
                radius,
                allegiance,
                &mut pointer,
                &mut len,
            )
        };
        if code != 0 {
            return Err(err!(api::units_query, code));
        }
        Ok(unsafe { borrowed_slice(pointer, len) }.to_vec())
    }
}
