#include "UnitsWeapons.h"

namespace {

// Scratch buffer
static thread_local char scratchBuffer[8192];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_IMPLEMENTED_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "UnitsWeapons API not yet fully implemented" };
static const Error INVALID_UNIT_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid unit or weapon ID" };

static void NativeGetUnitWeaponCount(const GetUnitWeaponCountQuery* query, GetUnitWeaponCountResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->count = 0;
}

static void NativeGetUnitMaxRange(const GetUnitMaxRangeQuery* query, GetUnitMaxRangeResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->maxRange = 0.0f;
}

static void NativeGetUnitWeaponState(const GetUnitWeaponStateQuery* query, GetUnitWeaponStateResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetUnitWeaponDamages(const GetUnitWeaponDamagesQuery* query, GetUnitWeaponDamagesResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->damages.damages = nullptr;
	result->damages.damageCount = 0;
}

static void NativeGetUnitWeaponVectors(const GetUnitWeaponVectorsQuery* query, GetUnitWeaponVectorsResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetUnitWeaponTryTarget(const GetUnitWeaponTryTargetQuery* query, GetUnitWeaponTryTargetResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->canTarget = false;
}

static void NativeGetUnitWeaponTestTarget(const GetUnitWeaponTestTargetQuery* query, GetUnitWeaponTestTargetResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->canTarget = false;
}

static void NativeGetUnitWeaponTestRange(const GetUnitWeaponTestRangeQuery* query, GetUnitWeaponTestRangeResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->inRange = false;
}

static void NativeGetUnitWeaponHaveFreeLineOfFire(const GetUnitWeaponHaveFreeLineOfFireQuery* query, GetUnitWeaponHaveFreeLineOfFireResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->hasFreeLineOfFire = false;
}

static void NativeGetUnitWeaponCanFire(const GetUnitWeaponCanFireQuery* query, GetUnitWeaponCanFireResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->canFire = false;
}

static void NativeGetUnitWeaponTarget(const GetUnitWeaponTargetQuery* query, GetUnitWeaponTargetResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->target.targetType = 0;
	result->target.targetID = -1;
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
