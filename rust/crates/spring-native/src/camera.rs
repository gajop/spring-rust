use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct Camera<'a> {
    api: &'a sys::CameraApi,
}

impl<'a> Camera<'a> {
    pub(crate) fn new(api: &'a sys::CameraApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/camera_generated.rs"));
