#include "Game/Rust/api/MetalMap.h"

#include <algorithm>

#include "Map/MetalMap.h"

namespace {

static const Error METAL_MAP_UNAVAILABLE_ERROR = {
	.code = 1,
	.message = "Metal map is not available"
};

static bool MetalMapReady()
{
	return (metalMap.GetSizeX() > 0) && (metalMap.GetSizeZ() > 0);
}

static MetalMapQuery ClampMetalMapQuery(MetalMapQuery query)
{
	MetalMapQuery clamped = query;
	if (!MetalMapReady())
		return clamped;

	const int32_t maxX = std::max(metalMap.GetSizeX() - 1, 0);
	const int32_t maxZ = std::max(metalMap.GetSizeZ() - 1, 0);

	clamped.x = std::clamp(query.x, 0, maxX);
	clamped.z = std::clamp(query.z, 0, maxZ);
	return clamped;
}

static MetalMapSizeResult NativeGetMetalMapSize()
{
	MetalMapSizeResult result = {};
	if (!MetalMapReady()) {
		result.error = &METAL_MAP_UNAVAILABLE_ERROR;
		return result;
	}

	result.mapWidth = metalMap.GetSizeX();
	result.mapHeight = metalMap.GetSizeZ();
	return result;
}

static MetalMapSampleResult NativeGetMetalAmount(MetalMapQuery query)
{
	MetalMapSampleResult result = {};
	if (!MetalMapReady()) {
		result.error = &METAL_MAP_UNAVAILABLE_ERROR;
		return result;
	}

	const MetalMapQuery clamped = ClampMetalMapQuery(query);
	result.sample.query = clamped;
	result.sample.amount = metalMap.GetMetalAmount(clamped.x, clamped.z);
	return result;
}

static MetalMapSampleResult NativeGetMetalExtraction(MetalMapQuery query)
{
	MetalMapSampleResult result = {};
	if (!MetalMapReady()) {
		result.error = &METAL_MAP_UNAVAILABLE_ERROR;
		return result;
	}

	const MetalMapQuery clamped = ClampMetalMapQuery(query);
	result.sample.query = clamped;
	result.sample.amount = static_cast<float>(metalMap.GetMetalExtraction(clamped.x, clamped.z));
	return result;
}

static MetalMapWriteResult NativeSetMetalAmount(MetalMapWriteRequest request)
{
	MetalMapWriteResult result = {};
	if (!MetalMapReady()) {
		result.error = &METAL_MAP_UNAVAILABLE_ERROR;
		return result;
	}

	const MetalMapQuery clamped = ClampMetalMapQuery(request.query);
	metalMap.SetMetalAmount(clamped.x, clamped.z, request.amount);

	result.sample.query = clamped;
	result.sample.amount = metalMap.GetMetalAmount(clamped.x, clamped.z);
	return result;
}

} // namespace

const MetalMapApi METAL_MAP_API = {
	.GetMetalMapSize = NativeGetMetalMapSize,
	.GetMetalAmount = NativeGetMetalAmount,
	.GetMetalExtraction = NativeGetMetalExtraction,
	.SetMetalAmount = NativeSetMetalAmount,
};

