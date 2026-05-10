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

// Queries
struct ValidUnitIDQuery { int32_t unitID; };
struct ValidUnitIDResult { const Error* error; bool valid; };

struct GetAllUnitsQuery { uint8_t _unused; };
struct GetAllUnitsResult { const Error* error; int32_t* units; uint32_t count; };

struct GetTeamUnitsQuery { int32_t teamID; };
struct GetTeamUnitsResult { const Error* error; int32_t* units; uint32_t count; };

struct GetTeamUnitsSortedQuery { int32_t teamID; };
struct GetTeamUnitsSortedResult { const Error* error; int32_t* units; uint32_t count; };

struct GetTeamUnitsCountsQuery { int32_t teamID; };
struct GetTeamUnitsCountsResult { const Error* error; UnitDefCount* counts; uint32_t count; };

struct GetTeamUnitsByDefsQuery { int32_t teamID; const int32_t* unitDefIDs; uint32_t defCount; };
struct GetTeamUnitsByDefsResult { const Error* error; int32_t* units; uint32_t count; };

struct GetTeamUnitDefCountQuery { int32_t teamID; int32_t unitDefID; };
struct GetTeamUnitDefCountResult { const Error* error; uint32_t count; };

struct GetTeamUnitCountQuery { int32_t teamID; };
struct GetTeamUnitCountResult { const Error* error; uint32_t count; };

struct GetUnitsInRectangleQuery { RectangleQuery rect; UnitFilterParams filter; };
struct GetUnitsInRectangleResult { const Error* error; int32_t* units; uint32_t count; };

struct GetUnitsInBoxQuery { BoxQuery box; UnitFilterParams filter; };
struct GetUnitsInBoxResult { const Error* error; int32_t* units; uint32_t count; };

struct GetUnitsInPlanesQuery { PlanesQuery planes; UnitFilterParams filter; };
struct GetUnitsInPlanesResult { const Error* error; int32_t* units; uint32_t count; };

struct GetUnitsInSphereQuery { SphereQuery sphere; UnitFilterParams filter; };
struct GetUnitsInSphereResult { const Error* error; int32_t* units; uint32_t count; };

struct GetUnitsInCylinderQuery { CylinderQuery cylinder; UnitFilterParams filter; };
struct GetUnitsInCylinderResult { const Error* error; int32_t* units; uint32_t count; };

struct GetUnitArrayCentroidQuery { const int32_t* unitIDs; uint32_t count; };
struct GetUnitArrayCentroidResult { const Error* error; Float3 centroid; };

struct GetUnitMapCentroidQuery { const int32_t* unitIDs; uint32_t count; };
struct GetUnitMapCentroidResult { const Error* error; Float3 centroid; };

struct GetUnitNearestAllyQuery { Float3 pos; float radius; };
struct GetUnitNearestAllyResult { const Error* error; int32_t unitID; };

struct GetUnitNearestEnemyQuery { Float3 pos; float radius; };
struct GetUnitNearestEnemyResult { const Error* error; int32_t unitID; };

struct GetClosestEnemyUnitQuery {
	Float3 pos;
	float range;
	int32_t allyTeamID;
	bool useLOS;
	bool sphereDistTest;
	bool checkSightDist;
};
struct GetClosestEnemyUnitResult { const Error* error; int32_t unitID; };

struct GetUnitSeparationQuery { int32_t unitID1; int32_t unitID2; bool positional; bool checkMap; };
struct GetUnitSeparationResult { const Error* error; float separation; };

struct GetRenderUnitsQuery { uint8_t _unused; };
struct GetRenderUnitsResult { const Error* error; int32_t* units; uint32_t count; };

struct GetRenderUnitsDrawFlagChangedQuery { uint8_t _unused; };
struct GetRenderUnitsDrawFlagChangedResult { const Error* error; int32_t* units; uint32_t count; };

// API structure
struct UnitsQueryApi {
	void (*ValidUnitID)(const ValidUnitIDQuery* query, ValidUnitIDResult* result);
	void (*GetAllUnits)(const GetAllUnitsQuery* query, GetAllUnitsResult* result);
	void (*GetTeamUnits)(const GetTeamUnitsQuery* query, GetTeamUnitsResult* result);
	void (*GetTeamUnitsSorted)(const GetTeamUnitsSortedQuery* query, GetTeamUnitsSortedResult* result);
	void (*GetTeamUnitsCounts)(const GetTeamUnitsCountsQuery* query, GetTeamUnitsCountsResult* result);
	void (*GetTeamUnitsByDefs)(const GetTeamUnitsByDefsQuery* query, GetTeamUnitsByDefsResult* result);
	void (*GetTeamUnitDefCount)(const GetTeamUnitDefCountQuery* query, GetTeamUnitDefCountResult* result);
	void (*GetTeamUnitCount)(const GetTeamUnitCountQuery* query, GetTeamUnitCountResult* result);
	void (*GetUnitsInRectangle)(const GetUnitsInRectangleQuery* query, GetUnitsInRectangleResult* result);
	void (*GetUnitsInBox)(const GetUnitsInBoxQuery* query, GetUnitsInBoxResult* result);
	void (*GetUnitsInPlanes)(const GetUnitsInPlanesQuery* query, GetUnitsInPlanesResult* result);
	void (*GetUnitsInSphere)(const GetUnitsInSphereQuery* query, GetUnitsInSphereResult* result);
	void (*GetUnitsInCylinder)(const GetUnitsInCylinderQuery* query, GetUnitsInCylinderResult* result);
	void (*GetUnitArrayCentroid)(const GetUnitArrayCentroidQuery* query, GetUnitArrayCentroidResult* result);
	void (*GetUnitMapCentroid)(const GetUnitMapCentroidQuery* query, GetUnitMapCentroidResult* result);
	void (*GetUnitNearestAlly)(const GetUnitNearestAllyQuery* query, GetUnitNearestAllyResult* result);
	void (*GetUnitNearestEnemy)(const GetUnitNearestEnemyQuery* query, GetUnitNearestEnemyResult* result);
	void (*GetClosestEnemyUnit)(const GetClosestEnemyUnitQuery* query, GetClosestEnemyUnitResult* result);
	void (*GetUnitSeparation)(const GetUnitSeparationQuery* query, GetUnitSeparationResult* result);
	void (*GetRenderUnits)(const GetRenderUnitsQuery* query, GetRenderUnitsResult* result);
	void (*GetRenderUnitsDrawFlagChanged)(const GetRenderUnitsDrawFlagChangedQuery* query, GetRenderUnitsDrawFlagChangedResult* result);
};

extern const UnitsQueryApi UNITS_QUERY_API;

#ifdef __cplusplus
}
#endif
