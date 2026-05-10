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

// Queries
struct GetCEGIDQuery { const char* cegName; };
struct GetCEGIDResult { const Error* error; int32_t cegID; };

struct TestBuildOrderQuery {
	int32_t unitDefID;
	Float3 pos;
	int32_t facing;
};

struct TestBuildOrderResult {
	const Error* error;
	bool canBuild;
	int32_t feature;  // Blocking feature ID or -1
};

struct Pos2BuildPosQuery {
	int32_t unitDefID;
	Float3 pos;
	int32_t facing;
};

struct Pos2BuildPosResult {
	const Error* error;
	Float3 buildPos;
};

struct ClosestBuildPosQuery {
	int32_t teamID;
	int32_t unitDefID;
	Float3 pos;
	float searchRadius;
	int32_t minDist;
	int32_t facing;
};

struct ClosestBuildPosResult {
	const Error* error;
	Float3 buildPos;
};

struct TestMoveOrderQuery {
	int32_t unitDefID;
	Float3 pos;
};

struct TestMoveOrderResult {
	const Error* error;
	bool canMove;
};

struct GetUnitDefDimensionsQuery {
	int32_t unitDefID;
};

struct GetUnitDefDimensionsResult {
	const Error* error;
	Float3 dimensions;
};

// API structure
struct UtilsApi {
	void (*GetCEGID)(
		const GetCEGIDQuery* query,
		GetCEGIDResult* result
	);

	void (*TestBuildOrder)(
		const TestBuildOrderQuery* query,
		TestBuildOrderResult* result
	);

	void (*Pos2BuildPos)(
		const Pos2BuildPosQuery* query,
		Pos2BuildPosResult* result
	);

	void (*ClosestBuildPos)(
		const ClosestBuildPosQuery* query,
		ClosestBuildPosResult* result
	);

	void (*TestMoveOrder)(
		const TestMoveOrderQuery* query,
		TestMoveOrderResult* result
	);

	void (*GetUnitDefDimensions)(
		const GetUnitDefDimensionsQuery* query,
		GetUnitDefDimensionsResult* result
	);
};

extern const UtilsApi UTILS_API;

#ifdef __cplusplus
}
#endif
