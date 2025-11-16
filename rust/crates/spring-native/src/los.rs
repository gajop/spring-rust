use std::mem::MaybeUninit;

use crate::{error::Error, sys};

pub struct Los<'a> {
    api: &'a sys::LOSApi,
}

impl<'a> Los<'a> {
    pub(crate) fn new(api: &'a sys::LOSApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/los_generated.rs"));
