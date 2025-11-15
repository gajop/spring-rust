use std::{mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct MoveCtrl<'a> {
    api: &'a sys::MoveCtrlApi,
}

impl<'a> MoveCtrl<'a> {
    pub(crate) fn new(api: &'a sys::MoveCtrlApi) -> Self {
        Self { api }
    }

    #[inline(always)]
    fn get_fn<T>(option: Option<T>, name: &str) -> Result<T, Error> {
        option.ok_or_else(|| Error::unavailable(name))
    }
}

include!(concat!(env!("OUT_DIR"), "/move_ctrl_generated.rs"));
