/// Helper utilities for module initialization and callback exports.
///
/// This module provides macros and functions to reduce boilerplate when
/// creating native Spring modules.

/// Setup panic handler to notify Spring when Rust code panics.
///
/// This should be called early in `InitializeNativeModule`. It replaces
/// the default panic handler with one that attempts to log the panic
/// message to Spring before aborting.
///
/// # Safety
///
/// This function sets a global panic hook and should only be called once
/// during module initialization.
pub fn setup_panic_handler() {
    use std::panic;

    let default_panic = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // Call the default panic handler first (prints to stderr)
        default_panic(panic_info);

        // Try to log to Spring if possible
        // Note: We can't safely call Spring APIs here because the interface
        // might not be initialized or the panic might have corrupted state.
        // The default panic handler output to stderr is the best we can do.
        eprintln!("Spring Native Module Panic!");
    }));
}

/// Restore Rust's default panic hook when a native module is unloaded.
///
/// The hook installed by [`setup_panic_handler`] is process-global.  Native
/// modules can be hot-reloaded, so leaving the boxed hook in that global slot
/// would retain module-owned code and state until process exit.
pub fn clear_panic_handler() {
    drop(std::panic::take_hook());
}

/// Helper macro to convert C string pointers to Rust &str safely.
///
/// Returns `Err` if the pointer is null or contains invalid UTF-8.
#[macro_export]
macro_rules! cstr_to_str {
    ($ptr:expr) => {
        if $ptr.is_null() {
            Err($crate::Error::new(1, "Null string pointer".to_string()))
        } else {
            unsafe { std::ffi::CStr::from_ptr($ptr) }
                .to_str()
                .map_err(|e| $crate::Error::new(1, format!("Invalid UTF-8: {}", e)))
        }
    };
}

/// Helper macro to convert C byte array pointers to Rust slices safely.
///
/// Returns `Err` if the pointer is null.
#[macro_export]
macro_rules! bytes_to_slice {
    ($ptr:expr, $len:expr) => {
        if $ptr.is_null() {
            Err($crate::Error::new(1, "Null pointer".to_string()))
        } else if $len == 0 {
            Ok(&[])
        } else {
            Ok(unsafe { std::slice::from_raw_parts($ptr, $len as usize) })
        }
    };
}

/// Helper to convert Result<(), Error> to an error pointer for FFI.
///
/// On `Ok(())`, returns a null pointer.
/// On `Err(e)`, leaks the error and returns a pointer to it.
pub fn result_to_error_ptr(result: Result<(), crate::Error>) -> *const crate::sys::Error {
    match result {
        Ok(()) => std::ptr::null(),
        Err(e) => {
            // Leak the error so it can be returned to C
            // The engine is responsible for freeing this via Memory::FreeError
            Box::into_raw(Box::new(crate::sys::Error {
                code: e.code(),
                message: std::ffi::CString::new(e.message())
                    .unwrap_or_else(|_| std::ffi::CString::new("Invalid error message").unwrap())
                    .into_raw(),
            }))
        }
    }
}

/// Catch panics at FFI boundary and convert to error pointer.
///
/// This prevents panics from unwinding across the C ABI boundary, which would
/// cause undefined behavior and crash Spring.
pub fn catch_panic_ffi<F>(f: F) -> *const crate::sys::Error
where
    F: FnOnce() -> Result<(), crate::Error> + std::panic::UnwindSafe,
{
    match std::panic::catch_unwind(f) {
        Ok(result) => result_to_error_ptr(result),
        Err(panic_info) => {
            // Convert panic to error
            let panic_msg = if let Some(s) = panic_info.downcast_ref::<&str>() {
                format!("Panic: {}", s)
            } else if let Some(s) = panic_info.downcast_ref::<String>() {
                format!("Panic: {}", s)
            } else {
                "Panic: unknown cause".to_string()
            };
            result_to_error_ptr(Err(crate::Error::new(1, panic_msg)))
        }
    }
}

/// Decode a Lua-provided native-module message.
///
/// Lua strings are length-prefixed and may contain embedded NUL bytes, so this
/// must not use `CStr::from_ptr`.
///
/// # Safety
///
/// `message` must point to `message_length` bytes that remain valid for the
/// returned string slice's lifetime.
pub unsafe fn lua_call_message_to_str<'a>(
    message: *const std::ffi::c_char,
    message_length: u32,
) -> Result<&'a str, crate::Error> {
    if message.is_null() {
        Err(crate::Error::new(1, "Null message pointer".to_string()))
    } else {
        let bytes = std::slice::from_raw_parts(message as *const u8, message_length as usize);
        std::str::from_utf8(bytes)
            .map_err(|e| crate::Error::new(1, format!("Invalid UTF-8: {}", e)))
    }
}

/// Macro to generate callback export boilerplate.
///
/// This macro generates the `#[no_mangle] extern "C"` function that Spring
/// will call, handling all the FFI conversions and calling your safe Rust
/// implementation.
///
/// # Example
///
/// ```ignore
/// impl_callback! {
///     fn GameStart(
///         interface: *const NativeInterface,
///         module_data: *mut c_void,
///         _query: *const sys::GameStartQuery,
///         result: *mut sys::GameStartResult,
///     ) -> |module: &mut MyModule| {
///         module.game_start()
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_callback {
    (
        fn $name:ident(
            interface: *const $iface:ty,
            module_data: *mut std::ffi::c_void,
            query: *const $query:ty,
            result: *mut $result:ty,
        ) -> |$module:ident: &mut $module_type:ty| $body:block
    ) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(
            _interface: *const $iface,
            module_data: *mut std::ffi::c_void,
            _query: *const $query,
            result: *mut $result,
        ) {
            if module_data.is_null() || result.is_null() {
                eprintln!("Warning: {} called with null pointers", stringify!($name));
                return;
            }

            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                let $module = data.module();
                let callback_result = $body;

                (*result).error = $crate::module_entry::result_to_error_ptr(callback_result);
            }
        }
    };
}

pub use impl_callback;

/// Export all Spring callbacks for a NativeModule implementation.
///
/// This macro generates all the FFI boilerplate needed to expose your module to Spring.
/// You only need to implement the `NativeModule` trait - this macro handles everything else.
///
/// # Example
///
/// ```rust,ignore
/// use spring_native::prelude::*;
///
/// struct MyModule;
///
/// impl NativeModule for MyModule {
///     fn new() -> Self { MyModule }
///
///     fn game_start(&mut self) -> Result<(), Error> {
///         println!("Game started!");
///         Ok(())
///     }
/// }
///
/// // That's it! This generates all FFI exports:
/// spring_native::export_module!(MyModule);
/// ```
#[macro_export]
macro_rules! export_module {
    ($module_type:ty) => {
        use std::ffi::c_void;

        /// API version this module was built against.
        ///
        /// This symbol is read by the host BEFORE calling InitializeNativeModule
        /// to check for major version compatibility.
        #[no_mangle]
        pub static NativeModuleApiVersion: [u32; 3] = [
            $crate::NATIVE_API_VERSION_MAJOR,
            $crate::NATIVE_API_VERSION_MINOR,
            $crate::NATIVE_API_VERSION_PATCH,
        ];

        /// Module entry point - Spring calls this when loading the module
        #[no_mangle]
        pub unsafe extern "C" fn InitializeNativeModule(
            interface: *const $crate::sys::NativeInterface,
            query: *const $crate::sys::InitializeNativeModuleQuery,
            result: *mut $crate::sys::InitializeNativeModuleResult,
        ) {
            if interface.is_null() || query.is_null() || result.is_null() {
                return;
            }

            $crate::module_entry::setup_panic_handler();

            unsafe {
                let q = &*query;

                // Validate host version compatibility
                // Major version MUST match
                if q.hostVersionMajor != $crate::NATIVE_API_VERSION_MAJOR {
                    let msg = format!(
                        "Incompatible API version: host v{}.{}.{}, module v{}.{}.{}",
                        q.hostVersionMajor,
                        q.hostVersionMinor,
                        q.hostVersionPatch,
                        $crate::NATIVE_API_VERSION_MAJOR,
                        $crate::NATIVE_API_VERSION_MINOR,
                        $crate::NATIVE_API_VERSION_PATCH
                    );
                    (*result).error =
                        $crate::module_entry::result_to_error_ptr(Err($crate::Error::new(1, msg)));
                    (*result).moduleData = std::ptr::null_mut();
                    (*result).moduleVersionMajor = $crate::NATIVE_API_VERSION_MAJOR;
                    (*result).moduleVersionMinor = $crate::NATIVE_API_VERSION_MINOR;
                    (*result).moduleVersionPatch = $crate::NATIVE_API_VERSION_PATCH;
                    return;
                }

                // Create module
                let module_data = $crate::ModuleData::<$module_type>::new(interface);

                // Success - report module version
                (*result).error = std::ptr::null();
                (*result).moduleData = Box::into_raw(module_data) as *mut c_void;
                (*result).moduleVersionMajor = $crate::NATIVE_API_VERSION_MAJOR;
                (*result).moduleVersionMinor = $crate::NATIVE_API_VERSION_MINOR;
                (*result).moduleVersionPatch = $crate::NATIVE_API_VERSION_PATCH;
            }
        }

        // Helper macro for callbacks with no parameters
        macro_rules! export_simple_callback {
            ($name:ident, $method:ident, $query:ty, $result:ty) => {
                #[no_mangle]
                pub unsafe extern "C" fn $name(
                    _interface: *const $crate::sys::NativeInterface,
                    module_data: *mut c_void,
                    _query: *const $query,
                    result: *mut $result,
                ) {
                    if module_data.is_null() || result.is_null() {
                        return;
                    }
                    unsafe {
                        (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                            let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                            data.module().$method()
                        });
                    }
                }
            };
        }

        macro_rules! export_unit_id_callback {
            ($name:ident, $method:ident, $query:ty, $result:ty) => {
                #[no_mangle]
                pub unsafe extern "C" fn $name(
                    _interface: *const $crate::sys::NativeInterface,
                    module_data: *mut c_void,
                    query: *const $query,
                    result: *mut $result,
                ) {
                    if module_data.is_null() || query.is_null() || result.is_null() {
                        return;
                    }
                    unsafe {
                        (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                            let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                            data.module().$method((&*query).unitID)
                        });
                    }
                }
            };
        }

        macro_rules! export_unit_los_callback {
            ($name:ident, $method:ident) => {
                #[no_mangle]
                pub unsafe extern "C" fn $name(
                    _interface: *const $crate::sys::NativeInterface,
                    module_data: *mut c_void,
                    query: *const $crate::sys::UnitLosEventQuery,
                    result: *mut $crate::sys::UnitLosEventResult,
                ) {
                    if module_data.is_null() || query.is_null() || result.is_null() {
                        return;
                    }
                    unsafe {
                        (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                            let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                            let q = &*query;
                            data.module().$method(q.unitID, q.allyTeam)
                        });
                    }
                }
            };
        }

        macro_rules! export_projectile_id_callback {
            ($name:ident, $method:ident) => {
                #[no_mangle]
                pub unsafe extern "C" fn $name(
                    _interface: *const $crate::sys::NativeInterface,
                    module_data: *mut c_void,
                    query: *const $crate::sys::ProjectileEventQuery,
                    result: *mut $crate::sys::ProjectileEventResult,
                ) {
                    if module_data.is_null() || query.is_null() || result.is_null() {
                        return;
                    }
                    unsafe {
                        (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                            let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                            data.module().$method((&*query).projectileID)
                        });
                    }
                }
            };
        }

        macro_rules! finish_bool_callback {
            ($result:expr, $callback_result:expr) => {
                match $callback_result {
                    Ok(value) => {
                        (*$result).value = value;
                        (*$result).error = std::ptr::null();
                    }
                    Err(err) => {
                        (*$result).value = false;
                        (*$result).error = $crate::module_entry::result_to_error_ptr(Err(err));
                    }
                }
            };
        }

        macro_rules! export_simple_bool_callback {
            ($name:ident, $method:ident) => {
                #[no_mangle]
                pub unsafe extern "C" fn $name(
                    _interface: *const $crate::sys::NativeInterface,
                    module_data: *mut c_void,
                    _query: *const $crate::sys::SimpleCallinQuery,
                    result: *mut $crate::sys::BoolCallinResult,
                ) {
                    if module_data.is_null() || result.is_null() {
                        return;
                    }
                    unsafe {
                        let callback_result =
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                let data =
                                    &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                                data.module().$method()
                            }))
                            .unwrap_or_else(|_| Err($crate::Error::new(1, "Panic".to_string())));
                        finish_bool_callback!(result, callback_result);
                    }
                }
            };
        }

        macro_rules! finish_string_callback {
            ($result:expr, $callback_result:expr) => {
                match $callback_result {
                    Ok(Some(value)) => {
                        (*$result).value = std::ffi::CString::new(value)
                            .unwrap_or_else(|_| std::ffi::CString::new("").unwrap())
                            .into_raw();
                        (*$result).error = std::ptr::null();
                    }
                    Ok(None) => {
                        (*$result).value = std::ptr::null();
                        (*$result).error = std::ptr::null();
                    }
                    Err(err) => {
                        (*$result).value = std::ptr::null();
                        (*$result).error = $crate::module_entry::result_to_error_ptr(Err(err));
                    }
                }
            };
        }

        // Game events
        #[no_mangle]
        pub unsafe extern "C" fn Load(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::ArchiveCallinQuery,
            result: *mut $crate::sys::ArchiveCallinResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    data.module().load((&*query).archive)
                });
            }
        }

        export_simple_callback!(
            GamePreload,
            game_preload,
            $crate::sys::GamePreloadQuery,
            $crate::sys::GamePreloadResult
        );
        export_simple_callback!(
            GameStart,
            game_start,
            $crate::sys::GameStartQuery,
            $crate::sys::GameStartResult
        );
        // Shutdown transfers ownership of the opaque module pointer back to
        // Rust. The host is about to unload the shared object, so merely
        // forwarding NativeModule::shutdown is insufficient: Drop must run
        // while module code is still loaded to release host resources such as
        // RmlUi contexts.
        #[no_mangle]
        pub unsafe extern "C" fn Shutdown(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            _query: *const $crate::sys::ShutdownQuery,
            result: *mut $crate::sys::ShutdownResult,
        ) {
            if module_data.is_null() || result.is_null() {
                return;
            }
            unsafe {
                let mut data = Box::from_raw(module_data as *mut $crate::ModuleData<$module_type>);
                let shutdown_result =
                    $crate::module_entry::catch_panic_ffi(std::panic::AssertUnwindSafe(|| {
                        data.module().shutdown()
                    }));
                $crate::module_entry::clear_panic_handler();
                (*result).error = shutdown_result;
                // `data` is dropped here even if shutdown returned an error or
                // panicked. The engine guarantees this call occurs once before
                // unloading the native shared object.
            }
        }
        export_simple_callback!(
            Update,
            update,
            $crate::sys::UpdateQuery,
            $crate::sys::UpdateResult
        );
        export_simple_callback!(
            DrawScreen,
            draw_screen,
            $crate::sys::DrawScreenQuery,
            $crate::sys::DrawScreenResult
        );
        export_simple_callback!(
            DrawGenesis,
            draw_genesis,
            $crate::sys::SimpleCallinQuery,
            $crate::sys::SimpleCallinResult
        );
        export_simple_callback!(
            DrawWorld,
            draw_world,
            $crate::sys::SimpleCallinQuery,
            $crate::sys::SimpleCallinResult
        );
        export_simple_callback!(
            DrawWorldPreUnit,
            draw_world_pre_unit,
            $crate::sys::SimpleCallinQuery,
            $crate::sys::SimpleCallinResult
        );
        export_simple_callback!(
            DrawPreDecals,
            draw_pre_decals,
            $crate::sys::SimpleCallinQuery,
            $crate::sys::SimpleCallinResult
        );

        #[no_mangle]
        pub unsafe extern "C" fn DrawWorldPreParticles(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::DrawWorldPreParticlesQuery,
            result: *mut $crate::sys::DrawWorldPreParticlesResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module().draw_world_pre_particles(
                        q.drawAboveWater,
                        q.drawBelowWater,
                        q.drawReflection,
                        q.drawRefraction,
                    )
                });
            }
        }

        export_simple_callback!(
            DrawWaterPost,
            draw_water_post,
            $crate::sys::SimpleCallinQuery,
            $crate::sys::SimpleCallinResult
        );
        export_simple_callback!(
            DrawWorldShadow,
            draw_world_shadow,
            $crate::sys::SimpleCallinQuery,
            $crate::sys::SimpleCallinResult
        );
        export_simple_callback!(
            DrawShadowPassTransparent,
            draw_shadow_pass_transparent,
            $crate::sys::SimpleCallinQuery,
            $crate::sys::SimpleCallinResult
        );
        export_simple_callback!(
            DrawWorldReflection,
            draw_world_reflection,
            $crate::sys::SimpleCallinQuery,
            $crate::sys::SimpleCallinResult
        );
        export_simple_callback!(
            DrawWorldRefraction,
            draw_world_refraction,
            $crate::sys::SimpleCallinQuery,
            $crate::sys::SimpleCallinResult
        );
        export_simple_callback!(
            DrawGroundPreForward,
            draw_ground_pre_forward,
            $crate::sys::SimpleCallinQuery,
            $crate::sys::SimpleCallinResult
        );
        export_simple_callback!(
            DrawGroundPostForward,
            draw_ground_post_forward,
            $crate::sys::SimpleCallinQuery,
            $crate::sys::SimpleCallinResult
        );
        export_simple_callback!(
            DrawGroundPreDeferred,
            draw_ground_pre_deferred,
            $crate::sys::SimpleCallinQuery,
            $crate::sys::SimpleCallinResult
        );
        export_simple_callback!(
            DrawGroundDeferred,
            draw_ground_deferred,
            $crate::sys::SimpleCallinQuery,
            $crate::sys::SimpleCallinResult
        );
        export_simple_callback!(
            DrawGroundPostDeferred,
            draw_ground_post_deferred,
            $crate::sys::SimpleCallinQuery,
            $crate::sys::SimpleCallinResult
        );
        export_simple_callback!(
            DrawUnitsPostDeferred,
            draw_units_post_deferred,
            $crate::sys::SimpleCallinQuery,
            $crate::sys::SimpleCallinResult
        );
        export_simple_callback!(
            DrawFeaturesPostDeferred,
            draw_features_post_deferred,
            $crate::sys::SimpleCallinQuery,
            $crate::sys::SimpleCallinResult
        );
        export_simple_callback!(
            DrawScreenEffects,
            draw_screen_effects,
            $crate::sys::SimpleCallinQuery,
            $crate::sys::SimpleCallinResult
        );
        export_simple_callback!(
            DrawScreenPost,
            draw_screen_post,
            $crate::sys::SimpleCallinQuery,
            $crate::sys::SimpleCallinResult
        );
        export_simple_callback!(
            DrawInMiniMap,
            draw_in_minimap,
            $crate::sys::SimpleCallinQuery,
            $crate::sys::SimpleCallinResult
        );
        export_simple_callback!(
            DrawInMiniMapBackground,
            draw_in_minimap_background,
            $crate::sys::SimpleCallinQuery,
            $crate::sys::SimpleCallinResult
        );

        macro_rules! export_minimap_2_callback {
            ($name:ident, $method:ident, $query:ty, $field1:ident, $field2:ident) => {
                #[no_mangle]
                pub unsafe extern "C" fn $name(
                    _interface: *const $crate::sys::NativeInterface,
                    module_data: *mut c_void,
                    query: *const $query,
                    result: *mut $crate::sys::SimpleCallinResult,
                ) {
                    if module_data.is_null() || query.is_null() || result.is_null() {
                        return;
                    }
                    unsafe {
                        (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                            let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                            let q = &*query;
                            data.module().$method(q.$field1, q.$field2)
                        });
                    }
                }
            };
        }

        macro_rules! export_minimap_3_callback {
            ($name:ident, $method:ident, $query:ty, $field1:ident, $field2:ident, $field3:ident) => {
                #[no_mangle]
                pub unsafe extern "C" fn $name(
                    _interface: *const $crate::sys::NativeInterface,
                    module_data: *mut c_void,
                    query: *const $query,
                    result: *mut $crate::sys::SimpleCallinResult,
                ) {
                    if module_data.is_null() || query.is_null() || result.is_null() {
                        return;
                    }
                    unsafe {
                        (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                            let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                            let q = &*query;
                            data.module().$method(q.$field1, q.$field2, q.$field3)
                        });
                    }
                }
            };
        }

        macro_rules! export_minimap_8_callback {
            (
                $name:ident,
                $method:ident,
                $query:ty,
                $field1:ident,
                $field2:ident,
                $field3:ident,
                $field4:ident,
                $field5:ident,
                $field6:ident,
                $field7:ident,
                $field8:ident
            ) => {
                #[no_mangle]
                pub unsafe extern "C" fn $name(
                    _interface: *const $crate::sys::NativeInterface,
                    module_data: *mut c_void,
                    query: *const $query,
                    result: *mut $crate::sys::SimpleCallinResult,
                ) {
                    if module_data.is_null() || query.is_null() || result.is_null() {
                        return;
                    }
                    unsafe {
                        (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                            let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                            let q = &*query;
                            data.module().$method(
                                q.$field1,
                                q.$field2,
                                q.$field3,
                                q.$field4,
                                q.$field5,
                                q.$field6,
                                q.$field7,
                                q.$field8,
                            )
                        });
                    }
                }
            };
        }

        export_minimap_2_callback!(
            MiniMapRotationChanged,
            minimap_rotation_changed,
            $crate::sys::MiniMapRotationChangedQuery,
            newRot,
            oldRot
        );
        export_minimap_3_callback!(
            MiniMapStateChanged,
            minimap_state_changed,
            $crate::sys::MiniMapStateChangedQuery,
            isMinimized,
            isMaximized,
            isSlaved
        );
        export_minimap_8_callback!(
            MiniMapGeometryChanged,
            minimap_geometry_changed,
            $crate::sys::MiniMapGeometryChangedQuery,
            newPosX,
            newPosY,
            newDimX,
            newDimY,
            oldPosX,
            oldPosY,
            oldDimX,
            oldDimY
        );

        macro_rules! export_draw_bool_2_callback {
            ($name:ident, $method:ident, $query:ty, $field1:ident, $field2:ident) => {
                #[no_mangle]
                pub unsafe extern "C" fn $name(
                    _interface: *const $crate::sys::NativeInterface,
                    module_data: *mut c_void,
                    query: *const $query,
                    result: *mut $crate::sys::BoolCallinResult,
                ) {
                    if module_data.is_null() || query.is_null() || result.is_null() {
                        return;
                    }
                    unsafe {
                        let callback_result =
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                let data =
                                    &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                                let q = &*query;
                                data.module().$method(q.$field1, q.$field2)
                            }))
                            .unwrap_or_else(|_| Err($crate::Error::new(1, "Panic".to_string())));
                        finish_bool_callback!(result, callback_result);
                    }
                }
            };
        }

        macro_rules! export_draw_bool_3_callback {
            ($name:ident, $method:ident, $query:ty, $field1:ident, $field2:ident, $field3:ident) => {
                #[no_mangle]
                pub unsafe extern "C" fn $name(
                    _interface: *const $crate::sys::NativeInterface,
                    module_data: *mut c_void,
                    query: *const $query,
                    result: *mut $crate::sys::BoolCallinResult,
                ) {
                    if module_data.is_null() || query.is_null() || result.is_null() {
                        return;
                    }
                    unsafe {
                        let callback_result =
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                let data =
                                    &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                                let q = &*query;
                                data.module().$method(q.$field1, q.$field2, q.$field3)
                            }))
                            .unwrap_or_else(|_| Err($crate::Error::new(1, "Panic".to_string())));
                        finish_bool_callback!(result, callback_result);
                    }
                }
            };
        }

        export_draw_bool_2_callback!(
            DrawUnit,
            draw_unit,
            $crate::sys::DrawUnitQuery,
            unitID,
            drawMode
        );
        export_draw_bool_2_callback!(
            DrawFeature,
            draw_feature,
            $crate::sys::DrawFeatureQuery,
            featureID,
            drawMode
        );
        export_draw_bool_3_callback!(
            DrawShield,
            draw_shield,
            $crate::sys::DrawShieldQuery,
            unitID,
            weaponID,
            drawMode
        );
        export_draw_bool_2_callback!(
            DrawProjectile,
            draw_projectile,
            $crate::sys::DrawProjectileQuery,
            projectileID,
            drawMode
        );
        export_draw_bool_2_callback!(
            DrawMaterial,
            draw_material,
            $crate::sys::DrawMaterialQuery,
            uuid,
            drawMode
        );

        #[no_mangle]
        pub unsafe extern "C" fn DrawBuildSquare(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::DrawBuildSquareQuery,
            result: *mut $crate::sys::DrawBuildSquareResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    let statuses = if q.statusCount == 0 || q.statuses.is_null() {
                        &[]
                    } else {
                        std::slice::from_raw_parts(q.statuses, q.statusCount as usize)
                    };
                    data.module()
                        .draw_build_square(q.unitDefID, q.x, q.z, q.facing, statuses)
                });
            }
        }

        macro_rules! export_draw_objects_lua_callback {
            ($name:ident, $method:ident) => {
                #[no_mangle]
                pub unsafe extern "C" fn $name(
                    _interface: *const $crate::sys::NativeInterface,
                    module_data: *mut c_void,
                    query: *const $crate::sys::DrawObjectsLuaQuery,
                    result: *mut $crate::sys::DrawObjectsLuaResult,
                ) {
                    if module_data.is_null() || query.is_null() || result.is_null() {
                        return;
                    }
                    unsafe {
                        (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                            let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                            let q = &*query;
                            data.module().$method(
                                q.deferredPass,
                                q.drawReflection,
                                q.drawRefraction,
                            )
                        });
                    }
                }
            };
        }

        macro_rules! export_draw_alpha_objects_lua_callback {
            ($name:ident, $method:ident) => {
                #[no_mangle]
                pub unsafe extern "C" fn $name(
                    _interface: *const $crate::sys::NativeInterface,
                    module_data: *mut c_void,
                    query: *const $crate::sys::DrawAlphaObjectsLuaQuery,
                    result: *mut $crate::sys::DrawAlphaObjectsLuaResult,
                ) {
                    if module_data.is_null() || query.is_null() || result.is_null() {
                        return;
                    }
                    unsafe {
                        (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                            let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                            let q = &*query;
                            data.module().$method(q.drawReflection, q.drawRefraction)
                        });
                    }
                }
            };
        }

        export_draw_objects_lua_callback!(DrawOpaqueUnitsLua, draw_opaque_units_lua);
        export_draw_objects_lua_callback!(DrawOpaqueFeaturesLua, draw_opaque_features_lua);
        export_draw_alpha_objects_lua_callback!(DrawAlphaUnitsLua, draw_alpha_units_lua);
        export_draw_alpha_objects_lua_callback!(DrawAlphaFeaturesLua, draw_alpha_features_lua);
        export_simple_callback!(
            DrawShadowUnitsLua,
            draw_shadow_units_lua,
            $crate::sys::SimpleCallinQuery,
            $crate::sys::SimpleCallinResult
        );
        export_simple_callback!(
            DrawShadowFeaturesLua,
            draw_shadow_features_lua,
            $crate::sys::SimpleCallinQuery,
            $crate::sys::SimpleCallinResult
        );

        #[no_mangle]
        pub unsafe extern "C" fn GameOver(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::GameOverEventQuery,
            result: *mut $crate::sys::GameOverEventResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    let winning_ally_teams = if q.winningAllyTeams.is_null() || q.count == 0 {
                        &[]
                    } else {
                        std::slice::from_raw_parts(q.winningAllyTeams, q.count as usize)
                    };
                    data.module().game_over(winning_ally_teams)
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn GameFrame(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::GameFrameQuery,
            result: *mut $crate::sys::GameFrameResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    data.module().game_frame((&*query).gameFrame)
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn GameFramePost(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::GameFramePostQuery,
            result: *mut $crate::sys::GameFramePostResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    data.module().game_frame_post((&*query).gameFrame)
                });
            }
        }

        // Download events
        #[no_mangle]
        pub unsafe extern "C" fn DownloadFailed(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::DownloadFailedQuery,
            result: *mut $crate::sys::DownloadFailedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module().download_failed(q.downloadID, q.errorID)
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn DownloadFinished(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::DownloadFinishedQuery,
            result: *mut $crate::sys::DownloadFinishedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    data.module().download_finished((&*query).downloadID)
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn DownloadProgress(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::DownloadProgressQuery,
            result: *mut $crate::sys::DownloadProgressResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module()
                        .download_progress(q.downloadID, q.downloaded, q.total)
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn DownloadQueued(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::DownloadQueuedQuery,
            result: *mut $crate::sys::DownloadQueuedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    if q.archiveName.is_null() || q.archiveType.is_null() {
                        Err($crate::Error::new(1, "Null string pointer".to_string()))
                    } else {
                        match std::ffi::CStr::from_ptr(q.archiveName).to_str() {
                            Ok(name) => match std::ffi::CStr::from_ptr(q.archiveType).to_str() {
                                Ok(atype) => {
                                    data.module().download_queued(q.downloadID, name, atype)
                                }
                                Err(e) => {
                                    Err($crate::Error::new(1, format!("Invalid UTF-8: {}", e)))
                                }
                            },
                            Err(e) => Err($crate::Error::new(1, format!("Invalid UTF-8: {}", e))),
                        }
                    }
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn DownloadStarted(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::DownloadStartedQuery,
            result: *mut $crate::sys::DownloadStartedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    data.module().download_started((&*query).downloadID)
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn Save(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::ArchiveCallinQuery,
            result: *mut $crate::sys::ArchiveCallinResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    data.module().save((&*query).archive)
                });
            }
        }

        // Feature events
        #[no_mangle]
        pub unsafe extern "C" fn FeatureCreated(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::FeatureCreatedQuery,
            result: *mut $crate::sys::FeatureCreatedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    data.module().feature_created((&*query).featureID)
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn FeatureDestroyed(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::FeatureDestroyedQuery,
            result: *mut $crate::sys::FeatureDestroyedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    data.module().feature_destroyed((&*query).featureID)
                });
            }
        }

        // Game events
        #[no_mangle]
        pub unsafe extern "C" fn GameID(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::GameIDQuery,
            result: *mut $crate::sys::GameIDResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    if q.gameID.is_null() || q.numBytes == 0 {
                        data.module().game_id(&[])
                    } else {
                        let slice = std::slice::from_raw_parts(q.gameID, q.numBytes as usize);
                        data.module().game_id(slice)
                    }
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn GamePaused(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::GamePausedQuery,
            result: *mut $crate::sys::GamePausedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module().game_paused(q.playerID, q.paused)
                });
            }
        }

        // Player events
        #[no_mangle]
        pub unsafe extern "C" fn PlayerAdded(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::PlayerAddedQuery,
            result: *mut $crate::sys::PlayerAddedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    data.module().player_added((&*query).playerID)
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn PlayerChanged(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::PlayerChangedQuery,
            result: *mut $crate::sys::PlayerChangedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    data.module().player_changed((&*query).playerID)
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn PlayerRemoved(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::PlayerRemovedQuery,
            result: *mut $crate::sys::PlayerRemovedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module().player_removed(q.playerID, q.reason)
                });
            }
        }

        // Team events
        #[no_mangle]
        pub unsafe extern "C" fn TeamChanged(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::TeamChangedQuery,
            result: *mut $crate::sys::TeamChangedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    data.module().team_changed((&*query).teamID)
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn TeamDied(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::TeamDiedQuery,
            result: *mut $crate::sys::TeamDiedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    data.module().team_died((&*query).teamID)
                });
            }
        }

        // Unit events
        #[no_mangle]
        pub unsafe extern "C" fn UnitCreated(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::UnitCreatedQuery,
            result: *mut $crate::sys::UnitCreatedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module().unit_created(q.unitID, q.builderID)
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn UnitDestroyed(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::UnitDestroyedQuery,
            result: *mut $crate::sys::UnitDestroyedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module().unit_destroyed(q.unitID, q.attackerID)
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn UnitExperience(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::UnitExperienceQuery,
            result: *mut $crate::sys::UnitExperienceResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module().unit_experience(q.unitID, q.oldExperience)
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn UnitFinished(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::UnitFinishedQuery,
            result: *mut $crate::sys::UnitFinishedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    data.module().unit_finished((&*query).unitID)
                });
            }
        }

        export_unit_id_callback!(
            UnitReverseBuilt,
            unit_reverse_built,
            $crate::sys::UnitReverseBuiltQuery,
            $crate::sys::UnitReverseBuiltResult
        );

        #[no_mangle]
        pub unsafe extern "C" fn UnitConstructionDecayed(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::UnitConstructionDecayedQuery,
            result: *mut $crate::sys::UnitConstructionDecayedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module().unit_construction_decayed(
                        q.unitID,
                        q.timeSinceLastBuild,
                        q.iterationPeriod,
                        q.part,
                    )
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn UnitFromFactory(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::UnitFromFactoryQuery,
            result: *mut $crate::sys::UnitFromFactoryResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module()
                        .unit_from_factory(q.unitID, q.factoryID, q.userOrders)
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn UnitGiven(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::UnitGivenQuery,
            result: *mut $crate::sys::UnitGivenResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module().unit_given(q.unitID, q.oldTeam, q.newTeam)
                });
            }
        }

        export_unit_id_callback!(
            UnitIdle,
            unit_idle,
            $crate::sys::UnitIdleQuery,
            $crate::sys::UnitIdleResult
        );

        #[no_mangle]
        pub unsafe extern "C" fn UnitCommand(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::UnitCommandQuery,
            result: *mut $crate::sys::UnitCommandResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module().unit_command(
                        q.unitID,
                        q.unitDefID,
                        q.unitTeam,
                        q.command,
                        q.playerNum,
                        q.fromSynced,
                        q.fromLua,
                    )
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn UnitCmdDone(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::UnitCmdDoneQuery,
            result: *mut $crate::sys::UnitCmdDoneResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module()
                        .unit_cmd_done(q.unitID, q.unitDefID, q.unitTeam, q.command)
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn UnitDamaged(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::UnitDamagedQuery,
            result: *mut $crate::sys::UnitDamagedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module().unit_damaged(
                        q.unitID,
                        q.unitDefID,
                        q.unitTeam,
                        q.damage,
                        q.paralyzer,
                        q.weaponDefID,
                        q.projectileID,
                        q.attackerID,
                        q.attackerDefID,
                        q.attackerTeam,
                    )
                });
            }
        }

        export_unit_id_callback!(
            UnitHarvestStorageFull,
            unit_harvest_storage_full,
            $crate::sys::UnitHarvestStorageFullQuery,
            $crate::sys::UnitHarvestStorageFullResult
        );

        #[no_mangle]
        pub unsafe extern "C" fn UnitSeismicPing(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::UnitSeismicPingQuery,
            result: *mut $crate::sys::UnitSeismicPingResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module().unit_seismic_ping(
                        q.pos,
                        q.strength,
                        q.allyTeam,
                        q.unitID,
                        q.unitDefID,
                    )
                });
            }
        }
        export_unit_los_callback!(UnitEnteredRadar, unit_entered_radar);
        export_unit_los_callback!(UnitEnteredLos, unit_entered_los);
        export_unit_los_callback!(UnitLeftRadar, unit_left_radar);
        export_unit_los_callback!(UnitLeftLos, unit_left_los);
        export_unit_id_callback!(
            UnitEnteredUnderwater,
            unit_entered_underwater,
            $crate::sys::UnitMovementClassEventQuery,
            $crate::sys::UnitMovementClassEventResult
        );
        export_unit_id_callback!(
            UnitEnteredWater,
            unit_entered_water,
            $crate::sys::UnitMovementClassEventQuery,
            $crate::sys::UnitMovementClassEventResult
        );
        export_unit_id_callback!(
            UnitEnteredAir,
            unit_entered_air,
            $crate::sys::UnitMovementClassEventQuery,
            $crate::sys::UnitMovementClassEventResult
        );
        export_unit_id_callback!(
            UnitLeftUnderwater,
            unit_left_underwater,
            $crate::sys::UnitMovementClassEventQuery,
            $crate::sys::UnitMovementClassEventResult
        );
        export_unit_id_callback!(
            UnitLeftWater,
            unit_left_water,
            $crate::sys::UnitMovementClassEventQuery,
            $crate::sys::UnitMovementClassEventResult
        );
        export_unit_id_callback!(
            UnitLeftAir,
            unit_left_air,
            $crate::sys::UnitMovementClassEventQuery,
            $crate::sys::UnitMovementClassEventResult
        );

        #[no_mangle]
        pub unsafe extern "C" fn UnitLoaded(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::UnitLoadedQuery,
            result: *mut $crate::sys::UnitLoadedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module().unit_loaded(q.unitID, q.transportID)
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn UnitStunned(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::UnitStunnedQuery,
            result: *mut $crate::sys::UnitStunnedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module().unit_stunned(q.unitID, q.stunned)
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn UnitTaken(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::UnitTakenQuery,
            result: *mut $crate::sys::UnitTakenResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module().unit_taken(q.unitID, q.oldTeam, q.newTeam)
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn UnitUnloaded(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::UnitUnloadedQuery,
            result: *mut $crate::sys::UnitUnloadedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module().unit_unloaded(q.unitID, q.transportID)
                });
            }
        }

        export_unit_id_callback!(
            UnitCloaked,
            unit_cloaked,
            $crate::sys::UnitCloakEventQuery,
            $crate::sys::UnitCloakEventResult
        );
        export_unit_id_callback!(
            UnitDecloaked,
            unit_decloaked,
            $crate::sys::UnitCloakEventQuery,
            $crate::sys::UnitCloakEventResult
        );
        export_unit_id_callback!(
            UnitMoved,
            unit_moved,
            $crate::sys::UnitMoveEventQuery,
            $crate::sys::UnitMoveEventResult
        );
        export_unit_id_callback!(
            UnitMoveFailed,
            unit_move_failed,
            $crate::sys::UnitMoveEventQuery,
            $crate::sys::UnitMoveEventResult
        );
        export_unit_id_callback!(
            UnitArrivedAtGoal,
            unit_arrived_at_goal,
            $crate::sys::UnitMoveEventQuery,
            $crate::sys::UnitMoveEventResult
        );

        #[no_mangle]
        pub unsafe extern "C" fn UnitUnitCollision(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::UnitUnitCollisionQuery,
            result: *mut $crate::sys::BoolCallinResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                let callback_result =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                        let q = &*query;
                        data.module()
                            .unit_unit_collision(q.colliderID, q.collideeID)
                    }))
                    .unwrap_or_else(|_| Err($crate::Error::new(1, "Panic".to_string())));
                finish_bool_callback!(result, callback_result);
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn UnitFeatureCollision(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::UnitFeatureCollisionQuery,
            result: *mut $crate::sys::BoolCallinResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                let callback_result =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                        let q = &*query;
                        data.module()
                            .unit_feature_collision(q.colliderID, q.collideeID)
                    }))
                    .unwrap_or_else(|_| Err($crate::Error::new(1, "Panic".to_string())));
                finish_bool_callback!(result, callback_result);
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn RenderUnitDestroyed(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::RenderUnitDestroyedQuery,
            result: *mut $crate::sys::RenderUnitDestroyedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    data.module().render_unit_destroyed((&*query).unitID)
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn FeatureMoved(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::FeatureMovedQuery,
            result: *mut $crate::sys::FeatureMovedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module().feature_moved(q.featureID, q.oldPos)
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn FeatureDamaged(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::FeatureDamagedQuery,
            result: *mut $crate::sys::FeatureDamagedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module().feature_damaged(
                        q.featureID,
                        q.featureDefID,
                        q.featureTeam,
                        q.damage,
                        q.weaponDefID,
                        q.projectileID,
                        q.attackerID,
                        q.attackerDefID,
                        q.attackerTeam,
                    )
                });
            }
        }

        export_projectile_id_callback!(ProjectileCreated, projectile_created);
        export_projectile_id_callback!(ProjectileDestroyed, projectile_destroyed);

        #[no_mangle]
        pub unsafe extern "C" fn Explosion(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::ExplosionQuery,
            result: *mut $crate::sys::BoolCallinResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                let callback_result =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                        let q = &*query;
                        data.module()
                            .explosion(q.weaponDefID, q.pos, q.ownerID, q.projectileID)
                    }))
                    .unwrap_or_else(|_| Err($crate::Error::new(1, "Panic".to_string())));
                finish_bool_callback!(result, callback_result);
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn LastMessagePosition(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::LastMessagePositionQuery,
            result: *mut $crate::sys::LastMessagePositionResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    data.module().last_message_position((&*query).pos)
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn UnsyncedHeightMapUpdate(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::RectChangedQuery,
            result: *mut $crate::sys::RectChangedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module()
                        .unsynced_height_map_update(q.x1, q.z1, q.x2, q.z2)
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn CameraRotationChanged(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::Float3CallinQuery,
            result: *mut $crate::sys::Float3CallinResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    data.module().camera_rotation_changed((&*query).value)
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn CameraPositionChanged(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::Float3CallinQuery,
            result: *mut $crate::sys::Float3CallinResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    data.module().camera_position_changed((&*query).value)
                });
            }
        }

        export_simple_bool_callback!(KeyMapChanged, key_map_changed);

        #[no_mangle]
        pub unsafe extern "C" fn KeyPress(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::KeyPressQuery,
            result: *mut $crate::sys::BoolCallinResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                let callback_result =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                        let q = &*query;
                        data.module().key_press(q.keyCode, q.scanCode, q.isRepeat)
                    }))
                    .unwrap_or_else(|_| Err($crate::Error::new(1, "Panic".to_string())));
                finish_bool_callback!(result, callback_result);
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn KeyRelease(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::KeyReleaseQuery,
            result: *mut $crate::sys::BoolCallinResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                let callback_result =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                        let q = &*query;
                        data.module().key_release(q.keyCode, q.scanCode)
                    }))
                    .unwrap_or_else(|_| Err($crate::Error::new(1, "Panic".to_string())));
                finish_bool_callback!(result, callback_result);
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn TextInput(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::TextInputQuery,
            result: *mut $crate::sys::BoolCallinResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                let callback_result =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                        let q = &*query;
                        let utf8 = $crate::cstr_to_str!(q.utf8)?;
                        data.module().text_input(utf8)
                    }))
                    .unwrap_or_else(|_| Err($crate::Error::new(1, "Panic".to_string())));
                finish_bool_callback!(result, callback_result);
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn TextEditing(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::TextEditingQuery,
            result: *mut $crate::sys::BoolCallinResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                let callback_result =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                        let q = &*query;
                        let utf8 = $crate::cstr_to_str!(q.utf8)?;
                        data.module().text_editing(utf8, q.start, q.length)
                    }))
                    .unwrap_or_else(|_| Err($crate::Error::new(1, "Panic".to_string())));
                finish_bool_callback!(result, callback_result);
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn MouseMove(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::MouseMoveQuery,
            result: *mut $crate::sys::BoolCallinResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                let callback_result =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                        let q = &*query;
                        data.module().mouse_move(q.x, q.y, q.dx, q.dy, q.button)
                    }))
                    .unwrap_or_else(|_| Err($crate::Error::new(1, "Panic".to_string())));
                finish_bool_callback!(result, callback_result);
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn MousePress(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::MousePressQuery,
            result: *mut $crate::sys::BoolCallinResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                let callback_result =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                        let q = &*query;
                        data.module().mouse_press(q.x, q.y, q.button)
                    }))
                    .unwrap_or_else(|_| Err($crate::Error::new(1, "Panic".to_string())));
                finish_bool_callback!(result, callback_result);
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn MouseRelease(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::MouseReleaseQuery,
            result: *mut $crate::sys::MouseReleaseResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module().mouse_release(q.x, q.y, q.button)
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn MouseWheel(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::MouseWheelQuery,
            result: *mut $crate::sys::BoolCallinResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                let callback_result =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                        let q = &*query;
                        data.module().mouse_wheel(q.up, q.value)
                    }))
                    .unwrap_or_else(|_| Err($crate::Error::new(1, "Panic".to_string())));
                finish_bool_callback!(result, callback_result);
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn IsAbove(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::ScreenPositionQuery,
            result: *mut $crate::sys::BoolCallinResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                let callback_result =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                        let q = &*query;
                        data.module().is_above(q.x, q.y)
                    }))
                    .unwrap_or_else(|_| Err($crate::Error::new(1, "Panic".to_string())));
                finish_bool_callback!(result, callback_result);
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn GetTooltip(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::ScreenPositionQuery,
            result: *mut $crate::sys::StringCallinResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                let callback_result =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                        let q = &*query;
                        data.module().get_tooltip(q.x, q.y)
                    }))
                    .unwrap_or_else(|_| Err($crate::Error::new(1, "Panic".to_string())));
                finish_string_callback!(result, callback_result);
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn DefaultCommand(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::DefaultCommandQuery,
            result: *mut $crate::sys::DefaultCommandResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module()
                        .default_command(q.unitID, q.featureID, q.currentCommand)
                }))
                .unwrap_or_else(|_| Err($crate::Error::new(1, "Panic".to_string())))
                {
                    Ok(Some(command)) => {
                        (*result).value = true;
                        (*result).command = command;
                        (*result).error = std::ptr::null();
                    }
                    Ok(None) => {
                        (*result).value = false;
                        (*result).command = (&*query).currentCommand;
                        (*result).error = std::ptr::null();
                    }
                    Err(err) => {
                        (*result).value = false;
                        (*result).command = (&*query).currentCommand;
                        (*result).error = $crate::module_entry::result_to_error_ptr(Err(err));
                    }
                }
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn ActiveCommandChanged(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::ActiveCommandChangedQuery,
            result: *mut $crate::sys::ActiveCommandChangedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module().active_command_changed(
                        q.cmdID,
                        q.cmdType,
                        $crate::cstr_to_str!(q.name)?,
                        $crate::cstr_to_str!(q.action)?,
                        $crate::cstr_to_str!(q.tooltip)?,
                    )
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn CommandNotify(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::CommandNotifyQuery,
            result: *mut $crate::sys::BoolCallinResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                let callback_result =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                        data.module().command_notify((&*query).command)
                    }))
                    .unwrap_or_else(|_| Err($crate::Error::new(1, "Panic".to_string())));
                finish_bool_callback!(result, callback_result);
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn AddConsoleLine(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::AddConsoleLineQuery,
            result: *mut $crate::sys::BoolCallinResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                let callback_result =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                        let q = &*query;
                        data.module().add_console_line(
                            $crate::cstr_to_str!(q.message)?,
                            $crate::cstr_to_str!(q.section)?,
                            q.level,
                        )
                    }))
                    .unwrap_or_else(|_| Err($crate::Error::new(1, "Panic".to_string())));
                finish_bool_callback!(result, callback_result);
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn GroupChanged(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::GroupChangedQuery,
            result: *mut $crate::sys::BoolCallinResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                let callback_result =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                        data.module().group_changed((&*query).groupID)
                    }))
                    .unwrap_or_else(|_| Err($crate::Error::new(1, "Panic".to_string())));
                finish_bool_callback!(result, callback_result);
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn GameSetup(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::GameSetupQuery,
            result: *mut $crate::sys::GameSetupResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module()
                        .game_setup($crate::cstr_to_str!(q.state)?, q.ready)
                }))
                .unwrap_or_else(|_| Err($crate::Error::new(1, "Panic".to_string())))
                {
                    Ok(Some(ready)) => {
                        (*result).handled = true;
                        (*result).ready = ready;
                        (*result).error = std::ptr::null();
                    }
                    Ok(None) => {
                        (*result).handled = false;
                        (*result).ready = (&*query).ready;
                        (*result).error = std::ptr::null();
                    }
                    Err(err) => {
                        (*result).handled = false;
                        (*result).ready = (&*query).ready;
                        (*result).error = $crate::module_entry::result_to_error_ptr(Err(err));
                    }
                }
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn WorldTooltip(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::WorldTooltipQuery,
            result: *mut $crate::sys::StringCallinResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                let callback_result =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                        let q = &*query;
                        data.module()
                            .world_tooltip(q.kind, q.unitID, q.featureID, q.groundPos)
                    }))
                    .unwrap_or_else(|_| Err($crate::Error::new(1, "Panic".to_string())));
                finish_string_callback!(result, callback_result);
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn MapDrawCmd(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::MapDrawCmdQuery,
            result: *mut $crate::sys::BoolCallinResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                let callback_result =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                        let q = &*query;
                        let label = if q.hasLabel {
                            Some($crate::cstr_to_str!(q.label)?)
                        } else {
                            None
                        };
                        data.module().map_draw_cmd(
                            q.playerID,
                            q.type_,
                            if q.hasPos0 { Some(q.pos0) } else { None },
                            if q.hasPos1 { Some(q.pos1) } else { None },
                            label,
                        )
                    }))
                    .unwrap_or_else(|_| Err($crate::Error::new(1, "Panic".to_string())));
                finish_bool_callback!(result, callback_result);
            }
        }

        export_simple_callback!(
            ViewResize,
            view_resize,
            $crate::sys::ViewResizeQuery,
            $crate::sys::ViewResizeResult
        );
        export_simple_callback!(
            SunChanged,
            sun_changed,
            $crate::sys::SunChangedQuery,
            $crate::sys::SunChangedResult
        );
        export_simple_callback!(
            FontsChanged,
            fonts_changed,
            $crate::sys::SimpleCallinQuery,
            $crate::sys::SimpleCallinResult
        );

        #[no_mangle]
        pub unsafe extern "C" fn GameProgress(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::GameProgressQuery,
            result: *mut $crate::sys::GameProgressResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    data.module().game_progress((&*query).gameFrame)
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn StockpileChanged(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::StockpileChangedQuery,
            result: *mut $crate::sys::StockpileChangedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module().stockpile_changed(
                        q.unitID,
                        q.unitDefID,
                        q.unitTeam,
                        q.weaponNum,
                        q.oldCount,
                        q.newCount,
                    )
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn CollectGarbage(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::CollectGarbageQuery,
            result: *mut $crate::sys::CollectGarbageResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    data.module().collect_garbage((&*query).forced)
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn Pong(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::PongQuery,
            result: *mut $crate::sys::PongResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    data.module()
                        .pong(q.pingTag, q.packetSendTimeMillis, q.packetRecvTimeMillis)
                });
            }
        }

        // Special events
        #[no_mangle]
        pub unsafe extern "C" fn HandleLuaMsg(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::HandleLuaMsgQuery,
            result: *mut $crate::sys::HandleLuaMsgResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    if q.data.is_null() || q.dataLength == 0 {
                        data.module()
                            .handle_lua_msg(q.playerID, q.script, q.mode, &[])
                    } else {
                        let slice = std::slice::from_raw_parts(q.data, q.dataLength as usize);
                        data.module()
                            .handle_lua_msg(q.playerID, q.script, q.mode, slice)
                    }
                });
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn HandleLuaCall(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::HandleLuaCallQuery,
            result: *mut $crate::sys::HandleLuaCallResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() {
                return;
            }
            unsafe {
                (*result).error = $crate::module_entry::catch_panic_ffi(|| {
                    let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                    let q = &*query;
                    let message =
                        $crate::module_entry::lua_call_message_to_str(q.message, q.messageLength)?;
                    data.module().handle_lua_call(message)
                });
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::lua_call_message_to_str;

    #[test]
    fn lua_call_message_preserves_embedded_nul() {
        let bytes = b"{\"kind\":\"a\0b\"}";
        let message = unsafe {
            lua_call_message_to_str(
                bytes.as_ptr() as *const std::ffi::c_char,
                bytes.len() as u32,
            )
        }
        .expect("valid UTF-8 with embedded NUL should decode");

        assert_eq!(message.as_bytes(), bytes);
    }

    #[test]
    fn lua_call_message_rejects_null_pointer() {
        let error = unsafe { lua_call_message_to_str(std::ptr::null(), 0) }
            .expect_err("null pointer should be rejected");

        assert_eq!(error.message(), "Null message pointer");
    }
}
