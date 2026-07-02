#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Unit Definitions API
// @see rts/Lua/LuaUnitDefs.cpp
//
// Static unit definition data (read-only game data from unit files)
// ============================================================================

// Note: This is a simplified API. Full UnitDef has 200+ properties.
// Extend as needed for specific use cases.

struct UnitDefBasicInfo {
	int32_t id;
	const char* name;
	const char* humanName;
	const char* tooltip;
	int32_t unitDefID;
};

struct UnitDefCosts {
	float metalCost;
	float energyCost;
	float buildTime;
};

struct UnitDefPhysics {
	float mass;
	float height;
	float radius;
	float speed;
	float turnRate;
	float acceleration;
	float brakeRate;
	bool canFly;
	bool canMove;
	bool canHover;
	bool floatOnWater;
	int32_t moveDefID;
};

struct UnitDefWeapons {
	int32_t* weaponDefIDs;
	uint32_t weaponCount;
};

struct UnitDefBuildOptions {
	int32_t* buildableUnitDefIDs;
	uint32_t buildableCount;
};

struct UnitDefSensors {
	float losRadius;
	float airLosRadius;
	float radarRadius;
	float sonarRadius;
	float seismicRadius;
	float radarJammerRadius;
	float sonarJammerRadius;
};

struct UnitDefHealth {
	float health;
	float autoHeal;
	float idleAutoHeal;
	int32_t idleTime;
};

// Queries
struct GetUnitDefIDsQuery { uint8_t _unused; };
struct GetUnitDefIDsResult { const Error* error; int32_t* ids; uint32_t count; };

struct GetUnitDefCountQuery { uint8_t _unused; };
struct GetUnitDefCountResult { const Error* error; uint32_t count; };

struct GetUnitDefByIDQuery { int32_t unitDefID; };
struct GetUnitDefByIDResult {
	const Error* error;
	bool exists;
	UnitDefBasicInfo basic;
	UnitDefCosts costs;
	UnitDefPhysics physics;
	UnitDefWeapons weapons;
	UnitDefBuildOptions buildOptions;
	UnitDefSensors sensors;
	UnitDefHealth health;
};

struct GetUnitDefIDByNameQuery { const char* unitDefName; };
struct GetUnitDefIDByNameResult { const Error* error; int32_t id; };

struct ValidUnitDefIDQuery { int32_t unitDefID; };
struct ValidUnitDefIDResult { const Error* error; bool valid; };

struct GetUnitDefNameQuery { int32_t unitDefID; };
struct GetUnitDefNameResult { const Error* error; const char* name; };

struct GetUnitDefHumanNameQuery { int32_t unitDefID; };
struct GetUnitDefHumanNameResult { const Error* error; const char* humanName; };

struct GetUnitDefCostsQuery { int32_t unitDefID; };
struct GetUnitDefCostsResult { const Error* error; UnitDefCosts costs; };

struct GetUnitDefSpeedQuery { int32_t unitDefID; };
struct GetUnitDefSpeedResult { const Error* error; float speed; };

struct GetUnitDefHealthQuery { int32_t unitDefID; };
struct GetUnitDefHealthResult { const Error* error; float health; };

struct GetUnitDefCustomParamQuery { int32_t unitDefID; const char* key; };
struct GetUnitDefCustomParamResult { const Error* error; const char* value; };

struct GetUnitDefCustomParamKeysQuery { int32_t unitDefID; };
struct GetUnitDefCustomParamKeysResult { const Error* error; const char** keys; uint32_t count; };

// API structure
struct UnitDefsApi {
	void (*GetUnitDefIDs)(const GetUnitDefIDsQuery* query, GetUnitDefIDsResult* result);
	void (*GetUnitDefCount)(const GetUnitDefCountQuery* query, GetUnitDefCountResult* result);
	void (*GetUnitDefByID)(const GetUnitDefByIDQuery* query, GetUnitDefByIDResult* result);
	void (*GetUnitDefIDByName)(const GetUnitDefIDByNameQuery* query, GetUnitDefIDByNameResult* result);
	void (*ValidUnitDefID)(const ValidUnitDefIDQuery* query, ValidUnitDefIDResult* result);
	void (*GetUnitDefName)(const GetUnitDefNameQuery* query, GetUnitDefNameResult* result);
	void (*GetUnitDefHumanName)(const GetUnitDefHumanNameQuery* query, GetUnitDefHumanNameResult* result);
	void (*GetUnitDefCosts)(const GetUnitDefCostsQuery* query, GetUnitDefCostsResult* result);
	void (*GetUnitDefSpeed)(const GetUnitDefSpeedQuery* query, GetUnitDefSpeedResult* result);
	void (*GetUnitDefHealth)(const GetUnitDefHealthQuery* query, GetUnitDefHealthResult* result);
	void (*GetUnitDefCustomParam)(const GetUnitDefCustomParamQuery* query, GetUnitDefCustomParamResult* result);
	void (*GetUnitDefCustomParamKeys)(const GetUnitDefCustomParamKeysQuery* query, GetUnitDefCustomParamKeysResult* result);
};

extern const UnitDefsApi UNIT_DEFS_API;

#ifdef __cplusplus
}
#endif
