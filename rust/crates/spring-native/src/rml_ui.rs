use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct RmlUi<'a> {
    api: &'a sys::RmlUiApi,
}

impl<'a> RmlUi<'a> {
    pub(crate) fn new(api: &'a sys::RmlUiApi) -> Self {
        Self { api }
    }

    #[allow(non_snake_case)]
    pub fn sol_lua_data_model___set_dirty(
        &self,
        data_model_handle: u64,
        property: &str,
    ) -> Result<bool, Error> {
        self.sol_lua_data_model_set_dirty(data_model_handle, property)
    }
}

include!(concat!(env!("OUT_DIR"), "/rml_ui_generated.rs"));
