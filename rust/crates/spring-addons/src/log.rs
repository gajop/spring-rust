//! Engine logging that is safe to call from any target.
//!
//! `spring`'s message imports only exist on wasm32, so every call has to be
//! target-gated. Doing that once here keeps the rest of the crate portable and
//! lets it compile and run its unit tests on the host.

const SECTION: &str = "spring-addons";

/// Spring's `LOG_LEVEL_WARNING`.
const WARNING: i32 = 40;

/// Spring's `LOG_LEVEL_INFO`.
const INFO: i32 = 30;

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

/// Write a line to the engine log at info level.
pub fn info(message: &str) {
    write(INFO, message);
}

fn write(level: i32, message: &str) {
    #[cfg(target_arch = "wasm32")]
    let _ = spring::log(SECTION, level, message);
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (level, message, SECTION);
}

/// Report a callback the guest was asked to run but does not know about.
///
/// Every environment has to export `spring:callback/dispatch` so that
/// synchronous closure callouts (`gfx::render_to_texture` and friends) work;
/// only the UI environment additionally keeps a registry of retained
/// callbacks. Anything else reaching the dispatcher is a bug worth naming.
pub fn warn_unhandled_callback(callback_id: u32) {
    let mut buffer = [0u8; 8];
    for (index, slot) in buffer.iter_mut().enumerate() {
        let nibble = (callback_id >> (28 - index * 4)) & 0xf;
        *slot = match nibble {
            0..=9 => b'0' + nibble as u8,
            value => b'a' + (value - 10) as u8,
        };
    }
    let hex = core::str::from_utf8(&buffer).unwrap_or("????????");
    let mut message = alloc::string::String::from("unhandled callback id 0x");
    message.push_str(hex);
    warning(&message);
}
