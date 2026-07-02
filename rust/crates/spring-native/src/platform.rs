use std::{ffi::CStr, mem::MaybeUninit};

use crate::{error::Error, sys};

pub struct Platform<'a> {
    api: &'a sys::PlatformApi,
}

impl<'a> Platform<'a> {
    pub(crate) fn new(api: &'a sys::PlatformApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/platform_generated.rs"));
