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

// Queries
struct GetConfigIntQuery {
	const char* key;
	int32_t defaultValue;
};

struct GetConfigIntResult {
	const Error* error;
	int32_t value;
};

struct GetConfigFloatQuery {
	const char* key;
	float defaultValue;
};

struct GetConfigFloatResult {
	const Error* error;
	float value;
};

struct GetConfigStringQuery {
	const char* key;
	const char* defaultValue;
};

struct GetConfigStringResult {
	const Error* error;
	const char* value;
};

struct GetConfigParamsQuery {
	uint8_t _unused;
};

struct GetConfigParamsResult {
	const Error* error;
	ConfigParam* params;
	uint32_t count;
};

struct SetConfigIntQuery {
	const char* key;
	int32_t value;
};

struct SetConfigIntResult {
	const Error* error;
	bool success;
};

struct SetConfigFloatQuery {
	const char* key;
	float value;
};

struct SetConfigFloatResult {
	const Error* error;
	bool success;
};

struct SetConfigStringQuery {
	const char* key;
	const char* value;
};

struct SetConfigStringResult {
	const Error* error;
	bool success;
};

struct GetLogSectionsQuery {
	uint8_t _unused;
};

struct GetLogSectionsResult {
	const Error* error;
	const char** sections;
	uint32_t count;
};

struct SetLogSectionFilterLevelQuery {
	const char* section;
	int32_t level;
};

struct SetLogSectionFilterLevelResult {
	const Error* error;
	bool success;
};

// API structure
struct ConfigApi {
	void (*GetConfigInt)(
		const GetConfigIntQuery* query,
		GetConfigIntResult* result
	);

	void (*GetConfigFloat)(
		const GetConfigFloatQuery* query,
		GetConfigFloatResult* result
	);

	void (*GetConfigString)(
		const GetConfigStringQuery* query,
		GetConfigStringResult* result
	);

	void (*GetConfigParams)(
		const GetConfigParamsQuery* query,
		GetConfigParamsResult* result
	);

	void (*SetConfigInt)(
		const SetConfigIntQuery* query,
		SetConfigIntResult* result
	);

	void (*SetConfigFloat)(
		const SetConfigFloatQuery* query,
		SetConfigFloatResult* result
	);

	void (*SetConfigString)(
		const SetConfigStringQuery* query,
		SetConfigStringResult* result
	);

	void (*GetLogSections)(
		const GetLogSectionsQuery* query,
		GetLogSectionsResult* result
	);

	void (*SetLogSectionFilterLevel)(
		const SetLogSectionFilterLevelQuery* query,
		SetLogSectionFilterLevelResult* result
	);
};

extern const ConfigApi CONFIG_API;

#ifdef __cplusplus
}
#endif
