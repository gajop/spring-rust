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

// Queries
struct GetFeatureDefIDsQuery { uint8_t _unused; };
struct GetFeatureDefIDsResult { const Error* error; int32_t* ids; uint32_t count; };

struct GetFeatureDefCountQuery { uint8_t _unused; };
struct GetFeatureDefCountResult { const Error* error; uint32_t count; };

struct GetFeatureDefByIDQuery { int32_t featureDefID; };
struct GetFeatureDefByIDResult { const Error* error; FeatureDefInfo info; bool exists; };

struct GetFeatureDefIDByNameQuery { const char* featureDefName; };
struct GetFeatureDefIDByNameResult { const Error* error; int32_t id; };

struct ValidFeatureDefIDQuery { int32_t featureDefID; };
struct ValidFeatureDefIDResult { const Error* error; bool valid; };

struct GetFeatureDefNameQuery { int32_t featureDefID; };
struct GetFeatureDefNameResult { const Error* error; const char* name; };

struct GetFeatureDefMetalQuery { int32_t featureDefID; };
struct GetFeatureDefMetalResult { const Error* error; float metal; };

struct GetFeatureDefEnergyQuery { int32_t featureDefID; };
struct GetFeatureDefEnergyResult { const Error* error; float energy; };

struct GetFeatureDefCustomParamQuery { int32_t featureDefID; const char* key; };
struct GetFeatureDefCustomParamResult { const Error* error; const char* value; };

struct GetFeatureDefCustomParamKeysQuery { int32_t featureDefID; };
struct GetFeatureDefCustomParamKeysResult { const Error* error; const char** keys; uint32_t count; };

// API structure
struct FeatureDefsApi {
	void (*GetFeatureDefIDs)(const GetFeatureDefIDsQuery* query, GetFeatureDefIDsResult* result);
	void (*GetFeatureDefCount)(const GetFeatureDefCountQuery* query, GetFeatureDefCountResult* result);
	void (*GetFeatureDefByID)(const GetFeatureDefByIDQuery* query, GetFeatureDefByIDResult* result);
	void (*GetFeatureDefIDByName)(const GetFeatureDefIDByNameQuery* query, GetFeatureDefIDByNameResult* result);
	void (*ValidFeatureDefID)(const ValidFeatureDefIDQuery* query, ValidFeatureDefIDResult* result);
	void (*GetFeatureDefName)(const GetFeatureDefNameQuery* query, GetFeatureDefNameResult* result);
	void (*GetFeatureDefMetal)(const GetFeatureDefMetalQuery* query, GetFeatureDefMetalResult* result);
	void (*GetFeatureDefEnergy)(const GetFeatureDefEnergyQuery* query, GetFeatureDefEnergyResult* result);
	void (*GetFeatureDefCustomParam)(const GetFeatureDefCustomParamQuery* query, GetFeatureDefCustomParamResult* result);
	void (*GetFeatureDefCustomParamKeys)(const GetFeatureDefCustomParamKeysQuery* query, GetFeatureDefCustomParamKeysResult* result);
};

extern const FeatureDefsApi FEATURE_DEFS_API;

#ifdef __cplusplus
}
#endif
