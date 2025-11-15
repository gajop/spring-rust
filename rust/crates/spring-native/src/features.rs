use std::{mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct Features<'a> {
    api: &'a sys::FeaturesApi,
}

impl<'a> Features<'a> {
    pub(crate) fn new(api: &'a sys::FeaturesApi) -> Self {
        Self { api }
    }

    #[inline(always)]
    fn get_fn<T>(option: Option<T>, name: &str) -> Result<T, Error> {
        option.ok_or_else(|| Error::unavailable(name))
    }
}

include!(concat!(env!("OUT_DIR"), "/features_generated.rs"));
