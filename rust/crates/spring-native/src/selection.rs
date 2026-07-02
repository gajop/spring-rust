use std::{mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct Selection<'a> {
    api: &'a sys::SelectionApi,
}

impl<'a> Selection<'a> {
    pub(crate) fn new(api: &'a sys::SelectionApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/selection_generated.rs"));
