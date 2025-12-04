#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Unsynced Read API
// @see rts/Lua/LuaUnsyncedRead.cpp
//
// Client-side property queries (rendering, UI state, view information)
// These functions query client-specific state that is not synchronized
// ============================================================================

// Unit rendering state queries
struct GetUnitNoDrawQuery { int32_t unitID; };
struct GetUnitNoDrawResult { const Error* error; bool noDraw; };

struct GetUnitLuaDrawQuery { int32_t unitID; };
struct GetUnitLuaDrawResult { const Error* error; bool luaDraw; };

struct GetUnitEngineDrawMaskQuery { int32_t unitID; };
struct GetUnitEngineDrawMaskResult { const Error* error; uint32_t engineDrawMask; };

struct GetUnitAlwaysUpdateMatrixQuery { int32_t unitID; };
struct GetUnitAlwaysUpdateMatrixResult { const Error* error; bool alwaysUpdateMatrix; };

struct GetUnitDrawFlagQuery { int32_t unitID; };
struct GetUnitDrawFlagResult { const Error* error; uint8_t drawFlag; };

// Unit UI state queries
struct GetUnitNoSelectQuery { int32_t unitID; };
struct GetUnitNoSelectResult { const Error* error; bool noSelect; };

struct GetUnitNoMinimapQuery { int32_t unitID; };
struct GetUnitNoMinimapResult { const Error* error; bool noMinimap; };

struct GetUnitNoGroupQuery { int32_t unitID; };
struct GetUnitNoGroupResult { const Error* error; bool noGroup; };

// Unit view/transform queries
struct GetUnitViewPositionQuery { int32_t unitID; bool useMidPos; };
struct GetUnitViewPositionResult { const Error* error; Float3 position; };

struct GetUnitTransformMatrixQuery { int32_t unitID; };
struct GetUnitTransformMatrixResult { const Error* error; float matrix[16]; }; // 4x4 matrix

// Unit selection volume
struct GetUnitSelectionVolumeDataQuery { int32_t unitID; };
struct GetUnitSelectionVolumeDataResult {
	const Error* error;
	Float3 scales;      // scaleX, scaleY, scaleZ
	Float3 offsets;     // offsetX, offsetY, offsetZ
	int32_t volumeType;
	bool useContHitTest;
	int32_t primaryAxis;
	bool ignoreHits;
};

// Unit icon data
struct GetUnitIconDataQuery { int32_t unitID; bool fullData; };
struct GetUnitIconDataResult {
	const Error* error;
	const char* iconName;
	float atlasTexCoords[4];  // x1, y1, x2, y2
	// Full data fields (if fullData=true):
	float size;
	float distance;
	bool radiusAdjust;
};

struct UnitRenderingApi {
	void (*GetUnitNoDraw)(const GetUnitNoDrawQuery* query, GetUnitNoDrawResult* result);
	void (*GetUnitLuaDraw)(const GetUnitLuaDrawQuery* query, GetUnitLuaDrawResult* result);
	void (*GetUnitEngineDrawMask)(const GetUnitEngineDrawMaskQuery* query, GetUnitEngineDrawMaskResult* result);
	void (*GetUnitAlwaysUpdateMatrix)(const GetUnitAlwaysUpdateMatrixQuery* query, GetUnitAlwaysUpdateMatrixResult* result);
	void (*GetUnitDrawFlag)(const GetUnitDrawFlagQuery* query, GetUnitDrawFlagResult* result);
	void (*GetUnitNoSelect)(const GetUnitNoSelectQuery* query, GetUnitNoSelectResult* result);
	void (*GetUnitNoMinimap)(const GetUnitNoMinimapQuery* query, GetUnitNoMinimapResult* result);
	void (*GetUnitNoGroup)(const GetUnitNoGroupQuery* query, GetUnitNoGroupResult* result);
	void (*GetUnitViewPosition)(const GetUnitViewPositionQuery* query, GetUnitViewPositionResult* result);
	void (*GetUnitTransformMatrix)(const GetUnitTransformMatrixQuery* query, GetUnitTransformMatrixResult* result);
	void (*GetUnitSelectionVolumeData)(const GetUnitSelectionVolumeDataQuery* query, GetUnitSelectionVolumeDataResult* result);
	void (*GetUnitIconData)(const GetUnitIconDataQuery* query, GetUnitIconDataResult* result);
};

struct UnsyncedReadApi {
	const UnitRenderingApi* unitRendering;
};

extern const UnsyncedReadApi UNSYNCED_READ_API;

#ifdef __cplusplus
}
#endif
