#![no_std]
#![doc = include_str!("../REENTRANCY.md")]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod event;
pub mod macros;
pub mod panic;
pub mod rules;
pub mod runtime;
pub mod ui;
pub mod unsynced;

pub mod reexports {
    pub use spring::*;
}

pub use event::{CommandEvent, EventResult, KeyEvent, ViewGeometry};
pub use panic::{install_panic_hook, log_error};
pub use rules::{Gadget, GadgetHandler, UnitDestroyedEvent, UnitPreDamagedEvent};
pub use runtime::{AddonContext, AddonRuntime, Resource, Resources, active_callins};
pub use ui::{Widget, WidgetHandler};
pub use unsynced::{UnsyncedAddon, UnsyncedHandler};
