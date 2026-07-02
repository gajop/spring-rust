use std::{mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct Features<'a> {
    api: &'a sys::FeaturesApi,
}

impl<'a> Features<'a> {
    pub(crate) fn new(api: &'a sys::FeaturesApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/features_generated.rs"));
