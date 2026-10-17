/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use super::*;
use crate::{ApiError, ErrorCode, ModuleRef, Result, Spring, UnitId, capabilities::Synced};

pub struct Cus<'call, E: Synced> {
    spring: Spring<'call, E>,
}

impl<E: Synced> Spring<'_, E> {
    pub fn cus(&self) -> Cus<'_, E> {
        Cus {
            spring: self.reborrow(),
        }
    }
}

impl<E: Synced> Cus<'_, E> {
    pub fn attach(
        &self,
        unit: UnitId,
        instance: u32,
        capabilities: ScriptCapabilities,
    ) -> Result<()> {
        self.spring.engine.check()?;
        #[cfg(not(target_arch = "wasm32"))]
        let attached = {
            let function = self.api()?.Attach.ok_or_else(unavailable)?;
            let query = crate::__sys::CusAttachQuery {
                unitID: unit.0,
                instanceID: instance,
                capabilities: capabilities.0,
            };
            let mut result = crate::__sys::CusAttachResult::default();
            unsafe {
                function(&query, &mut result);
            }
            unsafe {
                crate::native::check_error(result.error)?;
            }
            result.attached != 0
        };
        #[cfg(target_arch = "wasm32")]
        let attached = unpack_bool(unsafe {
            imports::attach(
                unit.0,
                instance as i32,
                capabilities.0 as i32,
                (capabilities.0 >> 32) as i32,
            )
        })?;
        self.spring.engine.finish(if attached {
            Ok(())
        } else {
            Err(ApiError::new(ErrorCode::InvalidState as i32))
        })
    }

    pub fn engine(&self, unit: UnitId, instance: u32) -> CusEngine<E> {
        CusEngine {
            module: self.spring.module_ref(),
            unit,
            instance,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn api(&self) -> Result<&crate::__sys::CusApi> {
        self.spring.engine.check()?;
        unsafe {
            self.spring
                .engine
                .raw()
                .as_ref()
                .and_then(|interface| interface.cus.as_ref())
        }
        .ok_or_else(unavailable)
    }

    fn operation(
        &self,
        unit: UnitId,
        instance: u32,
        operation: u32,
        piece: i32,
        axis: i32,
        target: i32,
        value: i32,
        first: f32,
        second: f32,
        third: f32,
    ) -> Result<()> {
        self.spring.engine.check()?;
        #[cfg(not(target_arch = "wasm32"))]
        let outcome = {
            let function = self.api()?.Operation.ok_or_else(unavailable)?;
            let query = crate::__sys::CusOperationQuery {
                unitID: unit.0,
                instanceID: instance,
                operation,
                piece,
                axis,
                target,
                value,
                first,
                second,
                third,
            };
            let mut result = crate::__sys::CusOperationResult::default();
            unsafe {
                function(&query, &mut result);
                crate::native::check_error(result.error)?;
            }
            if result.completed != 0 {
                Ok(())
            } else {
                Err(ApiError::new(ErrorCode::OperationFailed as i32))
            }
        };
        #[cfg(target_arch = "wasm32")]
        let outcome = unpack_i32(unsafe {
            imports::operation(
                unit.0,
                instance as i32,
                operation as i32,
                piece,
                axis,
                target,
                value,
                first,
                second,
                third,
            )
        })
        .map(|_| ());
        self.spring.engine.finish(outcome)
    }

    fn animation_active(
        &self,
        unit: UnitId,
        instance: u32,
        animation: u32,
        piece: Piece,
        axis: i32,
    ) -> Result<bool> {
        self.spring.engine.check()?;
        #[cfg(not(target_arch = "wasm32"))]
        let outcome = {
            let function = self.api()?.AnimationActive.ok_or_else(unavailable)?;
            let query = crate::__sys::CusAnimationQuery {
                unitID: unit.0,
                instanceID: instance,
                animation,
                piece: piece.0,
                axis,
            };
            let mut result = crate::__sys::CusAnimationResult::default();
            unsafe {
                function(&query, &mut result);
            }
            unsafe { crate::native::check_error(result.error) }.map(|_| result.active != 0)
        };
        #[cfg(target_arch = "wasm32")]
        let outcome = unpack_bool(unsafe {
            imports::animation_active(unit.0, instance as i32, animation as i32, piece.0, axis)
        });
        self.spring.engine.finish(outcome)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn unavailable() -> ApiError {
    ApiError::new(ErrorCode::NotAvailable as i32)
}

#[cfg(target_arch = "wasm32")]
fn unpack_i32(packed: i64) -> Result<i32> {
    let status = ((packed as u64) >> 32) as u32 as i32;
    if status == 0 {
        Ok(packed as i32)
    } else {
        Err(ApiError::new(status))
    }
}
#[cfg(target_arch = "wasm32")]
fn unpack_bool(packed: i64) -> Result<bool> {
    match unpack_i32(packed)? {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(ApiError::new(ErrorCode::Internal as i32)),
    }
}

pub struct CusEngine<E: Synced> {
    module: ModuleRef<E>,
    unit: UnitId,
    instance: u32,
}

impl<E: Synced> CusEngine<E> {
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
        self.module
            .with_spring(|spring| {
                spring.cus().operation(
                    self.unit,
                    self.instance,
                    operation,
                    piece,
                    axis,
                    target,
                    value,
                    first,
                    second,
                    third,
                )
            })
            .expect("CUS engine operation failed");
    }
    fn animation_active(&self, animation: u32, piece: Piece, axis: i32) -> bool {
        self.module
            .with_spring(|spring| {
                spring
                    .cus()
                    .animation_active(self.unit, self.instance, animation, piece, axis)
            })
            .expect("CUS animation query failed")
    }
}

#[cfg(target_arch = "wasm32")]
mod imports {
    #[link(wasm_import_module = "spring:cus")]
    unsafe extern "C" {
        pub fn attach(
            unit: i32,
            instance: i32,
            capabilities_low: i32,
            capabilities_high: i32,
        ) -> i64;
        pub fn operation(
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
        pub fn animation_active(
            unit: i32,
            instance: i32,
            animation: i32,
            piece: i32,
            axis: i32,
        ) -> i64;
    }
}

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

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::{__sys as sys, All, ModuleContext};

    #[test]
    fn missing_cus_table_is_an_error() {
        let interface = sys::NativeInterface::default();
        let context = unsafe { ModuleContext::<All>::from_native(&interface) }.unwrap();
        context
            .with_spring(|spring| {
                assert_eq!(
                    spring
                        .cus()
                        .attach(UnitId(1), 1, ScriptCapabilities(0))
                        .unwrap_err()
                        .code,
                    ErrorCode::NotAvailable as i32
                );
            })
            .unwrap();
    }

    unsafe extern "C" fn incomplete(
        _: *const sys::CusOperationQuery,
        _: *mut sys::CusOperationResult,
    ) {
    }
    #[test]
    fn incomplete_engine_operation_is_an_error() {
        let api = sys::CusApi {
            Operation: Some(incomplete),
            ..Default::default()
        };
        let interface = sys::NativeInterface {
            cus: &api,
            ..Default::default()
        };
        let context = unsafe { ModuleContext::<All>::from_native(&interface) }.unwrap();
        context
            .with_spring(|spring| {
                assert_eq!(
                    spring
                        .cus()
                        .operation(UnitId(1), 1, OP_SHOW, 1, -1, -1, 0, 0.0, 0.0, 0.0)
                        .unwrap_err()
                        .code,
                    ErrorCode::OperationFailed as i32
                );
            })
            .unwrap();
    }
}

impl<E: Synced> UnitEngine for CusEngine<E> {
    fn engine_scope(&self) -> super::shared::EngineScope {
        Some(crate::ModuleRef {
            owner: self.module.owner.clone(),
            environment: core::marker::PhantomData,
        })
    }
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
