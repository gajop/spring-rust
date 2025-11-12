#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// PathFinder API
// @see rts/Lua/LuaPathFinder.cpp
// ============================================================================

// Queries
struct RequestPathQuery {
	uint32_t moveDefID;
	const char* moveDefName;  // If non-null, use name instead of ID
	Float3 startPos;
	Float3 endPos;
	float radius;
};

struct RequestPathResult {
	const Error* error;
	uint32_t pathID;  // 0 if path request failed
};

struct DeletePathQuery { uint32_t pathID; };
struct DeletePathResult { const Error* error; bool success; };

struct GetPathWayPointsQuery { uint32_t pathID; };
struct GetPathWayPointsResult {
	const Error* error;
	Float3* points;
	uint32_t pointCount;
	int32_t* starts;  // Starting indices for each segment
	uint32_t startCount;
};

struct GetNextWayPointQuery {
	uint32_t pathID;
	Float3 callerPos;
	float minDist;
};

struct GetNextWayPointResult {
	const Error* error;
	Float3 waypoint;
	bool hasWaypoint;  // false if path complete or invalid
};

struct InitPathNodeCostsArrayQuery {
	uint32_t overlayIndex;
	uint32_t sizeX;
	uint32_t sizeZ;
};

struct InitPathNodeCostsArrayResult { const Error* error; bool success; };

struct FreePathNodeCostsArrayQuery { uint32_t overlayIndex; };
struct FreePathNodeCostsArrayResult { const Error* error; bool success; };

struct SetPathNodeCostsQuery { uint32_t overlayIndex; };
struct SetPathNodeCostsResult { const Error* error; bool success; };

struct GetPathNodeCostsQuery { uint32_t overlayIndex; };
struct GetPathNodeCostsResult { const Error* error; float* costs; uint32_t count; };

struct SetPathNodeCostQuery {
	uint32_t overlayIndex;
	uint32_t costIndex;
	float cost;
};

struct SetPathNodeCostResult { const Error* error; bool success; };

struct GetPathNodeCostQuery { uint32_t x; uint32_t z; };
struct GetPathNodeCostResult { const Error* error; float cost; };

// API structure
struct PathFinderApi {
	void (*RequestPath)(const RequestPathQuery* query, RequestPathResult* result);
	void (*DeletePath)(const DeletePathQuery* query, DeletePathResult* result);
	void (*GetPathWayPoints)(const GetPathWayPointsQuery* query, GetPathWayPointsResult* result);
	void (*GetNextWayPoint)(const GetNextWayPointQuery* query, GetNextWayPointResult* result);
	void (*InitPathNodeCostsArray)(const InitPathNodeCostsArrayQuery* query, InitPathNodeCostsArrayResult* result);
	void (*FreePathNodeCostsArray)(const FreePathNodeCostsArrayQuery* query, FreePathNodeCostsArrayResult* result);
	void (*SetPathNodeCosts)(const SetPathNodeCostsQuery* query, SetPathNodeCostsResult* result);
	void (*GetPathNodeCosts)(const GetPathNodeCostsQuery* query, GetPathNodeCostsResult* result);
	void (*SetPathNodeCost)(const SetPathNodeCostQuery* query, SetPathNodeCostResult* result);
	void (*GetPathNodeCost)(const GetPathNodeCostQuery* query, GetPathNodeCostResult* result);
};

extern const PathFinderApi PATH_FINDER_API;

#ifdef __cplusplus
}
#endif
