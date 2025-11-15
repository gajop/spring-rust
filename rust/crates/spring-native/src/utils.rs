use std::mem::MaybeUninit;

use crate::{error::Error, sys};

pub struct Utils<'a> {
    api: &'a sys::UtilsApi,
}

impl<'a> Utils<'a> {
    pub(crate) fn new(api: &'a sys::UtilsApi) -> Self {
        Self { api }
    }

    #[inline(always)]
    fn get_fn<T>(option: Option<T>, name: &str) -> Result<T, Error> {
        option.ok_or_else(|| Error::unavailable(name))
    }
}

include!(concat!(env!("OUT_DIR"), "/utils_generated.rs"));
