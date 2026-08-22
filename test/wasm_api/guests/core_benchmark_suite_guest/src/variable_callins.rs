use std::hint::black_box;
use std::slice;

const SCRATCH_BYTES: usize = 4096;
const ADD_CONSOLE_HEADER_BYTES: usize = 20;
const COMMAND_NOTIFY_HEADER_BYTES: usize = 24;

#[repr(align(16))]
struct AlignedScratch([u8; SCRATCH_BYTES]);

static mut SCRATCH: AlignedScratch = AlignedScratch([0; SCRATCH_BYTES]);

#[inline]
fn pack_bool(value: bool) -> i64 {
    value as i64
}

#[inline]
fn read_u32(bytes: &[u8], offset: usize) -> Option<u32> {
    let end = offset.checked_add(4)?;
    let raw: [u8; 4] = bytes.get(offset..end)?.try_into().ok()?;
    Some(u32::from_le_bytes(raw))
}

#[inline]
fn read_i32(bytes: &[u8], offset: usize) -> Option<i32> {
    read_u32(bytes, offset).map(|value| value as i32)
}

#[inline]
fn range(bytes: &[u8], offset: u32, length: u32) -> Option<&[u8]> {
    let start = offset as usize;
    let end = start.checked_add(length as usize)?;
    bytes.get(start..end)
}

#[export_name = "spring:callin/scratch-info"]
pub extern "C" fn scratch_info() -> i64 {
    let pointer = unsafe { core::ptr::addr_of_mut!(SCRATCH.0) as *mut u8 as usize };
    debug_assert!(pointer <= u32::MAX as usize);
    (((SCRATCH_BYTES as u64) << 32) | pointer as u32 as u64) as i64
}

#[cfg(benchmark_callin_consoleline)]
#[export_name = "spring:callin/add-console-line"]
pub extern "C" fn add_console_line(used_bytes: i32) -> i64 {
    if used_bytes < ADD_CONSOLE_HEADER_BYTES as i32 || used_bytes as usize > SCRATCH_BYTES {
        return ((1u64) << 32) as i64;
    }
    let bytes = unsafe {
        slice::from_raw_parts(core::ptr::addr_of!(SCRATCH.0) as *const u8, used_bytes as usize)
    };
    let Some(message_offset) = read_u32(bytes, 0) else { return ((1u64) << 32) as i64; };
    let Some(message_len) = read_u32(bytes, 4) else { return ((1u64) << 32) as i64; };
    let Some(section_offset) = read_u32(bytes, 8) else { return ((1u64) << 32) as i64; };
    let Some(section_len) = read_u32(bytes, 12) else { return ((1u64) << 32) as i64; };
    let Some(level) = read_i32(bytes, 16) else { return ((1u64) << 32) as i64; };
    let Some(message) = range(bytes, message_offset, message_len) else { return ((1u64) << 32) as i64; };
    let Some(section) = range(bytes, section_offset, section_len) else { return ((1u64) << 32) as i64; };
    black_box((message, section, level));
    pack_bool(false)
}

#[cfg(benchmark_callin_commandnotify)]
#[export_name = "spring:callin/command-notify"]
pub extern "C" fn command_notify(used_bytes: i32) -> i64 {
    if used_bytes < COMMAND_NOTIFY_HEADER_BYTES as i32 || used_bytes as usize > SCRATCH_BYTES {
        return ((1u64) << 32) as i64;
    }
    let bytes = unsafe {
        slice::from_raw_parts(core::ptr::addr_of!(SCRATCH.0) as *const u8, used_bytes as usize)
    };
    let Some(id) = read_i32(bytes, 0) else { return ((1u64) << 32) as i64; };
    let Some(timeout) = read_i32(bytes, 4) else { return ((1u64) << 32) as i64; };
    let Some(page_index) = read_u32(bytes, 8) else { return ((1u64) << 32) as i64; };
    let Some(param_count) = read_u32(bytes, 12) else { return ((1u64) << 32) as i64; };
    let Some(tag) = read_u32(bytes, 16) else { return ((1u64) << 32) as i64; };
    let Some(options) = read_u32(bytes, 20) else { return ((1u64) << 32) as i64; };
    let Some(param_bytes) = (param_count as usize).checked_mul(4) else { return ((1u64) << 32) as i64; };
    let Some(params_end) = COMMAND_NOTIFY_HEADER_BYTES.checked_add(param_bytes) else { return ((1u64) << 32) as i64; };
    if params_end > bytes.len() {
        return ((1u64) << 32) as i64;
    }
    // Keep the benchmark on the transport/wire path without allocating an
    // owned Vec. The payload is already little-endian wasm memory; reading a
    // few representative values avoids optimizing the variable body away.
    let first = if param_count == 0 {
        0.0
    } else {
        let raw: [u8; 4] = bytes[COMMAND_NOTIFY_HEADER_BYTES..COMMAND_NOTIFY_HEADER_BYTES + 4]
            .try_into()
            .unwrap();
        f32::from_le_bytes(raw)
    };
    black_box((id, timeout, page_index, param_count, tag, options, first));
    pack_bool(false)
}
