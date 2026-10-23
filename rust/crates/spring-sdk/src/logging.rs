/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use crate::{Spring, capabilities::Common};
use core::{fmt::Debug, panic::Location};
use std::sync::Mutex;

pub trait ResultExt<T> {
    fn log_err<E: Common>(self, spring: &Spring<'_, E>, context: &str) -> Option<T>;
    #[track_caller]
    fn log_err_once<E: Common>(self, spring: &Spring<'_, E>, context: &str) -> Option<T>;
    /// Like [`log_err`](Self::log_err), with the context built only on failure: for calls
    /// made every frame or for every unit.
    fn log_err_with<E: Common, C: core::fmt::Display>(
        self,
        spring: &Spring<'_, E>,
        context: impl FnOnce() -> C,
    ) -> Option<T>;
    /// Like [`log_err_once`](Self::log_err_once), with the context built only on the first
    /// failure at this call site.
    #[track_caller]
    fn log_err_once_with<E: Common, C: core::fmt::Display>(
        self,
        spring: &Spring<'_, E>,
        context: impl FnOnce() -> C,
    ) -> Option<T>;
}
impl<T, Error: Debug> ResultExt<T> for core::result::Result<T, Error> {
    fn log_err<E: Common>(self, spring: &Spring<'_, E>, context: &str) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(error) => {
                crate::log_warning!(spring, "rust", "{context}: {error:?}");
                None
            }
        }
    }
    #[track_caller]
    fn log_err_once<E: Common>(self, spring: &Spring<'_, E>, context: &str) -> Option<T> {
        self.log_err_once_with(spring, || context)
    }
    fn log_err_with<E: Common, C: core::fmt::Display>(
        self,
        spring: &Spring<'_, E>,
        context: impl FnOnce() -> C,
    ) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(error) => {
                crate::log_warning!(spring, "rust", "{}: {error:?}", context());
                None
            }
        }
    }
    #[track_caller]
    fn log_err_once_with<E: Common, C: core::fmt::Display>(
        self,
        spring: &Spring<'_, E>,
        context: impl FnOnce() -> C,
    ) -> Option<T> {
        static FAILED: Mutex<Vec<(&'static str, u32, u32)>> = Mutex::new(Vec::new());
        match self {
            Ok(value) => Some(value),
            Err(error) => {
                let location = Location::caller();
                let site = (location.file(), location.line(), location.column());
                let first = {
                    let mut failed = FAILED.lock().unwrap_or_else(|p| p.into_inner());
                    if failed.contains(&site) {
                        false
                    } else {
                        failed.push(site);
                        true
                    }
                };
                if first {
                    crate::log_warning!(spring, "rust", "{}: {error:?} (logged once)", context());
                }
                None
            }
        }
    }
}
#[macro_export]
macro_rules! log_at {
    ($spring:expr, $level:expr, $section:expr, $($arg:tt)+) => {{
        let _ = $spring.messages().log($section, $level, &$crate::__format!($($arg)+));
    }};
}
#[macro_export]
macro_rules! log_once {
    ($spring:expr, $level:expr, $section:expr, $($arg:tt)+) => {{
        static LOGGED: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);
        if !LOGGED.swap(true, core::sync::atomic::Ordering::Relaxed) {
            $crate::log_at!($spring, $level, $section, $($arg)+);
        }
    }};
}

#[macro_export]
macro_rules! log_debug {
    ($spring:expr, $section:expr, $($arg:tt)+) => {
        $crate::log_at!($spring, $crate::log_level::DEBUG, $section, $($arg)+)
    };
}

#[macro_export]
macro_rules! log_info {
    ($spring:expr, $section:expr, $($arg:tt)+) => {
        $crate::log_at!($spring, $crate::log_level::INFO, $section, $($arg)+)
    };
}

#[macro_export]
macro_rules! log_notice {
    ($spring:expr, $section:expr, $($arg:tt)+) => {
        $crate::log_at!($spring, $crate::log_level::NOTICE, $section, $($arg)+)
    };
}

#[macro_export]
macro_rules! log_warning {
    ($spring:expr, $section:expr, $($arg:tt)+) => {
        $crate::log_at!($spring, $crate::log_level::WARNING, $section, $($arg)+)
    };
}

#[macro_export]
macro_rules! log_error {
    ($spring:expr, $section:expr, $($arg:tt)+) => {
        $crate::log_at!($spring, $crate::log_level::ERROR, $section, $($arg)+)
    };
}
