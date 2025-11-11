#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Units Query API
// @see rts/Lua/LuaSyncedRead.cpp
//
// Unit lists, spatial queries, and basic unit existence checks
// ============================================================================

// Spatial query parameters
struct RectangleQuery {
	float minX;
	float minZ;
	float maxX;
	float maxZ;
};

struct BoxQuery {
	Float3 min;
	Float3 max;
};

struct SphereQuery {
	Float3 center;
	float radius;
};

struct CylinderQuery {
	Float3 center;
	float radius;
	float height;
};

struct PlanesQuery {
	// 6 planes defining a frustum (each plane is ax + by + cz + d = 0)
	Float4 planes[6];
	uint32_t planeCount;
};

// Unit filter for queries
enum UnitFilter {
	UNIT_FILTER_ALL = 0,
	UNIT_FILTER_MY_UNITS = 1,
	UNIT_FILTER_ALLY_UNITS = 2,
	UNIT_FILTER_ENEMY_UNITS = 3,
	UNIT_FILTER_TEAM = 4,  // Specific team
	UNIT_FILTER_ALLYTEAM = 5,  // Specific allyteam
};

struct UnitFilterParams {
	UnitFilter filter;
	int32_t teamID;      // For TEAM filter
	int32_t allyTeamID;  // For ALLYTEAM filter
};

// Unit counts by def
struct UnitDefCount {
	int32_t unitDefID;
	uint32_t count;
};

struct UnitDefCountsResult {
	const Error* error;
	UnitDefCount* counts;
	uint32_t countCount;
};

// Nearest unit queries
struct NearestUnitQuery {
	Float3 pos;
	float radius;
	bool spherical;  // true for sphere, false for cylinder
};

// Separation query
struct SeparationQuery {
	int32_t unitID1;
	int32_t unitID2;
	bool positional;  // true for positional, false for collision volume
	bool checkMap;    // Include terrain height
};

// API structure
struct UnitsQueryApi {
	// Validation
	BoolResult (*ValidUnitID)(int32_t unitID);

	// Get all units
	Int32Array (*GetAllUnits)();

	// Get units by team
	Int32Array (*GetTeamUnits)(int32_t teamID);
	Int32Array (*GetTeamUnitsSorted)(int32_t teamID);  // Sorted by unitDefID
	UnitDefCountsResult (*GetTeamUnitsCounts)(int32_t teamID);
	Int32Array (*GetTeamUnitsByDefs)(int32_t teamID, const int32_t* unitDefIDs, uint32_t count);
	UInt32Result (*GetTeamUnitDefCount)(int32_t teamID, int32_t unitDefID);
	UInt32Result (*GetTeamUnitCount)(int32_t teamID);

	// Spatial queries
	Int32Array (*GetUnitsInRectangle)(RectangleQuery query, UnitFilterParams filter);
	Int32Array (*GetUnitsInBox)(BoxQuery query, UnitFilterParams filter);
	Int32Array (*GetUnitsInPlanes)(PlanesQuery query, UnitFilterParams filter);
	Int32Array (*GetUnitsInSphere)(SphereQuery query, UnitFilterParams filter);
	Int32Array (*GetUnitsInCylinder)(CylinderQuery query, UnitFilterParams filter);

	// Centroid calculations
	Float3Result (*GetUnitArrayCentroid)(const int32_t* unitIDs, uint32_t count);
	Float3Result (*GetUnitMapCentroid)(const int32_t* unitIDs, uint32_t count);  // Map of unitID->true

	// Nearest unit
	Int32Result (*GetUnitNearestAlly)(Float3 pos, float radius);
	Int32Result (*GetUnitNearestEnemy)(Float3 pos, float radius);

	// Separation
	FloatResult (*GetUnitSeparation)(int32_t unitID1, int32_t unitID2, bool positional, bool checkMap);
};

extern const UnitsQueryApi UNITS_QUERY_API;

#ifdef __cplusplus
}
#endif
