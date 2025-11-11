#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Utilities API
// @see rts/Lua/LuaUtils.cpp
//
// General utility functions
// ============================================================================

// CEG (Custom Explosion Generator) ID
struct CEGIDResult {
	const Error* error;
	int32_t cegID;
};

// Test order results
struct TestBuildOrderResult {
	const Error* error;
	bool canBuild;
	int32_t feature;  // Blocking feature ID or -1
};

struct TestMoveOrderResult {
	const Error* error;
	bool canMove;
};

// Build position adjustment
struct BuildPosQuery {
	int32_t unitDefID;
	Float3 pos;
	int32_t facing;
};

// API structure
struct UtilsApi {
	// CEG
	CEGIDResult (*GetCEGID)(const char* cegName);

	// Build testing
	TestBuildOrderResult (*TestBuildOrder)(int32_t unitDefID, Float3 pos, int32_t facing);
	Float3Result (*Pos2BuildPos)(BuildPosQuery query);
	Float3Result (*ClosestBuildPos)(int32_t teamID, int32_t unitDefID, Float3 pos, float searchRadius, int32_t minDist, int32_t facing);

	// Move testing
	TestMoveOrderResult (*TestMoveOrder)(int32_t unitDefID, Float3 pos);

	// Unit dimensions
	Float3Result (*GetUnitDefDimensions)(int32_t unitDefID);
};

extern const UtilsApi UTILS_API;

#ifdef __cplusplus
}
#endif
