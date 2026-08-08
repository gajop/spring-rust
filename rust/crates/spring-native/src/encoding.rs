use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct Encoding<'a> {
    api: &'a sys::EncodingApi,
}

impl<'a> Encoding<'a> {
    pub(crate) fn new(api: &'a sys::EncodingApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/encoding_generated.rs"));
