/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use super::engine::*;
use super::types::*;
use crate::UnitId;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::rc::{Rc, Weak};
use alloc::vec::Vec;
use core::cell::{Cell, RefCell};
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

pub type TaskFuture = Pin<Box<dyn Future<Output = ()> + 'static>>;

/// A known CUS task entry point.
///
/// The scheduler accepts this named definition rather than an arbitrary
/// future.  That keeps task identity and signal/debug metadata under CUS
/// control and leaves room for durable generated task storage later.
pub struct TaskDefinition {
    name: &'static str,
    start: Box<dyn FnOnce(UnitCtx) -> TaskFuture>,
}

impl TaskDefinition {
    #[inline]
    pub fn new<F>(name: &'static str, start: fn(UnitCtx) -> F) -> Self
    where
        F: Future<Output = ()> + 'static,
    {
        Self {
            name,
            start: Box::new(move |context| Box::pin(start(context))),
        }
    }

    /// Build a named task which receives the script's shared state.  The
    /// entry point remains a known function; only its per-instance state is
    /// captured by the task definition.
    pub fn with_state<S: 'static, F>(
        name: &'static str,
        state: Rc<RefCell<S>>,
        start: fn(Rc<RefCell<S>>, UnitCtx) -> F,
    ) -> Self
    where
        F: Future<Output = ()> + 'static,
    {
        Self {
            name,
            start: Box::new(move |context| Box::pin(start(Rc::clone(&state), context))),
        }
    }

    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    fn start(self, context: UnitCtx) -> TaskFuture {
        (self.start)(context)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct TaskId(pub(crate) u64);

/// Stable handle for a task in one CUS instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TaskHandle(pub(crate) TaskId);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct AnimationKey {
    kind: AnimationKind,
    piece: Piece,
    axis: Option<Axis>,
}

pub(crate) fn animation_key_from_call(kind: i32, piece: i32, axis: i32) -> Option<AnimationKey> {
    let kind = match kind {
        0 => AnimationKind::Turn,
        1 => AnimationKind::Spin,
        2 => AnimationKind::Move,
        3 => AnimationKind::Scale,
        _ => return None,
    };
    let axis = match axis {
        -1 => None,
        0 => Some(Axis::X),
        1 => Some(Axis::Y),
        2 => Some(Axis::Z),
        _ => return None,
    };
    Some(AnimationKey {
        kind,
        piece: Piece(piece),
        axis,
    })
}

struct TaskWaker {
    scheduler: Weak<RefCell<SchedulerState>>,
    id: TaskId,
}

impl TaskWaker {
    fn wake(&self) {
        if let Some(scheduler) = self.scheduler.upgrade() {
            scheduler.borrow_mut().queue_ready(self.id);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TaskStatus {
    Running,
    Suspended,
    Complete,
    Cancelled,
}

struct TaskSlot {
    id: TaskId,
    name: &'static str,
    signal_mask: SignalMask,
    status: TaskStatus,
    future: Option<TaskFuture>,
    last_polled_epoch: u64,
    wake_frame: Option<u64>,
    animation_wait: Option<AnimationKey>,
    animation_ready: bool,
    queued: bool,
    waker: Rc<TaskWaker>,
}

struct SchedulerState {
    frame: Cell<u64>,
    last_drained_frame: Cell<Option<u64>>,
    epoch: u64,
    next_task_id: u64,
    tasks: Vec<TaskSlot>,
    sleepers: BTreeMap<u64, Vec<TaskId>>,
    animation_waiters: BTreeMap<AnimationKey, Vec<TaskId>>,
    ready_tasks: Vec<TaskId>,
    poll_queue: Vec<TaskId>,
    polling_task: Option<TaskId>,
}

impl SchedulerState {
    fn new() -> Self {
        Self {
            frame: Cell::new(0),
            last_drained_frame: Cell::new(None),
            epoch: 0,
            next_task_id: 1,
            tasks: Vec::new(),
            sleepers: BTreeMap::new(),
            animation_waiters: BTreeMap::new(),
            ready_tasks: Vec::new(),
            poll_queue: Vec::new(),
            polling_task: None,
        }
    }

    fn queue_ready(&mut self, id: TaskId) {
        let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) else {
            return;
        };
        if matches!(task.status, TaskStatus::Complete | TaskStatus::Cancelled) {
            return;
        }
        if !task.queued {
            task.queued = true;
            self.ready_tasks.push(id);
        }
    }

    fn register_sleep(&mut self, id: TaskId, deadline: u64) {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.wake_frame = Some(deadline);
            task.animation_wait = None;
            self.sleepers.entry(deadline).or_default().push(id);
        }
    }

    fn register_animation(&mut self, id: TaskId, key: AnimationKey) {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.wake_frame = None;
            task.animation_wait = Some(key);
            self.animation_waiters.entry(key).or_default().push(id);
        }
    }

    fn clear_wait_registration(&mut self, id: TaskId) {
        let (wake_frame, animation_wait) = self
            .tasks
            .iter()
            .find(|task| task.id == id)
            .map(|task| (task.wake_frame, task.animation_wait))
            .unwrap_or((None, None));
        if let Some(frame) = wake_frame {
            remove_waiter(&mut self.sleepers, frame, id);
        }
        if let Some(key) = animation_wait {
            remove_waiter(&mut self.animation_waiters, key, id);
        }
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.wake_frame = None;
            task.animation_wait = None;
        }
    }

    fn wake_animation(&mut self, key: AnimationKey) {
        let Some(waiters) = self.animation_waiters.remove(&key) else {
            return;
        };
        for id in waiters {
            let matches = self
                .tasks
                .iter()
                .find(|task| task.id == id)
                .is_some_and(|task| task.animation_wait == Some(key));
            if !matches {
                continue;
            }
            if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
                task.animation_wait = None;
                task.animation_ready = true;
            }
            self.queue_ready(id);
        }
    }

    fn take_animation_ready(&mut self, id: TaskId) -> bool {
        let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) else {
            return false;
        };
        let ready = task.animation_ready;
        task.animation_ready = false;
        ready
    }
}

pub(crate) fn remove_waiter<K: Ord, V: PartialEq>(
    waiters: &mut BTreeMap<K, Vec<V>>,
    key: K,
    id: V,
) {
    let Some(ids) = waiters.get_mut(&key) else {
        return;
    };
    ids.retain(|candidate| *candidate != id);
    if ids.is_empty() {
        waiters.remove(&key);
    }
}

/// Per-instance unit-script context.
#[derive(Clone)]
pub struct UnitCtx {
    unit: UnitId,
    engine: Rc<RefCell<dyn UnitEngine>>,
    scheduler: Rc<RefCell<SchedulerState>>,
    current_task: Option<TaskId>,
}

impl UnitCtx {
    /// Create a context around an engine backend.  The backend is reference
    /// counted because named task futures own their context while suspended.
    pub fn new<E>(unit: UnitId, engine: Rc<RefCell<E>>) -> Self
    where
        E: UnitEngine + 'static,
    {
        let engine: Rc<RefCell<dyn UnitEngine>> = engine;
        Self {
            unit,
            engine,
            scheduler: Rc::new(RefCell::new(SchedulerState::new())),
            current_task: None,
        }
    }

    #[inline]
    fn for_task(&self, task: TaskId) -> Self {
        Self {
            unit: self.unit,
            engine: Rc::clone(&self.engine),
            scheduler: Rc::clone(&self.scheduler),
            current_task: Some(task),
        }
    }

    #[inline]
    pub const fn unit(&self) -> UnitId {
        self.unit
    }

    #[inline]
    pub fn frame(&self) -> u64 {
        self.scheduler.borrow().frame.get()
    }

    #[inline]
    pub fn turn(&self, piece: Piece, axis: Axis, destination: Angle, speed: AngularSpeed) {
        self.engine
            .borrow_mut()
            .turn(self.unit, piece, axis, destination, speed);
    }

    #[inline]
    pub fn move_piece(&self, piece: Piece, axis: Axis, destination: f32, speed: f32) {
        self.engine
            .borrow_mut()
            .move_piece(self.unit, piece, axis, destination, speed);
    }

    #[inline]
    pub fn spin(&self, piece: Piece, axis: Axis, speed: AngularSpeed, acceleration: AngularSpeed) {
        self.engine
            .borrow_mut()
            .spin(self.unit, piece, axis, speed, acceleration);
    }

    #[inline]
    pub fn stop_spin(&self, piece: Piece, axis: Axis, deceleration: AngularSpeed) {
        self.engine
            .borrow_mut()
            .stop_spin(self.unit, piece, axis, deceleration);
    }

    #[inline]
    pub fn scale(&self, piece: Piece, destination: f32, speed: f32) {
        self.engine
            .borrow_mut()
            .scale(self.unit, piece, destination, speed);
    }

    #[inline]
    pub fn move_now(&self, piece: Piece, axis: Axis, destination: f32) {
        self.engine
            .borrow_mut()
            .move_now(self.unit, piece, axis, destination);
    }

    #[inline]
    pub fn turn_now(&self, piece: Piece, axis: Axis, destination: Angle) {
        self.engine
            .borrow_mut()
            .turn_now(self.unit, piece, axis, destination);
    }

    #[inline]
    pub fn scale_now(&self, piece: Piece, destination: f32) {
        self.engine
            .borrow_mut()
            .scale_now(self.unit, piece, destination);
    }

    #[inline]
    pub fn show(&self, piece: Piece) {
        self.engine.borrow_mut().show(self.unit, piece);
    }

    #[inline]
    pub fn hide(&self, piece: Piece) {
        self.engine.borrow_mut().hide(self.unit, piece);
    }

    #[inline]
    pub fn explode(&self, piece: Piece, flags: SfxFlags) {
        self.engine.borrow_mut().explode(self.unit, piece, flags);
    }

    #[inline]
    pub fn emit_sfx(&self, piece: Piece, sfx: i32) {
        self.engine.borrow_mut().emit_sfx(self.unit, piece, sfx);
    }

    #[inline]
    pub fn attach_unit(&self, piece: Piece, target: UnitId) {
        self.engine
            .borrow_mut()
            .attach_unit(self.unit, piece, target);
    }

    #[inline]
    pub fn drop_unit(&self, target: UnitId) {
        self.engine.borrow_mut().drop_unit(self.unit, target);
    }

    #[inline]
    pub fn set_unit_value(&self, value: i32, parameter: i32) {
        self.engine
            .borrow_mut()
            .set_unit_value(self.unit, value, parameter);
    }

    #[inline]
    pub fn set_aim_ready(&self, weapon: WeaponId, ready: bool) {
        self.engine
            .borrow_mut()
            .aim_script_finished(self.unit, weapon, ready);
    }

    #[inline]
    pub fn set_shield_enabled(&self, weapon: WeaponId, enabled: bool) {
        self.engine
            .borrow_mut()
            .aim_shield_finished(self.unit, weapon, enabled);
    }

    #[inline]
    pub fn set_killed_finished(&self, wreck_level: WreckLevel) {
        self.engine
            .borrow_mut()
            .killed_script_finished(self.unit, wreck_level);
    }

    #[inline]
    pub fn sleep(&self, duration: Duration) -> Sleep {
        Sleep::new(Rc::clone(&self.scheduler), duration.to_frames())
    }

    #[inline]
    pub fn next_frame(&self) -> Sleep {
        Sleep::new(Rc::clone(&self.scheduler), 1)
    }

    #[inline]
    pub fn sleep_frames(&self, frames: u64) -> Sleep {
        Sleep::new(Rc::clone(&self.scheduler), frames.max(1))
    }

    #[inline]
    pub fn wait_for_turn(&self, piece: Piece, axis: Axis) -> AnimationWait {
        self.wait_for(AnimationKind::Turn, piece, Some(axis))
    }

    #[inline]
    pub fn wait_for_move(&self, piece: Piece, axis: Axis) -> AnimationWait {
        self.wait_for(AnimationKind::Move, piece, Some(axis))
    }

    #[inline]
    pub fn wait_for_spin(&self, piece: Piece, axis: Axis) -> AnimationWait {
        self.wait_for(AnimationKind::Spin, piece, Some(axis))
    }

    #[inline]
    pub fn wait_for_scale(&self, piece: Piece) -> AnimationWait {
        self.wait_for(AnimationKind::Scale, piece, None)
    }

    fn wait_for(&self, kind: AnimationKind, piece: Piece, axis: Option<Axis>) -> AnimationWait {
        let active = self
            .engine
            .borrow()
            .animation_active(self.unit, kind, piece, axis);
        AnimationWait {
            key: AnimationKey { kind, piece, axis },
            active,
            scheduler: Rc::clone(&self.scheduler),
        }
    }

    /// Start a known task immediately.  The child is polled until its first
    /// suspension or completion before this method returns.
    pub fn spawn(&self, definition: TaskDefinition) -> TaskHandle {
        let parent_mask = self
            .current_task
            .and_then(|id| task_mask(&self.scheduler, id))
            .unwrap_or_default();
        spawn_task(self, definition, parent_mask)
    }

    /// Cancel suspended/waiting tasks whose masks intersect `signal`.
    /// Running tasks, including the caller, are deliberately left alive.
    pub fn signal(&self, signal: SignalMask) {
        let current = self.current_task;
        let mut scheduler = self.scheduler.borrow_mut();
        for index in 0..scheduler.tasks.len() {
            let (id, status, mask) = {
                let task = &scheduler.tasks[index];
                (task.id, task.status, task.signal_mask)
            };
            if Some(id) == current || status != TaskStatus::Suspended {
                continue;
            }
            if mask.intersects(signal) {
                scheduler.clear_wait_registration(id);
                let task = &mut scheduler.tasks[index];
                task.status = TaskStatus::Cancelled;
                task.future = None;
            }
        }
    }

    #[inline]
    pub fn set_signal_mask(&self, signal_mask: SignalMask) {
        if let Some(current) = self.current_task
            && let Some(task) = self
                .scheduler
                .borrow_mut()
                .tasks
                .iter_mut()
                .find(|task| task.id == current)
        {
            task.signal_mask = signal_mask;
        }
    }

    #[inline]
    pub fn signal_mask(&self) -> SignalMask {
        self.current_task
            .and_then(|id| task_mask(&self.scheduler, id))
            .unwrap_or_default()
    }
}

/// A millisecond/frame timer future.
pub struct Sleep {
    scheduler: Rc<RefCell<SchedulerState>>,
    deadline: u64,
}

impl Sleep {
    fn new(scheduler: Rc<RefCell<SchedulerState>>, frames: u64) -> Self {
        let now = scheduler.borrow().frame.get();
        Self {
            scheduler,
            deadline: now.saturating_add(frames.max(1)),
        }
    }
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut scheduler = self.scheduler.borrow_mut();
        if scheduler.frame.get() >= self.deadline {
            Poll::Ready(())
        } else {
            let Some(task) = scheduler.polling_task else {
                return Poll::Pending;
            };
            scheduler.register_sleep(task, self.deadline);
            Poll::Pending
        }
    }
}

/// A future that completes after an engine animation has settled.
pub struct AnimationWait {
    key: AnimationKey,
    active: bool,
    scheduler: Rc<RefCell<SchedulerState>>,
}

impl Future for AnimationWait {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let mut scheduler = this.scheduler.borrow_mut();
        let animation_ready = if this.active {
            let Some(task) = scheduler.polling_task else {
                return Poll::Pending;
            };
            scheduler.take_animation_ready(task)
        } else {
            false
        };
        if !this.active || animation_ready {
            Poll::Ready(())
        } else {
            let Some(task) = scheduler.polling_task else {
                return Poll::Pending;
            };
            scheduler.register_animation(task, this.key);
            Poll::Pending
        }
    }
}

/// A deterministic scheduler for one CUS script instance.
pub struct CusScheduler {
    context: UnitCtx,
}

impl CusScheduler {
    #[inline]
    pub fn new(context: UnitCtx) -> Self {
        Self { context }
    }

    #[inline]
    pub fn context(&self) -> UnitCtx {
        self.context.clone()
    }

    #[inline]
    pub fn spawn(&mut self, definition: TaskDefinition) -> TaskHandle {
        self.context.spawn(definition)
    }

    pub(crate) fn wake_animation(&self, key: AnimationKey) {
        self.context.scheduler.borrow_mut().wake_animation(key);
    }

    /// Notify the scheduler that an engine animation has completed.
    pub fn animation_finished(&mut self, kind: AnimationKind, piece: Piece, axis: Option<Axis>) {
        self.wake_animation(AnimationKey { kind, piece, axis });
    }

    /// Advance the instance to `frame` and drain only tasks which have a
    /// timer, completion event, or explicit waker ready. Tasks are kept in
    /// stable task insertion order within the ready set.
    pub fn tick(&mut self, frame: u64) {
        let mut scheduler = self.context.scheduler.borrow_mut();
        if frame < scheduler.frame.get() {
            return;
        }
        // Drain each simulation frame at most once.  Wakes that arrive after
        // this pass are intentionally deferred until the next frame so a
        // repeated engine tick cannot poll a task twice in one frame.
        if scheduler.last_drained_frame.get() == Some(frame) {
            return;
        }
        scheduler.frame.set(frame);
        scheduler.last_drained_frame.set(Some(frame));
        if scheduler.ready_tasks.is_empty()
            && !scheduler
                .sleepers
                .first_key_value()
                .is_some_and(|(&deadline, _)| deadline <= frame)
        {
            return;
        }
        scheduler.epoch = scheduler.epoch.saturating_add(1);
        let epoch = scheduler.epoch;
        scheduler.poll_queue.clear();
        while let Some((&deadline, _)) = scheduler.sleepers.first_key_value() {
            if deadline > frame {
                break;
            }
            let Some(ids) = scheduler.sleepers.remove(&deadline) else {
                continue;
            };
            for id in ids {
                let due = scheduler
                    .tasks
                    .iter()
                    .find(|task| task.id == id)
                    .is_some_and(|task| task.wake_frame == Some(deadline));
                if due {
                    scheduler.queue_ready(id);
                }
            }
        }
        while let Some(id) = scheduler.ready_tasks.pop() {
            scheduler.poll_queue.push(id);
        }
        scheduler.poll_queue.sort_unstable_by_key(|id| id.0);
        // Pop from the back after reversing so the ready set is consumed in
        // ascending task-ID order without allocating a per-frame snapshot.
        scheduler.poll_queue.reverse();
        drop(scheduler);

        loop {
            let id = self.context.scheduler.borrow_mut().poll_queue.pop();
            let Some(id) = id else { break };
            poll_task(&self.context, id, epoch);
        }
        let mut scheduler = self.context.scheduler.borrow_mut();
        scheduler
            .tasks
            .retain(|task| !matches!(task.status, TaskStatus::Complete | TaskStatus::Cancelled));
    }

    #[inline]
    pub(crate) fn is_due(&self, frame: u64) -> bool {
        let scheduler = self.context.scheduler.borrow();
        !scheduler.ready_tasks.is_empty()
            || scheduler
                .sleepers
                .first_key_value()
                .is_some_and(|(&deadline, _)| deadline <= frame)
    }

    #[inline]
    pub(crate) fn next_wake_frame(&self) -> Option<u64> {
        let scheduler = self.context.scheduler.borrow();
        if !scheduler.ready_tasks.is_empty() {
            return Some(scheduler.frame.get());
        }
        scheduler
            .sleepers
            .first_key_value()
            .map(|(&frame, _)| frame)
    }

    #[inline]
    pub fn task_state(&self, handle: TaskHandle) -> Option<TaskState> {
        task_state(&self.context.scheduler, handle.0)
    }

    #[inline]
    pub fn task_name(&self, handle: TaskHandle) -> Option<&'static str> {
        self.context
            .scheduler
            .borrow()
            .tasks
            .iter()
            .find(|task| task.id == handle.0)
            .map(|task| task.name)
    }

    #[inline]
    pub fn task_count(&self) -> usize {
        self.context
            .scheduler
            .borrow()
            .tasks
            .iter()
            .filter(|task| !matches!(task.status, TaskStatus::Complete | TaskStatus::Cancelled))
            .count()
    }
}

/// Public task state for diagnostics and deterministic tests.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaskState {
    Running,
    Suspended,
    Complete,
    Cancelled,
}

fn task_mask(scheduler: &Rc<RefCell<SchedulerState>>, id: TaskId) -> Option<SignalMask> {
    scheduler
        .borrow()
        .tasks
        .iter()
        .find(|task| task.id == id)
        .map(|task| task.signal_mask)
}

fn task_state(scheduler: &Rc<RefCell<SchedulerState>>, id: TaskId) -> Option<TaskState> {
    scheduler
        .borrow()
        .tasks
        .iter()
        .find(|task| task.id == id)
        .map(|task| match task.status {
            TaskStatus::Running => TaskState::Running,
            TaskStatus::Suspended => TaskState::Suspended,
            TaskStatus::Complete => TaskState::Complete,
            TaskStatus::Cancelled => TaskState::Cancelled,
        })
}

fn spawn_task(
    context: &UnitCtx,
    definition: TaskDefinition,
    parent_mask: SignalMask,
) -> TaskHandle {
    let (id, epoch, task_context) = {
        let mut scheduler = context.scheduler.borrow_mut();
        let id = TaskId(scheduler.next_task_id);
        scheduler.next_task_id = scheduler.next_task_id.saturating_add(1);
        let epoch = scheduler.epoch;
        (id, epoch, context.for_task(id))
    };
    let name = definition.name();
    let future = definition.start(task_context);
    {
        let mut scheduler = context.scheduler.borrow_mut();
        scheduler.tasks.push(TaskSlot {
            id,
            name,
            signal_mask: parent_mask,
            status: TaskStatus::Suspended,
            future: Some(future),
            last_polled_epoch: u64::MAX,
            wake_frame: None,
            animation_wait: None,
            animation_ready: false,
            queued: false,
            waker: Rc::new(TaskWaker {
                scheduler: Rc::downgrade(&context.scheduler),
                id,
            }),
        });
    }

    // A spawned child always gets its first poll synchronously.  Do not hold
    // the RefCell borrow used for insertion while polling guest code.
    poll_task(context, id, epoch);
    TaskHandle(id)
}

fn poll_task(context: &UnitCtx, id: TaskId, epoch: u64) {
    let (mut future, task_waker_state) = {
        let mut scheduler = context.scheduler.borrow_mut();
        let can_poll = scheduler
            .tasks
            .iter()
            .find(|task| task.id == id)
            .is_some_and(|task| {
                task.last_polled_epoch != epoch
                    && task.status != TaskStatus::Cancelled
                    && task.future.is_some()
            });
        if !can_poll {
            return;
        }
        scheduler.clear_wait_registration(id);
        scheduler.polling_task = Some(id);
        let Some(task) = scheduler.tasks.iter_mut().find(|task| task.id == id) else {
            return;
        };
        task.queued = false;
        task.last_polled_epoch = epoch;
        task.status = TaskStatus::Running;
        (
            task.future.take().expect("checked above"),
            Rc::clone(&task.waker),
        )
    };

    let waker = task_waker(&task_waker_state);
    let mut cx = Context::from_waker(&waker);
    let result = future.as_mut().poll(&mut cx);

    let mut scheduler = context.scheduler.borrow_mut();
    scheduler.polling_task = None;
    let Some(task) = scheduler.tasks.iter_mut().find(|task| task.id == id) else {
        return;
    };
    match result {
        Poll::Ready(()) => {
            task.future = None;
            task.status = TaskStatus::Complete;
        }
        Poll::Pending if task.status == TaskStatus::Cancelled => {
            task.future = None;
        }
        Poll::Pending => {
            task.future = Some(future);
            task.status = TaskStatus::Suspended;
        }
    }
}

unsafe fn clone_task_waker(data: *const ()) -> RawWaker {
    // SAFETY: `data` is created by `task_waker` from an owned Rc pointer and
    // remains valid for every clone of the raw waker.
    let state = unsafe { Rc::from_raw(data.cast::<TaskWaker>()) };
    let clone = Rc::clone(&state);
    core::mem::forget(state);
    RawWaker::new(Rc::into_raw(clone).cast(), &TASK_WAKER_VTABLE)
}

unsafe fn wake_task_waker(data: *const ()) {
    // SAFETY: `data` is one owned Rc reference transferred to this callback.
    let state = unsafe { Rc::from_raw(data.cast::<TaskWaker>()) };
    state.wake();
}

unsafe fn wake_task_waker_by_ref(data: *const ()) {
    // SAFETY: temporarily reconstruct and then return the borrowed Rc reference
    // to the raw waker.
    let state = unsafe { Rc::from_raw(data.cast::<TaskWaker>()) };
    state.wake();
    core::mem::forget(state);
}

unsafe fn drop_task_waker(data: *const ()) {
    // SAFETY: `data` is one owned Rc reference transferred to this callback.
    drop(unsafe { Rc::from_raw(data.cast::<TaskWaker>()) });
}

static TASK_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
    clone_task_waker,
    wake_task_waker,
    wake_task_waker_by_ref,
    drop_task_waker,
);

fn task_waker(state: &Rc<TaskWaker>) -> Waker {
    let raw = RawWaker::new(Rc::into_raw(Rc::clone(state)).cast(), &TASK_WAKER_VTABLE);
    // SAFETY: the raw waker owns one Rc reference and its vtable preserves the
    // reference-counting contract for clone, wake, and drop.
    unsafe { Waker::from_raw(raw) }
}
