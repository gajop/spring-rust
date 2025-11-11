#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Display API
// @see rts/Lua/LuaUnsyncedRead.cpp, LuaUnsyncedCtrl.cpp
//
// Display, window, and rendering queries/control (unsynced)
// ============================================================================

// View geometry
struct ViewGeometry {
	int32_t viewSizeX;
	int32_t viewSizeY;
	int32_t viewPosX;
	int32_t viewPosY;
};

struct ViewGeometryResult {
	const Error* error;
	ViewGeometry geom;
};

// Minimap geometry
struct MinimapGeometry {
	int32_t sizeX;
	int32_t sizeY;
	int32_t posX;
	int32_t posY;
	bool minimized;
	bool maximized;
};

struct MinimapGeometryResult {
	const Error* error;
	MinimapGeometry geom;
};

// Team colors
struct TeamColor {
	float r;
	float g;
	float b;
	float a;
};

struct TeamColorResult {
	const Error* error;
	TeamColor color;
};

// API structure
struct DisplayApi {
	// View geometry
	UInt32Result (*GetNumDisplays)();
	ViewGeometryResult (*GetViewGeometry)();
	ViewGeometryResult (*GetWindowGeometry)();
	ViewGeometryResult (*GetScreenGeometry)(int32_t screenNum);
	MinimapGeometryResult (*GetMiniMapGeometry)();

	// Frame info
	UInt32Result (*GetDrawFrame)();
	FloatResult (*GetFrameTimeOffset)();
	FloatResult (*GetLastUpdateSeconds)();

	// FPS and performance
	UInt32Result (*GetFPS)();
	FloatResult (*GetGameSpeed)();

	// Team colors
	TeamColorResult (*GetTeamColor)(int32_t teamID);
	TeamColorResult (*GetTeamOrigColor)(int32_t teamID);

	// Visibility queries
	BoolResult (*IsAABBInView)(Float3 mins, Float3 maxs);
	BoolResult (*IsSphereInView)(Float3 center, float radius);

	// GUI state
	BoolResult (*IsGUIHidden)();
	BoolResult (*HaveShadows)();
	BoolResult (*HaveAdvShading)();

	// Control
	BoolResult (*SetTeamColor)(int32_t teamID, TeamColor color);
};

extern const DisplayApi DISPLAY_API;

#ifdef __cplusplus
}
#endif
