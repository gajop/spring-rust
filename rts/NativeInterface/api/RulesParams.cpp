#include "RulesParams.h"

#include "NativeInterface/WasmUiVisibility.h"

#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Features/Feature.h"
#include "Sim/Features/FeatureHandler.h"
#include "Sim/Misc/Team.h"
#include "Sim/Misc/TeamHandler.h"
#include "Sim/Misc/GlobalSynced.h"
#include "Game/Players/Player.h"
#include "Game/Players/PlayerHandler.h"
#include "Game/Game.h"
#include "Game/GlobalUnsynced.h"
#include "Lua/LuaHandleSynced.h"
#include "Lua/LuaRulesParams.h"
#include <cstring>
#include <variant>

namespace {

// Scratch buffer
static thread_local char scratchBuffer[1024];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Game not ready" };
static const Error INVALID_ID_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid ID" };
static const Error BUFFER_OVERFLOW_ERROR = { .code = ERROR_BUFFER_OVERFLOW, .message = "Buffer overflow" };

static bool IsReady() {
	return (gs != nullptr);
}

// Helper to allocate from scratch buffer
template<typename T>
static T* AllocateArray(size_t count) {
	size_t needed = count * sizeof(T);
	if (bufferPos + needed > sizeof(scratchBuffer)) {
		return nullptr;
	}
	T* ptr = reinterpret_cast<T*>(&scratchBuffer[bufferPos]);
	bufferPos += needed;
	return ptr;
}

// Helper to copy string to scratch buffer
static const char* CopyString(const std::string& str) {
	size_t len = str.length() + 1;
	if (bufferPos + len > sizeof(scratchBuffer)) {
		return nullptr;
	}
	char* ptr = &scratchBuffer[bufferPos];
	memcpy(ptr, str.c_str(), len);
	bufferPos += len;
	return ptr;
}

static void ResetParamValue(RulesParamValue& value) {
	value.type = RULESPARAM_TYPE_BOOL;
	value.boolValue = false;
}

// Helper to convert LuaRulesParams::Param to RulesParamValue
static bool ConvertParam(const LuaRulesParams::Param& param, RulesParamValue& outValue) {
	if (std::holds_alternative<bool>(param.value)) {
		outValue.type = RULESPARAM_TYPE_BOOL;
		outValue.boolValue = std::get<bool>(param.value);
	} else if (std::holds_alternative<float>(param.value)) {
		outValue.type = RULESPARAM_TYPE_FLOAT;
		outValue.floatValue = std::get<float>(param.value);
	} else if (std::holds_alternative<std::string>(param.value)) {
		outValue.type = RULESPARAM_TYPE_STRING;
		outValue.stringValue = CopyString(std::get<std::string>(param.value));
		return outValue.stringValue != nullptr;
	}
	return true;
}

static int TeamRulesParamMask(int teamID) {
	if (!WasmUiVisibility::Active() || WasmUiVisibility::FullRead() ||
		(game != nullptr && game->IsGameOver()) ||
		(WasmUiVisibility::ReadAllyTeam() >= 0 &&
			teamHandler.AllyTeam(teamID) == WasmUiVisibility::ReadAllyTeam()))
		return RULESPARAMLOS_PRIVATE_MASK;

	if (WasmUiVisibility::ReadTeam() >= 0 &&
		teamHandler.AlliedTeams(teamID, WasmUiVisibility::ReadTeam()))
		return RULESPARAMLOS_ALLIED_MASK;

	return RULESPARAMLOS_PUBLIC_MASK;
}

static int PlayerRulesParamMask(int playerID) {
	if (!WasmUiVisibility::Active() || WasmUiVisibility::FullRead() ||
		(game != nullptr && game->IsGameOver()) ||
		playerID == WasmUiVisibility::ReadPlayer())
		return RULESPARAMLOS_PRIVATE_MASK;

	return RULESPARAMLOS_PUBLIC_MASK;
}

template<typename Params>
static size_t CountVisibleParams(const Params& params, int allowedMask) {
	size_t count = 0;
	for (const auto& pair : params) {
		if (WasmUiVisibility::RulesParamVisible(pair.second.los, allowedMask))
			++count;
	}
	return count;
}

template<typename Params>
static bool WriteVisibleParamNames(const Params& params, int allowedMask,
	const char**& names, uint32_t& count, const Error*& error) {
	const size_t visibleCount = CountVisibleParams(params, allowedMask);
	if (visibleCount == 0)
		return true;

	names = AllocateArray<const char*>(visibleCount);
	if (names == nullptr) {
		error = &BUFFER_OVERFLOW_ERROR;
		return false;
	}

	uint32_t index = 0;
	for (const auto& pair : params) {
		if (!WasmUiVisibility::RulesParamVisible(pair.second.los, allowedMask))
			continue;
		names[index] = CopyString(pair.first);
		if (names[index] == nullptr) {
			error = &BUFFER_OVERFLOW_ERROR;
			count = index;
			return false;
		}
		++index;
	}

	count = index;
	return true;
}

static void NativeGetGameRulesParam(const GetGameRulesParamQuery* query, GetGameRulesParamResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	ResetParamValue(result->value);
	result->exists = false;
	result->los = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const LuaRulesParams::Params& params = CSplitLuaHandle::GetGameParams();
	auto it = params.find(query->paramName);
	if (it != params.end() && WasmUiVisibility::RulesParamVisible(it->second.los,
		RULESPARAMLOS_PRIVATE_MASK)) {
		if (!ConvertParam(it->second, result->value)) {
			result->error = &BUFFER_OVERFLOW_ERROR;
			return;
		}
		result->los = it->second.los;
		result->exists = true;
	}
}

static void NativeGetGameRulesParams(const GetGameRulesParamsQuery* query, GetGameRulesParamsResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->names = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const LuaRulesParams::Params& params = CSplitLuaHandle::GetGameParams();
	if (params.empty()) {
		return;
	}

	WriteVisibleParamNames(params, RULESPARAMLOS_PRIVATE_MASK, result->names,
		result->count, result->error);
}

static void NativeGetTeamRulesParam(const GetTeamRulesParamQuery* query, GetTeamRulesParamResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	ResetParamValue(result->value);
	result->exists = false;
	result->los = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CTeam* team = teamHandler.IsValidTeam(query->teamID) ? teamHandler.Team(query->teamID) : nullptr;
	if (team == nullptr) {
		result->error = &INVALID_ID_ERROR;
		return;
	}

	auto it = team->modParams.find(query->paramName);
	const int allowedMask = TeamRulesParamMask(query->teamID);
	if (it != team->modParams.end() && WasmUiVisibility::RulesParamVisible(it->second.los,
		allowedMask)) {
		if (!ConvertParam(it->second, result->value)) {
			result->error = &BUFFER_OVERFLOW_ERROR;
			return;
		}
		result->los = it->second.los;
		result->exists = true;
	}
}

static void NativeGetTeamRulesParams(const GetTeamRulesParamsQuery* query, GetTeamRulesParamsResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->names = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CTeam* team = teamHandler.IsValidTeam(query->teamID) ? teamHandler.Team(query->teamID) : nullptr;
	if (team == nullptr) {
		result->error = &INVALID_ID_ERROR;
		return;
	}

	if (team->modParams.empty()) {
		return;
	}

	WriteVisibleParamNames(team->modParams, TeamRulesParamMask(query->teamID),
		result->names, result->count, result->error);
}

static void NativeGetPlayerRulesParam(const GetPlayerRulesParamQuery* query, GetPlayerRulesParamResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	ResetParamValue(result->value);
	result->exists = false;
	result->los = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CPlayer* player = playerHandler.IsValidPlayer(query->playerID) ? playerHandler.Player(query->playerID) : nullptr;
	if (player == nullptr) {
		result->error = &INVALID_ID_ERROR;
		return;
	}

	auto it = player->modParams.find(query->paramName);
	const int allowedMask = PlayerRulesParamMask(query->playerID);
	if (it != player->modParams.end() && WasmUiVisibility::RulesParamVisible(it->second.los,
		allowedMask)) {
		if (!ConvertParam(it->second, result->value)) {
			result->error = &BUFFER_OVERFLOW_ERROR;
			return;
		}
		result->los = it->second.los;
		result->exists = true;
	}
}

static void NativeGetPlayerRulesParams(const GetPlayerRulesParamsQuery* query, GetPlayerRulesParamsResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->names = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CPlayer* player = playerHandler.IsValidPlayer(query->playerID) ? playerHandler.Player(query->playerID) : nullptr;
	if (player == nullptr) {
		result->error = &INVALID_ID_ERROR;
		return;
	}

	if (player->modParams.empty()) {
		return;
	}

	WriteVisibleParamNames(player->modParams, PlayerRulesParamMask(query->playerID),
		result->names, result->count, result->error);
}

static void NativeGetUnitRulesParam(const GetUnitRulesParamQuery* query, GetUnitRulesParamResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	ResetParamValue(result->value);
	result->exists = false;
	result->los = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = WasmUiVisibility::FindUnit(query->unitID, WasmUiVisibility::UnitAccess::Visible);
	if (unit == nullptr) {
		result->error = &INVALID_ID_ERROR;
		return;
	}

	auto it = unit->modParams.find(query->paramName);
	if (it != unit->modParams.end() && WasmUiVisibility::RulesParamVisible(it->second.los,
		WasmUiVisibility::UnitRulesParamMask(unit))) {
		if (!ConvertParam(it->second, result->value)) {
			result->error = &BUFFER_OVERFLOW_ERROR;
			return;
		}
		result->los = it->second.los;
		result->exists = true;
	}
}

static void NativeGetUnitRulesParams(const GetUnitRulesParamsQuery* query, GetUnitRulesParamsResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->names = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = WasmUiVisibility::FindUnit(query->unitID, WasmUiVisibility::UnitAccess::Visible);
	if (unit == nullptr) {
		result->error = &INVALID_ID_ERROR;
		return;
	}

	if (unit->modParams.empty()) {
		return;
	}

	WriteVisibleParamNames(unit->modParams, WasmUiVisibility::UnitRulesParamMask(unit),
		result->names, result->count, result->error);
}

static void NativeGetFeatureRulesParam(const GetFeatureRulesParamQuery* query, GetFeatureRulesParamResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	ResetParamValue(result->value);
	result->exists = false;
	result->los = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = WasmUiVisibility::FindFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_ID_ERROR;
		return;
	}

	auto it = feature->modParams.find(query->paramName);
	if (it != feature->modParams.end() && WasmUiVisibility::RulesParamVisible(it->second.los,
		WasmUiVisibility::FeatureRulesParamMask(feature))) {
		if (!ConvertParam(it->second, result->value)) {
			result->error = &BUFFER_OVERFLOW_ERROR;
			return;
		}
		result->los = it->second.los;
		result->exists = true;
	}
}

static void NativeGetFeatureRulesParams(const GetFeatureRulesParamsQuery* query, GetFeatureRulesParamsResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->names = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = WasmUiVisibility::FindFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_ID_ERROR;
		return;
	}

	if (feature->modParams.empty()) {
		return;
	}

	WriteVisibleParamNames(feature->modParams, WasmUiVisibility::FeatureRulesParamMask(feature),
		result->names, result->count, result->error);
}

static void NativeSetGameRulesParam(const SetGameRulesParamQuery* query, SetGameRulesParamResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	LuaRulesParams::Params& params = const_cast<LuaRulesParams::Params&>(CSplitLuaHandle::GetGameParams());
	LuaRulesParams::Param param;
	param.los = query->los;

	switch (query->value.type) {
		case RULESPARAM_TYPE_BOOL:
			param.value = query->value.boolValue;
			break;
		case RULESPARAM_TYPE_FLOAT:
			param.value = query->value.floatValue;
			break;
		case RULESPARAM_TYPE_STRING:
			param.value = std::string(query->value.stringValue);
			break;
	}

	params[query->paramName] = param;
	result->success = true;
}

static void NativeSetTeamRulesParam(const SetTeamRulesParamQuery* query, SetTeamRulesParamResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CTeam* team = teamHandler.IsValidTeam(query->teamID) ? teamHandler.Team(query->teamID) : nullptr;
	if (team == nullptr) {
		result->error = &INVALID_ID_ERROR;
		return;
	}

	LuaRulesParams::Param param;
	param.los = query->los;

	switch (query->value.type) {
		case RULESPARAM_TYPE_BOOL:
			param.value = query->value.boolValue;
			break;
		case RULESPARAM_TYPE_FLOAT:
			param.value = query->value.floatValue;
			break;
		case RULESPARAM_TYPE_STRING:
			param.value = std::string(query->value.stringValue);
			break;
	}

	team->modParams[query->paramName] = param;
	result->success = true;
}

static void NativeSetPlayerRulesParam(const SetPlayerRulesParamQuery* query, SetPlayerRulesParamResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CPlayer* player = playerHandler.IsValidPlayer(query->playerID) ? playerHandler.Player(query->playerID) : nullptr;
	if (player == nullptr) {
		result->error = &INVALID_ID_ERROR;
		return;
	}

	LuaRulesParams::Param param;
	param.los = query->los;

	switch (query->value.type) {
		case RULESPARAM_TYPE_BOOL:
			param.value = query->value.boolValue;
			break;
		case RULESPARAM_TYPE_FLOAT:
			param.value = query->value.floatValue;
			break;
		case RULESPARAM_TYPE_STRING:
			param.value = std::string(query->value.stringValue);
			break;
	}

	player->modParams[query->paramName] = param;
	result->success = true;
}

static void NativeSetUnitRulesParam(const SetUnitRulesParamQuery* query, SetUnitRulesParamResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_ID_ERROR;
		return;
	}

	LuaRulesParams::Param param;
	param.los = query->los;

	switch (query->value.type) {
		case RULESPARAM_TYPE_BOOL:
			param.value = query->value.boolValue;
			break;
		case RULESPARAM_TYPE_FLOAT:
			param.value = query->value.floatValue;
			break;
		case RULESPARAM_TYPE_STRING:
			param.value = std::string(query->value.stringValue);
			break;
	}

	unit->modParams[query->paramName] = param;
	result->success = true;
}

static void NativeSetFeatureRulesParam(const SetFeatureRulesParamQuery* query, SetFeatureRulesParamResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_ID_ERROR;
		return;
	}

	LuaRulesParams::Param param;
	param.los = query->los;

	switch (query->value.type) {
		case RULESPARAM_TYPE_BOOL:
			param.value = query->value.boolValue;
			break;
		case RULESPARAM_TYPE_FLOAT:
			param.value = query->value.floatValue;
			break;
		case RULESPARAM_TYPE_STRING:
			param.value = std::string(query->value.stringValue);
			break;
	}

	feature->modParams[query->paramName] = param;
	result->success = true;
}

} // namespace

const RulesParamsApi RULES_PARAMS_API = {
	.GetGameRulesParam = NativeGetGameRulesParam,
	.GetGameRulesParams = NativeGetGameRulesParams,
	.GetTeamRulesParam = NativeGetTeamRulesParam,
	.GetTeamRulesParams = NativeGetTeamRulesParams,
	.GetPlayerRulesParam = NativeGetPlayerRulesParam,
	.GetPlayerRulesParams = NativeGetPlayerRulesParams,
	.GetUnitRulesParam = NativeGetUnitRulesParam,
	.GetUnitRulesParams = NativeGetUnitRulesParams,
	.GetFeatureRulesParam = NativeGetFeatureRulesParam,
	.GetFeatureRulesParams = NativeGetFeatureRulesParams,
	.SetGameRulesParam = NativeSetGameRulesParam,
	.SetTeamRulesParam = NativeSetTeamRulesParam,
	.SetPlayerRulesParam = NativeSetPlayerRulesParam,
	.SetUnitRulesParam = NativeSetUnitRulesParam,
	.SetFeatureRulesParam = NativeSetFeatureRulesParam,
};
