//! Logging through the current addon's scoped engine handle.

use alloc::{rc::Rc, vec::Vec};
use core::cell::RefCell;

type Logger = Rc<dyn Fn(i32, &str)>;
std::thread_local! {
    static LOGGERS: RefCell<Vec<Logger>> = const { RefCell::new(Vec::new()) };
}

pub(crate) struct Scope;
impl Drop for Scope {
    fn drop(&mut self) {
        LOGGERS.with(|loggers| {
            loggers.borrow_mut().pop();
        });
    }
}

pub(crate) fn enter<E: spring::capabilities::Common>(spring: &spring::Spring<'_, E>) -> Scope {
    let module = spring.module_ref();
    LOGGERS.with(|loggers| {
        loggers.borrow_mut().push(Rc::new(move |level, message| {
            let _ = module.with_spring(|spring| spring.messages().log(SECTION, level, message));
        }))
    });
    Scope
}

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
    let logger = LOGGERS.with(|loggers| loggers.borrow().last().cloned());
    if let Some(logger) = logger {
        logger(level, message);
    }
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
