#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Projectiles API
// @see rts/Lua/LuaSyncedRead.cpp
//
// Projectile queries (bullets, missiles, beams, etc.)
// ============================================================================

// Projectile target
struct ProjectileTarget {
	int32_t targetType;  // 0=none, 1=unit, 2=ground, 3=feature
	int32_t targetID;    // Unit or feature ID
	Float3 targetPos;
};

struct ProjectileTargetResult {
	const Error* error;
	ProjectileTarget target;
};

// Piece projectile params
struct PieceProjectileParams {
	Float3 pos;
	Float3 speed;
	Float3 gravity;
	Float3 spinVec;
	int32_t modelPieceNum;
	int32_t modelObjectType;  // 1=s3o, 2=obj, etc.
	const char* modelName;
	int32_t team;
};

struct PieceProjectileParamsResult {
	const Error* error;
	PieceProjectileParams params;
	bool isPieceProjectile;
};

// Projectile damages
struct ProjectileDamages {
	float* damages;           // Indexed by armor type
	uint32_t damageCount;
	float paralyzeDamageTime;
	float impulseFactor;
	float impulseBoost;
	float craterMult;
	float craterBoost;
	float defaultDamage;
};

struct ProjectileDamagesResult {
	const Error* error;
	ProjectileDamages damages;
};

// API structure
struct ProjectilesApi {
	// Spatial queries
	Int32Array (*GetProjectilesInRectangle)(float minX, float minZ, float maxX, float maxZ, bool synced, bool weapon);
	Int32Array (*GetProjectilesInSphere)(Float3 center, float radius, bool synced, bool weapon);

	// Basic info
	Float3Result (*GetProjectilePosition)(int32_t projectileID);
	Float3Result (*GetProjectileDirection)(int32_t projectileID);
	Float3Result (*GetProjectileVelocity)(int32_t projectileID);
	Float3Result (*GetProjectileGravity)(int32_t projectileID);

	// Piece projectile
	PieceProjectileParamsResult (*GetPieceProjectileParams)(int32_t projectileID);

	// Target
	ProjectileTargetResult (*GetProjectileTarget)(int32_t projectileID);

	// State
	BoolResult (*GetProjectileIsIntercepted)(int32_t projectileID);
	FloatResult (*GetProjectileTimeToLive)(int32_t projectileID);

	// Owner
	Int32Result (*GetProjectileOwnerID)(int32_t projectileID);
	Int32Result (*GetProjectileTeamID)(int32_t projectileID);
	Int32Result (*GetProjectileAllyTeamID)(int32_t projectileID);

	// Type
	UInt32Result (*GetProjectileType)(int32_t projectileID);
	Int32Result (*GetProjectileDefID)(int32_t projectileID);  // Weapon def ID

	// Damages
	ProjectileDamagesResult (*GetProjectileDamages)(int32_t projectileID);
};

extern const ProjectilesApi PROJECTILES_API;

#ifdef __cplusplus
}
#endif
