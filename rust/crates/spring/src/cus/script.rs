/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use super::engine::*;
use super::scheduler::*;
use super::types::*;
use crate::{UnitId, WeaponDefId};
use alloc::vec::Vec;

/// Standard synchronous unit-script surface. Suspendable behavior is
/// represented by named [`TaskDefinition`] values started through the context;
/// this keeps engine-consumed queries synchronous and makes the async boundary
/// explicit.
pub trait UnitScript: Sized {
    fn new(_ctx: &mut InitCtx<'_>) -> Self;

    fn raw_call(&mut self, _ctx: &UnitCtx, _function: i32) {}
    fn create(&mut self, _ctx: &UnitCtx) {}

    fn wind_changed(&mut self, _ctx: &UnitCtx, _heading: f32, _speed: f32) {}
    fn extraction_rate_changed(&mut self, _ctx: &UnitCtx, _speed: f32) {}
    fn world_rock_unit(&mut self, _ctx: &UnitCtx, _direction: crate::Float3) {}
    fn rock_unit(&mut self, _ctx: &UnitCtx, _direction: crate::Float3) {}
    fn world_hit_by_weapon(
        &mut self,
        _ctx: &UnitCtx,
        _hit_direction: crate::Float3,
        _weapon: WeaponDefId,
        damage: f32,
    ) -> f32 {
        damage
    }
    fn hit_by_weapon(
        &mut self,
        _ctx: &UnitCtx,
        _hit_direction: crate::Float3,
        _weapon: WeaponDefId,
        damage: f32,
    ) -> f32 {
        damage
    }
    fn set_sfx_occupy(&mut self, _ctx: &UnitCtx, _terrain_type: i32) {}

    fn query_landing_pads(&mut self, _ctx: &UnitCtx, _out: &mut Vec<Piece>) {}

    fn query_transport(&mut self, _ctx: &UnitCtx, _transportee: UnitId) -> Piece {
        Piece::INVALID
    }

    fn query_weapon(&mut self, _ctx: &UnitCtx, _weapon: WeaponId) -> Piece {
        Piece::INVALID
    }

    fn aim_from_weapon(&mut self, _ctx: &UnitCtx, _weapon: WeaponId) -> Piece {
        Piece::INVALID
    }

    fn block_shot(
        &mut self,
        _ctx: &UnitCtx,
        _weapon: WeaponId,
        _target: UnitId,
        _user_target: bool,
    ) -> bool {
        false
    }

    fn target_weight(&mut self, _ctx: &UnitCtx, _weapon: WeaponId, _target: UnitId) -> f32 {
        1.0
    }

    fn query_nano_piece(&mut self, _ctx: &UnitCtx) -> Piece {
        Piece::INVALID
    }

    fn query_build_info(&mut self, _ctx: &UnitCtx) -> Piece {
        Piece::INVALID
    }

    fn begin_transport(&mut self, _ctx: &UnitCtx, _transportee: UnitId) {}
    fn transport_pickup(&mut self, _ctx: &UnitCtx, _transportee: UnitId) {}
    fn transport_drop(&mut self, _ctx: &UnitCtx, _transportee: UnitId, _position: crate::Float3) {}
    fn start_building_with_aim(&mut self, _ctx: &UnitCtx, _heading: f32, _pitch: f32) {}
    fn destroy(&mut self, _ctx: &UnitCtx) {}
    fn start_moving(&mut self, _ctx: &UnitCtx, _reversing: bool) {}
    fn stop_moving(&mut self, _ctx: &UnitCtx) {}
    fn start_skidding(&mut self, _ctx: &UnitCtx, _velocity: crate::Float3) {}
    fn stop_skidding(&mut self, _ctx: &UnitCtx) {}
    fn change_heading(&mut self, _ctx: &UnitCtx, _delta: i16) {}
    fn start_unload(&mut self, _ctx: &UnitCtx) {}
    fn end_transport(&mut self, _ctx: &UnitCtx) {}
    fn start_building(&mut self, _ctx: &UnitCtx) {}
    fn stop_building(&mut self, _ctx: &UnitCtx) {}
    fn falling(&mut self, _ctx: &UnitCtx) {}
    fn landed(&mut self, _ctx: &UnitCtx) {}
    fn activate(&mut self, _ctx: &UnitCtx) {}
    fn deactivate(&mut self, _ctx: &UnitCtx) {}
    fn move_rate(&mut self, _ctx: &UnitCtx, _rate: i32) {}
    fn fire_weapon(&mut self, _ctx: &UnitCtx, _weapon: WeaponId) {}
    fn end_burst(&mut self, _ctx: &UnitCtx, _weapon: WeaponId) {}
    fn aim_weapon(
        &mut self,
        _ctx: &UnitCtx,
        _weapon: WeaponId,
        _heading: f32,
        _pitch: f32,
    ) -> Option<bool> {
        None
    }
    fn aim_shield_weapon(&mut self, _ctx: &UnitCtx, _weapon: WeaponId) -> Option<bool> {
        None
    }
    fn shot(&mut self, _ctx: &UnitCtx, _weapon: WeaponId) {}
    fn anim_finished(&mut self, _ctx: &UnitCtx, _kind: i32, _piece: Piece, _axis: i32) {}

    fn killed(&mut self, _ctx: &UnitCtx, _recent_damage: f32, _max_health: f32) -> WreckLevel {
        WreckLevel::None
    }
}

/// Capability bits used by the engine adapter to skip absent entry points.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct ScriptCapabilities(pub u64);

impl ScriptCapabilities {
    pub const RAW_CALL: u64 = 1 << 0;
    pub const CREATE: u64 = 1 << 1;
    pub const KILLED: u64 = 1 << 2;
    pub const WIND_CHANGED: u64 = 1 << 3;
    pub const EXTRACTION_RATE_CHANGED: u64 = 1 << 4;
    pub const WORLD_ROCK_UNIT: u64 = 1 << 5;
    pub const ROCK_UNIT: u64 = 1 << 6;
    pub const WORLD_HIT_BY_WEAPON: u64 = 1 << 7;
    pub const HIT_BY_WEAPON: u64 = 1 << 8;
    pub const SET_SFX_OCCUPY: u64 = 1 << 9;
    pub const QUERY_LANDING_PADS: u64 = 1 << 10;
    pub const BEGIN_TRANSPORT: u64 = 1 << 11;
    pub const QUERY_TRANSPORT: u64 = 1 << 12;
    pub const TRANSPORT_PICKUP: u64 = 1 << 13;
    pub const TRANSPORT_DROP: u64 = 1 << 14;
    pub const START_BUILDING_WITH_AIM: u64 = 1 << 15;
    pub const QUERY_NANO_PIECE: u64 = 1 << 16;
    pub const QUERY_BUILD_INFO: u64 = 1 << 17;
    pub const DESTROY: u64 = 1 << 18;
    pub const START_MOVING: u64 = 1 << 19;
    pub const STOP_MOVING: u64 = 1 << 20;
    pub const START_SKIDDING: u64 = 1 << 21;
    pub const STOP_SKIDDING: u64 = 1 << 22;
    pub const CHANGE_HEADING: u64 = 1 << 23;
    pub const START_UNLOAD: u64 = 1 << 24;
    pub const END_TRANSPORT: u64 = 1 << 25;
    pub const START_BUILDING: u64 = 1 << 26;
    pub const STOP_BUILDING: u64 = 1 << 27;
    pub const FALLING: u64 = 1 << 28;
    pub const LANDED: u64 = 1 << 29;
    pub const ACTIVATE: u64 = 1 << 30;
    pub const DEACTIVATE: u64 = 1 << 31;
    pub const MOVE_RATE: u64 = 1 << 32;
    pub const FIRE_WEAPON: u64 = 1 << 33;
    pub const END_BURST: u64 = 1 << 34;
    pub const QUERY_WEAPON: u64 = 1 << 35;
    pub const AIM_WEAPON: u64 = 1 << 36;
    pub const AIM_SHIELD_WEAPON: u64 = 1 << 37;
    pub const AIM_FROM_WEAPON: u64 = 1 << 38;
    pub const SHOT: u64 = 1 << 39;
    pub const BLOCK_SHOT: u64 = 1 << 40;
    pub const TARGET_WEIGHT: u64 = 1 << 41;
    pub const ANIM_FINISHED: u64 = 1 << 42;

    #[inline]
    pub const fn new(bits: u64) -> Self {
        Self(bits)
    }

    #[inline]
    pub const fn contains(self, capability: u64) -> bool {
        self.0 & capability != 0
    }
}
