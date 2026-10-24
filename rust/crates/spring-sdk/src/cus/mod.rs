/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

//! Unit scripts use the same scheduler on native and Wasm. Engine operations
//! obtain a fresh Spring scope from the module that owns the script.
//!
//! Add `cus: CusHost<MyScript>` to `export_rules_gadgets!`. The host then
//! belongs to the same module as the gadgets and is dropped at shutdown.
//! Register spring-addons' `CusHostGadget::new(with_cus_host)` to attach the
//! scripts of `UnitDefScript` types and wake them each frame, as Lua's
//! `unit_script.lua` does.
//! `with_cus_module(|host, spring| ...)` supplies it alongside a fresh Spring
//! reference. Attach, definition lookup, and named calls return engine errors
//! normally. Script operations fail the module if the engine rejects them.
//!
//! Native scripts, task futures, and shared captures must be `Send`. Use
//! `shared::{Shared, SharedCell}` for script state shared with tasks: native
//! uses `Arc` and checked mutexes, while Wasm uses `Rc` and `RefCell`.
//! End state borrows before engine calls that can enter the same script.

#[path = "../../../spring-sdk-internal/src/cus/engine.rs"]
mod engine;
#[doc(hidden)]
pub mod exports;
pub mod host;
#[path = "../../../spring-sdk-internal/src/cus/instance.rs"]
mod instance;
#[path = "../../../spring-sdk-internal/src/cus/registry.rs"]
mod registry;
#[path = "../../../spring-sdk-internal/src/cus/scheduler.rs"]
mod scheduler;
#[path = "../../../spring-sdk-internal/src/cus/script.rs"]
mod script;
pub mod shared;
mod transport;
#[path = "../../../spring-sdk-internal/src/cus/types.rs"]
mod types;

pub use engine::*;
pub use instance::*;
pub use registry::*;
pub use scheduler::*;
pub use script::*;
pub use transport::{Cus, CusEngine};

impl UnitCtx {
    /// Run synced engine calls during this script's active module callin.
    /// Suspended tasks keep an identifier, never a borrowed Spring handle.
    pub fn with_spring<R>(
        &self,
        call: impl for<'call> FnOnce(&mut crate::Spring<'call, crate::RulesSynced>) -> crate::Result<R>,
    ) -> crate::Result<R> {
        if !self.is_alive() {
            return Err(crate::ApiError::new(crate::ErrorCode::InvalidState as i32));
        }
        self.engine_scope
            .as_ref()
            .ok_or(crate::ApiError::new(crate::ErrorCode::NotAvailable as i32))?
            .with_spring(call)
    }
}
pub use types::*;

pub mod prelude {
    pub use super::shared::{Shared, SharedCell};
    pub use super::*;
    pub use crate::{UnitId, WeaponDefId};
}

#[cfg(test)]
#[path = "../../../spring-sdk-internal/src/cus/tests.rs"]
mod tests;
