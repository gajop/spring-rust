#[cfg(feature = "alloc")]
pub use crate::owned::profiling::{get_profiler_record_names, get_profiler_time_record};

// Unsynced profiling helpers for the Spring Core-Wasm guest SDK.

use crate::{ApiError, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LuaMemUsage {
    pub handle_alloced_kb: f32,
    pub handle_allocs_k: f32,
    pub global_alloced_kb: f32,
    pub global_allocs_k: f32,
    pub unsynced_alloced_kb: f32,
    pub unsynced_allocs_k: f32,
    pub synced_alloced_kb: f32,
    pub synced_allocs_k: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VidMemUsage {
    pub used_mb: f32,
    pub available_mb: f32,
}

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:profiling")]
    unsafe extern "C" {
        #[link_name = "get-timer"]
        pub safe fn get_timer(unused: i32, output: i32) -> i32;
        #[link_name = "get-timer-micros"]
        pub safe fn get_timer_micros(unused: i32, output: i32) -> i32;
        #[link_name = "diff-timers"]
        pub safe fn diff_timers(
            end_timer: i64,
            start_timer: i64,
            return_ms: i32,
            from_micros: i32,
        ) -> i64;
        #[link_name = "get-frame-timer"]
        pub safe fn get_frame_timer(last_frame_time: i32, output: i32) -> i32;
        #[link_name = "get-draw-seconds"]
        pub safe fn get_draw_seconds(unused: i32) -> i64;
        #[link_name = "get-lua-mem-usage"]
        pub safe fn get_lua_mem_usage(unused: i32, output: i32) -> i32;
        #[link_name = "get-vid-mem-usage"]
        pub safe fn get_vid_mem_usage(unused: i32, output: i32) -> i32;
        #[link_name = "get-synced-gc-info"]
        pub safe fn get_synced_gc_info(collect: i32) -> i64;
    }
}

#[inline]
pub fn get_timer() -> Result<u64> {
    #[cfg(target_arch = "wasm32")]
    {
        let mut value = [0u8; 8];
        let pointer = super::wasm_output_ptr(&mut value)?;
        let status = raw::get_timer(0, pointer);
        if status != 0 {
            return Err(ApiError::new(status));
        }
        Ok(u64::from_le_bytes(value))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Err(unreachable!())
    }
}

#[inline]
pub fn get_timer_micros() -> Result<u64> {
    #[cfg(target_arch = "wasm32")]
    {
        let mut value = [0u8; 8];
        let pointer = super::wasm_output_ptr(&mut value)?;
        let status = raw::get_timer_micros(0, pointer);
        if status != 0 {
            return Err(ApiError::new(status));
        }
        Ok(u64::from_le_bytes(value))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Err(unreachable!())
    }
}

#[inline]
pub fn diff_timers(
    end_timer: u64,
    start_timer: u64,
    return_ms: bool,
    from_microseconds: bool,
) -> Result<f32> {
    #[cfg(target_arch = "wasm32")]
    {
        crate::decode_packed_f32(raw::diff_timers(
            end_timer as i64,
            start_timer as i64,
            return_ms as i32,
            from_microseconds as i32,
        ))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (end_timer, start_timer, return_ms, from_microseconds);
        Err(unreachable!())
    }
}

#[inline]
pub fn get_frame_timer(last_frame_time: bool) -> Result<u64> {
    #[cfg(target_arch = "wasm32")]
    {
        let mut value = [0u8; 8];
        let pointer = super::wasm_output_ptr(&mut value)?;
        let status = raw::get_frame_timer(last_frame_time as i32, pointer);
        if status != 0 {
            return Err(ApiError::new(status));
        }
        Ok(u64::from_le_bytes(value))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = last_frame_time;
        Err(unreachable!())
    }
}

#[inline]
pub fn get_draw_seconds() -> Result<f32> {
    #[cfg(target_arch = "wasm32")]
    {
        crate::decode_packed_f32(raw::get_draw_seconds(0))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Err(unreachable!())
    }
}

#[inline]
pub fn get_lua_mem_usage() -> Result<LuaMemUsage> {
    #[cfg(target_arch = "wasm32")]
    {
        let mut values = [0.0f32; 8];
        let pointer = super::wasm_output_ptr(&mut values)?;
        let status = raw::get_lua_mem_usage(0, pointer);
        if status != 0 {
            return Err(ApiError::new(status));
        }
        Ok(LuaMemUsage {
            handle_alloced_kb: values[0],
            handle_allocs_k: values[1],
            global_alloced_kb: values[2],
            global_allocs_k: values[3],
            unsynced_alloced_kb: values[4],
            unsynced_allocs_k: values[5],
            synced_alloced_kb: values[6],
            synced_allocs_k: values[7],
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Err(unreachable!())
    }
}

#[inline]
pub fn get_vid_mem_usage() -> Result<VidMemUsage> {
    #[cfg(target_arch = "wasm32")]
    {
        let mut values = [0.0f32; 2];
        let pointer = super::wasm_output_ptr(&mut values)?;
        let status = raw::get_vid_mem_usage(0, pointer);
        if status != 0 {
            return Err(ApiError::new(status));
        }
        Ok(VidMemUsage {
            used_mb: values[0],
            available_mb: values[1],
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Err(unreachable!())
    }
}

#[inline]
pub fn get_synced_gc_info(collect: bool) -> Result<f32> {
    #[cfg(target_arch = "wasm32")]
    {
        crate::decode_packed_f32(raw::get_synced_gc_info(collect as i32))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = collect;
        Err(unreachable!())
    }
}
