use std::{mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct Selection<'a> {
    api: &'a sys::SelectionApi,
}

impl<'a> Selection<'a> {
    pub(crate) fn new(api: &'a sys::SelectionApi) -> Self {
        Self { api }
    }

    #[inline(always)]
    fn get_fn<T>(option: Option<T>, name: &str) -> Result<T, Error> {
        option.ok_or_else(|| Error::unavailable(name))
    }
}

include!(concat!(env!("OUT_DIR"), "/selection_generated.rs"));
