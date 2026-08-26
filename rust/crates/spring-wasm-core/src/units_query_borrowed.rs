// Allocation-free list-input UnitsQuery wrappers for the Spring Core-Wasm SDK.

use super::{ApiError, BufferFill, ErrorCode, Result};

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:units-query")]
    unsafe extern "C" {
        #[link_name = "get-team-units-by-defs"]
        pub fn get_team_units_by_defs(
            team_id: i32,
            unit_def_ids: i32,
            unit_def_count: i32,
            output: i32,
            capacity: i32,
        ) -> i64;
        #[link_name = "get-unit-array-centroid"]
        pub fn get_unit_array_centroid(unit_ids: i32, count: i32, output: i32) -> i32;
        #[link_name = "get-unit-map-centroid"]
        pub fn get_unit_map_centroid(unit_ids: i32, count: i32, output: i32) -> i32;
    }
}

#[cfg(target_arch = "wasm32")]
#[inline]
fn input_parts(values: &[i32]) -> Result<(i32, i32)> {
    if values.len() > u32::MAX as usize {
        return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
    }
    if values.is_empty() {
        return Ok((0, 0));
    }
    let pointer = values.as_ptr() as usize;
    if pointer > u32::MAX as usize {
        return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
    }
    Ok((pointer as u32 as i32, values.len() as u32 as i32))
}

#[cfg(target_arch = "wasm32")]
#[inline]
fn output_parts(values: &mut [i32]) -> Result<(i32, i32)> {
    if values.len() > u32::MAX as usize {
        return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
    }
    if values.is_empty() {
        return Ok((0, 0));
    }
    let pointer = values.as_mut_ptr() as usize;
    if pointer > u32::MAX as usize {
        return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
    }
    Ok((pointer as u32 as i32, values.len() as u32 as i32))
}

#[cfg(target_arch = "wasm32")]
#[inline]
fn decode_fill(packed: i64) -> Result<BufferFill> {
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
pub fn get_team_units_by_defs_into(
    team_id: i32,
    unit_def_ids: &[i32],
    output: &mut [i32],
) -> Result<BufferFill> {
    #[cfg(target_arch = "wasm32")]
    {
        let (input, input_count) = input_parts(unit_def_ids)?;
        let (output, capacity) = output_parts(output)?;
        decode_fill(unsafe {
            raw::get_team_units_by_defs(team_id, input, input_count, output, capacity)
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (team_id, unit_def_ids, output);
        Err(unreachable!())
    }
}

#[cfg(target_arch = "wasm32")]
#[inline]
fn centroid(
    unit_ids: &[i32],
    call: unsafe extern "C" fn(i32, i32, i32) -> i32,
) -> Result<[f32; 3]> {
    let (input, count) = input_parts(unit_ids)?;
    let mut output = [0.0f32; 3];
    let pointer = output.as_mut_ptr() as usize;
    if pointer > u32::MAX as usize {
        return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
    }
    let status = unsafe { call(input, count, pointer as u32 as i32) };
    if status == 0 {
        Ok(output)
    } else {
        Err(ApiError::new(status))
    }
}

#[inline]
pub fn get_unit_array_centroid(unit_ids: &[i32]) -> Result<[f32; 3]> {
    #[cfg(target_arch = "wasm32")]
    {
        centroid(unit_ids, raw::get_unit_array_centroid)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = unit_ids;
        Err(unreachable!())
    }
}

#[inline]
pub fn get_unit_map_centroid(unit_ids: &[i32]) -> Result<[f32; 3]> {
    #[cfg(target_arch = "wasm32")]
    {
        centroid(unit_ids, raw::get_unit_map_centroid)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = unit_ids;
        Err(unreachable!())
    }
}
