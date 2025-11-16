use std::mem::MaybeUninit;

use crate::{error::Error, sys};

pub struct Display<'a> {
    api: &'a sys::DisplayApi,
}

impl<'a> Display<'a> {
    pub(crate) fn new(api: &'a sys::DisplayApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/display_generated.rs"));
