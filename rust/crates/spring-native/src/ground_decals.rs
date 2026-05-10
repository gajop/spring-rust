use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct GroundDecals<'a> {
    api: &'a sys::GroundDecalsApi,
}

impl<'a> GroundDecals<'a> {
    pub(crate) fn new(api: &'a sys::GroundDecalsApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/ground_decals_generated.rs"));
