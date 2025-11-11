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

// Move type data queries
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

struct MoveTypeDataResult {
	const Error* error;
	MoveTypeData data;
};

// Estimated path
struct PathWaypoint {
	Float3 pos;
	float eta;  // Estimated time of arrival
};

struct EstimatedPathResult {
	const Error* error;
	PathWaypoint* waypoints;
	uint32_t count;
};

// API structure (read-only queries)
struct MoveCtrlApi {
	// Get move type data
	MoveTypeDataResult (*GetUnitMoveTypeData)(int32_t unitID);

	// Get estimated path
	EstimatedPathResult (*GetUnitEstimatedPath)(int32_t unitID);
};

extern const MoveCtrlApi MOVE_CTRL_API;

#ifdef __cplusplus
}
#endif
