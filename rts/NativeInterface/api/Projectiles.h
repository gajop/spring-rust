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

// Queries
struct GetProjectilesInRectangleQuery { float minX; float minZ; float maxX; float maxZ; bool synced; bool weapon; };
struct GetProjectilesInRectangleResult { const Error* error; int32_t* projectiles; uint32_t count; };

struct GetProjectilesInSphereQuery { Float3 center; float radius; bool synced; bool weapon; };
struct GetProjectilesInSphereResult { const Error* error; int32_t* projectiles; uint32_t count; };

struct GetProjectilePositionQuery { int32_t projectileID; };
struct GetProjectilePositionResult { const Error* error; Float3 position; };

struct GetProjectileDirectionQuery { int32_t projectileID; };
struct GetProjectileDirectionResult { const Error* error; Float3 direction; };

struct GetProjectileVelocityQuery { int32_t projectileID; };
struct GetProjectileVelocityResult { const Error* error; Float3 velocity; };

struct GetProjectileGravityQuery { int32_t projectileID; };
struct GetProjectileGravityResult { const Error* error; Float3 gravity; };

struct GetPieceProjectileParamsQuery { int32_t projectileID; };
struct GetPieceProjectileParamsResult { const Error* error; PieceProjectileParams params; bool isPieceProjectile; };

struct GetProjectileTargetQuery { int32_t projectileID; };
struct GetProjectileTargetResult { const Error* error; ProjectileTarget target; };

struct GetProjectileIsInterceptedQuery { int32_t projectileID; };
struct GetProjectileIsInterceptedResult { const Error* error; bool isIntercepted; };

struct GetProjectileTimeToLiveQuery { int32_t projectileID; };
struct GetProjectileTimeToLiveResult { const Error* error; float ttl; };

struct GetProjectileOwnerIDQuery { int32_t projectileID; };
struct GetProjectileOwnerIDResult { const Error* error; int32_t ownerID; };

struct GetProjectileTeamIDQuery { int32_t projectileID; };
struct GetProjectileTeamIDResult { const Error* error; int32_t teamID; };

struct GetProjectileAllyTeamIDQuery { int32_t projectileID; };
struct GetProjectileAllyTeamIDResult { const Error* error; int32_t allyTeamID; };

struct GetProjectileTypeQuery { int32_t projectileID; };
struct GetProjectileTypeResult { const Error* error; uint32_t type; };

struct GetProjectileDefIDQuery { int32_t projectileID; };
struct GetProjectileDefIDResult { const Error* error; int32_t defID; };

struct GetProjectileDamagesQuery { int32_t projectileID; };
struct GetProjectileDamagesResult { const Error* error; ProjectileDamages damages; };

struct GetAllProjectilesQuery { bool synced; bool weapon; };
struct GetAllProjectilesResult { const Error* error; int32_t* projectiles; uint32_t count; };

// API structure
struct ProjectilesApi {
	void (*GetAllProjectiles)(const GetAllProjectilesQuery* query, GetAllProjectilesResult* result);
	void (*GetProjectilesInRectangle)(const GetProjectilesInRectangleQuery* query, GetProjectilesInRectangleResult* result);
	void (*GetProjectilesInSphere)(const GetProjectilesInSphereQuery* query, GetProjectilesInSphereResult* result);
	void (*GetProjectilePosition)(const GetProjectilePositionQuery* query, GetProjectilePositionResult* result);
	void (*GetProjectileDirection)(const GetProjectileDirectionQuery* query, GetProjectileDirectionResult* result);
	void (*GetProjectileVelocity)(const GetProjectileVelocityQuery* query, GetProjectileVelocityResult* result);
	void (*GetProjectileGravity)(const GetProjectileGravityQuery* query, GetProjectileGravityResult* result);
	void (*GetPieceProjectileParams)(const GetPieceProjectileParamsQuery* query, GetPieceProjectileParamsResult* result);
	void (*GetProjectileTarget)(const GetProjectileTargetQuery* query, GetProjectileTargetResult* result);
	void (*GetProjectileIsIntercepted)(const GetProjectileIsInterceptedQuery* query, GetProjectileIsInterceptedResult* result);
	void (*GetProjectileTimeToLive)(const GetProjectileTimeToLiveQuery* query, GetProjectileTimeToLiveResult* result);
	void (*GetProjectileOwnerID)(const GetProjectileOwnerIDQuery* query, GetProjectileOwnerIDResult* result);
	void (*GetProjectileTeamID)(const GetProjectileTeamIDQuery* query, GetProjectileTeamIDResult* result);
	void (*GetProjectileAllyTeamID)(const GetProjectileAllyTeamIDQuery* query, GetProjectileAllyTeamIDResult* result);
	void (*GetProjectileType)(const GetProjectileTypeQuery* query, GetProjectileTypeResult* result);
	void (*GetProjectileDefID)(const GetProjectileDefIDQuery* query, GetProjectileDefIDResult* result);
	void (*GetProjectileDamages)(const GetProjectileDamagesQuery* query, GetProjectileDamagesResult* result);
};

extern const ProjectilesApi PROJECTILES_API;

#ifdef __cplusplus
}
#endif
