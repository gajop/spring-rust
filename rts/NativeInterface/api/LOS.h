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
	bool inLosOrRadar;
	bool inLos;
	bool inRadar;
	bool inJammer;
};

// Radar error params (position error in fog of war)
struct RadarErrorParams {
	float radarErrorSize;
	float baseRadarErrorSize;
	float baseRadarErrorMult;
};

// Queries
struct GetPositionLosStateQuery { Float3 pos; int32_t allyTeamID; };
struct GetPositionLosStateResult { const Error* error; PositionLosState state; };

struct IsPosInLosQuery { Float3 pos; int32_t allyTeamID; };
struct IsPosInLosResult { const Error* error; bool inLos; };

struct IsPosInRadarQuery { Float3 pos; int32_t allyTeamID; };
struct IsPosInRadarResult { const Error* error; bool inRadar; };

struct IsPosInAirLosQuery { Float3 pos; int32_t allyTeamID; };
struct IsPosInAirLosResult { const Error* error; bool inAirLos; };

struct IsUnitInLosQuery { int32_t unitID; int32_t allyTeamID; };
struct IsUnitInLosResult { const Error* error; bool inLos; };

struct IsUnitInAirLosQuery { int32_t unitID; int32_t allyTeamID; };
struct IsUnitInAirLosResult { const Error* error; bool inAirLos; };

struct IsUnitInRadarQuery { int32_t unitID; int32_t allyTeamID; };
struct IsUnitInRadarResult { const Error* error; bool inRadar; };

struct IsUnitInJammerQuery { int32_t unitID; int32_t allyTeamID; };
struct IsUnitInJammerResult { const Error* error; bool inJammer; };

struct GetRadarErrorParamsQuery { int32_t allyTeamID; };
struct GetRadarErrorParamsResult { const Error* error; RadarErrorParams params; };

struct GetClosestValidPositionQuery {
	int32_t unitDefID;
	float x;
	float z;
	float radius;
};

struct GetClosestValidPositionResult { const Error* error; Float3 position; };

// API structure
struct LOSApi {
	void (*GetPositionLosState)(const GetPositionLosStateQuery* query, GetPositionLosStateResult* result);
	void (*IsPosInLos)(const IsPosInLosQuery* query, IsPosInLosResult* result);
	void (*IsPosInRadar)(const IsPosInRadarQuery* query, IsPosInRadarResult* result);
	void (*IsPosInAirLos)(const IsPosInAirLosQuery* query, IsPosInAirLosResult* result);
	void (*IsUnitInLos)(const IsUnitInLosQuery* query, IsUnitInLosResult* result);
	void (*IsUnitInAirLos)(const IsUnitInAirLosQuery* query, IsUnitInAirLosResult* result);
	void (*IsUnitInRadar)(const IsUnitInRadarQuery* query, IsUnitInRadarResult* result);
	void (*IsUnitInJammer)(const IsUnitInJammerQuery* query, IsUnitInJammerResult* result);
	void (*GetRadarErrorParams)(const GetRadarErrorParamsQuery* query, GetRadarErrorParamsResult* result);
	void (*GetClosestValidPosition)(const GetClosestValidPositionQuery* query, GetClosestValidPositionResult* result);
};

extern const LOSApi LOS_API;

#ifdef __cplusplus
}
#endif
