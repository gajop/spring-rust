#pragma once

#include <stdint.h>
#include "Common.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Metal Map API
// @see rts/Lua/LuaMetalMap.cpp
//
// Metal resource map queries and modifications
// ============================================================================

// Query: Get metal map size
struct GetMetalMapSizeQuery {
	// No parameters needed
	uint8_t _unused;  // C doesn't allow empty structs
};

struct GetMetalMapSizeResult {
	const Error* error;
	int32_t width;
	int32_t height;
};

// Query: Get metal amount at position
struct GetMetalAmountQuery {
	int32_t x;
	int32_t z;
};

struct GetMetalAmountResult {
	const Error* error;
	float amount;
};

// Query: Get metal extraction rate at position
struct GetMetalExtractionQuery {
	int32_t x;
	int32_t z;
};

struct GetMetalExtractionResult {
	const Error* error;
	float extraction;
};

// Query: Set metal amount at position
struct SetMetalAmountQuery {
	int32_t x;
	int32_t z;
	float amount;
};

struct SetMetalAmountResult {
	const Error* error;
};

// API structure
struct MetalMapApi {
	void (*GetMetalMapSize)(
		const GetMetalMapSizeQuery* query,
		GetMetalMapSizeResult* result
	);

	void (*GetMetalAmount)(
		const GetMetalAmountQuery* query,
		GetMetalAmountResult* result
	);

	void (*GetMetalExtraction)(
		const GetMetalExtractionQuery* query,
		GetMetalExtractionResult* result
	);

	void (*SetMetalAmount)(
		const SetMetalAmountQuery* query,
		SetMetalAmountResult* result
	);
};

extern const MetalMapApi METAL_MAP_API;

#ifdef __cplusplus
}
#endif
