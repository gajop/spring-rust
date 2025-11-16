use std::{mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct Player<'a> {
    api: &'a sys::PlayerApi,
}

impl<'a> Player<'a> {
    pub(crate) fn new(api: &'a sys::PlayerApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/player_generated.rs"));
