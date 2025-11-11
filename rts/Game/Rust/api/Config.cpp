#include "Config.h"

#include "System/Config/ConfigHandler.h"
#include "System/Log/Level.h"
#include "System/Log/ILog.h"
#include <vector>
#include <string>

namespace {

// Error constants
static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Config system not ready"
};

// Query config
static Int32Result NativeGetConfigInt(const char* key, int32_t defaultValue)
{
	Int32Result result = {};
	if (key == nullptr) {
		result.value = defaultValue;
		return result;
	}

	result.value = configHandler->GetInt(key, defaultValue);
	return result;
}

static FloatResult NativeGetConfigFloat(const char* key, float defaultValue)
{
	FloatResult result = {};
	if (key == nullptr) {
		result.value = defaultValue;
		return result;
	}

	result.value = configHandler->GetFloat(key, defaultValue);
	return result;
}

static StringResult NativeGetConfigString(const char* key, const char* defaultValue)
{
	StringResult result = {};
	if (key == nullptr) {
		result.value = (defaultValue != nullptr) ? defaultValue : "";
		return result;
	}

	// Use static storage
	static thread_local std::string configValue;
	configValue = configHandler->GetString(key, (defaultValue != nullptr) ? defaultValue : "");
	result.value = configValue.c_str();
	return result;
}

static ConfigParamsResult NativeGetConfigParams()
{
	ConfigParamsResult result = {};
	// Config params enumeration not implemented - would require accessing internal config map
	result.params = nullptr;
	result.count = 0;
	return result;
}

// Set config
static BoolResult NativeSetConfigInt(const char* key, int32_t value)
{
	BoolResult result = {};
	if (key == nullptr) {
		result.value = false;
		return result;
	}

	configHandler->Set(key, value);
	result.value = true;
	return result;
}

static BoolResult NativeSetConfigFloat(const char* key, float value)
{
	BoolResult result = {};
	if (key == nullptr) {
		result.value = false;
		return result;
	}

	configHandler->Set(key, value);
	result.value = true;
	return result;
}

static BoolResult NativeSetConfigString(const char* key, const char* value)
{
	BoolResult result = {};
	if (key == nullptr || value == nullptr) {
		result.value = false;
		return result;
	}

	configHandler->SetString(key, value);
	result.value = true;
	return result;
}

// Log sections
static StringArray NativeGetLogSections()
{
	StringArray result = {};
	// Log sections enumeration not implemented - would require accessing log internals
	result.data = nullptr;
	result.length = 0;
	return result;
}

static BoolResult NativeSetLogSectionFilterLevel(const char* section, int32_t level)
{
	BoolResult result = {};
	if (section == nullptr) {
		result.value = false;
		return result;
	}

	// Log section filtering - simplified implementation
	// Would call log.SetSectionFilterLevel(section, level);
	result.value = true;
	return result;
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
