// Unsynced profiling helpers for the Spring Core-Wasm guest SDK.

use crate::{ApiError, ErrorCode, Result};

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
    extern "C" {
        #[link_name = "get-timer"]
        pub fn get_timer(unused: i32, output: i32) -> i32;
        #[link_name = "get-timer-micros"]
        pub fn get_timer_micros(unused: i32, output: i32) -> i32;
        #[link_name = "diff-timers"]
        pub fn diff_timers(
            end_timer: i64,
            start_timer: i64,
            return_ms: i32,
            from_micros: i32,
        ) -> i64;
        #[link_name = "get-frame-timer"]
        pub fn get_frame_timer(last_frame_time: i32, output: i32) -> i32;
        #[link_name = "get-draw-seconds"]
        pub fn get_draw_seconds(unused: i32) -> i64;
        #[link_name = "get-lua-mem-usage"]
        pub fn get_lua_mem_usage(unused: i32, output: i32) -> i32;
        #[link_name = "get-vid-mem-usage"]
        pub fn get_vid_mem_usage(unused: i32, output: i32) -> i32;
        #[link_name = "get-synced-gc-info"]
        pub fn get_synced_gc_info(collect: i32) -> i64;
    }
}

#[inline]
pub fn get_timer() -> Result<u64> {
    #[cfg(target_arch = "wasm32")]
    {
        let mut value = [0u8; 8];
        let pointer = value.as_mut_ptr() as usize;
        if pointer > u32::MAX as usize {
            return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
        }
        let status = unsafe { raw::get_timer(0, pointer as u32 as i32) };
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
        let pointer = value.as_mut_ptr() as usize;
        if pointer > u32::MAX as usize {
            return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
        }
        let status = unsafe { raw::get_timer_micros(0, pointer as u32 as i32) };
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
        crate::unpack_f32(unsafe {
            raw::diff_timers(
                end_timer as i64,
                start_timer as i64,
                return_ms as i32,
                from_microseconds as i32,
            )
        })
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
        let pointer = value.as_mut_ptr() as usize;
        if pointer > u32::MAX as usize {
            return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
        }
        let status = unsafe { raw::get_frame_timer(last_frame_time as i32, pointer as u32 as i32) };
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
        crate::unpack_f32(unsafe { raw::get_draw_seconds(0) })
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
        let pointer = values.as_mut_ptr() as usize;
        if pointer > u32::MAX as usize {
            return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
        }
        let status = unsafe { raw::get_lua_mem_usage(0, pointer as u32 as i32) };
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
        let pointer = values.as_mut_ptr() as usize;
        if pointer > u32::MAX as usize {
            return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
        }
        let status = unsafe { raw::get_vid_mem_usage(0, pointer as u32 as i32) };
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
        crate::unpack_f32(unsafe { raw::get_synced_gc_info(collect as i32) })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = collect;
        Err(unreachable!())
    }
}
