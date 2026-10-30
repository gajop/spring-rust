/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

//! Columns: one property of many units in one engine call.
//!
//! `spring.columns().get_units_f32(UnitColumn::Velocity, &ids, ColumnArg::None, &mut out, Some(&mut found))`
//! fills `out` with every unit's x, then every unit's y, then every unit's z (`components()`
//! planes of `ids.len()` values), so both sides loop over plain arrays. Each entry is read
//! (or written) exactly as the per-unit call reads it, with that call's visibility; an entry
//! the per-unit call would answer with an error is 0 in `out` and 0 in `found`.
//!
//! In Wasm the whole column is one engine call (`spring:columns`, engines after `2026.10.3101`);
//! the native backend runs the per-unit calls.

use crate::capabilities::{Game, Synced};
use crate::{ApiError, Environment, ErrorCode, Result, Spring};

/// A unit property a column carries. The numbers are the engine ABI.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnitColumn {
    /// f32 x3, as `get_unit_position` with no options.
    Position = 1,
    /// f32 x3, `get_unit_position` with `mid_pos`.
    MidPosition = 2,
    /// f32 x3, `get_unit_position` with `aim_pos`.
    AimPosition = 3,
    /// f32 x3, `get_unit_velocity`.
    Velocity = 4,
    /// f32 x9: front xyz, up xyz, right xyz (`get_unit_vectors`).
    Vectors = 5,
    /// i32, `get_unit_ally_team`.
    AllyTeam = 6,
    /// i32, `get_unit_team`.
    Team = 7,
    /// i32, `get_unit_def_id`.
    DefId = 8,
    /// i32 0/1: `valid_unit_id` and not `get_unit_is_dead`. Always found.
    Alive = 9,
    /// i32 0/1, `get_unit_is_cloaked`.
    Cloaked = 10,
    /// f32 x2: health, max health (`get_unit_health`).
    Health = 11,
    /// f32, `get_unit_radius`.
    Radius = 12,
    /// i32 0/1, `is_unit_in_los`; the argument is the ally team. 0 where that call errs.
    InLos = 13,
    /// i32 0/1, `is_unit_in_radar`; the argument is the ally team.
    InRadar = 14,
    /// i32 0/1, `get_unit_is_stunned`.
    Stunned = 15,
    /// x5: type i32, unit i32, x y z f32 (`get_unit_weapon_target`); the argument is the
    /// weapon number from 1. Read it with [`Columns::get_units_raw`].
    WeaponTarget = 16,
    /// i32, the fire state of `get_unit_states`.
    FireState = 17,
    /// i32, the move state of `get_unit_states`.
    MoveState = 18,
    /// Set only, f32 x3: `move_ctrl().set_velocity`.
    MoveCtrlVelocity = 20,
    /// Set only, f32 x6: front xyz, right xyz (`set_unit_direction`).
    Direction = 21,
}

impl UnitColumn {
    /// Values per unit: the planes a column of this property has.
    pub const fn components(self) -> usize {
        match self {
            Self::Position | Self::MidPosition | Self::AimPosition | Self::Velocity => 3,
            Self::MoveCtrlVelocity => 3,
            Self::Vectors => 9,
            Self::Health => 2,
            Self::WeaponTarget => 5,
            Self::Direction => 6,
            _ => 1,
        }
    }

    const fn floats(self) -> bool {
        matches!(
            self,
            Self::Position
                | Self::MidPosition
                | Self::AimPosition
                | Self::Velocity
                | Self::Vectors
                | Self::Health
                | Self::Radius
                | Self::MoveCtrlVelocity
                | Self::Direction
        )
    }

    const fn settable(self) -> bool {
        matches!(self, Self::MoveCtrlVelocity | Self::Direction)
    }
}

/// The argument a property takes (an ally team, a weapon number): none, one for every unit,
/// or one per unit.
#[derive(Clone, Copy, Debug)]
pub enum ColumnArg<'a> {
    None,
    All(i32),
    Each(&'a [i32]),
}

impl<'a> ColumnArg<'a> {
    #[cfg(not(target_arch = "wasm32"))]
    fn of(self, i: usize) -> i32 {
        match self {
            Self::None => 0,
            Self::All(value) => value,
            Self::Each(values) => values[i],
        }
    }
    #[cfg(target_arch = "wasm32")]
    fn parts(self) -> (Option<&'a [i32]>, i32) {
        match self {
            Self::None => (None, 0),
            Self::All(value) => (None, value),
            Self::Each(values) => (Some(values), 0),
        }
    }
}

pub struct Columns<'a, 'call, E: Environment>(&'a Spring<'call, E>);

impl<'call, E: Environment> Spring<'call, E> {
    pub fn columns(&self) -> Columns<'_, 'call, E> {
        Columns(self)
    }
}

fn invalid<T>() -> Result<T> {
    Err(ApiError::new(ErrorCode::InvalidArgument as i32))
}

fn check_shape(
    property: UnitColumn,
    count: usize,
    planes: usize,
    flags: Option<&[u8]>,
    arg: ColumnArg<'_>,
) -> Result<()> {
    let args_ok = match arg {
        ColumnArg::Each(values) => values.len() == count,
        _ => true,
    };
    if planes != property.components() * count || flags.is_some_and(|f| f.len() != count) || !args_ok
    {
        return invalid();
    }
    Ok(())
}

impl<E: Game> Columns<'_, '_, E> {
    /// Read a float property of every unit in `units` into `out`; returns how many were found.
    pub fn get_units_f32(
        &self,
        property: UnitColumn,
        units: &[i32],
        arg: ColumnArg<'_>,
        out: &mut [f32],
        found: Option<&mut [u8]>,
    ) -> Result<usize> {
        if !property.floats() || property.settable() {
            return invalid();
        }
        // f32 and u32 share size and alignment; the engine writes IEEE bits
        let planes = unsafe { core::slice::from_raw_parts_mut(out.as_mut_ptr().cast::<u32>(), out.len()) };
        self.get_units_raw(property, units, arg, planes, found)
    }

    /// Read an integer property of every unit in `units` into `out`; returns how many were found.
    pub fn get_units_i32(
        &self,
        property: UnitColumn,
        units: &[i32],
        arg: ColumnArg<'_>,
        out: &mut [i32],
        found: Option<&mut [u8]>,
    ) -> Result<usize> {
        if property.floats() || property.settable() || property == UnitColumn::WeaponTarget {
            return invalid();
        }
        let planes = unsafe { core::slice::from_raw_parts_mut(out.as_mut_ptr().cast::<u32>(), out.len()) };
        self.get_units_raw(property, units, arg, planes, found)
    }

    /// Any readable property as raw 4-byte values (`f32::from_bits`, `as i32`), for mixed
    /// properties such as [`UnitColumn::WeaponTarget`].
    pub fn get_units_raw(
        &self,
        property: UnitColumn,
        units: &[i32],
        arg: ColumnArg<'_>,
        planes: &mut [u32],
        found: Option<&mut [u8]>,
    ) -> Result<usize> {
        if property.settable() {
            return invalid();
        }
        check_shape(property, units.len(), planes.len(), found.as_deref(), arg)?;
        self.0.engine.check()?;
        #[cfg(target_arch = "wasm32")]
        {
            let (args, arg) = arg.parts();
            self.0.engine.finish(spring_sdk_internal::columns::get_units(
                property as i32,
                units,
                args,
                arg,
                planes,
                found,
            ))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut found = found;
            planes.fill(0);
            let mut count = 0;
            for (i, &unit) in units.iter().enumerate() {
                let ok = self.read_one(property, unit, arg.of(i), planes, units.len(), i);
                if let Some(found) = found.as_deref_mut() {
                    found[i] = ok as u8;
                }
                count += ok as usize;
            }
            Ok(count)
        }
    }

    /// The float unit rules parameter `name` of every unit in `units`, as
    /// `get_unit_rules_param_f32` reads it; 0 where it has none. Returns how many had it.
    pub fn get_units_rules_param_f32(
        &self,
        name: &str,
        units: &[i32],
        out: &mut [f32],
        found: Option<&mut [u8]>,
    ) -> Result<usize> {
        if out.len() != units.len() || found.as_deref().is_some_and(|f| f.len() != units.len()) {
            return invalid();
        }
        self.0.engine.check()?;
        #[cfg(target_arch = "wasm32")]
        {
            self.0.engine.finish(spring_sdk_internal::columns::get_units_rules_param(
                name, units, out, found,
            ))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut found = found;
            let mut count = 0;
            for (i, &unit) in units.iter().enumerate() {
                let value = self.0.rules_params().get_unit_rules_param_f32(unit, name).ok();
                out[i] = value.unwrap_or(0.0);
                if let Some(found) = found.as_deref_mut() {
                    found[i] = value.is_some() as u8;
                }
                count += value.is_some() as usize;
            }
            Ok(count)
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn read_one(
        &self,
        property: UnitColumn,
        unit: i32,
        arg: i32,
        planes: &mut [u32],
        count: usize,
        i: usize,
    ) -> bool {
        let mut put = |plane: usize, bits: u32| planes[plane * count + i] = bits;
        let spring = self.0;
        let mut floats = |values: &[f32]| {
            for (plane, value) in values.iter().enumerate() {
                put(plane, value.to_bits());
            }
        };
        let info = spring.units_info();
        match property {
            UnitColumn::Position | UnitColumn::MidPosition | UnitColumn::AimPosition => info
                .get_unit_position(
                    unit,
                    crate::units_info::GetUnitPositionOptions {
                        mid_pos: property == UnitColumn::MidPosition,
                        aim_pos: property == UnitColumn::AimPosition,
                    },
                )
                .map(|p| floats(&[p.x, p.y, p.z]))
                .is_ok(),
            UnitColumn::Velocity => info
                .get_unit_velocity(unit)
                .map(|v| floats(&[v.x, v.y, v.z]))
                .is_ok(),
            UnitColumn::Vectors => info
                .get_unit_vectors(unit)
                .map(|v| {
                    let (f, u, r) = (v.front_dir, v.up_dir, v.right_dir);
                    floats(&[f.x, f.y, f.z, u.x, u.y, u.z, r.x, r.y, r.z])
                })
                .is_ok(),
            UnitColumn::Health => info
                .get_unit_health(unit)
                .map(|h| floats(&[h.health, h.max_health]))
                .is_ok(),
            UnitColumn::Radius => info.get_unit_radius(unit).map(|r| floats(&[r])).is_ok(),
            UnitColumn::AllyTeam => info.get_unit_ally_team(unit).map(|v| put(0, v as u32)).is_ok(),
            UnitColumn::Team => info.get_unit_team(unit).map(|v| put(0, v as u32)).is_ok(),
            UnitColumn::DefId => info.get_unit_def_id(unit).map(|v| put(0, v as u32)).is_ok(),
            UnitColumn::Alive => {
                let alive = spring.units_query().valid_unit_id(unit).unwrap_or(false)
                    && !info.get_unit_is_dead(unit).unwrap_or(true);
                put(0, alive as u32);
                true
            }
            UnitColumn::Cloaked => info.get_unit_is_cloaked(unit).map(|v| put(0, v as u32)).is_ok(),
            UnitColumn::Stunned => info.get_unit_is_stunned(unit).map(|v| put(0, v as u32)).is_ok(),
            UnitColumn::InLos => spring.los().is_unit_in_los(unit, arg).map(|v| put(0, v as u32)).is_ok(),
            UnitColumn::InRadar => spring.los().is_unit_in_radar(unit, arg).map(|v| put(0, v as u32)).is_ok(),
            UnitColumn::WeaponTarget => spring
                .units_weapons()
                .get_unit_weapon_target(unit, arg)
                .map(|t| {
                    put(0, t.target_type as u32);
                    put(1, t.target_id as u32);
                    put(2, t.target_pos.x.to_bits());
                    put(3, t.target_pos.y.to_bits());
                    put(4, t.target_pos.z.to_bits());
                })
                .is_ok(),
            UnitColumn::FireState | UnitColumn::MoveState => info
                .get_unit_states(unit, Default::default())
                .map(|s| {
                    put(0, if property == UnitColumn::FireState { s.fire_state } else { s.move_state } as u32)
                })
                .is_ok(),
            UnitColumn::MoveCtrlVelocity | UnitColumn::Direction => false,
        }
    }
}

impl<E: Synced> Columns<'_, '_, E> {
    /// Write a float property of every unit in `units` from `values` (planes as for reads);
    /// returns how many were applied.
    pub fn set_units_f32(
        &self,
        property: UnitColumn,
        units: &[i32],
        values: &[f32],
        done: Option<&mut [u8]>,
    ) -> Result<usize> {
        if !property.settable() {
            return invalid();
        }
        check_shape(property, units.len(), values.len(), done.as_deref(), ColumnArg::None)?;
        self.0.engine.check()?;
        #[cfg(target_arch = "wasm32")]
        {
            let planes = unsafe { core::slice::from_raw_parts(values.as_ptr().cast::<u32>(), values.len()) };
            self.0.engine.finish(spring_sdk_internal::columns::set_units(
                property as i32,
                units,
                None,
                0,
                planes,
                done,
            ))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut done = done;
            let count = units.len();
            let at = |plane: usize, i: usize| values[plane * count + i];
            let mut applied = 0;
            for (i, &unit) in units.iter().enumerate() {
                let vector = |first: usize| crate::Float3 {
                    x: at(first, i),
                    y: at(first + 1, i),
                    z: at(first + 2, i),
                };
                let ok = match property {
                    UnitColumn::MoveCtrlVelocity => {
                        self.0.move_ctrl().set_velocity(unit, vector(0)).unwrap_or(false)
                    }
                    UnitColumn::Direction => self
                        .0
                        .unit_control()
                        .set_unit_direction(unit, vector(0), vector(3))
                        .unwrap_or(false),
                    _ => false,
                };
                if let Some(done) = done.as_deref_mut() {
                    done[i] = ok as u8;
                }
                applied += ok as usize;
            }
            Ok(applied)
        }
    }
}
