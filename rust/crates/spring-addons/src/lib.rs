#![no_std]
#![doc = include_str!("../REENTRANCY.md")]

extern crate alloc;

extern crate std;

pub mod event;
pub mod log;
pub mod macros;
pub mod panic;
pub mod rules;
pub mod runtime;
pub mod ui;
pub mod unsynced;

pub mod reexports {
    pub use spring::*;
    #[cfg(feature = "std")]
    pub use std::thread_local;
}

pub use event::{CommandEvent, EventResult, KeyEvent, ViewGeometry, keys};
pub use panic::{borrow_conflict, install_panic_hook};
pub use rules::{Gadget, GadgetHandler, UnitDestroyedEvent, UnitPreDamagedEvent};
pub use runtime::{AddonContext, AddonRuntime, Resource, Resources, with_active_callins};
pub use ui::{HideDefaultInterface, Widget, WidgetHandler};
pub use unsynced::{UnsyncedAddon, UnsyncedHandler};

#[cfg(all(test, target_arch = "wasm32"))]
mod export_macro_compile_tests {
    use super::UnsyncedHandler;

    fn setup(
        _handler: &mut UnsyncedHandler<()>,
        _spring: &spring::Spring<'_, spring::RulesUnsynced>,
    ) {
    }

    crate::export_rules_unsynced_addons! {
        state: (),
        setup: setup,
        scratch: 4096,
    }
}
