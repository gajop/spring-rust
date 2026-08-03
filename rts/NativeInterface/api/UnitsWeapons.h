#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Units Weapons API
// @see rts/Lua/LuaSyncedRead.cpp
//
// Unit weapon queries and targeting
// ============================================================================

// Weapon state
struct UnitWeaponState {
	float reloadTime;
	float reloadFrame;
	float range;
	float projectileSpeed;
	float accuracy;
	float sprayAngle;
	float aimFromHeight;
	float salvoSize;
	float salvoDelay;
	float salvoError;
	float targetMoveError;
	float turnRate;
	bool autoTarget;
};

// Weapon damages
struct UnitWeaponDamages {
	float* damages;           // Indexed by armor type
	uint32_t damageCount;
	float paralyzeDamageTime;
	float impulseFactor;
	float impulseBoost;
	float craterMult;
	float craterBoost;
	float defaultDamage;
};

// Weapon vectors
struct UnitWeaponVectors {
	Float3 weaponMuzzlePos;
	Float3 weaponAimPos;
	Float3 weaponDir;
};

// Weapon targeting
struct UnitWeaponTarget {
	int32_t targetType;  // 0=none, 1=unit, 2=ground
	int32_t targetID;    // Unit ID if type==1
	Float3 targetPos;
};

// Queries
struct GetUnitWeaponCountQuery { int32_t unitID; };
struct GetUnitWeaponCountResult { const Error* error; uint32_t count; };

struct GetUnitMaxRangeQuery { int32_t unitID; };
struct GetUnitMaxRangeResult { const Error* error; float maxRange; };

struct GetUnitWeaponStateQuery { int32_t unitID; int32_t weaponNum; const char* key; };
struct GetUnitWeaponStateResult { const Error* error; UnitWeaponState state; };

struct GetUnitWeaponDamagesQuery { int32_t unitID; int32_t weaponNum; };
struct GetUnitWeaponDamagesResult { const Error* error; UnitWeaponDamages damages; };

struct GetUnitWeaponVectorsQuery { int32_t unitID; int32_t weaponNum; };
struct GetUnitWeaponVectorsResult { const Error* error; UnitWeaponVectors vectors; };

struct GetUnitWeaponTryTargetOptions { bool userTarget; bool isGroundTarget; };
struct GetUnitWeaponTryTargetQuery { int32_t unitID; int32_t weaponNum; int32_t targetID; Float3 targetPos; GetUnitWeaponTryTargetOptions options; };
struct GetUnitWeaponTryTargetResult { const Error* error; bool canTarget; };

struct GetUnitWeaponTestTargetOptions { bool isGroundTarget; };
struct GetUnitWeaponTestTargetQuery { int32_t unitID; int32_t weaponNum; int32_t targetID; Float3 targetPos; GetUnitWeaponTestTargetOptions options; };
struct GetUnitWeaponTestTargetResult { const Error* error; bool canTarget; };

struct GetUnitWeaponTestRangeQuery { int32_t unitID; int32_t weaponNum; Float3 targetPos; };
struct GetUnitWeaponTestRangeResult { const Error* error; bool inRange; };

struct GetUnitWeaponHaveFreeLineOfFireOptions { bool isGroundTarget; };
struct GetUnitWeaponHaveFreeLineOfFireQuery { int32_t unitID; int32_t weaponNum; int32_t targetID; Float3 sourcePos; Float3 targetPos; GetUnitWeaponHaveFreeLineOfFireOptions options; };
struct GetUnitWeaponHaveFreeLineOfFireResult { const Error* error; bool hasFreeLineOfFire; };

struct GetUnitWeaponCanFireQuery { int32_t unitID; int32_t weaponNum; };
struct GetUnitWeaponCanFireResult { const Error* error; bool canFire; };

struct GetUnitWeaponTargetQuery { int32_t unitID; int32_t weaponNum; };
struct GetUnitWeaponTargetResult { const Error* error; UnitWeaponTarget target; };

// API structure
struct UnitsWeaponsApi {
	void (*GetUnitWeaponCount)(const GetUnitWeaponCountQuery* query, GetUnitWeaponCountResult* result);
	void (*GetUnitMaxRange)(const GetUnitMaxRangeQuery* query, GetUnitMaxRangeResult* result);
	void (*GetUnitWeaponState)(const GetUnitWeaponStateQuery* query, GetUnitWeaponStateResult* result);
	void (*GetUnitWeaponDamages)(const GetUnitWeaponDamagesQuery* query, GetUnitWeaponDamagesResult* result);
	void (*GetUnitWeaponVectors)(const GetUnitWeaponVectorsQuery* query, GetUnitWeaponVectorsResult* result);
	void (*GetUnitWeaponTryTarget)(const GetUnitWeaponTryTargetQuery* query, GetUnitWeaponTryTargetResult* result);
	void (*GetUnitWeaponTestTarget)(const GetUnitWeaponTestTargetQuery* query, GetUnitWeaponTestTargetResult* result);
	void (*GetUnitWeaponTestRange)(const GetUnitWeaponTestRangeQuery* query, GetUnitWeaponTestRangeResult* result);
	void (*GetUnitWeaponHaveFreeLineOfFire)(const GetUnitWeaponHaveFreeLineOfFireQuery* query, GetUnitWeaponHaveFreeLineOfFireResult* result);
	void (*GetUnitWeaponCanFire)(const GetUnitWeaponCanFireQuery* query, GetUnitWeaponCanFireResult* result);
	void (*GetUnitWeaponTarget)(const GetUnitWeaponTargetQuery* query, GetUnitWeaponTargetResult* result);
};

extern const UnitsWeaponsApi UNITS_WEAPONS_API;

#ifdef __cplusplus
}
#endif
