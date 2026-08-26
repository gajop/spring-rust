// Benchmark-critical Core-Wasm guest wrappers.
//
// This module only keeps calls that have no other home: the benchmark-only
// consume imports, terrain mutation, and the immediate Gfx paths. Timer,
// message, RulesParams and Terrain-read wrappers that once lived here are now
// in profiling/messages/rules_params/terrain, which validate their inputs; the
// duplicates here made every one of those names ambiguous through `lib.rs`.
// `set_height_map_func` left for the same reason: the reviewed synchronous
// callback wrapper in `terrain_control` is the single definition.

use super::{ApiError, ErrorCode, Result};

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:profiling")]
    unsafe extern "C" {}

    #[link(wasm_import_module = "spring:messages")]
    unsafe extern "C" {}

    #[link(wasm_import_module = "spring:benchmark")]
    unsafe extern "C" {
        #[link_name = "consume-string"]
        pub fn consume_string(pointer: i32, length: i32) -> i64;
        #[link_name = "consume-f32-list"]
        pub fn consume_f32_list(pointer: i32, count: i32) -> i64;
    }

    #[link(wasm_import_module = "spring:rules-params")]
    unsafe extern "C" {}

    #[link(wasm_import_module = "spring:terrain")]
    unsafe extern "C" {}

    #[link(wasm_import_module = "spring:terrain-control")]
    unsafe extern "C" {
        #[link_name = "set-height-map"]
        pub fn set_height_map(x: f32, z: f32, height: f32, terraform: f32) -> i64;
        #[link_name = "level-height-map"]
        pub fn level_height_map(x1: f32, z1: f32, x2: f32, z2: f32, height: f32) -> i64;
    }

    #[link(wasm_import_module = "spring:gfx")]
    unsafe extern "C" {
        #[link_name = "vertex"]
        pub fn gfx_vertex(x: f32, y: f32, z: f32, w: f32, count: i32) -> i32;
        #[link_name = "begin-end"]
        pub fn gfx_begin_end(primitive: i32, callback_id: i32, user_data: i32) -> i32;
    }
}

#[inline]
fn bytes_parts(value: &[u8]) -> (i32, i32) {
    if value.is_empty() {
        return (0, 0);
    }
    let pointer = value.as_ptr() as usize;
    debug_assert!(pointer <= u32::MAX as usize);
    debug_assert!(value.len() <= u32::MAX as usize);
    (pointer as u32 as i32, value.len() as u32 as i32)
}

#[inline]
fn f32_parts(value: &[f32]) -> (i32, i32) {
    if value.is_empty() {
        return (0, 0);
    }
    let pointer = value.as_ptr() as usize;
    debug_assert!(pointer <= u32::MAX as usize);
    debug_assert!(value.len() <= u32::MAX as usize);
    (pointer as u32 as i32, value.len() as u32 as i32)
}

#[inline]
fn unpack_u32_local(packed: i64) -> Result<u32> {
    let packed = packed as u64;
    let status = (packed >> 32) as u32 as i32;
    if status == 0 {
        Ok(packed as u32)
    } else {
        Err(ApiError::new(status))
    }
}

#[inline]
fn unpack_bool_local(packed: i64) -> Result<bool> {
    match unpack_u32_local(packed)? {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(ApiError::new(ErrorCode::Internal as i32)),
    }
}

#[inline]
fn status_result(status: i32) -> Result<()> {
    if status == 0 {
        Ok(())
    } else {
        Err(ApiError::new(status))
    }
}

// Benchmark-only wrappers. They intentionally expose a no-allocation borrowed
// input path so the suite can measure the Core variable-input transport floor.
#[inline]
pub fn benchmark_consume_string(value: &str) -> Result<u32> {
    #[cfg(target_arch = "wasm32")]
    {
        let (pointer, length) = bytes_parts(value.as_bytes());
        unpack_u32_local(unsafe { raw::consume_string(pointer, length) })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = value;
        Err(unreachable!())
    }
}

#[inline]
pub fn benchmark_consume_f32_list(value: &[f32]) -> Result<u32> {
    #[cfg(target_arch = "wasm32")]
    {
        let (pointer, count) = f32_parts(value);
        unpack_u32_local(unsafe { raw::consume_f32_list(pointer, count) })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = value;
        Err(unreachable!())
    }
}

#[inline]
pub fn set_height_map(x: f32, z: f32, height: f32, terraform: f32) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        unpack_bool_local(unsafe { raw::set_height_map(x, z, height, terraform) })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (x, z, height, terraform);
        Err(unreachable!())
    }
}

#[inline]
pub fn level_height_map(x1: f32, z1: f32, x2: f32, z2: f32, height: f32) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        unpack_bool_local(unsafe { raw::level_height_map(x1, z1, x2, z2, height) })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (x1, z1, x2, z2, height);
        Err(unreachable!())
    }
}

#[inline]
pub fn gfx_vertex(x: f32, y: f32, z: f32, w: f32, count: u32) -> Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        status_result(unsafe { raw::gfx_vertex(x, y, z, w, count as i32) })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (x, y, z, w, count);
        Err(unreachable!())
    }
}

#[inline]
pub fn gfx_begin_end(primitive: u32, callback_id: u32, user_data: u32) -> Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        status_result(unsafe {
            raw::gfx_begin_end(primitive as i32, callback_id as i32, user_data as i32)
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (primitive, callback_id, user_data);
        Err(unreachable!())
    }
}
