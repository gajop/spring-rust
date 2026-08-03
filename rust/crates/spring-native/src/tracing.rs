use std::{mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct Tracing<'a> {
    api: &'a sys::TracingApi,
}

impl<'a> Tracing<'a> {
    pub(crate) fn new(api: &'a sys::TracingApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/tracing_generated.rs"));
