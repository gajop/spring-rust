use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct Vfs<'a> {
    api: &'a sys::VFSApi,
}

impl<'a> Vfs<'a> {
    pub(crate) fn new(api: &'a sys::VFSApi) -> Self {
        Self { api }
    }

    #[inline(always)]
    fn get_fn<T>(option: Option<T>, name: &str) -> Result<T, Error> {
        option.ok_or_else(|| Error::unavailable(name))
    }
}

include!(concat!(env!("OUT_DIR"), "/vfs_generated.rs"));
