use std::{mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct Player<'a> {
    api: &'a sys::PlayerApi,
}

impl<'a> Player<'a> {
    pub(crate) fn new(api: &'a sys::PlayerApi) -> Self {
        Self { api }
    }

    #[inline(always)]
    fn get_fn<T>(option: Option<T>, name: &str) -> Result<T, Error> {
        option.ok_or_else(|| Error::unavailable(name))
    }
}

include!(concat!(env!("OUT_DIR"), "/player_generated.rs"));
