#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Weapon Definitions API
// @see rts/Sim/Weapons/WeaponDef.h
//
// Static weapon definition data
// ============================================================================

struct WeaponDefInfo {
	int32_t id;
	const char* name;
	const char* type;  // "Cannon", "BeamLaser", "MissileLauncher", etc.
	const char* description;

	// Damage
	float* damages;          // Array indexed by armor type
	uint32_t damageCount;
	float defaultDamage;
	float paralyzeDamageTime;

	// Range and accuracy
	float range;
	float heightMod;
	float accuracy;
	float sprayAngle;
	float movingAccuracy;
	float targetMoveError;

	// Projectile
	float projectileSpeed;
	float startVelocity;
	float weaponAcceleration;
	bool turret;
	float turnRate;

	// Timings
	float reload;
	float beamTime;
	int32_t salvoSize;
	float salvoDelay;

	// Area of effect
	float areaOfEffect;
	float edgeEffectiveness;
	float craterMult;
	float craterBoost;

	// Ballistics
	float myGravity;
	bool noSelfDamage;
	float impulseFactor;
	float impulseBoost;

	// Targeting
	bool waterWeapon;
	bool fireSubmersed;
	bool submarineWeapon;
	bool canAttackGround;
	bool groundBounce;
	float heightBoostFactor;
	float proximityPriority;

	// Visual/Effects
	const char* cegTag;
};

struct WeaponDefResult {
	const Error* error;
	WeaponDefInfo info;
	bool exists;
};

// API structure
struct WeaponDefsApi {
	// Get all weapon def IDs
	Int32Array (*GetWeaponDefIDs)();

	// Get weapon def count
	UInt32Result (*GetWeaponDefCount)();

	// Get weapon def by ID
	WeaponDefResult (*GetWeaponDefByID)(int32_t weaponDefID);

	// Get weapon def ID by name
	Int32Result (*GetWeaponDefID)(const char* weaponDefName);

	// Check if weapon def is valid
	BoolResult (*ValidWeaponDefID)(int32_t weaponDefID);

	// Quick property accessors
	StringResult (*GetWeaponDefName)(int32_t weaponDefID);
	FloatResult (*GetWeaponDefRange)(int32_t weaponDefID);
	FloatResult (*GetWeaponDefDamage)(int32_t weaponDefID);

	// Custom params
	StringResult (*GetWeaponDefCustomParam)(int32_t weaponDefID, const char* key);
	StringArray (*GetWeaponDefCustomParamKeys)(int32_t weaponDefID);
};

extern const WeaponDefsApi WEAPON_DEFS_API;

#ifdef __cplusplus
}
#endif
