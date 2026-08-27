    pub mod system_control {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, PartialEq)]
        pub struct CallAsTeamQuery {
            pub team_id: i32,
            pub callback: u32,
            pub user_data: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct CallAsTeamResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ClearWatchDogTimerQuery {
            pub thread_name: String,
            pub keep_stopped: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ClearWatchDogTimerResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GarbageCollectCtrlResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGameNameQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameNameResult {
            pub name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGameStateQuery {
            pub max_latency: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGameStateResult {
            pub done_loading: bool,
            pub is_saved_game: bool,
            pub is_client_paused: bool,
            pub is_sim_lagging: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGatherModeQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGatherModeResult {
            pub mode: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetMenuNameQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMenuNameResult {
            pub name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetReplayFilePathQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetReplayFilePathResult {
            pub path: String,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetReplayLengthQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetReplayLengthResult {
            pub seconds: f32,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetReplayRecordingFilePathQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetReplayRecordingFilePathResult {
            pub path: String,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetVideoCapturingModeQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetVideoCapturingModeResult {
            pub allow_record: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsReplayQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsReplayResult {
            pub is_replay: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct PingQuery {
            pub tag: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct PingResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct QuitQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct QuitResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ReloadQuery {
            pub start_script: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ReloadResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RequestStartPositionQuery {
            pub pos: Float3,
            pub ready: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RequestStartPositionResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RestartQuery {
            pub cmd_args: String,
            pub start_script: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RestartResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetShareLevelQuery {
            pub resource: String,
            pub level: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetShareLevelResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ShareResourcesQuery {
            pub team_id: i32,
            pub resource: String,
            pub amount: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ShareResourcesResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct StartQuery {
            pub cmd_args: String,
            pub start_script: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct StartResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct YieldQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct YieldResult {
            pub keep_yielding: bool,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_game_name {
            #[link(wasm_import_module = "spring:system-control")]
            unsafe extern "C" {
                #[link_name = "get-game-name"]
                pub safe fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_menu_name {
            #[link(wasm_import_module = "spring:system-control")]
            unsafe extern "C" {
                #[link_name = "get-menu-name"]
                pub safe fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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
            unsafe extern "C" {
                #[link_name = "call-as-team"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:system-control.call-as-team."]
        #[doc(hidden)]
        #[inline]
        pub fn call_as_team(p0: i32, p1: i32, p2: i32) -> i64 {
            __core_owned_call_as_team::call(p0, p1, p2)
        }

        #[inline]
        pub fn clear_watch_dog_timer(thread_name: &str, keep_stopped: bool) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(thread_name, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(thread_name)?),
            };
            crate::generated::borrowed::system_control::clear_watch_dog_timer(__core_string_0_buf.as_cstr(), keep_stopped)
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
                    let descriptor_ptr = crate::wasm_output_ptr(&mut descriptor)?;
                    let (output_ptr, output_capacity) = crate::wasm_mut_slice_parts(&mut output)?;
                    descriptor[0] = output_ptr as u32;
                    descriptor[1] = output_capacity as u32;
                    let status = __core_variable_output_get_game_name::call(unused as i32, descriptor_ptr);
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(super::decode_core_string(output));
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, 0);
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
                    let descriptor_ptr = crate::wasm_output_ptr(&mut descriptor)?;
                    let (output_ptr, output_capacity) = crate::wasm_mut_slice_parts(&mut output)?;
                    descriptor[0] = output_ptr as u32;
                    descriptor[1] = output_capacity as u32;
                    let status = __core_variable_output_get_menu_name::call(unused as i32, descriptor_ptr);
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(super::decode_core_string(output));
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, 0);
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
            unsafe extern "C" {
                #[link_name = "get-replay-file-path"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:system-control.get-replay-file-path."]
        #[doc(hidden)]
        #[inline]
        pub fn get_replay_file_path(p0: i32, p1: i32) -> i32 {
            __core_owned_get_replay_file_path::call(p0, p1)
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
            unsafe extern "C" {
                #[link_name = "get-replay-recording-file-path"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:system-control.get-replay-recording-file-path."]
        #[doc(hidden)]
        #[inline]
        pub fn get_replay_recording_file_path(p0: i32, p1: i32) -> i32 {
            __core_owned_get_replay_recording_file_path::call(p0, p1)
        }

        #[inline]
        pub fn get_video_capturing_mode(unused: u8) -> Result<bool> {
            let value = crate::generated::system_control::get_video_capturing_mode(unused)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_window_display_mode {
            #[link(wasm_import_module = "spring:system-control")]
            unsafe extern "C" {
                #[link_name = "get-window-display-mode"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:system-control.get-window-display-mode."]
        #[doc(hidden)]
        #[inline]
        pub fn get_window_display_mode(p0: i32, p1: i32) -> i32 {
            __core_owned_get_window_display_mode::call(p0, p1)
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
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(start_script, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(start_script)?),
            };
            crate::generated::borrowed::system_control::reload(__core_string_0_buf.as_cstr())
        }

        #[inline]
        pub fn request_start_position(pos: Float3, ready: bool) -> Result<bool> {
            let value = crate::generated::system_control::request_start_position(crate::generated::system_control::Float3 { x: pos.x, y: pos.y, z: pos.z }, ready)?;
            Ok(value)
        }

        #[inline]
        pub fn restart(cmd_args: &str, start_script: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(cmd_args, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(cmd_args)?),
            };
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(start_script, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(start_script)?),
            };
            crate::generated::borrowed::system_control::restart(__core_string_0_buf.as_cstr(), __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn set_share_level(resource: &str, level: f32) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(resource, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(resource)?),
            };
            crate::generated::borrowed::system_control::set_share_level(__core_string_0_buf.as_cstr(), level)
        }

        #[inline]
        pub fn share_resources(team_id: i32, resource: &str, amount: f32) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(resource, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(resource)?),
            };
            crate::generated::borrowed::system_control::share_resources(team_id, __core_string_1_buf.as_cstr(), amount)
        }

        #[inline]
        pub fn start(cmd_args: &str, start_script: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(cmd_args, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(cmd_args)?),
            };
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(start_script, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(start_script)?),
            };
            crate::generated::borrowed::system_control::start(__core_string_0_buf.as_cstr(), __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn yield_(unused: u8) -> Result<bool> {
            let value = crate::generated::system_control::yield_(unused)?;
            Ok(value)
        }

    }

