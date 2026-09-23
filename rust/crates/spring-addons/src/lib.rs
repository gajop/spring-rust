#![no_std]
#![doc = include_str!("../REENTRANCY.md")]

extern crate alloc;

#[cfg(feature = "std")]
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
    #[cfg(target_arch = "wasm32")]
    pub use spring::{
        export_callin_scratch, export_draw_feature, export_draw_genesis, export_draw_screen,
        export_draw_screen_effects, export_draw_unit, export_draw_world_pre_particles,
        export_draw_world_pre_unit, export_feature_created, export_feature_destroyed,
        export_projectile_created, export_projectile_destroyed, export_recv_from_synced,
        export_unit_destroyed, export_unit_finished, export_unit_given, export_unit_taken,
        export_view_resize,
    };
    #[cfg(feature = "std")]
    pub use std::thread_local;
}

pub use event::{CommandEvent, EventResult, KeyEvent, ViewGeometry};
pub use panic::{borrow_conflict, install_panic_hook};
pub use rules::{Gadget, GadgetHandler, UnitDestroyedEvent, UnitPreDamagedEvent};
pub use runtime::{AddonContext, AddonRuntime, Resource, Resources, with_active_callins};
pub use ui::{HideDefaultInterface, Widget, WidgetHandler};
pub use unsynced::{UnsyncedAddon, UnsyncedHandler};

#[cfg(all(test, target_arch = "wasm32"))]
mod export_macro_compile_tests {
    use super::UnsyncedHandler;

    fn setup(_handler: &mut UnsyncedHandler<()>) {}

    crate::export_rules_unsynced_addons! {
        state: (),
        setup: setup,
        scratch: 4096,
    }
}
