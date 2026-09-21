#include "UnitsWeapons.h"

#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Weapons/Weapon.h"
#include "Sim/Weapons/WeaponDef.h"
#include "Sim/Projectiles/WeaponProjectiles/WeaponProjectile.h"
#include "Sim/Misc/GlobalConstants.h"
#include "Sim/Misc/GlobalSynced.h"
#include "System/float3.h"

#include <vector>

namespace {

static thread_local std::vector<float> damageValues;

// Static errors
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Unit system not ready" };
static const Error INVALID_UNIT_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid unit ID" };
static const Error INVALID_WEAPON_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid weapon number" };

static bool IsReady() {
	return (gs != nullptr);
}

static const CWeapon* GetWeaponByOneBasedNumber(const CUnit* unit, int32_t weaponNum)
{
	const int weaponIndex = weaponNum - 1;
	if (weaponIndex < 0 || weaponIndex >= static_cast<int>(unit->weapons.size()))
		return nullptr;

	return unit->weapons[weaponIndex];
}

static void NativeGetUnitWeaponCount(const GetUnitWeaponCountQuery* query, GetUnitWeaponCountResult* result)
{
	result->error = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->count = unit->weapons.size();
}

static void NativeGetUnitMaxRange(const GetUnitMaxRangeQuery* query, GetUnitMaxRangeResult* result)
{
	result->error = nullptr;
	result->maxRange = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->maxRange = unit->maxRange;
}

static void NativeGetUnitWeaponState(const GetUnitWeaponStateQuery* query, GetUnitWeaponStateResult* result)
{
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	// The whole state struct is returned at once, so the key only selects a field
	// on the guest side.
	(void)query->key;

	const CWeapon* weapon = GetWeaponByOneBasedNumber(unit, query->weaponNum);
	if (weapon == nullptr || weapon->weaponDef == nullptr) {
		result->error = &INVALID_WEAPON_ERROR;
		return;
	}

	// The live reload, not weaponDef->reload: SetUnitWeaponState writes
	// weapon->reloadTime, so reading the def value made every write look lost.
	// Stored in frames, reported in seconds, matching Spring.GetUnitWeaponState.
	result->state.reloadTime = weapon->reloadTime * INV_GAME_SPEED;
	result->state.reloadFrame = weapon->reloadStatus;
	result->state.range = weapon->range;
	result->state.projectileSpeed = weapon->projectileSpeed;
	result->state.accuracy = weapon->AccuracyExperience();
	result->state.sprayAngle = weapon->SprayAngleExperience();
	result->state.aimFromHeight = weapon->aimFromPos.y;
	result->state.salvoSize = weapon->salvoSize;
	result->state.salvoDelay = weapon->salvoDelay * INV_GAME_SPEED;
	result->state.salvoError = weapon->salvoError.Length();
	result->state.targetMoveError = weapon->MoveErrorExperience();
	result->state.turnRate = 0.0f; // Not easily accessible
	result->state.autoTarget = !weapon->noAutoTarget;
}

static void NativeGetUnitWeaponDamages(const GetUnitWeaponDamagesQuery* query, GetUnitWeaponDamagesResult* result)
{
	result->error = nullptr;
	result->damages.damages = nullptr;
	result->damages.damageCount = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const CWeapon* weapon = GetWeaponByOneBasedNumber(unit, query->weaponNum);
	if (weapon == nullptr || weapon->damages == nullptr) {
		result->error = &INVALID_WEAPON_ERROR;
		return;
	}

	// Read the live weapon damage array. SetUnitWeaponDamages mutates this
	// array; the weapon definition contains only its initial values.
	const DynDamageArray& damages = *weapon->damages;

	damageValues.clear();
	damageValues.reserve(damages.GetNumTypes());
	for (int i = 0; i < damages.GetNumTypes(); i++)
		damageValues.push_back(damages.Get(i));

	result->damages.damages = damageValues.empty() ? nullptr : damageValues.data();
	result->damages.damageCount = damageValues.size();
	result->damages.paralyzeDamageTime = damages.paralyzeDamageTime;
	result->damages.impulseFactor = damages.impulseFactor;
	result->damages.impulseBoost = damages.impulseBoost;
	result->damages.craterMult = damages.craterMult;
	result->damages.craterBoost = damages.craterBoost;
	result->damages.defaultDamage = damages.GetDefault();
}

static void NativeGetUnitWeaponVectors(const GetUnitWeaponVectorsQuery* query, GetUnitWeaponVectorsResult* result)
{
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const CWeapon* weapon = GetWeaponByOneBasedNumber(unit, query->weaponNum);
	if (weapon == nullptr) {
		result->error = &INVALID_WEAPON_ERROR;
		return;
	}

	result->vectors.weaponMuzzlePos.x = weapon->weaponMuzzlePos.x;
	result->vectors.weaponMuzzlePos.y = weapon->weaponMuzzlePos.y;
	result->vectors.weaponMuzzlePos.z = weapon->weaponMuzzlePos.z;

	result->vectors.weaponAimPos.x = weapon->aimFromPos.x;
	result->vectors.weaponAimPos.y = weapon->aimFromPos.y;
	result->vectors.weaponAimPos.z = weapon->aimFromPos.z;

	const float3* dir = &weapon->wantedDir;
	switch (weapon->weaponDef->projectileType) {
		case WEAPON_MISSILE_PROJECTILE:
		case WEAPON_TORPEDO_PROJECTILE:
		case WEAPON_STARBURST_PROJECTILE:
			dir = &weapon->weaponDir;
			break;
		default:
			break;
	}

	result->vectors.weaponDir.x = dir->x;
	result->vectors.weaponDir.y = dir->y;
	result->vectors.weaponDir.z = dir->z;
}

static void NativeGetUnitWeaponTryTarget(const GetUnitWeaponTryTargetQuery* query, GetUnitWeaponTryTargetResult* result)
{
	result->error = nullptr;
	result->canTarget = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const CWeapon* weapon = GetWeaponByOneBasedNumber(unit, query->weaponNum);
	if (weapon == nullptr) {
		result->error = &INVALID_WEAPON_ERROR;
		return;
	}

	const float3 targetPos(query->targetPos.x, query->targetPos.y, query->targetPos.z);
	SWeaponTarget target;
	if (query->options.isGroundTarget) {
		// Keep the explicit position in the target.  The old native path
		// discarded query->targetPos and consequently tested every ground
		// target at the origin.
		target = SWeaponTarget(targetPos, query->options.userTarget);
	} else if (query->targetID >= 0) {
		const CUnit* targetUnit = unitHandler.GetUnit(query->targetID);
		if (targetUnit == nullptr)
			return;
		target = SWeaponTarget(targetUnit, query->options.userTarget);
	} else {
		return; // Invalid target
	}

	result->canTarget = weapon->TryTarget(target);
}

static void NativeGetUnitWeaponTestTarget(const GetUnitWeaponTestTargetQuery* query, GetUnitWeaponTestTargetResult* result)
{
	result->error = nullptr;
	result->canTarget = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const CWeapon* weapon = GetWeaponByOneBasedNumber(unit, query->weaponNum);
	if (weapon == nullptr) {
		result->error = &INVALID_WEAPON_ERROR;
		return;
	}

	SWeaponTarget target;
	float3 tgtPos(query->targetPos.x, query->targetPos.y, query->targetPos.z);

	if (query->options.isGroundTarget) {
		target.type = Target_Pos;
		target.groundPos = tgtPos;
	} else if (query->targetID >= 0) {
		const CUnit* targetUnit = unitHandler.GetUnit(query->targetID);
		if (targetUnit == nullptr)
			return;
		tgtPos = weapon->GetUnitLeadTargetPos(targetUnit);
		target = SWeaponTarget(targetUnit, tgtPos, true);
	} else {
		return; // Invalid target
	}

	result->canTarget = weapon->TestTarget(tgtPos, target);
}

static void NativeGetUnitWeaponTestRange(const GetUnitWeaponTestRangeQuery* query, GetUnitWeaponTestRangeResult* result)
{
	result->error = nullptr;
	result->inRange = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const CWeapon* weapon = GetWeaponByOneBasedNumber(unit, query->weaponNum);
	if (weapon == nullptr) {
		result->error = &INVALID_WEAPON_ERROR;
		return;
	}

	float3 tgtPos(query->targetPos.x, query->targetPos.y, query->targetPos.z);
	SWeaponTarget target;
	target.type = Target_Pos;
	target.groundPos = tgtPos;

	result->inRange = weapon->TestRange(tgtPos, target);
}

static void NativeGetUnitWeaponHaveFreeLineOfFire(const GetUnitWeaponHaveFreeLineOfFireQuery* query, GetUnitWeaponHaveFreeLineOfFireResult* result)
{
	result->error = nullptr;
	result->hasFreeLineOfFire = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const CWeapon* weapon = GetWeaponByOneBasedNumber(unit, query->weaponNum);
	if (weapon == nullptr) {
		result->error = &INVALID_WEAPON_ERROR;
		return;
	}

	SWeaponTarget target;
	float3 srcPos(query->sourcePos.x, query->sourcePos.y, query->sourcePos.z);
	float3 tgtPos(query->targetPos.x, query->targetPos.y, query->targetPos.z);

	if (query->options.isGroundTarget) {
		target.type = Target_Pos;
		target.groundPos = tgtPos;
	} else if (query->targetID >= 0) {
		const CUnit* targetUnit = unitHandler.GetUnit(query->targetID);
		if (targetUnit == nullptr)
			return;
		tgtPos = weapon->GetUnitLeadTargetPos(targetUnit);
		target = SWeaponTarget(targetUnit, tgtPos, true);
	} else {
		return; // Invalid target
	}

	result->hasFreeLineOfFire = weapon->HaveFreeLineOfFire(srcPos, tgtPos, target);
}

static void NativeGetUnitWeaponCanFire(const GetUnitWeaponCanFireQuery* query, GetUnitWeaponCanFireResult* result)
{
	result->error = nullptr;
	result->canFire = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const CWeapon* weapon = GetWeaponByOneBasedNumber(unit, query->weaponNum);
	if (weapon == nullptr) {
		result->error = &INVALID_WEAPON_ERROR;
		return;
	}

	result->canFire = weapon->CanFire(false, false, false);
}

static void NativeGetUnitWeaponTarget(const GetUnitWeaponTargetQuery* query, GetUnitWeaponTargetResult* result)
{
	result->error = nullptr;
	result->target.targetType = 0; // No target
	result->target.targetID = -1;
	result->target.targetPos = {0.0f, 0.0f, 0.0f};

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const CWeapon* weapon = GetWeaponByOneBasedNumber(unit, query->weaponNum);
	if (weapon == nullptr) {
		result->error = &INVALID_WEAPON_ERROR;
		return;
	}

	const SWeaponTarget& target = weapon->GetCurrentTarget();
	const float3& targetPos = weapon->GetCurrentTargetPos();

	switch (target.type) {
		case Target_None:
			result->target.targetType = 0;
			result->target.targetID = -1;
			break;
		case Target_Unit:
			result->target.targetType = 1;
			result->target.targetID = (target.unit != nullptr) ? target.unit->id : -1;
			result->target.targetPos.x = targetPos.x;
			result->target.targetPos.y = targetPos.y;
			result->target.targetPos.z = targetPos.z;
			break;
		case Target_Pos:
			result->target.targetType = 2;
			result->target.targetID = -1;
			result->target.targetPos.x = targetPos.x;
			result->target.targetPos.y = targetPos.y;
			result->target.targetPos.z = targetPos.z;
			break;
		case Target_Intercept:
			result->target.targetType = 3;
			result->target.targetID = (target.intercept != nullptr) ? target.intercept->id : -1;
			result->target.targetPos.x = targetPos.x;
			result->target.targetPos.y = targetPos.y;
			result->target.targetPos.z = targetPos.z;
			break;
		default:
			result->target.targetType = 0;
			result->target.targetID = -1;
			break;
	}
}

} // namespace

const UnitsWeaponsApi UNITS_WEAPONS_API = {
	.GetUnitWeaponCount = NativeGetUnitWeaponCount,
	.GetUnitMaxRange = NativeGetUnitMaxRange,
	.GetUnitWeaponState = NativeGetUnitWeaponState,
	.GetUnitWeaponDamages = NativeGetUnitWeaponDamages,
	.GetUnitWeaponVectors = NativeGetUnitWeaponVectors,
	.GetUnitWeaponTryTarget = NativeGetUnitWeaponTryTarget,
	.GetUnitWeaponTestTarget = NativeGetUnitWeaponTestTarget,
	.GetUnitWeaponTestRange = NativeGetUnitWeaponTestRange,
	.GetUnitWeaponHaveFreeLineOfFire = NativeGetUnitWeaponHaveFreeLineOfFire,
	.GetUnitWeaponCanFire = NativeGetUnitWeaponCanFire,
	.GetUnitWeaponTarget = NativeGetUnitWeaponTarget,
};
