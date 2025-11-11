#include "UnitsWeapons.h"

#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Weapons/Weapon.h"
#include <vector>

namespace {

static const Error NOT_IMPLEMENTED_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "UnitsWeapons API partially implemented"
};

static const Error INVALID_UNIT_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid unit or weapon ID"
};

// Basic stubs - weapons system is complex
static UnitWeaponStateResult NativeGetUnitWeaponState(int32_t unitID, int32_t weaponNum)
{
	UnitWeaponStateResult result = {};
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

static UnitWeaponDamagesResult NativeGetUnitWeaponDamages(int32_t unitID, int32_t weaponNum)
{
	UnitWeaponDamagesResult result = {};
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

static UnitWeaponVectorsResult NativeGetUnitWeaponVectors(int32_t unitID, int32_t weaponNum)
{
	UnitWeaponVectorsResult result = {};
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

static UnitWeaponTargetResult NativeGetUnitWeaponTarget(int32_t unitID, int32_t weaponNum)
{
	UnitWeaponTargetResult result = {};
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

} // namespace

const UnitsWeaponsApi UNITS_WEAPONS_API = {
	.GetUnitWeaponState = NativeGetUnitWeaponState,
	.GetUnitWeaponDamages = NativeGetUnitWeaponDamages,
	.GetUnitWeaponVectors = NativeGetUnitWeaponVectors,
	.GetUnitWeaponTarget = NativeGetUnitWeaponTarget,
};
