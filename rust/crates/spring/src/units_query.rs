#[cfg(feature = "alloc")]
pub use crate::owned::units_query::{
    get_closest_enemy_unit, get_render_units, get_render_units_draw_flag_changed,
    get_team_units_by_defs, get_team_units_counts, get_team_units_sorted, get_units_in_planes,
};

// UnitsQuery portion of the Spring Core-Wasm guest SDK.

use super::{ApiError, DefId, ErrorCode, Float3, Result, TeamId, UnitId};

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
    unsafe extern "C" {
        #[link_name = "valid-unit-id"]
        pub safe fn valid_unit_id(unit_id: i32) -> i64;
        #[link_name = "get-all-units"]
        pub safe fn get_all_units(output: i32, capacity: i32) -> i64;
        #[link_name = "get-team-units"]
        pub safe fn get_team_units(team_id: i32, output: i32, capacity: i32) -> i64;
        #[link_name = "get-team-unit-def-count"]
        pub safe fn get_team_unit_def_count(team_id: i32, unit_def_id: i32) -> i64;
        #[link_name = "get-team-unit-count"]
        pub safe fn get_team_unit_count(team_id: i32) -> i64;
        #[link_name = "get-units-in-rectangle"]
        pub safe fn get_units_in_rectangle(
            xmin: f32,
            zmin: f32,
            xmax: f32,
            zmax: f32,
            allegiance: i32,
            output: i32,
            capacity: i32,
        ) -> i64;
        #[link_name = "get-units-in-box"]
        pub safe fn get_units_in_box(
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
        pub safe fn get_units_in_sphere(
            x: f32,
            y: f32,
            z: f32,
            radius: f32,
            allegiance: i32,
            output: i32,
            capacity: i32,
        ) -> i64;
        #[link_name = "get-units-in-cylinder"]
        pub safe fn get_units_in_cylinder(
            x: f32,
            z: f32,
            radius: f32,
            allegiance: i32,
            output: i32,
            capacity: i32,
        ) -> i64;
        #[link_name = "get-unit-nearest-ally"]
        pub safe fn get_unit_nearest_ally(unit_id: i32, range: f32) -> i64;
        #[link_name = "get-unit-nearest-enemy"]
        pub safe fn get_unit_nearest_enemy(unit_id: i32, range: f32, flags: i32) -> i64;
        #[link_name = "get-unit-separation"]
        pub safe fn get_unit_separation(unit_id1: i32, unit_id2: i32, flags: i32) -> i64;
    }
}

#[inline]
fn decode_buffer_result(packed: i64, capacity: usize) -> Result<BufferFill> {
    let packed = packed as u64;
    let count = packed as u32 as usize;
    let status = (packed >> 32) as u32 as i32;
    if status == 0 {
        if count > capacity {
            Err(ApiError::new(ErrorCode::Internal as i32))
        } else {
            Ok(BufferFill::Complete(count))
        }
    } else if status == ErrorCode::BufferOverflow as i32 {
        Ok(BufferFill::Insufficient { required: count })
    } else {
        Err(ApiError::new(status))
    }
}

#[inline]
fn output_parts(output: &mut [i32]) -> Result<(i32, i32)> {
    super::wasm_mut_slice_parts(output)
}

#[inline]
pub fn valid_unit_id(unit_id: impl Into<UnitId>) -> Result<bool> {
    let unit_id = unit_id.into();
    #[cfg(target_arch = "wasm32")]
    {
        super::unpack_bool(units_query_raw::valid_unit_id(unit_id.0))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = unit_id;
        Err(unreachable!())
    }
}

#[inline]
pub fn get_team_unit_count(team_id: impl Into<TeamId>) -> Result<u32> {
    let team_id = team_id.into();
    #[cfg(target_arch = "wasm32")]
    {
        let packed = units_query_raw::get_team_unit_count(team_id.0) as u64;
        let status = (packed >> 32) as u32 as i32;
        if status == 0 {
            Ok(packed as u32)
        } else {
            Err(ApiError::new(status))
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = team_id;
        Err(unreachable!())
    }
}

#[inline]
pub fn get_team_unit_def_count(
    team_id: impl Into<TeamId>,
    unit_def_id: impl Into<DefId>,
) -> Result<u32> {
    let team_id = team_id.into();
    let unit_def_id = unit_def_id.into();
    #[cfg(target_arch = "wasm32")]
    {
        let packed = units_query_raw::get_team_unit_def_count(team_id.0, unit_def_id.0) as u64;
        let status = (packed >> 32) as u32 as i32;
        if status == 0 {
            Ok(packed as u32)
        } else {
            Err(ApiError::new(status))
        }
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
        let (pointer, capacity) = output_parts(output)?;
        decode_buffer_result(
            units_query_raw::get_all_units(pointer, capacity),
            output.len(),
        )
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = output;
        Err(unreachable!())
    }
}

#[inline]
pub fn get_team_units_into(team_id: impl Into<TeamId>, output: &mut [i32]) -> Result<BufferFill> {
    let team_id = team_id.into();
    #[cfg(target_arch = "wasm32")]
    {
        let (pointer, capacity) = output_parts(output)?;
        decode_buffer_result(
            units_query_raw::get_team_units(team_id.0, pointer, capacity),
            output.len(),
        )
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
        let (pointer, capacity) = output_parts(output)?;
        decode_buffer_result(
            units_query_raw::get_units_in_rectangle(
                xmin, zmin, xmax, zmax, allegiance, pointer, capacity,
            ),
            output.len(),
        )
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (xmin, zmin, xmax, zmax, allegiance, output);
        Err(unreachable!())
    }
}

#[inline]
pub fn get_units_in_box_into(
    min: Float3,
    max: Float3,
    allegiance: i32,
    output: &mut [i32],
) -> Result<BufferFill> {
    #[cfg(target_arch = "wasm32")]
    {
        let (pointer, capacity) = output_parts(output)?;
        decode_buffer_result(
            units_query_raw::get_units_in_box(
                min.x, min.y, min.z, max.x, max.y, max.z, allegiance, pointer, capacity,
            ),
            output.len(),
        )
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (min, max, allegiance, output);
        Err(unreachable!())
    }
}

#[inline]
pub fn get_units_in_sphere_into(
    center: Float3,
    radius: f32,
    allegiance: i32,
    output: &mut [i32],
) -> Result<BufferFill> {
    #[cfg(target_arch = "wasm32")]
    {
        let (pointer, capacity) = output_parts(output)?;
        decode_buffer_result(
            units_query_raw::get_units_in_sphere(
                center.x, center.y, center.z, radius, allegiance, pointer, capacity,
            ),
            output.len(),
        )
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
        let (pointer, capacity) = output_parts(output)?;
        decode_buffer_result(
            units_query_raw::get_units_in_cylinder(x, z, radius, allegiance, pointer, capacity),
            output.len(),
        )
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (x, z, radius, allegiance, output);
        Err(unreachable!())
    }
}

#[inline]
pub fn get_unit_nearest_ally(unit_id: impl Into<UnitId>, range: f32) -> Result<UnitId> {
    let unit_id = unit_id.into();
    #[cfg(target_arch = "wasm32")]
    {
        super::unpack_i32(units_query_raw::get_unit_nearest_ally(unit_id.0, range)).map(UnitId)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (unit_id, range);
        Err(unreachable!())
    }
}

#[inline]
pub fn get_unit_nearest_enemy(
    unit_id: impl Into<UnitId>,
    range: f32,
    use_los: bool,
    sphere_distance: bool,
    check_sight_distance: bool,
) -> Result<UnitId> {
    let unit_id = unit_id.into();
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
        super::unpack_i32(units_query_raw::get_unit_nearest_enemy(
            unit_id.0,
            range,
            flags as i32,
        ))
        .map(UnitId)
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
    unit_id1: impl Into<UnitId>,
    unit_id2: impl Into<UnitId>,
    positional: bool,
    check_map: bool,
) -> Result<f32> {
    let unit_id1 = unit_id1.into();
    let unit_id2 = unit_id2.into();
    #[cfg(target_arch = "wasm32")]
    {
        let flags = (if positional { SEPARATION_POSITIONAL } else { 0 })
            | (if check_map { SEPARATION_CHECK_MAP } else { 0 });
        super::decode_packed_f32(units_query_raw::get_unit_separation(
            unit_id1.0,
            unit_id2.0,
            flags as i32,
        ))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (unit_id1, unit_id2, positional, check_map);
        Err(unreachable!())
    }
}

#[cfg(all(feature = "alloc", target_arch = "wasm32"))]
fn collect_list(mut call: impl FnMut(i32, i32) -> i64) -> Result<Vec<i32>> {
    let first = decode_buffer_result(call(0, 0), 0)?;
    let required = match first {
        BufferFill::Complete(0) => return Ok(Vec::new()),
        BufferFill::Complete(count) | BufferFill::Insufficient { required: count } => count,
    };
    let mut output = vec![0i32; required];
    for _ in 0..3 {
        let (pointer, capacity) = super::wasm_mut_slice_parts(&mut output)?;
        let fill = decode_buffer_result(call(pointer, capacity), output.len())?;
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
        collect_list(|pointer, capacity| units_query_raw::get_all_units(pointer, capacity))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Err(unreachable!())
    }
}

#[cfg(feature = "alloc")]
pub fn get_team_units(team_id: impl Into<TeamId>) -> Result<Vec<i32>> {
    let team_id = team_id.into();
    #[cfg(target_arch = "wasm32")]
    {
        collect_list(|pointer, capacity| {
            units_query_raw::get_team_units(team_id.0, pointer, capacity)
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = team_id;
        Err(unreachable!())
    }
}
