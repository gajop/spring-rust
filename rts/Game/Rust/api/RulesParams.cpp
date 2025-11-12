#include "RulesParams.h"

namespace {

// Scratch buffer
static thread_local char scratchBuffer[8192];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_IMPLEMENTED_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "RulesParams API not yet fully implemented" };

// All functions return not-implemented stubs
// RulesParams system requires Lua integration which is complex

static void NativeGetGameRulesParam(const GetGameRulesParamQuery* query, GetGameRulesParamResult* result) {
	bufferPos = 0;
	result->exists = false;
	result->los = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetGameRulesParams(const GetGameRulesParamsQuery* query, GetGameRulesParamsResult* result) {
	bufferPos = 0;
	result->names = nullptr;
	result->count = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetTeamRulesParam(const GetTeamRulesParamQuery* query, GetTeamRulesParamResult* result) {
	bufferPos = 0;
	result->exists = false;
	result->los = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetTeamRulesParams(const GetTeamRulesParamsQuery* query, GetTeamRulesParamsResult* result) {
	bufferPos = 0;
	result->names = nullptr;
	result->count = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetPlayerRulesParam(const GetPlayerRulesParamQuery* query, GetPlayerRulesParamResult* result) {
	bufferPos = 0;
	result->exists = false;
	result->los = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetPlayerRulesParams(const GetPlayerRulesParamsQuery* query, GetPlayerRulesParamsResult* result) {
	bufferPos = 0;
	result->names = nullptr;
	result->count = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetUnitRulesParam(const GetUnitRulesParamQuery* query, GetUnitRulesParamResult* result) {
	bufferPos = 0;
	result->exists = false;
	result->los = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetUnitRulesParams(const GetUnitRulesParamsQuery* query, GetUnitRulesParamsResult* result) {
	bufferPos = 0;
	result->names = nullptr;
	result->count = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetFeatureRulesParam(const GetFeatureRulesParamQuery* query, GetFeatureRulesParamResult* result) {
	bufferPos = 0;
	result->exists = false;
	result->los = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetFeatureRulesParams(const GetFeatureRulesParamsQuery* query, GetFeatureRulesParamsResult* result) {
	bufferPos = 0;
	result->names = nullptr;
	result->count = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeSetGameRulesParam(const SetGameRulesParamQuery* query, SetGameRulesParamResult* result) {
	bufferPos = 0;
	result->success = false;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeSetTeamRulesParam(const SetTeamRulesParamQuery* query, SetTeamRulesParamResult* result) {
	bufferPos = 0;
	result->success = false;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeSetPlayerRulesParam(const SetPlayerRulesParamQuery* query, SetPlayerRulesParamResult* result) {
	bufferPos = 0;
	result->success = false;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeSetUnitRulesParam(const SetUnitRulesParamQuery* query, SetUnitRulesParamResult* result) {
	bufferPos = 0;
	result->success = false;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeSetFeatureRulesParam(const SetFeatureRulesParamQuery* query, SetFeatureRulesParamResult* result) {
	bufferPos = 0;
	result->success = false;
	result->error = &NOT_IMPLEMENTED_ERROR;
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
