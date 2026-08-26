//! Internal conversions for values owned by the engine.

use std::ffi::{CStr, c_char};

/// Copy an engine-owned NUL-terminated string.
///
/// # Safety
///
/// A non-null `raw` pointer must reference a valid NUL-terminated byte string
/// for the duration of this call.
pub(crate) unsafe fn copy_c_string(raw: *const c_char) -> Option<String> {
    if raw.is_null() {
        return None;
    }
    Some(
        unsafe { CStr::from_ptr(raw) }
            .to_string_lossy()
            .into_owned(),
    )
}
