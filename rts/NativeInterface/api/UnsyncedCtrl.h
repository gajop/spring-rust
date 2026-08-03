/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Unsynced Control API
// @see rts/Lua/LuaUnsyncedCtrl.cpp
//
// Client-side control for rendering and UI state (non-deterministic)
// ============================================================================

// Unit rendering/UI toggles
struct SetUnitNoDrawQuery { int32_t unitID; bool noDraw; };
struct SetUnitNoDrawResult { const Error* error; bool success; };

struct SetUnitEngineDrawMaskQuery { int32_t unitID; uint32_t drawMask; };
struct SetUnitEngineDrawMaskResult { const Error* error; bool success; };

struct SetUnitAlwaysUpdateMatrixQuery { int32_t unitID; bool alwaysUpdateMatrix; };
struct SetUnitAlwaysUpdateMatrixResult { const Error* error; bool success; };

struct SetUnitNoMinimapQuery { int32_t unitID; bool noMinimap; };
struct SetUnitNoMinimapResult { const Error* error; bool success; };

struct SetUnitNoGroupQuery { int32_t unitID; bool noGroup; };
struct SetUnitNoGroupResult { const Error* error; bool success; };

struct SetUnitNoSelectQuery { int32_t unitID; bool noSelect; };
struct SetUnitNoSelectResult { const Error* error; bool success; };

struct SetUnitLeaveTracksQuery { int32_t unitID; bool leaveTracks; };
struct SetUnitLeaveTracksResult { const Error* error; bool success; };

struct SetMiniMapRotationQuery { float radians; };
struct SetMiniMapRotationResult { const Error* error; bool success; int32_t rotation; };

struct SetClipboardQuery { const char* text; };
struct SetClipboardResult { const Error* error; bool success; };

struct SetMouseCursorQuery {
	const char* cursorName;
	float scale; // use 1.0f for default scale
};
struct SetMouseCursorResult { const Error* error; bool success; };

struct AssignMouseCursorQuery {
	const char* commandName;
	const char* cursorFileName;
	bool overwrite;
	bool hotSpotTopLeft;
};
struct AssignMouseCursorResult { const Error* error; bool success; };

struct ReplaceMouseCursorQuery {
	const char* oldCursorFileName;
	const char* newCursorFileName;
	bool hotSpotTopLeft;
};
struct ReplaceMouseCursorResult { const Error* error; bool success; };

struct WarpMouseQuery { int32_t x; int32_t y; };
struct WarpMouseResult { const Error* error; bool success; };

struct SetActiveCommandOptions {
	bool leftClick;
	bool rightClick;
	bool alt;
	bool ctrl;
	bool meta;
	bool shift;
};
struct SetActiveCommandQuery {
	int32_t cmdIndex;           // -1 to clear, or non-negative command index
	int32_t button;             // SDL button (use 1 for left)
	SetActiveCommandOptions options;
};
struct SetActiveCommandResult { const Error* error; bool success; };

struct SDLStartTextInputQuery { uint8_t _unused; };
struct SDLStartTextInputResult { const Error* error; bool success; };

struct SDLStopTextInputQuery { uint8_t _unused; };
struct SDLStopTextInputResult { const Error* error; bool success; };

struct SDLSetTextInputRectQuery { int32_t x; int32_t y; int32_t w; int32_t h; };
struct SDLSetTextInputRectResult { const Error* error; bool success; };

struct SetBoxSelectionByEngineQuery { bool state; };
struct SetBoxSelectionByEngineResult { const Error* error; bool success; };

struct SetBuildFacingQuery { int32_t facing; };
struct SetBuildFacingResult { const Error* error; bool success; };

struct SetBuildSpacingQuery { int32_t spacing; };
struct SetBuildSpacingResult { const Error* error; bool success; };

struct SetWindowGeometryOptions { bool fullScreen; bool borderless; };
struct SetWindowGeometryQuery {
	int32_t displayIndex;
	int32_t windowPosX;
	int32_t windowPosY;
	int32_t windowSizeX;
	int32_t windowSizeY;
	SetWindowGeometryOptions options;
};
struct SetWindowGeometryResult { const Error* error; bool success; };

struct SetWindowMinimizedQuery { uint8_t _unused; };
struct SetWindowMinimizedResult { const Error* error; bool minimized; };

struct SetWindowMaximizedQuery { uint8_t _unused; };
struct SetWindowMaximizedResult { const Error* error; bool maximized; };

struct SetWMCaptionQuery { const char* title; const char* titleShort; };
struct SetWMCaptionResult { const Error* error; bool success; };

struct SetWMIconQuery { const char* iconFileName; bool forceResolution; };
struct SetWMIconResult { const Error* error; bool success; };

struct SetVideoCapturingModeQuery { bool allowCaptureMode; };
struct SetVideoCapturingModeResult { const Error* error; bool success; };

struct RunDollyCameraQuery { float runtimeMs; };
struct RunDollyCameraResult { const Error* error; bool success; };

struct PauseDollyCameraQuery { float percent; };
struct PauseDollyCameraResult { const Error* error; bool success; };

struct ResumeDollyCameraQuery { uint8_t _unused; };
struct ResumeDollyCameraResult { const Error* error; bool success; };

struct SetDollyCameraModeQuery { int32_t mode; };
struct SetDollyCameraModeResult { const Error* error; bool success; };

struct SetDollyCameraPositionQuery { Float3 position; };
struct SetDollyCameraPositionResult { const Error* error; bool success; };

struct SetDollyCameraCurveQuery {
	int32_t degree;
	const Float4* controlPoints;
	uint32_t controlPointsCount;
	const float* knots;
	uint32_t knotsCount;
};
struct SetDollyCameraCurveResult { const Error* error; bool success; };

struct SetDollyCameraLookPositionQuery { Float3 position; };
struct SetDollyCameraLookPositionResult { const Error* error; bool success; };

struct SetDollyCameraLookUnitQuery { int32_t unitID; };
struct SetDollyCameraLookUnitResult { const Error* error; bool success; };

struct SetDollyCameraLookCurveQuery {
	int32_t degree;
	const Float4* controlPoints;
	uint32_t controlPointsCount;
	const float* knots;
	uint32_t knotsCount;
};
struct SetDollyCameraLookCurveResult { const Error* error; bool success; };

struct SetDollyCameraRelativeModeQuery { int32_t mode; };
struct SetDollyCameraRelativeModeResult { const Error* error; bool success; };

struct SetVideoCapturingTimeOffsetQuery { float timeOffset; };
struct SetVideoCapturingTimeOffsetResult { const Error* error; bool success; };

struct SetCameraOffsetQuery { Float3 posOffset; Float3 tiltOffset; };
struct SetCameraOffsetResult { const Error* error; bool success; };

struct SetDrawGroundQuery { bool drawGround; };
struct SetDrawGroundResult { const Error* error; bool success; };

struct SetDrawSkyQuery { bool drawSky; };
struct SetDrawSkyResult { const Error* error; bool success; };

struct SetDrawWaterQuery { bool drawWater; };
struct SetDrawWaterResult { const Error* error; bool success; };

struct SetDrawGroundDeferredQuery { bool drawDeferred; bool drawForward; };
struct SetDrawGroundDeferredResult { const Error* error; bool success; bool deferred; bool forward; };

struct SetDrawModelsDeferredQuery {
	bool drawUnitsDeferred;
	bool drawFeaturesDeferred;
	bool drawUnitsForward;
	bool drawFeaturesForward;
};
struct SetDrawModelsDeferredResult { const Error* error; bool success; bool unitsDeferred; bool featuresDeferred; bool unitsForward; bool featuresForward; };

struct SetAtmosphereQuery { AtmosphereParams params; };
struct SetAtmosphereResult { const Error* error; bool success; };

struct SetSunDirectionQuery { Float3 dir; float intensity; };
struct SetSunDirectionResult { const Error* error; bool success; };

struct SetSunLightingQuery { SunLightingParams params; };
struct SetSunLightingResult { const Error* error; bool success; };

struct SetWaterParamsQuery { WaterParams params; };
struct SetWaterParamsResult { const Error* error; bool success; };

struct SetMapShaderQuery { int32_t standardShaderID; int32_t deferredShaderID; };
struct SetMapShaderResult { const Error* error; bool success; };

struct SetMapShadingTextureQuery { const char* texType; const char* texName; int32_t num; };
struct SetMapShadingTextureResult { const Error* error; bool success; };

struct SetSkyBoxTextureQuery { const char* texName; };
struct SetSkyBoxTextureResult { const Error* error; bool success; };

// texType is "texture", "foamTexture" or "normalTexture" (mirrors the string keys
// of Lua Spring.SetWaterParams, which the value-typed WaterParams cannot carry).
struct SetWaterTextureQuery { const char* texType; const char* texName; };
struct SetWaterTextureResult { const Error* error; bool success; };
// Reads the current path back (for undo); texType as in SetWaterTexture.
struct GetWaterTextureQuery { const char* texType; };
struct GetWaterTextureResult { const Error* error; const char* texName; };

struct SetMapRenderingParamsQuery { MapRenderingParams params; };
struct SetMapRenderingParamsResult { const Error* error; bool success; };

struct SetLosViewColorsQuery { RgbColor always; RgbColor los; RgbColor radar; RgbColor jam; RgbColor radar2; };
struct SetLosViewColorsResult { const Error* error; bool success; };

struct SetDrawSelectionInfoQuery { bool draw; };
struct SetDrawSelectionInfoResult { const Error* error; bool success; };

struct SetShockFrontFactorsOptions {
	float minArea;
	bool hasMinArea;
	float minPower;
	bool hasMinPower;
	float distAdj;
	bool hasDistAdj;
};
struct SetShockFrontFactorsQuery { SetShockFrontFactorsOptions options; };
struct SetShockFrontFactorsResult { const Error* error; bool success; };

struct SetCustomCommandDrawDataQuery { int32_t cmdID; DefRef cmdReference; Float4 color; bool showArea; };
struct SetCustomCommandDrawDataResult { const Error* error; bool success; };

struct SetLastMessagePositionQuery { Float3 pos; };
struct SetLastMessagePositionResult { const Error* error; bool success; };

struct LoadCmdColorsConfigQuery { const char* filename; };
struct LoadCmdColorsConfigResult { const Error* error; bool success; };

struct LoadCtrlPanelConfigQuery { const char* filename; };
struct LoadCtrlPanelConfigResult { const Error* error; bool success; };

struct LoadModelTexturesQuery { const char* modelName; };
struct LoadModelTexturesResult { const Error* error; bool success; };

struct ForceLayoutUpdateQuery { uint8_t _unused; };
struct ForceLayoutUpdateResult { const Error* error; bool success; };

struct ForceTesselationUpdateQuery { bool normal; bool shadow; };
struct ForceTesselationUpdateResult { const Error* error; bool success; };

struct SetAutoShowMetalQuery { bool enable; };
struct SetAutoShowMetalResult { const Error* error; bool success; };

struct SetUnitIconDrawQuery { int32_t unitID; bool drawIcon; };
struct SetUnitIconDrawResult { const Error* error; bool success; };

struct SetUnitIconQuery { int32_t unitID; const char* iconName; };
struct SetUnitIconResult { const Error* error; bool success; };

struct SetUnitDefIconQuery { int32_t unitDefID; const char* iconName; };
struct SetUnitDefIconResult { const Error* error; bool success; };

struct SetUnitDefImageQuery { int32_t unitDefID; const char* image; };
struct SetUnitDefImageResult { const Error* error; bool success; };

struct SetCustomPaletteColorQuery { int32_t index; float r; float g; float b; };
struct SetCustomPaletteColorResult { const Error* error; bool success; };

struct SetUnitPaletteIndexQuery { int32_t unitID; int32_t customIndex; };
struct SetUnitPaletteIndexResult { const Error* error; bool success; };

struct SetFeaturePaletteIndexQuery { int32_t featureID; int32_t customIndex; };
struct SetFeaturePaletteIndexResult { const Error* error; bool success; };

struct SetEngineBuildSquareRenderingQuery { bool enabled; };
struct SetEngineBuildSquareRenderingResult { const Error* error; bool success; };

struct SetFeatureNoDrawQuery { int32_t featureID; bool noDraw; };
struct SetFeatureNoDrawResult { const Error* error; bool success; };

struct SetFeatureEngineDrawMaskQuery { int32_t featureID; uint32_t mask; };
struct SetFeatureEngineDrawMaskResult { const Error* error; bool success; };

struct SetFeatureAlwaysUpdateMatrixQuery { int32_t featureID; bool enable; };
struct SetFeatureAlwaysUpdateMatrixResult { const Error* error; bool success; };

struct SetFeatureFadeQuery { int32_t featureID; bool allow; };
struct SetFeatureFadeResult { const Error* error; bool success; };

struct SetNanoProjectileParamsQuery { float r; float v; float a; float randR; float randV; float randA; };
struct SetNanoProjectileParamsResult { const Error* error; bool success; };

struct PreloadFeatureDefModelQuery { int32_t defID; };
struct PreloadFeatureDefModelResult { const Error* error; bool success; };

struct PreloadUnitDefModelQuery { int32_t defID; };
struct PreloadUnitDefModelResult { const Error* error; bool success; };

struct SelectUnitMapQuery { const int32_t* unitIDs; uint32_t count; bool append; };
struct SelectUnitMapResult { const Error* error; bool success; };

struct DeselectUnitMapQuery { const int32_t* unitIDs; uint32_t count; };
struct DeselectUnitMapResult { const Error* error; bool success; };

struct DrawUnitCommandsQuery { const int32_t* unitIDs; uint32_t count; bool tableOrArray; int32_t queueDrawDepth; };
struct DrawUnitCommandsResult { const Error* error; bool success; };

struct UnsyncedCtrlApi {
	void (*SetUnitNoDraw)(const SetUnitNoDrawQuery* query, SetUnitNoDrawResult* result);
	void (*SetUnitEngineDrawMask)(const SetUnitEngineDrawMaskQuery* query, SetUnitEngineDrawMaskResult* result);
	void (*SetUnitAlwaysUpdateMatrix)(const SetUnitAlwaysUpdateMatrixQuery* query, SetUnitAlwaysUpdateMatrixResult* result);
	void (*SetUnitNoMinimap)(const SetUnitNoMinimapQuery* query, SetUnitNoMinimapResult* result);
	void (*SetUnitNoGroup)(const SetUnitNoGroupQuery* query, SetUnitNoGroupResult* result);
	void (*SetUnitNoSelect)(const SetUnitNoSelectQuery* query, SetUnitNoSelectResult* result);
	void (*SetUnitLeaveTracks)(const SetUnitLeaveTracksQuery* query, SetUnitLeaveTracksResult* result);
	void (*SetMiniMapRotation)(const SetMiniMapRotationQuery* query, SetMiniMapRotationResult* result);
	void (*SetClipboard)(const SetClipboardQuery* query, SetClipboardResult* result);
	void (*SetMouseCursor)(const SetMouseCursorQuery* query, SetMouseCursorResult* result);
	void (*AssignMouseCursor)(const AssignMouseCursorQuery* query, AssignMouseCursorResult* result);
	void (*ReplaceMouseCursor)(const ReplaceMouseCursorQuery* query, ReplaceMouseCursorResult* result);
	void (*WarpMouse)(const WarpMouseQuery* query, WarpMouseResult* result);
	void (*SetActiveCommand)(const SetActiveCommandQuery* query, SetActiveCommandResult* result);
	void (*SDLStartTextInput)(const SDLStartTextInputQuery* query, SDLStartTextInputResult* result);
	void (*SDLStopTextInput)(const SDLStopTextInputQuery* query, SDLStopTextInputResult* result);
	void (*SDLSetTextInputRect)(const SDLSetTextInputRectQuery* query, SDLSetTextInputRectResult* result);
	void (*SetBoxSelectionByEngine)(const SetBoxSelectionByEngineQuery* query, SetBoxSelectionByEngineResult* result);
	void (*SetBuildFacing)(const SetBuildFacingQuery* query, SetBuildFacingResult* result);
	void (*SetBuildSpacing)(const SetBuildSpacingQuery* query, SetBuildSpacingResult* result);
	void (*SetWindowGeometry)(const SetWindowGeometryQuery* query, SetWindowGeometryResult* result);
	void (*SetWindowMinimized)(const SetWindowMinimizedQuery* query, SetWindowMinimizedResult* result);
	void (*SetWindowMaximized)(const SetWindowMaximizedQuery* query, SetWindowMaximizedResult* result);
	void (*SetWMCaption)(const SetWMCaptionQuery* query, SetWMCaptionResult* result);
	void (*SetWMIcon)(const SetWMIconQuery* query, SetWMIconResult* result);
	void (*SetVideoCapturingMode)(const SetVideoCapturingModeQuery* query, SetVideoCapturingModeResult* result);
	void (*RunDollyCamera)(const RunDollyCameraQuery* query, RunDollyCameraResult* result);
	void (*PauseDollyCamera)(const PauseDollyCameraQuery* query, PauseDollyCameraResult* result);
	void (*ResumeDollyCamera)(const ResumeDollyCameraQuery* query, ResumeDollyCameraResult* result);
	void (*SetDollyCameraMode)(const SetDollyCameraModeQuery* query, SetDollyCameraModeResult* result);
	void (*SetDollyCameraPosition)(const SetDollyCameraPositionQuery* query, SetDollyCameraPositionResult* result);
	void (*SetDollyCameraCurve)(const SetDollyCameraCurveQuery* query, SetDollyCameraCurveResult* result);
	void (*SetDollyCameraLookPosition)(const SetDollyCameraLookPositionQuery* query, SetDollyCameraLookPositionResult* result);
	void (*SetDollyCameraLookUnit)(const SetDollyCameraLookUnitQuery* query, SetDollyCameraLookUnitResult* result);
	void (*SetDollyCameraLookCurve)(const SetDollyCameraLookCurveQuery* query, SetDollyCameraLookCurveResult* result);
	void (*SetDollyCameraRelativeMode)(const SetDollyCameraRelativeModeQuery* query, SetDollyCameraRelativeModeResult* result);
	void (*SetVideoCapturingTimeOffset)(const SetVideoCapturingTimeOffsetQuery* query, SetVideoCapturingTimeOffsetResult* result);
	void (*SetCameraOffset)(const SetCameraOffsetQuery* query, SetCameraOffsetResult* result);
	void (*SetDrawGround)(const SetDrawGroundQuery* query, SetDrawGroundResult* result);
	void (*SetDrawSky)(const SetDrawSkyQuery* query, SetDrawSkyResult* result);
	void (*SetDrawWater)(const SetDrawWaterQuery* query, SetDrawWaterResult* result);
	void (*SetDrawGroundDeferred)(const SetDrawGroundDeferredQuery* query, SetDrawGroundDeferredResult* result);
	void (*SetDrawModelsDeferred)(const SetDrawModelsDeferredQuery* query, SetDrawModelsDeferredResult* result);
	void (*SetAtmosphere)(const SetAtmosphereQuery* query, SetAtmosphereResult* result);
	void (*SetSunDirection)(const SetSunDirectionQuery* query, SetSunDirectionResult* result);
	void (*SetSunLighting)(const SetSunLightingQuery* query, SetSunLightingResult* result);
	void (*SetWaterParams)(const SetWaterParamsQuery* query, SetWaterParamsResult* result);
	void (*SetMapShader)(const SetMapShaderQuery* query, SetMapShaderResult* result);
	void (*SetMapShadingTexture)(const SetMapShadingTextureQuery* query, SetMapShadingTextureResult* result);
	void (*SetSkyBoxTexture)(const SetSkyBoxTextureQuery* query, SetSkyBoxTextureResult* result);
	void (*SetMapRenderingParams)(const SetMapRenderingParamsQuery* query, SetMapRenderingParamsResult* result);
	void (*SetLosViewColors)(const SetLosViewColorsQuery* query, SetLosViewColorsResult* result);
	void (*SetDrawSelectionInfo)(const SetDrawSelectionInfoQuery* query, SetDrawSelectionInfoResult* result);
	void (*SetShockFrontFactors)(const SetShockFrontFactorsQuery* query, SetShockFrontFactorsResult* result);
	void (*SetCustomCommandDrawData)(const SetCustomCommandDrawDataQuery* query, SetCustomCommandDrawDataResult* result);
	void (*SetLastMessagePosition)(const SetLastMessagePositionQuery* query, SetLastMessagePositionResult* result);
	void (*LoadCmdColorsConfig)(const LoadCmdColorsConfigQuery* query, LoadCmdColorsConfigResult* result);
	void (*LoadCtrlPanelConfig)(const LoadCtrlPanelConfigQuery* query, LoadCtrlPanelConfigResult* result);
	void (*LoadModelTextures)(const LoadModelTexturesQuery* query, LoadModelTexturesResult* result);
	void (*ForceLayoutUpdate)(const ForceLayoutUpdateQuery* query, ForceLayoutUpdateResult* result);
	void (*ForceTesselationUpdate)(const ForceTesselationUpdateQuery* query, ForceTesselationUpdateResult* result);
	void (*SetAutoShowMetal)(const SetAutoShowMetalQuery* query, SetAutoShowMetalResult* result);
	void (*SetUnitIconDraw)(const SetUnitIconDrawQuery* query, SetUnitIconDrawResult* result);
	void (*SetUnitIcon)(const SetUnitIconQuery* query, SetUnitIconResult* result);
	void (*SetUnitDefIcon)(const SetUnitDefIconQuery* query, SetUnitDefIconResult* result);
	void (*SetUnitDefImage)(const SetUnitDefImageQuery* query, SetUnitDefImageResult* result);
	void (*SetCustomPaletteColor)(const SetCustomPaletteColorQuery* query, SetCustomPaletteColorResult* result);
	void (*SetUnitPaletteIndex)(const SetUnitPaletteIndexQuery* query, SetUnitPaletteIndexResult* result);
	void (*SetFeaturePaletteIndex)(const SetFeaturePaletteIndexQuery* query, SetFeaturePaletteIndexResult* result);
	void (*SetEngineBuildSquareRendering)(const SetEngineBuildSquareRenderingQuery* query, SetEngineBuildSquareRenderingResult* result);
	void (*SetFeatureNoDraw)(const SetFeatureNoDrawQuery* query, SetFeatureNoDrawResult* result);
	void (*SetFeatureEngineDrawMask)(const SetFeatureEngineDrawMaskQuery* query, SetFeatureEngineDrawMaskResult* result);
	void (*SetFeatureAlwaysUpdateMatrix)(const SetFeatureAlwaysUpdateMatrixQuery* query, SetFeatureAlwaysUpdateMatrixResult* result);
	void (*SetFeatureFade)(const SetFeatureFadeQuery* query, SetFeatureFadeResult* result);
	void (*SetNanoProjectileParams)(const SetNanoProjectileParamsQuery* query, SetNanoProjectileParamsResult* result);
	void (*PreloadFeatureDefModel)(const PreloadFeatureDefModelQuery* query, PreloadFeatureDefModelResult* result);
	void (*PreloadUnitDefModel)(const PreloadUnitDefModelQuery* query, PreloadUnitDefModelResult* result);
	void (*SelectUnitMap)(const SelectUnitMapQuery* query, SelectUnitMapResult* result);
	void (*DeselectUnitMap)(const DeselectUnitMapQuery* query, DeselectUnitMapResult* result);
	void (*DrawUnitCommands)(const DrawUnitCommandsQuery* query, DrawUnitCommandsResult* result);
	void (*SetWaterTexture)(const SetWaterTextureQuery* query, SetWaterTextureResult* result);
	void (*GetWaterTexture)(const GetWaterTextureQuery* query, GetWaterTextureResult* result);
};

extern const UnsyncedCtrlApi UNSYNCED_CTRL_API;

#ifdef __cplusplus
}
#endif
