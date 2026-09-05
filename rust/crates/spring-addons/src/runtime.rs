use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::any::{type_name, Any, TypeId};
use core::cell::{Cell, Ref, RefCell, RefMut};

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

    pub fn callin<R>(&self, global: &G, f: impl FnOnce(&AddonContext<'_, G>) -> R) -> R {
        self.depth.set(self.depth.get() + 1);
        let ctx = self.context(global);
        let result = f(&ctx);
        let depth = self.depth.get() - 1;
        self.depth.set(depth);
        if depth == 0 {
            self.flush(global);
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
        Self { entries: Vec::new() }
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
            panic!(
                "spring-addons: resource `{}` cannot be read because it is mutably borrowed; \
                 a Spring callout may have synchronously re-entered an addon callin",
                type_name::<T>()
            )
        });
        Ref::map(borrow, |value| {
            value.downcast_ref::<T>().expect("resource type id mismatch")
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
            panic!(
                "spring-addons: resource `{}` cannot be mutably accessed because it is already borrowed; \
                 a Spring callout may have synchronously re-entered an addon callin",
                type_name::<T>()
            )
        });
        RefMut::map(borrow, |value| {
            value.downcast_mut::<T>().expect("resource type id mismatch")
        })
    }

    /// Try to borrow a resource immutably. Returns `None` for a missing resource
    /// or a borrow conflict.
    pub fn try_access<T: 'static>(&self) -> Option<Ref<'_, T>> {
        let entry = self.entry::<T>()?;
        let borrow = entry.value.try_borrow().ok()?;
        Some(Ref::map(borrow, |value| {
            value.downcast_ref::<T>().expect("resource type id mismatch")
        }))
    }

    /// Try to borrow a resource mutably. Returns `None` for a missing resource
    /// or a borrow conflict.
    pub fn try_access_mut<T: 'static>(&self) -> Option<RefMut<'_, T>> {
        let entry = self.entry::<T>()?;
        let borrow = entry.value.try_borrow_mut().ok()?;
        Some(RefMut::map(borrow, |value| {
            value.downcast_mut::<T>().expect("resource type id mismatch")
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

        runtime.callin(&state, |outer| {
            outer.delay(|state| state.set(state.get() + 10));
            runtime.callin(&state, |inner| {
                inner.delay(|state| state.set(state.get() + 1));
                assert_eq!(state.get(), 0);
            });
            assert_eq!(state.get(), 0);
        });

        assert_eq!(state.get(), 11);
    }
}
