//! Build native Spring/Recoil Skirmish AIs as Rust `cdylib`s.
//!
//! Implement [`SkirmishAi`] and invoke [`export_ai!`]. The SDK owns dispatch,
//! per-instance state and panic containment. [`Context`] is borrowed only during
//! engine callbacks: copy observations into your planner, then submit recorded orders.
#![forbid(unsafe_op_in_unsafe_fn)]
mod context;
mod event;
mod runtime;

pub use context::{Context, FeatureSnapshot, OrderOptions, UnitSnapshot};
pub use event::{Event, Visibility};
pub use runtime::Runtime;
pub use spring_skirmish_ai_sys as sys;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct AiId(pub i32);
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct TeamId(pub i32);
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct UnitId(pub i32);
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct UnitDefId(pub i32);
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct FeatureId(pub i32);
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct FeatureDefId(pub i32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    MissingCallback(&'static str),
    InvalidInput(&'static str),
    Engine(i32),
    UnknownAi(AiId),
    DuplicateAi(AiId),
    Quarantined(AiId),
    Panicked,
    Busy,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}
pub type Result<T> = std::result::Result<T, Error>;

/// One independent state value per engine AI instance, including multiple teams
/// loading the same shared library. Engine calls are serialized by the runtime.
pub trait SkirmishAi: Send + Sized + 'static {
    fn new(context: &Context<'_>) -> Result<Self>;
    fn handle(&mut self, context: &Context<'_>, event: Event<'_>) -> Result<()>;
}

/// Export the three native C-interface entry points from a `cdylib` crate.
/// Use the default unwinding panic strategy to contain Rust panics at the boundary.
/// `panic=abort` terminates the host process and cannot be contained.
#[macro_export]
macro_rules! export_ai {
    ($ai:ty) => {
        static SPRING_AI_RUNTIME: $crate::Runtime<$ai> = $crate::Runtime::new();
        /// # Safety
        /// Called by the engine with a live callback table matching this SDK's ABI.
        #[unsafe(no_mangle)]
        pub unsafe extern "system" fn init(
            id: i32,
            callbacks: *const $crate::sys::SSkirmishAICallback,
        ) -> i32 {
            unsafe { SPRING_AI_RUNTIME.init($crate::AiId(id), callbacks) }
        }
        /// # Safety
        /// Data must match the event topic and remain valid for this call.
        #[unsafe(no_mangle)]
        pub unsafe extern "system" fn handleEvent(
            id: i32,
            topic: i32,
            data: *const ::std::ffi::c_void,
        ) -> i32 {
            unsafe { SPRING_AI_RUNTIME.handle($crate::AiId(id), topic, data) }
        }
        #[unsafe(no_mangle)]
        pub extern "system" fn release(id: i32) -> i32 {
            SPRING_AI_RUNTIME.release($crate::AiId(id))
        }
    };
}

#[cfg(test)]
mod tests;
