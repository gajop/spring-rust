use std::{mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct UnitsCommands<'a> {
    api: &'a sys::UnitsCommandsApi,
}

impl<'a> UnitsCommands<'a> {
    pub(crate) fn new(api: &'a sys::UnitsCommandsApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/units_commands_generated.rs"));
