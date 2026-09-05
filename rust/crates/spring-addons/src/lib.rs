#![no_std]
#![doc = include_str!("../REENTRANCY.md")]

extern crate alloc;

pub mod event;
pub mod macros;
pub mod rules;
pub mod runtime;
pub mod ui;
pub mod unsynced;

pub mod reexports {
    pub use spring::*;
}

pub use event::{CommandEvent, EventResult, KeyEvent, ViewGeometry};
pub use rules::{Gadget, GadgetHandler, UnitDestroyedEvent, UnitPreDamagedEvent};
pub use runtime::{AddonContext, Resources};
pub use ui::{Widget, WidgetHandler};
pub use unsynced::{UnsyncedAddon, UnsyncedHandler};
