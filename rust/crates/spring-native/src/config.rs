use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct Config<'a> {
    api: &'a sys::ConfigApi,
}

impl<'a> Config<'a> {
    pub(crate) fn new(api: &'a sys::ConfigApi) -> Self {
        Self { api }
    }

    #[inline(always)]
    fn get_fn<T>(option: Option<T>, name: &str) -> Result<T, Error> {
        option.ok_or_else(|| Error::unavailable(name))
    }
}

include!(concat!(env!("OUT_DIR"), "/config_generated.rs"));
