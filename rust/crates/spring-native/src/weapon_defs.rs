use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct WeaponDefs<'a> {
    api: &'a sys::WeaponDefsApi,
}

impl<'a> WeaponDefs<'a> {
    pub(crate) fn new(api: &'a sys::WeaponDefsApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/weapon_defs_generated.rs"));
