#include "Display.h"

#include "Game/Game.h"
#include "Game/GlobalUnsynced.h"
#include "Game/UI/MiniMap.h"
#include "Game/Camera.h"
#include "Rendering/GlobalRendering.h"
#include "Sim/Misc/TeamHandler.h"
#include "Rendering/ShadowHandler.h"
#include "System/TimeProfiler.h"

namespace {

// Error constants
static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Display system not ready"
};

// Helper: check if ready
static bool IsReady()
{
	return (globalRendering != nullptr);
}

// View geometry
static UInt32Result NativeGetNumDisplays()
{
	UInt32Result result = {};
	result.value = 1; // Simplified: single display
	return result;
}

static ViewGeometryResult NativeGetViewGeometry()
{
	ViewGeometryResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	result.geom.viewSizeX = globalRendering->viewSizeX;
	result.geom.viewSizeY = globalRendering->viewSizeY;
	result.geom.viewPosX = globalRendering->viewPosX;
	result.geom.viewPosY = globalRendering->viewPosY;
	return result;
}

static ViewGeometryResult NativeGetWindowGeometry()
{
	ViewGeometryResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	result.geom.viewSizeX = globalRendering->winSizeX;
	result.geom.viewSizeY = globalRendering->winSizeY;
	result.geom.viewPosX = globalRendering->winPosX;
	result.geom.viewPosY = globalRendering->winPosY;
	return result;
}

static ViewGeometryResult NativeGetScreenGeometry(int32_t screenNum)
{
	ViewGeometryResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Simplified: return window geometry (multi-screen not fully supported)
	result.geom.viewSizeX = globalRendering->screenSizeX;
	result.geom.viewSizeY = globalRendering->screenSizeY;
	result.geom.viewPosX = 0;
	result.geom.viewPosY = 0;
	return result;
}

static MinimapGeometryResult NativeGetMiniMapGeometry()
{
	MinimapGeometryResult result = {};
	if (minimap == nullptr) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	result.geom.sizeX = minimap->GetSizeX();
	result.geom.sizeY = minimap->GetSizeY();
	result.geom.posX = minimap->GetPosX();
	result.geom.posY = minimap->GetPosY();
	result.geom.minimized = minimap->GetMinimized();
	result.geom.maximized = minimap->GetMaximized();
	return result;
}

// Frame info
static UInt32Result NativeGetDrawFrame()
{
	UInt32Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	result.value = globalRendering->drawFrame;
	return result;
}

static FloatResult NativeGetFrameTimeOffset()
{
	FloatResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	result.value = globalRendering->timeOffset;
	return result;
}

static FloatResult NativeGetLastUpdateSeconds()
{
	FloatResult result = {};
	if (game == nullptr) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	result.value = game->lastUpdateTime;
	return result;
}

// FPS and performance
static UInt32Result NativeGetFPS()
{
	UInt32Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	result.value = static_cast<uint32_t>(globalRendering->FPS);
	return result;
}

static FloatResult NativeGetGameSpeed()
{
	FloatResult result = {};
	if (game == nullptr || gu == nullptr) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	result.value = gu->simSpeed;
	return result;
}

// Team colors
static TeamColorResult NativeGetTeamColor(int32_t teamID)
{
	TeamColorResult result = {};

	if (!teamHandler.IsValidTeam(teamID)) {
		result.color.r = 1.0f;
		result.color.g = 1.0f;
		result.color.b = 1.0f;
		result.color.a = 1.0f;
		return result;
	}

	const CTeam* team = teamHandler.Team(teamID);
	const auto& color = team->color;

	result.color.r = color[0];
	result.color.g = color[1];
	result.color.b = color[2];
	result.color.a = color[3];
	return result;
}

static TeamColorResult NativeGetTeamOrigColor(int32_t teamID)
{
	TeamColorResult result = {};

	if (!teamHandler.IsValidTeam(teamID)) {
		result.color.r = 1.0f;
		result.color.g = 1.0f;
		result.color.b = 1.0f;
		result.color.a = 1.0f;
		return result;
	}

	const CTeam* team = teamHandler.Team(teamID);
	const auto& color = team->origColor;

	result.color.r = color[0];
	result.color.g = color[1];
	result.color.b = color[2];
	result.color.a = color[3];
	return result;
}

// Visibility queries
static BoolResult NativeIsAABBInView(Float3 mins, Float3 maxs)
{
	BoolResult result = {};
	if (camera == nullptr) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const float3 vMins(mins.x, mins.y, mins.z);
	const float3 vMaxs(maxs.x, maxs.y, maxs.z);

	result.value = camera->InView(vMins, vMaxs);
	return result;
}

static BoolResult NativeIsSphereInView(Float3 center, float radius)
{
	BoolResult result = {};
	if (camera == nullptr) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const float3 vCenter(center.x, center.y, center.z);
	result.value = camera->InView(vCenter, radius);
	return result;
}

// GUI state
static BoolResult NativeIsGUIHidden()
{
	BoolResult result = {};
	result.value = (game != nullptr) && game->hideInterface;
	return result;
}

static BoolResult NativeHaveShadows()
{
	BoolResult result = {};
	result.value = shadowHandler.ShadowsLoaded();
	return result;
}

static BoolResult NativeHaveAdvShading()
{
	BoolResult result = {};
	result.value = true; // Simplified: assume advanced shading available
	return result;
}

// Control
static BoolResult NativeSetTeamColor(int32_t teamID, TeamColor color)
{
	BoolResult result = {};

	if (!teamHandler.IsValidTeam(teamID)) {
		result.value = false;
		return result;
	}

	CTeam* team = teamHandler.Team(teamID);
	team->color[0] = color.r;
	team->color[1] = color.g;
	team->color[2] = color.b;
	team->color[3] = color.a;

	result.value = true;
	return result;
}

} // namespace

const DisplayApi DISPLAY_API = {
	.GetNumDisplays = NativeGetNumDisplays,
	.GetViewGeometry = NativeGetViewGeometry,
	.GetWindowGeometry = NativeGetWindowGeometry,
	.GetScreenGeometry = NativeGetScreenGeometry,
	.GetMiniMapGeometry = NativeGetMiniMapGeometry,

	.GetDrawFrame = NativeGetDrawFrame,
	.GetFrameTimeOffset = NativeGetFrameTimeOffset,
	.GetLastUpdateSeconds = NativeGetLastUpdateSeconds,

	.GetFPS = NativeGetFPS,
	.GetGameSpeed = NativeGetGameSpeed,

	.GetTeamColor = NativeGetTeamColor,
	.GetTeamOrigColor = NativeGetTeamOrigColor,

	.IsAABBInView = NativeIsAABBInView,
	.IsSphereInView = NativeIsSphereInView,

	.IsGUIHidden = NativeIsGUIHidden,
	.HaveShadows = NativeHaveShadows,
	.HaveAdvShading = NativeHaveAdvShading,

	.SetTeamColor = NativeSetTeamColor,
};
