use std::mem::MaybeUninit;

use crate::{error::Error, sys};

pub struct UnitsWeapons<'a> {
    api: &'a sys::UnitsWeaponsApi,
}

impl<'a> UnitsWeapons<'a> {
    pub(crate) fn new(api: &'a sys::UnitsWeaponsApi) -> Self {
        Self { api }
    }

    #[inline(always)]
    fn get_fn<T>(option: Option<T>, name: &str) -> Result<T, Error> {
        option.ok_or_else(|| Error::unavailable(name))
    }
}

include!(concat!(env!("OUT_DIR"), "/units_weapons_generated.rs"));
