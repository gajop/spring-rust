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

// Scratch buffer for dynamic data
static thread_local char scratchBuffer[8192];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
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
static void NativeGetNumDisplays(const GetNumDisplaysQuery* query, GetNumDisplaysResult* result)
{
	bufferPos = 0;

	result->error = nullptr;
	result->count = 1; // Simplified: single display
}

static void NativeGetViewGeometry(const GetViewGeometryQuery* query, GetViewGeometryResult* result)
{
	bufferPos = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->error = nullptr;
	result->geom.viewSizeX = globalRendering->viewSizeX;
	result->geom.viewSizeY = globalRendering->viewSizeY;
	result->geom.viewPosX = globalRendering->viewPosX;
	result->geom.viewPosY = globalRendering->viewPosY;
}

static void NativeGetWindowGeometry(const GetWindowGeometryQuery* query, GetWindowGeometryResult* result)
{
	bufferPos = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->error = nullptr;
	result->geom.viewSizeX = globalRendering->winSizeX;
	result->geom.viewSizeY = globalRendering->winSizeY;
	result->geom.viewPosX = globalRendering->winPosX;
	result->geom.viewPosY = globalRendering->winPosY;
}

static void NativeGetScreenGeometry(const GetScreenGeometryQuery* query, GetScreenGeometryResult* result)
{
	bufferPos = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	// Simplified: return window geometry (multi-screen not fully supported)
	result->error = nullptr;
	result->geom.viewSizeX = globalRendering->screenSizeX;
	result->geom.viewSizeY = globalRendering->screenSizeY;
	result->geom.viewPosX = 0;
	result->geom.viewPosY = 0;
}

static void NativeGetMiniMapGeometry(const GetMiniMapGeometryQuery* query, GetMiniMapGeometryResult* result)
{
	bufferPos = 0;

	if (minimap == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->error = nullptr;
	result->geom.sizeX = minimap->GetSizeX();
	result->geom.sizeY = minimap->GetSizeY();
	result->geom.posX = minimap->GetPosX();
	result->geom.posY = minimap->GetPosY();
	result->geom.minimized = minimap->GetMinimized();
	result->geom.maximized = minimap->GetMaximized();
}

// Frame info
static void NativeGetDrawFrame(const GetDrawFrameQuery* query, GetDrawFrameResult* result)
{
	bufferPos = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->error = nullptr;
	result->frame = globalRendering->drawFrame;
}

static void NativeGetFrameTimeOffset(const GetFrameTimeOffsetQuery* query, GetFrameTimeOffsetResult* result)
{
	bufferPos = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->error = nullptr;
	result->offset = globalRendering->timeOffset;
}

static void NativeGetLastUpdateSeconds(const GetLastUpdateSecondsQuery* query, GetLastUpdateSecondsResult* result)
{
	bufferPos = 0;

	if (game == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->error = nullptr;
	result->seconds = game->lastFrameTime;
}

// FPS and performance
static void NativeGetFPS(const GetFPSQuery* query, GetFPSResult* result)
{
	bufferPos = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->error = nullptr;
	result->fps = static_cast<uint32_t>(globalRendering->FPS);
}

static void NativeGetGameSpeed(const GetGameSpeedQuery* query, GetGameSpeedResult* result)
{
	bufferPos = 0;

	if (game == nullptr || gu == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->error = nullptr;
	result->speed = gs->wantedSpeedFactor;
}

// Team colors
static void NativeGetTeamColor(const GetTeamColorQuery* query, GetTeamColorResult* result)
{
	bufferPos = 0;

	if (!teamHandler.IsValidTeam(query->teamID)) {
		result->error = nullptr;
		result->color.r = 1.0f;
		result->color.g = 1.0f;
		result->color.b = 1.0f;
		result->color.a = 1.0f;
		return;
	}

	const CTeam* team = teamHandler.Team(query->teamID);
	const auto& color = team->color;

	result->error = nullptr;
	result->color.r = color[0];
	result->color.g = color[1];
	result->color.b = color[2];
	result->color.a = color[3];
}

static void NativeGetTeamOrigColor(const GetTeamOrigColorQuery* query, GetTeamOrigColorResult* result)
{
	bufferPos = 0;

	if (!teamHandler.IsValidTeam(query->teamID)) {
		result->error = nullptr;
		result->color.r = 1.0f;
		result->color.g = 1.0f;
		result->color.b = 1.0f;
		result->color.a = 1.0f;
		return;
	}

	const CTeam* team = teamHandler.Team(query->teamID);
	const auto& color = team->origColor;

	result->error = nullptr;
	result->color.r = color[0];
	result->color.g = color[1];
	result->color.b = color[2];
	result->color.a = color[3];
}

// Visibility queries
static void NativeIsAABBInView(const IsAABBInViewQuery* query, IsAABBInViewResult* result)
{
	bufferPos = 0;

	if (camera == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const float3 vMins(query->mins.x, query->mins.y, query->mins.z);
	const float3 vMaxs(query->maxs.x, query->maxs.y, query->maxs.z);

	result->error = nullptr;
	result->inView = camera->InView(vMins, vMaxs);
}

static void NativeIsSphereInView(const IsSphereInViewQuery* query, IsSphereInViewResult* result)
{
	bufferPos = 0;

	if (camera == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const float3 vCenter(query->center.x, query->center.y, query->center.z);
	result->error = nullptr;
	result->inView = camera->InView(vCenter, query->radius);
}

// GUI state
static void NativeIsGUIHidden(const IsGUIHiddenQuery* query, IsGUIHiddenResult* result)
{
	bufferPos = 0;

	result->error = nullptr;
	result->hidden = (game != nullptr) && game->hideInterface;
}

static void NativeHaveShadows(const HaveShadowsQuery* query, HaveShadowsResult* result)
{
	bufferPos = 0;

	result->error = nullptr;
	result->enabled = shadowHandler.ShadowsLoaded();
}

static void NativeHaveAdvShading(const HaveAdvShadingQuery* query, HaveAdvShadingResult* result)
{
	bufferPos = 0;

	result->error = nullptr;
	result->enabled = true; // Simplified: assume advanced shading available
}

// Control
static void NativeSetTeamColor(const SetTeamColorQuery* query, SetTeamColorResult* result)
{
	bufferPos = 0;

	if (!teamHandler.IsValidTeam(query->teamID)) {
		result->error = nullptr;
		result->success = false;
		return;
	}

	CTeam* team = teamHandler.Team(query->teamID);
	team->color[0] = query->color.r;
	team->color[1] = query->color.g;
	team->color[2] = query->color.b;
	team->color[3] = query->color.a;

	result->error = nullptr;
	result->success = true;
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
