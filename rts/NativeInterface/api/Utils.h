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
	int32_t status;   // 0=blocked, 1=occupied, 2=reclaimable/open
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

struct TestMoveOrderOptions {
	bool testTerrain;
	bool testObjects;
	bool centerOnly;
};
struct TestMoveOrderQuery {
	int32_t unitDefID;
	Float3 pos;
	Float3 dir;
	TestMoveOrderOptions options;
};

struct TestMoveOrderResult {
	const Error* error;
	bool canMove;
};

struct GetUnitDefDimensionsQuery {
	int32_t unitDefID;
};

struct UnitDefDimensions {
	float height;
	float radius;
	float midx;
	float minx;
	float maxx;
	float midy;
	float miny;
	float maxy;
	float midz;
	float minz;
	float maxz;
};

struct GetUnitDefDimensionsResult {
	const Error* error;
	UnitDefDimensions dimensions;
};

// A feature def's model bounds. Same shape as a unit's: a caller that has to
// frame or box a definition needs this for both, and only units had it.
struct GetFeatureDefDimensionsQuery {
	int32_t featureDefID;
};

struct GetFeatureDefDimensionsResult {
	const Error* error;
	UnitDefDimensions dimensions;
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

	void (*GetFeatureDefDimensions)(
		const GetFeatureDefDimensionsQuery* query,
		GetFeatureDefDimensionsResult* result
	);
};

extern const UtilsApi UTILS_API;

#ifdef __cplusplus
}
#endif
