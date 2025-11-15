use std::mem::MaybeUninit;

use crate::{error::Error, sys};

pub struct Los<'a> {
    api: &'a sys::LOSApi,
}

impl<'a> Los<'a> {
    pub(crate) fn new(api: &'a sys::LOSApi) -> Self {
        Self { api }
    }

    #[inline(always)]
    fn get_fn<T>(option: Option<T>, name: &str) -> Result<T, Error> {
        option.ok_or_else(|| Error::unavailable(name))
    }
}

include!(concat!(env!("OUT_DIR"), "/los_generated.rs"));
