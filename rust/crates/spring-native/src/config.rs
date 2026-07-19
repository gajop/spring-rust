use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, raw::copy_c_string, sys};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigValueType {
    Integer,
    Float,
    String,
    Boolean,
    Unknown(i64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigParameter {
    pub name: String,
    pub value_type: ConfigValueType,
    pub description: Option<String>,
    pub default_value: Option<String>,
    pub minimum_value: Option<String>,
    pub maximum_value: Option<String>,
    pub read_only: bool,
}

pub struct Config<'a> {
    api: &'a sys::ConfigApi,
}

impl<'a> Config<'a> {
    pub(crate) fn new(api: &'a sys::ConfigApi) -> Self {
        Self { api }
    }

    pub fn get_config_parameters(&self) -> Result<Vec<ConfigParameter>, Error> {
        self.get_config_params().map(|parameters| {
            parameters
                .into_iter()
                .filter_map(ConfigParameter::from_raw)
                .collect()
        })
    }
}

impl ConfigParameter {
    fn from_raw(parameter: sys::ConfigParam) -> Option<Self> {
        // SAFETY: configuration metadata is engine-owned and valid for this call.
        unsafe {
            Some(Self {
                name: copy_c_string(parameter.name)?,
                value_type: ConfigValueType::from(parameter.type_),
                description: copy_c_string(parameter.description),
                default_value: copy_c_string(parameter.defaultValue),
                minimum_value: copy_c_string(parameter.minimumValue),
                maximum_value: copy_c_string(parameter.maximumValue),
                read_only: parameter.readOnly,
            })
        }
    }
}

impl From<u32> for ConfigValueType {
    fn from(value: u32) -> Self {
        Self::from(i64::from(value))
    }
}

impl From<i32> for ConfigValueType {
    fn from(value: i32) -> Self {
        Self::from(i64::from(value))
    }
}

impl From<i64> for ConfigValueType {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Integer,
            1 => Self::Float,
            2 => Self::String,
            3 => Self::Boolean,
            value => Self::Unknown(value),
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/config_generated.rs"));
