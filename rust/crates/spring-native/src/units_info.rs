use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct UnitsInfo<'a> {
    api: &'a sys::UnitsInfoApi,
}

impl<'a> UnitsInfo<'a> {
    pub(crate) fn new(api: &'a sys::UnitsInfoApi) -> Self {
        Self { api }
    }

    #[inline(always)]
    fn get_fn<T>(option: Option<T>, name: &str) -> Result<T, Error> {
        option.ok_or_else(|| Error::unavailable(name))
    }
}

include!(concat!(env!("OUT_DIR"), "/units_info_generated.rs"));
