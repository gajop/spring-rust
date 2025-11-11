#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// RulesParams API
// @see rts/Lua/LuaRulesParams.h
//
// Custom parameters that can be attached to game objects, teams, and players.
// These params have LOS-based visibility rules.
// ============================================================================

// LOS visibility masks for rules params
enum RulesParamLOS {
	RULESPARAMLOS_PRIVATE = 1,   // only readable by the ally (default)
	RULESPARAMLOS_ALLIED  = 2,   // readable for ally + ingame allied
	RULESPARAMLOS_INLOS   = 4,   // readable if the unit is in LOS
	RULESPARAMLOS_TYPED   = 8,   // readable if the unit is typed (in radar and was once in LOS)
	RULESPARAMLOS_INRADAR = 16,  // readable if the unit is in radar
	RULESPARAMLOS_PUBLIC  = 32,  // readable for all

	// Convenience masks (include all states beneath them)
	RULESPARAMLOS_PRIVATE_MASK = 63,  // All flags
	RULESPARAMLOS_ALLIED_MASK  = 62,  // PUBLIC | INRADAR | TYPED | INLOS | ALLIED
	RULESPARAMLOS_INLOS_MASK   = 60,  // PUBLIC | INRADAR | TYPED | INLOS
	RULESPARAMLOS_TYPED_MASK   = 56,  // PUBLIC | INRADAR | TYPED
	RULESPARAMLOS_INRADAR_MASK = 48,  // PUBLIC | INRADAR
	RULESPARAMLOS_PUBLIC_MASK  = 32   // PUBLIC
};

// Parameter value type discriminant
enum RulesParamType {
	RULESPARAM_TYPE_BOOL = 0,
	RULESPARAM_TYPE_FLOAT = 1,
	RULESPARAM_TYPE_STRING = 2
};

// Parameter value (tagged union)
struct RulesParamValue {
	RulesParamType type;
	union {
		bool boolValue;
		float floatValue;
		const char* stringValue;
	};
};

// Query for a single parameter
struct RulesParamQuery {
	const char* paramName;
	int32_t teamID;      // For team params (-1 if not applicable)
	int32_t playerID;    // For player params (-1 if not applicable)
	int32_t unitID;      // For unit params (-1 if not applicable)
	int32_t featureID;   // For feature params (-1 if not applicable)
};

struct RulesParamResult {
	const Error* error;
	RulesParamValue value;
	int32_t los;  // LOS mask for this param
	bool exists;  // false if param doesn't exist
};

// Get all param names for an object
struct RulesParamNamesQuery {
	int32_t teamID;      // -1 if not applicable
	int32_t playerID;    // -1 if not applicable
	int32_t unitID;      // -1 if not applicable
	int32_t featureID;   // -1 if not applicable
};

struct RulesParamNamesResult {
	const Error* error;
	StringArray names;
};

// Set a parameter (synced control only)
struct RulesParamSet {
	const char* paramName;
	RulesParamValue value;
	int32_t los;  // LOS mask

	int32_t teamID;      // -1 if not applicable
	int32_t playerID;    // -1 if not applicable
	int32_t unitID;      // -1 if not applicable
	int32_t featureID;   // -1 if not applicable
};

// API structure
struct RulesParamsApi {
	// Get a game rules param
	RulesParamResult (*GetGameRulesParam)(const char* paramName);

	// Get all game rules param names
	RulesParamNamesResult (*GetGameRulesParams)();

	// Get a team rules param
	RulesParamResult (*GetTeamRulesParam)(int32_t teamID, const char* paramName);

	// Get all team rules param names
	RulesParamNamesResult (*GetTeamRulesParams)(int32_t teamID);

	// Get a player rules param
	RulesParamResult (*GetPlayerRulesParam)(int32_t playerID, const char* paramName);

	// Get all player rules param names
	RulesParamNamesResult (*GetPlayerRulesParams)(int32_t playerID);

	// Get a unit rules param
	RulesParamResult (*GetUnitRulesParam)(int32_t unitID, const char* paramName);

	// Get all unit rules param names
	RulesParamNamesResult (*GetUnitRulesParams)(int32_t unitID);

	// Get a feature rules param
	RulesParamResult (*GetFeatureRulesParam)(int32_t featureID, const char* paramName);

	// Get all feature rules param names
	RulesParamNamesResult (*GetFeatureRulesParams)(int32_t featureID);

	// Set params (synced control only - would be in SyncedCtrl API)
	BoolResult (*SetGameRulesParam)(const char* paramName, RulesParamValue value, int32_t los);
	BoolResult (*SetTeamRulesParam)(int32_t teamID, const char* paramName, RulesParamValue value, int32_t los);
	BoolResult (*SetPlayerRulesParam)(int32_t playerID, const char* paramName, RulesParamValue value, int32_t los);
	BoolResult (*SetUnitRulesParam)(int32_t unitID, const char* paramName, RulesParamValue value, int32_t los);
	BoolResult (*SetFeatureRulesParam)(int32_t featureID, const char* paramName, RulesParamValue value, int32_t los);
};

extern const RulesParamsApi RULES_PARAMS_API;

#ifdef __cplusplus
}
#endif
