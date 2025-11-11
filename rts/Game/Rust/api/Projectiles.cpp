#include "Projectiles.h"

#include "Sim/Projectiles/Projectile.h"
#include "Sim/Projectiles/ProjectileHandler.h"
#include "Sim/Projectiles/WeaponProjectiles/WeaponProjectile.h"
#include "Sim/Projectiles/WeaponProjectiles/MissileProjectile.h"
#include "Sim/Projectiles/PieceProjectile.h"
#include "Sim/Misc/QuadField.h"
#include "Sim/Misc/GlobalSynced.h"
#include "Sim/Weapons/WeaponDef.h"
#include "System/float3.h"

namespace {

// Error constants
static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Projectile system not ready"
};

static const Error INVALID_PROJECTILE_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid projectile ID"
};

// Helper: check if ready
static bool IsReady()
{
	return (gs != nullptr);
}

// Spatial queries
static Int32Array NativeGetProjectilesInRectangle(float minX, float minZ, float maxX, float maxZ, bool synced, bool weapon)
{
	Int32Array result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Use static storage - valid for call duration only
	static thread_local std::vector<int32_t> projectiles;
	projectiles.clear();

	const float3 mins(minX, 0.0f, minZ);
	const float3 maxs(maxX, 0.0f, maxZ);

	const auto& foundProjectiles = quadField.GetProjectilesExact(mins, maxs);
	for (const CProjectile* proj : foundProjectiles) {
		if (proj != nullptr) {
			if (synced && !proj->synced) continue;
			if (weapon && !proj->weapon) continue;
			projectiles.push_back(proj->id);
		}
	}

	result.data = projectiles.data();
	result.length = static_cast<uint32_t>(projectiles.size());
	return result;
}

static Int32Array NativeGetProjectilesInSphere(Float3 center, float radius, bool synced, bool weapon)
{
	Int32Array result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Use static storage - valid for call duration only
	static thread_local std::vector<int32_t> projectiles;
	projectiles.clear();

	const float3 pos(center.x, center.y, center.z);
	const float radiusSq = radius * radius;

	const auto& foundProjectiles = quadField.GetProjectilesExact(pos, radius);
	for (const CProjectile* proj : foundProjectiles) {
		if (proj != nullptr) {
			if (synced && !proj->synced) continue;
			if (weapon && !proj->weapon) continue;

			const float distSq = proj->pos.SqDistance(pos);
			if (distSq <= radiusSq) {
				projectiles.push_back(proj->id);
			}
		}
	}

	result.data = projectiles.data();
	result.length = static_cast<uint32_t>(projectiles.size());
	return result;
}

// Basic info
static Float3Result NativeGetProjectilePosition(int32_t projectileID)
{
	Float3Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(projectileID);
	if (proj == nullptr) {
		result.error = &INVALID_PROJECTILE_ERROR;
		return result;
	}

	result.value.x = proj->pos.x;
	result.value.y = proj->pos.y;
	result.value.z = proj->pos.z;
	return result;
}

static Float3Result NativeGetProjectileDirection(int32_t projectileID)
{
	Float3Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(projectileID);
	if (proj == nullptr) {
		result.error = &INVALID_PROJECTILE_ERROR;
		return result;
	}

	const float3 dir = proj->dir;
	result.value.x = dir.x;
	result.value.y = dir.y;
	result.value.z = dir.z;
	return result;
}

static Float3Result NativeGetProjectileVelocity(int32_t projectileID)
{
	Float3Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(projectileID);
	if (proj == nullptr) {
		result.error = &INVALID_PROJECTILE_ERROR;
		return result;
	}

	const float3 speed = proj->speed;
	result.value.x = speed.x;
	result.value.y = speed.y;
	result.value.z = speed.z;
	return result;
}

static Float3Result NativeGetProjectileGravity(int32_t projectileID)
{
	Float3Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(projectileID);
	if (proj == nullptr) {
		result.error = &INVALID_PROJECTILE_ERROR;
		return result;
	}

	// Most projectiles use standard gravity
	result.value.x = 0.0f;
	result.value.y = -proj->mygravity;
	result.value.z = 0.0f;
	return result;
}

// Piece projectile
static PieceProjectileParamsResult NativeGetPieceProjectileParams(int32_t projectileID)
{
	PieceProjectileParamsResult result = {};
	result.isPieceProjectile = false;

	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(projectileID);
	if (proj == nullptr) {
		result.error = &INVALID_PROJECTILE_ERROR;
		return result;
	}

	const CPieceProjectile* pProj = dynamic_cast<const CPieceProjectile*>(proj);
	if (pProj == nullptr) {
		return result; // Not a piece projectile
	}

	result.isPieceProjectile = true;
	result.params.pos.x = pProj->pos.x;
	result.params.pos.y = pProj->pos.y;
	result.params.pos.z = pProj->pos.z;
	result.params.speed.x = pProj->speed.x;
	result.params.speed.y = pProj->speed.y;
	result.params.speed.z = pProj->speed.z;
	result.params.gravity.x = 0.0f;
	result.params.gravity.y = -pProj->mygravity;
	result.params.gravity.z = 0.0f;
	result.params.spinVec.x = 0.0f;
	result.params.spinVec.y = 0.0f;
	result.params.spinVec.z = 0.0f;
	result.params.modelPieceNum = 0; // Not directly accessible
	result.params.modelObjectType = 0;
	result.params.modelName = "";
	result.params.team = pProj->GetTeamID();

	return result;
}

// Target
static ProjectileTargetResult NativeGetProjectileTarget(int32_t projectileID)
{
	ProjectileTargetResult result = {};
	result.target.targetType = 0; // No target

	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(projectileID);
	if (proj == nullptr) {
		result.error = &INVALID_PROJECTILE_ERROR;
		return result;
	}

	const CWeaponProjectile* wProj = dynamic_cast<const CWeaponProjectile*>(proj);
	if (wProj == nullptr) {
		return result; // Not a weapon projectile
	}

	if (wProj->GetTargetObject() != nullptr) {
		// Has unit/feature target
		result.target.targetType = 1; // Unit (could be feature too)
		result.target.targetID = wProj->GetTargetObject()->id;
		result.target.targetPos.x = wProj->GetTargetPos().x;
		result.target.targetPos.y = wProj->GetTargetPos().y;
		result.target.targetPos.z = wProj->GetTargetPos().z;
	} else {
		// Ground target
		result.target.targetType = 2; // Ground
		result.target.targetID = -1;
		result.target.targetPos.x = wProj->GetTargetPos().x;
		result.target.targetPos.y = wProj->GetTargetPos().y;
		result.target.targetPos.z = wProj->GetTargetPos().z;
	}

	return result;
}

// State
static BoolResult NativeGetProjectileIsIntercepted(int32_t projectileID)
{
	BoolResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(projectileID);
	if (proj == nullptr) {
		result.error = &INVALID_PROJECTILE_ERROR;
		return result;
	}

	const CWeaponProjectile* wProj = dynamic_cast<const CWeaponProjectile*>(proj);
	if (wProj != nullptr) {
		result.value = (wProj->GetTargetObject() == nullptr && wProj->GetTargetPos() == ZeroVector);
	} else {
		result.value = false;
	}

	return result;
}

static FloatResult NativeGetProjectileTimeToLive(int32_t projectileID)
{
	FloatResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(projectileID);
	if (proj == nullptr) {
		result.error = &INVALID_PROJECTILE_ERROR;
		return result;
	}

	const CWeaponProjectile* wProj = dynamic_cast<const CWeaponProjectile*>(proj);
	if (wProj != nullptr) {
		result.value = wProj->GetTTL();
	} else {
		result.value = 0.0f;
	}

	return result;
}

// Owner
static Int32Result NativeGetProjectileOwnerID(int32_t projectileID)
{
	Int32Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(projectileID);
	if (proj == nullptr) {
		result.error = &INVALID_PROJECTILE_ERROR;
		return result;
	}

	result.value = proj->GetOwnerID();
	return result;
}

static Int32Result NativeGetProjectileTeamID(int32_t projectileID)
{
	Int32Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(projectileID);
	if (proj == nullptr) {
		result.error = &INVALID_PROJECTILE_ERROR;
		return result;
	}

	result.value = proj->GetTeamID();
	return result;
}

static Int32Result NativeGetProjectileAllyTeamID(int32_t projectileID)
{
	Int32Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(projectileID);
	if (proj == nullptr) {
		result.error = &INVALID_PROJECTILE_ERROR;
		return result;
	}

	result.value = proj->GetAllyteamID();
	return result;
}

// Type
static UInt32Result NativeGetProjectileType(int32_t projectileID)
{
	UInt32Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(projectileID);
	if (proj == nullptr) {
		result.error = &INVALID_PROJECTILE_ERROR;
		return result;
	}

	result.value = proj->GetProjectileType();
	return result;
}

static Int32Result NativeGetProjectileDefID(int32_t projectileID)
{
	Int32Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(projectileID);
	if (proj == nullptr) {
		result.error = &INVALID_PROJECTILE_ERROR;
		return result;
	}

	const CWeaponProjectile* wProj = dynamic_cast<const CWeaponProjectile*>(proj);
	if (wProj != nullptr && wProj->weaponDef != nullptr) {
		result.value = wProj->weaponDef->id;
	} else {
		result.value = -1;
	}

	return result;
}

// Damages
static ProjectileDamagesResult NativeGetProjectileDamages(int32_t projectileID)
{
	ProjectileDamagesResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CProjectile* proj = projectileHandler.GetProjectileBySyncedID(projectileID);
	if (proj == nullptr) {
		result.error = &INVALID_PROJECTILE_ERROR;
		return result;
	}

	const CWeaponProjectile* wProj = dynamic_cast<const CWeaponProjectile*>(proj);
	if (wProj == nullptr || wProj->weaponDef == nullptr) {
		// No damage info available
		result.damages.damages = nullptr;
		result.damages.damageCount = 0;
		result.damages.defaultDamage = 0.0f;
		return result;
	}

	const WeaponDef* weaponDef = wProj->weaponDef;
	const DamageArray& damages = weaponDef->damages;

	// Use static storage - valid for call duration only
	static thread_local std::vector<float> damageValues;
	damageValues.clear();

	for (int i = 0; i < damages.GetNumTypes(); i++) {
		damageValues.push_back(damages.Get(i));
	}

	result.damages.damages = damageValues.data();
	result.damages.damageCount = static_cast<uint32_t>(damageValues.size());
	result.damages.paralyzeDamageTime = weaponDef->damages.paralyzeDamageTime;
	result.damages.impulseFactor = weaponDef->damages.impulseFactor;
	result.damages.impulseBoost = weaponDef->damages.impulseBoost;
	result.damages.craterMult = weaponDef->damages.craterMult;
	result.damages.craterBoost = weaponDef->damages.craterBoost;
	result.damages.defaultDamage = weaponDef->damages.GetDefault();

	return result;
}

} // namespace

const ProjectilesApi PROJECTILES_API = {
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
