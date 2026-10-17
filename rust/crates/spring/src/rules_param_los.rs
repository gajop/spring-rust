/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

//! Visibility masks for rules parameters, shared by native and Wasm APIs.

/// Readable by the owning ally only. Lua's default.
pub const PRIVATE: i32 = 1;
/// Readable by the owning ally and its in-game allies.
pub const ALLIED: i32 = 2;
/// Readable while the unit is in line of sight.
pub const INLOS: i32 = 4;
/// Readable after identification while tracked by radar.
pub const TYPED: i32 = 8;
/// Readable while the unit is on radar.
pub const INRADAR: i32 = 16;
/// Readable by everyone, including spectators and unsynced code.
pub const PUBLIC: i32 = 32;
