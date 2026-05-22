#include "Display.h"

#include "Game/Game.h"
#include "Game/GlobalUnsynced.h"
#include "Sim/Misc/GlobalSynced.h"
#include "Game/UI/MiniMap.h"
#include "Game/Camera.h"
#include "Rendering/GlobalRendering.h"
#include "Rendering/Env/IWater.h"
#include "Rendering/Map/InfoTexture/IInfoTextureHandler.h"
#include "Sim/Misc/TeamHandler.h"
#include "Rendering/ShadowHandler.h"
#include "System/TimeProfiler.h"
#include "Map/ReadMap.h"
#include "Map/BaseGroundDrawer.h"
#include <string>
#include <cstring>

namespace {

// Scratch buffer for dynamic data
static thread_local char scratchBuffer[1024];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Display system not ready"
};

static const char* CopyToScratch(const std::string& str)
{
	const size_t len = str.size() + 1;
	if (bufferPos + len > sizeof(scratchBuffer))
		return "";

	char* ptr = &scratchBuffer[bufferPos];
	memcpy(ptr, str.c_str(), len);
	bufferPos += len;
	return ptr;
}

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

static void NativeGetDualViewGeometry(const GetDualViewGeometryQuery* query, GetDualViewGeometryResult* result)
{
	bufferPos = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->error = nullptr;
	result->geom.viewSizeX = globalRendering->dualViewSizeX;
	result->geom.viewSizeY = globalRendering->dualViewSizeY;
	result->geom.viewPosX = globalRendering->dualViewPosX;
	result->geom.viewPosY = globalRendering->dualViewPosY;
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
	result->geom.viewPosX = globalRendering->screenPosX;
	result->geom.viewPosY = globalRendering->screenPosY;
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

static void NativeGetMiniMapDualScreen(const GetMiniMapDualScreenQuery* query, GetMiniMapDualScreenResult* result)
{
	bufferPos = 0;

	if (!IsReady() || minimap == nullptr) {
		result->error = &NOT_READY_ERROR;
		result->dualScreen = false;
		result->position = "";
		return;
	}

	result->error = nullptr;
	result->dualScreen = globalRendering->dualScreenMode;
	if (!result->dualScreen) {
		result->position = "";
		return;
	}

	result->position = (globalRendering->dualScreenMiniMapOnLeft ? "left" : "right");
}

static void NativeGetMiniMapRotation(const GetMiniMapRotationQuery* query, GetMiniMapRotationResult* result)
{
	bufferPos = 0;

	if (minimap == nullptr) {
		result->error = &NOT_READY_ERROR;
		result->rotation = 0.0f;
		return;
	}

	result->error = nullptr;
	result->rotation = minimap->GetRotation();
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
	const uint32_t frame = globalRendering->drawFrame;
	result->low16 = frame & 0xFFFFu;
	result->high16 = frame >> 16;
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
	result->seconds = game->lastFrameTime.toSecsf();
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

static void NativeGetMapDrawMode(const GetMapDrawModeQuery* query, GetMapDrawModeResult* result)
{
	bufferPos = 0;

	if (infoTextureHandler == nullptr) {
		result->error = &NOT_READY_ERROR;
		result->mode = "";
		return;
	}

	const std::string& mode = infoTextureHandler->GetMode();
	const char* mapped = "";

	if (mode.empty()) {
		mapped = "normal";
	} else if (mode == "path") {
		mapped = "pathTraversability";
	} else if (mode == "heat") {
		mapped = "pathHeat";
	} else if (mode == "flow") {
		mapped = "pathFlow";
	} else if (mode == "pathcost") {
		mapped = "pathCost";
	} else {
		mapped = CopyToScratch(mode);
	}

	result->error = nullptr;
	result->mode = mapped;
}

static void NativeGetWaterMode(const GetWaterModeQuery* query, GetWaterModeResult* result)
{
	bufferPos = 0;

	const auto& water = IWater::GetWater();
	if (water == nullptr) {
		result->error = &NOT_READY_ERROR;
		result->mode = 0;
		result->name = "";
		return;
	}

	const int id = water->GetID();
	result->error = nullptr;
	result->mode = id;
	result->name = IWater::GetWaterName(static_cast<IWater::WATER_RENDERER>(id));
}

static void NativeGetLosViewColors(const GetLosViewColorsQuery* query, GetLosViewColorsResult* result)
{
	bufferPos = 0;

	if (readMap == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CBaseGroundDrawer* gd = readMap->GetGroundDrawer();
	if (gd == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const float scale = (float)CBaseGroundDrawer::losColorScale;
	const auto fillColor = [scale](const int* color, Float3& out) {
		out.x = color[0] / scale;
		out.y = color[1] / scale;
		out.z = color[2] / scale;
	};

	fillColor(gd->alwaysColor, result->alwaysColor);
	fillColor(gd->losColor, result->losColor);
	fillColor(gd->radarColor, result->radarColor);
	fillColor(gd->jamColor, result->jamColor);
	fillColor(gd->radarColor2, result->radarColor2);
	result->error = nullptr;
}

static void NativeGetGameSpeed(const GetGameSpeedQuery* query, GetGameSpeedResult* result)
{
	bufferPos = 0;

	if (game == nullptr || gs == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->error = nullptr;
	result->wantedSpeed = gs->wantedSpeedFactor;
	result->speed = gs->speedFactor;
	result->paused = gs->paused;
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
	.GetDualViewGeometry = NativeGetDualViewGeometry,
	.GetWindowGeometry = NativeGetWindowGeometry,
	.GetScreenGeometry = NativeGetScreenGeometry,
	.GetMiniMapGeometry = NativeGetMiniMapGeometry,
	.GetMiniMapDualScreen = NativeGetMiniMapDualScreen,
	.GetMiniMapRotation = NativeGetMiniMapRotation,

	.GetDrawFrame = NativeGetDrawFrame,
	.GetFrameTimeOffset = NativeGetFrameTimeOffset,
	.GetLastUpdateSeconds = NativeGetLastUpdateSeconds,

	.GetFPS = NativeGetFPS,
	.GetMapDrawMode = NativeGetMapDrawMode,
	.GetWaterMode = NativeGetWaterMode,
	.GetLosViewColors = NativeGetLosViewColors,
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
