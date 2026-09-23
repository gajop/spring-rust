use std::ffi::CStr;
use std::mem::MaybeUninit;

use crate::{error::Error, sys};

pub struct UnsyncedCtrl<'a> {
    api: &'a sys::UnsyncedCtrlApi,
}

impl<'a> UnsyncedCtrl<'a> {
    pub(crate) fn new(api: &'a sys::UnsyncedCtrlApi) -> Self {
        Self { api }
    }

    /// The unsynced RNG behind unsynced Lua's `math.random`, for local effects
    pub fn random(&self) -> UnsyncedRandom<'_> {
        UnsyncedRandom::new(
            unsafe { self.api.random.as_ref() }.expect("random API must be initialized"),
        )
    }
}

include!(concat!(env!("OUT_DIR"), "/unsynced_ctrl_generated.rs"));

pub struct UnsyncedRandom<'a> {
    api: &'a sys::UnsyncedRandomApi,
}

impl<'a> UnsyncedRandom<'a> {
    pub(crate) fn new(api: &'a sys::UnsyncedRandomApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/unsynced_random_generated.rs"));
