use std::mem::MaybeUninit;

use crate::{error::Error, sys};

pub struct MetalMap<'a> {
    api: &'a sys::MetalMapApi,
}

impl<'a> MetalMap<'a> {
    pub(crate) fn new(api: &'a sys::MetalMapApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/metal_map_generated.rs"));
