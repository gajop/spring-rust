//! Guest-side crash reporting.
//!
//! A wasm guest has no stderr. Without help, a Rust panic reaches the engine
//! log only as `wasm trap: unreachable` plus a host backtrace, which says
//! nothing about what the guest was actually doing. Everything here exists to
//! get the panic message, its source location and the active callin stack into
//! the engine log *before* the trap escapes.

use alloc::format;

const SECTION: &str = "spring-addons";

/// Spring's `LOG_LEVEL_ERROR`.
const ERROR: i32 = 50;

/// Write a line to the engine log at error level.
///
/// Deliberately infallible: this runs on paths that are already failing, so a
/// failed log must never itself panic.
pub fn log_error(message: &str) {
    #[cfg(target_arch = "wasm32")]
    let _ = spring::log(SECTION, ERROR, message);
    #[cfg(not(target_arch = "wasm32"))]
    let _ = message;
}

/// Report a borrow conflict and then panic with the same text.
///
/// The log call happens first so the diagnostic survives even if the panic hook
/// is unavailable or the trap escapes before the hook runs.
#[cold]
#[inline(never)]
pub fn borrow_conflict(what: &str, detail: &str) -> ! {
    let message = format!(
        "{what} {detail} Active callins (outermost first): {:?}. A Spring callout made by an \
         outer callin re-entered the guest while that callin still held the borrow.",
        crate::runtime::active_callins()
    );
    log_error(&message);
    panic!("{message}");
}

/// Install a process-global panic hook that logs to the engine.
///
/// Idempotent, and safe to call from every handler entry point. On targets
/// without `std` this is a no-op.
pub fn install_panic_hook() {
    #[cfg(feature = "std")]
    {
        use core::sync::atomic::{AtomicBool, Ordering};
        static INSTALLED: AtomicBool = AtomicBool::new(false);
        if INSTALLED.swap(true, Ordering::Relaxed) {
            return;
        }
        let previous = std::panic::take_hook();
        std::panic::set_hook(std::boxed::Box::new(move |info| {
            log_error(&format!(
                "guest panic: {info}\n  active callins (outermost first): {:?}",
                crate::runtime::active_callins()
            ));
            previous(info);
        }));
    }
}
