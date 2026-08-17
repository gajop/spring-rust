#include "LOS.h"

#include "NativeInterface/WasmUiVisibility.h"

#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Misc/LosHandler.h"
#include "Sim/Misc/GlobalSynced.h"
#include "Sim/Misc/TeamHandler.h"
#include "Map/ReadMap.h"
#include "System/float3.h"

namespace {

// Scratch buffer
static thread_local char scratchBuffer[1024];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "LOS system not ready" };
static const Error INVALID_ALLY_TEAM_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid ally team ID" };
static const Error INVALID_UNIT_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid unit ID" };
static const Error NOT_IMPLEMENTED_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "GetClosestValidPosition is not implemented by the Lua API" };

static bool IsReady() { return (gs != nullptr) && (losHandler != nullptr); }

static void NativeGetPositionLosState(const GetPositionLosStateQuery* query, GetPositionLosStateResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	if (!teamHandler.IsValidAllyTeam(query->allyTeamID)) {
		result->error = &INVALID_ALLY_TEAM_ERROR;
		return;
	}
	if (!WasmUiVisibility::IsLosPerspectiveAllowed(query->allyTeamID)) {
		result->error = &INVALID_ALLY_TEAM_ERROR;
		return;
	}

	const float3 position(query->pos.x, query->pos.y, query->pos.z);
	result->error = nullptr;
	result->state.inLos = losHandler->InLos(position, query->allyTeamID);
	result->state.inRadar = losHandler->InRadar(position, query->allyTeamID);
	result->state.inJammer = losHandler->InJammer(position, query->allyTeamID);
	result->state.inLosOrRadar = result->state.inLos || result->state.inRadar;
}

static void NativeIsPosInLos(const IsPosInLosQuery* query, IsPosInLosResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	if (!teamHandler.IsValidAllyTeam(query->allyTeamID)) {
		result->error = &INVALID_ALLY_TEAM_ERROR;
		return;
	}
	if (!WasmUiVisibility::IsLosPerspectiveAllowed(query->allyTeamID)) {
		result->error = &INVALID_ALLY_TEAM_ERROR;
		return;
	}

	const float3 position(query->pos.x, query->pos.y, query->pos.z);
	result->error = nullptr;
	result->inLos = losHandler->InLos(position, query->allyTeamID);
}

static void NativeIsPosInRadar(const IsPosInRadarQuery* query, IsPosInRadarResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	if (!teamHandler.IsValidAllyTeam(query->allyTeamID)) {
		result->error = &INVALID_ALLY_TEAM_ERROR;
		return;
	}
	if (!WasmUiVisibility::IsLosPerspectiveAllowed(query->allyTeamID)) {
		result->error = &INVALID_ALLY_TEAM_ERROR;
		return;
	}

	const float3 position(query->pos.x, query->pos.y, query->pos.z);
	result->error = nullptr;
	result->inRadar = losHandler->InRadar(position, query->allyTeamID);
}

static void NativeIsPosInAirLos(const IsPosInAirLosQuery* query, IsPosInAirLosResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	if (!teamHandler.IsValidAllyTeam(query->allyTeamID)) {
		result->error = &INVALID_ALLY_TEAM_ERROR;
		return;
	}
	if (!WasmUiVisibility::IsLosPerspectiveAllowed(query->allyTeamID)) {
		result->error = &INVALID_ALLY_TEAM_ERROR;
		return;
	}

	const float3 position(query->pos.x, query->pos.y, query->pos.z);
	result->error = nullptr;
	result->inAirLos = losHandler->InAirLos(position, query->allyTeamID);
}

static void NativeIsUnitInLos(const IsUnitInLosQuery* query, IsUnitInLosResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	if (!teamHandler.IsValidAllyTeam(query->allyTeamID)) {
		result->error = &INVALID_ALLY_TEAM_ERROR;
		return;
	}
	if (!WasmUiVisibility::IsLosPerspectiveAllowed(query->allyTeamID)) {
		result->error = &INVALID_ALLY_TEAM_ERROR;
		return;
	}

	const CUnit* unit = WasmUiVisibility::FindUnit(query->unitID, WasmUiVisibility::UnitAccess::Typed);
	if (unit == nullptr) { result->error = &INVALID_UNIT_ERROR; return; }

	result->error = nullptr;
	result->inLos = losHandler->InLos(unit, query->allyTeamID);
}

static void NativeIsUnitInAirLos(const IsUnitInAirLosQuery* query, IsUnitInAirLosResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	if (!teamHandler.IsValidAllyTeam(query->allyTeamID)) {
		result->error = &INVALID_ALLY_TEAM_ERROR;
		return;
	}
	if (!WasmUiVisibility::IsLosPerspectiveAllowed(query->allyTeamID)) {
		result->error = &INVALID_ALLY_TEAM_ERROR;
		return;
	}

	const CUnit* unit = WasmUiVisibility::FindUnit(query->unitID, WasmUiVisibility::UnitAccess::Typed);
	if (unit == nullptr) { result->error = &INVALID_UNIT_ERROR; return; }

	result->error = nullptr;
	result->inAirLos = losHandler->InAirLos(unit, query->allyTeamID);
}

static void NativeIsUnitInRadar(const IsUnitInRadarQuery* query, IsUnitInRadarResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	if (!teamHandler.IsValidAllyTeam(query->allyTeamID)) {
		result->error = &INVALID_ALLY_TEAM_ERROR;
		return;
	}
	if (!WasmUiVisibility::IsLosPerspectiveAllowed(query->allyTeamID)) {
		result->error = &INVALID_ALLY_TEAM_ERROR;
		return;
	}

	const CUnit* unit = WasmUiVisibility::FindUnit(query->unitID, WasmUiVisibility::UnitAccess::Typed);
	if (unit == nullptr) { result->error = &INVALID_UNIT_ERROR; return; }

	result->error = nullptr;
	result->inRadar = losHandler->InRadar(unit, query->allyTeamID);
}

static void NativeIsUnitInJammer(const IsUnitInJammerQuery* query, IsUnitInJammerResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	if (!teamHandler.IsValidAllyTeam(query->allyTeamID)) {
		result->error = &INVALID_ALLY_TEAM_ERROR;
		return;
	}
	if (!WasmUiVisibility::IsLosPerspectiveAllowed(query->allyTeamID)) {
		result->error = &INVALID_ALLY_TEAM_ERROR;
		return;
	}

	const CUnit* unit = WasmUiVisibility::FindUnit(query->unitID, WasmUiVisibility::UnitAccess::Typed);
	if (unit == nullptr) { result->error = &INVALID_UNIT_ERROR; return; }

	result->error = nullptr;
	result->inJammer = losHandler->InJammer(unit, query->allyTeamID);
}

static void NativeGetRadarErrorParams(const GetRadarErrorParamsQuery* query, GetRadarErrorParamsResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	if (!teamHandler.IsValidAllyTeam(query->allyTeamID)) {
		result->error = &INVALID_ALLY_TEAM_ERROR;
		return;
	}
	if (!WasmUiVisibility::IsLosPerspectiveAllowed(query->allyTeamID)) {
		result->error = &INVALID_ALLY_TEAM_ERROR;
		return;
	}

	result->error = nullptr;
	result->params.radarErrorSize = losHandler->GetAllyTeamRadarErrorSize(query->allyTeamID);
	result->params.baseRadarErrorSize = losHandler->GetBaseRadarErrorSize();
	result->params.baseRadarErrorMult = losHandler->GetBaseRadarErrorMult();
}

static void NativeGetClosestValidPosition(const GetClosestValidPositionQuery* query, GetClosestValidPositionResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	result->error = &NOT_IMPLEMENTED_ERROR;
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
