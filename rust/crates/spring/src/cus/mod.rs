/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */
//! Portable CUS runtime shared by native and Core-Wasm transports.
//!
//! The module is intentionally split by responsibility so transport code stays
//! thin and the scheduler remains backend-neutral.

extern crate alloc;

mod engine;
mod instance;
mod registry;
mod scheduler;
mod script;
mod types;

pub mod prelude;

#[cfg(target_arch = "wasm32")]
pub mod core_module;

#[cfg(target_arch = "wasm32")]
/// Core-Wasm transport for the portable CUS engine. The host validates the
/// unit/instance pair and performs the actual CUnitScript operation.
pub mod wasm;

pub use engine::*;
pub use instance::*;
pub use registry::*;
pub use scheduler::*;
pub use script::*;
pub use types::*;

#[cfg(test)]
mod tests;
