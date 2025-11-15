use std::mem::MaybeUninit;

use crate::{error::Error, sys};

pub struct Sound<'a> {
    api: &'a sys::SoundApi,
}

impl<'a> Sound<'a> {
    pub(crate) fn new(api: &'a sys::SoundApi) -> Self {
        Self { api }
    }

    #[inline(always)]
    fn get_fn<T>(option: Option<T>, name: &str) -> Result<T, Error> {
        option.ok_or_else(|| Error::unavailable(name))
    }
}

include!(concat!(env!("OUT_DIR"), "/sound_generated.rs"));
