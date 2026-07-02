use std::{mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct UnitsQuery<'a> {
    api: &'a sys::UnitsQueryApi,
}

impl<'a> UnitsQuery<'a> {
    pub(crate) fn new(api: &'a sys::UnitsQueryApi) -> Self {
        Self { api }
    }
}

/// Helper trait for ergonomic rectangle construction.
pub trait RectangleQueryExt {
    fn rect(self) -> sys::RectangleQuery;
}

impl RectangleQueryExt for (f32, f32, f32, f32) {
    fn rect(self) -> sys::RectangleQuery {
        let (min_x, min_z, max_x, max_z) = self;
        sys::RectangleQuery {
            minX: min_x,
            minZ: min_z,
            maxX: max_x,
            maxZ: max_z,
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/units_query_generated.rs"));
