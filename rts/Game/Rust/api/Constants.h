#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Constants API
// @see rts/Lua/LuaConst*.cpp
//
// Game, engine, and command constants
// ============================================================================

// Command IDs (subset of most common)
// Note: When included in C++ with Command.h, these are already defined as constexpr
#ifndef COMMAND_H
enum CommandID {
	CMD_STOP = 0,
	CMD_WAIT = 5,
	CMD_MOVE = 10,
	CMD_PATROL = 15,
	CMD_FIGHT = 16,
	CMD_ATTACK = 20,
	CMD_AREA_ATTACK = 21,
	CMD_GUARD = 25,
	CMD_AISELECT = 30,
	CMD_GROUPSELECT = 35,
	CMD_GROUPADD = 36,
	CMD_GROUPCLEAR = 37,
	CMD_REPAIR = 40,
	CMD_FIRE_STATE = 45,
	CMD_MOVE_STATE = 50,
	CMD_SETBASE = 55,
	CMD_INTERNAL = 60,
	CMD_SELFD = 65,
	CMD_SET_WANTED_MAX_SPEED = 70,
	CMD_LOAD_UNITS = 75,
	CMD_LOAD_ONTO = 76,
	CMD_UNLOAD_UNITS = 80,
	CMD_UNLOAD_UNIT = 81,
	CMD_ONOFF = 85,
	CMD_RECLAIM = 90,
	CMD_CLOAK = 95,
	CMD_STOCKPILE = 100,
	CMD_MANUALFIRE = 105,
	CMD_RESTORE = 110,
	CMD_REPEAT = 115,
	CMD_TRAJECTORY = 120,
	CMD_RESURRECT = 125,
	CMD_CAPTURE = 130,
	CMD_AUTOREPAIRLEVEL = 135,
	CMD_LOOPBACKATTACK = 140,
	CMD_IDLEMODE = 145,
};

// Command options (bitfield)
enum CommandOption {
	CMD_OPT_INTERNAL = (1 << 0),
	CMD_OPT_RIGHT = (1 << 1),
	CMD_OPT_SHIFT = (1 << 2),
	CMD_OPT_CTRL = (1 << 3),
	CMD_OPT_ALT = (1 << 4),
	CMD_OPT_META = (1 << 5),
};

// Fire state
enum FireState {
	FIRESTATE_HOLDFIRE = 0,
	FIRESTATE_RETURNFIRE = 1,
	FIRESTATE_FIREATWILL = 2,
	FIRESTATE_FIREATNEUTRAL = 3,
};

// Move state
enum MoveState {
	MOVESTATE_HOLDPOS = 0,
	MOVESTATE_MANEUVER = 1,
	MOVESTATE_ROAM = 2,
};
#endif // COMMAND_H

// Unit categories (bitfield)
enum UnitCategory {
	CAT_LAND = (1 << 0),
	CAT_SHIP = (1 << 1),
	CAT_HOVER = (1 << 2),
	CAT_AMPHIB = (1 << 3),
	CAT_AIR = (1 << 4),
	CAT_BUILDER = (1 << 5),
	CAT_FACTORY = (1 << 6),
	CAT_WEAPON = (1 << 7),
	CAT_MOBILEUNIT = (1 << 8),
	CAT_NOTAIR = (1 << 9),
};

// Damage types
enum DamageType {
	DAMAGE_EXPLOSION_GENERATOR = -1,
	DAMAGE_COLLISION_OBJECT = -2,
	DAMAGE_COLLISION_GROUND = -3,
	DAMAGE_KILLED = -4,
	DAMAGE_RECLAIMED = -5,
	DAMAGE_SELFD = -6,
	DAMAGE_CRUSHED = -7,
	DAMAGE_FIRE = -8,
};

// COB constants
enum COBConstant {
	COB_ACTIVATION = 1,
	COB_STANDINGMOVEORDERS = 2,
	COB_STANDINGFIREORDERS = 3,
	COB_HEALTH = 4,
	COB_INBUILDSTANCE = 5,
	COB_BUSY = 6,
	COB_PIECE_XZ = 7,
	COB_PIECE_Y = 8,
	COB_UNIT_XZ = 9,
	COB_UNIT_Y = 10,
	COB_UNIT_HEIGHT = 11,
	COB_XZ_ATAN = 12,
	COB_XZ_HYPOT = 13,
	COB_GROUND_HEIGHT = 14,
	COB_BUILD_PERCENT_LEFT = 15,
};

#ifdef __cplusplus
}
#endif
