//! Guest-side crash reporting.
//!
//! A wasm guest has no stderr. Without help, a Rust panic reaches the engine
//! log only as `wasm trap: unreachable` plus a host backtrace, which says
//! nothing about what the guest was actually doing. Everything here exists to
//! get the panic message, its source location and the active callin stack into
//! the engine log *before* the trap escapes.

use alloc::format;

use crate::log;

/// Report a borrow conflict and then panic with the same text.
///
/// The log call happens first so the diagnostic survives even if the panic hook
/// is unavailable or the trap escapes before the hook runs.
#[cold]
#[inline(never)]
pub fn borrow_conflict(what: &str, detail: &str) -> ! {
    let message = crate::runtime::with_active_callins(|callins| {
        format!(
            "{what} {detail} Active callins (outermost first): {callins:?}. A Spring callout made \
             by an outer callin re-entered the guest while that callin still held the borrow."
        )
    });
    log::error(&message);
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
            crate::runtime::with_active_callins(|callins| {
                log::error(&format!(
                    "guest panic: {info}\n  active callins (outermost first): {callins:?}"
                ));
            });
            previous(info);
        }));
    }
}
