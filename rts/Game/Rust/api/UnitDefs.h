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

struct UnitDefResult {
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

// Get unit def ID from name
struct UnitDefIDQuery {
	const char* unitDefName;
};

// API structure
struct UnitDefsApi {
	// Get all unit def IDs
	Int32Array (*GetUnitDefIDs)();

	// Get unit def count
	UInt32Result (*GetUnitDefCount)();

	// Get unit def by ID
	UnitDefResult (*GetUnitDefByID)(int32_t unitDefID);

	// Get unit def ID by name
	Int32Result (*GetUnitDefID)(const char* unitDefName);

	// Check if unit def is valid
	BoolResult (*ValidUnitDefID)(int32_t unitDefID);

	// Get specific property queries (for most common queries)
	StringResult (*GetUnitDefName)(int32_t unitDefID);
	StringResult (*GetUnitDefHumanName)(int32_t unitDefID);
	UnitDefCosts (*GetUnitDefCosts)(int32_t unitDefID);
	FloatResult (*GetUnitDefSpeed)(int32_t unitDefID);
	FloatResult (*GetUnitDefHealth)(int32_t unitDefID);

	// Custom params
	StringResult (*GetUnitDefCustomParam)(int32_t unitDefID, const char* key);
	StringArray (*GetUnitDefCustomParamKeys)(int32_t unitDefID);
};

extern const UnitDefsApi UNIT_DEFS_API;

#ifdef __cplusplus
}
#endif
