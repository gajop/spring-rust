use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct UnsyncedRead<'a> {
    api: &'a sys::UnsyncedReadApi,
    unit_rendering_api: &'a sys::UnitRenderingApi,
}

impl<'a> UnsyncedRead<'a> {
    pub(crate) fn new(api: &'a sys::UnsyncedReadApi) -> Self {
        unsafe {
            Self {
                api,
                unit_rendering_api: api
                    .unitRendering
                    .as_ref()
                    .expect("unitRendering API must be initialized"),
            }
        }
    }

    /// Access unit rendering state queries (noDraw, luaDraw, draw masks, transforms, icons).
    pub fn unit_rendering(&self) -> UnitRendering<'_> {
        UnitRendering::new(self.unit_rendering_api)
    }
}

pub struct UnitRendering<'a> {
    api: &'a sys::UnitRenderingApi,
}

impl<'a> UnitRendering<'a> {
    pub(crate) fn new(api: &'a sys::UnitRenderingApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/unsynced_read_generated.rs"));
include!(concat!(env!("OUT_DIR"), "/unit_rendering_generated.rs"));
