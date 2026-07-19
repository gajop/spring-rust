use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, raw::copy_c_string, sys};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitDefParamType {
    Missing,
    Integer,
    Boolean,
    Float,
    String,
    Table,
    Unknown(i32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitDefParamKey {
    pub name: String,
    pub value_type: UnitDefParamType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitDefBasicInfo {
    pub id: i32,
    pub name: String,
    pub human_name: String,
    pub tooltip: String,
    pub unit_def_id: i32,
}

pub struct UnitDefs<'a> {
    api: &'a sys::UnitDefsApi,
}

impl<'a> UnitDefs<'a> {
    pub(crate) fn new(api: &'a sys::UnitDefsApi) -> Self {
        Self { api }
    }

    pub fn get_unit_def_basic_info(
        &self,
        unit_def_id: i32,
    ) -> Result<Option<UnitDefBasicInfo>, Error> {
        self.get_unit_def_by_id(unit_def_id)
            .map(|(exists, info, ..)| exists.then(|| UnitDefBasicInfo::from_raw(info)))
    }

    pub fn get_unit_def_parameter_keys(&self) -> Result<Vec<UnitDefParamKey>, Error> {
        self.get_unit_def_param_keys().map(|keys| {
            keys.into_iter()
                .filter_map(UnitDefParamKey::from_raw)
                .collect()
        })
    }
}

impl UnitDefBasicInfo {
    fn from_raw(info: sys::UnitDefBasicInfo) -> Self {
        // SAFETY: unit definition strings are engine-owned and valid for this call.
        unsafe {
            Self {
                id: info.id,
                name: copy_c_string(info.name).unwrap_or_default(),
                human_name: copy_c_string(info.humanName).unwrap_or_default(),
                tooltip: copy_c_string(info.tooltip).unwrap_or_default(),
                unit_def_id: info.unitDefID,
            }
        }
    }
}

impl UnitDefParamKey {
    fn from_raw(key: sys::UnitDefParamKey) -> Option<Self> {
        // SAFETY: parameter metadata is engine-owned and valid for this call.
        unsafe {
            Some(Self {
                name: copy_c_string(key.name)?,
                value_type: UnitDefParamType::from(key.type_),
            })
        }
    }
}

impl From<i32> for UnitDefParamType {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Missing,
            1 => Self::Integer,
            2 => Self::Boolean,
            3 => Self::Float,
            4 => Self::String,
            5 => Self::Table,
            value => Self::Unknown(value),
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/unit_defs_generated.rs"));
