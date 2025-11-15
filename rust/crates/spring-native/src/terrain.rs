use std::{ffi::CStr, mem::MaybeUninit};

use crate::{error::Error, sys};

pub struct Terrain<'a> {
    api: &'a sys::TerrainApi,
}

impl<'a> Terrain<'a> {
    pub(crate) fn new(api: &'a sys::TerrainApi) -> Self {
        Self { api }
    }

    #[inline(always)]
    fn get_fn<T>(option: Option<T>, name: &str) -> Result<T, Error> {
        option.ok_or_else(|| Error::unavailable(name))
    }
}

include!(concat!(env!("OUT_DIR"), "/terrain_generated.rs"));
