use std::mem::MaybeUninit;

use crate::{error::Error, sys};

pub struct DebugInput<'a> {
    api: &'a sys::DebugInputApi,
}

impl<'a> DebugInput<'a> {
    pub(crate) fn new(api: &'a sys::DebugInputApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/debug_input_generated.rs"));
