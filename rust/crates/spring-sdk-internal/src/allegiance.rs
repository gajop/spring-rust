/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

//! The `allegiance` argument of the `get_units_in_*` queries: a team id (0 or more) or one of
//! these, as Lua's `Spring.ALL_UNITS` and friends. "My" and "ally" are the reading module's team
//! and ally team: a UI module's player; a synced module reads as no team, so for it
//! [`MY_UNITS`] and [`ALLY_UNITS`] find nothing and [`ENEMY_UNITS`] finds every unit.

/// Every unit.
pub const ALL_UNITS: i32 = -1;
/// Units of the reading team.
pub const MY_UNITS: i32 = -2;
/// Units of the reading ally team.
pub const ALLY_UNITS: i32 = -3;
/// Units outside the reading ally team.
pub const ENEMY_UNITS: i32 = -4;
