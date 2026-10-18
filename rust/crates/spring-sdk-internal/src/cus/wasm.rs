/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use super::{Angle, AngularSpeed, AnimationKind, Axis, Piece, ScriptCapabilities, SfxFlags};
use crate::{ApiError, Result, UnitId, unpack_bool};

#[link(wasm_import_module = "spring:cus")]
unsafe extern "C" {
    #[link_name = "attach"]
    fn attach(unit: i32, instance: i32, capabilities_low: i32, capabilities_high: i32) -> i64;
    #[link_name = "operation"]
    fn operation(
        unit: i32,
        instance: i32,
        operation: i32,
        piece: i32,
        axis: i32,
        target: i32,
        value: i32,
        first: f32,
        second: f32,
        third: f32,
    ) -> i64;
    #[link_name = "animation-active"]
    fn animation_active(unit: i32, instance: i32, animation: i32, piece: i32, axis: i32) -> i64;
}

const OP_TURN: i32 = 0;
const OP_MOVE: i32 = 1;
const OP_SPIN: i32 = 2;
const OP_STOP_SPIN: i32 = 3;
const OP_SCALE: i32 = 4;
const OP_MOVE_NOW: i32 = 5;
const OP_TURN_NOW: i32 = 6;
const OP_SCALE_NOW: i32 = 7;
const OP_SHOW: i32 = 8;
const OP_HIDE: i32 = 9;
const OP_EXPLODE: i32 = 10;
const OP_EMIT_SFX: i32 = 11;
const OP_ATTACH_UNIT: i32 = 12;
const OP_DROP_UNIT: i32 = 13;
const OP_SET_UNIT_VALUE: i32 = 14;
const OP_AIM_SCRIPT_FINISHED: i32 = 15;
const OP_AIM_SHIELD_FINISHED: i32 = 16;
const OP_KILLED_SCRIPT_FINISHED: i32 = 17;

/// The host identity of one attached Rust CUS instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WasmCus {
    unit: UnitId,
    instance: u32,
}

impl WasmCus {
    pub fn attach(unit: UnitId, instance: u32, capabilities: ScriptCapabilities) -> Result<Self> {
        let packed = unsafe {
            attach(
                unit.0,
                instance as i32,
                capabilities.0 as u32 as i32,
                (capabilities.0 >> 32) as u32 as i32,
            )
        };
        if unpack_bool(packed)? {
            Ok(Self { unit, instance })
        } else {
            Err(ApiError::new(8))
        }
    }

    pub const fn new(unit: UnitId, instance: u32) -> Self {
        Self { unit, instance }
    }

    pub const fn unit(self) -> UnitId {
        self.unit
    }

    pub const fn instance(self) -> u32 {
        self.instance
    }

    pub fn engine(self) -> WasmCusEngine {
        WasmCusEngine { host: self }
    }
}

pub struct WasmCusEngine {
    host: WasmCus,
}

impl WasmCusEngine {
    fn operation(
        &self,
        operation_id: i32,
        piece: i32,
        axis: i32,
        target: i32,
        value: i32,
        first: f32,
        second: f32,
        third: f32,
    ) {
        unsafe {
            let _ = operation(
                self.host.unit.0,
                self.host.instance as i32,
                operation_id,
                piece,
                axis,
                target,
                value,
                first,
                second,
                third,
            );
        }
    }

    fn active(&self, animation: i32, piece: Piece, axis: i32) -> bool {
        unsafe {
            unpack_bool(animation_active(
                self.host.unit.0,
                self.host.instance as i32,
                animation,
                piece.0,
                axis,
            ))
            .unwrap_or(false)
        }
    }
}

impl super::UnitEngine for WasmCusEngine {
    fn turn(
        &mut self,
        _: UnitId,
        piece: Piece,
        axis: Axis,
        destination: Angle,
        speed: AngularSpeed,
    ) {
        self.operation(
            OP_TURN,
            piece.0,
            axis.index(),
            -1,
            0,
            destination.0,
            speed.0,
            0.0,
        );
    }

    fn move_piece(&mut self, _: UnitId, piece: Piece, axis: Axis, destination: f32, speed: f32) {
        self.operation(
            OP_MOVE,
            piece.0,
            axis.index(),
            -1,
            0,
            destination,
            speed,
            0.0,
        );
    }

    fn spin(
        &mut self,
        _: UnitId,
        piece: Piece,
        axis: Axis,
        speed: AngularSpeed,
        acceleration: AngularSpeed,
    ) {
        self.operation(
            OP_SPIN,
            piece.0,
            axis.index(),
            -1,
            0,
            speed.0,
            acceleration.0,
            0.0,
        );
    }

    fn stop_spin(&mut self, _: UnitId, piece: Piece, axis: Axis, deceleration: AngularSpeed) {
        self.operation(
            OP_STOP_SPIN,
            piece.0,
            axis.index(),
            -1,
            0,
            deceleration.0,
            0.0,
            0.0,
        );
    }

    fn scale(&mut self, _: UnitId, piece: Piece, destination: f32, speed: f32) {
        self.operation(OP_SCALE, piece.0, -1, -1, 0, destination, speed, 0.0);
    }

    fn move_now(&mut self, _: UnitId, piece: Piece, axis: Axis, destination: f32) {
        self.operation(
            OP_MOVE_NOW,
            piece.0,
            axis.index(),
            -1,
            0,
            destination,
            0.0,
            0.0,
        );
    }

    fn turn_now(&mut self, _: UnitId, piece: Piece, axis: Axis, destination: Angle) {
        self.operation(
            OP_TURN_NOW,
            piece.0,
            axis.index(),
            -1,
            0,
            destination.0,
            0.0,
            0.0,
        );
    }

    fn scale_now(&mut self, _: UnitId, piece: Piece, destination: f32) {
        self.operation(OP_SCALE_NOW, piece.0, -1, -1, 0, destination, 0.0, 0.0);
    }

    fn show(&mut self, _: UnitId, piece: Piece) {
        self.operation(OP_SHOW, piece.0, -1, -1, 0, 0.0, 0.0, 0.0);
    }

    fn hide(&mut self, _: UnitId, piece: Piece) {
        self.operation(OP_HIDE, piece.0, -1, -1, 0, 0.0, 0.0, 0.0);
    }

    fn explode(&mut self, _: UnitId, piece: Piece, flags: SfxFlags) {
        self.operation(OP_EXPLODE, piece.0, -1, -1, flags.0, 0.0, 0.0, 0.0);
    }

    fn emit_sfx(&mut self, _: UnitId, piece: Piece, sfx: i32) {
        self.operation(OP_EMIT_SFX, piece.0, -1, -1, sfx, 0.0, 0.0, 0.0);
    }

    fn attach_unit(&mut self, _: UnitId, piece: Piece, target: UnitId) {
        self.operation(OP_ATTACH_UNIT, piece.0, -1, target.0, 0, 0.0, 0.0, 0.0);
    }

    fn drop_unit(&mut self, _: UnitId, target: UnitId) {
        self.operation(OP_DROP_UNIT, -1, -1, target.0, 0, 0.0, 0.0, 0.0);
    }

    fn set_unit_value(&mut self, _: UnitId, value: i32, parameter: i32) {
        self.operation(OP_SET_UNIT_VALUE, -1, -1, parameter, value, 0.0, 0.0, 0.0);
    }

    fn aim_script_finished(&mut self, _: UnitId, weapon: super::WeaponId, ready: bool) {
        self.operation(
            OP_AIM_SCRIPT_FINISHED,
            -1,
            -1,
            weapon.0,
            ready as i32,
            0.0,
            0.0,
            0.0,
        );
    }

    fn aim_shield_finished(&mut self, _: UnitId, weapon: super::WeaponId, enabled: bool) {
        self.operation(
            OP_AIM_SHIELD_FINISHED,
            -1,
            -1,
            weapon.0,
            enabled as i32,
            0.0,
            0.0,
            0.0,
        );
    }

    fn killed_script_finished(&mut self, _: UnitId, wreck_level: super::WreckLevel) {
        self.operation(
            OP_KILLED_SCRIPT_FINISHED,
            -1,
            -1,
            -1,
            wreck_level as i32,
            0.0,
            0.0,
            0.0,
        );
    }

    fn animation_active(
        &self,
        _: UnitId,
        kind: AnimationKind,
        piece: Piece,
        axis: Option<Axis>,
    ) -> bool {
        let animation = match kind {
            AnimationKind::Turn => 0,
            AnimationKind::Move => 1,
            AnimationKind::Spin => 2,
            AnimationKind::Scale => 3,
        };
        self.active(animation, piece, axis.map_or(-1, Axis::index))
    }
}
