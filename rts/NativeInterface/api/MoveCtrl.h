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
};

extern const MoveCtrlApi MOVE_CTRL_API;

#ifdef __cplusplus
}
#endif
