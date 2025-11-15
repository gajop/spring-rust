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
        pub extern "C" fn $name(
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
        pub extern "C" fn InitializeNativeModule(
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
                        q.hostVersionMajor, q.hostVersionMinor, q.hostVersionPatch,
                        $crate::NATIVE_API_VERSION_MAJOR,
                        $crate::NATIVE_API_VERSION_MINOR,
                        $crate::NATIVE_API_VERSION_PATCH
                    );
                    (*result).error = $crate::module_entry::result_to_error_ptr(
                        Err($crate::Error::new(1, msg))
                    );
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
                pub extern "C" fn $name(
                    _interface: *const $crate::sys::NativeInterface,
                    module_data: *mut c_void,
                    _query: *const $query,
                    result: *mut $result,
                ) {
                    if module_data.is_null() || result.is_null() {
                        return;
                    }
                    unsafe {
                        let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                        let res = data.module().$method();
                        (*result).error = $crate::module_entry::result_to_error_ptr(res);
                    }
                }
            };
        }

        // Game events
        export_simple_callback!(GamePreload, game_preload, $crate::sys::GamePreloadQuery, $crate::sys::GamePreloadResult);
        export_simple_callback!(GameStart, game_start, $crate::sys::GameStartQuery, $crate::sys::GameStartResult);
        export_simple_callback!(Shutdown, shutdown, $crate::sys::ShutdownQuery, $crate::sys::ShutdownResult);

        // Download events
        #[no_mangle]
        pub extern "C" fn DownloadFailed(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::DownloadFailedQuery,
            result: *mut $crate::sys::DownloadFailedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() { return; }
            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                let q = &*query;
                (*result).error = $crate::module_entry::result_to_error_ptr(
                    data.module().download_failed(q.downloadID, q.errorID)
                );
            }
        }

        #[no_mangle]
        pub extern "C" fn DownloadFinished(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::DownloadFinishedQuery,
            result: *mut $crate::sys::DownloadFinishedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() { return; }
            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                (*result).error = $crate::module_entry::result_to_error_ptr(
                    data.module().download_finished((&*query).downloadID)
                );
            }
        }

        #[no_mangle]
        pub extern "C" fn DownloadProgress(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::DownloadProgressQuery,
            result: *mut $crate::sys::DownloadProgressResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() { return; }
            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                let q = &*query;
                (*result).error = $crate::module_entry::result_to_error_ptr(
                    data.module().download_progress(q.downloadID, q.downloaded, q.total)
                );
            }
        }

        #[no_mangle]
        pub extern "C" fn DownloadQueued(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::DownloadQueuedQuery,
            result: *mut $crate::sys::DownloadQueuedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() { return; }
            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                let q = &*query;
                let res = if q.archiveName.is_null() || q.archiveType.is_null() {
                    Err($crate::Error::new(1, "Null string pointer".to_string()))
                } else {
                    match std::ffi::CStr::from_ptr(q.archiveName).to_str() {
                        Ok(name) => match std::ffi::CStr::from_ptr(q.archiveType).to_str() {
                            Ok(atype) => data.module().download_queued(q.downloadID, name, atype),
                            Err(e) => Err($crate::Error::new(1, format!("Invalid UTF-8: {}", e))),
                        },
                        Err(e) => Err($crate::Error::new(1, format!("Invalid UTF-8: {}", e))),
                    }
                };
                (*result).error = $crate::module_entry::result_to_error_ptr(res);
            }
        }

        #[no_mangle]
        pub extern "C" fn DownloadStarted(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::DownloadStartedQuery,
            result: *mut $crate::sys::DownloadStartedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() { return; }
            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                (*result).error = $crate::module_entry::result_to_error_ptr(
                    data.module().download_started((&*query).downloadID)
                );
            }
        }

        // Feature events
        #[no_mangle]
        pub extern "C" fn FeatureCreated(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::FeatureCreatedQuery,
            result: *mut $crate::sys::FeatureCreatedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() { return; }
            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                (*result).error = $crate::module_entry::result_to_error_ptr(
                    data.module().feature_created((&*query).featureID)
                );
            }
        }

        #[no_mangle]
        pub extern "C" fn FeatureDestroyed(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::FeatureDestroyedQuery,
            result: *mut $crate::sys::FeatureDestroyedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() { return; }
            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                (*result).error = $crate::module_entry::result_to_error_ptr(
                    data.module().feature_destroyed((&*query).featureID)
                );
            }
        }

        // Game events
        #[no_mangle]
        pub extern "C" fn GameID(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::GameIDQuery,
            result: *mut $crate::sys::GameIDResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() { return; }
            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                let q = &*query;
                let res = if q.gameID.is_null() || q.numBytes == 0 {
                    data.module().game_id(&[])
                } else {
                    let slice = std::slice::from_raw_parts(q.gameID, q.numBytes as usize);
                    data.module().game_id(slice)
                };
                (*result).error = $crate::module_entry::result_to_error_ptr(res);
            }
        }

        #[no_mangle]
        pub extern "C" fn GamePaused(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::GamePausedQuery,
            result: *mut $crate::sys::GamePausedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() { return; }
            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                let q = &*query;
                (*result).error = $crate::module_entry::result_to_error_ptr(
                    data.module().game_paused(q.playerID, q.paused)
                );
            }
        }

        // Player events
        #[no_mangle]
        pub extern "C" fn PlayerAdded(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::PlayerAddedQuery,
            result: *mut $crate::sys::PlayerAddedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() { return; }
            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                (*result).error = $crate::module_entry::result_to_error_ptr(
                    data.module().player_added((&*query).playerID)
                );
            }
        }

        #[no_mangle]
        pub extern "C" fn PlayerChanged(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::PlayerChangedQuery,
            result: *mut $crate::sys::PlayerChangedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() { return; }
            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                (*result).error = $crate::module_entry::result_to_error_ptr(
                    data.module().player_changed((&*query).playerID)
                );
            }
        }

        #[no_mangle]
        pub extern "C" fn PlayerRemoved(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::PlayerRemovedQuery,
            result: *mut $crate::sys::PlayerRemovedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() { return; }
            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                let q = &*query;
                (*result).error = $crate::module_entry::result_to_error_ptr(
                    data.module().player_removed(q.playerID, q.reason)
                );
            }
        }

        // Team events
        #[no_mangle]
        pub extern "C" fn TeamChanged(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::TeamChangedQuery,
            result: *mut $crate::sys::TeamChangedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() { return; }
            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                (*result).error = $crate::module_entry::result_to_error_ptr(
                    data.module().team_changed((&*query).teamID)
                );
            }
        }

        #[no_mangle]
        pub extern "C" fn TeamDied(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::TeamDiedQuery,
            result: *mut $crate::sys::TeamDiedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() { return; }
            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                (*result).error = $crate::module_entry::result_to_error_ptr(
                    data.module().team_died((&*query).teamID)
                );
            }
        }

        // Unit events
        #[no_mangle]
        pub extern "C" fn UnitCreated(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::UnitCreatedQuery,
            result: *mut $crate::sys::UnitCreatedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() { return; }
            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                let q = &*query;
                (*result).error = $crate::module_entry::result_to_error_ptr(
                    data.module().unit_created(q.unitID, q.builderID)
                );
            }
        }

        #[no_mangle]
        pub extern "C" fn UnitDestroyed(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::UnitDestroyedQuery,
            result: *mut $crate::sys::UnitDestroyedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() { return; }
            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                let q = &*query;
                (*result).error = $crate::module_entry::result_to_error_ptr(
                    data.module().unit_destroyed(q.unitID, q.attackerID)
                );
            }
        }

        #[no_mangle]
        pub extern "C" fn UnitExperience(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::UnitExperienceQuery,
            result: *mut $crate::sys::UnitExperienceResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() { return; }
            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                let q = &*query;
                (*result).error = $crate::module_entry::result_to_error_ptr(
                    data.module().unit_experience(q.unitID, q.oldExperience)
                );
            }
        }

        #[no_mangle]
        pub extern "C" fn UnitFinished(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::UnitFinishedQuery,
            result: *mut $crate::sys::UnitFinishedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() { return; }
            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                (*result).error = $crate::module_entry::result_to_error_ptr(
                    data.module().unit_finished((&*query).unitID)
                );
            }
        }

        #[no_mangle]
        pub extern "C" fn UnitFromFactory(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::UnitFromFactoryQuery,
            result: *mut $crate::sys::UnitFromFactoryResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() { return; }
            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                let q = &*query;
                (*result).error = $crate::module_entry::result_to_error_ptr(
                    data.module().unit_from_factory(q.unitID, q.factoryID, q.userOrders)
                );
            }
        }

        #[no_mangle]
        pub extern "C" fn UnitGiven(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::UnitGivenQuery,
            result: *mut $crate::sys::UnitGivenResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() { return; }
            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                let q = &*query;
                (*result).error = $crate::module_entry::result_to_error_ptr(
                    data.module().unit_given(q.unitID, q.oldTeam, q.newTeam)
                );
            }
        }

        #[no_mangle]
        pub extern "C" fn UnitLoaded(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::UnitLoadedQuery,
            result: *mut $crate::sys::UnitLoadedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() { return; }
            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                let q = &*query;
                (*result).error = $crate::module_entry::result_to_error_ptr(
                    data.module().unit_loaded(q.unitID, q.transportID)
                );
            }
        }

        #[no_mangle]
        pub extern "C" fn UnitStunned(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::UnitStunnedQuery,
            result: *mut $crate::sys::UnitStunnedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() { return; }
            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                let q = &*query;
                (*result).error = $crate::module_entry::result_to_error_ptr(
                    data.module().unit_stunned(q.unitID, q.stunned)
                );
            }
        }

        #[no_mangle]
        pub extern "C" fn UnitTaken(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::UnitTakenQuery,
            result: *mut $crate::sys::UnitTakenResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() { return; }
            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                let q = &*query;
                (*result).error = $crate::module_entry::result_to_error_ptr(
                    data.module().unit_taken(q.unitID, q.oldTeam, q.newTeam)
                );
            }
        }

        #[no_mangle]
        pub extern "C" fn UnitUnloaded(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::UnitUnloadedQuery,
            result: *mut $crate::sys::UnitUnloadedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() { return; }
            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                let q = &*query;
                (*result).error = $crate::module_entry::result_to_error_ptr(
                    data.module().unit_unloaded(q.unitID, q.transportID)
                );
            }
        }

        #[no_mangle]
        pub extern "C" fn RenderUnitDestroyed(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::RenderUnitDestroyedQuery,
            result: *mut $crate::sys::RenderUnitDestroyedResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() { return; }
            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                (*result).error = $crate::module_entry::result_to_error_ptr(
                    data.module().render_unit_destroyed((&*query).unitID)
                );
            }
        }

        // Special events
        #[no_mangle]
        pub extern "C" fn HandleLuaMsg(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::HandleLuaMsgQuery,
            result: *mut $crate::sys::HandleLuaMsgResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() { return; }
            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                let q = &*query;
                let res = if q.data.is_null() || q.dataLength == 0 {
                    data.module().handle_lua_msg(q.playerID, q.script, q.mode, &[])
                } else {
                    let slice = std::slice::from_raw_parts(q.data, q.dataLength as usize);
                    data.module().handle_lua_msg(q.playerID, q.script, q.mode, slice)
                };
                (*result).error = $crate::module_entry::result_to_error_ptr(res);
            }
        }

        #[no_mangle]
        pub extern "C" fn HandleLuaCall(
            _interface: *const $crate::sys::NativeInterface,
            module_data: *mut c_void,
            query: *const $crate::sys::HandleLuaCallQuery,
            result: *mut $crate::sys::HandleLuaCallResult,
        ) {
            if module_data.is_null() || query.is_null() || result.is_null() { return; }
            unsafe {
                let data = &mut *(module_data as *mut $crate::ModuleData<$module_type>);
                let q = &*query;
                let res = if q.message.is_null() {
                    Err($crate::Error::new(1, "Null message pointer"))
                } else {
                    match std::ffi::CStr::from_ptr(q.message).to_str() {
                        Ok(message) => data.module().handle_lua_call(message),
                        Err(e) => Err($crate::Error::new(1, format!("Invalid UTF-8: {}", e))),
                    }
                };
                (*result).error = $crate::module_entry::result_to_error_ptr(res);
            }
        }
    };
}
