use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct Camera<'a> {
    api: &'a sys::CameraApi,
}

impl<'a> Camera<'a> {
    pub(crate) fn new(api: &'a sys::CameraApi) -> Self {
        Self { api }
    }

    #[inline(always)]
    fn get_fn<T>(option: Option<T>, name: &str) -> Result<T, Error> {
        option.ok_or_else(|| Error::unavailable(name))
    }
}

include!(concat!(env!("OUT_DIR"), "/camera_generated.rs"));
