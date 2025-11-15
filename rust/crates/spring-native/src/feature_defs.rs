use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct FeatureDefs<'a> {
    api: &'a sys::FeatureDefsApi,
}

impl<'a> FeatureDefs<'a> {
    pub(crate) fn new(api: &'a sys::FeatureDefsApi) -> Self {
        Self { api }
    }

    #[inline(always)]
    fn get_fn<T>(option: Option<T>, name: &str) -> Result<T, Error> {
        option.ok_or_else(|| Error::unavailable(name))
    }
}

include!(concat!(env!("OUT_DIR"), "/feature_defs_generated.rs"));
