use std::{mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct PathFinder<'a> {
    api: &'a sys::PathFinderApi,
}

impl<'a> PathFinder<'a> {
    pub(crate) fn new(api: &'a sys::PathFinderApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/path_finder_generated.rs"));
