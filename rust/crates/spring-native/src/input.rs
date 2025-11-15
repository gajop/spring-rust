use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct Input<'a> {
    api: &'a sys::InputApi,
}

impl<'a> Input<'a> {
    pub(crate) fn new(api: &'a sys::InputApi) -> Self {
        Self { api }
    }

    #[inline(always)]
    fn get_fn<T>(option: Option<T>, name: &str) -> Result<T, Error> {
        option.ok_or_else(|| Error::unavailable(name))
    }
}

include!(concat!(env!("OUT_DIR"), "/input_generated.rs"));
