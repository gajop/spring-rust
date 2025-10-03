#pragma once

#include <stdint.h>

#include "Game/Rust/api/Common.h"

#ifdef __cplusplus
extern "C" {
#endif

struct MetalMapQuery {
	int32_t x;
	int32_t z;
};

struct MetalMapSample {
	MetalMapQuery query;
	float amount;
};

struct MetalMapWriteRequest {
	MetalMapQuery query;
	float amount;
};

struct MetalMapSizeResult {
	const Error* error;
	int32_t mapWidth;
	int32_t mapHeight;
};

struct MetalMapSampleResult {
	const Error* error;
	MetalMapSample sample;
};

struct MetalMapWriteResult {
	const Error* error;
	MetalMapSample sample;
};

struct MetalMapApi {
	MetalMapSizeResult (*GetMetalMapSize)();
	MetalMapSampleResult (*GetMetalAmount)(MetalMapQuery query);
	MetalMapSampleResult (*GetMetalExtraction)(MetalMapQuery query);
	MetalMapWriteResult (*SetMetalAmount)(MetalMapWriteRequest request);
};

extern const MetalMapApi METAL_MAP_API;

#ifdef __cplusplus
}
#endif

