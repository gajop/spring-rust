use std::mem::MaybeUninit;

use crate::{error::Error, sys};

pub struct Lights<'a> {
    api: &'a sys::LightsApi,
}

impl<'a> Lights<'a> {
    pub(crate) fn new(api: &'a sys::LightsApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/lights_generated.rs"));
