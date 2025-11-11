#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Configuration API
// @see rts/Lua/LuaUnsyncedRead.cpp, LuaUnsyncedCtrl.cpp
//
// Engine configuration queries and control (unsynced)
// ============================================================================

// Config value types
enum ConfigValueType {
	CONFIG_TYPE_INT = 0,
	CONFIG_TYPE_FLOAT = 1,
	CONFIG_TYPE_STRING = 2,
	CONFIG_TYPE_BOOL = 3,
};

// Config parameter info
struct ConfigParam {
	const char* name;
	ConfigValueType type;
	const char* description;
	const char* defaultValue;
	const char* minimumValue;
	const char* maximumValue;
	bool readOnly;
};

struct ConfigParamResult {
	const Error* error;
	ConfigParam param;
	bool exists;
};

// Config params list
struct ConfigParamsResult {
	const Error* error;
	ConfigParam* params;
	uint32_t count;
};

// API structure
struct ConfigApi {
	// Query config
	Int32Result (*GetConfigInt)(const char* key, int32_t defaultValue);
	FloatResult (*GetConfigFloat)(const char* key, float defaultValue);
	StringResult (*GetConfigString)(const char* key, const char* defaultValue);
	ConfigParamsResult (*GetConfigParams)();

	// Set config (unsynced)
	BoolResult (*SetConfigInt)(const char* key, int32_t value);
	BoolResult (*SetConfigFloat)(const char* key, float value);
	BoolResult (*SetConfigString)(const char* key, const char* value);

	// Log sections
	StringArray (*GetLogSections)();
	BoolResult (*SetLogSectionFilterLevel)(const char* section, int32_t level);
};

extern const ConfigApi CONFIG_API;

#ifdef __cplusplus
}
#endif
