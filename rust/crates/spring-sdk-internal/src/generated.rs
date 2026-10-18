//! Generated full-surface Core bindings.
//!
//! `build.rs` stages the latest codegen output into OUT_DIR. Before the first
//! regeneration the staged file is intentionally empty, so the hand-written
//! benchmark/specialized SDK remains usable.

include!(concat!(env!("OUT_DIR"), "/core_generated.rs"));
