use std::{mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct Projectiles<'a> {
    api: &'a sys::ProjectilesApi,
}

impl<'a> Projectiles<'a> {
    pub(crate) fn new(api: &'a sys::ProjectilesApi) -> Self {
        Self { api }
    }

    #[inline(always)]
    fn get_fn<T>(option: Option<T>, name: &str) -> Result<T, Error> {
        option.ok_or_else(|| Error::unavailable(name))
    }
}

include!(concat!(env!("OUT_DIR"), "/projectiles_generated.rs"));
