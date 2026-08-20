// Unsynced profiling helpers used by the benchmark suite.

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

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:profiling")]
    extern "C" {
        #[link_name = "get-lua-mem-usage"]
        pub fn get_lua_mem_usage(output: i32) -> i32;
        #[link_name = "get-synced-gc-info"]
        pub fn get_synced_gc_info(collect: i32) -> i64;
    }
}

#[inline]
pub fn get_lua_mem_usage() -> Result<LuaMemUsage> {
    #[cfg(target_arch = "wasm32")]
    {
        let mut values = [0.0f32; 8];
        let pointer = values.as_mut_ptr() as usize;
        debug_assert!(pointer <= u32::MAX as usize);
        let status = unsafe { raw::get_lua_mem_usage(pointer as u32 as i32) };
        if status != 0 {
            return Err(ApiError::new(status));
        }
        return Ok(LuaMemUsage {
            handle_alloced_kb: values[0],
            handle_allocs_k: values[1],
            global_alloced_kb: values[2],
            global_allocs_k: values[3],
            unsynced_alloced_kb: values[4],
            unsynced_allocs_k: values[5],
            synced_alloced_kb: values[6],
            synced_allocs_k: values[7],
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

#[inline]
pub fn get_synced_gc_info(collect: bool) -> Result<f32> {
    #[cfg(target_arch = "wasm32")]
    {
        let packed = unsafe { raw::get_synced_gc_info(collect as i32) } as u64;
        let status = (packed >> 32) as u32 as i32;
        if status != 0 {
            return Err(ApiError::new(status));
        }
        return Ok(f32::from_bits(packed as u32));
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = collect;
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}
