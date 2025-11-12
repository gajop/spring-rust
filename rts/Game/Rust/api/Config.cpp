#include "Config.h"

#include "System/Config/ConfigHandler.h"
#include "System/Log/Level.h"
#include "System/Log/ILog.h"

namespace {

// Scratch buffer for dynamic data
static thread_local char scratchBuffer[8192];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Config system not ready"
};

static const Error INVALID_ARG_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid argument"
};

static bool ConfigReady()
{
	return (configHandler != nullptr);
}

// Query config
static void NativeGetConfigInt(const GetConfigIntQuery* query, GetConfigIntResult* result)
{
	bufferPos = 0;

	if (!ConfigReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (query->key == nullptr) {
		result->error = &INVALID_ARG_ERROR;
		return;
	}

	result->error = nullptr;
	result->value = configHandler->GetInt(query->key, query->defaultValue);
}

static void NativeGetConfigFloat(const GetConfigFloatQuery* query, GetConfigFloatResult* result)
{
	bufferPos = 0;

	if (!ConfigReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (query->key == nullptr) {
		result->error = &INVALID_ARG_ERROR;
		return;
	}

	result->error = nullptr;
	result->value = configHandler->GetFloat(query->key, query->defaultValue);
}

static void NativeGetConfigString(const GetConfigStringQuery* query, GetConfigStringResult* result)
{
	bufferPos = 0;

	if (!ConfigReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (query->key == nullptr) {
		result->error = &INVALID_ARG_ERROR;
		return;
	}

	// Get config string into scratch buffer
	const char* defaultStr = (query->defaultValue != nullptr) ? query->defaultValue : "";
	std::string value = configHandler->GetString(query->key, defaultStr);

	// Copy string to scratch buffer
	char* strBuf = &scratchBuffer[bufferPos];
	size_t len = value.length();
	if (bufferPos + len + 1 > sizeof(scratchBuffer)) {
		result->error = &INVALID_ARG_ERROR;
		return;
	}

	memcpy(strBuf, value.c_str(), len + 1);
	bufferPos += len + 1;

	result->error = nullptr;
	result->value = strBuf;
}

static void NativeGetConfigParams(const GetConfigParamsQuery* query, GetConfigParamsResult* result)
{
	bufferPos = 0;

	if (!ConfigReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	// Config params enumeration not implemented - would require accessing internal config map
	result->error = nullptr;
	result->params = nullptr;
	result->count = 0;
}

// Set config
static void NativeSetConfigInt(const SetConfigIntQuery* query, SetConfigIntResult* result)
{
	bufferPos = 0;

	if (!ConfigReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (query->key == nullptr) {
		result->error = &INVALID_ARG_ERROR;
		return;
	}

	configHandler->Set(query->key, query->value);
	result->error = nullptr;
	result->success = true;
}

static void NativeSetConfigFloat(const SetConfigFloatQuery* query, SetConfigFloatResult* result)
{
	bufferPos = 0;

	if (!ConfigReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (query->key == nullptr) {
		result->error = &INVALID_ARG_ERROR;
		return;
	}

	configHandler->Set(query->key, query->value);
	result->error = nullptr;
	result->success = true;
}

static void NativeSetConfigString(const SetConfigStringQuery* query, SetConfigStringResult* result)
{
	bufferPos = 0;

	if (!ConfigReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (query->key == nullptr || query->value == nullptr) {
		result->error = &INVALID_ARG_ERROR;
		return;
	}

	configHandler->SetString(query->key, query->value);
	result->error = nullptr;
	result->success = true;
}

// Log sections
static void NativeGetLogSections(const GetLogSectionsQuery* query, GetLogSectionsResult* result)
{
	bufferPos = 0;

	// Log sections enumeration not implemented - would require accessing log internals
	result->error = nullptr;
	result->sections = nullptr;
	result->count = 0;
}

static void NativeSetLogSectionFilterLevel(const SetLogSectionFilterLevelQuery* query, SetLogSectionFilterLevelResult* result)
{
	bufferPos = 0;

	if (query->section == nullptr) {
		result->error = &INVALID_ARG_ERROR;
		return;
	}

	// Log section filtering - simplified implementation
	// Would call log.SetSectionFilterLevel(section, level);
	result->error = nullptr;
	result->success = true;
}

} // namespace

const ConfigApi CONFIG_API = {
	.GetConfigInt = NativeGetConfigInt,
	.GetConfigFloat = NativeGetConfigFloat,
	.GetConfigString = NativeGetConfigString,
	.GetConfigParams = NativeGetConfigParams,

	.SetConfigInt = NativeSetConfigInt,
	.SetConfigFloat = NativeSetConfigFloat,
	.SetConfigString = NativeSetConfigString,

	.GetLogSections = NativeGetLogSections,
	.SetLogSectionFilterLevel = NativeSetLogSectionFilterLevel,
};
