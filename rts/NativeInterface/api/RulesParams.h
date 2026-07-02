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

// Queries
struct GetGameRulesParamQuery { const char* paramName; };
struct GetGameRulesParamResult { const Error* error; RulesParamValue value; int32_t los; bool exists; };

struct GetGameRulesParamsQuery { uint8_t _unused; };
struct GetGameRulesParamsResult { const Error* error; const char** names; uint32_t count; };

struct GetTeamRulesParamQuery { int32_t teamID; const char* paramName; };
struct GetTeamRulesParamResult { const Error* error; RulesParamValue value; int32_t los; bool exists; };

struct GetTeamRulesParamsQuery { int32_t teamID; };
struct GetTeamRulesParamsResult { const Error* error; const char** names; uint32_t count; };

struct GetPlayerRulesParamQuery { int32_t playerID; const char* paramName; };
struct GetPlayerRulesParamResult { const Error* error; RulesParamValue value; int32_t los; bool exists; };

struct GetPlayerRulesParamsQuery { int32_t playerID; };
struct GetPlayerRulesParamsResult { const Error* error; const char** names; uint32_t count; };

struct GetUnitRulesParamQuery { int32_t unitID; const char* paramName; };
struct GetUnitRulesParamResult { const Error* error; RulesParamValue value; int32_t los; bool exists; };

struct GetUnitRulesParamsQuery { int32_t unitID; };
struct GetUnitRulesParamsResult { const Error* error; const char** names; uint32_t count; };

struct GetFeatureRulesParamQuery { int32_t featureID; const char* paramName; };
struct GetFeatureRulesParamResult { const Error* error; RulesParamValue value; int32_t los; bool exists; };

struct GetFeatureRulesParamsQuery { int32_t featureID; };
struct GetFeatureRulesParamsResult { const Error* error; const char** names; uint32_t count; };

struct SetGameRulesParamQuery { const char* paramName; RulesParamValue value; int32_t los; };
struct SetGameRulesParamResult { const Error* error; bool success; };

struct SetTeamRulesParamQuery { int32_t teamID; const char* paramName; RulesParamValue value; int32_t los; };
struct SetTeamRulesParamResult { const Error* error; bool success; };

struct SetPlayerRulesParamQuery { int32_t playerID; const char* paramName; RulesParamValue value; int32_t los; };
struct SetPlayerRulesParamResult { const Error* error; bool success; };

struct SetUnitRulesParamQuery { int32_t unitID; const char* paramName; RulesParamValue value; int32_t los; };
struct SetUnitRulesParamResult { const Error* error; bool success; };

struct SetFeatureRulesParamQuery { int32_t featureID; const char* paramName; RulesParamValue value; int32_t los; };
struct SetFeatureRulesParamResult { const Error* error; bool success; };

// API structure
struct RulesParamsApi {
	void (*GetGameRulesParam)(const GetGameRulesParamQuery* query, GetGameRulesParamResult* result);
	void (*GetGameRulesParams)(const GetGameRulesParamsQuery* query, GetGameRulesParamsResult* result);
	void (*GetTeamRulesParam)(const GetTeamRulesParamQuery* query, GetTeamRulesParamResult* result);
	void (*GetTeamRulesParams)(const GetTeamRulesParamsQuery* query, GetTeamRulesParamsResult* result);
	void (*GetPlayerRulesParam)(const GetPlayerRulesParamQuery* query, GetPlayerRulesParamResult* result);
	void (*GetPlayerRulesParams)(const GetPlayerRulesParamsQuery* query, GetPlayerRulesParamsResult* result);
	void (*GetUnitRulesParam)(const GetUnitRulesParamQuery* query, GetUnitRulesParamResult* result);
	void (*GetUnitRulesParams)(const GetUnitRulesParamsQuery* query, GetUnitRulesParamsResult* result);
	void (*GetFeatureRulesParam)(const GetFeatureRulesParamQuery* query, GetFeatureRulesParamResult* result);
	void (*GetFeatureRulesParams)(const GetFeatureRulesParamsQuery* query, GetFeatureRulesParamsResult* result);
	void (*SetGameRulesParam)(const SetGameRulesParamQuery* query, SetGameRulesParamResult* result);
	void (*SetTeamRulesParam)(const SetTeamRulesParamQuery* query, SetTeamRulesParamResult* result);
	void (*SetPlayerRulesParam)(const SetPlayerRulesParamQuery* query, SetPlayerRulesParamResult* result);
	void (*SetUnitRulesParam)(const SetUnitRulesParamQuery* query, SetUnitRulesParamResult* result);
	void (*SetFeatureRulesParam)(const SetFeatureRulesParamQuery* query, SetFeatureRulesParamResult* result);
};

extern const RulesParamsApi RULES_PARAMS_API;

#ifdef __cplusplus
}
#endif
