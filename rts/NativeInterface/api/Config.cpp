#include "Config.h"

#include "System/Config/ConfigHandler.h"
#include "System/Config/ConfigVariable.h"
#include "System/GlobalConfig.h"
#include "System/Log/DefaultFilter.h"
#include "System/Log/Level.h"
#include "System/Log/ILog.h"

#include <vector>

namespace {

// Scratch buffer for dynamic data
// NOTE: Reduced from 8192 to 1024 to avoid static initialization issues with sound thread
static thread_local char scratchBuffer[1024];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;
static thread_local std::vector<ConfigParam> configParamBuffer;
static thread_local std::vector<std::string> configStringBuffer;
static thread_local std::vector<const char*> logSectionBuffer;

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

static ConfigValueType ConfigTypeFromString(const std::string& type)
{
	if (type == "int")
		return CONFIG_TYPE_INT;
	if (type == "float")
		return CONFIG_TYPE_FLOAT;
	if (type == "bool")
		return CONFIG_TYPE_BOOL;
	return CONFIG_TYPE_STRING;
}

static const char* StoreConfigString(const StringConvertibleOptionalValue& value)
{
	if (!value.IsSet())
		return nullptr;

	configStringBuffer.emplace_back(value.ToString());
	return configStringBuffer.back().c_str();
}

static const char* StoreConfigString(const ConfigVariableMetaData::OptionalString& value)
{
	if (!value.IsSet())
		return nullptr;

	configStringBuffer.emplace_back(value.ToString());
	return configStringBuffer.back().c_str();
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
	result->exists = query->hasDefault || configHandler->IsSet(query->key);
	result->value = result->exists
		? (query->hasDefault ? configHandler->GetIntSafe(query->key, query->defaultValue) : configHandler->GetInt(query->key))
		: 0;
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
	result->exists = query->hasDefault || configHandler->IsSet(query->key);
	result->value = result->exists
		? (query->hasDefault ? configHandler->GetFloatSafe(query->key, query->defaultValue) : configHandler->GetFloat(query->key))
		: 0.0f;
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

	result->exists = query->hasDefault || configHandler->IsSet(query->key);
	if (!result->exists) {
		result->error = nullptr;
		result->value = nullptr;
		return;
	}

	const std::string defaultStr = (query->defaultValue != nullptr) ? query->defaultValue : "";
	std::string value = query->hasDefault
		? configHandler->GetStringSafe(query->key, defaultStr)
		: configHandler->GetString(query->key);

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
	configParamBuffer.clear();
	configStringBuffer.clear();

	if (!ConfigReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const ConfigVariable::MetaDataMap& cfgMap = ConfigVariable::GetMetaDataMap();
	configParamBuffer.reserve(cfgMap.size());
	configStringBuffer.reserve(cfgMap.size() * 6);

	for (const auto& [_, meta]: cfgMap) {
		configStringBuffer.emplace_back(meta->GetKey());
		const char* name = configStringBuffer.back().c_str();

		ConfigParam param = {};
		param.name = name;
		param.type = ConfigTypeFromString(meta->GetType());
		param.description = StoreConfigString(meta->GetDescription());
		param.defaultValue = StoreConfigString(meta->GetDefaultValue());
		param.minimumValue = StoreConfigString(meta->GetMinimumValue());
		param.maximumValue = StoreConfigString(meta->GetMaximumValue());
		param.readOnly = meta->GetReadOnly().IsSet() && meta->GetReadOnly().Get();
		configParamBuffer.push_back(param);
	}

	result->error = nullptr;
	result->params = configParamBuffer.data();
	result->count = configParamBuffer.size();
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

	configHandler->EnableWriting(globalConfig.luaWritableConfigFile);
	configHandler->Set(query->key, query->value, query->useOverlay);
	configHandler->EnableWriting(true);
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

	configHandler->EnableWriting(globalConfig.luaWritableConfigFile);
	configHandler->Set(query->key, query->value, query->useOverlay);
	configHandler->EnableWriting(true);
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

	configHandler->EnableWriting(globalConfig.luaWritableConfigFile);
	configHandler->SetString(query->key, query->value, query->useOverlay);
	configHandler->EnableWriting(true);
	result->error = nullptr;
	result->success = true;
}

// Log sections
static void NativeGetLogSections(const GetLogSectionsQuery* query, GetLogSectionsResult* result)
{
	bufferPos = 0;
	logSectionBuffer.clear();

	const int count = log_filter_section_getNumRegisteredSections();
	logSectionBuffer.reserve(count);
	for (int i = 0; i < count; ++i) {
		logSectionBuffer.push_back(log_filter_section_getRegisteredIndex(i));
	}

	result->error = nullptr;
	result->sections = logSectionBuffer.data();
	result->count = logSectionBuffer.size();
}

static void NativeSetLogSectionFilterLevel(const SetLogSectionFilterLevelQuery* query, SetLogSectionFilterLevelResult* result)
{
	bufferPos = 0;

	if (query->section == nullptr) {
		result->error = &INVALID_ARG_ERROR;
		return;
	}

	// Match Spring.SetLogSectionFilterLevel: runtime-created sections must be
	// interned before their filter level is applied, otherwise GetLogSections
	// cannot observe the same section through the native/Wasm path.
	log_frontend_register_runtime_section(query->level, query->section);
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
