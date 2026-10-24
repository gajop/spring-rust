/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

//! Module ownership and engine entry scopes. No mutable module borrow is held
//! across user code. Put independently mutable state behind Resource/RefCell,
//! and end those borrows before callouts that can reenter it.

use crate::{ApiError, Environment, ErrorCode, ModuleContext, Result, Spring};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

pub trait Module<E: Environment>: Sized + 'static {
    fn initialize(spring: &mut Spring<'_, E>) -> Result<Self>;
    /// Runs once the module is installed, right after `initialize` and before any callin, while
    /// the engine loads the module: Lua's `Initialize`. Unlike `initialize`, it may re-enter the
    /// module (its slot, CUS host and addon state are reachable). An error unloads the module.
    fn start(&self, _spring: &mut Spring<'_, E>) -> Result<()> {
        Ok(())
    }
    fn shutdown(&self, _spring: &mut Spring<'_, E>) -> Result<()> {
        Ok(())
    }
    /// Results arrive in submission order, before the next GameFrame (synced)
    /// or Update (local). The bytes belong to this callback; copy to retain them.
    fn job_result(
        &self,
        _spring: &mut Spring<'_, E>,
        _id: crate::jobs::JobId,
        _result: core::result::Result<&[u8], crate::jobs::JobError>,
    ) -> Result<()> {
        Ok(())
    }
}

struct Instance<E: Environment, T> {
    context: ModuleContext<E>,
    module: T,
}
/// One module instance per engine thread, normally declared by `export_module!`.
/// A second initialization is rejected until the first instance is shut down.
pub struct ModuleSlot<E: Environment, T> {
    instance: RefCell<Option<Rc<Instance<E, T>>>>,
    transitioning: Cell<bool>,
    poisoned: Cell<bool>,
    #[cfg(not(target_arch = "wasm32"))]
    native_gate: RefCell<Option<std::sync::Arc<crate::native_thread::Gate>>>,
}
impl<E: Environment, T: Module<E>> Default for ModuleSlot<E, T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<E: Environment, T: Module<E>> ModuleSlot<E, T> {
    pub const fn new() -> Self {
        Self {
            instance: RefCell::new(None),
            transitioning: Cell::new(false),
            poisoned: Cell::new(false),
            #[cfg(not(target_arch = "wasm32"))]
            native_gate: RefCell::new(None),
        }
    }

    /// # Safety
    /// Same admission requirements as `ModuleContext::from_native`.
    #[cfg(not(target_arch = "wasm32"))]
    pub unsafe fn initialize_native(
        &self,
        interface: *const crate::__sys::NativeInterface,
    ) -> Result<*mut core::ffi::c_void> {
        let context = unsafe {
            ModuleContext::<E>::from_native_with_gate(interface, self.native_gate.borrow().clone())?
        };
        self.initialize(context)?;
        Ok(self.identity())
    }

    /// # Safety
    /// The host must have admitted the module in environment E.
    #[cfg(target_arch = "wasm32")]
    pub unsafe fn initialize_wasm(&self) -> Result<()> {
        self.initialize(unsafe { ModuleContext::<E>::from_wasm() })
    }

    fn initialize(&self, context: ModuleContext<E>) -> Result<()> {
        if self.transitioning.replace(true) {
            return Err(invalid());
        }
        let result = self.boundary(|| {
            if self.instance.borrow().is_some() {
                return Err(invalid());
            }
            let module = context.with_spring(T::initialize)??;
            *self.instance.borrow_mut() = Some(Rc::new(Instance { context, module }));
            self.poisoned.set(false);
            Ok(())
        });
        self.transitioning.set(false);
        result?;
        self.invoke(|module, spring| module.start(spring))
    }

    /// Run a callin. Clone the instance before invoking user code so nested
    /// callins can enter immediately and no registry borrow crosses a callout.
    /// A module borrow cannot escape a callin even though the slot uses shared
    /// access and can be shut down through another shared reference.
    /// ```compile_fail
    /// use spring_sdk::{Menu,Spring,Result,module::{Module,ModuleSlot}};
    /// struct App;
    /// impl Module<Menu> for App {
    ///     fn initialize(_: &mut Spring<'_,Menu>)->Result<Self> {Ok(Self)}
    /// }
    /// fn escape(slot:&ModuleSlot<Menu,App>)->&App {
    ///     slot.invoke(|module,_|Ok(module)).unwrap()
    /// }
    /// ```
    pub fn invoke<R>(
        &self,
        handler: impl for<'call> FnOnce(&T, &mut Spring<'call, E>) -> Result<R>,
    ) -> Result<R> {
        if self.transitioning.get() || self.poisoned.get() {
            return Err(invalid());
        }
        self.boundary(|| {
            let instance = self
                .instance
                .borrow()
                .as_ref()
                .cloned()
                .ok_or_else(invalid)?;
            instance
                .context
                .with_spring(|spring| handler(&instance.module, spring))?
        })
    }

    #[doc(hidden)]
    pub fn invoke_for_environment<const MASK: u32, R>(
        &self,
        handler: impl for<'call> FnOnce(&T, &mut Spring<'call, E>) -> Result<R>,
    ) -> Result<R> {
        const {
            assert!(
                MASK & E::MASK != 0,
                "callin is not available in this module environment"
            );
        }
        self.invoke(handler)
    }

    pub fn shutdown(&self) -> Result<()> {
        if self.transitioning.replace(true) {
            return Err(invalid());
        }
        let result = self.boundary(|| {
            let instance = self.instance.borrow_mut().take().ok_or_else(invalid)?;
            if self.poisoned.get() {
                return Ok(());
            }
            instance
                .context
                .with_spring(|spring| instance.module.shutdown(spring))?
        });
        self.transitioning.set(false);
        result
    }

    #[doc(hidden)]
    pub fn deliver_job_result(
        &self,
        id: crate::jobs::JobId,
        result: core::result::Result<&[u8], crate::jobs::JobError>,
    ) -> Result<()> {
        self.invoke(|module, spring| module.job_result(spring, id, result))
    }

    #[doc(hidden)]
    pub fn invoke_tick_for_environment<const MASK: u32, const PHASE: u8, R>(
        &self,
        handler: impl for<'call> FnOnce(&T, &mut Spring<'call, E>) -> Result<R>,
    ) -> Result<R> {
        self.invoke_for_environment::<MASK, _>(|module, spring| {
            #[cfg(not(target_arch = "wasm32"))]
            let advance = crate::jobs::native::is_phase::<E>(PHASE)
                && !spring.engine.owner.jobs.borrow().advancing();
            #[cfg(not(target_arch = "wasm32"))]
            if advance {
                let deliveries = spring.engine.owner.jobs.borrow_mut().begin()?;
                // Release the queue borrow before user callbacks. They may
                // submit more work or make reentrant engine calls.
                for (id, result) in deliveries {
                    module.job_result(spring, id, result.as_deref().map_err(|e| *e))?;
                }
            }
            let result = handler(module, spring)?;
            #[cfg(not(target_arch = "wasm32"))]
            if advance {
                spring.engine.owner.jobs.borrow_mut().end()?;
            }
            Ok(result)
        })
    }

    #[doc(hidden)]
    pub fn identity(&self) -> *mut core::ffi::c_void {
        self.instance
            .borrow()
            .as_ref()
            .map_or(core::ptr::null_mut(), |instance| {
                Rc::as_ptr(instance).cast_mut().cast()
            })
    }
    #[doc(hidden)]
    pub fn validate_identity(&self, identity: *mut core::ffi::c_void) -> Result<()> {
        if identity.is_null() || identity != self.identity() {
            Err(invalid())
        } else {
            Ok(())
        }
    }

    #[cold]
    #[inline(never)]
    fn fail(&self) {
        self.poisoned.set(true);
        if let Some(instance) = self.instance.borrow().as_ref() {
            instance.context.owner.alive.set(false);
        }
    }

    fn boundary<R>(&self, call: impl FnOnce() -> Result<R>) -> Result<R> {
        #[cfg(not(target_arch = "wasm32"))]
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(call))
            .unwrap_or_else(|_| Err(invalid()));
        #[cfg(target_arch = "wasm32")]
        let result = call();
        if result.is_err() {
            self.fail();
        }
        result
    }
}
fn invalid() -> ApiError {
    ApiError::new(ErrorCode::InvalidState as i32)
}

/// Trap errors at a Wasm entry boundary. The engine records the trap and
/// unloads the module; continuing with partially updated game state is unsafe.
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
pub fn wasm_boundary<R>(result: Result<R>) -> R {
    result.unwrap_or_else(|_| core::arch::wasm32::unreachable())
}

#[cfg(not(target_arch = "wasm32"))]
thread_local! {
    static ERROR:RefCell<crate::__sys::Error>=const {RefCell::new(crate::__sys::Error {code:0,message:core::ptr::null()})};
    static ERROR_MESSAGE:RefCell<std::ffi::CString>=RefCell::new(std::ffi::CString::default());
}
/// Error storage belongs to this thread and remains valid until the next
/// callin error is returned. The host reads it immediately at the boundary.
#[cfg(not(target_arch = "wasm32"))]
#[doc(hidden)]
#[inline]
pub fn error_pointer(result: Result<()>) -> *const crate::__sys::Error {
    match result {
        Ok(()) => core::ptr::null(),
        Err(error) => native_error(error),
    }
}
#[cfg(not(target_arch = "wasm32"))]
#[cold]
#[inline(never)]
fn native_error(error: ApiError) -> *const crate::__sys::Error {
    ERROR.with(|slot| {
        let mut slot = slot.borrow_mut();
        slot.code = error.code;
        ERROR_MESSAGE.with(|message| {
            let mut message = message.borrow_mut();
            *message = std::ffi::CString::new(format!(
                "spring-sdk module callin failed (error code {})",
                error.code
            ))
            .unwrap();
            slot.message = message.as_ptr();
        });
        &*slot as *const _
    })
}

/// # Safety
/// Pointer must reference a host query byte array of `length` bytes.
#[cfg(not(target_arch = "wasm32"))]
#[doc(hidden)]
pub unsafe fn copy_message(pointer: *const u8, length: usize) -> Result<Vec<u8>> {
    Ok(unsafe { crate::native::read_slice(pointer, length)? }.to_vec())
}

#[doc(hidden)]
pub fn decode_string(bytes: Vec<u8>) -> String {
    match String::from_utf8(bytes) {
        Ok(value) => value,
        Err(error) => error.into_bytes().into_iter().map(char::from).collect(),
    }
}
fn word(bytes: &[u8], offset: usize) -> Result<u32> {
    let end = offset.checked_add(4).ok_or_else(invalid)?;
    Ok(u32::from_le_bytes(
        bytes
            .get(offset..end)
            .ok_or_else(invalid)?
            .try_into()
            .unwrap(),
    ))
}
/// Reviewed specialized console codec: offset/length pairs then level.
#[doc(hidden)]
pub fn console_payload(bytes: &[u8]) -> Result<(&[u8], &[u8], i32)> {
    if bytes.len() < 20 {
        return Err(invalid());
    }
    let span = |offset: usize| -> Result<&[u8]> {
        let begin = word(bytes, offset)? as usize;
        let length = word(bytes, offset + 4)? as usize;
        if begin < 20 {
            return Err(invalid());
        }
        bytes
            .get(begin..begin.checked_add(length).ok_or_else(invalid)?)
            .ok_or_else(invalid)
    };
    Ok((span(0)?, span(8)?, word(bytes, 16)? as i32))
}
/// Reviewed specialized command codec: six words followed by f32 parameters.
#[doc(hidden)]
pub fn command_payload(bytes: &[u8]) -> Result<(i32, i32, u32, u32, u8, Vec<f32>)> {
    let count = word(bytes, 12)? as usize;
    let expected = count
        .checked_mul(4)
        .and_then(|n| n.checked_add(24))
        .ok_or_else(invalid)?;
    if bytes.len() != expected {
        return Err(invalid());
    }
    let options = word(bytes, 20)?.try_into().map_err(|_| invalid())?;
    let params = bytes[24..]
        .as_chunks::<4>()
        .0
        .iter()
        .map(|b| f32::from_le_bytes(*b))
        .collect();
    Ok((
        word(bytes, 0)? as i32,
        word(bytes, 4)? as i32,
        word(bytes, 8)?,
        word(bytes, 16)?,
        options,
        params,
    ))
}

/// Native storage shared by initialization, subsequent engine threads, and
/// shutdown. The module is Send so its owned state can move between serialized
/// engine entries. A scoped Spring reference still cannot leave a callin.
#[cfg(not(target_arch = "wasm32"))]
pub struct NativeSlot<E: Environment, T> {
    gate: std::sync::OnceLock<std::sync::Arc<crate::native_thread::Gate>>,
    slot: std::cell::UnsafeCell<ModuleSlot<E, T>>,
}
// The gate excludes concurrent accesses to the slot. Its internal Rc instance
// clones never escape an entry. T and retained user captures are Send; no user
// value is accessed while another engine thread owns the gate.
#[cfg(not(target_arch = "wasm32"))]
unsafe impl<E: Environment, T: Send> Sync for NativeSlot<E, T> {}
#[cfg(not(target_arch = "wasm32"))]
impl<E: Environment, T: Module<E> + Send> Default for NativeSlot<E, T> {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(not(target_arch = "wasm32"))]
impl<E: Environment, T: Module<E> + Send> NativeSlot<E, T> {
    pub const fn new() -> Self {
        Self {
            gate: std::sync::OnceLock::new(),
            slot: std::cell::UnsafeCell::new(ModuleSlot::new()),
        }
    }
    /// Read/invoke from the current engine thread. This cannot admit a worker.
    pub fn with<R>(&self, call: impl for<'a> FnOnce(&'a ModuleSlot<E, T>) -> R) -> R {
        let gate = self.gate.get().expect("native module has not initialized");
        let _entry = gate
            .enter()
            .expect("native module accessed outside engine thread");
        call(unsafe { &*self.slot.get() })
    }
    /// # Safety
    /// Called only from an engine ABI entry for this module, with live host
    /// arguments. The host must finish all entries before unloading the DLL.
    #[doc(hidden)]
    pub unsafe fn with_host<R>(&self, call: impl for<'a> FnOnce(&'a ModuleSlot<E, T>) -> R) -> R {
        let gate = self.gate.get_or_init(|| {
            let gate = std::sync::Arc::new(crate::native_thread::Gate::new());
            // OnceLock excludes all slot access until publication. Bind
            // the context gate once, rather than borrowing this RefCell
            // on every subsequent engine entry.
            unsafe { &*self.slot.get() }
                .native_gate
                .replace(Some(gate.clone()));
            gate
        });
        let _entry = unsafe { gate.enter_host() };
        let slot = unsafe { &*self.slot.get() };
        call(slot)
    }
}
