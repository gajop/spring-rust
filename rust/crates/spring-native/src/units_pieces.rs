use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct UnitsPieces<'a> {
    api: &'a sys::UnitsPiecesApi,
}

impl<'a> UnitsPieces<'a> {
    pub(crate) fn new(api: &'a sys::UnitsPiecesApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/units_pieces_generated.rs"));
