/// Prelude module for common types and traits when building Spring native modules.
///
/// # Example
///
/// ```rust
/// use spring_native::prelude::*;
///
/// struct MyModule;
///
/// impl NativeModule for MyModule {
///     fn new(_interface: NativeInterfaceRef) -> Self { MyModule }
///
///     fn game_start(&mut self) -> Result<(), Error> {
///         Ok(())
///     }
/// }
/// ```
pub use crate::{
    callbacks::{GameSetupPlayerState, KeyAction, ModuleData, NativeModule, ViewGeometry},
    constants,
    error::Error,
    interface::NativeInterfaceRef,
    module_entry::{result_to_error_ptr, setup_panic_handler},
    sys, NATIVE_API_VERSION_MAJOR, NATIVE_API_VERSION_MINOR, NATIVE_API_VERSION_PATCH,
};

// Re-export commonly used macros
pub use crate::{bytes_to_slice, cstr_to_str, export_module, impl_callback};
