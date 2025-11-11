#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Feature Definitions API
// @see rts/Lua/LuaFeatureDefs.cpp
//
// Static feature definition data (rocks, wrecks, trees, etc.)
// ============================================================================

struct FeatureDefInfo {
	int32_t id;
	const char* name;
	const char* description;
	const char* tooltip;
	float metal;
	float energy;
	float maxHealth;
	float reclaimTime;
	float mass;
	bool destructable;
	bool reclaimable;
	bool blocking;
	bool burnable;
	bool floating;
	bool geoThermal;
	const char* modelName;
	const char* resurrectAs;  // Unit def name to resurrect as
};

struct FeatureDefResult {
	const Error* error;
	FeatureDefInfo info;
	bool exists;
};

// API structure
struct FeatureDefsApi {
	// Get all feature def IDs
	Int32Array (*GetFeatureDefIDs)();

	// Get feature def count
	UInt32Result (*GetFeatureDefCount)();

	// Get feature def by ID
	FeatureDefResult (*GetFeatureDefByID)(int32_t featureDefID);

	// Get feature def ID by name
	Int32Result (*GetFeatureDefID)(const char* featureDefName);

	// Check if feature def is valid
	BoolResult (*ValidFeatureDefID)(int32_t featureDefID);

	// Quick property accessors
	StringResult (*GetFeatureDefName)(int32_t featureDefID);
	FloatResult (*GetFeatureDefMetal)(int32_t featureDefID);
	FloatResult (*GetFeatureDefEnergy)(int32_t featureDefID);

	// Custom params
	StringResult (*GetFeatureDefCustomParam)(int32_t featureDefID, const char* key);
	StringArray (*GetFeatureDefCustomParamKeys)(int32_t featureDefID);
};

extern const FeatureDefsApi FEATURE_DEFS_API;

#ifdef __cplusplus
}
#endif
