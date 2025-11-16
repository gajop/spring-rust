use std::mem::MaybeUninit;

use crate::{error::Error, sys};

pub struct Memory<'a> {
    api: &'a sys::MemoryApi,
}

impl<'a> Memory<'a> {
    pub(crate) fn new(api: &'a sys::MemoryApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/memory_generated.rs"));
