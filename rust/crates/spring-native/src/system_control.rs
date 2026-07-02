use std::{ffi::CStr, mem::MaybeUninit};

use crate::{error::Error, sys};

pub struct SystemControl<'a> {
    api: &'a sys::SystemControlApi,
}

impl<'a> SystemControl<'a> {
    pub(crate) fn new(api: &'a sys::SystemControlApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/system_control_generated.rs"));
