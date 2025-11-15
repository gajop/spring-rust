use std::mem::MaybeUninit;

use crate::{error::Error, sys};

pub struct Memory<'a> {
    api: &'a sys::MemoryApi,
}

impl<'a> Memory<'a> {
    pub(crate) fn new(api: &'a sys::MemoryApi) -> Self {
        Self { api }
    }

    #[inline(always)]
    fn get_fn<T>(option: Option<T>, name: &str) -> Result<T, Error> {
        option.ok_or_else(|| Error::unavailable(name))
    }
}

include!(concat!(env!("OUT_DIR"), "/memory_generated.rs"));
