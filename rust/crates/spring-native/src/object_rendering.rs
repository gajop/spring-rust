use std::mem::MaybeUninit;

use crate::{error::Error, sys};

pub struct ObjectRendering<'a> {
    api: &'a sys::ObjectRenderingApi,
}

impl<'a> ObjectRendering<'a> {
    pub(crate) fn new(api: &'a sys::ObjectRenderingApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/object_rendering_generated.rs"));
