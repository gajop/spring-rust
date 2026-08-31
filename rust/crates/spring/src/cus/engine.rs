/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use super::types::*;
use crate::UnitId;

/// Immediate operations and completion queries supplied by the engine.
///
/// Both the native adapter and the Core-Wasm host implement this trait. The
/// defaults make small tests and game-owned mock backends easy to write.
pub trait UnitEngine {
    fn turn(
        &mut self,
        _unit: UnitId,
        _piece: Piece,
        _axis: Axis,
        _destination: Angle,
        _speed: AngularSpeed,
    ) {
    }
    fn move_piece(
        &mut self,
        _unit: UnitId,
        _piece: Piece,
        _axis: Axis,
        _destination: f32,
        _speed: f32,
    ) {
    }
    fn spin(
        &mut self,
        _unit: UnitId,
        _piece: Piece,
        _axis: Axis,
        _speed: AngularSpeed,
        _acceleration: AngularSpeed,
    ) {
    }
    fn stop_spin(
        &mut self,
        _unit: UnitId,
        _piece: Piece,
        _axis: Axis,
        _deceleration: AngularSpeed,
    ) {
    }
    fn scale(&mut self, _unit: UnitId, _piece: Piece, _destination: f32, _speed: f32) {}
    fn move_now(&mut self, _unit: UnitId, _piece: Piece, _axis: Axis, _destination: f32) {}
    fn turn_now(&mut self, _unit: UnitId, _piece: Piece, _axis: Axis, _destination: Angle) {}
    fn scale_now(&mut self, _unit: UnitId, _piece: Piece, _destination: f32) {}
    fn show(&mut self, _unit: UnitId, _piece: Piece) {}
    fn hide(&mut self, _unit: UnitId, _piece: Piece) {}
    fn explode(&mut self, _unit: UnitId, _piece: Piece, _flags: SfxFlags) {}
    fn emit_sfx(&mut self, _unit: UnitId, _piece: Piece, _sfx: i32) {}
    fn attach_unit(&mut self, _unit: UnitId, _piece: Piece, _target: UnitId) {}
    fn drop_unit(&mut self, _unit: UnitId, _target: UnitId) {}
    fn set_unit_value(&mut self, _unit: UnitId, _value: i32, _parameter: i32) {}
    fn aim_script_finished(&mut self, _unit: UnitId, _weapon: WeaponId, _ready: bool) {}
    fn aim_shield_finished(&mut self, _unit: UnitId, _weapon: WeaponId, _enabled: bool) {}
    fn killed_script_finished(&mut self, _unit: UnitId, _wreck_level: WreckLevel) {}

    /// Return true while an engine animation is still active. This is checked
    /// once when a wait is created; completion wakes the task through the
    /// deterministic `AnimFinished` event.
    fn animation_active(
        &self,
        _unit: UnitId,
        _kind: AnimationKind,
        _piece: Piece,
        _axis: Option<Axis>,
    ) -> bool {
        false
    }
}

/// Animation kind used by completion waits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnimationKind {
    Turn,
    Move,
    Spin,
    Scale,
}

/// Resolves a model piece name during synchronous script construction.
pub trait PieceResolver {
    fn piece(&self, name: &str) -> Piece;
}

/// Construction-only context.  It has no scheduler and therefore cannot
/// accidentally start a task before the instance is registered.
pub struct InitCtx<'a> {
    unit: UnitId,
    pieces: &'a dyn PieceResolver,
}

impl<'a> InitCtx<'a> {
    #[inline]
    pub fn new(unit: UnitId, pieces: &'a dyn PieceResolver) -> Self {
        Self { unit, pieces }
    }

    #[inline]
    pub const fn unit(&self) -> UnitId {
        self.unit
    }

    #[inline]
    pub fn piece(&self, name: &str) -> Piece {
        self.pieces.piece(name)
    }
}
