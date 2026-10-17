//! Engine calls that run once the current callin has returned.
//!
//! Many engine calls run callins of this module before they return:
//! [`create_unit`](crate::create_unit) runs `UnitCreated`, damage runs
//! `UnitPreDamaged`, an order runs `AllowCommand`. Code that holds some of its
//! state across such a call then meets itself again. A command is the same
//! call, queued: it runs after the outermost callin of this module returns,
//! in the same frame, and the callins it causes find nothing borrowed.
//!
//! ```ignore
//! use spring::commands;
//!
//! // Runs once this callin returns; UnitCreated runs inside it.
//! commands::create_unit(def, position, team, CreateUnitOptions::default())
//!     // Runs right after the unit was created, after its callins.
//!     .then(move |unit| {
//!         if let Ok(unit) = unit {
//!             let _ = spring::set_unit_neutral(unit, true);
//!         }
//!     });
//!
//! // Any engine call, or any other work, in the same queue.
//! commands::queue(move || spring::spawn_ceg("smoke", position, up, 0.0, 0.0, 0.0));
//! commands::run(move || crate::with_global(|global| global.reset()));
//! ```
//!
//! Order:
//! - Commands run first in, first out.
//! - A `.then` runs right after its command, before the next command.
//! - Commands queued while commands run (from a `.then`, or from the callins a
//!   command causes) join the end of the queue and run in the same pass.
//!
//! The queue is applied when the outermost callin of the module returns:
//! every spring-addons callin, every unit-script export, and
//! `with_global`/`with_addon_context`. [`flush`] applies it by hand, for
//! exports of your own.
//!
//! Commands are separate from spring-addons' `ctx.delay`. A callin's delayed
//! actions run before the commands it queued. Use [`run`] for work that must
//! come after queued commands.

use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::rc::Rc;
use core::cell::{Cell, RefCell};

type Command = Box<dyn FnOnce()>;
type Continuation<T> = Rc<RefCell<Option<Box<dyn FnOnce(T)>>>>;

/// Stops a queue that keeps refilling itself (a `.then` that always queues
/// another command). Far above anything a frame does.
const MAX_COMMANDS_PER_FLUSH: u32 = 1_000_000;

struct Queue {
    commands: RefCell<VecDeque<Command>>,
    depth: Cell<u32>,
    flushing: Cell<bool>,
}

impl Queue {
    const fn new() -> Self {
        Self {
            commands: RefCell::new(VecDeque::new()),
            depth: Cell::new(0),
            flushing: Cell::new(false),
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod storage {
    use super::Queue;

    struct Shared(Queue);

    // SAFETY: a wasm guest is single-threaded.
    unsafe impl Sync for Shared {}

    static QUEUE: Shared = Shared(Queue::new());

    pub(super) fn with<R>(f: impl FnOnce(&Queue) -> R) -> R {
        f(&QUEUE.0)
    }
}

// Host builds (spring-addons' unit tests) run tests on several threads, so
// each thread gets its own queue.
#[cfg(not(target_arch = "wasm32"))]
mod storage {
    extern crate std;

    use super::Queue;

    std::thread_local! {
        static QUEUE: Queue = const { Queue::new() };
    }

    pub(super) fn with<R>(f: impl FnOnce(&Queue) -> R) -> R {
        QUEUE.with(f)
    }
}

/// The result of a queued command, for a `.then`.
#[must_use = "a command's result is only available through `.then`; drop it if unused"]
pub struct Pending<T> {
    then: Continuation<T>,
}

impl<T: 'static> Pending<T> {
    /// Run `f` with the command's result, right after the command ran (and
    /// after the callins it caused), before the next command.
    pub fn then(self, f: impl FnOnce(T) + 'static) {
        *self.then.borrow_mut() = Some(Box::new(f));
    }
}

/// Queue `f`, usually one engine call. Its result goes to [`Pending::then`].
pub fn queue<T: 'static>(f: impl FnOnce() -> T + 'static) -> Pending<T> {
    let then: Continuation<T> = Rc::new(RefCell::new(None));
    let slot = Rc::clone(&then);
    push(Box::new(move || {
        let result = f();
        let continuation = slot.borrow_mut().take();
        if let Some(continuation) = continuation {
            continuation(result);
        }
    }));
    Pending { then }
}

/// Queue work that doesn't return anything, after the commands already
/// queued.
pub fn run(f: impl FnOnce() + 'static) {
    push(Box::new(f));
}

fn push(command: Command) {
    storage::with(|queue| queue.commands.borrow_mut().push_back(command));
}

/// Whether commands are waiting.
pub fn is_empty() -> bool {
    storage::with(|queue| queue.commands.borrow().is_empty())
}

/// Whether this runs while commands are being applied.
pub fn is_flushing() -> bool {
    storage::with(|queue| queue.flushing.get())
}

/// Apply every queued command now, including those they queue. Does nothing
/// when called from inside a command.
pub fn flush() {
    if storage::with(|queue| queue.flushing.replace(true)) {
        return;
    }
    // Keep `flushing` set until the loop is done, even if a command panics
    // on the host.
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            storage::with(|queue| queue.flushing.set(false));
        }
    }
    let _reset = Reset;

    let mut applied = 0u32;
    loop {
        // Don't hold the queue borrow while a command runs: it may queue more.
        let command = storage::with(|queue| queue.commands.borrow_mut().pop_front());
        let Some(command) = command else {
            break;
        };
        applied += 1;
        if applied > MAX_COMMANDS_PER_FLUSH {
            let dropped = storage::with(|queue| {
                let mut commands = queue.commands.borrow_mut();
                let dropped = commands.len() + 1;
                commands.clear();
                dropped
            });
            report_runaway(dropped);
            break;
        }
        command();
    }
}

#[cfg(target_arch = "wasm32")]
fn report_runaway(dropped: usize) {
    crate::log_error!(
        "commands",
        "more than {} commands in one pass; dropped the remaining {}",
        MAX_COMMANDS_PER_FLUSH,
        dropped
    );
}

#[cfg(not(target_arch = "wasm32"))]
fn report_runaway(dropped: usize) {
    panic!("more than {MAX_COMMANDS_PER_FLUSH} commands in one pass ({dropped} left)");
}

/// Marks a callin of this module as running. When the outermost one ends
/// (the guard is dropped), the queued commands are applied.
///
/// spring-addons and the unit-script exports already do this; only exports
/// written by hand need it.
pub struct Scope(());

impl Scope {
    pub fn enter() -> Self {
        storage::with(|queue| queue.depth.set(queue.depth.get() + 1));
        Scope(())
    }
}

impl Drop for Scope {
    fn drop(&mut self) {
        let outermost = storage::with(|queue| {
            let depth = queue.depth.get().saturating_sub(1);
            queue.depth.set(depth);
            depth == 0
        });
        if outermost {
            flush();
        }
    }
}

/// Run `f` as a callin: see [`Scope`].
pub fn scope<R>(f: impl FnOnce() -> R) -> R {
    let _scope = Scope::enter();
    f()
}

// Queued versions of the engine calls that most often call back into the
// module. Anything else goes through `queue(move || spring::...)`.
#[cfg(target_arch = "wasm32")]
mod calls {
    use super::{Pending, queue};
    use crate::{CreateUnitOptions, Float3, Result, UnitDefRef, UnitId};
    use alloc::string::String;
    use alloc::vec::Vec;

    /// Queued [`crate::create_unit`]; `.then` gets the new unit id.
    pub fn create_unit(
        unit_def: UnitDefRef<'_>,
        position: Float3,
        team_id: i32,
        options: CreateUnitOptions,
    ) -> Pending<Result<i32>> {
        let name = String::from(unit_def.name);
        let id = unit_def.id;
        queue(move || {
            crate::create_unit(UnitDefRef { name: &name, id }, position, team_id, options)
        })
    }

    /// Queued [`crate::destroy_unit`].
    pub fn destroy_unit(
        unit_id: i32,
        options: crate::owned::unit_control::DestroyUnitOptions,
    ) -> Pending<Result<bool>> {
        queue(move || crate::destroy_unit(unit_id, options))
    }

    /// Queued [`crate::transfer_unit`].
    pub fn transfer_unit(
        unit_id: i32,
        new_team_id: i32,
        given: bool,
        adjust_unit_limit: bool,
    ) -> Pending<Result<bool>> {
        queue(move || crate::transfer_unit(unit_id, new_team_id, given, adjust_unit_limit))
    }

    /// Queued [`crate::add_unit_damage`].
    pub fn add_unit_damage(
        unit_id: i32,
        damage: f32,
        paralyze_time: f32,
        weapon_def_id: i32,
        attacker_id: i32,
        impulse: Float3,
    ) -> Pending<Result<bool>> {
        queue(move || {
            crate::add_unit_damage(
                unit_id,
                damage,
                paralyze_time,
                weapon_def_id,
                attacker_id,
                impulse,
            )
        })
    }

    /// Queued [`crate::set_unit_health`].
    pub fn set_unit_health(
        unit_id: i32,
        value: crate::owned::unit_control::UnitHealthValue,
    ) -> Pending<Result<bool>> {
        queue(move || crate::set_unit_health(unit_id, value))
    }

    /// Queued [`crate::set_unit_position`].
    pub fn set_unit_position(unit_id: i32, position: Float3) -> Pending<Result<bool>> {
        queue(move || crate::set_unit_position(unit_id, position))
    }

    /// Queued [`crate::give_order_to_unit`].
    pub fn give_order_to_unit(
        unit_id: impl Into<UnitId>,
        cmd_id: i32,
        params: &[f32],
        options: u32,
        timeout: i32,
    ) -> Pending<Result<bool>> {
        let unit_id = unit_id.into();
        let params: Vec<f32> = params.to_vec();
        queue(move || crate::give_order_to_unit(unit_id, cmd_id, &params, options, timeout))
    }

    /// Queued [`crate::give_order_to_unit_array`].
    pub fn give_order_to_unit_array(
        unit_ids: &[i32],
        cmd_id: i32,
        params: &[f32],
        options: u32,
        timeout: i32,
    ) -> Pending<Result<bool>> {
        let unit_ids: Vec<i32> = unit_ids.to_vec();
        let params: Vec<f32> = params.to_vec();
        queue(move || crate::give_order_to_unit_array(&unit_ids, cmd_id, &params, options, timeout))
    }

    /// Queued [`crate::unit_attach`].
    pub fn unit_attach(
        transporter_id: i32,
        transportee_id: i32,
        piece: i32,
    ) -> Pending<Result<bool>> {
        queue(move || crate::unit_attach(transporter_id, transportee_id, piece))
    }

    /// Queued [`crate::unit_detach`].
    pub fn unit_detach(transportee_id: i32) -> Pending<Result<bool>> {
        queue(move || crate::unit_detach(transportee_id))
    }

    /// Queued `send_to_unsynced`: its receivers run inside the call.
    pub fn send_to_unsynced(message: &str) -> Pending<Result<bool>> {
        let message = String::from(message);
        queue(move || crate::messages::send_to_unsynced(&message))
    }
}

#[cfg(target_arch = "wasm32")]
pub use calls::*;

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use alloc::vec;
    use alloc::vec::Vec;
    use std::rc::Rc as StdRc;

    fn log() -> StdRc<RefCell<Vec<&'static str>>> {
        StdRc::new(RefCell::new(Vec::new()))
    }

    #[test]
    fn commands_wait_for_the_outermost_scope() {
        let seen = log();
        {
            let _outer = Scope::enter();
            let s = seen.clone();
            run(move || s.borrow_mut().push("a"));
            {
                let _inner = Scope::enter();
                let s = seen.clone();
                run(move || s.borrow_mut().push("b"));
            }
            assert!(seen.borrow().is_empty(), "applied inside the outer callin");
        }
        assert_eq!(*seen.borrow(), vec!["a", "b"]);
    }

    #[test]
    fn then_runs_right_after_its_command_and_new_commands_go_last() {
        let seen = log();
        scope(|| {
            let s = seen.clone();
            queue(|| 7).then(move |value| {
                assert_eq!(value, 7);
                s.borrow_mut().push("then");
                let s = s.clone();
                run(move || s.borrow_mut().push("queued by then"));
            });
            let s = seen.clone();
            run(move || s.borrow_mut().push("second"));
        });
        assert_eq!(*seen.borrow(), vec!["then", "second", "queued by then"]);
    }

    #[test]
    fn a_callin_inside_a_command_does_not_flush_early() {
        let seen = log();
        scope(|| {
            let s = seen.clone();
            run(move || {
                // A callin raised by this command: its commands join the queue.
                scope(|| {
                    let s2 = s.clone();
                    run(move || s2.borrow_mut().push("from nested callin"));
                });
                s.borrow_mut().push("first");
            });
            let s = seen.clone();
            run(move || s.borrow_mut().push("second"));
        });
        assert_eq!(
            *seen.borrow(),
            vec!["first", "second", "from nested callin"]
        );
    }
}
