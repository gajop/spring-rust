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
}

include!(concat!(env!("OUT_DIR"), "/unsynced_ctrl_generated.rs"));
