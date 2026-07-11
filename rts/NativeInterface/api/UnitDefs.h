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

// Note: This is a partial API. Full UnitDef has 200+ properties.
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
	// Where the unit can exist. Lua exposes all four (LuaUnitDefs.cpp); without
	// them a caller cannot tell a ground unit from an amphibious or naval one.
	bool canSubmerge;
	float waterline;
	float minWaterDepth;
	float maxWaterDepth;
};

// The unit's classification. These are computed by UnitDef, not stored, and Lua
// exposes them as functions (ADD_FUNCTION("isBuilding", ...) and friends), so
// they cannot be read from the fields above.
struct UnitDefClassify {
	bool isTransport;
	bool isImmobile;
	bool isBuilding;
	bool isBuilder;
	bool isMobileBuilder;
	bool isStaticBuilder;
	bool isFactory;
	bool isExtractor;
	bool isGroundUnit;
	bool isAirUnit;
	bool isStrafingAirUnit;
	bool isHoveringAirUnit;
	bool isFighterAirUnit;
	bool isBomberAirUnit;
};

// ----------------------------------------------------------------------------
// Every UnitDef property, by name
//
// The structs above are a hand-picked subset, which is how callers kept ending
// up short. These read the *same reflection table Lua reads*
// (LuaUnitDefs::GetParamMap), so every property `UnitDefs[id].foo` has in Lua is
// reachable here as GetUnitDefParam*(id, "foo") -- and a property added to the
// engine shows up in both at once, with no list to keep in sync.
//
// Scalars only. A handful of Lua's entries are tables (model, weapons, sounds,
// collisionVolume, customParams, buildOptions); those have dedicated calls
// above, and GetUnitDefParamType reports them as UNIT_DEF_PARAM_TABLE.
// ----------------------------------------------------------------------------

enum UnitDefParamType {
	UNIT_DEF_PARAM_MISSING = 0,
	UNIT_DEF_PARAM_INT = 1,
	UNIT_DEF_PARAM_BOOL = 2,
	UNIT_DEF_PARAM_FLOAT = 3,
	UNIT_DEF_PARAM_STRING = 4,
	UNIT_DEF_PARAM_TABLE = 5,
};

struct UnitDefParamKey { const char* name; int32_t type; };

// Every property name the engine knows, with its type.
struct GetUnitDefParamKeysQuery { uint8_t _unused; };
struct GetUnitDefParamKeysResult { const Error* error; UnitDefParamKey* keys; uint32_t count; };

struct GetUnitDefParamTypeQuery { const char* key; };
struct GetUnitDefParamTypeResult { const Error* error; int32_t type; };

struct GetUnitDefParamBoolQuery { int32_t unitDefID; const char* key; };
struct GetUnitDefParamBoolResult { const Error* error; bool value; };

struct GetUnitDefParamIntQuery { int32_t unitDefID; const char* key; };
struct GetUnitDefParamIntResult { const Error* error; int32_t value; };

struct GetUnitDefParamFloatQuery { int32_t unitDefID; const char* key; };
struct GetUnitDefParamFloatResult { const Error* error; float value; };

struct GetUnitDefParamStringQuery { int32_t unitDefID; const char* key; };
struct GetUnitDefParamStringResult { const Error* error; const char* value; };

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
	UnitDefClassify classify;
};

struct GetUnitDefClassifyQuery { int32_t unitDefID; };
struct GetUnitDefClassifyResult { const Error* error; UnitDefClassify classify; };

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
	void (*GetUnitDefClassify)(const GetUnitDefClassifyQuery* query, GetUnitDefClassifyResult* result);
	void (*GetUnitDefParamKeys)(const GetUnitDefParamKeysQuery* query, GetUnitDefParamKeysResult* result);
	void (*GetUnitDefParamType)(const GetUnitDefParamTypeQuery* query, GetUnitDefParamTypeResult* result);
	void (*GetUnitDefParamBool)(const GetUnitDefParamBoolQuery* query, GetUnitDefParamBoolResult* result);
	void (*GetUnitDefParamInt)(const GetUnitDefParamIntQuery* query, GetUnitDefParamIntResult* result);
	void (*GetUnitDefParamFloat)(const GetUnitDefParamFloatQuery* query, GetUnitDefParamFloatResult* result);
	void (*GetUnitDefParamString)(const GetUnitDefParamStringQuery* query, GetUnitDefParamStringResult* result);
};

extern const UnitDefsApi UNIT_DEFS_API;

#ifdef __cplusplus
}
#endif
