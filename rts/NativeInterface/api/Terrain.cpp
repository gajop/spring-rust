#include "Terrain.h"

#include "Map/ReadMap.h"
#include "Map/MapInfo.h"
#include "Map/MapDimensions.h"
#include "Map/Ground.h"
#include "Map/MetalMap.h"
#include "Sim/Misc/SmoothHeightMesh.h"
#include "Sim/Misc/GroundBlockingObjectMap.h"
#include "Rendering/Env/GrassDrawer.h"
#include "System/float3.h"

namespace {

// Scratch buffer for dynamic data
static thread_local char scratchBuffer[1024];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Map not ready"
};

static const Error INVALID_ARG_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid argument"
};

static bool MapReady()
{
	return (readMap != nullptr);
}

static void NativeIsPosInMap(const IsPosInMapQuery* query, IsPosInMapResult* result)
{
	bufferPos = 0;

	if (!MapReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->error = nullptr;
	result->inMap = (query->x >= 0.0f && query->z >= 0.0f &&
		query->x < mapDims.mapx * SQUARE_SIZE && query->z < mapDims.mapy * SQUARE_SIZE);
	result->inPlayArea = (query->x >= 0.0f && query->z >= 0.0f &&
		query->x < mapDims.pwr2mapx * SQUARE_SIZE && query->z < mapDims.pwr2mapy * SQUARE_SIZE);
}

static void NativeGetGroundHeight(const GetGroundHeightQuery* query, GetGroundHeightResult* result)
{
	bufferPos = 0;

	if (!MapReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->error = nullptr;
	result->height = CGround::GetHeightReal(query->x, query->z);
}

static void NativeGetGroundOrigHeight(const GetGroundOrigHeightQuery* query, GetGroundOrigHeightResult* result)
{
	bufferPos = 0;

	if (!MapReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->error = nullptr;
	result->height = CGround::GetOrigHeight(query->x, query->z);
}

static void NativeGetSmoothMeshHeight(const GetSmoothMeshHeightQuery* query, GetSmoothMeshHeightResult* result)
{
	bufferPos = 0;

	if (!MapReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->error = nullptr;
	result->height = smoothGround.GetHeight(query->x, query->z);
}

static void NativeGetWaterPlaneLevel(const GetWaterPlaneLevelQuery* query, GetWaterPlaneLevelResult* result)
{
	bufferPos = 0;

	if (!MapReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->error = nullptr;
	result->level = 0.0f;  // Default water level
}

static void NativeGetWaterLevel(const GetWaterLevelQuery* query, GetWaterLevelResult* result)
{
	bufferPos = 0;

	if (!MapReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->error = nullptr;
	result->level = 0.0f;  // Water level same everywhere
}

static void NativeGetGroundNormal(const GetGroundNormalQuery* query, GetGroundNormalResult* result)
{
	bufferPos = 0;

	if (!MapReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const float3 normal = query->smoothed
		? CGround::GetNormal(query->x, query->z, true)
		: CGround::GetSmoothNormal(query->x, query->z, true);
	result->error = nullptr;
	result->normal.x = normal.x;
	result->normal.y = normal.y;
	result->normal.z = normal.z;
	result->slope = std::acos(normal.y);  // Angle from vertical
}

static void NativeGetGroundInfo(const GetGroundInfoQuery* query, GetGroundInfoResult* result)
{
	bufferPos = 0;

	if (!MapReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const int hmx = std::clamp(int(query->x / SQUARE_SIZE), 0, mapDims.mapxm1);
	const int hmz = std::clamp(int(query->z / SQUARE_SIZE), 0, mapDims.mapym1);
	const int tmx = hmx >> 1;
	const int tmz = hmz >> 1;
	const int typeIndex = readMap->GetTypeMapSynced()[tmz * mapDims.hmapx + tmx];

	const CMapInfo::TerrainType& tt = mapInfo->terrainTypes[typeIndex];

	result->error = nullptr;
	result->terrainTypeIndex = typeIndex;
	result->terrainTypeName = tt.name.c_str();
	result->metalExtraction = metalMap.GetMetalAmount(hmx, hmz);
	result->hardness = tt.hardness;
	result->tankSpeed = tt.tankSpeed;
	result->kbotSpeed = tt.kbotSpeed;
	result->hoverSpeed = tt.hoverSpeed;
	result->shipSpeed = tt.shipSpeed;
	result->receiveTracks = tt.receiveTracks;
}

static void NativeGetTerrainTypeData(const GetTerrainTypeDataQuery* query, GetTerrainTypeDataResult* result)
{
	bufferPos = 0;

	if (!MapReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (query->terrainTypeIndex < 0 || query->terrainTypeIndex >= CMapInfo::NUM_TERRAIN_TYPES) {
		char* msg = &scratchBuffer[bufferPos];
		bufferPos += snprintf(msg, sizeof(scratchBuffer) - bufferPos,
			"Terrain type index %d out of range [0-%d]",
			query->terrainTypeIndex, CMapInfo::NUM_TERRAIN_TYPES - 1) + 1;
		dynamicError.code = ERROR_OUT_OF_BOUNDS;
		dynamicError.message = msg;
		result->error = &dynamicError;
		return;
	}

	const CMapInfo::TerrainType& tt = mapInfo->terrainTypes[query->terrainTypeIndex];

	result->error = nullptr;
	result->index = query->terrainTypeIndex;
	result->name = tt.name.c_str();
	result->hardness = tt.hardness;
	result->tankSpeed = tt.tankSpeed;
	result->kbotSpeed = tt.kbotSpeed;
	result->hoverSpeed = tt.hoverSpeed;
	result->shipSpeed = tt.shipSpeed;
	result->receiveTracks = tt.receiveTracks;
}

static void NativeGetGroundExtremes(const GetGroundExtremesQuery* query, GetGroundExtremesResult* result)
{
	bufferPos = 0;

	if (!MapReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->error = nullptr;
	result->initMinHeight = readMap->GetInitMinHeight();
	result->initMaxHeight = readMap->GetInitMaxHeight();
	result->currMinHeight = readMap->GetCurrMinHeight();
	result->currMaxHeight = readMap->GetCurrMaxHeight();
}

static void NativeGetHeightMapSize(const GetHeightMapSizeQuery* /*query*/, GetHeightMapSizeResult* result)
{
	bufferPos = 0;

	if (!MapReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->error = nullptr;
	result->pointsX = mapDims.mapxp1;
	result->pointsZ = mapDims.mapyp1;
}

static void NativeGetGroundBlocked(const GetGroundBlockedQuery* query, GetGroundBlockedResult* result)
{
	bufferPos = 0;

	if (!MapReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const int x1 = query->x1 / SQUARE_SIZE;
	const int z1 = query->z1 / SQUARE_SIZE;
	const int x2 = query->x2 / SQUARE_SIZE;
	const int z2 = query->z2 / SQUARE_SIZE;

	result->error = nullptr;
	result->blocked = (groundBlockingObjectMap.GroundBlocked(x1, z1) != nullptr) ||
	                  (groundBlockingObjectMap.GroundBlocked(x2, z2) != nullptr);
}

static void NativeGetGrass(const GetGrassQuery* query, GetGrassResult* result)
{
	bufferPos = 0;

	if (!MapReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (grassDrawer == nullptr) {
		result->error = nullptr;
		result->grassLevel = 0.0f;
		return;
	}

	const float3 pos(query->x, 0.0f, query->z);
	result->error = nullptr;
	result->grassLevel = grassDrawer->GetGrass(pos.cClampInBounds());
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
	.GetHeightMapSize = NativeGetHeightMapSize,
	.GetGroundBlocked = NativeGetGroundBlocked,
	.GetGrass = NativeGetGrass,
};
