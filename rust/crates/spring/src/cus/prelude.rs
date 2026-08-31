/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

/// The imports normally needed by a game-owned CUS module.
pub use alloc::boxed::Box;
pub use alloc::vec::Vec;

pub use super::{
    Angle, AngleExt, AngularSpeed, AngularSpeedExt, AnimationKind, AnimationWait, Axis, CusHandle,
    CusInstance, CusRegistry, CusScheduler, Duration, DurationExt, InitCtx, Piece,
    ScriptCapabilities, SfxFlags, SignalMask, TaskDefinition, TaskFuture, TaskHandle, TaskState,
    UnitCtx, UnitEngine, UnitScript, WeaponId, WreckLevel,
};
pub use crate::{UnitId, WeaponDefId};
