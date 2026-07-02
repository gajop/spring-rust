use std::mem::MaybeUninit;

use crate::{error::Error, sys};

pub struct MathExtra<'a> {
    api: &'a sys::MathExtraApi,
}

impl<'a> MathExtra<'a> {
    pub(crate) fn new(api: &'a sys::MathExtraApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/math_extra_generated.rs"));
