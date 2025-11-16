use std::mem::MaybeUninit;

use crate::{error::Error, sys};

pub struct Utils<'a> {
    api: &'a sys::UtilsApi,
}

impl<'a> Utils<'a> {
    pub(crate) fn new(api: &'a sys::UtilsApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/utils_generated.rs"));
