use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::slice;

use crate::{error::Error, sys};

pub struct Gfx<'a> {
    api: &'a sys::GfxApi,
}

impl<'a> Gfx<'a> {
    pub(crate) fn new(api: &'a sys::GfxApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/gfx_generated.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    unsafe extern "C" fn mock_upload_texture(
        query: *const sys::GfxUploadTextureQuery,
        result: *mut sys::GfxEmptyResult,
    ) {
        let query = unsafe { &*query };
        assert_eq!(
            unsafe { CStr::from_ptr(query.name) }.to_bytes(),
            b"!native7"
        );
        assert_eq!(query.target, 0);
        assert_eq!(query.level, 2);
        assert_eq!((query.xoff, query.yoff, query.zoff), (3, 4, 5));
        assert_eq!((query.width, query.height, query.depth), (6, 7, 8));
        assert_eq!(query.format, 0x1908);
        assert_eq!(query.pixelType, 0x1401);
        assert_eq!(query.dataSize, 4);
        assert_eq!(
            unsafe { std::slice::from_raw_parts(query.data, 4) },
            &[1, 2, 3, 4]
        );
        unsafe { (*result).error = std::ptr::null() };
    }

    #[test]
    fn upload_texture_passes_a_byte_slice_and_region() {
        let mut api = sys::GfxApi::default();
        api.UploadTexture = Some(mock_upload_texture);
        let gfx = Gfx::new(&api);

        gfx.upload_texture(
            "!native7",
            0,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            0x1908,
            0x1401,
            &[1, 2, 3, 4],
        )
        .expect("mock upload should succeed");
    }
}
