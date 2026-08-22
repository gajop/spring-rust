    pub mod messages {
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
        pub struct ConsoleEntry {
            pub text: String,
            pub priority: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DefRef {
            pub name: String,
            pub id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct EchoQuery {
            pub message: String,
            pub rest: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct EchoResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Error {
            pub code: i32,
            pub message: String,
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
        pub struct GetConsoleBufferQuery {
            pub max_lines: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetConsoleBufferResult {
            pub entries: Vec<ConsoleEntry>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCurrentTooltipQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCurrentTooltipResult {
            pub tooltip: String,
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
        pub struct IsUserWritingQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsUserWritingResult {
            pub writing: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct LogQuery {
            pub section: String,
            pub level: i32,
            pub message: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct LogResult {
            pub success: bool,
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
        pub struct SendAllyChatQuery {
            pub message: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendAllyChatResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendCommandsQuery {
            pub command: String,
            pub rest: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendCommandsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendLuaGaiaQuery {
            pub message: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendLuaGaiaResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendLuaMenuMsgQuery {
            pub message: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendLuaMenuMsgResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendLuaRulesQuery {
            pub message: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendLuaRulesResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendLuaUIQuery {
            pub message: String,
            pub mode: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendLuaUIResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendMessageQuery {
            pub message: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendMessageResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendMessageToAllyTeamQuery {
            pub ally_team_id: i32,
            pub message: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendMessageToAllyTeamResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendMessageToPlayerQuery {
            pub player_id: i32,
            pub message: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendMessageToPlayerResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendMessageToSpectatorsQuery {
            pub message: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendMessageToSpectatorsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendMessageToTeamQuery {
            pub team_id: i32,
            pub message: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendMessageToTeamResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendPrivateChatQuery {
            pub message: String,
            pub player_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendPrivateChatResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendPublicChatQuery {
            pub message: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendPublicChatResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendSkirmishAIMessageQuery {
            pub ai_id: i32,
            pub message: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendSkirmishAIMessageResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendSpectatorChatQuery {
            pub message: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendSpectatorChatResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendToUnsyncedQuery {
            pub message: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendToUnsyncedResult {
            pub success: bool,
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
        mod __core_variable_output_get_current_tooltip {
            #[link(wasm_import_module = "spring:messages")]
            extern "C" {
                #[link_name = "get-current-tooltip"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[inline]
        pub fn echo(message: &str, rest: &str) -> Result<bool> {
            crate::messages::echo(message, rest)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_console_buffer {
            #[link(wasm_import_module = "spring:messages")]
            extern "C" {
                #[link_name = "get-console-buffer"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:messages.get-console-buffer."]
        #[inline]
        pub unsafe fn get_console_buffer(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_console_buffer::call(p0, p1) }
        }

        #[inline]
        pub fn get_current_tooltip(unused: u8) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_current_tooltip::call(unused as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(super::decode_core_string(output));
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, 0);
                    descriptor[0] = output.as_mut_ptr() as usize as u32;
                    descriptor[1] = output.len() as u32;
                    descriptor[2] = 0;
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = (unused as i32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn is_user_writing(unused: u8) -> Result<bool> {
            let value = crate::generated::messages::is_user_writing(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn log(section: &str, level: i32, message: &str) -> Result<bool> {
            crate::messages::log(section, level, message)
        }

        #[inline]
        pub fn send_ally_chat(message: &str) -> Result<bool> {
            crate::messages::send_ally_chat(message)
        }

        #[inline]
        pub fn send_commands(command: &str, rest: &str) -> Result<bool> {
            crate::messages::send_commands(command, rest)
        }

        #[inline]
        pub fn send_lua_gaia_msg(message: &str) -> Result<bool> {
            crate::messages::send_lua_gaia_msg(message)
        }

        #[inline]
        pub fn send_lua_menu_msg(message: &str) -> Result<bool> {
            crate::messages::send_lua_menu_msg(message)
        }

        #[inline]
        pub fn send_lua_rules_msg(message: &str) -> Result<bool> {
            crate::messages::send_lua_rules_msg(message)
        }

        #[inline]
        pub fn send_lua_ui_msg(message: &str, mode: &str) -> Result<bool> {
            crate::messages::send_lua_ui_msg(message, mode)
        }

        #[inline]
        pub fn send_message(message: &str) -> Result<bool> {
            crate::messages::send_message(message)
        }

        #[inline]
        pub fn send_message_to_ally_team(ally_team_id: i32, message: &str) -> Result<bool> {
            crate::messages::send_message_to_ally_team(ally_team_id, message)
        }

        #[inline]
        pub fn send_message_to_player(player_id: i32, message: &str) -> Result<bool> {
            crate::messages::send_message_to_player(player_id, message)
        }

        #[inline]
        pub fn send_message_to_spectators(message: &str) -> Result<bool> {
            crate::messages::send_message_to_spectators(message)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_send_message_to_team {
            #[link(wasm_import_module = "spring:messages")]
            extern "C" {
                #[link_name = "send-message-to-team"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:messages.send-message-to-team."]
        #[inline]
        pub unsafe fn send_message_to_team(p0: i32, p1: i32, p2: i32) -> i64 {
            unsafe { __core_owned_send_message_to_team::call(p0, p1, p2) }
        }

        #[inline]
        pub fn send_private_chat(message: &str, player_id: i32) -> Result<bool> {
            crate::messages::send_private_chat(message, player_id)
        }

        #[inline]
        pub fn send_public_chat(message: &str) -> Result<bool> {
            crate::messages::send_public_chat(message)
        }

        #[inline]
        pub fn send_skirmish_ai_message(ai_id: i32, message: &str) -> Result<bool> {
            crate::messages::send_skirmish_ai_message(ai_id, message)
        }

        #[inline]
        pub fn send_spectator_chat(message: &str) -> Result<bool> {
            crate::messages::send_spectator_chat(message)
        }

        #[inline]
        pub fn send_to_unsynced(message: &str) -> Result<bool> {
            crate::messages::send_to_unsynced(message)
        }

    }

