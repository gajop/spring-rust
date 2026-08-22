    pub mod units_commands {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum CommonErrorCode {
            ErrorAlreadyExists,
            ErrorBufferOverflow,
            ErrorInternal,
            ErrorInvalidArgument,
            ErrorInvalidId,
            ErrorInvalidState,
            ErrorNone,
            ErrorNotAvailable,
            ErrorNotFound,
            ErrorOperationFailed,
            ErrorOutOfBounds,
            ErrorPermissionDenied,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AtmosphereParams {
            pub fog_color: Option<Vec<f32>>,
            pub sky_color: Option<Vec<f32>>,
            pub sun_color: Option<Vec<f32>>,
            pub cloud_color: Option<Vec<f32>>,
            pub sky_axis_angle: Option<Vec<f32>>,
            pub fog_start: Option<f32>,
            pub fog_end: Option<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct BoolResult {
            pub value: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct BuildQueueEntry {
            pub unit_def_id: i32,
            pub num_ordered: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CollisionVolumeData {
            pub scale_x: f32,
            pub scale_y: f32,
            pub scale_z: f32,
            pub offset_x: f32,
            pub offset_y: f32,
            pub offset_z: f32,
            pub volume_type: i32,
            pub test_type: i32,
            pub primary_axis: i32,
            pub disabled: bool,
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
        pub struct DefRef {
            pub name: String,
            pub id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Error {
            pub code: i32,
            pub message: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FactoryQueueInfo {
            pub total_count: u32,
            pub current_count: u32,
            pub unit_def_i_ds: Vec<i32>,
            pub counts: Vec<u32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FindUnitCmdDescQuery {
            pub unit_id: i32,
            pub cmd_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FindUnitCmdDescResult {
            pub cmd_index: i32,
            pub found: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float2 {
            pub x: f32,
            pub y: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float2Result {
            pub value: Float2,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float3 {
            pub x: f32,
            pub y: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float3Array {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float3Result {
            pub value: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float4 {
            pub x: f32,
            pub y: f32,
            pub z: f32,
            pub w: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float4Result {
            pub value: Float4,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FloatArray {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FloatResult {
            pub value: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCommandParamsQuery {
            pub command: CommandFFI,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCommandParamsResult {
            pub params: Vec<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCommandQueueQuery {
            pub unit_id: i32,
            pub max_commands: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCommandQueueResult {
            pub commands: Vec<CommandFFI>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFactoryBuggerOffQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFactoryBuggerOffResult {
            pub perform: bool,
            pub offset: f32,
            pub radius: f32,
            pub rel_heading: i32,
            pub spherical: bool,
            pub forced: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFactoryCommandCountQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFactoryCommandCountResult {
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFactoryCommandsQuery {
            pub unit_id: i32,
            pub max_commands: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFactoryCommandsResult {
            pub commands: Vec<CommandFFI>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFactoryCountsQuery {
            pub unit_id: i32,
            pub count: i32,
            pub add_cmds: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFactoryCountsResult {
            pub info: FactoryQueueInfo,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFullBuildQueueQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFullBuildQueueResult {
            pub entries: Vec<BuildQueueEntry>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetRealBuildQueueQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetRealBuildQueueResult {
            pub unit_def_i_ds: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitCmdDescsQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitCmdDescsResult {
            pub cmd_descs: Vec<CommandDescription>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitCommandCountQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitCommandCountResult {
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitCommandsQuery {
            pub unit_id: i32,
            pub max_commands: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitCommandsResult {
            pub commands: Vec<CommandFFI>,
        }

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct GiveOrderToUnitMapResult {
            pub units_ordered: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Int2 {
            pub x: i32,
            pub y: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Int3 {
            pub x: i32,
            pub y: i32,
            pub z: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Int32Array {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Int32Result {
            pub value: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct MapRenderingParams {
            pub splat_tex_scales: Option<Vec<f32>>,
            pub splat_tex_mults: Option<Vec<f32>>,
            pub void_water: Option<bool>,
            pub void_ground: Option<bool>,
            pub splat_detail_normal_diffuse_alpha: Option<bool>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct NativeExplosionParams {
            pub damages: f32,
            pub weapon_def_id: i32,
            pub owner_id: i32,
            pub hit_unit_id: i32,
            pub hit_feature_id: i32,
            pub crater_area_of_effect: f32,
            pub damage_area_of_effect: f32,
            pub edge_effectiveness: f32,
            pub explosion_speed: f32,
            pub gfx_mod: f32,
            pub impact_only: bool,
            pub ignore_owner: bool,
            pub damage_ground: bool,
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct NativeProjectileParams {
            pub pos: Float3,
            pub speed: Float3,
            pub spread: Float3,
            pub end: Float3,
            pub owner: i32,
            pub team: i32,
            pub weapon_num: i32,
            pub ttl: f32,
            pub gravity: f32,
            pub tracking: f32,
            pub max_range: f32,
            pub up_time: f32,
            pub start_alpha: f32,
            pub end_alpha: f32,
            pub model: String,
            pub ceg_tag: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct NumberOrBool {
            pub number: f32,
            pub boolean: bool,
            pub use_boolean: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ProjectileTargetRef {
            pub target_id: i32,
            pub target_type: i32,
            pub pos: Float3,
            pub is_ground_target: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ResourcePack {
            pub metal: f32,
            pub energy: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RgbColor {
            pub r: f32,
            pub g: f32,
            pub b: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SoundEffectParams {
            pub preset: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct StringArray {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct StringResult {
            pub value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SunLightingParams {
            pub ground_ambient_color: Option<Vec<f32>>,
            pub ground_diffuse_color: Option<Vec<f32>>,
            pub ground_specular_color: Option<Vec<f32>>,
            pub model_ambient_color: Option<Vec<f32>>,
            pub model_diffuse_color: Option<Vec<f32>>,
            pub model_specular_color: Option<Vec<f32>>,
            pub specular_exponent: Option<f32>,
            pub ground_shadow_density: Option<f32>,
            pub model_shadow_density: Option<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UInt32Array {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UInt32Result {
            pub value: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitCostOverrides {
            pub build_time: f32,
            pub metal_cost: f32,
            pub energy_cost: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitHealthValue {
            pub health: f32,
            pub capture: f32,
            pub paralyze: f32,
            pub build: f32,
            pub use_amounts: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitTargetRef {
            pub target_id: i32,
            pub pos: Float3,
            pub is_ground_target: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct WaterParams {
            pub absorb: Option<Vec<f32>>,
            pub base_color: Option<Vec<f32>>,
            pub min_color: Option<Vec<f32>>,
            pub surface_color: Option<Vec<f32>>,
            pub diffuse_color: Option<Vec<f32>>,
            pub specular_color: Option<Vec<f32>>,
            pub plane_color: Option<Vec<f32>>,
            pub repeat_x: Option<f32>,
            pub repeat_y: Option<f32>,
            pub surface_alpha: Option<f32>,
            pub ambient_factor: Option<f32>,
            pub diffuse_factor: Option<f32>,
            pub specular_factor: Option<f32>,
            pub specular_power: Option<f32>,
            pub fresnel_min: Option<f32>,
            pub fresnel_max: Option<f32>,
            pub fresnel_power: Option<f32>,
            pub reflection_distortion: Option<f32>,
            pub blur_base: Option<f32>,
            pub blur_exponent: Option<f32>,
            pub perlin_start_freq: Option<f32>,
            pub perlin_lacunarity: Option<f32>,
            pub perlin_amplitude: Option<f32>,
            pub wind_speed: Option<f32>,
            pub wave_offset_factor: Option<f32>,
            pub wave_length: Option<f32>,
            pub wave_foam_distortion: Option<f32>,
            pub wave_foam_intensity: Option<f32>,
            pub caustics_resolution: Option<f32>,
            pub caustics_strength: Option<f32>,
            pub num_tiles: Option<f32>,
            pub shore_waves: Option<bool>,
            pub force_rendering: Option<bool>,
            pub has_water_plane: Option<bool>,
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_real_build_queue {
            #[link(wasm_import_module = "spring:units-commands")]
            extern "C" {
                #[link_name = "get-real-build-queue"]
                pub fn call(punit_id: i32, output: i32) -> i32;
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FindUnitCmdDescValue {
            pub cmd_index: i32,
            pub found: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
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

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_command_params {
            #[link(wasm_import_module = "spring:units-commands")]
            extern "C" {
                #[link_name = "get-command-params"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-commands.get-command-params."]
        #[inline]
        pub unsafe fn get_command_params(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_command_params::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_command_queue {
            #[link(wasm_import_module = "spring:units-commands")]
            extern "C" {
                #[link_name = "get-command-queue"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-commands.get-command-queue."]
        #[inline]
        pub unsafe fn get_command_queue(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_get_command_queue::call(p0, p1, p2) }
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

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_factory_commands {
            #[link(wasm_import_module = "spring:units-commands")]
            extern "C" {
                #[link_name = "get-factory-commands"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-commands.get-factory-commands."]
        #[inline]
        pub unsafe fn get_factory_commands(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_get_factory_commands::call(p0, p1, p2) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_factory_counts {
            #[link(wasm_import_module = "spring:units-commands")]
            extern "C" {
                #[link_name = "get-factory-counts"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-commands.get-factory-counts."]
        #[inline]
        pub unsafe fn get_factory_counts(p0: i32, p1: i32, p2: i32, p3: i32) -> i32 {
            unsafe { __core_owned_get_factory_counts::call(p0, p1, p2, p3) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_full_build_queue {
            #[link(wasm_import_module = "spring:units-commands")]
            extern "C" {
                #[link_name = "get-full-build-queue"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-commands.get-full-build-queue."]
        #[inline]
        pub unsafe fn get_full_build_queue(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_full_build_queue::call(p0, p1) }
        }

        #[inline]
        pub fn get_real_build_queue(unit_id: i32) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_real_build_queue::call(unit_id as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (unit_id as i32);
                Err(unreachable!())
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_cmd_descs {
            #[link(wasm_import_module = "spring:units-commands")]
            extern "C" {
                #[link_name = "get-unit-cmd-descs"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-commands.get-unit-cmd-descs."]
        #[inline]
        pub unsafe fn get_unit_cmd_descs(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_unit_cmd_descs::call(p0, p1) }
        }

        #[inline]
        pub fn get_unit_command_count(unit_id: i32) -> Result<u32> {
            let value = crate::generated::units_commands::get_unit_command_count(unit_id)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_commands {
            #[link(wasm_import_module = "spring:units-commands")]
            extern "C" {
                #[link_name = "get-unit-commands"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-commands.get-unit-commands."]
        #[inline]
        pub unsafe fn get_unit_commands(p0: i32, p1: i32, p2: i32, p3: i32) -> i64 {
            unsafe { __core_owned_get_unit_commands::call(p0, p1, p2, p3) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_current_command {
            #[link(wasm_import_module = "spring:units-commands")]
            extern "C" {
                #[link_name = "get-unit-current-command"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-commands.get-unit-current-command."]
        #[inline]
        pub unsafe fn get_unit_current_command(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_get_unit_current_command::call(p0, p1, p2) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_give_order {
            #[link(wasm_import_module = "spring:units-commands")]
            extern "C" {
                #[link_name = "give-order"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-commands.give-order."]
        #[inline]
        pub unsafe fn give_order(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32) -> i64 {
            unsafe { __core_owned_give_order::call(p0, p1, p2, p3, p4) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_give_order_array_to_unit_map {
            #[link(wasm_import_module = "spring:units-commands")]
            extern "C" {
                #[link_name = "give-order-array-to-unit-map"]
                pub fn call(p0: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-commands.give-order-array-to-unit-map."]
        #[inline]
        pub unsafe fn give_order_array_to_unit_map(p0: i32) -> i64 {
            unsafe { __core_owned_give_order_array_to_unit_map::call(p0) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_give_order_to_unit_map {
            #[link(wasm_import_module = "spring:units-commands")]
            extern "C" {
                #[link_name = "give-order-to-unit-map"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-commands.give-order-to-unit-map."]
        #[inline]
        pub unsafe fn give_order_to_unit_map(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32) -> i64 {
            unsafe { __core_owned_give_order_to_unit_map::call(p0, p1, p2, p3, p4, p5, p6) }
        }

    }

