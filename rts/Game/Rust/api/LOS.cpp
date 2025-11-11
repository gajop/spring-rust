#include "LOS.h"

#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Misc/LosHandler.h"
#include "Sim/Misc/GlobalSynced.h"
#include "Sim/Misc/TeamHandler.h"
#include "Map/ReadMap.h"
#include "System/float3.h"

namespace {

// Error constants
static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "LOS system not ready"
};

static const Error INVALID_ALLY_TEAM_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid ally team ID"
};

static const Error INVALID_UNIT_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid unit ID"
};

// Helper: check if ready
static bool IsReady()
{
	return (gs != nullptr) && (losHandler != nullptr);
}

// Position-based LOS
static PositionLosStateResult NativeGetPositionLosState(Float3 pos, int32_t allyTeamID)
{
	PositionLosStateResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidAllyTeam(allyTeamID)) {
		result.error = &INVALID_ALLY_TEAM_ERROR;
		return result;
	}

	const float3 position(pos.x, pos.y, pos.z);
	result.state.los = losHandler->InLos(position, allyTeamID);
	result.state.radar = losHandler->InRadar(position, allyTeamID);
	result.state.prevLos = false; // Not directly available

	return result;
}

static BoolResult NativeIsPosInLos(Float3 pos, int32_t allyTeamID)
{
	BoolResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidAllyTeam(allyTeamID)) {
		result.error = &INVALID_ALLY_TEAM_ERROR;
		return result;
	}

	const float3 position(pos.x, pos.y, pos.z);
	result.value = losHandler->InLos(position, allyTeamID);
	return result;
}

static BoolResult NativeIsPosInRadar(Float3 pos, int32_t allyTeamID)
{
	BoolResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidAllyTeam(allyTeamID)) {
		result.error = &INVALID_ALLY_TEAM_ERROR;
		return result;
	}

	const float3 position(pos.x, pos.y, pos.z);
	result.value = losHandler->InRadar(position, allyTeamID);
	return result;
}

static BoolResult NativeIsPosInAirLos(Float3 pos, int32_t allyTeamID)
{
	BoolResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidAllyTeam(allyTeamID)) {
		result.error = &INVALID_ALLY_TEAM_ERROR;
		return result;
	}

	const float3 position(pos.x, pos.y, pos.z);
	result.value = losHandler->InAirLos(position, allyTeamID);
	return result;
}

// Unit-based LOS
static BoolResult NativeIsUnitInLos(int32_t unitID, int32_t allyTeamID)
{
	BoolResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidAllyTeam(allyTeamID)) {
		result.error = &INVALID_ALLY_TEAM_ERROR;
		return result;
	}

	const CUnit* unit = unitHandler.GetUnit(unitID);
	if (unit == nullptr) {
		result.error = &INVALID_UNIT_ERROR;
		return result;
	}

	result.value = losHandler->InLos(unit, allyTeamID);
	return result;
}

static BoolResult NativeIsUnitInAirLos(int32_t unitID, int32_t allyTeamID)
{
	BoolResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidAllyTeam(allyTeamID)) {
		result.error = &INVALID_ALLY_TEAM_ERROR;
		return result;
	}

	const CUnit* unit = unitHandler.GetUnit(unitID);
	if (unit == nullptr) {
		result.error = &INVALID_UNIT_ERROR;
		return result;
	}

	result.value = losHandler->InAirLos(unit, allyTeamID);
	return result;
}

static BoolResult NativeIsUnitInRadar(int32_t unitID, int32_t allyTeamID)
{
	BoolResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidAllyTeam(allyTeamID)) {
		result.error = &INVALID_ALLY_TEAM_ERROR;
		return result;
	}

	const CUnit* unit = unitHandler.GetUnit(unitID);
	if (unit == nullptr) {
		result.error = &INVALID_UNIT_ERROR;
		return result;
	}

	result.value = losHandler->InRadar(unit, allyTeamID);
	return result;
}

static BoolResult NativeIsUnitInJammer(int32_t unitID, int32_t allyTeamID)
{
	BoolResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidAllyTeam(allyTeamID)) {
		result.error = &INVALID_ALLY_TEAM_ERROR;
		return result;
	}

	const CUnit* unit = unitHandler.GetUnit(unitID);
	if (unit == nullptr) {
		result.error = &INVALID_UNIT_ERROR;
		return result;
	}

	result.value = losHandler->InJammer(unit, allyTeamID);
	return result;
}

// Radar error
static RadarErrorParamsResult NativeGetRadarErrorParams(int32_t allyTeamID)
{
	RadarErrorParamsResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidAllyTeam(allyTeamID)) {
		result.error = &INVALID_ALLY_TEAM_ERROR;
		return result;
	}

	// Default radar error params (actual values would come from modRules)
	result.params.baseErrMult = 1.0f;
	result.params.baseErrSize = 0.0f;
	result.params.errorMult = 0.1f;
	result.params.errorSize = 10.0f;
	result.params.baseSpeed = 0.0f;
	result.params.speedMult = 0.0f;

	return result;
}

// Closest valid position (for placement)
static Float3Result NativeGetClosestValidPosition(ClosestValidPositionQuery query)
{
	Float3Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Simplified - just return the input position
	// Full implementation would check terrain, blocking, etc.
	result.value.x = query.pos.x;
	result.value.y = query.pos.y;
	result.value.z = query.pos.z;

	return result;
}

} // namespace

const LOSApi LOS_API = {
	.GetPositionLosState = NativeGetPositionLosState,
	.IsPosInLos = NativeIsPosInLos,
	.IsPosInRadar = NativeIsPosInRadar,
	.IsPosInAirLos = NativeIsPosInAirLos,

	.IsUnitInLos = NativeIsUnitInLos,
	.IsUnitInAirLos = NativeIsUnitInAirLos,
	.IsUnitInRadar = NativeIsUnitInRadar,
	.IsUnitInJammer = NativeIsUnitInJammer,

	.GetRadarErrorParams = NativeGetRadarErrorParams,

	.GetClosestValidPosition = NativeGetClosestValidPosition,
};
