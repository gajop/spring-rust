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

struct UnitWeaponStateResult {
	const Error* error;
	UnitWeaponState state;
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

struct UnitWeaponDamagesResult {
	const Error* error;
	UnitWeaponDamages damages;
};

// Weapon vectors
struct UnitWeaponVectors {
	Float3 weaponMuzzlePos;
	Float3 weaponAimPos;
	Float3 weaponDir;
};

struct UnitWeaponVectorsResult {
	const Error* error;
	UnitWeaponVectors vectors;
};

// Weapon targeting
struct UnitWeaponTarget {
	int32_t targetType;  // 0=none, 1=unit, 2=ground
	int32_t targetID;    // Unit ID if type==1
	Float3 targetPos;
};

struct UnitWeaponTargetResult {
	const Error* error;
	UnitWeaponTarget target;
};

// Try target query
struct WeaponTryTargetQuery {
	int32_t unitID;
	int32_t weaponNum;
	int32_t targetID;       // For unit target
	Float3 targetPos;       // For ground target
	bool userTarget;        // User-issued target
	bool isGroundTarget;    // false for unit target, true for ground
};

// Test target query
struct WeaponTestTargetQuery {
	int32_t unitID;
	int32_t weaponNum;
	int32_t targetID;       // For unit target
	Float3 targetPos;       // For ground target
	bool isGroundTarget;
};

// Test range query
struct WeaponTestRangeQuery {
	int32_t unitID;
	int32_t weaponNum;
	Float3 targetPos;
};

// Line of fire query
struct WeaponLineOfFireQuery {
	int32_t unitID;
	int32_t weaponNum;
	int32_t targetID;       // For unit target
	Float3 targetPos;       // For ground target
	bool isGroundTarget;
};

// Can fire query
struct WeaponCanFireQuery {
	int32_t unitID;
	int32_t weaponNum;
};

// API structure
struct UnitsWeaponsApi {
	// Weapon count
	UInt32Result (*GetUnitWeaponCount)(int32_t unitID);

	// Max range (any weapon)
	FloatResult (*GetUnitMaxRange)(int32_t unitID);

	// Weapon state
	UnitWeaponStateResult (*GetUnitWeaponState)(int32_t unitID, int32_t weaponNum);

	// Weapon damages
	UnitWeaponDamagesResult (*GetUnitWeaponDamages)(int32_t unitID, int32_t weaponNum);

	// Weapon vectors
	UnitWeaponVectorsResult (*GetUnitWeaponVectors)(int32_t unitID, int32_t weaponNum);

	// Weapon targeting
	BoolResult (*GetUnitWeaponTryTarget)(WeaponTryTargetQuery query);
	BoolResult (*GetUnitWeaponTestTarget)(WeaponTestTargetQuery query);
	BoolResult (*GetUnitWeaponTestRange)(WeaponTestRangeQuery query);
	BoolResult (*GetUnitWeaponHaveFreeLineOfFire)(WeaponLineOfFireQuery query);
	BoolResult (*GetUnitWeaponCanFire)(WeaponCanFireQuery query);
	UnitWeaponTargetResult (*GetUnitWeaponTarget)(int32_t unitID, int32_t weaponNum);
};

extern const UnitsWeaponsApi UNITS_WEAPONS_API;

#ifdef __cplusplus
}
#endif
