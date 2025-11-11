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
// (Already partially exposed in NativeInterface - this consolidates it)
// ============================================================================

// Terrain type data
struct TerrainTypeData {
	int32_t index;
	const char* name;
	float hardness;
	float tankSpeed;
	float kbotSpeed;
	float hoverSpeed;
	float shipSpeed;
	bool receiveTracks;
};

struct TerrainTypeDataResult {
	const Error* error;
	TerrainTypeData data;
};

// Ground info at a position
struct GroundInfo {
	float x;
	float z;
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

struct GroundInfoResult {
	const Error* error;
	GroundInfo info;
};

// Ground normal
struct GroundNormal {
	Float3 normal;
	float slope;  // Angle in radians
};

struct GroundNormalResult {
	const Error* error;
	GroundNormal data;
	bool valid;
};

// Ground extremes (height range)
struct GroundExtremes {
	float initMinHeight;
	float initMaxHeight;
	float currMinHeight;
	float currMaxHeight;
};

struct GroundExtremesResult {
	const Error* error;
	GroundExtremes extremes;
};

// Position in map check
struct PosInMapResult {
	const Error* error;
	bool inMap;
	bool inPlayArea;
};

// Blocking check
struct BlockingQuery {
	Float2 pos1;
	Float2 pos2;  // Optional second position
	bool hasPos2;
};

// API structure
struct TerrainApi {
	// Position queries
	PosInMapResult (*IsPosInMap)(float x, float z);

	// Height queries
	FloatResult (*GetGroundHeight)(float x, float z);
	FloatResult (*GetGroundOrigHeight)(float x, float z);
	FloatResult (*GetSmoothMeshHeight)(float x, float z);
	FloatResult (*GetWaterPlaneLevel)();  // Global water level
	FloatResult (*GetWaterLevel)(float x, float z);  // Water level at position

	// Normal queries
	GroundNormalResult (*GetGroundNormal)(float x, float z);

	// Terrain info
	GroundInfoResult (*GetGroundInfo)(float x, float z);
	TerrainTypeDataResult (*GetTerrainTypeData)(int32_t terrainTypeIndex);
	GroundExtremesResult (*GetGroundExtremes)();

	// Blocking
	BoolResult (*GetGroundBlocked)(float x1, float z1, float x2, float z2);

	// Grass (decoration)
	FloatResult (*GetGrass)(float x, float z);
};

extern const TerrainApi TERRAIN_API;

#ifdef __cplusplus
}
#endif
