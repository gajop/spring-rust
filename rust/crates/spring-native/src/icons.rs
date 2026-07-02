use std::{mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct Icons<'a> {
    api: &'a sys::IconsApi,
}

impl<'a> Icons<'a> {
    pub(crate) fn new(api: &'a sys::IconsApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/icons_generated.rs"));
