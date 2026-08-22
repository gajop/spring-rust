// UnitsQuery portion of the Spring Core-Wasm guest SDK.

use super::{ApiError, ErrorCode, Result};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferFill {
    Complete(usize),
    Insufficient { required: usize },
}

const NEAREST_USE_LOS: u32 = 1 << 0;
const NEAREST_SPHERE_DISTANCE: u32 = 1 << 1;
const NEAREST_CHECK_SIGHT_DISTANCE: u32 = 1 << 2;
const SEPARATION_POSITIONAL: u32 = 1 << 0;
const SEPARATION_CHECK_MAP: u32 = 1 << 1;

#[cfg(target_arch = "wasm32")]
mod units_query_raw {
    #[link(wasm_import_module = "spring:units-query")]
    extern "C" {
        #[link_name = "valid-unit-id"]
        pub fn valid_unit_id(unit_id: i32) -> i64;
        #[link_name = "get-all-units"]
        pub fn get_all_units(output: i32, capacity: i32) -> i64;
        #[link_name = "get-team-units"]
        pub fn get_team_units(team_id: i32, output: i32, capacity: i32) -> i64;
        #[link_name = "get-team-unit-def-count"]
        pub fn get_team_unit_def_count(team_id: i32, unit_def_id: i32) -> i64;
        #[link_name = "get-team-unit-count"]
        pub fn get_team_unit_count(team_id: i32) -> i64;
        #[link_name = "get-units-in-rectangle"]
        pub fn get_units_in_rectangle(
            xmin: f32,
            zmin: f32,
            xmax: f32,
            zmax: f32,
            allegiance: i32,
            output: i32,
            capacity: i32,
        ) -> i64;
        #[link_name = "get-units-in-box"]
        pub fn get_units_in_box(
            xmin: f32,
            ymin: f32,
            zmin: f32,
            xmax: f32,
            ymax: f32,
            zmax: f32,
            allegiance: i32,
            output: i32,
            capacity: i32,
        ) -> i64;
        #[link_name = "get-units-in-sphere"]
        pub fn get_units_in_sphere(
            x: f32,
            y: f32,
            z: f32,
            radius: f32,
            allegiance: i32,
            output: i32,
            capacity: i32,
        ) -> i64;
        #[link_name = "get-units-in-cylinder"]
        pub fn get_units_in_cylinder(
            x: f32,
            z: f32,
            radius: f32,
            allegiance: i32,
            output: i32,
            capacity: i32,
        ) -> i64;
        #[link_name = "get-unit-nearest-ally"]
        pub fn get_unit_nearest_ally(unit_id: i32, range: f32) -> i64;
        #[link_name = "get-unit-nearest-enemy"]
        pub fn get_unit_nearest_enemy(unit_id: i32, range: f32, flags: i32) -> i64;
        #[link_name = "get-unit-separation"]
        pub fn get_unit_separation(unit_id1: i32, unit_id2: i32, flags: i32) -> i64;
    }
}

#[inline]
fn decode_buffer_result(packed: i64) -> Result<BufferFill> {
    let packed = packed as u64;
    let count = packed as u32 as usize;
    let status = (packed >> 32) as u32 as i32;
    if status == 0 {
        Ok(BufferFill::Complete(count))
    } else if status == ErrorCode::BufferOverflow as i32 {
        Ok(BufferFill::Insufficient { required: count })
    } else {
        Err(ApiError::new(status))
    }
}

#[inline]
fn output_parts(output: &mut [i32]) -> (i32, i32) {
    if output.is_empty() {
        return (0, 0);
    }
    let pointer = output.as_mut_ptr() as usize;
    debug_assert!(pointer <= u32::MAX as usize);
    debug_assert!(output.len() <= u32::MAX as usize);
    (pointer as u32 as i32, output.len() as u32 as i32)
}

#[inline]
pub fn valid_unit_id(unit_id: i32) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        // SAFETY: generated scalar signature; no guest memory is touched.
        return super::unpack_bool(unsafe { units_query_raw::valid_unit_id(unit_id) });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = unit_id;
        Err(unreachable!())
    }
}

#[inline]
pub fn get_team_unit_count(team_id: i32) -> Result<u32> {
    #[cfg(target_arch = "wasm32")]
    {
        // SAFETY: generated scalar signature; no guest memory is touched.
        let packed = unsafe { units_query_raw::get_team_unit_count(team_id) } as u64;
        let status = (packed >> 32) as u32 as i32;
        return if status == 0 {
            Ok(packed as u32)
        } else {
            Err(ApiError::new(status))
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = team_id;
        Err(unreachable!())
    }
}

#[inline]
pub fn get_team_unit_def_count(team_id: i32, unit_def_id: i32) -> Result<u32> {
    #[cfg(target_arch = "wasm32")]
    {
        // SAFETY: generated scalar signature; no guest memory is touched.
        let packed =
            unsafe { units_query_raw::get_team_unit_def_count(team_id, unit_def_id) } as u64;
        let status = (packed >> 32) as u32 as i32;
        return if status == 0 {
            Ok(packed as u32)
        } else {
            Err(ApiError::new(status))
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (team_id, unit_def_id);
        Err(unreachable!())
    }
}

#[inline]
pub fn get_all_units_into(output: &mut [i32]) -> Result<BufferFill> {
    #[cfg(target_arch = "wasm32")]
    {
        let (pointer, capacity) = output_parts(output);
        // SAFETY: output is a live i32 slice for this synchronous call. Host
        // validates pointer/capacity before invoking the native query.
        return decode_buffer_result(unsafe { units_query_raw::get_all_units(pointer, capacity) });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = output;
        Err(unreachable!())
    }
}

#[inline]
pub fn get_team_units_into(team_id: i32, output: &mut [i32]) -> Result<BufferFill> {
    #[cfg(target_arch = "wasm32")]
    {
        let (pointer, capacity) = output_parts(output);
        // SAFETY: same caller-owned list convention as get_all_units_into.
        return decode_buffer_result(unsafe {
            units_query_raw::get_team_units(team_id, pointer, capacity)
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (team_id, output);
        Err(unreachable!())
    }
}

#[inline]
pub fn get_units_in_rectangle_into(
    xmin: f32,
    zmin: f32,
    xmax: f32,
    zmax: f32,
    allegiance: i32,
    output: &mut [i32],
) -> Result<BufferFill> {
    #[cfg(target_arch = "wasm32")]
    {
        let (pointer, capacity) = output_parts(output);
        // SAFETY: scalar query plus validated caller-owned output slice.
        return decode_buffer_result(unsafe {
            units_query_raw::get_units_in_rectangle(
                xmin, zmin, xmax, zmax, allegiance, pointer, capacity,
            )
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (xmin, zmin, xmax, zmax, allegiance, output);
        Err(unreachable!())
    }
}

#[inline]
pub fn get_units_in_box_into(
    min: [f32; 3],
    max: [f32; 3],
    allegiance: i32,
    output: &mut [i32],
) -> Result<BufferFill> {
    #[cfg(target_arch = "wasm32")]
    {
        let (pointer, capacity) = output_parts(output);
        // SAFETY: scalar query plus validated caller-owned output slice.
        return decode_buffer_result(unsafe {
            units_query_raw::get_units_in_box(
                min[0], min[1], min[2], max[0], max[1], max[2], allegiance, pointer, capacity,
            )
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (min, max, allegiance, output);
        Err(unreachable!())
    }
}

#[inline]
pub fn get_units_in_sphere_into(
    center: [f32; 3],
    radius: f32,
    allegiance: i32,
    output: &mut [i32],
) -> Result<BufferFill> {
    #[cfg(target_arch = "wasm32")]
    {
        let (pointer, capacity) = output_parts(output);
        // SAFETY: scalar query plus validated caller-owned output slice.
        return decode_buffer_result(unsafe {
            units_query_raw::get_units_in_sphere(
                center[0], center[1], center[2], radius, allegiance, pointer, capacity,
            )
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (center, radius, allegiance, output);
        Err(unreachable!())
    }
}

#[inline]
pub fn get_units_in_cylinder_into(
    x: f32,
    z: f32,
    radius: f32,
    allegiance: i32,
    output: &mut [i32],
) -> Result<BufferFill> {
    #[cfg(target_arch = "wasm32")]
    {
        let (pointer, capacity) = output_parts(output);
        // SAFETY: scalar query plus validated caller-owned output slice.
        return decode_buffer_result(unsafe {
            units_query_raw::get_units_in_cylinder(x, z, radius, allegiance, pointer, capacity)
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (x, z, radius, allegiance, output);
        Err(unreachable!())
    }
}

#[inline]
pub fn get_unit_nearest_ally(unit_id: i32, range: f32) -> Result<i32> {
    #[cfg(target_arch = "wasm32")]
    {
        // SAFETY: generated scalar signature.
        return super::unpack_i32(unsafe {
            units_query_raw::get_unit_nearest_ally(unit_id, range)
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (unit_id, range);
        Err(unreachable!())
    }
}

#[inline]
pub fn get_unit_nearest_enemy(
    unit_id: i32,
    range: f32,
    use_los: bool,
    sphere_distance: bool,
    check_sight_distance: bool,
) -> Result<i32> {
    #[cfg(target_arch = "wasm32")]
    {
        let flags = (if use_los { NEAREST_USE_LOS } else { 0 })
            | (if sphere_distance {
                NEAREST_SPHERE_DISTANCE
            } else {
                0
            })
            | (if check_sight_distance {
                NEAREST_CHECK_SIGHT_DISTANCE
            } else {
                0
            });
        // SAFETY: generated scalar signature.
        return super::unpack_i32(unsafe {
            units_query_raw::get_unit_nearest_enemy(unit_id, range, flags as i32)
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (
            unit_id,
            range,
            use_los,
            sphere_distance,
            check_sight_distance,
        );
        Err(unreachable!())
    }
}

#[inline]
pub fn get_unit_separation(
    unit_id1: i32,
    unit_id2: i32,
    positional: bool,
    check_map: bool,
) -> Result<f32> {
    #[cfg(target_arch = "wasm32")]
    {
        let flags = (if positional { SEPARATION_POSITIONAL } else { 0 })
            | (if check_map { SEPARATION_CHECK_MAP } else { 0 });
        // SAFETY: generated scalar signature.
        return super::unpack_f32(unsafe {
            units_query_raw::get_unit_separation(unit_id1, unit_id2, flags as i32)
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (unit_id1, unit_id2, positional, check_map);
        Err(unreachable!())
    }
}

#[cfg(all(feature = "alloc", target_arch = "wasm32"))]
fn collect_list(mut call: impl FnMut(i32, i32) -> i64) -> Result<Vec<i32>> {
    let first = decode_buffer_result(call(0, 0))?;
    let required = match first {
        BufferFill::Complete(0) => return Ok(Vec::new()),
        BufferFill::Complete(count) | BufferFill::Insufficient { required: count } => count,
    };
    let mut output = vec![0i32; required];
    for _ in 0..3 {
        let pointer = output.as_mut_ptr() as usize;
        debug_assert!(pointer <= u32::MAX as usize);
        let fill = decode_buffer_result(call(pointer as u32 as i32, output.len() as u32 as i32))?;
        match fill {
            BufferFill::Complete(count) => {
                output.truncate(count);
                return Ok(output);
            }
            BufferFill::Insufficient { required } => output.resize(required, 0),
        }
    }
    Err(ApiError::new(ErrorCode::BufferOverflow as i32))
}

#[cfg(feature = "alloc")]
pub fn get_all_units() -> Result<Vec<i32>> {
    #[cfg(target_arch = "wasm32")]
    {
        // SAFETY is contained inside each call: ptr/capacity describe the live
        // Vec allocation synchronously and the host bounds-checks both.
        return collect_list(|pointer, capacity| unsafe {
            units_query_raw::get_all_units(pointer, capacity)
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Err(unreachable!())
    }
}

#[cfg(feature = "alloc")]
pub fn get_team_units(team_id: i32) -> Result<Vec<i32>> {
    #[cfg(target_arch = "wasm32")]
    {
        return collect_list(|pointer, capacity| unsafe {
            units_query_raw::get_team_units(team_id, pointer, capacity)
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = team_id;
        Err(unreachable!())
    }
}
