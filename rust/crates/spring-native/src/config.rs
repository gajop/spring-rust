use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct Config<'a> {
    api: &'a sys::ConfigApi,
}

impl<'a> Config<'a> {
    pub(crate) fn new(api: &'a sys::ConfigApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/config_generated.rs"));
