use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct Profiling<'a> {
    api: &'a sys::ProfilingApi,
}

impl<'a> Profiling<'a> {
    pub(crate) fn new(api: &'a sys::ProfilingApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/profiling_generated.rs"));
