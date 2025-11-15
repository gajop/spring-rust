use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct Messages<'a> {
    api: &'a sys::MessagesApi,
}

impl<'a> Messages<'a> {
    pub(crate) fn new(api: &'a sys::MessagesApi) -> Self {
        Self { api }
    }

    #[inline(always)]
    fn get_fn<T>(option: Option<T>, name: &str) -> Result<T, Error> {
        option.ok_or_else(|| Error::unavailable(name))
    }
}

include!(concat!(env!("OUT_DIR"), "/messages_generated.rs"));
