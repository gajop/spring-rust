#![no_std]

extern crate alloc;

pub mod event;
pub mod macros;
pub mod rules;
pub mod ui;

pub mod reexports {
    pub use spring::*;
}

pub use event::{EventResult, KeyEvent};
pub use rules::{
    Gadget, GadgetHandler, PendingRulesEvent, UnitDestroyedEvent, UnitPreDamagedEvent,
};
pub use ui::{Widget, WidgetHandler};
