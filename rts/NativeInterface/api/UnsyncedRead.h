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

struct GetUnitIconQuery { int32_t unitID; };
struct GetUnitIconResult {
	const Error* error;
	const char* iconName;
	float atlasTexCoords[4];
	float size;
	float distance;
	bool radiusAdjust;
};

// Camera queries
struct GetCameraRotationQuery { uint8_t _unused; };
struct GetCameraRotationResult { const Error* error; float rotX; float rotY; float rotZ; };

struct GetCameraVectorsQuery { uint8_t _unused; };
struct GetCameraVectorsResult { const Error* error; Float3 forward; Float3 up; Float3 right; };

struct GetFrustumPlanesQuery { uint8_t _unused; };
struct GetFrustumPlanesResult { const Error* error; float planes[16]; }; // top, bottom, left, right (x,y,z,w) flattened

// Visibility queries
struct GetVisibleUnitsQuery { int32_t teamID; float radius; bool includeIcons; };
struct GetVisibleUnitsResult { const Error* error; int32_t* unitIDs; uint32_t count; };

struct GetVisibleFeaturesQuery { int32_t allyTeamID; float radius; bool includeIcons; bool includeGeos; };
struct GetVisibleFeaturesResult { const Error* error; int32_t* featureIDs; uint32_t count; };

struct GetVisibleProjectilesQuery { int32_t allyTeamID; bool includeSyncedProjectiles; bool includeWeaponProjectiles; bool includePieceProjectiles; };
struct GetVisibleProjectilesResult { const Error* error; int32_t* projectileIDs; uint32_t count; };

struct GetUnitsInScreenRectangleQuery { float left; float top; float right; float bottom; int32_t allegiance; };
struct GetUnitsInScreenRectangleResult { const Error* error; int32_t* unitIDs; uint32_t count; };

struct GetFeaturesInScreenRectangleQuery { float left; float top; float right; float bottom; };
struct GetFeaturesInScreenRectangleResult { const Error* error; int32_t* featureIDs; uint32_t count; };

struct IsUnitVisibleQuery { int32_t unitID; float radius; bool checkIcon; };
struct IsUnitVisibleResult { const Error* error; bool visible; };

struct IsUnitInViewQuery { int32_t unitID; };
struct IsUnitInViewResult { const Error* error; bool inView; };

struct IsUnitIconQuery { int32_t unitID; };
struct IsUnitIconResult { const Error* error; bool isIcon; };

// Clipboard access
struct GetClipboardQuery { uint8_t _unused; };
struct GetClipboardResult { const Error* error; const char* text; };

struct GetPrevFrameSyncChecksumQuery { uint8_t _unused; };
struct GetPrevFrameSyncChecksumResult { const Error* error; const char* checksum; };

// Command/GUI queries
struct ActiveCommandDescription {
	int32_t id;
	int32_t type;
	const char* name;
	const char* action;
	const char* tooltip;
	const char* texture;
	const char* cursor;
	bool queueing;
	bool hidden;
	bool disabled;
	bool showUnique;
	bool onlyTexture;
	const char** params;
	uint32_t paramCount;
};

struct GetActiveCmdDescQuery { int32_t cmdIndex; };
struct GetActiveCmdDescResult { const Error* error; ActiveCommandDescription cmdDesc; bool hasCommand; };

struct GetActiveCmdDescsQuery { uint8_t _unused; };
struct GetActiveCmdDescsResult { const Error* error; ActiveCommandDescription* cmdDescs; uint32_t count; };

struct GetCmdDescIndexQuery { int32_t cmdID; };
struct GetCmdDescIndexResult { const Error* error; int32_t index; };

struct GetBoxSelectionByEngineQuery { uint8_t _unused; };
struct GetBoxSelectionByEngineResult { const Error* error; bool enabled; };

struct GetBuildFacingQuery { uint8_t _unused; };
struct GetBuildFacingResult { const Error* error; int32_t facing; };

struct GetBuildSpacingQuery { uint8_t _unused; };
struct GetBuildSpacingResult { const Error* error; int32_t spacing; };

struct GetDrawSelectionInfoQuery { uint8_t _unused; };
struct GetDrawSelectionInfoResult { const Error* error; bool draw; };

// Projectiles / render flags
struct GetNanoProjectileParamsQuery { uint8_t _unused; };
struct GetNanoProjectileParamsResult { const Error* error; float r; float v; float a; float randR; float randV; float randA; };

struct GetPieceProjectileNameQuery { int32_t projectileID; };
struct GetPieceProjectileNameResult { const Error* error; const char* name; };

struct GetTeamDamageStatsQuery { int32_t teamID; };
struct GetTeamDamageStatsResult { const Error* error; float damageDealt; float damageReceived; bool success; };

// Messages / misc
struct GetLastMessagePositionsQuery { uint8_t _unused; };
struct GetLastMessagePositionsResult { const Error* error; Float3* positions; uint32_t count; };

struct SolveNURBSCurveQuery {
	int32_t degree;
	const Float4* points;
	uint32_t pointCount;
	const float* knots;
	uint32_t knotCount;
	int32_t segments;
};
struct SolveNURBSCurveResult { const Error* error; Float3* points; uint32_t count; bool success; };

struct IsUnitSelectedQuery { int32_t unitID; };
struct IsUnitSelectedResult { const Error* error; bool selected; };

struct IsUnitAlliedQuery { int32_t unitID; };
struct IsUnitAlliedResult { const Error* error; bool allied; };

struct GetCustomPaletteColorQuery { int32_t index; };
struct GetCustomPaletteColorResult { const Error* error; float r; float g; float b; bool success; };

struct GetUnitPaletteIndexQuery { int32_t unitID; };
struct GetUnitPaletteIndexResult { const Error* error; int32_t customIndex; bool usingCustomColor; };

struct GetFeaturePaletteIndexQuery { int32_t featureID; };
struct GetFeaturePaletteIndexResult { const Error* error; int32_t customIndex; bool usingCustomColor; };

struct GetGameSecondsInterpolatedQuery { uint8_t _unused; };
struct GetGameSecondsInterpolatedResult { const Error* error; float seconds; };

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
	void (*GetUnitIcon)(const GetUnitIconQuery* query, GetUnitIconResult* result);
	void (*GetCameraRotation)(const GetCameraRotationQuery* query, GetCameraRotationResult* result);
	void (*GetCameraVectors)(const GetCameraVectorsQuery* query, GetCameraVectorsResult* result);
	void (*GetFrustumPlanes)(const GetFrustumPlanesQuery* query, GetFrustumPlanesResult* result);
	void (*GetVisibleUnits)(const GetVisibleUnitsQuery* query, GetVisibleUnitsResult* result);
	void (*GetVisibleFeatures)(const GetVisibleFeaturesQuery* query, GetVisibleFeaturesResult* result);
	void (*GetVisibleProjectiles)(const GetVisibleProjectilesQuery* query, GetVisibleProjectilesResult* result);
	void (*GetUnitsInScreenRectangle)(const GetUnitsInScreenRectangleQuery* query, GetUnitsInScreenRectangleResult* result);
	void (*GetFeaturesInScreenRectangle)(const GetFeaturesInScreenRectangleQuery* query, GetFeaturesInScreenRectangleResult* result);
	void (*IsUnitVisible)(const IsUnitVisibleQuery* query, IsUnitVisibleResult* result);
	void (*IsUnitInView)(const IsUnitInViewQuery* query, IsUnitInViewResult* result);
	void (*IsUnitIcon)(const IsUnitIconQuery* query, IsUnitIconResult* result);
};

struct UnsyncedReadApi {
	const UnitRenderingApi* unitRendering;
	void (*GetClipboard)(const GetClipboardQuery* query, GetClipboardResult* result);
	void (*GetPrevFrameSyncChecksum)(const GetPrevFrameSyncChecksumQuery* query, GetPrevFrameSyncChecksumResult* result);
	void (*GetActiveCmdDesc)(const GetActiveCmdDescQuery* query, GetActiveCmdDescResult* result);
	void (*GetActiveCmdDescs)(const GetActiveCmdDescsQuery* query, GetActiveCmdDescsResult* result);
	void (*GetCmdDescIndex)(const GetCmdDescIndexQuery* query, GetCmdDescIndexResult* result);
	void (*GetBoxSelectionByEngine)(const GetBoxSelectionByEngineQuery* query, GetBoxSelectionByEngineResult* result);
	void (*GetBuildFacing)(const GetBuildFacingQuery* query, GetBuildFacingResult* result);
	void (*GetBuildSpacing)(const GetBuildSpacingQuery* query, GetBuildSpacingResult* result);
	void (*GetDrawSelectionInfo)(const GetDrawSelectionInfoQuery* query, GetDrawSelectionInfoResult* result);
	void (*GetNanoProjectileParams)(const GetNanoProjectileParamsQuery* query, GetNanoProjectileParamsResult* result);
	void (*GetPieceProjectileName)(const GetPieceProjectileNameQuery* query, GetPieceProjectileNameResult* result);
	void (*GetTeamDamageStats)(const GetTeamDamageStatsQuery* query, GetTeamDamageStatsResult* result);
	void (*GetLastMessagePositions)(const GetLastMessagePositionsQuery* query, GetLastMessagePositionsResult* result);
	void (*SolveNURBSCurve)(const SolveNURBSCurveQuery* query, SolveNURBSCurveResult* result);
	void (*IsUnitSelected)(const IsUnitSelectedQuery* query, IsUnitSelectedResult* result);
	void (*IsUnitAllied)(const IsUnitAlliedQuery* query, IsUnitAlliedResult* result);
	void (*GetCustomPaletteColor)(const GetCustomPaletteColorQuery* query, GetCustomPaletteColorResult* result);
	void (*GetUnitPaletteIndex)(const GetUnitPaletteIndexQuery* query, GetUnitPaletteIndexResult* result);
	void (*GetFeaturePaletteIndex)(const GetFeaturePaletteIndexQuery* query, GetFeaturePaletteIndexResult* result);
	void (*GetGameSecondsInterpolated)(const GetGameSecondsInterpolatedQuery* query, GetGameSecondsInterpolatedResult* result);
};

extern const UnsyncedReadApi UNSYNCED_READ_API;

#ifdef __cplusplus
}
#endif
