#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Terrain/Ground API
// @see rts/Lua/LuaSyncedRead.cpp, NativeInterface.h
//
// Terrain height, normals, blocking, terrain types, etc.
// ============================================================================

// Queries
struct IsPosInMapQuery {
	float x;
	float z;
};

struct IsPosInMapResult {
	const Error* error;
	bool inMap;
	bool inPlayArea;
};

struct GetGroundHeightQuery {
	float x;
	float z;
};

struct GetGroundHeightResult {
	const Error* error;
	float height;
};

struct GetGroundOrigHeightQuery {
	float x;
	float z;
};

struct GetGroundOrigHeightResult {
	const Error* error;
	float height;
};

struct GetSmoothMeshHeightQuery {
	float x;
	float z;
};

struct GetSmoothMeshHeightResult {
	const Error* error;
	float height;
};

struct GetWaterPlaneLevelQuery {
	uint8_t _unused;
};

struct GetWaterPlaneLevelResult {
	const Error* error;
	float level;
};

struct GetWaterLevelQuery {
	float x;
	float z;
};

struct GetWaterLevelResult {
	const Error* error;
	float level;
};

struct GetGroundNormalQuery {
	float x;
	float z;
};

struct GetGroundNormalResult {
	const Error* error;
	Float3 normal;
	float slope;
};

struct GetGroundInfoQuery {
	float x;
	float z;
};

struct GetGroundInfoResult {
	const Error* error;
	int32_t terrainTypeIndex;
	const char* terrainTypeName;
	float metalExtraction;
	float hardness;
	float tankSpeed;
	float kbotSpeed;
	float hoverSpeed;
	float shipSpeed;
	bool receiveTracks;
};

struct GetTerrainTypeDataQuery {
	int32_t terrainTypeIndex;
};

struct GetTerrainTypeDataResult {
	const Error* error;
	int32_t index;
	const char* name;
	float hardness;
	float tankSpeed;
	float kbotSpeed;
	float hoverSpeed;
	float shipSpeed;
	bool receiveTracks;
};

struct GetGroundExtremesQuery {
	uint8_t _unused;
};

struct GetGroundExtremesResult {
	const Error* error;
	float initMinHeight;
	float initMaxHeight;
	float currMinHeight;
	float currMaxHeight;
};

struct GetGroundBlockedQuery {
	float x1;
	float z1;
	float x2;
	float z2;
};

struct GetGroundBlockedResult {
	const Error* error;
	bool blocked;
};

struct GetGrassQuery {
	float x;
	float z;
};

struct GetGrassResult {
	const Error* error;
	float grassLevel;
};

// API structure
struct TerrainApi {
	void (*IsPosInMap)(
		const IsPosInMapQuery* query,
		IsPosInMapResult* result
	);

	void (*GetGroundHeight)(
		const GetGroundHeightQuery* query,
		GetGroundHeightResult* result
	);

	void (*GetGroundOrigHeight)(
		const GetGroundOrigHeightQuery* query,
		GetGroundOrigHeightResult* result
	);

	void (*GetSmoothMeshHeight)(
		const GetSmoothMeshHeightQuery* query,
		GetSmoothMeshHeightResult* result
	);

	void (*GetWaterPlaneLevel)(
		const GetWaterPlaneLevelQuery* query,
		GetWaterPlaneLevelResult* result
	);

	void (*GetWaterLevel)(
		const GetWaterLevelQuery* query,
		GetWaterLevelResult* result
	);

	void (*GetGroundNormal)(
		const GetGroundNormalQuery* query,
		GetGroundNormalResult* result
	);

	void (*GetGroundInfo)(
		const GetGroundInfoQuery* query,
		GetGroundInfoResult* result
	);

	void (*GetTerrainTypeData)(
		const GetTerrainTypeDataQuery* query,
		GetTerrainTypeDataResult* result
	);

	void (*GetGroundExtremes)(
		const GetGroundExtremesQuery* query,
		GetGroundExtremesResult* result
	);

	void (*GetGroundBlocked)(
		const GetGroundBlockedQuery* query,
		GetGroundBlockedResult* result
	);

	void (*GetGrass)(
		const GetGrassQuery* query,
		GetGrassResult* result
	);
};

extern const TerrainApi TERRAIN_API;

#ifdef __cplusplus
}
#endif
