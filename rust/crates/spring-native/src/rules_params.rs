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
            sys::RulesParamType_RULESPARAM_TYPE_BOOL => {
                // SAFETY: the native API tags this union value as BOOL.
                RulesParamValue::Bool(unsafe { value.__bindgen_anon_1.boolValue })
            }
            sys::RulesParamType_RULESPARAM_TYPE_STRING => {
                // SAFETY: the native API tags this union value as STRING.
                let ptr = unsafe { value.__bindgen_anon_1.stringValue };
                if ptr.is_null() {
                    RulesParamValue::String(String::new())
                } else {
                    // SAFETY: rules-param strings are returned as NUL-terminated
                    // C strings valid for the duration of the call.
                    RulesParamValue::String(
                        unsafe { CStr::from_ptr(ptr) }
                            .to_string_lossy()
                            .into_owned(),
                    )
                }
            }
            _ => {
                // SAFETY: FLOAT uses this union arm; unknown tags fall back to
                // the same numeric representation for forward compatibility.
                RulesParamValue::Float(unsafe { value.__bindgen_anon_1.floatValue })
            }
        }
    }

    pub(crate) fn to_sys(&self) -> Result<SysRulesParamValue, Error> {
        Ok(match self {
            RulesParamValue::Bool(value) => SysRulesParamValue {
                value: sys::RulesParamValue {
                    type_: sys::RulesParamType_RULESPARAM_TYPE_BOOL,
                    __bindgen_anon_1: sys::RulesParamValue__bindgen_ty_1 { boolValue: *value },
                },
                _string: None,
            },
            RulesParamValue::Float(value) => SysRulesParamValue {
                value: sys::RulesParamValue {
                    type_: sys::RulesParamType_RULESPARAM_TYPE_FLOAT,
                    __bindgen_anon_1: sys::RulesParamValue__bindgen_ty_1 { floatValue: *value },
                },
                _string: None,
            },
            RulesParamValue::String(value) => {
                let string = CString::new(value.as_str())
                    .map_err(|_| Error::invalid_argument("rules-param string value"))?;
                SysRulesParamValue {
                    value: sys::RulesParamValue {
                        type_: sys::RulesParamType_RULESPARAM_TYPE_STRING,
                        __bindgen_anon_1: sys::RulesParamValue__bindgen_ty_1 {
                            stringValue: string.as_ptr(),
                        },
                    },
                    _string: Some(string),
                }
            }
        })
    }
}

include!(concat!(env!("OUT_DIR"), "/rules_params_generated.rs"));
