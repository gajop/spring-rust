/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use super::engine::*;
use super::scheduler::*;
use super::script::*;
use super::types::*;
use crate::{UnitId, WeaponDefId};
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::future::Future;

/// An attached, typed script instance with its own scheduler and generation-
/// guarded context.
pub struct CusInstance<S> {
    unit: UnitId,
    state: Rc<RefCell<S>>,
    scheduler: CusScheduler,
}

impl<S> CusInstance<S> {
    pub fn attach<E>(unit: UnitId, state: S, engine: Rc<RefCell<E>>) -> Self
    where
        E: UnitEngine + 'static,
    {
        let context = UnitCtx::new(unit, engine);
        Self {
            unit,
            state: Rc::new(RefCell::new(state)),
            scheduler: CusScheduler::new(context),
        }
    }

    /// Construct a script state from the synchronous piece-mapping hook and
    /// attach its scheduler to the supplied engine backend.
    pub fn construct<E>(unit: UnitId, pieces: &dyn PieceResolver, engine: Rc<RefCell<E>>) -> Self
    where
        S: UnitScript,
        E: UnitEngine + 'static,
    {
        let mut init = InitCtx::new(unit, pieces);
        Self::attach(unit, S::new(&mut init), engine)
    }

    #[inline]
    pub const fn unit(&self) -> UnitId {
        self.unit
    }

    #[inline]
    pub fn context(&self) -> UnitCtx {
        self.scheduler.context()
    }

    #[inline]
    pub fn state(&self) -> Rc<RefCell<S>> {
        Rc::clone(&self.state)
    }

    #[inline]
    pub fn with_state<R>(&self, f: impl FnOnce(&mut S) -> R) -> R {
        f(&mut self.state.borrow_mut())
    }

    #[inline]
    pub fn spawn(&mut self, definition: TaskDefinition) -> TaskHandle {
        self.scheduler.spawn(definition)
    }

    #[inline]
    pub fn spawn_with_state<F>(
        &mut self,
        name: &'static str,
        start: fn(Rc<RefCell<S>>, UnitCtx) -> F,
    ) -> TaskHandle
    where
        S: 'static,
        F: Future<Output = ()> + 'static,
    {
        self.spawn(TaskDefinition::with_state(
            name,
            Rc::clone(&self.state),
            start,
        ))
    }

    #[inline]
    pub fn tick(&mut self, frame: u64) {
        self.scheduler.tick(frame);
    }

    #[inline]
    pub(crate) fn is_due(&self, frame: u64) -> bool {
        self.scheduler.is_due(frame)
    }

    #[inline]
    pub(crate) fn next_wake_frame(&self) -> Option<u64> {
        self.scheduler.next_wake_frame()
    }

    #[inline]
    pub fn scheduler(&self) -> &CusScheduler {
        &self.scheduler
    }
}

impl<S: UnitScript> CusInstance<S> {
    /// Dispatch one engine call through the typed script state.  Arguments
    /// use the exact normalized layout produced by the engine adapter; missing
    /// values are treated as an unavailable call rather than panicking.
    pub fn invoke(
        &self,
        call: UnitScriptCall,
        float_arguments: &[f32],
        integer_arguments: &[i32],
        result: &mut UnitScriptCallResult,
    ) -> bool {
        result.int_value = -1;
        result.float_value = 1.0;
        result.bool_value = false;
        result.complete = false;
        result.int_values.clear();
        let f = |index: usize| float_arguments.get(index).copied().unwrap_or_default();
        let i = |index: usize| integer_arguments.get(index).copied().unwrap_or_default();
        let context = self.context();
        self.with_state(|script| {
            match call {
                UnitScriptCall::RawCall => script.raw_call(&context, i(0)),
                UnitScriptCall::Create => script.create(&context),
                UnitScriptCall::Killed => {
                    result.int_value = script.killed(&context, f(0), f(1)) as i32;
                    result.complete = true;
                }
                UnitScriptCall::WindChanged => script.wind_changed(&context, f(0), f(1)),
                UnitScriptCall::ExtractionRateChanged => {
                    script.extraction_rate_changed(&context, f(0))
                }
                UnitScriptCall::WorldRockUnit => {
                    script.world_rock_unit(&context, crate::Float3::new(f(0), f(1), f(2)))
                }
                UnitScriptCall::RockUnit => {
                    script.rock_unit(&context, crate::Float3::new(f(0), f(1), f(2)))
                }
                UnitScriptCall::WorldHitByWeapon => {
                    result.float_value = script.world_hit_by_weapon(
                        &context,
                        crate::Float3::new(f(0), f(1), f(2)),
                        WeaponDefId(i(0)),
                        f(3),
                    );
                }
                UnitScriptCall::HitByWeapon => {
                    result.float_value = script.hit_by_weapon(
                        &context,
                        crate::Float3::new(f(0), f(1), f(2)),
                        WeaponDefId(i(0)),
                        f(3),
                    );
                }
                UnitScriptCall::SetSfxOccupy => script.set_sfx_occupy(&context, i(0)),
                UnitScriptCall::QueryLandingPads => {
                    let mut pieces = Vec::new();
                    script.query_landing_pads(&context, &mut pieces);
                    result
                        .int_values
                        .extend(pieces.into_iter().map(|piece| piece.0));
                }
                UnitScriptCall::BeginTransport => script.begin_transport(&context, UnitId(i(0))),
                UnitScriptCall::QueryTransport => {
                    result.int_value = script.query_transport(&context, UnitId(i(0))).0
                }
                UnitScriptCall::TransportPickup => script.transport_pickup(&context, UnitId(i(0))),
                UnitScriptCall::TransportDrop => script.transport_drop(
                    &context,
                    UnitId(i(0)),
                    crate::Float3::new(f(0), f(1), f(2)),
                ),
                UnitScriptCall::StartBuildingWithAim => {
                    script.start_building_with_aim(&context, f(0), f(1))
                }
                UnitScriptCall::QueryNanoPiece => {
                    result.int_value = script.query_nano_piece(&context).0
                }
                UnitScriptCall::QueryBuildInfo => {
                    result.int_value = script.query_build_info(&context).0
                }
                UnitScriptCall::Destroy => script.destroy(&context),
                UnitScriptCall::StartMoving => script.start_moving(&context, i(0) != 0),
                UnitScriptCall::StopMoving => script.stop_moving(&context),
                UnitScriptCall::StartSkidding => {
                    script.start_skidding(&context, crate::Float3::new(f(0), f(1), f(2)))
                }
                UnitScriptCall::StopSkidding => script.stop_skidding(&context),
                UnitScriptCall::ChangeHeading => script.change_heading(&context, i(0) as i16),
                UnitScriptCall::StartUnload => script.start_unload(&context),
                UnitScriptCall::EndTransport => script.end_transport(&context),
                UnitScriptCall::StartBuilding => script.start_building(&context),
                UnitScriptCall::StopBuilding => script.stop_building(&context),
                UnitScriptCall::Falling => script.falling(&context),
                UnitScriptCall::Landed => script.landed(&context),
                UnitScriptCall::Activate => script.activate(&context),
                UnitScriptCall::Deactivate => script.deactivate(&context),
                UnitScriptCall::MoveRate => script.move_rate(&context, i(0)),
                UnitScriptCall::FireWeapon => script.fire_weapon(&context, WeaponId(i(0))),
                UnitScriptCall::EndBurst => script.end_burst(&context, WeaponId(i(0))),
                UnitScriptCall::QueryWeapon => {
                    result.int_value = script.query_weapon(&context, WeaponId(i(0))).0
                }
                UnitScriptCall::AimWeapon => {
                    if let Some(ready) = script.aim_weapon(&context, WeaponId(i(0)), f(0), f(1)) {
                        result.bool_value = ready;
                        result.complete = true;
                    }
                }
                UnitScriptCall::AimShieldWeapon => {
                    if let Some(enabled) = script.aim_shield_weapon(&context, WeaponId(i(0))) {
                        result.bool_value = enabled;
                        result.complete = true;
                    }
                }
                UnitScriptCall::AimFromWeapon => {
                    result.int_value = script.aim_from_weapon(&context, WeaponId(i(0))).0
                }
                UnitScriptCall::Shot => script.shot(&context, WeaponId(i(0))),
                UnitScriptCall::BlockShot => {
                    result.bool_value =
                        script.block_shot(&context, WeaponId(i(0)), UnitId(i(1)), i(2) != 0)
                }
                UnitScriptCall::TargetWeight => {
                    result.float_value =
                        script.target_weight(&context, WeaponId(i(0)), UnitId(i(1)))
                }
                UnitScriptCall::AnimFinished => {
                    if let Some(key) = animation_key_from_call(i(0), i(1), i(2)) {
                        self.scheduler.wake_animation(key);
                    }
                    script.anim_finished(&context, i(0), Piece(i(1)), i(2))
                }
            }
            true
        })
    }

    /// Dispatch a synchronous query through the typed script state.
    pub fn query_landing_pads(&self, out: &mut Vec<Piece>) {
        let context = self.context();
        self.with_state(|script| script.query_landing_pads(&context, out));
    }

    #[inline]
    pub fn query_transport(&self, transportee: UnitId) -> Piece {
        let context = self.context();
        self.with_state(|script| script.query_transport(&context, transportee))
    }

    #[inline]
    pub fn query_weapon(&self, weapon: WeaponId) -> Piece {
        let context = self.context();
        self.with_state(|script| script.query_weapon(&context, weapon))
    }

    #[inline]
    pub fn aim_from_weapon(&self, weapon: WeaponId) -> Piece {
        let context = self.context();
        self.with_state(|script| script.aim_from_weapon(&context, weapon))
    }

    #[inline]
    pub fn block_shot(&self, weapon: WeaponId, target: UnitId, user_target: bool) -> bool {
        let context = self.context();
        self.with_state(|script| script.block_shot(&context, weapon, target, user_target))
    }

    #[inline]
    pub fn target_weight(&self, weapon: WeaponId, target: UnitId) -> f32 {
        let context = self.context();
        self.with_state(|script| script.target_weight(&context, weapon, target))
    }

    #[inline]
    pub fn query_nano_piece(&self) -> Piece {
        let context = self.context();
        self.with_state(|script| script.query_nano_piece(&context))
    }

    #[inline]
    pub fn query_build_info(&self) -> Piece {
        let context = self.context();
        self.with_state(|script| script.query_build_info(&context))
    }

    #[inline]
    pub fn hit_by_weapon(
        &self,
        hit_direction: crate::Float3,
        weapon: WeaponDefId,
        damage: f32,
    ) -> f32 {
        let context = self.context();
        self.with_state(|script| script.hit_by_weapon(&context, hit_direction, weapon, damage))
    }

    #[inline]
    pub fn world_hit_by_weapon(
        &self,
        hit_direction: crate::Float3,
        weapon: WeaponDefId,
        damage: f32,
    ) -> f32 {
        let context = self.context();
        self.with_state(|script| {
            script.world_hit_by_weapon(&context, hit_direction, weapon, damage)
        })
    }

    #[inline]
    pub fn killed(&self, recent_damage: f32, max_health: f32) -> WreckLevel {
        let context = self.context();
        self.with_state(|script| script.killed(&context, recent_damage, max_health))
    }
}
