#![no_std]

extern crate alloc;

pub mod event;
pub mod macros;
pub mod rules;
pub mod ui;
pub mod unsynced;

pub mod reexports {
    pub use spring::*;
}

pub use event::{CommandEvent, EventResult, KeyEvent, ViewGeometry};
pub use rules::{
    Gadget, GadgetHandler, PendingRulesEvent, UnitDestroyedEvent, UnitPreDamagedEvent,
};
pub use ui::{Widget, WidgetHandler};
pub use unsynced::{UnsyncedAddon, UnsyncedHandler};
