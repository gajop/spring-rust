    pub mod system_control {
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
        pub struct CallAsTeamQuery {
            pub team_id: i32,
            pub callback: u32,
            pub user_data: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CallAsTeamResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ClearWatchDogTimerQuery {
            pub thread_name: String,
            pub keep_stopped: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ClearWatchDogTimerResult {
            pub success: bool,
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
        pub struct GarbageCollectCtrlQuery {
            pub iters_per_batch: i32,
            pub num_steps_per_iter: i32,
            pub min_steps_per_iter: i32,
            pub max_steps_per_iter: i32,
            pub min_loop_run_time: f32,
            pub max_loop_run_time: f32,
            pub base_run_time_mult: f32,
            pub base_mem_load_mult: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GarbageCollectCtrlResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameNameQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameNameResult {
            pub name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameStateQuery {
            pub max_latency: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameStateResult {
            pub done_loading: bool,
            pub is_saved_game: bool,
            pub is_client_paused: bool,
            pub is_sim_lagging: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGatherModeQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGatherModeResult {
            pub mode: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMenuNameQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMenuNameResult {
            pub name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetReplayFilePathQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetReplayFilePathResult {
            pub path: String,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetReplayLengthQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetReplayLengthResult {
            pub seconds: f32,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetReplayRecordingFilePathQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetReplayRecordingFilePathResult {
            pub path: String,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetVideoCapturingModeQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetVideoCapturingModeResult {
            pub allow_record: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetWindowDisplayModeQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetWindowDisplayModeResult {
            pub width: i32,
            pub height: i32,
            pub bpp: i32,
            pub refresh: i32,
            pub format_name: String,
            pub success: bool,
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
        pub struct IsReplayQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsReplayResult {
            pub is_replay: bool,
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
        pub struct PingQuery {
            pub tag: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PingResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ProjectileTargetRef {
            pub target_id: i32,
            pub target_type: i32,
            pub pos: Float3,
            pub is_ground_target: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct QuitQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct QuitResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ReloadQuery {
            pub start_script: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ReloadResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RequestStartPositionQuery {
            pub pos: Float3,
            pub ready: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RequestStartPositionResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ResourcePack {
            pub metal: f32,
            pub energy: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RestartQuery {
            pub cmd_args: String,
            pub start_script: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RestartResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RgbColor {
            pub r: f32,
            pub g: f32,
            pub b: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetShareLevelQuery {
            pub resource: String,
            pub level: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetShareLevelResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ShareResourcesQuery {
            pub team_id: i32,
            pub resource: String,
            pub amount: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ShareResourcesResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SoundEffectParams {
            pub preset: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct StartQuery {
            pub cmd_args: String,
            pub start_script: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct StartResult {
            pub success: bool,
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct YieldQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct YieldResult {
            pub keep_yielding: bool,
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_game_name {
            #[link(wasm_import_module = "spring:system-control")]
            extern "C" {
                #[link_name = "get-game-name"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_menu_name {
            #[link(wasm_import_module = "spring:system-control")]
            extern "C" {
                #[link_name = "get-menu-name"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameStateValue {
            pub done_loading: bool,
            pub is_saved_game: bool,
            pub is_client_paused: bool,
            pub is_sim_lagging: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetReplayFilePathValue {
            pub path: String,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetReplayLengthValue {
            pub seconds: f32,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetReplayRecordingFilePathValue {
            pub path: String,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetWindowDisplayModeValue {
            pub width: i32,
            pub height: i32,
            pub bpp: i32,
            pub refresh: i32,
            pub format_name: String,
            pub success: bool,
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_call_as_team {
            #[link(wasm_import_module = "spring:system-control")]
            extern "C" {
                #[link_name = "call-as-team"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:system-control.call-as-team."]
        #[inline]
        pub unsafe fn call_as_team(p0: i32, p1: i32, p2: i32) -> i64 {
            unsafe { __core_owned_call_as_team::call(p0, p1, p2) }
        }

        #[inline]
        pub fn clear_watch_dog_timer(thread_name: &str, keep_stopped: bool) -> Result<bool> {
            let mut thread_name_bytes = thread_name.as_bytes().to_vec();
            if thread_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            thread_name_bytes.push(0);
            let thread_name_cstr = core::ffi::CStr::from_bytes_with_nul(&thread_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::system_control::clear_watch_dog_timer(&thread_name_cstr, keep_stopped)
        }

        #[inline]
        pub fn garbage_collect_ctrl(iters_per_batch: i32, num_steps_per_iter: i32, min_steps_per_iter: i32, max_steps_per_iter: i32, min_loop_run_time: f32, max_loop_run_time: f32, base_run_time_mult: f32, base_mem_load_mult: f32) -> Result<bool> {
            let value = crate::generated::system_control::garbage_collect_ctrl(iters_per_batch, num_steps_per_iter, min_steps_per_iter, max_steps_per_iter, min_loop_run_time, max_loop_run_time, base_run_time_mult, base_mem_load_mult)?;
            Ok(value)
        }

        #[inline]
        pub fn get_game_name(unused: u8) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_game_name::call(unused as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
        pub fn get_game_state(max_latency: f32) -> Result<GetGameStateValue> {
            let value = crate::generated::system_control::get_game_state(max_latency)?;
            Ok(GetGameStateValue {
                done_loading: value.0,
                is_saved_game: value.1,
                is_client_paused: value.2,
                is_sim_lagging: value.3
            })
        }

        #[inline]
        pub fn get_gather_mode(unused: u8) -> Result<i32> {
            let value = crate::generated::system_control::get_gather_mode(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_menu_name(unused: u8) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_menu_name::call(unused as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_replay_file_path {
            #[link(wasm_import_module = "spring:system-control")]
            extern "C" {
                #[link_name = "get-replay-file-path"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:system-control.get-replay-file-path."]
        #[inline]
        pub unsafe fn get_replay_file_path(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_replay_file_path::call(p0, p1) }
        }

        #[inline]
        pub fn get_replay_length(unused: u8) -> Result<GetReplayLengthValue> {
            let value = crate::generated::system_control::get_replay_length(unused)?;
            Ok(GetReplayLengthValue {
                seconds: value.0,
                success: value.1
            })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_replay_recording_file_path {
            #[link(wasm_import_module = "spring:system-control")]
            extern "C" {
                #[link_name = "get-replay-recording-file-path"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:system-control.get-replay-recording-file-path."]
        #[inline]
        pub unsafe fn get_replay_recording_file_path(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_replay_recording_file_path::call(p0, p1) }
        }

        #[inline]
        pub fn get_video_capturing_mode(unused: u8) -> Result<bool> {
            let value = crate::generated::system_control::get_video_capturing_mode(unused)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_window_display_mode {
            #[link(wasm_import_module = "spring:system-control")]
            extern "C" {
                #[link_name = "get-window-display-mode"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:system-control.get-window-display-mode."]
        #[inline]
        pub unsafe fn get_window_display_mode(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_window_display_mode::call(p0, p1) }
        }

        #[inline]
        pub fn is_replay(unused: u8) -> Result<bool> {
            let value = crate::generated::system_control::is_replay(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn ping(tag: u32) -> Result<bool> {
            let value = crate::generated::system_control::ping(tag)?;
            Ok(value)
        }

        #[inline]
        pub fn quit(unused: u8) -> Result<bool> {
            let value = crate::generated::system_control::quit(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn reload(start_script: &str) -> Result<bool> {
            let mut start_script_bytes = start_script.as_bytes().to_vec();
            if start_script_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            start_script_bytes.push(0);
            let start_script_cstr = core::ffi::CStr::from_bytes_with_nul(&start_script_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::system_control::reload(&start_script_cstr)
        }

        #[inline]
        pub fn request_start_position(pos: Float3, ready: bool) -> Result<bool> {
            let value = crate::generated::system_control::request_start_position(crate::generated::system_control::Float3 { x: pos.x, y: pos.y, z: pos.z }, ready)?;
            Ok(value)
        }

        #[inline]
        pub fn restart(cmd_args: &str, start_script: &str) -> Result<bool> {
            let mut cmd_args_bytes = cmd_args.as_bytes().to_vec();
            if cmd_args_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            cmd_args_bytes.push(0);
            let cmd_args_cstr = core::ffi::CStr::from_bytes_with_nul(&cmd_args_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            let mut start_script_bytes = start_script.as_bytes().to_vec();
            if start_script_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            start_script_bytes.push(0);
            let start_script_cstr = core::ffi::CStr::from_bytes_with_nul(&start_script_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::system_control::restart(&cmd_args_cstr, &start_script_cstr)
        }

        #[inline]
        pub fn set_share_level(resource: &str, level: f32) -> Result<bool> {
            let mut resource_bytes = resource.as_bytes().to_vec();
            if resource_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            resource_bytes.push(0);
            let resource_cstr = core::ffi::CStr::from_bytes_with_nul(&resource_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::system_control::set_share_level(&resource_cstr, level)
        }

        #[inline]
        pub fn share_resources(team_id: i32, resource: &str, amount: f32) -> Result<bool> {
            let mut resource_bytes = resource.as_bytes().to_vec();
            if resource_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            resource_bytes.push(0);
            let resource_cstr = core::ffi::CStr::from_bytes_with_nul(&resource_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::system_control::share_resources(team_id, &resource_cstr, amount)
        }

        #[inline]
        pub fn start(cmd_args: &str, start_script: &str) -> Result<bool> {
            let mut cmd_args_bytes = cmd_args.as_bytes().to_vec();
            if cmd_args_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            cmd_args_bytes.push(0);
            let cmd_args_cstr = core::ffi::CStr::from_bytes_with_nul(&cmd_args_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            let mut start_script_bytes = start_script.as_bytes().to_vec();
            if start_script_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            start_script_bytes.push(0);
            let start_script_cstr = core::ffi::CStr::from_bytes_with_nul(&start_script_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::system_control::start(&cmd_args_cstr, &start_script_cstr)
        }

        #[inline]
        pub fn yield_(unused: u8) -> Result<bool> {
            let value = crate::generated::system_control::yield_(unused)?;
            Ok(value)
        }

    }

