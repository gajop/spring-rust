use std::mem::MaybeUninit;

use crate::{error::Error, sys};

pub struct Markers<'a> {
    api: &'a sys::MarkersApi,
}

impl<'a> Markers<'a> {
    pub(crate) fn new(api: &'a sys::MarkersApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/markers_generated.rs"));
