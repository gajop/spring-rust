use std::mem::MaybeUninit;

use crate::{error::Error, sys};

pub struct Tracing<'a> {
    api: &'a sys::TracingApi,
}

impl<'a> Tracing<'a> {
    pub(crate) fn new(api: &'a sys::TracingApi) -> Self {
        Self { api }
    }

    #[inline(always)]
    fn get_fn<T>(option: Option<T>, name: &str) -> Result<T, Error> {
        option.ok_or_else(|| Error::unavailable(name))
    }
}

include!(concat!(env!("OUT_DIR"), "/tracing_generated.rs"));
