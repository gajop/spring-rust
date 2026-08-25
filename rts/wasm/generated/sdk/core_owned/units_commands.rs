    pub mod units_commands {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct BuildQueueEntry {
            pub unit_def_id: i32,
            pub num_ordered: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CommandDescription {
            pub cmd_id: i32,
            pub action: String,
            pub type_: i32,
            pub name: String,
            pub tooltip: String,
            pub texture: String,
            pub cursor: String,
            pub queueing: bool,
            pub hidden: bool,
            pub disabled: bool,
            pub show_unique: bool,
            pub only_texture: bool,
            pub params: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CommandFFI {
            pub cmd_id: i32,
            pub options: u8,
            pub tag: i32,
            pub ai_command_id: i32,
            pub time_out: f32,
            pub params: Vec<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FactoryQueueInfo {
            pub total_count: u32,
            pub current_count: u32,
            pub unit_def_i_ds: Vec<i32>,
            pub counts: Vec<u32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FindUnitCmdDescQuery {
            pub unit_id: i32,
            pub cmd_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FindUnitCmdDescResult {
            pub cmd_index: i32,
            pub found: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCommandParamsQuery {
            pub command: CommandFFI,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCommandParamsResult {
            pub params: Vec<f32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetCommandQueueQuery {
            pub unit_id: i32,
            pub max_commands: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCommandQueueResult {
            pub commands: Vec<CommandFFI>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFactoryBuggerOffQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFactoryBuggerOffResult {
            pub perform: bool,
            pub offset: f32,
            pub radius: f32,
            pub rel_heading: i32,
            pub spherical: bool,
            pub forced: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFactoryCommandCountQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFactoryCommandCountResult {
            pub count: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFactoryCommandsQuery {
            pub unit_id: i32,
            pub max_commands: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFactoryCommandsResult {
            pub commands: Vec<CommandFFI>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFactoryCountsQuery {
            pub unit_id: i32,
            pub count: i32,
            pub add_cmds: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFactoryCountsResult {
            pub info: FactoryQueueInfo,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFullBuildQueueQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFullBuildQueueResult {
            pub entries: Vec<BuildQueueEntry>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetRealBuildQueueQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetRealBuildQueueResult {
            pub unit_def_i_ds: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitCmdDescsQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitCmdDescsResult {
            pub cmd_descs: Vec<CommandDescription>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitCommandCountQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitCommandCountResult {
            pub count: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitCommandsQuery {
            pub unit_id: i32,
            pub max_commands: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitCommandsResult {
            pub commands: Vec<CommandFFI>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitCurrentCommandQuery {
            pub unit_id: i32,
            pub cmd_index: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitCurrentCommandResult {
            pub command: Option<CommandFFI>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GiveOrderArrayToUnitMapQuery {
            pub unit_i_ds: Vec<i32>,
            pub commands: Vec<CommandFFI>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GiveOrderArrayToUnitMapResult {
            pub units_ordered: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GiveOrderQuery {
            pub cmd_id: i32,
            pub params: Vec<f32>,
            pub options: u32,
            pub timeout: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GiveOrderResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GiveOrderToUnitMapQuery {
            pub unit_i_ds: Vec<i32>,
            pub cmd_id: i32,
            pub params: Vec<f32>,
            pub options: u32,
            pub timeout: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GiveOrderToUnitMapResult {
            pub units_ordered: i32,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_real_build_queue {
            #[link(wasm_import_module = "spring:units-commands")]
            extern "C" {
                #[link_name = "get-real-build-queue"]
                pub fn call(punit_id: i32, output: i32) -> i32;
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FindUnitCmdDescValue {
            pub cmd_index: i32,
            pub found: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFactoryBuggerOffValue {
            pub perform: bool,
            pub offset: f32,
            pub radius: f32,
            pub rel_heading: i32,
            pub spherical: bool,
            pub forced: bool,
        }

        #[inline]
        pub fn find_unit_cmd_desc(unit_id: i32, cmd_id: i32) -> Result<FindUnitCmdDescValue> {
            let value = crate::generated::units_commands::find_unit_cmd_desc(unit_id, cmd_id)?;
            Ok(FindUnitCmdDescValue {
                cmd_index: value.0,
                found: value.1
            })
        }

        #[inline]
        pub fn get_command_params(command: &CommandFFI) -> Result<Vec<f32>> {
            let __blob0 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&command.cmd_id.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(command.options as u32).to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&command.tag.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&command.ai_command_id.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&command.time_out.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(command.params.len() as u32).to_le_bytes()); for __item in command.params.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.to_bits().to_le_bytes()); } while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::units_commands::get_command_params(&__blob0, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required * 4);
                        let mut __result = Vec::<f32>::with_capacity(required);
                        let mut __cursor = 0usize;
                        for _ in 0..required {
                            __result.push(crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?);
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required * 4, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn get_command_queue(unit_id: i32, max_commands: u32) -> Result<Vec<CommandFFI>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::units_commands::get_command_queue(unit_id, max_commands as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(CommandFFI { cmd_id: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, options: crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as u8, tag: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, ai_command_id: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, time_out: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, params: { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items } }); } __items };
                        if !crate::generated::__core_wire::finish(&__output, &mut __cursor, 8) {
                            return Err(crate::ApiError::new(crate::ErrorCode::Internal as i32));
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn get_factory_bugger_off(unit_id: i32) -> Result<GetFactoryBuggerOffValue> {
            let value = crate::generated::units_commands::get_factory_bugger_off(unit_id)?;
            Ok(GetFactoryBuggerOffValue {
                perform: value.0,
                offset: value.1,
                radius: value.2,
                rel_heading: value.3,
                spherical: value.4,
                forced: value.5
            })
        }

        #[inline]
        pub fn get_factory_command_count(unit_id: i32) -> Result<u32> {
            let value = crate::generated::units_commands::get_factory_command_count(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_factory_commands(unit_id: i32, max_commands: u32) -> Result<Vec<CommandFFI>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::units_commands::get_factory_commands(unit_id, max_commands as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(CommandFFI { cmd_id: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, options: crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as u8, tag: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, ai_command_id: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, time_out: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, params: { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items } }); } __items };
                        if !crate::generated::__core_wire::finish(&__output, &mut __cursor, 8) {
                            return Err(crate::ApiError::new(crate::ErrorCode::Internal as i32));
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn get_factory_counts(unit_id: i32, count: i32, add_cmds: bool) -> Result<FactoryQueueInfo> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::units_commands::get_factory_counts(unit_id, count, add_cmds as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = FactoryQueueInfo { total_count: crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, current_count: crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, unit_def_i_ds: { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items }, counts: { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items } };
                        if !crate::generated::__core_wire::finish(&__output, &mut __cursor, 8) {
                            return Err(crate::ApiError::new(crate::ErrorCode::Internal as i32));
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_full_build_queue {
            #[link(wasm_import_module = "spring:units-commands")]
            unsafe extern "C" {
                #[link_name = "get-full-build-queue"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-commands.get-full-build-queue."]
        #[doc(hidden)]
        #[inline]
        pub fn get_full_build_queue(p0: i32, p1: i32) -> i32 {
            __core_owned_get_full_build_queue::call(p0, p1)
        }

        #[inline]
        pub fn get_real_build_queue(unit_id: i32) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_real_build_queue::call(unit_id, descriptor.as_mut_ptr() as usize as u32 as i32) };
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(output);
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, Default::default());
                    descriptor[0] = output.as_mut_ptr() as usize as u32;
                    descriptor[1] = output.len() as u32;
                    descriptor[2] = 0;
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = (unit_id);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_unit_cmd_descs(unit_id: i32) -> Result<Vec<CommandDescription>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::units_commands::get_unit_cmd_descs(unit_id, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(CommandDescription { cmd_id: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, action: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, type_: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, tooltip: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, texture: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, cursor: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, queueing: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, hidden: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, disabled: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, show_unique: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, only_texture: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, params: { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items } }); } __items };
                        if !crate::generated::__core_wire::finish(&__output, &mut __cursor, 8) {
                            return Err(crate::ApiError::new(crate::ErrorCode::Internal as i32));
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn get_unit_command_count(unit_id: i32) -> Result<u32> {
            let value = crate::generated::units_commands::get_unit_command_count(unit_id)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_commands {
            #[link(wasm_import_module = "spring:units-commands")]
            unsafe extern "C" {
                #[link_name = "get-unit-commands"]
                pub safe fn call(p0: i32, p1: i32, p2: i32, p3: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-commands.get-unit-commands."]
        #[doc(hidden)]
        #[inline]
        pub fn get_unit_commands(p0: i32, p1: i32, p2: i32, p3: i32) -> i64 {
            __core_owned_get_unit_commands::call(p0, p1, p2, p3)
        }

        #[inline]
        pub fn get_unit_current_command(unit_id: i32, cmd_index: i32) -> Result<Option<CommandFFI>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::units_commands::get_unit_current_command(unit_id, cmd_index, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = { if crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? { Some(CommandFFI { cmd_id: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, options: crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as u8, tag: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, ai_command_id: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, time_out: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, params: { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items } }) } else { None } };
                        if !crate::generated::__core_wire::finish(&__output, &mut __cursor, 8) {
                            return Err(crate::ApiError::new(crate::ErrorCode::Internal as i32));
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_give_order {
            #[link(wasm_import_module = "spring:units-commands")]
            unsafe extern "C" {
                #[link_name = "give-order"]
                pub safe fn call(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-commands.give-order."]
        #[doc(hidden)]
        #[inline]
        pub fn give_order(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32) -> i64 {
            __core_owned_give_order::call(p0, p1, p2, p3, p4)
        }

        #[inline]
        pub fn give_order_array_to_unit_map(unit_i_ds: &[i32], commands: &[CommandFFI]) -> Result<i32> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(unit_i_ds.len() as u32).to_le_bytes()); for __item in unit_i_ds.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.to_le_bytes());} __b };
            let __blob1 = { let mut __b = Vec::new(); __b.extend_from_slice(&(commands.len() as u32).to_le_bytes()); for __item in commands.iter() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.cmd_id.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item.options as u32).to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.tag.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.ai_command_id.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.time_out.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item.params.len() as u32).to_le_bytes()); for __item in __item.params.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.to_bits().to_le_bytes()); }} __b };
            crate::generated::dynamic_input::units_commands::give_order_array_to_unit_map(&__blob0, &__blob1)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_give_order_to_unit_map {
            #[link(wasm_import_module = "spring:units-commands")]
            unsafe extern "C" {
                #[link_name = "give-order-to-unit-map"]
                pub safe fn call(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-commands.give-order-to-unit-map."]
        #[doc(hidden)]
        #[inline]
        pub fn give_order_to_unit_map(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32) -> i64 {
            __core_owned_give_order_to_unit_map::call(p0, p1, p2, p3, p4, p5, p6)
        }

    }

