#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Move Control API
// @see rts/Lua/LuaSyncedMoveCtrl.cpp
//
// Direct unit movement control (bypasses command queue)
// ============================================================================

// Move type data
struct MoveTypeData {
	const char* name;  // "ground", "air", "static", etc.

	// Shared properties
	float maxSpeed;
	float maxWantedSpeed;
	float goalX;
	float goalY;
	float goalZ;

	// Ground move type specific
	float turnRate;
	float accRate;
	float decRate;
	float maxReverseSpeed;
	float wantedSpeed;
	float currentSpeed;
	float deltaSpeed;

	// Air move type specific
	float maxBank;
	float maxPitch;
	float maxAileron;
	float maxElevator;
	float maxRudder;
};

// Path waypoint
struct PathWaypoint {
	Float3 pos;
	float eta;  // Estimated time of arrival
};

struct MoveCtrlQuery {
	int32_t unitID;
	bool enable;
};

struct MoveCtrlResult {
	const Error* error;
	bool success;
};

struct IsMoveCtrlEnabledQuery {
	int32_t unitID;
};

struct IsMoveCtrlEnabledResult {
	const Error* error;
	bool enabled;
};

struct SetMoveCtrlGravityQuery {
	int32_t unitID;
	float gravityFactor;
};

struct SetMoveCtrlGravityResult {
	const Error* error;
	bool success;
};

// Marks or unmarks the unit on the blocking-map without changing its blocking
// (collidable) state, matching Spring.MoveCtrl.SetNoBlocking. Script move types
// re-Block their owner on every update that moved it, so a unit meant to stay
// off the blocking map needs this in addition to Spring.SetUnitBlocking.
struct SetNoBlockingQuery {
	int32_t unitID;
	bool noBlocking;
};

struct SetNoBlockingResult {
	const Error* error;
	bool success;
};

// Typed equivalent of MoveCtrl.SetGroundMoveTypeData(unitID,
// {maxSpeed = value}). The value uses the Lua-facing world-units-per-second
// convention; the engine converts it to its per-simulation-frame value.
struct SetGroundMoveTypeMaxSpeedQuery {
	int32_t unitID;
	float maxSpeed;
};

struct SetGroundMoveTypeMaxSpeedResult {
	const Error* error;
	bool success;
};

// The Lua MoveCtrl.Set*MoveTypeData APIs accept a string key, but exposing
// that string directly to Wasm would make typos and cross-move-type mistakes
// runtime-only failures. These enums are the closed set of keys currently
// accepted by the corresponding Lua setters. The native implementation still
// delegates to AMoveType::SetMemberValue so aliases and per-type semantics
// remain identical to Lua.
enum MoveTypeNumericField {
	MOVE_TYPE_MAX_SPEED = 0,
	MOVE_TYPE_MAX_WANTED_SPEED,
	MOVE_TYPE_MANEUVER_LEASH,
	MOVE_TYPE_WATERLINE,

	MOVE_TYPE_GROUND_TURN_RATE,
	MOVE_TYPE_GROUND_TURN_ACCEL,
	MOVE_TYPE_GROUND_ACC_RATE,
	MOVE_TYPE_GROUND_DEC_RATE,
	MOVE_TYPE_GROUND_MY_GRAVITY,
	MOVE_TYPE_GROUND_MAX_REVERSE_DIST,
	MOVE_TYPE_GROUND_MIN_REVERSE_ANGLE,
	MOVE_TYPE_GROUND_MAX_REVERSE_SPEED,
	MOVE_TYPE_GROUND_SQ_SKID_SPEED_MULT,
	MOVE_TYPE_GROUND_MIN_SCRIPT_CHANGE_HEADING,

	MOVE_TYPE_GUNSHIP_WANTED_HEIGHT,
	MOVE_TYPE_GUNSHIP_ACC_RATE,
	MOVE_TYPE_GUNSHIP_DEC_RATE,
	MOVE_TYPE_GUNSHIP_TURN_RATE,
	MOVE_TYPE_GUNSHIP_ALTITUDE_RATE,
	MOVE_TYPE_GUNSHIP_CURRENT_BANK,
	MOVE_TYPE_GUNSHIP_CURRENT_PITCH,
	MOVE_TYPE_GUNSHIP_MAX_DRIFT,

	MOVE_TYPE_AIR_WANTED_HEIGHT,
	MOVE_TYPE_AIR_TURN_RADIUS,
	MOVE_TYPE_AIR_ACC_RATE,
	MOVE_TYPE_AIR_DEC_RATE,
	MOVE_TYPE_AIR_MAX_ACC,
	MOVE_TYPE_AIR_MAX_DEC,
	MOVE_TYPE_AIR_MAX_BANK,
	MOVE_TYPE_AIR_MAX_PITCH,
	MOVE_TYPE_AIR_MAX_AILERON,
	MOVE_TYPE_AIR_MAX_ELEVATOR,
	MOVE_TYPE_AIR_MAX_RUDDER,
	MOVE_TYPE_AIR_ATTACK_SAFETY_DISTANCE,
	MOVE_TYPE_AIR_MY_GRAVITY,
	MOVE_TYPE_AIR_MANEUVER_BLOCK_TIME,
};

enum MoveTypeBooleanField {
	MOVE_TYPE_USE_WANTED_SPEED_INDIVIDUAL = 0,
	MOVE_TYPE_USE_WANTED_SPEED_FORMATION,

	MOVE_TYPE_GROUND_AT_GOAL,
	MOVE_TYPE_GROUND_AT_END_OF_PATH,
	MOVE_TYPE_GROUND_PUSH_RESISTANT,

	MOVE_TYPE_GUNSHIP_COLLIDE,
	MOVE_TYPE_GUNSHIP_DONT_LAND,
	MOVE_TYPE_GUNSHIP_AIR_STRAFE,
	MOVE_TYPE_GUNSHIP_USE_SMOOTH_MESH,
	MOVE_TYPE_GUNSHIP_BANKING_ALLOWED,

	MOVE_TYPE_AIR_COLLIDE,
	MOVE_TYPE_AIR_USE_SMOOTH_MESH,
	MOVE_TYPE_AIR_LOOPBACK_ATTACK,
};

struct SetMoveTypeNumericQuery {
	int32_t unitID;
	MoveTypeNumericField field;
	float value;
};

struct SetMoveTypeNumericResult {
	const Error* error;
	bool success;
};

struct SetMoveTypeBooleanQuery {
	int32_t unitID;
	MoveTypeBooleanField field;
	bool value;
};

struct SetMoveTypeBooleanResult {
	const Error* error;
	bool success;
};

// Queries
struct GetUnitMoveTypeDataQuery {
	int32_t unitID;
};

struct GetUnitMoveTypeDataResult {
	const Error* error;
	MoveTypeData data;
};

struct GetUnitEstimatedPathQuery {
	int32_t unitID;
};

struct GetUnitEstimatedPathResult {
	const Error* error;
	PathWaypoint* waypoints;
	uint32_t count;
	int32_t* starts;
	uint32_t startCount;
};

// API structure
struct MoveCtrlApi {
	void (*GetUnitMoveTypeData)(
		const GetUnitMoveTypeDataQuery* query,
		GetUnitMoveTypeDataResult* result
	);

	void (*GetUnitEstimatedPath)(
		const GetUnitEstimatedPathQuery* query,
		GetUnitEstimatedPathResult* result
	);

	void (*MoveCtrl)(
		const MoveCtrlQuery* query,
		MoveCtrlResult* result
	);

	void (*IsMoveCtrlEnabled)(
		const IsMoveCtrlEnabledQuery* query,
		IsMoveCtrlEnabledResult* result
	);

	void (*SetMoveCtrlGravity)(
		const SetMoveCtrlGravityQuery* query,
		SetMoveCtrlGravityResult* result
	);

	void (*SetGroundMoveTypeMaxSpeed)(
		const SetGroundMoveTypeMaxSpeedQuery* query,
		SetGroundMoveTypeMaxSpeedResult* result
	);

	void (*SetMoveTypeNumeric)(
		const SetMoveTypeNumericQuery* query,
		SetMoveTypeNumericResult* result
	);

	void (*SetMoveTypeBoolean)(
		const SetMoveTypeBooleanQuery* query,
		SetMoveTypeBooleanResult* result
	);

	void (*SetNoBlocking)(
		const SetNoBlockingQuery* query,
		SetNoBlockingResult* result
	);
};

extern const MoveCtrlApi MOVE_CTRL_API;

#ifdef __cplusplus
}
#endif
