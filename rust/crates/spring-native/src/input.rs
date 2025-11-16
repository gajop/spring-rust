use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct Input<'a> {
    api: &'a sys::InputApi,
}

impl<'a> Input<'a> {
    pub(crate) fn new(api: &'a sys::InputApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/input_generated.rs"));
