use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct Teams<'a> {
    api: &'a sys::TeamsApi,
}

impl<'a> Teams<'a> {
    pub(crate) fn new(api: &'a sys::TeamsApi) -> Self {
        Self { api }
    }

    #[inline(always)]
    fn get_fn<T>(option: Option<T>, name: &str) -> Result<T, Error> {
        option.ok_or_else(|| Error::unavailable(name))
    }
}

include!(concat!(env!("OUT_DIR"), "/teams_generated.rs"));
