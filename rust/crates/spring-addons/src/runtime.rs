use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::any::{Any, TypeId, type_name};
use core::cell::{Cell, Ref, RefCell, RefMut};

/// Names of the addon callins currently on the stack, outermost first.
///
/// It exists so a resource borrow conflict can name the outer callin that holds
/// the borrow and the nested callin that wanted it.
mod callin_stack {
    /// Deeper nesting than this is not recorded. Real callin stacks are two or
    /// three deep; the cap only bounds the diagnostic.
    const DEPTH: usize = 16;

    #[derive(Clone, Copy)]
    pub struct Stack {
        names: [&'static str; DEPTH],
        len: usize,
    }

    impl Stack {
        const fn new() -> Self {
            Self {
                names: [""; DEPTH],
                len: 0,
            }
        }

        fn push(&mut self, name: &'static str) {
            if self.len < DEPTH {
                self.names[self.len] = name;
            }
            self.len = self.len.saturating_add(1);
        }

        fn pop(&mut self) {
            self.len = self.len.saturating_sub(1);
        }

        fn active(&self) -> &[&'static str] {
            let len = if self.len > DEPTH { DEPTH } else { self.len };
            &self.names[..len]
        }
    }

    mod storage {
        use super::Stack;
        use core::cell::Cell;

        std::thread_local! {
            static STACK: Cell<Stack> = const { Cell::new(Stack::new()) };
        }

        pub fn update(f: impl FnOnce(&mut Stack)) {
            STACK.with(|cell| {
                let mut stack = cell.get();
                f(&mut stack);
                cell.set(stack);
            });
        }

        pub fn snapshot() -> Stack {
            STACK.with(Cell::get)
        }
    }

    /// Records `name` for as long as the guard lives.
    ///
    /// A guard rather than paired calls because a panic inside a callin must
    /// still unwind the stack: on the host that panic is caught by tests, and
    /// leaving the entry behind would misattribute every later diagnostic.
    pub struct Entry;

    impl Entry {
        pub fn push(name: &'static str) -> Self {
            storage::update(|stack| stack.push(name));
            Self
        }
    }

    impl Drop for Entry {
        fn drop(&mut self) {
            storage::update(Stack::pop);
        }
    }

    pub fn with_active<R>(f: impl FnOnce(&[&'static str]) -> R) -> R {
        // Snapshot first: `f` formats a diagnostic and may itself panic, and
        // the panic hook reads this same stack.
        let stack = storage::snapshot();
        f(stack.active())
    }
}

/// Run `f` with the addon callins currently executing, outermost first.
///
/// During a top-level `GameFrame` this is `["GameFrame"]`. If a Spring callout
/// made from that callin re-enters the guest it becomes, for example,
/// `["GameFrame", "UnitDestroyed"]`.
pub fn with_active_callins<R>(f: impl FnOnce(&[&'static str]) -> R) -> R {
    callin_stack::with_active(f)
}

#[cfg(not(target_arch = "wasm32"))]
type DelayedAction<G, E> = Box<dyn for<'call> FnOnce(&AddonContext<'call, G, E>) + Send>;
#[cfg(target_arch = "wasm32")]
type DelayedAction<G, E> = Box<dyn for<'call> FnOnce(&AddonContext<'call, G, E>)>;

pub struct AddonRuntime<G, E: spring::capabilities::Common = spring::RulesSynced> {
    delayed: RefCell<VecDeque<DelayedAction<G, E>>>,
    depth: Cell<usize>,
    flushing: Cell<bool>,
    commands: spring::commands::Queue<E>,
}

impl<G, E: spring::capabilities::Common> Default for AddonRuntime<G, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<G, E: spring::capabilities::Common> AddonRuntime<G, E> {
    pub const fn new() -> Self {
        Self {
            delayed: RefCell::new(VecDeque::new()),
            depth: Cell::new(0),
            flushing: Cell::new(false),
            commands: spring::commands::Queue::new(),
        }
    }

    pub fn context<'a>(
        &'a self,
        global: &'a G,
        spring: &'a spring::Spring<'_, E>,
    ) -> AddonContext<'a, G, E> {
        AddonContext {
            global,
            runtime: self,
            spring: spring.reborrow(),
        }
    }

    pub fn callin<R>(
        &self,
        name: &'static str,
        global: &G,
        spring: &spring::Spring<'_, E>,
        f: impl FnOnce(&AddonContext<'_, G, E>) -> R,
    ) -> R {
        struct Leave<'a>(&'a Cell<usize>);
        impl Drop for Leave<'_> {
            fn drop(&mut self) {
                self.0.set(self.0.get() - 1);
            }
        }
        let result = {
            let _entry = callin_stack::Entry::push(name);
            let _log = crate::log::enter(spring);
            self.depth.set(self.depth.get() + 1);
            let _leave = Leave(&self.depth);
            let ctx = self.context(global, spring);
            f(&ctx)
        };
        if self.depth.get() == 0 && !self.flushing.get() {
            let _entry = callin_stack::Entry::push("delayed");
            let _log = crate::log::enter(spring);
            self.flush(global, spring);
            self.commands.flush(&mut spring.reborrow());
        }
        result
    }

    fn delay(&self, f: DelayedAction<G, E>) {
        self.delayed.borrow_mut().push_back(f);
    }

    fn flush(&self, global: &G, spring: &spring::Spring<'_, E>) {
        if self.flushing.replace(true) {
            return;
        }
        struct Reset<'a>(&'a Cell<bool>);
        impl Drop for Reset<'_> {
            fn drop(&mut self) {
                self.0.set(false);
            }
        }
        let _reset = Reset(&self.flushing);
        loop {
            let action = self.delayed.borrow_mut().pop_front();
            let Some(action) = action else {
                break;
            };
            action(&self.context(global, spring));
        }
    }
}

/// Game state and scoped engine access for the current addon callin.
pub struct AddonContext<'a, G, E: spring::capabilities::Common = spring::RulesSynced> {
    global: &'a G,
    runtime: &'a AddonRuntime<G, E>,
    spring: spring::Spring<'a, E>,
}

impl<'a, G, E: spring::capabilities::Common> AddonContext<'a, G, E> {
    pub const fn global(&self) -> &'a G {
        self.global
    }

    pub const fn spring(&self) -> &spring::Spring<'a, E> {
        &self.spring
    }

    pub const fn commands(&self) -> &spring::commands::Queue<E> {
        &self.runtime.commands
    }

    /// Run after the outermost addon callin, before queued commands.
    pub fn delay(
        &self,
        f: impl for<'call> FnOnce(&AddonContext<'call, G, E>) + spring::RetainedCapture + 'static,
    ) {
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

#[cfg(not(target_arch = "wasm32"))]
type ResourceValue = dyn Any + Send;
#[cfg(target_arch = "wasm32")]
type ResourceValue = dyn Any;

struct ResourceEntry {
    type_id: TypeId,
    value: RefCell<Box<ResourceValue>>,
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
    pub fn insert<T: spring::RetainedCapture + 'static>(&mut self, value: T) {
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

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use alloc::rc::Rc;

    fn with_spring<R>(call: impl FnOnce(&spring::Spring<'_, spring::RulesSynced>) -> R) -> R {
        let interface = spring::__sys::NativeInterface::default();
        let context =
            unsafe { spring::ModuleContext::<spring::RulesSynced>::from_native(&interface) }
                .unwrap();
        context.with_spring(|spring| call(spring)).unwrap()
    }

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
    fn resource_borrows_are_independent() {
        struct Units(u32);
        struct Economy(u32);

        let units = Resource::new(Units(1));
        let economy = Resource::new(Economy(2));

        let mut units = units.access_mut();
        let economy = economy.access();
        units.0 += economy.0;
        assert_eq!(units.0, 3);
    }

    #[test]
    fn conflicting_resource_borrow_reports_the_active_callins() {
        struct Units;

        let runtime = AddonRuntime::<_, spring::RulesSynced>::new();
        let units = Resource::new(Units);

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            with_spring(|spring| {
                runtime.callin("GameFrame", &(), spring, |_| {
                    let _held = units.access_mut();
                    runtime.callin("UnitCreated", &(), spring, |_| {
                        let _conflict = units.access();
                    });
                })
            });
        }));

        let payload = result.expect_err("a conflicting borrow must panic");
        let message = payload
            .downcast_ref::<alloc::string::String>()
            .expect("panic payload should be the formatted message");
        assert!(message.contains("Units"), "{message}");
        assert!(message.contains("GameFrame"), "{message}");
        assert!(message.contains("UnitCreated"), "{message}");
    }

    #[test]
    fn delayed_work_runs_after_outermost_callin() {
        let runtime = AddonRuntime::<_, spring::RulesSynced>::new();
        let state = Rc::new(Cell::new(0));

        with_spring(|spring| {
            runtime.callin("outer", &state, spring, |outer| {
                outer.delay(|ctx| ctx.global().set(ctx.global().get() + 10));
                runtime.callin("inner", &state, spring, |inner| {
                    inner.delay(|ctx| ctx.global().set(ctx.global().get() + 1));
                    assert_eq!(state.get(), 0);
                });
                assert_eq!(state.get(), 0);
            })
        });

        assert_eq!(state.get(), 11);
    }
}
