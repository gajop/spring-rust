use spring::UnitId;
pub use spring::cus::{
    Angle, AngularSpeed, AnimationKind, Axis, CusHandle, CusInstance, CusRegistry, CusScheduler,
    Duration, DurationExt, InitCtx, Piece, PieceResolver, ScriptCapabilities, SfxFlags, SignalMask,
    TaskDefinition, TaskHandle, TaskState, UnitCtx, UnitEngine, UnitScript, UnitScriptCall,
    UnitScriptCallResult, WeaponId, WreckLevel,
};

use crate::{error::Error, interface::NativeInterfaceRef, sys};

const OP_TURN: u32 = 0;
const OP_MOVE: u32 = 1;
const OP_SPIN: u32 = 2;
const OP_STOP_SPIN: u32 = 3;
const OP_SCALE: u32 = 4;
const OP_MOVE_NOW: u32 = 5;
const OP_TURN_NOW: u32 = 6;
const OP_SCALE_NOW: u32 = 7;
const OP_SHOW: u32 = 8;
const OP_HIDE: u32 = 9;
const OP_EXPLODE: u32 = 10;
const OP_EMIT_SFX: u32 = 11;
const OP_ATTACH_UNIT: u32 = 12;
const OP_DROP_UNIT: u32 = 13;
const OP_SET_UNIT_VALUE: u32 = 14;
const OP_AIM_SCRIPT_FINISHED: u32 = 15;
const OP_AIM_SHIELD_FINISHED: u32 = 16;
const OP_KILLED_SCRIPT_FINISHED: u32 = 17;

const ANIMATION_TURN: u32 = 0;
const ANIMATION_MOVE: u32 = 1;
const ANIMATION_SPIN: u32 = 2;
const ANIMATION_SCALE: u32 = 3;

/// Native host access to the CUS attach and engine-operation API.
#[derive(Clone, Copy)]
pub struct NativeCus {
    interface: NativeInterfaceRef,
}

impl NativeCus {
    pub(crate) const fn new(interface: NativeInterfaceRef) -> Self {
        Self { interface }
    }

    fn api(&self) -> Result<&sys::CusApi, Error> {
        // SAFETY: NativeInterfaceRef can only be constructed from the engine's
        // live interface pointer, whose extension table is initialized before
        // module initialization.
        unsafe {
            self.interface
                .as_ptr()
                .as_ref()
                .and_then(|interface| interface.cus.as_ref())
                .ok_or_else(|| Error::new(1, "Rust CUS API is unavailable"))
        }
    }

    pub fn attach(
        &self,
        unit: UnitId,
        instance_id: u32,
        capabilities: ScriptCapabilities,
    ) -> Result<bool, Error> {
        let api = self.api()?;
        let function = api
            .Attach
            .ok_or_else(|| Error::new(1, "Rust CUS attach is unavailable"))?;
        let query = sys::CusAttachQuery {
            unitID: unit.0,
            instanceID: instance_id,
            capabilities: capabilities.0,
        };
        let mut result = sys::CusAttachResult {
            error: std::ptr::null(),
            attached: 0,
        };
        // SAFETY: query/result live for the synchronous C call.
        unsafe { function(&query, &mut result) };
        Error::result_or(result.error, result.attached != 0)
    }

    pub fn engine(&self, unit: UnitId, instance_id: u32) -> NativeCusEngine {
        NativeCusEngine {
            host: *self,
            unit,
            instance_id,
        }
    }
}

/// `spring::cus::UnitEngine` implementation for a native module.
pub struct NativeCusEngine {
    host: NativeCus,
    unit: UnitId,
    instance_id: u32,
}

/// Mutable results supplied to a native module's CUS dispatcher.  The output
/// slice borrows a buffer owned by the engine and is valid only for the
/// duration of the callback.
pub struct NativeCusCallResult<'a> {
    pub int_value: i32,
    pub float_value: f32,
    pub bool_value: bool,
    pub complete: bool,
    pub int_count: usize,
    pub int_values: &'a mut [i32],
}

impl NativeCusEngine {
    #[expect(
        clippy::too_many_arguments,
        reason = "The helper mirrors the fixed Rust CUS operation ABI"
    )]
    fn operation(
        &self,
        operation: u32,
        piece: i32,
        axis: i32,
        target: i32,
        value: i32,
        first: f32,
        second: f32,
        third: f32,
    ) {
        let Ok(api) = self.host.api() else { return };
        let Some(function) = api.Operation else {
            return;
        };
        let query = sys::CusOperationQuery {
            unitID: self.unit.0,
            instanceID: self.instance_id,
            operation,
            piece,
            axis,
            target,
            value,
            first,
            second,
            third,
        };
        let mut result = sys::CusOperationResult {
            error: std::ptr::null(),
            value: -1,
            completed: 0,
        };
        // SAFETY: query/result live for the synchronous C call.
        unsafe { function(&query, &mut result) };
    }

    fn animation_active(&self, animation: u32, piece: Piece, axis: i32) -> bool {
        let Ok(api) = self.host.api() else {
            return false;
        };
        let Some(function) = api.AnimationActive else {
            return false;
        };
        let query = sys::CusAnimationQuery {
            unitID: self.unit.0,
            instanceID: self.instance_id,
            animation,
            piece: piece.0,
            axis,
        };
        let mut result = sys::CusAnimationResult {
            error: std::ptr::null(),
            active: 0,
        };
        // SAFETY: query/result live for the synchronous C call.
        unsafe { function(&query, &mut result) };
        result.error.is_null() && result.active != 0
    }
}

impl UnitEngine for NativeCusEngine {
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

    fn aim_script_finished(&mut self, _: UnitId, weapon: WeaponId, ready: bool) {
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

    fn aim_shield_finished(&mut self, _: UnitId, weapon: WeaponId, enabled: bool) {
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

    fn killed_script_finished(&mut self, _: UnitId, wreck_level: WreckLevel) {
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
            AnimationKind::Turn => ANIMATION_TURN,
            AnimationKind::Move => ANIMATION_MOVE,
            AnimationKind::Spin => ANIMATION_SPIN,
            AnimationKind::Scale => ANIMATION_SCALE,
        };
        self.animation_active(animation, piece, axis.map_or(-1, Axis::index))
    }
}
