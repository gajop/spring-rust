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

// Minimap geometry
struct MinimapGeometry {
	int32_t sizeX;
	int32_t sizeY;
	int32_t posX;
	int32_t posY;
	bool minimized;
	bool maximized;
};

// Team colors
struct TeamColor {
	float r;
	float g;
	float b;
	float a;
};

// Queries
struct GetNumDisplaysQuery {
	uint8_t _unused;
};

struct GetNumDisplaysResult {
	const Error* error;
	uint32_t count;
};

struct GetViewGeometryQuery {
	uint8_t _unused;
};

struct GetViewGeometryResult {
	const Error* error;
	ViewGeometry geom;
};

struct GetDualViewGeometryQuery {
	uint8_t _unused;
};

struct GetDualViewGeometryResult {
	const Error* error;
	ViewGeometry geom;
};

struct GetWindowGeometryQuery {
	uint8_t _unused;
};

struct GetWindowGeometryResult {
	const Error* error;
	ViewGeometry geom;
};

struct GetScreenGeometryQuery {
	int32_t screenNum;
};

struct GetScreenGeometryResult {
	const Error* error;
	ViewGeometry geom;
};

struct GetMiniMapGeometryQuery {
	uint8_t _unused;
};

struct GetMiniMapGeometryResult {
	const Error* error;
	MinimapGeometry geom;
};

struct GetMiniMapDualScreenQuery { uint8_t _unused; };
struct GetMiniMapDualScreenResult { const Error* error; const char* position; bool dualScreen; };

struct GetMiniMapRotationQuery { uint8_t _unused; };
struct GetMiniMapRotationResult { const Error* error; float rotation; };

struct GetDrawFrameQuery {
	uint8_t _unused;
};

struct GetDrawFrameResult {
	const Error* error;
	uint32_t low16;
	uint32_t high16;
};

struct GetFrameTimeOffsetQuery {
	uint8_t _unused;
};

struct GetFrameTimeOffsetResult {
	const Error* error;
	float offset;
};

struct GetLastUpdateSecondsQuery {
	uint8_t _unused;
};

struct GetLastUpdateSecondsResult {
	const Error* error;
	float seconds;
};

struct GetFPSQuery {
	uint8_t _unused;
};

struct GetFPSResult {
	const Error* error;
	uint32_t fps;
};

struct GetMapDrawModeQuery { uint8_t _unused; };
struct GetMapDrawModeResult { const Error* error; const char* mode; };

struct GetWaterModeQuery { uint8_t _unused; };
struct GetWaterModeResult { const Error* error; int32_t mode; const char* name; };

struct GetLosViewColorsQuery { uint8_t _unused; };
struct GetLosViewColorsResult {
	const Error* error;
	Float3 alwaysColor;
	Float3 losColor;
	Float3 radarColor;
	Float3 jamColor;
	Float3 radarColor2;
};

struct GetGameSpeedQuery {
	uint8_t _unused;
};

struct GetGameSpeedResult {
	const Error* error;
	float wantedSpeed;
	float speed;
	bool paused;
};

struct GetTeamColorQuery {
	int32_t teamID;
};

struct GetTeamColorResult {
	const Error* error;
	TeamColor color;
};

struct GetTeamOrigColorQuery {
	int32_t teamID;
};

struct GetTeamOrigColorResult {
	const Error* error;
	TeamColor color;
};

struct IsAABBInViewQuery {
	Float3 mins;
	Float3 maxs;
};

struct IsAABBInViewResult {
	const Error* error;
	bool inView;
};

struct IsSphereInViewQuery {
	Float3 center;
	float radius;
};

struct IsSphereInViewResult {
	const Error* error;
	bool inView;
};

struct IsGUIHiddenQuery {
	uint8_t _unused;
};

struct IsGUIHiddenResult {
	const Error* error;
	bool hidden;
};

struct HaveShadowsQuery {
	uint8_t _unused;
};

struct HaveShadowsResult {
	const Error* error;
	bool enabled;
};

struct HaveAdvShadingQuery {
	uint8_t _unused;
};

struct HaveAdvShadingResult {
	const Error* error;
	bool enabled;
};

struct SetTeamColorQuery {
	int32_t teamID;
	TeamColor color;
};

struct SetTeamColorResult {
	const Error* error;
	bool success;
};

// API structure
struct DisplayApi {
	void (*GetNumDisplays)(
		const GetNumDisplaysQuery* query,
		GetNumDisplaysResult* result
	);

	void (*GetViewGeometry)(
		const GetViewGeometryQuery* query,
		GetViewGeometryResult* result
	);

	void (*GetDualViewGeometry)(
		const GetDualViewGeometryQuery* query,
		GetDualViewGeometryResult* result
	);

	void (*GetWindowGeometry)(
		const GetWindowGeometryQuery* query,
		GetWindowGeometryResult* result
	);

	void (*GetScreenGeometry)(
		const GetScreenGeometryQuery* query,
		GetScreenGeometryResult* result
	);

	void (*GetMiniMapGeometry)(
		const GetMiniMapGeometryQuery* query,
		GetMiniMapGeometryResult* result
	);

	void (*GetMiniMapDualScreen)(
		const GetMiniMapDualScreenQuery* query,
		GetMiniMapDualScreenResult* result
	);

	void (*GetMiniMapRotation)(
		const GetMiniMapRotationQuery* query,
		GetMiniMapRotationResult* result
	);

	void (*GetDrawFrame)(
		const GetDrawFrameQuery* query,
		GetDrawFrameResult* result
	);

	void (*GetFrameTimeOffset)(
		const GetFrameTimeOffsetQuery* query,
		GetFrameTimeOffsetResult* result
	);

	void (*GetLastUpdateSeconds)(
		const GetLastUpdateSecondsQuery* query,
		GetLastUpdateSecondsResult* result
	);

	void (*GetFPS)(
		const GetFPSQuery* query,
		GetFPSResult* result
	);

	void (*GetMapDrawMode)(
		const GetMapDrawModeQuery* query,
		GetMapDrawModeResult* result
	);

	void (*GetWaterMode)(
		const GetWaterModeQuery* query,
		GetWaterModeResult* result
	);

	void (*GetLosViewColors)(
		const GetLosViewColorsQuery* query,
		GetLosViewColorsResult* result
	);

	void (*GetGameSpeed)(
		const GetGameSpeedQuery* query,
		GetGameSpeedResult* result
	);

	void (*GetTeamColor)(
		const GetTeamColorQuery* query,
		GetTeamColorResult* result
	);

	void (*GetTeamOrigColor)(
		const GetTeamOrigColorQuery* query,
		GetTeamOrigColorResult* result
	);

	void (*IsAABBInView)(
		const IsAABBInViewQuery* query,
		IsAABBInViewResult* result
	);

	void (*IsSphereInView)(
		const IsSphereInViewQuery* query,
		IsSphereInViewResult* result
	);

	void (*IsGUIHidden)(
		const IsGUIHiddenQuery* query,
		IsGUIHiddenResult* result
	);

	void (*HaveShadows)(
		const HaveShadowsQuery* query,
		HaveShadowsResult* result
	);

	void (*HaveAdvShading)(
		const HaveAdvShadingQuery* query,
		HaveAdvShadingResult* result
	);

	void (*SetTeamColor)(
		const SetTeamColorQuery* query,
		SetTeamColorResult* result
	);
};

extern const DisplayApi DISPLAY_API;

#ifdef __cplusplus
}
#endif
