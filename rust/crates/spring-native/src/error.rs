use std::ffi::CStr;

use crate::sys;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    pub code: i32,
    pub message: Option<String>,
}

impl Error {
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: Some(message.into()),
        }
    }

    pub(crate) fn from_ptr(ptr: *const sys::Error) -> Option<Self> {
        if ptr.is_null() {
            return None;
        }
        unsafe {
            let raw = &*ptr;
            let message = if raw.message.is_null() {
                None
            } else {
                Some(CStr::from_ptr(raw.message).to_string_lossy().into_owned())
            };
            Some(Self {
                code: raw.code,
                message,
            })
        }
    }

    pub(crate) fn result_or<T>(ptr: *const sys::Error, value: T) -> Result<T, Self> {
        match Self::from_ptr(ptr) {
            Some(err) => Err(err),
            None => Ok(value),
        }
    }

    pub(crate) fn invalid_argument(name: &str) -> Self {
        // Mirrors CommonErrorCode::ERROR_INVALID_ARGUMENT
        const ERROR_INVALID_ARGUMENT: i32 = 1;
        Self::new(
            ERROR_INVALID_ARGUMENT,
            format!("Invalid argument for parameter `{}`", name),
        )
    }

    /// Get the error code.
    pub fn code(&self) -> i32 {
        self.code
    }

    /// Get the error message as a string slice.
    pub fn message(&self) -> &str {
        self.message.as_deref().unwrap_or("")
    }
}
