/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

//! Scoped engine access shared by native and WebAssembly modules.

extern crate alloc;
#[cfg(all(not(target_arch = "wasm32"), panic = "abort"))]
compile_error!(
    "native spring-sdk modules require panic=unwind to contain panics at C ABI boundaries"
);
use core::marker::PhantomData;
#[cfg(not(target_arch = "wasm32"))]
#[doc(hidden)]
pub use spring_native_sys as __sys;
pub use spring_sdk_internal::{
    ApiError, DamageResult, ErrorCode, Float3, Result, UnitId, WeaponDefId,
};
use std::cell::Cell;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc as Shared;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Arc as Shared;
#[cfg(not(target_arch = "wasm32"))]
mod native_thread;
#[path = "../../spring-sdk-internal/src/pcg.rs"]
mod pcg;

#[cfg(not(target_arch = "wasm32"))]
type Weak<T> = std::sync::Weak<T>;
#[cfg(target_arch = "wasm32")]
type Weak<T> = std::rc::Weak<T>;

/// Native retained captures must survive the loading-to-game-thread handoff.
#[cfg(not(target_arch = "wasm32"))]
pub trait RetainedCapture: Send {}
#[cfg(not(target_arch = "wasm32"))]
impl<T: Send> RetainedCapture for T {}
#[cfg(target_arch = "wasm32")]
pub trait RetainedCapture {}
#[cfg(target_arch = "wasm32")]
impl<T> RetainedCapture for T {}

mod sealed {
    pub trait Environment {}
}

/// An engine execution environment. Implementations are sealed.
pub trait Environment: sealed::Environment + 'static {
    const MASK: u32;
}

macro_rules! environments {
    ($($name:ident = $mask:expr),* $(,)?) => {$ (
        pub enum $name {}
        impl sealed::Environment for $name {}
        impl Environment for $name { const MASK:u32 = $mask; }
    )*};
}
environments!(
    RulesSynced = 1,
    RulesUnsynced = 2,
    GaiaSynced = 4,
    GaiaUnsynced = 8,
    Ui = 16,
    Menu = 32,
    Intro = 64,
    All = 127
);

pub mod capabilities {
    use super::*;
    macro_rules! capability {
        ($name:ident: [$($parent:ident),*] => $($environment:ident),*) => {
            pub trait $name: Environment $(+ $parent)* {}
            $(impl $name for $environment {})*
            impl $name for All {}
        };
    }
    capability!(Common: [] => RulesSynced, RulesUnsynced, GaiaSynced, GaiaUnsynced, Ui, Menu, Intro);
    capability!(Render: [Common] => RulesUnsynced, GaiaUnsynced, Ui, Menu, Intro);
    capability!(World: [Common] => RulesSynced, RulesUnsynced, GaiaSynced, GaiaUnsynced, Ui, Intro);
    capability!(Launch: [Common] => RulesSynced, RulesUnsynced, GaiaSynced, GaiaUnsynced, Ui, Menu);
    capability!(Game: [World, Launch] => RulesSynced, RulesUnsynced, GaiaSynced, GaiaUnsynced, Ui);
    capability!(Session: [Render, Launch] => RulesUnsynced, GaiaUnsynced, Ui, Menu);
    capability!(Synced: [Game] => RulesSynced, GaiaSynced);
    capability!(Local: [Game, Session] => RulesUnsynced, GaiaUnsynced, Ui);
    capability!(Downloads: [Session] => Ui, Menu);
    capability!(Replay: [Render, World] => RulesUnsynced, GaiaUnsynced, Ui, Intro);
}

/// Access to Spring for one engine callin. It cannot be stored beyond that
/// callin or sent/shared with a worker thread. It does not borrow game state.
///
/// Engine references cannot be sent to workers:
/// ```compile_fail
/// use spring_sdk::{Spring,Ui};
/// fn worker(spring: &mut Spring<'_,Ui>) {
///     std::thread::scope(|threads| { threads.spawn(move || { let _=spring.game().get_game_frame(); }); });
/// }
/// ```
/// Synced modules cannot invoke rendering calls:
/// ```compile_fail
/// use spring_sdk::{Spring,RulesSynced};
/// fn render(spring: &mut Spring<'_,RulesSynced>) {
///     let _=spring.gfx().get_matrix_data(0);
/// }
/// ```
/// UI modules cannot mutate synced units:
/// ```compile_fail
/// use spring_sdk::{Spring,Ui,unit_control::DestroyUnitOptions};
/// fn mutate(spring: &mut Spring<'_,Ui>) {
///     let _=spring.unit_control().destroy_unit(1,DestroyUnitOptions::default());
/// }
/// ```
/// Snapshot modules expose values, with no ambient engine functions:
/// ```compile_fail
/// use spring_sdk::types::game::get_game_frame;
/// ```
pub struct Spring<'call, E: Environment> {
    engine: Backend<'call>,
    environment: PhantomData<E>,
}

impl<E: Environment> Spring<'_, E> {
    /// Keep module identity in a long-lived model. Engine access through it
    /// still requires an active callin of this module.
    pub fn module_ref(&self) -> ModuleRef<E> {
        ModuleRef {
            owner: self.engine.owner.identity.clone(),
            environment: PhantomData,
        }
    }

    /// Pass another scoped handle to a helper while retaining the environment.
    /// It stays tied to this callin and cannot be stored or sent to a worker.
    pub fn reborrow(&self) -> Spring<'_, E> {
        Spring {
            engine: Backend {
                owner: self.engine.owner,
                scope: PhantomData,
            },
            environment: PhantomData,
        }
    }
}

/// Module context created at its engine entry point. Nested callins can enter
/// without borrowing the module's mutable application state.
pub struct ModuleContext<E: Environment> {
    owner: Shared<Authority>,
    environment: PhantomData<E>,
    affinity: PhantomData<*mut ()>,
}

impl<E: Environment> ModuleContext<E> {
    /// # Safety
    /// Called only by the module's engine initialization entry point, on the
    /// engine callin thread. The interface and its tables must remain valid
    /// until the context is dropped, and belong to environment `E`.
    #[cfg(not(target_arch = "wasm32"))]
    pub unsafe fn from_native(raw: *const spring_native_sys::NativeInterface) -> Result<Self> {
        unsafe { Self::from_native_with_gate(raw, None) }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) unsafe fn from_native_with_gate(
        raw: *const spring_native_sys::NativeInterface,
        gate: Option<Shared<native_thread::Gate>>,
    ) -> Result<Self> {
        if raw.is_null() {
            return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
        }
        Ok(Self {
            owner: Shared::new_cyclic(|identity| Authority {
                identity: identity.clone(),
                raw,
                native_tables: unsafe { native::Tables::new(raw) },
                gate: gate.unwrap_or_else(|| Shared::new(native_thread::Gate::new())),
                #[cfg(target_arch = "wasm32")]
                scopes: Cell::new(0),
                alive: Cell::new(true),
                error: Cell::new(None),
                unsynced_rng: Cell::new(None),
                panic: Default::default(),
                callback_failed: Default::default(),
                jobs: Default::default(),
            }),
            environment: PhantomData,
            affinity: PhantomData,
        })
    }

    /// # Safety
    /// The module must export `E::MASK` and be loaded in that environment.
    #[cfg(target_arch = "wasm32")]
    pub unsafe fn from_wasm() -> Self {
        Self {
            owner: Shared::new_cyclic(|identity| Authority {
                identity: identity.clone(),
                #[cfg(target_arch = "wasm32")]
                scopes: Cell::new(0),
                alive: Cell::new(true),
                error: Cell::new(None),
                unsynced_rng: Cell::new(None),
            }),
            environment: PhantomData,
            affinity: PhantomData,
        }
    }

    /// Enter an engine callin. The higher-ranked lifetime prevents a Spring
    /// handle (or one of its API families) escaping the callin. Nested calls
    /// enter another independent scope on the same engine thread.
    ///
    /// An API family cannot escape its callin:
    /// ```compile_fail
    /// use spring_sdk::{ModuleContext,Ui};
    /// fn escape(context: &ModuleContext<Ui>) {
    ///     let _leaked=context.with_spring(|spring| spring.game());
    /// }
    /// ```
    pub fn with_spring<R>(
        &self,
        callin: impl for<'call> FnOnce(&mut Spring<'call, E>) -> R,
    ) -> Result<R> {
        #[cfg(not(target_arch = "wasm32"))]
        let _entry = self.owner.gate.enter()?;
        let mut spring = Spring {
            engine: Backend {
                owner: &self.owner,
                scope: PhantomData,
            },
            environment: PhantomData,
        };
        spring.engine.check()?;
        let _scope = ActiveScope::enter(&self.owner);
        let result = callin(&mut spring);
        spring.engine.check()?;
        Ok(result)
    }
}

impl<E: Environment> Drop for ModuleContext<E> {
    fn drop(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        let _entry = self
            .owner
            .gate
            .enter()
            .expect("module context dropped outside engine entry");
        self.owner.alive.set(false);
        callbacks::clear_owner(&self.owner);
        #[cfg(not(target_arch = "wasm32"))]
        self.owner.jobs.borrow_mut().shutdown();
    }
}

struct Authority {
    identity: Weak<Authority>,
    #[cfg(not(target_arch = "wasm32"))]
    raw: *const spring_native_sys::NativeInterface,
    #[cfg(not(target_arch = "wasm32"))]
    native_tables: native::Tables,
    #[cfg(not(target_arch = "wasm32"))]
    gate: Shared<native_thread::Gate>,
    #[cfg(target_arch = "wasm32")]
    scopes: Cell<usize>,
    alive: Cell<bool>,
    error: Cell<Option<ApiError>>,
    unsynced_rng: Cell<Option<pcg::Pcg32>>,
    #[cfg(not(target_arch = "wasm32"))]
    panic: std::cell::RefCell<Option<Box<dyn core::any::Any + Send>>>,
    #[cfg(not(target_arch = "wasm32"))]
    callback_failed: std::sync::atomic::AtomicBool,
    #[cfg(not(target_arch = "wasm32"))]
    jobs: std::cell::RefCell<jobs::native::State>,
}

// Native access to Cells is serialized by gate. Authority never escapes the
// SDK: contexts and Spring tokens are !Send/!Sync; retained handlers are Send
// and invoke under this same gate. The host owns the interface lifetime.
#[cfg(not(target_arch = "wasm32"))]
unsafe impl Send for Authority {}
#[cfg(not(target_arch = "wasm32"))]
unsafe impl Sync for Authority {}

#[cfg(not(target_arch = "wasm32"))]
thread_local! {
    static ACTIVE_OWNER: Cell<*const Authority> = const { Cell::new(core::ptr::null()) };
    static OUTER_OWNERS: std::cell::RefCell<Vec<*const Authority>> = const { std::cell::RefCell::new(Vec::new()) };
}
struct ActiveScope<'a> {
    owner: &'a Authority,
    #[cfg(not(target_arch = "wasm32"))]
    previous: *const Authority,
    affinity: PhantomData<*mut ()>,
}
impl<'a> ActiveScope<'a> {
    #[inline]
    fn enter(owner: &'a Authority) -> Self {
        #[cfg(target_arch = "wasm32")]
        owner.scopes.set(owner.scopes.get() + 1);
        #[cfg(not(target_arch = "wasm32"))]
        let previous = ACTIVE_OWNER.with(|active| {
            let previous = active.get();
            if !core::ptr::eq(previous, owner) {
                active.set(owner);
            }
            previous
        });
        #[cfg(not(target_arch = "wasm32"))]
        if !previous.is_null() && !core::ptr::eq(previous, owner) {
            Self::push_outer(previous);
        }
        Self {
            owner,
            #[cfg(not(target_arch = "wasm32"))]
            previous,
            affinity: PhantomData,
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cold]
    #[inline(never)]
    fn push_outer(owner: *const Authority) {
        OUTER_OWNERS.with(|owners| owners.borrow_mut().push(owner));
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[cold]
    #[inline(never)]
    fn pop_outer(owner: *const Authority) {
        OUTER_OWNERS.with(|owners| {
            let removed = owners.borrow_mut().pop();
            debug_assert_eq!(removed, Some(owner));
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    #[inline]
    fn contains(owner: *const Authority) -> bool {
        ACTIVE_OWNER.with(|active| active.get() == owner)
            || OUTER_OWNERS.with(|owners| owners.borrow().contains(&owner))
    }
}
impl Drop for ActiveScope<'_> {
    #[inline]
    fn drop(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if !self.previous.is_null() && !core::ptr::eq(self.previous, self.owner) {
                Self::pop_outer(self.previous);
            }
            if !core::ptr::eq(self.previous, self.owner) {
                ACTIVE_OWNER.with(|active| active.set(self.previous));
            }
        }
        #[cfg(target_arch = "wasm32")]
        self.owner.scopes.set(self.owner.scopes.get() - 1);
    }
}

/// A module identifier for models that outlive one callin. It contains no
/// engine API table. Each use obtains a fresh scoped Spring reference, only
/// while this module is already inside an engine callin. Native identifiers
/// can move with module state; worker threads cannot obtain engine access.
pub struct ModuleRef<E: Environment> {
    #[cfg(not(target_arch = "wasm32"))]
    owner: std::sync::Weak<Authority>,
    #[cfg(target_arch = "wasm32")]
    owner: std::rc::Weak<Authority>,
    environment: PhantomData<fn() -> E>,
}
impl<E: Environment> Clone for ModuleRef<E> {
    fn clone(&self) -> Self {
        Self {
            owner: self.owner.clone(),
            environment: PhantomData,
        }
    }
}
impl<E: Environment> ModuleRef<E> {
    pub fn with_spring<R>(
        &self,
        call: impl for<'call> FnOnce(&mut Spring<'call, E>) -> Result<R>,
    ) -> Result<R> {
        let invalid = || ApiError::new(ErrorCode::InvalidState as i32);
        #[cfg(target_arch = "wasm32")]
        let strong = self.owner.upgrade().ok_or_else(invalid)?;
        #[cfg(target_arch = "wasm32")]
        let owner = &*strong;
        #[cfg(not(target_arch = "wasm32"))]
        let owner = {
            let pointer = self.owner.as_ptr();
            if !ActiveScope::contains(pointer) {
                return Err(invalid());
            }
            // TLS membership proves a live enclosing scope holds this
            // authority and its entry guard. Weak retains the allocation's
            // identity, so a dead owner's address cannot be reused here.
            unsafe { &*pointer }
        };
        #[cfg(target_arch = "wasm32")]
        if owner.scopes.get() == 0 {
            return Err(invalid());
        }
        let engine = Backend {
            owner,
            scope: PhantomData,
        };
        engine.check()?;
        let result = engine.with_nested(call);
        engine.finish(result)
    }
}

struct Backend<'call> {
    // The enclosing callback owns the strong reference. Keep a direct borrow
    // so API calls do not load through a reference to its Arc/Rc wrapper.
    owner: &'call Authority,
    // Invariant call lifetime, and !Send + !Sync on both targets.
    scope: PhantomData<(&'call mut &'call (), *mut ())>,
}

impl Backend<'_> {
    fn reborrow(&self) -> Backend<'_> {
        Backend {
            owner: self.owner,
            scope: PhantomData,
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    fn raw(&self) -> *const spring_native_sys::NativeInterface {
        self.owner.raw
    }

    #[inline]
    fn check(&self) -> Result<()> {
        // Every Backend borrows a live entry guard, or is constructed inside
        // that guard by a retained callback/ModuleRef. It is !Send. The gate
        // cannot hand off to another thread while this token exists.
        if !self.owner.alive.get() {
            return Err(ApiError::new(ErrorCode::InvalidState as i32));
        }
        #[cfg(not(target_arch = "wasm32"))]
        if self
            .owner
            .callback_failed
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            return self.callback_failure();
        }
        #[cfg(target_arch = "wasm32")]
        if let Some(error) = self.owner.error.take() {
            return Err(error);
        }
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[cold]
    #[inline(never)]
    fn callback_failure(&self) -> Result<()> {
        self.owner
            .callback_failed
            .store(false, std::sync::atomic::Ordering::Relaxed);
        let panic = self.owner.panic.borrow_mut().take();
        if let Some(panic) = panic {
            std::panic::resume_unwind(panic);
        }
        Err(self
            .owner
            .error
            .take()
            .unwrap_or_else(|| ApiError::new(ErrorCode::InvalidState as i32)))
    }

    #[inline]
    fn finish<R>(&self, result: Result<R>) -> Result<R> {
        self.check()?;
        result
    }

    fn with_nested<E: Environment, R>(
        &self,
        callback: impl for<'call> FnOnce(&mut Spring<'call, E>) -> R,
    ) -> R {
        let mut spring = Spring {
            engine: Backend {
                owner: self.owner,
                scope: PhantomData,
            },
            environment: PhantomData,
        };
        let _scope = ActiveScope::enter(self.owner);
        callback(&mut spring)
    }
}

pub mod types {
    #[cfg(target_arch = "wasm32")]
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rts/wasm/generated/sdk/scoped_types_wasm.rs"
    ));
    #[cfg(not(target_arch = "wasm32"))]
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rts/wasm/generated/sdk/scoped_types.rs"
    ));
}

/// Command arguments borrowed for one callback. Use `to_owned` to retain them.
/// ```compile_fail
/// fn escape(command: &spring_sdk::CallinCommand<'_>) -> &'static [f32] {
///     command.params
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CallinCommand<'call> {
    pub id: i32,
    pub time_out: i32,
    pub page_index: u32,
    pub num_params: u32,
    pub tag: u32,
    pub options: u8,
    pub params: &'call [f32],
}
impl CallinCommand<'_> {
    pub fn to_owned(&self) -> types::callins::NativeCallinCommand {
        types::callins::NativeCallinCommand {
            id: self.id,
            time_out: self.time_out,
            page_index: self.page_index,
            num_params: self.num_params,
            tag: self.tag,
            options: self.options,
            params: self.params.to_vec(),
        }
    }
}
impl<'a> From<&'a types::callins::NativeCallinCommand> for CallinCommand<'a> {
    fn from(value: &'a types::callins::NativeCallinCommand) -> Self {
        Self {
            id: value.id,
            time_out: value.time_out,
            page_index: value.page_index,
            num_params: value.num_params,
            tag: value.tag,
            options: value.options,
            params: &value.params,
        }
    }
}
impl<'a> From<&CallinCommand<'a>> for CallinCommand<'a> {
    fn from(value: &CallinCommand<'a>) -> Self {
        *value
    }
}

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../rts/wasm/generated/sdk/scoped_api.rs"
));

pub use spring_sdk_internal::{cmd, consts, gl, los_state, mouse};
#[path = "../../spring-sdk-internal/src/gfx_params.rs"]
mod gfx_params;
pub use gfx_params::*;
pub mod log_level {
    pub const DEBUG: i32 = 20;
    pub const INFO: i32 = 30;
    pub const NOTICE: i32 = 35;
    pub const WARNING: i32 = 40;
    pub const ERROR: i32 = 50;
    pub const FATAL: i32 = 60;
}
#[doc(hidden)]
pub use alloc::format as __format;
mod helpers;
mod logging;
pub use helpers::Hit;
pub use logging::ResultExt;
pub mod callbacks;
pub mod commands;
pub mod cus;
pub mod default_interface;
mod exports;
pub mod columns;
pub mod jobs;
pub mod module;
#[cfg(not(target_arch = "wasm32"))]
mod native;
pub mod rml;
#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use spring_sdk_internal::export_callin_scratch;
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
pub use spring_sdk_internal::generated::__core_callin_scratch_wire as __callin_wire;

/// Export the Wasm callback dispatcher once per module. Native callbacks use
/// C trampolines and need no exported dispatcher.
#[macro_export]
macro_rules! export_callbacks {
    () => {
        #[cfg(target_arch = "wasm32")]
        #[unsafe(export_name = "spring:callback/dispatch")]
        pub extern "C" fn __spring_sdk_callback_dispatch(id: i32, data: i32) {
            $crate::callbacks::__dispatch(id as u32, data as u32);
        }
    };
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../rts/wasm/generated/sdk/scoped_callins.rs"
));
#[cfg(not(target_arch = "wasm32"))]
#[doc(hidden)]
pub use native::callins as __native_callins;
