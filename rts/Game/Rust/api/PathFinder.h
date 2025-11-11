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

// Path request and management
struct PathRequest {
	// MoveDef identifier (either pathType index or move def name)
	uint32_t moveDefID;
	const char* moveDefName;  // If non-null, use name instead of ID

	// Start and end positions
	Float3 startPos;
	Float3 endPos;

	// Path radius
	float radius;
};

struct PathResult {
	const Error* error;
	uint32_t pathID;  // 0 if path request failed
};

// Path waypoint query
struct PathWayPointsResult {
	const Error* error;
	Float3Array points;
	Int32Array starts;  // Starting indices for each segment
};

// Get next waypoint from current position
struct NextWayPointRequest {
	uint32_t pathID;
	Float3 callerPos;
	float minDist;
};

struct NextWayPointResult {
	const Error* error;
	Float3 waypoint;
	bool hasWaypoint;  // false if path complete or invalid
};

// Node cost overlay management
struct NodeCostOverlayInit {
	uint32_t overlayIndex;
	uint32_t sizeX;
	uint32_t sizeZ;
};

struct NodeCostOverlaySet {
	uint32_t overlayIndex;
};

struct NodeCostSet {
	uint32_t overlayIndex;
	uint32_t costIndex;
	float cost;
};

struct NodeCostGet {
	uint32_t x;
	uint32_t z;
};

// API structure
struct PathFinderApi {
	// Request a path from start to end
	PathResult (*RequestPath)(PathRequest request);

	// Delete a path (called when path handle is no longer needed)
	BoolResult (*DeletePath)(uint32_t pathID);

	// Get all waypoints for a path
	PathWayPointsResult (*GetPathWayPoints)(uint32_t pathID);

	// Get next waypoint from current position
	NextWayPointResult (*GetNextWayPoint)(NextWayPointRequest request);

	// Initialize a node cost overlay array
	BoolResult (*InitPathNodeCostsArray)(NodeCostOverlayInit init);

	// Free a node cost overlay array
	BoolResult (*FreePathNodeCostsArray)(uint32_t overlayIndex);

	// Set the active node cost overlay
	BoolResult (*SetPathNodeCosts)(NodeCostOverlaySet set);

	// Get all costs from an overlay
	FloatArray (*GetPathNodeCosts)(uint32_t overlayIndex);

	// Set a specific node cost in an overlay
	BoolResult (*SetPathNodeCost)(NodeCostSet set);

	// Get the cost of a specific node from the active overlay
	FloatResult (*GetPathNodeCost)(NodeCostGet get);
};

extern const PathFinderApi PATH_FINDER_API;

#ifdef __cplusplus
}
#endif
