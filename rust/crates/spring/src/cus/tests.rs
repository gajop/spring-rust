/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use super::*;
use crate::{UnitId, WeaponDefId};
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;
use core::cell::{Cell, RefCell};
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

#[derive(Default)]
struct TestEngine {
    events: Vec<i32>,
    unit_events: Vec<UnitId>,
    active: bool,
    animation_queries: Cell<usize>,
}

impl UnitEngine for TestEngine {
    fn turn(&mut self, unit: UnitId, piece: Piece, _: Axis, _: Angle, _: AngularSpeed) {
        self.events.push(piece.0);
        self.unit_events.push(unit);
    }

    fn animation_active(&self, _: UnitId, _: AnimationKind, _: Piece, _: Option<Axis>) -> bool {
        self.animation_queries
            .set(self.animation_queries.get().saturating_add(1));
        self.active
    }
}

fn child(ctx: UnitCtx) -> TaskFuture {
    Box::pin(async move {
        ctx.turn(Piece(1), Axis::Y, Angle::ZERO, AngularSpeed(1.0));
        ctx.next_frame().await;
    })
}

fn parent(ctx: UnitCtx) -> TaskFuture {
    Box::pin(async move {
        ctx.spawn(TaskDefinition::new("child", child));
        ctx.turn(Piece(2), Axis::Y, Angle::ZERO, AngularSpeed(1.0));
    })
}

fn sleeping(ctx: UnitCtx) -> TaskFuture {
    Box::pin(async move { ctx.next_frame().await })
}

struct SelfWaking {
    polls: Rc<RefCell<usize>>,
}

impl Future for SelfWaking {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut polls = self.polls.borrow_mut();
        *polls += 1;
        if *polls < 3 {
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

fn self_waking(polls: Rc<RefCell<usize>>, _: UnitCtx) -> SelfWaking {
    SelfWaking { polls }
}

fn turn_after_frame(ctx: UnitCtx) -> TaskFuture {
    Box::pin(async move {
        ctx.next_frame().await;
        ctx.turn(
            Piece(6),
            Axis::X,
            Angle::ZERO,
            AngularSpeed::radians_per_second(1.0),
        );
    })
}

fn animating(ctx: UnitCtx) -> TaskFuture {
    Box::pin(async move {
        ctx.turn(
            Piece(3),
            Axis::X,
            Angle::ZERO,
            AngularSpeed::radians_per_second(1.0),
        );
        ctx.wait_for_turn(Piece(3), Axis::X).await;
        ctx.turn(
            Piece(4),
            Axis::X,
            Angle::ZERO,
            AngularSpeed::radians_per_second(1.0),
        );
    })
}

fn stateful(state: Rc<RefCell<i32>>, ctx: UnitCtx) -> TaskFuture {
    Box::pin(async move {
        let piece = Piece(*state.borrow());
        ctx.turn(
            piece,
            Axis::X,
            Angle::ZERO,
            AngularSpeed::radians_per_second(1.0),
        );
    })
}

struct TestScript;

impl UnitScript for TestScript {
    fn new(_: &mut InitCtx<'_>) -> Self {
        Self
    }

    fn query_weapon(&mut self, _: &UnitCtx, _: WeaponId) -> Piece {
        Piece(9)
    }

    fn killed(&mut self, _: &UnitCtx, _: f32, _: f32) -> WreckLevel {
        WreckLevel::Two
    }
}

#[test]
fn call_ordinals_and_engine_wreck_values_are_stable() {
    for ordinal in 0..=42 {
        assert_eq!(
            UnitScriptCall::from_u32(ordinal).map(|call| call as u32),
            Some(ordinal)
        );
    }
    assert_eq!(UnitScriptCall::from_u32(43), None);
    assert_eq!(WreckLevel::None as i32, -1);
    assert_eq!(WreckLevel::One as i32, 1);
    assert_eq!(WreckLevel::Two as i32, 2);
    assert_eq!(WreckLevel::Three as i32, 3);
}

#[test]
fn instance_dispatch_returns_typed_call_results() {
    let engine = Rc::new(RefCell::new(TestEngine::default()));
    let instance = CusInstance::attach(UnitId(7), TestScript, Rc::clone(&engine));
    let mut result = UnitScriptCallResult::default();
    assert!(instance.invoke(UnitScriptCall::Killed, &[4.0, 10.0], &[], &mut result));
    assert_eq!(result.int_value, WreckLevel::Two as i32);
    assert!(result.complete);
    assert!(instance.invoke(UnitScriptCall::QueryWeapon, &[], &[3], &mut result));
    assert_eq!(result.int_value, 9);
    assert!(!result.complete);
}

#[test]
fn lus_sleep_conversion_is_literal_and_has_a_minimum() {
    assert_eq!(Duration::from_millis(0).to_frames(), 1);
    assert_eq!(Duration::from_millis(32).to_frames(), 1);
    assert_eq!(Duration::from_millis(33).to_frames(), 1);
    assert_eq!(Duration::from_millis(99).to_frames(), 3);
    assert_eq!(Duration::from_millis(100).to_frames(), 3);
}

#[test]
fn spawn_polls_child_before_parent_continues() {
    let engine = Rc::new(RefCell::new(TestEngine::default()));
    let context = UnitCtx::new(UnitId(7), Rc::clone(&engine));
    let mut scheduler = CusScheduler::new(context);
    scheduler.spawn(TaskDefinition::new("parent", parent));
    assert_eq!(engine.borrow().events, vec![1, 2]);
}

#[test]
fn scheduler_does_not_drain_a_frame_twice() {
    let engine = Rc::new(RefCell::new(TestEngine::default()));
    let polls = Rc::new(RefCell::new(0));
    let context = UnitCtx::new(UnitId(7), Rc::clone(&engine));
    let mut scheduler = CusScheduler::new(context);
    scheduler.spawn(TaskDefinition::with_state(
        "self-waking",
        Rc::clone(&polls),
        self_waking,
    ));

    scheduler.tick(1);
    assert_eq!(*polls.borrow(), 2);
    scheduler.tick(1);
    assert_eq!(*polls.borrow(), 2);
    scheduler.tick(2);
    assert_eq!(*polls.borrow(), 3);
}

#[test]
fn animation_wait_yields_until_the_engine_reports_completion() {
    let engine = Rc::new(RefCell::new(TestEngine {
        active: true,
        ..TestEngine::default()
    }));
    let context = UnitCtx::new(UnitId(7), Rc::clone(&engine));
    let mut scheduler = CusScheduler::new(context);
    let task = scheduler.spawn(TaskDefinition::new("animating", animating));
    assert_eq!(engine.borrow().events, vec![3]);
    assert_eq!(engine.borrow().animation_queries.get(), 1);
    assert_eq!(scheduler.task_state(task), Some(TaskState::Suspended));

    engine.borrow_mut().active = false;
    scheduler.animation_finished(AnimationKind::Turn, Piece(3), Some(Axis::X));
    scheduler.tick(1);
    assert_eq!(engine.borrow().events, vec![3, 4]);
    assert_eq!(engine.borrow().animation_queries.get(), 1);
    assert_eq!(scheduler.task_count(), 0);
}

#[test]
fn named_tasks_can_access_the_attached_script_state() {
    let engine = Rc::new(RefCell::new(TestEngine::default()));
    let mut instance = CusInstance::attach(UnitId(7), 5i32, Rc::clone(&engine));
    instance.spawn_with_state("stateful", stateful);
    assert_eq!(engine.borrow().events, vec![5]);
}

#[test]
fn standard_queries_have_typed_neutral_defaults() {
    let engine = Rc::new(RefCell::new(TestEngine::default()));
    let instance = CusInstance::attach(UnitId(7), TestScript, Rc::clone(&engine));
    assert_eq!(instance.query_weapon(WeaponId(0)), Piece(9));
    assert_eq!(instance.query_transport(UnitId(8)), Piece::INVALID);
    assert_eq!(instance.query_nano_piece(), Piece::INVALID);
    assert_eq!(instance.query_build_info(), Piece::INVALID);
    assert!(!instance.block_shot(WeaponId(0), UnitId(8), false));
    assert_eq!(instance.target_weight(WeaponId(0), UnitId(8)), 1.0);
    assert_eq!(
        instance.world_hit_by_weapon(crate::Float3::ZERO, WeaponDefId(0), 12.0,),
        12.0
    );
}

#[test]
fn signal_uses_intersection_and_inherited_masks() {
    let engine = Rc::new(RefCell::new(TestEngine::default()));
    let context = UnitCtx::new(UnitId(7), Rc::clone(&engine));
    let mut scheduler = CusScheduler::new(context.clone());
    let parent = scheduler.spawn(TaskDefinition::new("parent", |ctx| {
        Box::pin(async move {
            ctx.set_signal_mask(SignalMask::new(0b0110));
            let child = ctx.spawn(TaskDefinition::new("child", sleeping));
            assert_eq!(ctx.signal_mask(), SignalMask::new(0b0110));
            ctx.signal(SignalMask::new(0b0010));
            assert_eq!(ctx.signal_mask(), SignalMask::new(0b0110));
            assert_eq!(child.0.0, 2);
        })
    }));
    assert!(scheduler.task_state(parent).is_some());
    scheduler.tick(1);
    assert_eq!(scheduler.task_count(), 0);
}

#[test]
fn registry_rejects_stale_generation() {
    let engine = Rc::new(RefCell::new(TestEngine::default()));
    let instance = CusInstance::attach(UnitId(4), 1u32, engine);
    let mut registry = CusRegistry::default();
    let old = registry.attach(instance);
    let _ = registry.detach(old).expect("attached");
    let second = CusInstance::attach(
        UnitId(4),
        2u32,
        Rc::new(RefCell::new(TestEngine::default())),
    );
    let fresh = registry.attach(second);
    assert!(registry.with(old, |_| ()).is_none());
    assert_eq!(
        registry.with(fresh, |value| value.with_state(|state| *state)),
        Some(2)
    );
}

#[test]
fn registry_drains_instances_in_unit_id_order() {
    let engine = Rc::new(RefCell::new(TestEngine::default()));
    let mut registry = CusRegistry::default();
    let later = registry.attach(CusInstance::attach(
        UnitId(8),
        TestScript,
        Rc::clone(&engine),
    ));
    let earlier = registry.attach(CusInstance::attach(
        UnitId(2),
        TestScript,
        Rc::clone(&engine),
    ));

    registry.with(later, |instance| {
        instance.spawn(TaskDefinition::new("turn_after_frame", turn_after_frame));
    });
    registry.with(earlier, |instance| {
        instance.spawn(TaskDefinition::new("turn_after_frame", turn_after_frame));
    });
    registry.tick(1);

    assert_eq!(engine.borrow().unit_events, vec![UnitId(2), UnitId(8)]);
}
