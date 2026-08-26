// UnitsPieces portion of the Spring Core-Wasm guest SDK.
//
// Script names use the reviewed flat list<string> ABI: one descriptor table
// plus one packed byte blob, both caller-owned and reusable.

#[cfg(feature = "alloc")]
use super::config::{StringListBuffer, StringListRequirements};
use super::config::{decode_string_list_result, mut_slice_parts};
#[cfg(feature = "alloc")]
use super::{ApiError, ErrorCode};
use super::{Result, StringListFill, StringRange};

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:units-pieces")]
    unsafe extern "C" {
        #[link_name = "get-unit-script-names-flat"]
        pub fn get_unit_script_names_flat(
            unit_id: i32,
            descriptor_ptr: i32,
            descriptor_capacity: i32,
            bytes_ptr: i32,
            bytes_capacity: i32,
            meta_ptr: i32,
        ) -> i32;
    }
}

/// Fetch unit script piece names into reusable caller-owned storage.
///
/// On success the returned view borrows `ranges` and `bytes`; no allocation or
/// UTF-8 validation occurs. On `Insufficient`, resize/reuse according to the
/// returned requirements and retry.
#[inline]
pub fn get_unit_script_names_into<'a>(
    unit_id: i32,
    ranges: &'a mut [StringRange],
    bytes: &'a mut [u8],
) -> Result<StringListFill<'a>> {
    #[cfg(target_arch = "wasm32")]
    {
        let (range_ptr, range_capacity) = mut_slice_parts(ranges);
        let (bytes_ptr, bytes_capacity) = mut_slice_parts(bytes);
        let mut meta = [0u32; 2];
        let meta_ptr = meta.as_mut_ptr() as usize as u32 as i32;

        // SAFETY: all pointers refer to live guest-owned buffers for this
        // synchronous import; the host validates each range before writing.
        let status = unsafe {
            raw::get_unit_script_names_flat(
                unit_id,
                range_ptr,
                range_capacity,
                bytes_ptr,
                bytes_capacity,
                meta_ptr,
            )
        };
        decode_string_list_result(status, meta, ranges, bytes)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (unit_id, ranges, bytes);
        Err(unreachable!())
    }
}

/// Fill reusable owned storage with unit script piece names.
///
/// Storage grows only when required and is retained for subsequent calls. The
/// completed names remain in the flat descriptor/blob representation; inspect
/// them through `buffer.view()` without allocating per-string objects.
#[cfg(feature = "alloc")]
pub fn fill_unit_script_names(unit_id: i32, buffer: &mut StringListBuffer) -> Result<()> {
    for _ in 0..3 {
        match get_unit_script_names_into(unit_id, &mut buffer.ranges, &mut buffer.bytes)? {
            StringListFill::Complete(view) => {
                let used = StringListRequirements {
                    strings: view.len(),
                    bytes: view.packed_bytes().len(),
                };
                buffer.commit(used);
                return Ok(());
            }
            StringListFill::Insufficient(required) => buffer.ensure(required),
        }
    }
    Err(ApiError::new(ErrorCode::BufferOverflow as i32))
}
