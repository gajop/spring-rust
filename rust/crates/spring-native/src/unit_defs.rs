use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct UnitDefs<'a> {
    api: &'a sys::UnitDefsApi,
}

impl<'a> UnitDefs<'a> {
    pub(crate) fn new(api: &'a sys::UnitDefsApi) -> Self {
        Self { api }
    }

    #[inline(always)]
    fn get_fn<T>(option: Option<T>, name: &str) -> Result<T, Error> {
        option.ok_or_else(|| Error::unavailable(name))
    }
}

include!(concat!(env!("OUT_DIR"), "/unit_defs_generated.rs"));
