// Config portion of the Spring Core-Wasm guest SDK.
//
// `list<string>` is represented as one descriptor table plus one packed byte
// blob. The performance API is caller-owned and allocation-free; it never
// materializes a Vec<String>.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(C)]
pub struct StringRange {
    pub offset: u32,
    pub len: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringListRequirements {
    pub strings: usize,
    pub bytes: usize,
}

#[derive(Debug)]
pub enum StringListFill<'a> {
    Complete(StringListView<'a>),
    Insufficient(StringListRequirements),
}

#[derive(Debug, Clone, Copy)]
pub struct StringListView<'a> {
    ranges: &'a [StringRange],
    bytes: &'a [u8],
}

impl<'a> StringListView<'a> {
    #[inline]
    pub fn len(&self) -> usize {
        self.ranges.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    #[inline]
    pub fn packed_bytes(&self) -> &'a [u8] {
        self.bytes
    }

    #[inline]
    pub fn ranges(&self) -> &'a [StringRange] {
        self.ranges
    }

    #[inline]
    pub fn get_bytes(&self, index: usize) -> Option<&'a [u8]> {
        let range = *self.ranges.get(index)?;
        let start = range.offset as usize;
        let end = start.checked_add(range.len as usize)?;
        self.bytes.get(start..end)
    }

    /// Interpret one item as UTF-8. Validation is deliberately per-use rather
    /// than imposed on the transport hot path.
    #[inline]
    pub fn get_str(
        &self,
        index: usize,
    ) -> Option<core::result::Result<&'a str, core::str::Utf8Error>> {
        Some(core::str::from_utf8(self.get_bytes(index)?))
    }

    #[inline]
    pub fn iter_bytes(&self) -> impl ExactSizeIterator<Item = &'a [u8]> + '_ {
        (0..self.len()).map(move |index| self.get_bytes(index).unwrap_or(&[]))
    }
}

/// Reusable owned storage for the flat `list<string>` ABI.
///
/// The vectors retain their high-water sizes across fills. After the first
/// sufficiently large call, repeated calls reuse the same descriptor and byte
/// buffers and `view()` remains allocation-free.
#[cfg(feature = "alloc")]
#[derive(Debug, Default)]
pub struct StringListBuffer {
    pub(crate) ranges: Vec<StringRange>,
    pub(crate) bytes: Vec<u8>,
    used_strings: usize,
    used_bytes: usize,
}

#[cfg(feature = "alloc")]
impl StringListBuffer {
    #[inline]
    pub const fn new() -> Self {
        Self {
            ranges: Vec::new(),
            bytes: Vec::new(),
            used_strings: 0,
            used_bytes: 0,
        }
    }

    #[inline]
    pub fn with_sizes(strings: usize, bytes: usize) -> Self {
        Self {
            ranges: vec![StringRange::default(); strings],
            bytes: vec![0; bytes],
            used_strings: 0,
            used_bytes: 0,
        }
    }

    #[inline]
    pub fn view(&self) -> StringListView<'_> {
        StringListView {
            ranges: &self.ranges[..self.used_strings],
            bytes: &self.bytes[..self.used_bytes],
        }
    }

    #[inline]
    pub fn storage_sizes(&self) -> StringListRequirements {
        StringListRequirements {
            strings: self.ranges.len(),
            bytes: self.bytes.len(),
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.used_strings = 0;
        self.used_bytes = 0;
    }

    #[inline]
    pub(crate) fn ensure(&mut self, required: StringListRequirements) {
        if self.ranges.len() < required.strings {
            self.ranges.resize(required.strings, StringRange::default());
        }
        if self.bytes.len() < required.bytes {
            self.bytes.resize(required.bytes, 0);
        }
    }

    #[inline]
    pub(crate) fn commit(&mut self, required: StringListRequirements) {
        self.used_strings = required.strings;
        self.used_bytes = required.bytes;
    }
}

#[inline]
pub(crate) fn mut_slice_parts<T>(slice: &mut [T]) -> (i32, i32) {
    if slice.is_empty() {
        return (0, 0);
    }
    let pointer = slice.as_mut_ptr() as usize;
    debug_assert!(pointer <= u32::MAX as usize);
    debug_assert!(slice.len() <= u32::MAX as usize);
    (pointer as u32 as i32, slice.len() as u32 as i32)
}

#[inline]
pub(crate) fn decode_string_list_result<'a>(
    status: i32,
    meta: [u32; 2],
    ranges: &'a mut [StringRange],
    bytes: &'a mut [u8],
) -> Result<StringListFill<'a>> {
    let required = StringListRequirements {
        strings: meta[0] as usize,
        bytes: meta[1] as usize,
    };
    if status == ErrorCode::BufferOverflow as i32 {
        return Ok(StringListFill::Insufficient(required));
    }
    if status != 0 {
        return Err(ApiError::new(status));
    }
    if required.strings > ranges.len() || required.bytes > bytes.len() {
        return Err(ApiError::new(ErrorCode::Internal as i32));
    }

    let used_ranges = &ranges[..required.strings];
    let used_bytes = &bytes[..required.bytes];
    for range in used_ranges {
        let start = range.offset as usize;
        let Some(end) = start.checked_add(range.len as usize) else {
            return Err(ApiError::new(ErrorCode::Internal as i32));
        };
        if end > used_bytes.len() {
            return Err(ApiError::new(ErrorCode::Internal as i32));
        }
    }

    Ok(StringListFill::Complete(StringListView {
        ranges: used_ranges,
        bytes: used_bytes,
    }))
}

#[cfg(target_arch = "wasm32")]
mod config_raw {
    #[link(wasm_import_module = "spring:config")]
    extern "C" {
        #[link_name = "get-log-sections-flat"]
        pub fn get_log_sections_flat(
            descriptor_ptr: i32,
            descriptor_capacity: i32,
            bytes_ptr: i32,
            bytes_capacity: i32,
            meta_ptr: i32,
        ) -> i32;
    }
}

/// Fetch registered log sections into reusable caller-owned storage.
#[inline]
pub fn get_log_sections_into<'a>(
    ranges: &'a mut [StringRange],
    bytes: &'a mut [u8],
) -> Result<StringListFill<'a>> {
    #[cfg(target_arch = "wasm32")]
    {
        let (range_ptr, range_capacity) = mut_slice_parts(ranges);
        let (bytes_ptr, bytes_capacity) = mut_slice_parts(bytes);
        let mut meta = [0u32; 2];
        let meta_ptr = meta.as_mut_ptr() as usize as u32 as i32;

        // SAFETY: all pointers refer to live guest slices/stack storage for the
        // synchronous import. The host validates every advertised range before
        // writing and returns required sizes through `meta`.
        let status = unsafe {
            config_raw::get_log_sections_flat(
                range_ptr,
                range_capacity,
                bytes_ptr,
                bytes_capacity,
                meta_ptr,
            )
        };
        return decode_string_list_result(status, meta, ranges, bytes);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (ranges, bytes);
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

/// Fill reusable owned storage with registered log sections.
///
/// Storage grows only when the host reports a larger requirement and is then
/// retained for subsequent calls. Use `buffer.view()` to inspect the result.
#[cfg(feature = "alloc")]
pub fn fill_log_sections(buffer: &mut StringListBuffer) -> Result<()> {
    for _ in 0..3 {
        match get_log_sections_into(&mut buffer.ranges, &mut buffer.bytes)? {
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
