/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

//! Logging an engine call's error instead of ignoring it with `let _ =`.

use core::cell::UnsafeCell;
use core::panic::Location;

/// Log an error and carry on, instead of `let _ = call();`.
///
/// ```rust,ignore
/// use spring::ResultExt;
/// spring::set_unit_rules_param(unit, "stage", 2.0, los::PUBLIC)
///     .log_err("hud: stage param");
/// let height = spring::get_ground_height(x, z).log_err_once("drill probe");
/// ```
pub trait ResultExt<T> {
    /// Log the error at warning level, with `context`, and return `None`.
    fn log_err(self, context: &str) -> Option<T>;

    /// Like [`log_err`](Self::log_err), but log only the first failure at
    /// this call site.
    #[track_caller]
    fn log_err_once(self, context: &str) -> Option<T>;
}

impl<T, E: core::fmt::Debug> ResultExt<T> for Result<T, E> {
    fn log_err(self, context: &str) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(error) => {
                crate::log_warning!("wasm", "{context}: {error:?}");
                None
            }
        }
    }

    #[track_caller]
    fn log_err_once(self, context: &str) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(error) => {
                if first_failure_at(Location::caller()) {
                    crate::log_warning!("wasm", "{context}: {error:?} (logged once)");
                }
                None
            }
        }
    }
}

struct Sites(UnsafeCell<alloc::vec::Vec<(&'static str, u32, u32)>>);

// SAFETY: Core Wasm guests are single-threaded.
unsafe impl Sync for Sites {}

static FAILED_SITES: Sites = Sites(UnsafeCell::new(alloc::vec::Vec::new()));

fn first_failure_at(location: &'static Location<'static>) -> bool {
    let site = (location.file(), location.line(), location.column());
    // SAFETY: single-threaded, and the reference doesn't outlive this call.
    let sites = unsafe { &mut *FAILED_SITES.0.get() };
    if sites.contains(&site) {
        return false;
    }
    sites.push(site);
    true
}
