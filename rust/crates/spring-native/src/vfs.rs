use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct Vfs<'a> {
    api: &'a sys::VFSApi,
}

impl<'a> Vfs<'a> {
    pub(crate) fn new(api: &'a sys::VFSApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/vfs_generated.rs"));
