use std::{mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct UnitsCommands<'a> {
    api: &'a sys::UnitsCommandsApi,
}

impl<'a> UnitsCommands<'a> {
    pub(crate) fn new(api: &'a sys::UnitsCommandsApi) -> Self {
        Self { api }
    }

    #[inline(always)]
    fn get_fn<T>(option: Option<T>, name: &str) -> Result<T, Error> {
        option.ok_or_else(|| Error::unavailable(name))
    }
}

include!(concat!(env!("OUT_DIR"), "/units_commands_generated.rs"));
