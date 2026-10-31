// This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
//! One property of many units in one engine call (`spring:columns`).
//!
//! Values travel as planes of 4-byte values: every unit's first component, then every
//! unit's second, and so on. Each entry runs the per-unit import's own engine read or
//! write; an entry that import would refuse is zeroed and marked 0 in `found`.

use crate::{ApiError, ErrorCode, Result, wasm_mut_slice_parts, wasm_slice_parts};

mod raw {
    #[link(wasm_import_module = "spring:columns")]
    unsafe extern "C" {
        #[link_name = "get-units"]
        pub safe fn get_units(
            property: i32,
            ids: i32,
            count: i32,
            args: i32,
            arg: i32,
            out: i32,
            out_bytes: i32,
            found: i32,
        ) -> i64;
        #[link_name = "set-units"]
        pub safe fn set_units(
            property: i32,
            ids: i32,
            count: i32,
            args: i32,
            arg: i32,
            values: i32,
            values_bytes: i32,
            done: i32,
        ) -> i64;
        #[link_name = "get-units-rules-param"]
        pub safe fn get_units_rules_param(
            name: i32,
            name_bytes: i32,
            ids: i32,
            count: i32,
            out: i32,
            found: i32,
        ) -> i64;
    }
}

fn unpack(packed: i64) -> Result<usize> {
    let packed = packed as u64;
    let status = (packed >> 32) as i32;
    if status != 0 {
        return Err(ApiError::new(status));
    }
    Ok(packed as u32 as usize)
}

fn flags(flags: Option<&mut [u8]>, count: usize) -> Result<i32> {
    match flags {
        None => Ok(0),
        Some(flags) if flags.len() == count => Ok(wasm_mut_slice_parts(flags)?.0),
        Some(_) => Err(ApiError::new(ErrorCode::InvalidArgument as i32)),
    }
}

fn arguments(args: Option<&[i32]>, count: usize) -> Result<i32> {
    match args {
        None => Ok(0),
        Some(args) if args.len() == count => Ok(wasm_slice_parts(args)?.0),
        Some(_) => Err(ApiError::new(ErrorCode::InvalidArgument as i32)),
    }
}

/// Read `property` of every unit in `ids` into `planes` (4-byte values, all of the first
/// component, then all of the second...). `args` gives each entry its argument, else
/// `arg` is every entry's. Returns how many entries were found.
pub fn get_units(
    property: i32,
    ids: &[i32],
    args: Option<&[i32]>,
    arg: i32,
    planes: &mut [u32],
    found: Option<&mut [u8]>,
) -> Result<usize> {
    let count = ids.len();
    let found = flags(found, count)?;
    let args = arguments(args, count)?;
    let (ids, _) = wasm_slice_parts(ids)?;
    let bytes = planes.len() * 4;
    let (out, _) = wasm_mut_slice_parts(planes)?;
    unpack(raw::get_units(
        property,
        ids,
        count as i32,
        args,
        arg,
        out,
        bytes as i32,
        found,
    ))
}

/// Write `property` of every unit in `ids` from `planes`; synced modules only.
pub fn set_units(
    property: i32,
    ids: &[i32],
    args: Option<&[i32]>,
    arg: i32,
    planes: &[u32],
    done: Option<&mut [u8]>,
) -> Result<usize> {
    let count = ids.len();
    let done = flags(done, count)?;
    let args = arguments(args, count)?;
    let (ids, _) = wasm_slice_parts(ids)?;
    let bytes = planes.len() * 4;
    let (values, _) = wasm_slice_parts(planes)?;
    unpack(raw::set_units(
        property,
        ids,
        count as i32,
        args,
        arg,
        values,
        bytes as i32,
        done,
    ))
}

/// The float unit rules parameter `name` of every unit in `ids`, as
/// `get-unit-rules-param-f32` reads it; 0 where it has none.
pub fn get_units_rules_param(
    name: &str,
    ids: &[i32],
    out: &mut [f32],
    found: Option<&mut [u8]>,
) -> Result<usize> {
    let count = ids.len();
    if out.len() != count {
        return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
    }
    let found = flags(found, count)?;
    let (name_pointer, name_bytes) = wasm_slice_parts(name.as_bytes())?;
    let (ids, _) = wasm_slice_parts(ids)?;
    let (out, _) = wasm_mut_slice_parts(out)?;
    unpack(raw::get_units_rules_param(
        name_pointer,
        name_bytes,
        ids,
        count as i32,
        out,
        found,
    ))
}
