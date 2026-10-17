/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

//! Unit scripts use the same scheduler on native and Wasm. Engine operations
//! obtain a fresh Spring scope from the module that owns the script.
//!
//! Add `cus: CusHost<MyScript>` to `export_rules_gadgets!`. The host then
//! belongs to the same module as the gadgets and is dropped at shutdown.
//! `with_cus_module(|host, spring| ...)` supplies it alongside a fresh Spring
//! reference. Attach, definition lookup, and named calls return engine errors
//! normally. Script operations fail the module if the engine rejects them.
//!
//! Native scripts, task futures, and shared captures must be `Send`. Use
//! `shared::{Shared, SharedCell}` for script state shared with tasks: native
//! uses `Arc` and checked mutexes, while Wasm uses `Rc` and `RefCell`.
//! End state borrows before engine calls that can enter the same script.

#[path = "../../../spring/src/cus/engine.rs"]
mod engine;
#[doc(hidden)]
pub mod exports;
pub mod host;
#[path = "../../../spring/src/cus/instance.rs"]
mod instance;
#[path = "../../../spring/src/cus/registry.rs"]
mod registry;
#[path = "../../../spring/src/cus/scheduler.rs"]
mod scheduler;
#[path = "../../../spring/src/cus/script.rs"]
mod script;
pub mod shared;
mod transport;
#[path = "../../../spring/src/cus/types.rs"]
mod types;

pub use engine::*;
pub use instance::*;
pub use registry::*;
pub use scheduler::*;
pub use script::*;
pub use transport::{Cus, CusEngine};
pub use types::*;

pub mod prelude {
    pub use super::shared::{Shared, SharedCell};
    pub use super::*;
    pub use crate::{UnitId, WeaponDefId};
}

#[cfg(test)]
#[path = "../../../spring/src/cus/tests.rs"]
mod tests;
