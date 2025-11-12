#include "MetalMap.h"

#include <algorithm>
#include "Map/MetalMap.h"

namespace {

// Scratch buffer for dynamic data (errors, arrays, strings)
static thread_local char scratchBuffer[8192];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error METAL_MAP_UNAVAILABLE_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Metal map is not available"
};

static bool MetalMapReady()
{
	return (metalMap.GetSizeX() > 0) && (metalMap.GetSizeZ() > 0);
}

static void NativeGetMetalMapSize(const GetMetalMapSizeQuery* query, GetMetalMapSizeResult* result)
{
	bufferPos = 0;

	if (!MetalMapReady()) {
		result->error = &METAL_MAP_UNAVAILABLE_ERROR;
		return;
	}

	result->error = nullptr;
	result->width = metalMap.GetSizeX();
	result->height = metalMap.GetSizeZ();
}

static void NativeGetMetalAmount(const GetMetalAmountQuery* query, GetMetalAmountResult* result)
{
	bufferPos = 0;

	if (!MetalMapReady()) {
		result->error = &METAL_MAP_UNAVAILABLE_ERROR;
		return;
	}

	const int32_t maxX = metalMap.GetSizeX() - 1;
	const int32_t maxZ = metalMap.GetSizeZ() - 1;

	if (query->x < 0 || query->x > maxX || query->z < 0 || query->z > maxZ) {
		char* msg = &scratchBuffer[bufferPos];
		bufferPos += snprintf(msg, sizeof(scratchBuffer) - bufferPos,
			"Coordinates (%d, %d) out of bounds [0-%d, 0-%d]",
			query->x, query->z, maxX, maxZ) + 1;
		dynamicError.code = ERROR_OUT_OF_BOUNDS;
		dynamicError.message = msg;
		result->error = &dynamicError;
		return;
	}

	result->error = nullptr;
	result->amount = metalMap.GetMetalAmount(query->x, query->z);
}

static void NativeGetMetalExtraction(const GetMetalExtractionQuery* query, GetMetalExtractionResult* result)
{
	bufferPos = 0;

	if (!MetalMapReady()) {
		result->error = &METAL_MAP_UNAVAILABLE_ERROR;
		return;
	}

	const int32_t maxX = metalMap.GetSizeX() - 1;
	const int32_t maxZ = metalMap.GetSizeZ() - 1;

	if (query->x < 0 || query->x > maxX || query->z < 0 || query->z > maxZ) {
		char* msg = &scratchBuffer[bufferPos];
		bufferPos += snprintf(msg, sizeof(scratchBuffer) - bufferPos,
			"Coordinates (%d, %d) out of bounds [0-%d, 0-%d]",
			query->x, query->z, maxX, maxZ) + 1;
		dynamicError.code = ERROR_OUT_OF_BOUNDS;
		dynamicError.message = msg;
		result->error = &dynamicError;
		return;
	}

	result->error = nullptr;
	result->extraction = static_cast<float>(metalMap.GetMetalExtraction(query->x, query->z));
}

static void NativeSetMetalAmount(const SetMetalAmountQuery* query, SetMetalAmountResult* result)
{
	bufferPos = 0;

	if (!MetalMapReady()) {
		result->error = &METAL_MAP_UNAVAILABLE_ERROR;
		return;
	}

	const int32_t maxX = metalMap.GetSizeX() - 1;
	const int32_t maxZ = metalMap.GetSizeZ() - 1;

	if (query->x < 0 || query->x > maxX || query->z < 0 || query->z > maxZ) {
		char* msg = &scratchBuffer[bufferPos];
		bufferPos += snprintf(msg, sizeof(scratchBuffer) - bufferPos,
			"Coordinates (%d, %d) out of bounds [0-%d, 0-%d]",
			query->x, query->z, maxX, maxZ) + 1;
		dynamicError.code = ERROR_OUT_OF_BOUNDS;
		dynamicError.message = msg;
		result->error = &dynamicError;
		return;
	}

	metalMap.SetMetalAmount(query->x, query->z, query->amount);
	result->error = nullptr;
}

} // namespace

const MetalMapApi METAL_MAP_API = {
	.GetMetalMapSize = NativeGetMetalMapSize,
	.GetMetalAmount = NativeGetMetalAmount,
	.GetMetalExtraction = NativeGetMetalExtraction,
	.SetMetalAmount = NativeSetMetalAmount,
};
