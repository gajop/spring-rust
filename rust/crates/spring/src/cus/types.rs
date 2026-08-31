/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use alloc::vec::Vec;

pub struct Angle(pub f32);

impl Angle {
    pub const ZERO: Self = Self(0.0);

    #[inline]
    pub const fn radians(value: f32) -> Self {
        Self(value)
    }

    #[inline]
    pub const fn degrees(value: f32) -> Self {
        Self(value * core::f32::consts::PI / 180.0)
    }

    #[inline]
    pub const fn value(self) -> f32 {
        self.0
    }
}

impl core::ops::Neg for Angle {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

/// Convenience constructor for authored degree values such as `20.deg()`.
pub trait AngleExt {
    fn deg(self) -> Angle;
}

macro_rules! impl_angle_ext {
    ($($type:ty),* $(,)?) => {
        $(
            impl AngleExt for $type {
                #[inline]
                fn deg(self) -> Angle {
                    Angle::degrees(self as f32)
                }
            }
        )*
    };
}

impl_angle_ext!(f32, f64, i32, i64, u32, u64, usize);

/// A unit-script piece number.  Piece numbers are zero-based, matching the
/// engine's `CUnitScript` storage and the native/Core Rust APIs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Piece(pub i32);

impl Piece {
    pub const INVALID: Self = Self(-1);

    #[inline]
    pub const fn index(self) -> usize {
        if self.0 < 0 { 0 } else { self.0 as usize }
    }

    #[inline]
    pub const fn value(self) -> i32 {
        self.0
    }
}

impl From<i32> for Piece {
    #[inline]
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<Piece> for i32 {
    #[inline]
    fn from(value: Piece) -> Self {
        value.0
    }
}

/// A weapon number in the portable unit-script API.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct WeaponId(pub i32);

impl WeaponId {
    #[inline]
    pub const fn index(self) -> usize {
        if self.0 < 0 { 0 } else { self.0 as usize }
    }

    #[inline]
    pub const fn value(self) -> i32 {
        self.0
    }
}

impl From<i32> for WeaponId {
    #[inline]
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<WeaponId> for i32 {
    #[inline]
    fn from(value: WeaponId) -> Self {
        value.0
    }
}

/// A model axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

impl Axis {
    #[inline]
    pub const fn index(self) -> i32 {
        self as i32
    }
}

/// An angular speed in radians per second.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(transparent)]
pub struct AngularSpeed(pub f32);

impl AngularSpeed {
    #[inline]
    pub const fn radians_per_second(value: f32) -> Self {
        Self(value)
    }

    #[inline]
    pub fn degrees_per_second(value: f32) -> Self {
        Self(value * core::f32::consts::PI / 180.0)
    }

    #[inline]
    pub const fn value(self) -> f32 {
        self.0
    }
}

/// Convenience constructor for authored angular speeds such as
/// `380.deg_per_sec()`.
pub trait AngularSpeedExt {
    fn deg_per_sec(self) -> AngularSpeed;
}

macro_rules! impl_angular_speed_ext {
    ($($type:ty),* $(,)?) => {
        $(
            impl AngularSpeedExt for $type {
                #[inline]
                fn deg_per_sec(self) -> AngularSpeed {
                    AngularSpeed::degrees_per_second(self as f32)
                }
            }
        )*
    };
}

impl_angular_speed_ext!(f32, f64, i32, i64, u32, u64, usize);

/// A deterministic duration used by CUS waits.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration {
    milliseconds: u64,
}

impl Duration {
    #[inline]
    pub const fn from_millis(milliseconds: u64) -> Self {
        Self { milliseconds }
    }

    #[inline]
    pub const fn from_secs(seconds: u64) -> Self {
        Self::from_millis(seconds.saturating_mul(1000))
    }

    #[inline]
    pub const fn as_millis(self) -> u64 {
        self.milliseconds
    }

    /// Convert a LUS-compatible millisecond sleep to simulation frames.
    ///
    /// The literal divisor and minimum are intentional compatibility rules:
    /// `Sleep(0)` resumes on the next frame and `Sleep(ms)` uses
    /// `max(1, floor(ms / 33))`.
    #[inline]
    pub const fn to_frames(self) -> u64 {
        let frames = self.milliseconds / 33;
        if frames == 0 { 1 } else { frames }
    }
}

/// Convenient duration constructors for game-owned code.
pub trait DurationExt {
    fn millis(self) -> Duration;
    fn seconds(self) -> Duration;
}

impl DurationExt for u64 {
    #[inline]
    fn millis(self) -> Duration {
        Duration::from_millis(self)
    }

    #[inline]
    fn seconds(self) -> Duration {
        Duration::from_secs(self)
    }
}

impl DurationExt for u32 {
    #[inline]
    fn millis(self) -> Duration {
        Duration::from_millis(self as u64)
    }

    #[inline]
    fn seconds(self) -> Duration {
        Duration::from_secs(self as u64)
    }
}

impl DurationExt for usize {
    #[inline]
    fn millis(self) -> Duration {
        Duration::from_millis(self as u64)
    }

    #[inline]
    fn seconds(self) -> Duration {
        Duration::from_secs(self as u64)
    }
}

impl DurationExt for i32 {
    #[inline]
    fn millis(self) -> Duration {
        Duration::from_millis(self.max(0) as u64)
    }

    #[inline]
    fn seconds(self) -> Duration {
        Duration::from_secs(self.max(0) as u64)
    }
}

impl DurationExt for i64 {
    #[inline]
    fn millis(self) -> Duration {
        Duration::from_millis(self.max(0) as u64)
    }

    #[inline]
    fn seconds(self) -> Duration {
        Duration::from_secs(self.max(0) as u64)
    }
}

/// Numeric LUS-compatible signal mask.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SignalMask(pub u32);

impl SignalMask {
    #[inline]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    #[inline]
    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }
}

/// The result level used by the engine after a death script settles.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(i32)]
pub enum WreckLevel {
    #[default]
    None = -1,
    One = 1,
    Two = 2,
    Three = 3,
}

/// Stable discriminants shared by the C++ adapter and both transport layers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum UnitScriptCall {
    RawCall = 0,
    Create = 1,
    Killed = 2,
    WindChanged = 3,
    ExtractionRateChanged = 4,
    WorldRockUnit = 5,
    RockUnit = 6,
    WorldHitByWeapon = 7,
    HitByWeapon = 8,
    SetSfxOccupy = 9,
    QueryLandingPads = 10,
    BeginTransport = 11,
    QueryTransport = 12,
    TransportPickup = 13,
    TransportDrop = 14,
    StartBuildingWithAim = 15,
    QueryNanoPiece = 16,
    QueryBuildInfo = 17,
    Destroy = 18,
    StartMoving = 19,
    StopMoving = 20,
    StartSkidding = 21,
    StopSkidding = 22,
    ChangeHeading = 23,
    StartUnload = 24,
    EndTransport = 25,
    StartBuilding = 26,
    StopBuilding = 27,
    Falling = 28,
    Landed = 29,
    Activate = 30,
    Deactivate = 31,
    MoveRate = 32,
    FireWeapon = 33,
    EndBurst = 34,
    QueryWeapon = 35,
    AimWeapon = 36,
    AimShieldWeapon = 37,
    AimFromWeapon = 38,
    Shot = 39,
    BlockShot = 40,
    TargetWeight = 41,
    AnimFinished = 42,
}

impl UnitScriptCall {
    #[inline]
    pub const fn from_u32(value: u32) -> Option<Self> {
        Some(match value {
            0 => Self::RawCall,
            1 => Self::Create,
            2 => Self::Killed,
            3 => Self::WindChanged,
            4 => Self::ExtractionRateChanged,
            5 => Self::WorldRockUnit,
            6 => Self::RockUnit,
            7 => Self::WorldHitByWeapon,
            8 => Self::HitByWeapon,
            9 => Self::SetSfxOccupy,
            10 => Self::QueryLandingPads,
            11 => Self::BeginTransport,
            12 => Self::QueryTransport,
            13 => Self::TransportPickup,
            14 => Self::TransportDrop,
            15 => Self::StartBuildingWithAim,
            16 => Self::QueryNanoPiece,
            17 => Self::QueryBuildInfo,
            18 => Self::Destroy,
            19 => Self::StartMoving,
            20 => Self::StopMoving,
            21 => Self::StartSkidding,
            22 => Self::StopSkidding,
            23 => Self::ChangeHeading,
            24 => Self::StartUnload,
            25 => Self::EndTransport,
            26 => Self::StartBuilding,
            27 => Self::StopBuilding,
            28 => Self::Falling,
            29 => Self::Landed,
            30 => Self::Activate,
            31 => Self::Deactivate,
            32 => Self::MoveRate,
            33 => Self::FireWeapon,
            34 => Self::EndBurst,
            35 => Self::QueryWeapon,
            36 => Self::AimWeapon,
            37 => Self::AimShieldWeapon,
            38 => Self::AimFromWeapon,
            39 => Self::Shot,
            40 => Self::BlockShot,
            41 => Self::TargetWeight,
            42 => Self::AnimFinished,
            _ => return None,
        })
    }
}

/// Result filled by the transport callback for one standard CUS call.
#[derive(Debug, Default)]
pub struct UnitScriptCallResult {
    pub int_value: i32,
    pub float_value: f32,
    pub bool_value: bool,
    pub complete: bool,
    pub int_values: Vec<i32>,
}

/// Flags accepted by the engine's piece explosion operation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct SfxFlags(pub i32);

impl SfxFlags {
    pub const SHATTER: Self = Self(1 << 0);
    pub const EXPLODE: Self = Self(1 << 1);
    pub const FALL: Self = Self(1 << 2);
    pub const SMOKE: Self = Self(1 << 3);
    pub const FIRE: Self = Self(1 << 4);
    pub const NONE: Self = Self(1 << 5);
    pub const NO_CEG_TRAIL: Self = Self(1 << 6);
    pub const NO_HEATCLOUD: Self = Self(1 << 7);
    pub const RECURSIVE: Self = Self(1 << 14);
}
