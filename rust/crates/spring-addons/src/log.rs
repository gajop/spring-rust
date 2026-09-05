//! Engine logging that is safe to call from any target.
//!
//! `spring`'s message imports only exist on wasm32, so every call has to be
//! target-gated. Doing that once here keeps the rest of the crate portable and
//! lets it compile and run its unit tests on the host.

const SECTION: &str = "spring-addons";

/// Spring's `LOG_LEVEL_WARNING`.
const WARNING: i32 = 40;

/// Spring's `LOG_LEVEL_ERROR`.
const ERROR: i32 = 50;

/// Write a line to the engine log at error level.
///
/// Deliberately infallible: this runs on paths that are already failing, so a
/// failed log must never itself panic.
pub fn error(message: &str) {
    write(ERROR, message);
}

/// Write a line to the engine log at warning level.
pub fn warning(message: &str) {
    write(WARNING, message);
}

fn write(level: i32, message: &str) {
    #[cfg(target_arch = "wasm32")]
    let _ = spring::log(SECTION, level, message);
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (level, message, SECTION);
}
