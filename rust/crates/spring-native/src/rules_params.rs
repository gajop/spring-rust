use std::{
    ffi::{CStr, CString},
    mem::MaybeUninit,
    slice,
};

use crate::{error::Error, sys};

#[derive(Debug, Clone, PartialEq)]
pub enum RulesParamValue {
    Bool(bool),
    Float(f32),
    String(String),
}

pub struct SysRulesParamValue {
    pub(crate) value: sys::RulesParamValue,
    _string: Option<CString>,
}

pub struct RulesParams<'a> {
    api: &'a sys::RulesParamsApi,
}

impl<'a> RulesParams<'a> {
    pub(crate) fn new(api: &'a sys::RulesParamsApi) -> Self {
        Self { api }
    }
}

impl RulesParamValue {
    pub(crate) fn from_sys(value: sys::RulesParamValue) -> Self {
        match value.type_ {
            sys::RulesParamType_RULESPARAM_TYPE_BOOL => RulesParamValue::Bool(value.boolValue),
            sys::RulesParamType_RULESPARAM_TYPE_STRING => {
                if value.stringValue.is_null() {
                    RulesParamValue::String(String::new())
                } else {
                    // SAFETY: rules-param strings are returned as NUL-terminated
                    // C strings valid for the duration of the call.
                    RulesParamValue::String(
                        unsafe { CStr::from_ptr(value.stringValue) }
                            .to_string_lossy()
                            .into_owned(),
                    )
                }
            }
            _ => RulesParamValue::Float(value.floatValue),
        }
    }

    pub(crate) fn to_sys(&self) -> Result<SysRulesParamValue, Error> {
        Ok(match self {
            RulesParamValue::Bool(value) => SysRulesParamValue {
                value: sys::RulesParamValue {
                    boolValue: *value,
                    ..empty_sys(sys::RulesParamType_RULESPARAM_TYPE_BOOL)
                },
                _string: None,
            },
            RulesParamValue::Float(value) => SysRulesParamValue {
                value: sys::RulesParamValue {
                    floatValue: *value,
                    ..empty_sys(sys::RulesParamType_RULESPARAM_TYPE_FLOAT)
                },
                _string: None,
            },
            RulesParamValue::String(value) => {
                let string = CString::new(value.as_str())
                    .map_err(|_| Error::invalid_argument("rules-param string value"))?;
                SysRulesParamValue {
                    value: sys::RulesParamValue {
                        stringValue: string.as_ptr(),
                        ..empty_sys(sys::RulesParamType_RULESPARAM_TYPE_STRING)
                    },
                    _string: Some(string),
                }
            }
        })
    }
}

fn empty_sys(type_: sys::RulesParamType) -> sys::RulesParamValue {
    sys::RulesParamValue {
        type_,
        boolValue: false,
        floatValue: 0.0,
        stringValue: std::ptr::null(),
    }
}

include!(concat!(env!("OUT_DIR"), "/rules_params_generated.rs"));
