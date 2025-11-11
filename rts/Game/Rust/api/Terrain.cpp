#include "Terrain.h"

#include "Map/ReadMap.h"
#include "Map/MapInfo.h"
#include "Map/MapDimensions.h"
#include "Map/Ground.h"
#include "Map/SMF/SMFReadMap.h"
#include "System/float3.h"

namespace {

// Error constants
static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Map not ready"
};

static const Error INVALID_ARG_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid argument"
};

// Helper: check if map is ready
static bool MapReady()
{
	return (readMap != nullptr);
}

// Position queries
static PosInMapResult NativeIsPosInMap(float x, float z)
{
	PosInMapResult result = {};
	if (!MapReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	result.inMap = (x >= 0.0f && z >= 0.0f &&
		x < mapDims.mapx * SQUARE_SIZE && z < mapDims.mapy * SQUARE_SIZE);
	result.inPlayArea = (x >= 0.0f && z >= 0.0f &&
		x < mapDims.pwr2mapx * SQUARE_SIZE && z < mapDims.pwr2mapy * SQUARE_SIZE);

	return result;
}

// Height queries
static FloatResult NativeGetGroundHeight(float x, float z)
{
	FloatResult result = {};
	if (!MapReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	result.value = CGround::GetHeightReal(x, z);
	return result;
}

static FloatResult NativeGetGroundOrigHeight(float x, float z)
{
	FloatResult result = {};
	if (!MapReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	result.value = CGround::GetOrigHeight(x, z);
	return result;
}

static FloatResult NativeGetSmoothMeshHeight(float x, float z)
{
	FloatResult result = {};
	if (!MapReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	result.value = CGround::GetSmoothMeshHeight(x, z);
	return result;
}

static FloatResult NativeGetWaterPlaneLevel()
{
	FloatResult result = {};
	if (!MapReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	result.value = 0.0f; // Default water level is 0
	return result;
}

static FloatResult NativeGetWaterLevel(float x, float z)
{
	FloatResult result = {};
	if (!MapReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Water level is same everywhere (for now)
	result.value = 0.0f;
	return result;
}

// Normal queries
static GroundNormalResult NativeGetGroundNormal(float x, float z)
{
	GroundNormalResult result = {};
	result.valid = true;

	if (!MapReady()) {
		result.error = &NOT_READY_ERROR;
		result.valid = false;
		return result;
	}

	const float3 normal = CGround::GetNormal(x, z);
	result.data.normal.x = normal.x;
	result.data.normal.y = normal.y;
	result.data.normal.z = normal.z;

	// Calculate slope from normal (angle from vertical)
	result.data.slope = std::acos(normal.y); // y is up

	return result;
}

// Terrain info
static GroundInfoResult NativeGetGroundInfo(float x, float z)
{
	GroundInfoResult result = {};
	if (!MapReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const int hmx = Clamp(int(x / SQUARE_SIZE), 0, mapDims.mapxm1);
	const int hmz = Clamp(int(z / SQUARE_SIZE), 0, mapDims.mapym1);
	const int typeIndex = readMap->GetTypeMapSynced()[hmz * mapDims.mapx + hmx];

	result.info.x = x;
	result.info.z = z;
	result.info.terrainTypeIndex = typeIndex;

	const CMapInfo::TerrainType& tt = mapInfo->terrainTypes[typeIndex];
	result.info.terrainTypeName = tt.name.c_str();
	result.info.metalExtraction = readMap->GetMetalAmount(hmx, hmz);
	result.info.hardness = tt.hardness;
	result.info.tankSpeed = tt.tankSpeed;
	result.info.kbotSpeed = tt.kbotSpeed;
	result.info.hoverSpeed = tt.hoverSpeed;
	result.info.shipSpeed = tt.shipSpeed;
	result.info.receiveTracks = tt.receiveTracks;

	return result;
}

static TerrainTypeDataResult NativeGetTerrainTypeData(int32_t terrainTypeIndex)
{
	TerrainTypeDataResult result = {};
	if (!MapReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (terrainTypeIndex < 0 || terrainTypeIndex >= static_cast<int32_t>(mapInfo->terrainTypes.size())) {
		result.error = &INVALID_ARG_ERROR;
		return result;
	}

	const CMapInfo::TerrainType& tt = mapInfo->terrainTypes[terrainTypeIndex];
	result.data.index = terrainTypeIndex;
	result.data.name = tt.name.c_str();
	result.data.hardness = tt.hardness;
	result.data.tankSpeed = tt.tankSpeed;
	result.data.kbotSpeed = tt.kbotSpeed;
	result.data.hoverSpeed = tt.hoverSpeed;
	result.data.shipSpeed = tt.shipSpeed;
	result.data.receiveTracks = tt.receiveTracks;

	return result;
}

static GroundExtremesResult NativeGetGroundExtremes()
{
	GroundExtremesResult result = {};
	if (!MapReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	result.extremes.initMinHeight = readMap->GetInitMinHeight();
	result.extremes.initMaxHeight = readMap->GetInitMaxHeight();
	result.extremes.currMinHeight = readMap->GetCurrMinHeight();
	result.extremes.currMaxHeight = readMap->GetCurrMaxHeight();

	return result;
}

// Blocking
static BoolResult NativeGetGroundBlocked(float x1, float z1, float x2, float z2)
{
	BoolResult result = {};
	if (!MapReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Check if position is blocked by features or ground
	result.value = CGround::GetBlocked(x1, z1) || CGround::GetBlocked(x2, z2);

	return result;
}

// Grass (decoration)
static FloatResult NativeGetGrass(float x, float z)
{
	FloatResult result = {};
	if (!MapReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Grass level (for rendering grass decoration)
	const int gx = Clamp(int(x / SQUARE_SIZE), 0, mapDims.mapxm1);
	const int gz = Clamp(int(z / SQUARE_SIZE), 0, mapDims.mapym1);

	// Grass map might not exist
	const unsigned char* grassMap = readMap->GetGrassMap();
	if (grassMap == nullptr) {
		result.value = 0.0f;
		return result;
	}

	result.value = grassMap[gz * mapDims.mapx + gx] / 255.0f;

	return result;
}

} // namespace

const TerrainApi TERRAIN_API = {
	.IsPosInMap = NativeIsPosInMap,

	.GetGroundHeight = NativeGetGroundHeight,
	.GetGroundOrigHeight = NativeGetGroundOrigHeight,
	.GetSmoothMeshHeight = NativeGetSmoothMeshHeight,
	.GetWaterPlaneLevel = NativeGetWaterPlaneLevel,
	.GetWaterLevel = NativeGetWaterLevel,

	.GetGroundNormal = NativeGetGroundNormal,

	.GetGroundInfo = NativeGetGroundInfo,
	.GetTerrainTypeData = NativeGetTerrainTypeData,
	.GetGroundExtremes = NativeGetGroundExtremes,

	.GetGroundBlocked = NativeGetGroundBlocked,

	.GetGrass = NativeGetGrass,
};
