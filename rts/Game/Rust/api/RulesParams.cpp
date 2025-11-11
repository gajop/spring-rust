#include "RulesParams.h"

namespace {

// Error constants
static const Error NOT_IMPLEMENTED_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "RulesParams API not yet fully implemented"
};

// All functions return not-implemented stubs
// RulesParams system requires Lua integration which is complex

static RulesParamResult NativeGetRulesParam(RulesParamQuery query)
{
	RulesParamResult result = {};
	result.exists = false;
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

static RulesParamNamesResult NativeGetRulesParamNames(RulesParamNamesQuery query)
{
	RulesParamNamesResult result = {};
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

} // namespace

const RulesParamsApi RULES_PARAMS_API = {
	.GetRulesParam = NativeGetRulesParam,
	.GetRulesParamNames = NativeGetRulesParamNames,
};
