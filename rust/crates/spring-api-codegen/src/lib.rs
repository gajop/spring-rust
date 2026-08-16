//! Generalized name for the shared Spring API generator.
//!
//! The old `spring-native-codegen` package remains as a compatibility package
//! for downstream users. New native/Wasm generation code should depend on this
//! facade so the transport-neutral model has one stable package identity.

pub use spring_native_codegen::*;
