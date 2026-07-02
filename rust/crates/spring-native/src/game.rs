use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct Game<'a> {
    api: &'a sys::GameApi,
}

impl<'a> Game<'a> {
    pub(crate) fn new(api: &'a sys::GameApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/game_generated.rs"));
