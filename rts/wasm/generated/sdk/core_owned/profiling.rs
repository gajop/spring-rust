    pub mod profiling {
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
        pub struct DefRef {
            pub name: String,
            pub id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DiffTimersOptions {
            pub return_ms: bool,
            pub from_micro_secs: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DiffTimersQuery {
            pub end_timer: u64,
            pub start_timer: u64,
            pub options: DiffTimersOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DiffTimersResult {
            pub seconds: f32,
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
        pub struct GetDrawSecondsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetDrawSecondsResult {
            pub seconds: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFrameTimerQuery {
            pub last_frame_time: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFrameTimerResult {
            pub timer: u64,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetLuaMemUsageQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetLuaMemUsageResult {
            pub handle_alloced_kb: f32,
            pub handle_allocs_k: f32,
            pub global_alloced_kb: f32,
            pub global_allocs_k: f32,
            pub unsynced_alloced_kb: f32,
            pub unsynced_allocs_k: f32,
            pub synced_alloced_kb: f32,
            pub synced_allocs_k: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProfilerRecordNamesQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProfilerRecordNamesResult {
            pub names: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProfilerTimeRecordQuery {
            pub name: String,
            pub include_frame_data: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProfilerTimeRecordResult {
            pub total_ms: f32,
            pub current_ms: f32,
            pub max_dt: f32,
            pub time_pct: f32,
            pub peak_pct: f32,
            pub frame_data: Vec<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSyncedGCInfoQuery {
            pub collect: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSyncedGCInfoResult {
            pub gc_kb: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTimerMicrosQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTimerMicrosResult {
            pub timer: u64,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTimerQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTimerResult {
            pub timer: u64,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetVidMemUsageQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetVidMemUsageResult {
            pub used_mb: f32,
            pub available_mb: f32,
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetLuaMemUsageValue {
            pub handle_alloced_kb: f32,
            pub handle_allocs_k: f32,
            pub global_alloced_kb: f32,
            pub global_allocs_k: f32,
            pub unsynced_alloced_kb: f32,
            pub unsynced_allocs_k: f32,
            pub synced_alloced_kb: f32,
            pub synced_allocs_k: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProfilerTimeRecordValue {
            pub total_ms: f32,
            pub current_ms: f32,
            pub max_dt: f32,
            pub time_pct: f32,
            pub peak_pct: f32,
            pub frame_data: Vec<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetVidMemUsageValue {
            pub used_mb: f32,
            pub available_mb: f32,
        }

        #[inline]
        pub fn diff_timers(end_timer: u64, start_timer: u64, options: DiffTimersOptions) -> Result<f32> {
            let value = crate::generated::profiling::diff_timers(end_timer, start_timer, crate::generated::profiling::DiffTimersOptions { return_ms: options.return_ms, from_micro_secs: options.from_micro_secs })?;
            Ok(value)
        }

        #[inline]
        pub fn get_draw_seconds(unused: u8) -> Result<f32> {
            let value = crate::generated::profiling::get_draw_seconds(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_frame_timer(last_frame_time: bool) -> Result<u64> {
            let value = crate::generated::profiling::get_frame_timer(last_frame_time)?;
            Ok(value)
        }

        #[inline]
        pub fn get_lua_mem_usage(unused: u8) -> Result<GetLuaMemUsageValue> {
            let value = crate::generated::profiling::get_lua_mem_usage(unused)?;
            Ok(GetLuaMemUsageValue {
                handle_alloced_kb: value.0,
                handle_allocs_k: value.1,
                global_alloced_kb: value.2,
                global_allocs_k: value.3,
                unsynced_alloced_kb: value.4,
                unsynced_allocs_k: value.5,
                synced_alloced_kb: value.6,
                synced_allocs_k: value.7
            })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_profiler_record_names {
            #[link(wasm_import_module = "spring:profiling")]
            extern "C" {
                #[link_name = "get-profiler-record-names"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:profiling.get-profiler-record-names."]
        #[inline]
        pub unsafe fn get_profiler_record_names(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_profiler_record_names::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_profiler_time_record {
            #[link(wasm_import_module = "spring:profiling")]
            extern "C" {
                #[link_name = "get-profiler-time-record"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:profiling.get-profiler-time-record."]
        #[inline]
        pub unsafe fn get_profiler_time_record(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_get_profiler_time_record::call(p0, p1, p2) }
        }

        #[inline]
        pub fn get_synced_gc_info(collect: bool) -> Result<f32> {
            let value = crate::generated::profiling::get_synced_gc_info(collect)?;
            Ok(value)
        }

        #[inline]
        pub fn get_timer(unused: u8) -> Result<u64> {
            let value = crate::generated::profiling::get_timer(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_timer_micros(unused: u8) -> Result<u64> {
            let value = crate::generated::profiling::get_timer_micros(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_vid_mem_usage(unused: u8) -> Result<GetVidMemUsageValue> {
            let value = crate::generated::profiling::get_vid_mem_usage(unused)?;
            Ok(GetVidMemUsageValue {
                used_mb: value.0,
                available_mb: value.1
            })
        }

    }

