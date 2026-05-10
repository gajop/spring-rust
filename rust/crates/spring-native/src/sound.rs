use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct Sound<'a> {
    api: &'a sys::SoundApi,
}

impl<'a> Sound<'a> {
    pub(crate) fn new(api: &'a sys::SoundApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/sound_generated.rs"));
