#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Line of Sight (LOS) API
// @see rts/Lua/LuaSyncedRead.cpp
//
// LOS, radar, and visibility queries
// ============================================================================

// Position LOS state
struct PositionLosState {
	bool los;        // In line of sight
	bool radar;      // In radar
	bool prevLos;    // Was in LOS last check
};

struct PositionLosStateResult {
	const Error* error;
	PositionLosState state;
};

// Radar error params (position error in fog of war)
struct RadarErrorParams {
	float baseErrMult;
	float baseErrSize;
	float errorMult;
	float errorSize;
	float baseSpeed;
	float speedMult;
};

struct RadarErrorParamsResult {
	const Error* error;
	RadarErrorParams params;
};

// Closest valid position (for unit placement)
struct ClosestValidPositionQuery {
	Float3 pos;
	float radius;
	int32_t unitDefID;
	int32_t teamID;
};

// API structure
struct LOSApi {
	// Position-based LOS
	PositionLosStateResult (*GetPositionLosState)(Float3 pos, int32_t allyTeamID);
	BoolResult (*IsPosInLos)(Float3 pos, int32_t allyTeamID);
	BoolResult (*IsPosInRadar)(Float3 pos, int32_t allyTeamID);
	BoolResult (*IsPosInAirLos)(Float3 pos, int32_t allyTeamID);

	// Unit-based LOS
	BoolResult (*IsUnitInLos)(int32_t unitID, int32_t allyTeamID);
	BoolResult (*IsUnitInAirLos)(int32_t unitID, int32_t allyTeamID);
	BoolResult (*IsUnitInRadar)(int32_t unitID, int32_t allyTeamID);
	BoolResult (*IsUnitInJammer)(int32_t unitID, int32_t allyTeamID);

	// Radar error
	RadarErrorParamsResult (*GetRadarErrorParams)(int32_t allyTeamID);

	// Closest valid position (for placement)
	Float3Result (*GetClosestValidPosition)(ClosestValidPositionQuery query);
};

extern const LOSApi LOS_API;

#ifdef __cplusplus
}
#endif
