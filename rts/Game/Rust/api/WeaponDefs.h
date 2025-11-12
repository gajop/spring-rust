#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Weapon Definitions API
// @see rts/Lua/LuaWeaponDefs.cpp
//
// Static weapon definition data
// ============================================================================

struct WeaponDefInfo {
	int32_t id;
	const char* name;
	const char* type;
	const char* description;
	float range;
	float reloadTime;
	float damage;
	float areaOfEffect;
	float projectileSpeed;
	bool paralyzer;
	bool impactOnly;
	bool turret;
};

// Queries
struct GetWeaponDefIDsQuery { uint8_t _unused; };
struct GetWeaponDefIDsResult { const Error* error; int32_t* ids; uint32_t count; };

struct GetWeaponDefCountQuery { uint8_t _unused; };
struct GetWeaponDefCountResult { const Error* error; uint32_t count; };

struct GetWeaponDefByIDQuery { int32_t weaponDefID; };
struct GetWeaponDefByIDResult { const Error* error; WeaponDefInfo info; bool exists; };

struct GetWeaponDefIDQuery { const char* weaponDefName; };
struct GetWeaponDefIDResult { const Error* error; int32_t id; };

struct ValidWeaponDefIDQuery { int32_t weaponDefID; };
struct ValidWeaponDefIDResult { const Error* error; bool valid; };

struct GetWeaponDefNameQuery { int32_t weaponDefID; };
struct GetWeaponDefNameResult { const Error* error; const char* name; };

struct GetWeaponDefRangeQuery { int32_t weaponDefID; };
struct GetWeaponDefRangeResult { const Error* error; float range; };

struct GetWeaponDefDamageQuery { int32_t weaponDefID; };
struct GetWeaponDefDamageResult { const Error* error; float damage; };

struct GetWeaponDefCustomParamQuery { int32_t weaponDefID; const char* key; };
struct GetWeaponDefCustomParamResult { const Error* error; const char* value; };

struct GetWeaponDefCustomParamKeysQuery { int32_t weaponDefID; };
struct GetWeaponDefCustomParamKeysResult { const Error* error; const char** keys; uint32_t count; };

// API structure
struct WeaponDefsApi {
	void (*GetWeaponDefIDs)(const GetWeaponDefIDsQuery* query, GetWeaponDefIDsResult* result);
	void (*GetWeaponDefCount)(const GetWeaponDefCountQuery* query, GetWeaponDefCountResult* result);
	void (*GetWeaponDefByID)(const GetWeaponDefByIDQuery* query, GetWeaponDefByIDResult* result);
	void (*GetWeaponDefID)(const GetWeaponDefIDQuery* query, GetWeaponDefIDResult* result);
	void (*ValidWeaponDefID)(const ValidWeaponDefIDQuery* query, ValidWeaponDefIDResult* result);
	void (*GetWeaponDefName)(const GetWeaponDefNameQuery* query, GetWeaponDefNameResult* result);
	void (*GetWeaponDefRange)(const GetWeaponDefRangeQuery* query, GetWeaponDefRangeResult* result);
	void (*GetWeaponDefDamage)(const GetWeaponDefDamageQuery* query, GetWeaponDefDamageResult* result);
	void (*GetWeaponDefCustomParam)(const GetWeaponDefCustomParamQuery* query, GetWeaponDefCustomParamResult* result);
	void (*GetWeaponDefCustomParamKeys)(const GetWeaponDefCustomParamKeysQuery* query, GetWeaponDefCustomParamKeysResult* result);
};

extern const WeaponDefsApi WEAPON_DEFS_API;

#ifdef __cplusplus
}
#endif
