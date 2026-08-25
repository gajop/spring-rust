    pub mod profiling {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DiffTimersOptions {
            pub return_ms: bool,
            pub from_micro_secs: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DiffTimersQuery {
            pub end_timer: u64,
            pub start_timer: u64,
            pub options: DiffTimersOptions,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DiffTimersResult {
            pub seconds: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetDrawSecondsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetDrawSecondsResult {
            pub seconds: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFrameTimerQuery {
            pub last_frame_time: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFrameTimerResult {
            pub timer: u64,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetLuaMemUsageQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetSyncedGCInfoQuery {
            pub collect: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetSyncedGCInfoResult {
            pub gc_kb: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTimerMicrosQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTimerMicrosResult {
            pub timer: u64,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTimerQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTimerResult {
            pub timer: u64,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetVidMemUsageQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetVidMemUsageResult {
            pub used_mb: f32,
            pub available_mb: f32,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[inline]
        pub fn get_profiler_record_names(unused: u8) -> Result<Vec<String>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::profiling::get_profiler_record_names(unused as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items };
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
        mod __core_owned_get_profiler_time_record {
            #[link(wasm_import_module = "spring:profiling")]
            unsafe extern "C" {
                #[link_name = "get-profiler-time-record"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:profiling.get-profiler-time-record."]
        #[doc(hidden)]
        #[inline]
        pub fn get_profiler_time_record(p0: i32, p1: i32, p2: i32) -> i32 {
            __core_owned_get_profiler_time_record::call(p0, p1, p2)
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

