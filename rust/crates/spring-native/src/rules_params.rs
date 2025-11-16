use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct RulesParams<'a> {
    api: &'a sys::RulesParamsApi,
}

impl<'a> RulesParams<'a> {
    pub(crate) fn new(api: &'a sys::RulesParamsApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/rules_params_generated.rs"));
