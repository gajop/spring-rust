use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::any::{Any, TypeId, type_name};
use core::cell::{Cell, Ref, RefCell, RefMut};

/// Names of the addon callins currently on the stack, outermost first.
///
/// Wasm guests are single-threaded and callins are strictly nested, so a plain
/// static is enough. It exists so a resource borrow conflict can name the outer
/// callin that holds the borrow and the nested callin that wanted it.
mod callin_stack {
    const DEPTH: usize = 16;

    static mut NAMES: [&str; DEPTH] = [""; DEPTH];
    static mut LEN: usize = 0;

    pub fn push(name: &'static str) {
        unsafe {
            if LEN < DEPTH {
                NAMES[LEN] = name;
            }
            LEN += 1;
        }
    }

    pub fn pop() {
        unsafe {
            LEN = LEN.saturating_sub(1);
        }
    }

    pub fn active() -> &'static [&'static str] {
        unsafe {
            let len = if LEN > DEPTH { DEPTH } else { LEN };
            &*core::ptr::addr_of!(NAMES[..len])
        }
    }
}

/// The addon callins currently executing, outermost first.
///
/// During a top-level `GameFrame` this is `["GameFrame"]`. If a Spring callout
/// made from that callin re-enters the guest it becomes, for example,
/// `["GameFrame", "UnitDestroyed"]`.
pub fn active_callins() -> &'static [&'static str] {
    callin_stack::active()
}

/// Per-handler execution state shared by all addon callins.
///
/// A delayed action runs only after the outermost addon callin has returned.
/// Nested callins can enqueue more actions; they are appended to the same FIFO
/// queue and drained by the outermost dispatcher.
pub struct AddonRuntime<G> {
    delayed: RefCell<VecDeque<Box<dyn FnOnce(&G)>>>,
    depth: Cell<usize>,
    flushing: Cell<bool>,
}

impl<G> Default for AddonRuntime<G> {
    fn default() -> Self {
        Self::new()
    }
}

impl<G> AddonRuntime<G> {
    pub const fn new() -> Self {
        Self {
            delayed: RefCell::new(VecDeque::new()),
            depth: Cell::new(0),
            flushing: Cell::new(false),
        }
    }

    pub fn context<'a>(&'a self, global: &'a G) -> AddonContext<'a, G> {
        AddonContext {
            global,
            runtime: self,
        }
    }

    pub fn callin<R>(
        &self,
        name: &'static str,
        global: &G,
        f: impl FnOnce(&AddonContext<'_, G>) -> R,
    ) -> R {
        callin_stack::push(name);
        self.depth.set(self.depth.get() + 1);
        let ctx = self.context(global);
        let result = f(&ctx);
        let depth = self.depth.get() - 1;
        self.depth.set(depth);
        callin_stack::pop();
        if depth == 0 {
            callin_stack::push("delayed");
            self.flush(global);
            callin_stack::pop();
        }
        result
    }

    fn delay(&self, f: Box<dyn FnOnce(&G)>) {
        self.delayed.borrow_mut().push_back(f);
    }

    fn flush(&self, global: &G) {
        if self.flushing.replace(true) {
            return;
        }

        loop {
            // Keep the queue borrow scoped to the pop. A delayed action may
            // issue a callout, re-enter through another callin, and enqueue
            // more work.
            let action = { self.delayed.borrow_mut().pop_front() };
            let Some(action) = action else {
                break;
            };
            action(global);
        }

        self.flushing.set(false);
    }
}

/// Context supplied to addon callins.
///
/// `global()` returns the shared game-global container. Games that need mutable
/// state should put independently borrowable resources inside that container
/// (for example [`Resources`]) rather than borrowing the entire global for the
/// duration of a callin.
pub struct AddonContext<'a, G> {
    global: &'a G,
    runtime: &'a AddonRuntime<G>,
}

impl<'a, G> AddonContext<'a, G> {
    #[inline]
    pub const fn global(&self) -> &'a G {
        self.global
    }

    /// Run work after the outermost currently-active addon callin returns.
    ///
    /// This is deliberately a general closure rather than a typed command
    /// buffer: it can contain Spring callouts, state access, or arbitrary game
    /// logic. Captured data must be owned because the action can outlive the
    /// current callin stack.
    pub fn delay(&self, f: impl FnOnce(&G) + 'static) {
        self.runtime.delay(Box::new(f));
    }
}

/// One independently borrowable piece of a game global.
///
/// The global stays the shared communication namespace; wrapping each
/// subsystem in a `Resource` means a callin borrows only the part it uses, so a
/// synchronous nested callin conflicts only when it genuinely touches the same
/// subsystem. Conflicts fail loudly with the active callin stack rather than
/// silently changing Spring semantics.
pub struct Resource<T> {
    value: RefCell<T>,
}

impl<T: Default> Default for Resource<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> Resource<T> {
    pub const fn new(value: T) -> Self {
        Self {
            value: RefCell::new(value),
        }
    }

    /// Borrow the resource immutably. Panics if it is mutably borrowed.
    pub fn access(&self) -> Ref<'_, T> {
        self.value.try_borrow().unwrap_or_else(|_| {
            crate::panic::borrow_conflict(
                "spring-addons:",
                &alloc::format!(
                    "resource `{}` cannot be read because it is mutably borrowed.",
                    type_name::<T>()
                ),
            )
        })
    }

    /// Borrow the resource mutably. Panics if it is already borrowed.
    ///
    /// Prefer short borrows around state manipulation: a Spring callout issued
    /// while this guard is alive may synchronously re-enter a callin that wants
    /// the same resource.
    pub fn access_mut(&self) -> RefMut<'_, T> {
        self.value.try_borrow_mut().unwrap_or_else(|_| {
            crate::panic::borrow_conflict(
                "spring-addons:",
                &alloc::format!(
                    "resource `{}` cannot be mutably accessed because it is already borrowed.",
                    type_name::<T>()
                ),
            )
        })
    }

    /// Borrow immutably, or return `None` on conflict.
    pub fn try_access(&self) -> Option<Ref<'_, T>> {
        self.value.try_borrow().ok()
    }

    /// Borrow mutably, or return `None` on conflict.
    pub fn try_access_mut(&self) -> Option<RefMut<'_, T>> {
        self.value.try_borrow_mut().ok()
    }

    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        f(&self.access())
    }

    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        f(&mut self.access_mut())
    }
}

struct ResourceEntry {
    type_id: TypeId,
    value: RefCell<Box<dyn Any>>,
}

/// A typed resource container intended for addon globals.
///
/// The container itself is never exclusively borrowed during dispatch. Each
/// resource has its own runtime borrow state, so unrelated resources remain
/// accessible during a re-entrant callin. Borrowing the same resource in an
/// incompatible way fails loudly rather than silently changing Spring
/// semantics.
#[derive(Default)]
pub struct Resources {
    entries: Vec<ResourceEntry>,
}

impl Resources {
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Insert a resource during setup.
    ///
    /// Duplicate resource types are rejected; each Rust type identifies one
    /// resource slot.
    pub fn insert<T: 'static>(&mut self, value: T) {
        let type_id = TypeId::of::<T>();
        assert!(
            !self.entries.iter().any(|entry| entry.type_id == type_id),
            "spring-addons: resource `{}` is already registered",
            type_name::<T>()
        );
        self.entries.push(ResourceEntry {
            type_id,
            value: RefCell::new(Box::new(value)),
        });
    }

    #[inline]
    pub fn contains<T: 'static>(&self) -> bool {
        self.entry::<T>().is_some()
    }

    /// Borrow a resource immutably.
    ///
    /// Panics if the resource is missing or currently mutably borrowed. A
    /// borrow conflict during a re-entrant callin is a game/framework usage
    /// error, not something spring-addons masks with a fallback result.
    pub fn access<T: 'static>(&self) -> Ref<'_, T> {
        let entry = self.required_entry::<T>();
        let borrow = entry.value.try_borrow().unwrap_or_else(|_| {
            crate::panic::borrow_conflict(
                "spring-addons:",
                &alloc::format!(
                    "resource `{}` cannot be read because it is mutably borrowed.",
                    type_name::<T>()
                ),
            )
        });
        Ref::map(borrow, |value| {
            value
                .downcast_ref::<T>()
                .expect("resource type id mismatch")
        })
    }

    /// Borrow a resource mutably.
    ///
    /// Panics if the resource is missing or already borrowed. Prefer short
    /// borrows around state manipulation; a Spring callout made while holding
    /// this guard may synchronously re-enter a callin that wants the same
    /// resource.
    pub fn access_mut<T: 'static>(&self) -> RefMut<'_, T> {
        let entry = self.required_entry::<T>();
        let borrow = entry.value.try_borrow_mut().unwrap_or_else(|_| {
            crate::panic::borrow_conflict(
                "spring-addons:",
                &alloc::format!(
                    "resource `{}` cannot be mutably accessed because it is already borrowed.",
                    type_name::<T>()
                ),
            )
        });
        RefMut::map(borrow, |value| {
            value
                .downcast_mut::<T>()
                .expect("resource type id mismatch")
        })
    }

    /// Try to borrow a resource immutably. Returns `None` for a missing resource
    /// or a borrow conflict.
    pub fn try_access<T: 'static>(&self) -> Option<Ref<'_, T>> {
        let entry = self.entry::<T>()?;
        let borrow = entry.value.try_borrow().ok()?;
        Some(Ref::map(borrow, |value| {
            value
                .downcast_ref::<T>()
                .expect("resource type id mismatch")
        }))
    }

    /// Try to borrow a resource mutably. Returns `None` for a missing resource
    /// or a borrow conflict.
    pub fn try_access_mut<T: 'static>(&self) -> Option<RefMut<'_, T>> {
        let entry = self.entry::<T>()?;
        let borrow = entry.value.try_borrow_mut().ok()?;
        Some(RefMut::map(borrow, |value| {
            value
                .downcast_mut::<T>()
                .expect("resource type id mismatch")
        }))
    }

    pub fn with<T: 'static, R>(&self, f: impl FnOnce(&T) -> R) -> R {
        let value = self.access::<T>();
        f(&value)
    }

    pub fn with_mut<T: 'static, R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut value = self.access_mut::<T>();
        f(&mut value)
    }

    fn entry<T: 'static>(&self) -> Option<&ResourceEntry> {
        let type_id = TypeId::of::<T>();
        self.entries.iter().find(|entry| entry.type_id == type_id)
    }

    fn required_entry<T: 'static>(&self) -> &ResourceEntry {
        self.entry::<T>().unwrap_or_else(|| {
            panic!(
                "spring-addons: resource `{}` is not registered",
                type_name::<T>()
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::rc::Rc;

    #[test]
    fn resources_are_borrowed_independently() {
        let mut resources = Resources::new();
        resources.insert::<u32>(1);
        resources.insert::<u64>(2);

        let mut first = resources.access_mut::<u32>();
        let second = resources.access::<u64>();
        *first += *second as u32;
        assert_eq!(*first, 3);
    }

    #[test]
    fn delayed_work_runs_after_outermost_callin() {
        let runtime = AddonRuntime::new();
        let state = Rc::new(Cell::new(0));

        runtime.callin("outer", &state, |outer| {
            outer.delay(|state| state.set(state.get() + 10));
            runtime.callin("inner", &state, |inner| {
                inner.delay(|state| state.set(state.get() + 1));
                assert_eq!(state.get(), 0);
            });
            assert_eq!(state.get(), 0);
        });

        assert_eq!(state.get(), 11);
    }
}
