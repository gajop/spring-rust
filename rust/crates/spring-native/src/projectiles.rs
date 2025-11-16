use std::{mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct Projectiles<'a> {
    api: &'a sys::ProjectilesApi,
}

impl<'a> Projectiles<'a> {
    pub(crate) fn new(api: &'a sys::ProjectilesApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/projectiles_generated.rs"));
