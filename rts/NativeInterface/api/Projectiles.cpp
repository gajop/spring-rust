#include "Projectiles.h"

#include "Sim/Projectiles/Projectile.h"
#include "Sim/Projectiles/ProjectileHandler.h"
#include "Sim/Projectiles/WeaponProjectiles/WeaponProjectile.h"
#include "Sim/Projectiles/WeaponProjectiles/MissileProjectile.h"
#include "Sim/Projectiles/PieceProjectile.h"
#include "Sim/Features/Feature.h"
#include "Sim/Misc/QuadField.h"
#include "Sim/Misc/GlobalSynced.h"
#include "Sim/Misc/DamageArray.h"
#include "Sim/Units/Unit.h"
#include "Sim/Weapons/WeaponDef.h"
#include "System/float3.h"

namespace {

// Scratch buffer
static thread_local char scratchBuffer[1024];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Projectile system not ready" };
static const Error INVALID_PROJECTILE_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid projectile ID" };

// Helper: check if ready
static bool IsReady()
{
	return (gs != nullptr);
}

// Spatial queries
static void NativeGetProjectilesInRectangle(const GetProjectilesInRectangleQuery* query, GetProjectilesInRectangleResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->projectiles = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const float3 mins(query->minX, 0.0f, query->minZ);
	const float3 maxs(query->maxX, 0.0f, query->maxZ);

	// Use scratch buffer for array
	int32_t* projectiles = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t count = 0;
	const size_t maxProjectiles = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);

	QuadFieldQuery qfq;
	quadField.GetProjectilesExact(qfq, mins, maxs);
	if (qfq.projectiles != nullptr) {
		for (const CProjectile* proj : *(qfq.projectiles)) {
			if (proj != nullptr) {
				if (!proj->synced) continue;
				if (proj->weapon && query->options.excludeWeaponProjectiles) continue;
				if (proj->piece && query->options.excludePieceProjectiles) continue;
				if (count < maxProjectiles) {
					projectiles[count++] = proj->id;
				}
			}
		}
	}

	result->projectiles = projectiles;
	result->count = count;
	bufferPos += count * sizeof(int32_t);
}

static void NativeGetProjectilesInSphere(const GetProjectilesInSphereQuery* query, GetProjectilesInSphereResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->projectiles = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const float3 pos(query->center.x, query->center.y, query->center.z);
	const float radiusSq = query->radius * query->radius;

	// Use scratch buffer for array
	int32_t* projectiles = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t count = 0;
	const size_t maxProjectiles = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);

	QuadFieldQuery qfq;
	quadField.GetProjectilesExact(qfq, pos, query->radius);
	if (qfq.projectiles != nullptr) {
		for (const CProjectile* proj : *(qfq.projectiles)) {
			if (proj != nullptr) {
				if (!proj->synced) continue;
				if (proj->weapon && query->options.excludeWeaponProjectiles) continue;
				if (proj->piece && query->options.excludePieceProjectiles) continue;

				const float distSq = proj->pos.SqDistance(pos);
				if (distSq <= radiusSq && count < maxProjectiles) {
					projectiles[count++] = proj->id;
				}
			}
		}
	}

	result->projectiles = projectiles;
	result->count = count;
	bufferPos += count * sizeof(int32_t);
}

// Basic info
static void NativeGetProjectilePosition(const GetProjectilePositionQuery* query, GetProjectilePositionResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->position.x = 0.0f;
	result->position.y = 0.0f;
	result->position.z = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (proj == nullptr) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}

	result->position.x = proj->pos.x;
	result->position.y = proj->pos.y;
	result->position.z = proj->pos.z;
}

static void NativeGetProjectileDirection(const GetProjectileDirectionQuery* query, GetProjectileDirectionResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->direction.x = 0.0f;
	result->direction.y = 0.0f;
	result->direction.z = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (proj == nullptr) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}

	const float3 dir = proj->dir;
	result->direction.x = dir.x;
	result->direction.y = dir.y;
	result->direction.z = dir.z;
}

static void NativeGetProjectileVelocity(const GetProjectileVelocityQuery* query, GetProjectileVelocityResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->velocity.x = 0.0f;
	result->velocity.y = 0.0f;
	result->velocity.z = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (proj == nullptr) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}

	const float3 speed = proj->speed;
	result->velocity.x = speed.x;
	result->velocity.y = speed.y;
	result->velocity.z = speed.z;
}

static void NativeGetProjectileGravity(const GetProjectileGravityQuery* query, GetProjectileGravityResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->gravity.x = 0.0f;
	result->gravity.y = 0.0f;
	result->gravity.z = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (proj == nullptr) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}

	// Lua returns the stored gravity directly. The simulation applies this
	// value as an UpVector delta during projectile updates.
	result->gravity.y = proj->mygravity;
}
// Piece projectile
static void NativeGetPieceProjectileParams(const GetPieceProjectileParamsQuery* query, GetPieceProjectileParamsResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->isPieceProjectile = false;
	result->params.explFlags = 0;
	result->params.spinAngle = 0.0f;
	result->params.spinSpeed = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (proj == nullptr) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}

	const CPieceProjectile* pProj = dynamic_cast<const CPieceProjectile*>(proj);
	if (pProj == nullptr) {
		return; // Not a piece projectile
	}

	result->isPieceProjectile = true;
	result->params.pos.x = pProj->pos.x;
	result->params.pos.y = pProj->pos.y;
	result->params.pos.z = pProj->pos.z;
	result->params.speed.x = pProj->speed.x;
	result->params.speed.y = pProj->speed.y;
	result->params.speed.z = pProj->speed.z;
	result->params.gravity.x = 0.0f;
	result->params.gravity.y = pProj->mygravity;
	result->params.gravity.z = 0.0f;
	result->params.explFlags = static_cast<int32_t>(pProj->explFlags);
	result->params.spinAngle = pProj->spinAngle;
	result->params.spinSpeed = pProj->spinSpeed;
	result->params.spinVec.x = pProj->spinVec.x;
	result->params.spinVec.y = pProj->spinVec.y;
	result->params.spinVec.z = pProj->spinVec.z;
	result->params.modelPieceNum = 0;
	result->params.modelObjectType = 0;
	result->params.modelName = "";
	result->params.team = pProj->GetTeamID();
}

// Target
static void NativeGetProjectileTarget(const GetProjectileTargetQuery* query, GetProjectileTargetResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->target.targetType = 0; // No target
	result->target.targetID = -1;
	result->target.targetPos.x = 0.0f;
	result->target.targetPos.y = 0.0f;
	result->target.targetPos.z = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (proj == nullptr) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}

	const CWeaponProjectile* wProj = dynamic_cast<const CWeaponProjectile*>(proj);
	if (wProj == nullptr) {
		return; // Not a weapon projectile
	}

	result->target.targetPos.x = wProj->GetTargetPos().x;
	result->target.targetPos.y = wProj->GetTargetPos().y;
	result->target.targetPos.z = wProj->GetTargetPos().z;

	const CWorldObject* target = wProj->GetTargetObject();
	if (target == nullptr) {
		result->target.targetType = 'g';
		result->target.targetID = -1;
	} else if (dynamic_cast<const CUnit*>(target) != nullptr) {
		result->target.targetType = 'u';
		result->target.targetID = target->id;
	} else if (dynamic_cast<const CFeature*>(target) != nullptr) {
		result->target.targetType = 'f';
		result->target.targetID = target->id;
	} else if (dynamic_cast<const CWeaponProjectile*>(target) != nullptr) {
		result->target.targetType = 'p';
		result->target.targetID = target->id;
	}
}

// State
static void NativeGetProjectileIsIntercepted(const GetProjectileIsInterceptedQuery* query, GetProjectileIsInterceptedResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->isIntercepted = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (proj == nullptr) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}

	const CWeaponProjectile* wProj = dynamic_cast<const CWeaponProjectile*>(proj);
	if (wProj != nullptr) {
		result->isIntercepted = wProj->IsBeingIntercepted();
	}
}

static void NativeGetProjectileTimeToLive(const GetProjectileTimeToLiveQuery* query, GetProjectileTimeToLiveResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->ttl = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (proj == nullptr) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}

	const CWeaponProjectile* wProj = dynamic_cast<const CWeaponProjectile*>(proj);
	if (wProj != nullptr) {
		result->ttl = wProj->GetTimeToLive();
	}
}

// Owner
static void NativeGetProjectileOwnerID(const GetProjectileOwnerIDQuery* query, GetProjectileOwnerIDResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->ownerID = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (proj == nullptr) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}

	result->ownerID = proj->GetOwnerID();
}

static void NativeGetProjectileTeamID(const GetProjectileTeamIDQuery* query, GetProjectileTeamIDResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->teamID = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (proj == nullptr) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}

	result->teamID = proj->GetTeamID();
}

static void NativeGetProjectileAllyTeamID(const GetProjectileAllyTeamIDQuery* query, GetProjectileAllyTeamIDResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->allyTeamID = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (proj == nullptr) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}

	result->allyTeamID = proj->GetAllyteamID();
}

// Type
static void NativeGetProjectileType(const GetProjectileTypeQuery* query, GetProjectileTypeResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->weapon = false;
	result->piece = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (proj == nullptr) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}

	result->weapon = proj->weapon;
	result->piece = proj->piece;
}

static void NativeGetProjectileDefID(const GetProjectileDefIDQuery* query, GetProjectileDefIDResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->defID = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (proj == nullptr) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}

	const CWeaponProjectile* wProj = dynamic_cast<const CWeaponProjectile*>(proj);
	if (wProj != nullptr && wProj->GetWeaponDef() != nullptr) {
		result->defID = wProj->GetWeaponDef()->id;
	}
}

// Damages
static void NativeGetProjectileDamages(const GetProjectileDamagesQuery* query, GetProjectileDamagesResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->damages.damages = nullptr;
	result->damages.damageCount = 0;
	result->damages.paralyzeDamageTime = 0.0f;
	result->damages.impulseFactor = 0.0f;
	result->damages.impulseBoost = 0.0f;
	result->damages.craterMult = 0.0f;
	result->damages.craterBoost = 0.0f;
	result->damages.defaultDamage = 0.0f;
	result->damages.dynDamageExp = 0.0f;
	result->damages.dynDamageMin = 0.0f;
	result->damages.dynDamageRange = 0.0f;
	result->damages.dynDamageInverted = false;
	result->damages.craterAreaOfEffect = 0.0f;
	result->damages.damageAreaOfEffect = 0.0f;
	result->damages.edgeEffectiveness = 0.0f;
	result->damages.explosionSpeed = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (proj == nullptr) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}

	const CWeaponProjectile* wProj = dynamic_cast<const CWeaponProjectile*>(proj);
	if (wProj == nullptr || wProj->damages == nullptr) {
		return;
	}

	const DynDamageArray& damages = *wProj->damages;

	// Use scratch buffer for array
	float* damageValues = reinterpret_cast<float*>(scratchBuffer + bufferPos);
	uint32_t count = 0;
	const size_t maxDamages = (sizeof(scratchBuffer) - bufferPos) / sizeof(float);

	for (int i = 0; i < damages.GetNumTypes() && count < maxDamages; i++) {
		damageValues[count++] = damages.Get(i);
	}

	result->damages.damages = damageValues;
	result->damages.damageCount = count;
	result->damages.paralyzeDamageTime = damages.paralyzeDamageTime;
	result->damages.impulseFactor = damages.impulseFactor;
	result->damages.impulseBoost = damages.impulseBoost;
	result->damages.craterMult = damages.craterMult;
	result->damages.craterBoost = damages.craterBoost;
	result->damages.defaultDamage = damages.GetDefault();
	result->damages.dynDamageExp = damages.dynDamageExp;
	result->damages.dynDamageMin = damages.dynDamageMin;
	result->damages.dynDamageRange = damages.dynDamageRange;
	result->damages.dynDamageInverted = damages.dynDamageInverted;
	result->damages.craterAreaOfEffect = damages.craterAreaOfEffect;
	result->damages.damageAreaOfEffect = damages.damageAreaOfEffect;
	result->damages.edgeEffectiveness = damages.edgeEffectiveness;
	result->damages.explosionSpeed = damages.explosionSpeed;
	bufferPos += count * sizeof(float);
}

static void NativeGetAllProjectiles(const GetAllProjectilesQuery* query, GetAllProjectilesResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->projectiles = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const auto& projectiles = projectileHandler.GetActiveProjectiles(true).GetData();

	if (projectiles.empty())
		return;

	const size_t maxCount = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);
	int32_t* out = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t count = 0;

	for (const CProjectile* proj : projectiles) {
		if (proj == nullptr)
			continue;
		if (proj->weapon && query->options.excludeWeaponProjectiles)
			continue;
		if (proj->piece && query->options.excludePieceProjectiles)
			continue;

		if (count >= maxCount)
			break;

		out[count++] = proj->id;
	}

	result->projectiles = out;
	result->count = count;
	bufferPos += count * sizeof(int32_t);
}

} // namespace

const ProjectilesApi PROJECTILES_API = {
	.GetAllProjectiles = NativeGetAllProjectiles,
	.GetProjectilesInRectangle = NativeGetProjectilesInRectangle,
	.GetProjectilesInSphere = NativeGetProjectilesInSphere,

	.GetProjectilePosition = NativeGetProjectilePosition,
	.GetProjectileDirection = NativeGetProjectileDirection,
	.GetProjectileVelocity = NativeGetProjectileVelocity,
	.GetProjectileGravity = NativeGetProjectileGravity,

	.GetPieceProjectileParams = NativeGetPieceProjectileParams,

	.GetProjectileTarget = NativeGetProjectileTarget,

	.GetProjectileIsIntercepted = NativeGetProjectileIsIntercepted,
	.GetProjectileTimeToLive = NativeGetProjectileTimeToLive,

	.GetProjectileOwnerID = NativeGetProjectileOwnerID,
	.GetProjectileTeamID = NativeGetProjectileTeamID,
	.GetProjectileAllyTeamID = NativeGetProjectileAllyTeamID,

	.GetProjectileType = NativeGetProjectileType,
	.GetProjectileDefID = NativeGetProjectileDefID,

	.GetProjectileDamages = NativeGetProjectileDamages,
};
