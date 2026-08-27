/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "UnsyncedCtrl.h"
#include "Gfx.h"

#include <cmath>
#include <algorithm>
#include <vector>
#include <string>
#include <SDL_clipboard.h>
#include <SDL_events.h>
#include <SDL_mouse.h>

#include "Game/SelectedUnitsHandler.h"
#include "Game/GlobalUnsynced.h"
#include "Game/CameraHandler.h"
#include "Game/Camera.h"
#include "Game/Camera/DollyController.h"
#include "Game/UI/MouseHandler.h"
#include "Game/UI/GuiHandler.h"
#include "Game/UI/CursorIcons.h"
#include "Game/UI/MiniMap.h"
#include "Game/UI/KeySet.h"
#include "Game/UI/CommandColors.h"
#include "Rendering/Env/IGroundDecalDrawer.h"
#include "Rendering/GL/myGL.h"
#include "Rendering/GlobalRendering.h"
#include "Rendering/Env/ISky.h"
#include "Rendering/Env/SunLighting.h"
#include "Rendering/Env/WaterRendering.h"
#include "Rendering/Env/MapRendering.h"
#include "Rendering/Env/IWater.h"
#include "Rendering/Textures/Bitmap.h"
#include "Rendering/Textures/NamedTextures.h"
#include "Rendering/Textures/S3OTextureHandler.h"
#include "Rendering/Units/UnitDrawer.h"
#include "Rendering/Features/FeatureDrawer.h"
#include "Rendering/CommandDrawer.h"
#include "Rendering/Env/Particles/Classes/NanoProjectile.h"
#include "Rendering/Models/IModelParser.h"
#include "Map/ReadMap.h"
#include "Map/BaseGroundDrawer.h"
#include "Map/SMF/SMFGroundDrawer.h"
#include "Map/SMF/ROAM/RoamMeshDrawer.h"
#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Units/UnitDefHandler.h"
#include "Sim/Features/FeatureHandler.h"
#include "Sim/Features/FeatureDefHandler.h"
#include "Sim/Misc/CustomColorPalette.h"
#include "Sim/Misc/GlobalConstants.h"
#include "Rendering/IconHandler.h"
#include "System/Platform/WindowManagerHelper.h"
#include "System/MathConstants.h"
#include "Game/Action.h"
#include "System/StringUtil.h"
#include "Game/IVideoCapturing.h"
#include "System/type2.h"
#include "System/EventHandler.h"
#include "Lua/LuaUI.h"

#ifndef SDL_BUTTON_LEFT
#define SDL_BUTTON_LEFT 1
#define SDL_BUTTON_MIDDLE 2
#define SDL_BUTTON_RIGHT 3
#endif

namespace {

static const Error INVALID_UNIT_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid unit ID"
};

static const Error INVALID_FEATURE_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid feature ID"
};

static const Error INVALID_ARGUMENT_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid argument"
};

static const Error GUI_UNAVAILABLE_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "GUI handler not available"
};

static const Error RENDERING_UNAVAILABLE_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Rendering context not available"
};

static const Error MINIMAP_UNAVAILABLE_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Minimap not available or rotation locked"
};

static const Error DECALS_UNAVAILABLE_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Ground decals system not ready"
};

static const Error MAP_UNAVAILABLE_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Map rendering not available"
};

static const Error INVALID_TEXTURE_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid texture"
};

static const Error MOUSE_UNAVAILABLE_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Mouse handler not available"
};

static const Error LUAUI_UNAVAILABLE_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "LuaUI not available"
};

static const Error INVALID_CURSOR_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Cursor name is null"
};

static const Error INVALID_CURSOR_MAPPING_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Command or cursor file name is null"
};

static const Error CLIPBOARD_ERROR = {
	.code = ERROR_OPERATION_FAILED,
	.message = "Failed to set clipboard text"
};

static const Error TEXT_INPUT_ERROR = {
	.code = ERROR_OPERATION_FAILED,
	.message = "Failed to configure text input"
};

static const Error ICON_LOAD_ERROR = {
	.code = ERROR_OPERATION_FAILED,
	.message = "Failed to load window icon"
};

static const Error INVALID_ICON_NAME_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Icon file name is null"
};

static const Error INVALID_UNIT_ICON_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid icon name"
};

static const Error INVALID_UNIT_IMAGE_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid unit image"
};

static const Error INVALID_CAPTION_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Window caption is null"
};

static const Error INVALID_WINDOW_GEOMETRY_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid window geometry parameters"
};

static const Error VIDEO_CAPTURE_UNAVAILABLE_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Video capturing not available"
};

static const Error NOT_AVAILABLE_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Not available in Native API"
};

static CUnit* GetUnit(const SetUnitNoDrawQuery* query) {
	return unitHandler.GetUnit(query->unitID);
}

static CUnit* GetUnit(const SetUnitEngineDrawMaskQuery* query) {
	return unitHandler.GetUnit(query->unitID);
}

static CUnit* GetUnit(const SetUnitAlwaysUpdateMatrixQuery* query) {
	return unitHandler.GetUnit(query->unitID);
}

static CUnit* GetUnit(const SetUnitNoMinimapQuery* query) {
	return unitHandler.GetUnit(query->unitID);
}

static CUnit* GetUnit(const SetUnitLuaDrawQuery* query) {
	return unitHandler.GetUnit(query->unitID);
}

static CUnit* GetUnit(const SetUnitNoGroupQuery* query) {
	return unitHandler.GetUnit(query->unitID);
}

static CUnit* GetUnit(const SetUnitNoSelectQuery* query) {
	return unitHandler.GetUnit(query->unitID);
}

static CUnit* GetUnit(const SetUnitLeaveTracksQuery* query) {
	return unitHandler.GetUnit(query->unitID);
}

static void NativeSetUnitNoDraw(const SetUnitNoDrawQuery* query, SetUnitNoDrawResult* result)
{
	result->error = nullptr;
	result->success = false;

	CUnit* unit = GetUnit(query);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->noDraw = query->noDraw;
	result->success = true;
}

static void NativeSetUnitEngineDrawMask(const SetUnitEngineDrawMaskQuery* query, SetUnitEngineDrawMaskResult* result)
{
	result->error = nullptr;
	result->success = false;

	CUnit* unit = GetUnit(query);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->engineDrawMask = static_cast<uint8_t>(query->drawMask);
	result->success = true;
}

static void NativeSetUnitAlwaysUpdateMatrix(const SetUnitAlwaysUpdateMatrixQuery* query, SetUnitAlwaysUpdateMatrixResult* result)
{
	result->error = nullptr;
	result->success = false;

	CUnit* unit = GetUnit(query);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->alwaysUpdateMat = query->alwaysUpdateMatrix;
	result->success = true;
}

static void NativeSetUnitLuaDraw(const SetUnitLuaDrawQuery* query, SetUnitLuaDrawResult* result)
{
	result->error = nullptr;
	result->success = false;

	CUnit* unit = GetUnit(query);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->luaDraw = query->luaDraw;
	result->success = true;
}

static void NativeSetUnitNoMinimap(const SetUnitNoMinimapQuery* query, SetUnitNoMinimapResult* result)
{
	result->error = nullptr;
	result->success = false;

	CUnit* unit = GetUnit(query);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->noMinimap = query->noMinimap;
	result->success = true;
}

static void NativeSetUnitNoGroup(const SetUnitNoGroupQuery* query, SetUnitNoGroupResult* result)
{
	result->error = nullptr;
	result->success = false;

	CUnit* unit = GetUnit(query);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->noGroup = query->noGroup;
	if (unit->noGroup) {
		unit->SetGroup(nullptr);
	}

	result->success = true;
}

static void NativeSetUnitNoSelect(const SetUnitNoSelectQuery* query, SetUnitNoSelectResult* result)
{
	result->error = nullptr;
	result->success = false;

	CUnit* unit = GetUnit(query);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->noSelect = query->noSelect;

	if (unit->noSelect) {
		const auto& selUnits = selectedUnitsHandler.selectedUnits;
		if (selUnits.find(unit->id) != selUnits.end()) {
			selectedUnitsHandler.RemoveUnit(unit);
		}
	}

	result->success = true;
}

static void NativeSetUnitLeaveTracks(const SetUnitLeaveTracksQuery* query, SetUnitLeaveTracksResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (groundDecals == nullptr) {
		result->error = &DECALS_UNAVAILABLE_ERROR;
		return;
	}

	CUnit* unit = GetUnit(query);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	groundDecals->SetUnitLeaveTracks(unit, query->leaveTracks);
	result->success = true;
}

static void NativeSetMiniMapRotation(const SetMiniMapRotationQuery* query, SetMiniMapRotationResult* result)
{
	result->error = nullptr;
	result->success = false;
	result->rotation = 0;

	if (minimap == nullptr || minimap->minimapCanFlip) {
		result->error = &MINIMAP_UNAVAILABLE_ERROR;
		return;
	}

	const float quad = query->radians / math::HALFPI;
	const float wrapped = std::fmod(std::fmod(quad, 4.0f) + 4.0f, 4.0f);
	const int rotation = static_cast<int>(std::round(wrapped)) % 4;

	minimap->SetRotation(CMiniMap::RotationOptions(rotation));
	result->rotation = rotation;
	result->success = true;
}

static void NativeSetClipboard(const SetClipboardQuery* query, SetClipboardResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (query->text == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const int rc = SDL_SetClipboardText(query->text);
	if (rc != 0) {
		result->error = &CLIPBOARD_ERROR;
		return;
	}

	result->success = true;
}

static void NativeSetMouseCursor(const SetMouseCursorQuery* query, SetMouseCursorResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (mouse == nullptr) {
		result->error = &MOUSE_UNAVAILABLE_ERROR;
		return;
	}

	if (query->cursorName == nullptr) {
		result->error = &INVALID_CURSOR_ERROR;
		return;
	}

	const float scale = (query->scale < 0.0f) ? 1.0f : query->scale;
	mouse->ChangeCursor(query->cursorName, scale);
	result->success = true;
}

static void NativeAssignMouseCursor(const AssignMouseCursorQuery* query, AssignMouseCursorResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (mouse == nullptr) {
		result->error = &MOUSE_UNAVAILABLE_ERROR;
		return;
	}

	if (query->commandName == nullptr || query->cursorFileName == nullptr) {
		result->error = &INVALID_CURSOR_MAPPING_ERROR;
		return;
	}

	const CMouseCursor::HotSpot hotSpot = query->hotSpotTopLeft ? CMouseCursor::TopLeft : CMouseCursor::Center;
	result->success = mouse->AssignMouseCursor(query->commandName, query->cursorFileName, hotSpot, query->overwrite);
}

static void NativeReplaceMouseCursor(const ReplaceMouseCursorQuery* query, ReplaceMouseCursorResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (mouse == nullptr) {
		result->error = &MOUSE_UNAVAILABLE_ERROR;
		return;
	}

	if (query->oldCursorFileName == nullptr || query->newCursorFileName == nullptr) {
		result->error = &INVALID_CURSOR_MAPPING_ERROR;
		return;
	}

	const CMouseCursor::HotSpot hotSpot = query->hotSpotTopLeft ? CMouseCursor::TopLeft : CMouseCursor::Center;
	result->success = mouse->ReplaceMouseCursor(query->oldCursorFileName, query->newCursorFileName, hotSpot);
}

static void NativeWarpMouse(const WarpMouseQuery* query, WarpMouseResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (mouse == nullptr) {
		result->error = &MOUSE_UNAVAILABLE_ERROR;
		return;
	}

	if (globalRendering == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}

	const int x = query->x;
	const int y = globalRendering->viewSizeY - query->y - 1;

	mouse->WarpMouse(x, y);
	result->success = true;
}

static void NativeSetActiveCommand(const SetActiveCommandQuery* query, SetActiveCommandResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (guihandler == nullptr) {
		result->error = &GUI_UNAVAILABLE_ERROR;
		return;
	}

	// Index path
	const int cmdIndex = query->cmdIndex;
	const int button = (query->button == 0) ? SDL_BUTTON_LEFT : query->button;

	if (cmdIndex < 0) {
		result->success = guihandler->SetActiveCommand(-1, false);
		return;
	}

	const bool hasClickState = query->options.leftClick || query->options.rightClick || query->options.alt || query->options.ctrl || query->options.meta || query->options.shift;

	if (hasClickState) {
		result->success = guihandler->SetActiveCommand(cmdIndex, button, query->options.leftClick, query->options.rightClick, query->options.alt, query->options.ctrl, query->options.meta, query->options.shift);
	} else {
		result->success = guihandler->SetActiveCommand(cmdIndex, button != SDL_BUTTON_LEFT);
	}
}

static void NativeSDLStartTextInput(const SDLStartTextInputQuery* /*query*/, SDLStartTextInputResult* result)
{
	result->error = nullptr;
	result->success = false;

	SDL_StartTextInput();
	result->success = true;
}

static void NativeSDLStopTextInput(const SDLStopTextInputQuery* /*query*/, SDLStopTextInputResult* result)
{
	result->error = nullptr;
	result->success = false;

	SDL_StopTextInput();
	result->success = true;
}

static void NativeSDLSetTextInputRect(const SDLSetTextInputRectQuery* query, SDLSetTextInputRectResult* result)
{
	result->error = nullptr;
	result->success = false;

	SDL_Rect rect;
	rect.x = query->x;
	rect.y = query->y;
	rect.w = query->w;
	rect.h = query->h;

	SDL_SetTextInputRect(&rect);
	result->success = true;
}

static void NativeSetBoxSelectionByEngine(const SetBoxSelectionByEngineQuery* query, SetBoxSelectionByEngineResult* result)
{
	result->error = nullptr;
	result->success = false;

	selectedUnitsHandler.SetBoxSelectionHandledByEngine(query->state);
	result->success = true;
}

static void NativeSetBuildFacing(const SetBuildFacingQuery* query, SetBuildFacingResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (guihandler == nullptr) {
		result->error = &GUI_UNAVAILABLE_ERROR;
		return;
	}

	guihandler->SetBuildFacing(query->facing);
	result->success = true;
}

static void NativeSetBuildSpacing(const SetBuildSpacingQuery* query, SetBuildSpacingResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (guihandler == nullptr) {
		result->error = &GUI_UNAVAILABLE_ERROR;
		return;
	}

	guihandler->SetBuildSpacing(query->spacing);
	result->success = true;
}

static void NativeSetWindowGeometry(const SetWindowGeometryQuery* query, SetWindowGeometryResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (globalRendering == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}

	const bool ok = globalRendering->SetWindowPosHelper(
		query->displayIndex,
		query->windowPosX,
		query->windowPosY,
		query->windowSizeX,
		query->windowSizeY,
		query->options.fullScreen,
		query->options.borderless
	);

	if (!ok) {
		result->error = &INVALID_WINDOW_GEOMETRY_ERROR;
		return;
	}

	result->success = true;
}

static void NativeSetWindowMinimized(const SetWindowMinimizedQuery* /*query*/, SetWindowMinimizedResult* result)
{
	result->error = nullptr;
	result->minimized = false;

	if (globalRendering == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}

	result->minimized = globalRendering->SetWindowMinimized();
}

static void NativeSetWindowMaximized(const SetWindowMaximizedQuery* /*query*/, SetWindowMaximizedResult* result)
{
	result->error = nullptr;
	result->maximized = false;

	if (globalRendering == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}

	result->maximized = globalRendering->SetWindowMaximized();
}

static void NativeSetWMCaption(const SetWMCaptionQuery* query, SetWMCaptionResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (globalRendering == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}

	if (query->title == nullptr) {
		result->error = &INVALID_CAPTION_ERROR;
		return;
	}

	globalRendering->SetWindowTitle(query->title);
	result->success = true;
}

static void NativeSetWMIcon(const SetWMIconQuery* query, SetWMIconResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (query->iconFileName == nullptr) {
		result->error = &INVALID_ICON_NAME_ERROR;
		return;
	}

	CBitmap iconTexture;
	if (!iconTexture.Load(query->iconFileName)) {
		result->error = &ICON_LOAD_ERROR;
		return;
	}

	WindowManagerHelper::SetIcon(&iconTexture, query->forceResolution);
	result->success = true;
}

static void NativeSetVideoCapturingMode(const SetVideoCapturingModeQuery* query, SetVideoCapturingModeResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (videoCapturing == nullptr) {
		result->error = &VIDEO_CAPTURE_UNAVAILABLE_ERROR;
		return;
	}

	videoCapturing->SetAllowRecord(query->allowCaptureMode);
	result->success = true;
}

static void NativeRunDollyCamera(const RunDollyCameraQuery* query, RunDollyCameraResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (camHandler == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}

	camHandler->GetDollyController().Run(query->runtimeMs);
	result->success = true;
}

static void NativePauseDollyCamera(const PauseDollyCameraQuery* query, PauseDollyCameraResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (camHandler == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}

	camHandler->GetDollyController().Pause(query->percent);
	result->success = true;
}

static void NativeResumeDollyCamera(const ResumeDollyCameraQuery* /*query*/, ResumeDollyCameraResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (camHandler == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}

	camHandler->GetDollyController().Resume();
	result->success = true;
}

static void NativeSetDollyCameraMode(const SetDollyCameraModeQuery* query, SetDollyCameraModeResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (camHandler == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}

	camHandler->GetDollyController().SetMode(query->mode);
	result->success = true;
}

static void NativeSetDollyCameraPosition(const SetDollyCameraPositionQuery* query, SetDollyCameraPositionResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (camHandler == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}

	const float3 pos(query->position.x, query->position.y, query->position.z);
	camHandler->GetDollyController().SetPosition(pos);
	result->success = true;
}

static void NativeSetDollyCameraCurve(const SetDollyCameraCurveQuery* query, SetDollyCameraCurveResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (camHandler == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}

	std::vector<float4> controlPoints;
	controlPoints.reserve(query->controlPointsCount);
	for (uint32_t i = 0; i < query->controlPointsCount; ++i) {
		const Float4& cp = query->controlPoints[i];
		controlPoints.emplace_back(cp.x, cp.y, cp.z, cp.w);
	}

	std::vector<float> knots;
	knots.reserve(query->knotsCount);
	for (uint32_t i = 0; i < query->knotsCount; ++i) {
		knots.push_back(query->knots[i]);
	}

	camHandler->GetDollyController().SetNURBS(query->degree, controlPoints, knots);
	result->success = true;
}

static void NativeSetDollyCameraLookPosition(const SetDollyCameraLookPositionQuery* query, SetDollyCameraLookPositionResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (camHandler == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}

	const float3 pos(query->position.x, query->position.y, query->position.z);
	auto& controller = camHandler->GetDollyController();
	controller.SetLookMode(CDollyController::DOLLY_LOOKMODE_POSITION);
	controller.SetLookPosition(pos);
	result->success = true;
}

static void NativeSetDollyCameraLookUnit(const SetDollyCameraLookUnitQuery* query, SetDollyCameraLookUnitResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (camHandler == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}

	auto& controller = camHandler->GetDollyController();
	controller.SetLookMode(CDollyController::DOLLY_LOOKMODE_UNIT);
	controller.SetLookUnit(query->unitID);
	result->success = true;
}

static void NativeSetDollyCameraLookCurve(const SetDollyCameraLookCurveQuery* query, SetDollyCameraLookCurveResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (camHandler == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}

	std::vector<float4> controlPoints;
	controlPoints.reserve(query->controlPointsCount);
	for (uint32_t i = 0; i < query->controlPointsCount; ++i) {
		const Float4& cp = query->controlPoints[i];
		controlPoints.emplace_back(cp.x, cp.y, cp.z, cp.w);
	}

	std::vector<float> knots;
	knots.reserve(query->knotsCount);
	for (uint32_t i = 0; i < query->knotsCount; ++i) {
		knots.push_back(query->knots[i]);
	}

	auto& controller = camHandler->GetDollyController();
	controller.SetLookMode(CDollyController::DOLLY_LOOKMODE_CURVE);
	controller.SetLookCurve(query->degree, controlPoints, knots);
	result->success = true;
}

static void NativeSetDollyCameraRelativeMode(const SetDollyCameraRelativeModeQuery* query, SetDollyCameraRelativeModeResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (camHandler == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}

	camHandler->GetDollyController().SetRelativeMode(query->mode);
	result->success = true;
}

static void NativeSetVideoCapturingTimeOffset(const SetVideoCapturingTimeOffsetQuery* query, SetVideoCapturingTimeOffsetResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (videoCapturing == nullptr) {
		result->error = &VIDEO_CAPTURE_UNAVAILABLE_ERROR;
		return;
	}

	videoCapturing->SetTimeOffset(query->timeOffset);
	result->success = true;
}

static void NativeSetCameraOffset(const SetCameraOffsetQuery* query, SetCameraOffsetResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (camera == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}

	camera->posOffset = float3(query->posOffset.x, query->posOffset.y, query->posOffset.z);
	camera->tiltOffset = float3(query->tiltOffset.x, query->tiltOffset.y, query->tiltOffset.z);
	result->success = true;
}

static void NativeSetDrawGround(const SetDrawGroundQuery* query, SetDrawGroundResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (globalRendering == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}

	globalRendering->drawGround = query->drawGround;
	result->success = true;
}

static void NativeSetDrawSky(const SetDrawSkyQuery* query, SetDrawSkyResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (globalRendering == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}

	globalRendering->drawSky = query->drawSky;
	result->success = true;
}

static void NativeSetDrawWater(const SetDrawWaterQuery* query, SetDrawWaterResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (globalRendering == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}

	globalRendering->drawWater = query->drawWater;
	result->success = true;
}

static void NativeSetDrawGroundDeferred(const SetDrawGroundDeferredQuery* query, SetDrawGroundDeferredResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (readMap == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}

	CBaseGroundDrawer* gd = readMap->GetGroundDrawer();
	if (gd == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}

	gd->SetDrawDeferredPass(query->drawDeferred);
	gd->SetDrawForwardPass(query->drawForward);

	result->success = true;
	result->deferred = gd->DrawDeferred();
	result->forward = gd->DrawForward();
}

static void NativeSetDrawModelsDeferred(const SetDrawModelsDeferredQuery* query, SetDrawModelsDeferredResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (unitDrawer == nullptr || featureDrawer == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}

	unitDrawer->SetDrawDeferredPass(query->drawUnitsDeferred);
	unitDrawer->SetDrawForwardPass(query->drawUnitsForward);

	featureDrawer->SetDrawDeferredPass(query->drawFeaturesDeferred);
	featureDrawer->SetDrawForwardPass(query->drawFeaturesForward);

	result->success = true;
	result->unitsDeferred = unitDrawer->DrawDeferred();
	result->featuresDeferred = featureDrawer->DrawDeferred();
	result->unitsForward = unitDrawer->DrawForward();
	result->featuresForward = featureDrawer->DrawForward();
}

static void NativeStubNotAvailable(const Error** error, bool* success)
{
	if (error != nullptr) {
		*error = &NOT_AVAILABLE_ERROR;
	}
	if (success != nullptr) {
		*success = false;
	}
}

static void NativeSetAtmosphere(const SetAtmosphereQuery* query, SetAtmosphereResult* result)
{
	result->error = nullptr;
	result->success = false;

	const auto& sky = ISky::GetSky();
	if (sky == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}

	const AtmosphereParams& p = query->params;
	if (p.hasFogColor)   sky->fogColor   = float4(p.fogColor[0], p.fogColor[1], p.fogColor[2], p.fogColor[3]);
	if (p.hasSkyColor)   sky->skyColor   = float4(p.skyColor[0], p.skyColor[1], p.skyColor[2], p.skyColor[3]);
	if (p.hasSunColor)   sky->sunColor   = float4(p.sunColor[0], p.sunColor[1], p.sunColor[2], p.sunColor[3]);
	if (p.hasCloudColor) sky->cloudColor = float4(p.cloudColor[0], p.cloudColor[1], p.cloudColor[2], p.cloudColor[3]);
	if (p.hasSkyAxisAngle) sky->SetSkyAxisAngle(float4(p.skyAxisAngle[0], p.skyAxisAngle[1], p.skyAxisAngle[2], p.skyAxisAngle[3]));
	if (p.hasFogStart)   sky->fogStart = p.fogStart;
	if (p.hasFogEnd)     sky->fogEnd   = p.fogEnd;

	sky->SetUpdated();
	result->success = true;
}

static void NativeSetSunDirection(const SetSunDirectionQuery* query, SetSunDirectionResult* result)
{
	result->error = nullptr;
	result->success = false;

	auto& skyPtr = ISky::GetSky();
	ISky* sky = skyPtr.get();
	if (sky == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}

	float3 dir(query->dir.x, query->dir.y, query->dir.z);
	dir.SafeNormalize();
	sky->GetLight()->SetLightDir(float4(dir, query->intensity));
	sunLighting->SetUpdated();
	// Notify listeners the sun changed so the ground/model shaders re-upload the
	// new light direction (SMFRenderState::UpdateShaderSkyUniforms reads it from
	// the sky light). Without this only the shading texture refreshes, leaving the
	// lit terrain stale until an unrelated SetSunLighting fires the same event.
	eventHandler.SunChanged();
	result->success = true;
}

static void NativeSetSunLighting(const SetSunLightingQuery* query, SetSunLightingResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (sunLighting == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}

	CSunLighting sl = *sunLighting;
	const SunLightingParams& p = query->params;
	if (p.hasGroundAmbientColor)  sl.groundAmbientColor  = float4(p.groundAmbientColor[0], p.groundAmbientColor[1], p.groundAmbientColor[2], p.groundAmbientColor[3]);
	if (p.hasGroundDiffuseColor)  sl.groundDiffuseColor  = float4(p.groundDiffuseColor[0], p.groundDiffuseColor[1], p.groundDiffuseColor[2], p.groundDiffuseColor[3]);
	if (p.hasGroundSpecularColor) sl.groundSpecularColor = float4(p.groundSpecularColor[0], p.groundSpecularColor[1], p.groundSpecularColor[2], p.groundSpecularColor[3]);
	if (p.hasModelAmbientColor)   sl.modelAmbientColor   = float4(p.modelAmbientColor[0], p.modelAmbientColor[1], p.modelAmbientColor[2], p.modelAmbientColor[3]);
	if (p.hasModelDiffuseColor)   sl.modelDiffuseColor   = float4(p.modelDiffuseColor[0], p.modelDiffuseColor[1], p.modelDiffuseColor[2], p.modelDiffuseColor[3]);
	if (p.hasModelSpecularColor)  sl.modelSpecularColor  = float4(p.modelSpecularColor[0], p.modelSpecularColor[1], p.modelSpecularColor[2], p.modelSpecularColor[3]);
	if (p.hasSpecularExponent)    sl.specularExponent    = p.specularExponent;
	if (p.hasGroundShadowDensity) sl.groundShadowDensity = p.groundShadowDensity;
	if (p.hasModelShadowDensity)  sl.modelShadowDensity  = p.modelShadowDensity;

	*sunLighting = sl;
	sunLighting->SetUpdated();
	result->success = true;
}

static void NativeSetWaterTexture(const SetWaterTextureQuery* query, SetWaterTextureResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (waterRendering == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}
	if (query->texType == nullptr || query->texName == nullptr) {
		result->error = &INVALID_TEXTURE_ERROR;
		return;
	}

	// Mirror the LUA_TSTRING keys of Spring.SetWaterParams: just store the path.
	// The renderer picks it up on the next water-mode (re)select, which SBC
	// triggers via SendCommands("water <mode>") right after applying params.
	const std::string texType = query->texType;
	const std::string texName = query->texName;
	if (texType == "texture") {
		waterRendering->texture = texName;
	} else if (texType == "foamTexture") {
		waterRendering->foamTexture = texName;
	} else if (texType == "normalTexture") {
		waterRendering->normalTexture = texName;
	} else {
		result->error = &INVALID_TEXTURE_ERROR;
		return;
	}
	result->success = true;
}

static void NativeGetWaterTexture(const GetWaterTextureQuery* query, GetWaterTextureResult* result)
{
	result->error = nullptr;
	result->texName = "";

	if (waterRendering == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}
	if (query->texType == nullptr) {
		result->error = &INVALID_TEXTURE_ERROR;
		return;
	}

	// Returns a pointer into the persistent waterRendering string; the caller
	// copies it before any further mutation.
	const std::string texType = query->texType;
	if (texType == "texture") {
		result->texName = waterRendering->texture.c_str();
	} else if (texType == "foamTexture") {
		result->texName = waterRendering->foamTexture.c_str();
	} else if (texType == "normalTexture") {
		result->texName = waterRendering->normalTexture.c_str();
	} else {
		result->error = &INVALID_TEXTURE_ERROR;
	}
}

static void NativeSetWaterParams(const SetWaterParamsQuery* query, SetWaterParamsResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (waterRendering == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}

	const WaterParams& p = query->params;
	if (p.hasAbsorb)        waterRendering->absorb        = float3(p.absorb[0], p.absorb[1], p.absorb[2]);
	if (p.hasBaseColor)     waterRendering->baseColor     = float3(p.baseColor[0], p.baseColor[1], p.baseColor[2]);
	if (p.hasMinColor)      waterRendering->minColor      = float3(p.minColor[0], p.minColor[1], p.minColor[2]);
	if (p.hasSurfaceColor)  waterRendering->surfaceColor  = float3(p.surfaceColor[0], p.surfaceColor[1], p.surfaceColor[2]);
	if (p.hasDiffuseColor)  waterRendering->diffuseColor  = float3(p.diffuseColor[0], p.diffuseColor[1], p.diffuseColor[2]);
	if (p.hasSpecularColor) waterRendering->specularColor = float3(p.specularColor[0], p.specularColor[1], p.specularColor[2]);
	if (p.hasPlaneColor) {
		waterRendering->planeColor.x = p.planeColor[0];
		waterRendering->planeColor.y = p.planeColor[1];
		waterRendering->planeColor.z = p.planeColor[2];
	}

	if (p.hasRepeatX)        waterRendering->repeatX        = p.repeatX;
	if (p.hasRepeatY)        waterRendering->repeatY        = p.repeatY;
	if (p.hasSurfaceAlpha)   waterRendering->surfaceAlpha   = p.surfaceAlpha;
	if (p.hasAmbientFactor)  waterRendering->ambientFactor  = p.ambientFactor;
	if (p.hasDiffuseFactor)  waterRendering->diffuseFactor  = p.diffuseFactor;
	if (p.hasSpecularFactor) waterRendering->specularFactor = p.specularFactor;
	if (p.hasSpecularPower)  waterRendering->specularPower  = p.specularPower;
	if (p.hasFresnelMin)     waterRendering->fresnelMin     = p.fresnelMin;
	if (p.hasFresnelMax)     waterRendering->fresnelMax     = p.fresnelMax;
	if (p.hasFresnelPower)   waterRendering->fresnelPower   = p.fresnelPower;
	if (p.hasReflectionDistortion) waterRendering->reflDistortion = p.reflectionDistortion;
	if (p.hasBlurBase)       waterRendering->blurBase       = p.blurBase;
	if (p.hasBlurExponent)   waterRendering->blurExponent   = p.blurExponent;
	if (p.hasPerlinStartFreq)  waterRendering->perlinStartFreq  = p.perlinStartFreq;
	if (p.hasPerlinLacunarity) waterRendering->perlinLacunarity = p.perlinLacunarity;
	if (p.hasPerlinAmplitude)  waterRendering->perlinAmplitude  = p.perlinAmplitude;
	if (p.hasWindSpeed)        waterRendering->windSpeed        = p.windSpeed;
	if (p.hasWaveOffsetFactor) waterRendering->waveOffsetFactor = p.waveOffsetFactor;
	if (p.hasWaveLength)       waterRendering->waveLength       = p.waveLength;
	if (p.hasWaveFoamDistortion) waterRendering->waveFoamDistortion = p.waveFoamDistortion;
	if (p.hasWaveFoamIntensity)  waterRendering->waveFoamIntensity  = p.waveFoamIntensity;
	if (p.hasCausticsResolution) waterRendering->causticsResolution = p.causticsResolution;
	if (p.hasCausticsStrength)   waterRendering->causticsStrength   = p.causticsStrength;
	if (p.hasNumTiles)           waterRendering->numTiles           = (unsigned char)p.numTiles;

	if (p.hasShoreWaves)     waterRendering->shoreWaves     = p.shoreWaves;
	if (p.hasForceRendering) waterRendering->forceRendering = p.forceRendering;
	if (p.hasHasWaterPlane)  waterRendering->hasWaterPlane  = p.hasWaterPlane;

	const int waterID = static_cast<int>(IWater::GetWater()->GetID());
	IWater::KillWater();
	IWater::SetWater(waterID);
	waterRendering->SetUpdated();
	result->success = true;
}

static void NativeSetMapShader(const SetMapShaderQuery* query, SetMapShaderResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (readMap == nullptr) {
		result->error = &MAP_UNAVAILABLE_ERROR;
		return;
	}

	CBaseGroundDrawer* groundDrawer = readMap->GetGroundDrawer();
	if (groundDrawer == nullptr) {
		result->error = &MAP_UNAVAILABLE_ERROR;
		return;
	}

	// Without Lua shader handles just treat IDs as raw program names.
	LuaMapShaderData luaMapShaderData;
	luaMapShaderData.shaderIDs[0] = query->standardShaderID;
	luaMapShaderData.shaderIDs[1] = query->deferredShaderID;
	groundDrawer->SetLuaShader(&luaMapShaderData);
	result->success = true;
}

static bool NativeMapTextureTypeFromName(const char* texType, unsigned int* type)
{
	if (texType == nullptr || type == nullptr)
		return false;

	switch (hashString(texType)) {
		case hashString("$grass"):              *type = MAP_BASE_GRASS_TEX; break;
		case hashString("$detail"):             *type = MAP_BASE_DETAIL_TEX; break;
		case hashString("$minimap"):            *type = MAP_BASE_MINIMAP_TEX; break;
		case hashString("$shading"):            *type = MAP_BASE_SHADING_TEX; break;
		case hashString("$normals"):            *type = MAP_BASE_NORMALS_TEX; break;
		case hashString("$ssmf_normals"):       *type = MAP_SSMF_NORMALS_TEX; break;
		case hashString("$ssmf_specular"):      *type = MAP_SSMF_SPECULAR_TEX; break;
		case hashString("$ssmf_splat_distr"):   *type = MAP_SSMF_SPLAT_DISTRIB_TEX; break;
		case hashString("$ssmf_splat_detail"):  *type = MAP_SSMF_SPLAT_DETAIL_TEX; break;
		case hashString("$ssmf_splat_normals"): *type = MAP_SSMF_SPLAT_NORMAL_TEX; break;
		case hashString("$ssmf_sky_refl"):      *type = MAP_SSMF_SKY_REFLECTION_TEX; break;
		case hashString("$ssmf_emission"):      *type = MAP_SSMF_LIGHT_EMISSION_TEX; break;
		case hashString("$ssmf_parallax"):      *type = MAP_SSMF_PARALLAX_HEIGHT_TEX; break;
		default: return false;
	}

	return true;
}

static void NativeSetMapShadingTexture(const SetMapShadingTextureQuery* query, SetMapShadingTextureResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (readMap == nullptr) {
		result->error = &MAP_UNAVAILABLE_ERROR;
		return;
	}

	MapTextureData texData;
	if (!NativeMapTextureTypeFromName(query->texType, &texData.type)) {
		result->error = &INVALID_TEXTURE_ERROR;
		return;
	}
	texData.num = std::max(query->num, 0);

	const char* texName = query->texName;
	if (texName != nullptr && texName[0] != '\0') {
		uint32_t nativeID = 0;
		int32_t nativeXSize = 0;
		int32_t nativeYSize = 0;
		uint32_t nativeTarget = 0;

		if (GetNativeGfxTextureInfo(texName, &nativeID, &nativeXSize, &nativeYSize, &nativeTarget)) {
			if (nativeTarget != GL_TEXTURE_2D) {
				result->error = &INVALID_TEXTURE_ERROR;
				return;
			}
			texData.id = nativeID;
			texData.size = int2(nativeXSize, nativeYSize);
		} else {
			const CNamedTextures::TexInfo* namedTexture = CNamedTextures::GetInfo(texName);
			if (namedTexture == nullptr) {
				result->error = &INVALID_TEXTURE_ERROR;
				return;
			}
			texData.id = namedTexture->id;
			texData.size = int2(namedTexture->xsize, namedTexture->ysize);
		}
	}

	result->success = readMap->SetLuaTexture(texData);
}

static void NativeSetSkyBoxTexture(const SetSkyBoxTextureQuery* query, SetSkyBoxTextureResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (query == nullptr || query->texName == nullptr) {
		result->error = &INVALID_TEXTURE_ERROR;
		return;
	}

	MapTextureData texData;
	const char* texName = query->texName;
	if (texName[0] != '\0') {
		uint32_t nativeID = 0;
		int32_t nativeXSize = 0;
		int32_t nativeYSize = 0;
		uint32_t nativeTarget = 0;

		if (GetNativeGfxTextureInfo(texName, &nativeID, &nativeXSize, &nativeYSize, &nativeTarget)) {
			texData.id = nativeID;
			texData.size = int2(nativeXSize, nativeYSize);
		} else if (const CNamedTextures::TexInfo* namedTexture = CNamedTextures::GetInfo(texName)) {
			texData.id = namedTexture->id;
			texData.size = int2(namedTexture->xsize, namedTexture->ysize);
		}
	}

	ISky::SetSkyLuaTexture(texData);
	result->success = true;
}

static void NativeSetMapRenderingParams(const SetMapRenderingParamsQuery* query, SetMapRenderingParamsResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (readMap == nullptr || mapRendering == nullptr) {
		result->error = &MAP_UNAVAILABLE_ERROR;
		return;
	}

	const MapRenderingParams& p = query->params;
	if (p.hasSplatTexScales) mapRendering->splatTexScales = float4(p.splatTexScales[0], p.splatTexScales[1], p.splatTexScales[2], p.splatTexScales[3]);
	if (p.hasSplatTexMults)  mapRendering->splatTexMults  = float4(p.splatTexMults[0], p.splatTexMults[1], p.splatTexMults[2], p.splatTexMults[3]);
	if (p.hasVoidWater)      mapRendering->voidWater      = p.voidWater;
	if (p.hasVoidGround)     mapRendering->voidGround     = p.voidGround;
	if (p.hasSplatDetailNormalDiffuseAlpha) mapRendering->splatDetailNormalDiffuseAlpha = p.splatDetailNormalDiffuseAlpha;

	CBaseGroundDrawer* groundDrawer = readMap->GetGroundDrawer();
	if (groundDrawer != nullptr)
		groundDrawer->UpdateRenderState();
	result->success = true;
}

static void NativeSetLosViewColors(const SetLosViewColorsQuery* query, SetLosViewColorsResult* result)
{
	result->error = nullptr;
	result->success = false;
	if (readMap == nullptr || readMap->GetGroundDrawer() == nullptr)
		return;

	const int scale = CBaseGroundDrawer::losColorScale;
	CBaseGroundDrawer* gd = readMap->GetGroundDrawer();
	gd->alwaysColor[0] = static_cast<int>(scale * query->always.r);
	gd->alwaysColor[1] = static_cast<int>(scale * query->always.g);
	gd->alwaysColor[2] = static_cast<int>(scale * query->always.b);
	gd->losColor[0] = static_cast<int>(scale * query->los.r);
	gd->losColor[1] = static_cast<int>(scale * query->los.g);
	gd->losColor[2] = static_cast<int>(scale * query->los.b);
	gd->radarColor[0] = static_cast<int>(scale * query->radar.r);
	gd->radarColor[1] = static_cast<int>(scale * query->radar.g);
	gd->radarColor[2] = static_cast<int>(scale * query->radar.b);
	gd->jamColor[0] = static_cast<int>(scale * query->jam.r);
	gd->jamColor[1] = static_cast<int>(scale * query->jam.g);
	gd->jamColor[2] = static_cast<int>(scale * query->jam.b);
	gd->radarColor2[0] = static_cast<int>(scale * query->radar2.r);
	gd->radarColor2[1] = static_cast<int>(scale * query->radar2.g);
	gd->radarColor2[2] = static_cast<int>(scale * query->radar2.b);
	result->success = true;
}

static void NativeSetDrawSelectionInfo(const SetDrawSelectionInfoQuery* query, SetDrawSelectionInfoResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (guihandler == nullptr) {
		result->error = &GUI_UNAVAILABLE_ERROR;
		return;
	}

	guihandler->SetDrawSelectionInfo(query->draw);
	result->success = true;
}

static void NativeSetShockFrontFactors(const SetShockFrontFactorsQuery* query, SetShockFrontFactorsResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (luaUI == nullptr) {
		result->error = &LUAUI_UNAVAILABLE_ERROR;
		return;
	}

	luaUI->SetShockFrontFactors(
		query->options.hasMinArea, query->options.minArea,
		query->options.hasMinPower, query->options.minPower,
		query->options.hasDistAdj, query->options.distAdj
	);
	result->success = true;
}

static void NativeSetCustomCommandDrawData(const SetCustomCommandDrawDataQuery* query, SetCustomCommandDrawDataResult* result)
{
	result->error = nullptr;
	result->success = false;

	int iconID = 0;
	if (query->cmdReference.id >= 0) {
		iconID = query->cmdReference.id;
	} else if (query->cmdReference.name != nullptr && query->cmdReference.name[0] != '\0') {
		iconID = query->cmdID;
		cursorIcons.SetCustomType(query->cmdID, query->cmdReference.name);
	} else {
		cursorIcons.SetCustomType(query->cmdID, "");
		cmdColors.ClearCustomCmdData(query->cmdID);
		result->success = true;
		return;
	}

	const float color[4] = { query->color.x, query->color.y, query->color.z, query->color.w };
	cmdColors.SetCustomCmdData(query->cmdID, iconID, color, query->showArea);
	result->success = true;
}

static void NativeSetLastMessagePosition(const SetLastMessagePositionQuery* query, SetLastMessagePositionResult* result)
{
	result->error = nullptr;
	result->success = false;

	const float3 pos(query->pos.x, query->pos.y, query->pos.z);
	eventHandler.LastMessagePosition(pos);
	result->success = true;
}

static void NativeLoadCmdColorsConfig(const LoadCmdColorsConfigQuery* query, LoadCmdColorsConfigResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (query == nullptr || query->filename == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	// Spring.LoadCmdColorsConfig receives the configuration text itself, not a
	// VFS filename.  Keep the native path on the same contract as Lua.
	result->success = cmdColors.LoadConfigFromString(query->filename);
}

static void NativeLoadCtrlPanelConfig(const LoadCtrlPanelConfigQuery* query, LoadCtrlPanelConfigResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (guihandler == nullptr) {
		result->error = &GUI_UNAVAILABLE_ERROR;
		return;
	}

	if (query == nullptr || query->filename == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	// Spring.LoadCtrlPanelConfig likewise passes an inline configuration string
	// to ReloadConfigFromString; it does not resolve a filename.
	result->success = guihandler->ReloadConfigFromString(query->filename);
}

static void NativeLoadModelTextures(const LoadModelTexturesQuery* query, LoadModelTexturesResult* result)
{
	result->error = nullptr;
	result->success = false;

	const std::string modelName = (query != nullptr && query->modelName != nullptr) ? query->modelName : "";
	if (modelName.empty()) {
		return;
	}

	for (S3DModel& model : modelLoader.GetModelsVec()) {
		if (model.name != modelName)
			continue;

		if (model.type == MODELTYPE_3DO) {
			return;
		}

		textureHandlerS3O.LoadTexture(&model);
		result->success = true;
		return;
	}
}

static void NativeForceLayoutUpdate(const ForceLayoutUpdateQuery* query, ForceLayoutUpdateResult* result)
{
	(void)query;
	result->error = nullptr;
	result->success = false;

	if (guihandler == nullptr) {
		result->error = &GUI_UNAVAILABLE_ERROR;
		return;
	}

	guihandler->ForceLayoutUpdate();
	result->success = true;
}

static void NativeForceTesselationUpdate(const ForceTesselationUpdateQuery* query, ForceTesselationUpdateResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (readMap == nullptr) {
		result->error = &MAP_UNAVAILABLE_ERROR;
		return;
	}

	CSMFGroundDrawer* smfDrawer = dynamic_cast<CSMFGroundDrawer*>(readMap->GetGroundDrawer());
	if (smfDrawer == nullptr) {
		result->error = &MAP_UNAVAILABLE_ERROR;
		return;
	}

	CRoamMeshDrawer* roamMeshDrawer = dynamic_cast<CRoamMeshDrawer*>(smfDrawer->GetMeshDrawer());
	if (roamMeshDrawer == nullptr) {
		result->error = &MAP_UNAVAILABLE_ERROR;
		return;
	}

	CRoamMeshDrawer::ForceNextTesselation(query->normal, query->shadow);
	result->success = true;
}

static void NativeSetAutoShowMetal(const SetAutoShowMetalQuery* query, SetAutoShowMetalResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (guihandler == nullptr) {
		result->error = &GUI_UNAVAILABLE_ERROR;
		return;
	}

	guihandler->autoShowMetal = query->enable;
	result->success = true;
}

static void NativeSetUnitIconDraw(const SetUnitIconDrawQuery* query, SetUnitIconDrawResult* result)
{
	result->error = nullptr;
	result->success = false;

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->drawIcon = query->drawIcon;
	result->success = true;
}

static void NativeSetUnitIcon(const SetUnitIconQuery* query, SetUnitIconResult* result)
{
	result->error = nullptr;
	result->success = false;

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (query->iconName == nullptr) {
		unit->customIconIndex = icon::INVALID_ICON_INDEX;
	} else {
		const auto iconIdx = icon::iconHandler.GetIconIdx(query->iconName);
		if (iconIdx == icon::INVALID_ICON_INDEX) {
			result->error = &INVALID_UNIT_ICON_ERROR;
			return;
		}

		unit->customIconIndex = iconIdx;
	}

	if (unitDrawer == nullptr) {
		result->error = &RENDERING_UNAVAILABLE_ERROR;
		return;
	}

	unitDrawer->UpdateCurrentUnitIcon(unit);
	result->success = true;
}

static void NativeSetUnitDefIcon(const SetUnitDefIconQuery* query, SetUnitDefIconResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (query->iconName == nullptr) {
		result->error = &INVALID_UNIT_ICON_ERROR;
		return;
	}

	const UnitDef* ud = unitDefHandler->GetUnitDefByID(query->unitDefID);
	if (ud == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const auto found = icon::iconHandler.FindIconIdx(query->iconName);
	if (!found.first) {
		result->error = &INVALID_UNIT_ICON_ERROR;
		return;
	}

	ud->iconName = query->iconName;

	if (ud->decoyDef != nullptr)
		ud->decoyDef->iconName = ud->iconName;

	const auto& decoyMap = unitDefHandler->GetDecoyDefIDs();
	const auto decoyMapIt = decoyMap.find((ud->decoyDef != nullptr) ? ud->decoyDef->id : ud->id);
	if (decoyMapIt != decoyMap.end()) {
		for (const int decoyDefID : decoyMapIt->second) {
			const UnitDef* decoyDef = unitDefHandler->GetUnitDefByID(decoyDefID);
			if (decoyDef != nullptr) {
				decoyDef->iconName = ud->iconName;
			}
		}
	}

	unitDrawer->UpdateUnitIconsByUnitDef(ud);
	result->success = true;
}

static void NativeSetUnitDefImage(const SetUnitDefImageQuery* query, SetUnitDefImageResult* result)
{
	result->error = nullptr;
	result->success = false;

	const UnitDef* ud = unitDefHandler->GetUnitDefByID(query->unitDefID);
	if (ud == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (query->image == nullptr || query->image[0] == '\0') {
		CUnitDrawer::SetUnitDefImage(ud, ud->buildPicName);
		result->success = true;
		return;
	}

	const std::string image = query->image;

	if (!image.empty() && image[0] == '!') {
		const auto* tex = CNamedTextures::GetInfo(image);
		if (tex == nullptr) {
			result->error = &INVALID_UNIT_IMAGE_ERROR;
			return;
		}
		CUnitDrawer::SetUnitDefImage(ud, tex->id, tex->xsize, tex->ysize);
		result->success = true;
		return;
	}

	CUnitDrawer::SetUnitDefImage(ud, image);
	result->success = true;
}

static void NativeSetCustomPaletteColor(const SetCustomPaletteColorQuery* query, SetCustomPaletteColorResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (query->index < 0 || query->index >= MAX_CUSTOM_COLORS) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	customColorPalette.SetColor(static_cast<uint16_t>(query->index), query->r, query->g, query->b);
	result->success = true;
}

static void NativeSetUnitPaletteIndex(const SetUnitPaletteIndexQuery* query, SetUnitPaletteIndexResult* result)
{
	result->error = nullptr;
	result->success = false;

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (query->customIndex < 0) {
		unit->paletteIndex = static_cast<uint16_t>(unit->team);
		result->success = true;
		return;
	}

	if (query->customIndex >= MAX_CUSTOM_COLORS) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	unit->paletteIndex = CCustomColorPalette::EncodePaletteIndex(static_cast<uint16_t>(query->customIndex));
	result->success = true;
}

static void NativeSetFeaturePaletteIndex(const SetFeaturePaletteIndexQuery* query, SetFeaturePaletteIndexResult* result)
{
	result->error = nullptr;
	result->success = false;

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	if (query->customIndex < 0) {
		feature->paletteIndex = static_cast<uint16_t>(feature->team);
		result->success = true;
		return;
	}

	if (query->customIndex >= MAX_CUSTOM_COLORS) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	feature->paletteIndex = CCustomColorPalette::EncodePaletteIndex(static_cast<uint16_t>(query->customIndex));
	result->success = true;
}

static void NativeSetEngineBuildSquareRendering(const SetEngineBuildSquareRenderingQuery* query, SetEngineBuildSquareRenderingResult* result)
{
	result->error = nullptr;
	result->success = true;
	CUnitDrawer::EngineBuildSquareRendering() = query->enabled;
}

static void NativeSetFeatureNoDraw(const SetFeatureNoDrawQuery* query, SetFeatureNoDrawResult* result)
{
	result->error = nullptr;
	result->success = false;

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	feature->noDraw = query->noDraw;
	result->success = true;
}

static void NativeSetFeatureEngineDrawMask(const SetFeatureEngineDrawMaskQuery* query, SetFeatureEngineDrawMaskResult* result)
{
	result->error = nullptr;
	result->success = false;

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	feature->engineDrawMask = static_cast<uint8_t>(query->mask);
	result->success = true;
}

static void NativeSetFeatureAlwaysUpdateMatrix(const SetFeatureAlwaysUpdateMatrixQuery* query, SetFeatureAlwaysUpdateMatrixResult* result)
{
	result->error = nullptr;
	result->success = false;

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	feature->alwaysUpdateMat = query->enable;
	result->success = true;
}

static void NativeSetFeatureFade(const SetFeatureFadeQuery* query, SetFeatureFadeResult* result)
{
	result->error = nullptr;
	result->success = false;

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	feature->alphaFade = query->allow;
	result->success = true;
}

static void NativeSetNanoProjectileParams(const SetNanoProjectileParamsQuery* query, SetNanoProjectileParamsResult* result)
{
	result->error = nullptr;
	result->success = true;

	CNanoProjectile::rotVal0    = query->r     * math::DEG_TO_RAD;
	CNanoProjectile::rotVel0    = query->v     * (math::DEG_TO_RAD / GAME_SPEED);
	CNanoProjectile::rotAcc0    = query->a     * (math::DEG_TO_RAD / (GAME_SPEED * GAME_SPEED));
	CNanoProjectile::rotValRng0 = query->randR * math::DEG_TO_RAD;
	CNanoProjectile::rotVelRng0 = query->randV * (math::DEG_TO_RAD / GAME_SPEED);
	CNanoProjectile::rotAccRng0 = query->randA * (math::DEG_TO_RAD / (GAME_SPEED * GAME_SPEED));
}

static void NativePreloadFeatureDefModel(const PreloadFeatureDefModelQuery* query, PreloadFeatureDefModelResult* result)
{
	result->error = nullptr;
	result->success = false;

	const FeatureDef* fd = featureDefHandler->GetFeatureDefByID(query->defID);
	if (fd == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	fd->PreloadModel();
	result->success = true;
}

static void NativePreloadUnitDefModel(const PreloadUnitDefModelQuery* query, PreloadUnitDefModelResult* result)
{
	result->error = nullptr;
	result->success = false;

	const UnitDef* ud = unitDefHandler->GetUnitDefByID(query->defID);
	if (ud == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	ud->PreloadModel();
	result->success = true;
}

static void NativeSelectUnitMap(const SelectUnitMapQuery* query, SelectUnitMapResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (!query->append) {
		selectedUnitsHandler.ClearSelected();
	}

	for (uint32_t i = 0; i < query->count; ++i) {
		CUnit* unit = unitHandler.GetUnit(query->unitIDs[i]);
		if (unit == nullptr || unit->noSelect)
			continue;

		selectedUnitsHandler.AddUnit(unit);
	}

	result->success = true;
}

static void NativeDeselectUnitMap(const DeselectUnitMapQuery* query, DeselectUnitMapResult* result)
{
	result->error = nullptr;
	result->success = false;

	for (uint32_t i = 0; i < query->count; ++i) {
		CUnit* unit = unitHandler.GetUnit(query->unitIDs[i]);
		if (unit == nullptr)
			continue;

		selectedUnitsHandler.RemoveUnit(unit);
	}

	result->success = true;
}

static void NativeDrawUnitCommands(const DrawUnitCommandsQuery* query, DrawUnitCommandsResult* result)
{
	result->error = nullptr;
	result->success = false;
	(void)query->tableOrArray;

	if (query->unitIDs == nullptr && query->count > 0) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	for (uint32_t i = 0; i < query->count; ++i) {
		const CUnit* unit = unitHandler.GetUnit(query->unitIDs[i]);
		if (unit != nullptr && unit->allyteam == gu->myAllyTeam)
			commandDrawer->AddLuaQueuedUnit(unit, query->queueDrawDepth);
	}

	result->success = true;
}

} // namespace

const UnsyncedCtrlApi UNSYNCED_CTRL_API = {
	.SetUnitNoDraw = NativeSetUnitNoDraw,
	.SetUnitEngineDrawMask = NativeSetUnitEngineDrawMask,
	.SetUnitAlwaysUpdateMatrix = NativeSetUnitAlwaysUpdateMatrix,
	.SetUnitNoMinimap = NativeSetUnitNoMinimap,
	.SetUnitNoGroup = NativeSetUnitNoGroup,
	.SetUnitNoSelect = NativeSetUnitNoSelect,
	.SetUnitLeaveTracks = NativeSetUnitLeaveTracks,
	.SetMiniMapRotation = NativeSetMiniMapRotation,
	.SetClipboard = NativeSetClipboard,
	.SetMouseCursor = NativeSetMouseCursor,
	.AssignMouseCursor = NativeAssignMouseCursor,
	.ReplaceMouseCursor = NativeReplaceMouseCursor,
	.WarpMouse = NativeWarpMouse,
	.SetActiveCommand = NativeSetActiveCommand,
	.SDLStartTextInput = NativeSDLStartTextInput,
	.SDLStopTextInput = NativeSDLStopTextInput,
	.SDLSetTextInputRect = NativeSDLSetTextInputRect,
	.SetBoxSelectionByEngine = NativeSetBoxSelectionByEngine,
	.SetBuildFacing = NativeSetBuildFacing,
	.SetBuildSpacing = NativeSetBuildSpacing,
	.SetWindowGeometry = NativeSetWindowGeometry,
	.SetWindowMinimized = NativeSetWindowMinimized,
	.SetWindowMaximized = NativeSetWindowMaximized,
	.SetWMCaption = NativeSetWMCaption,
	.SetWMIcon = NativeSetWMIcon,
	.SetVideoCapturingMode = NativeSetVideoCapturingMode,
	.RunDollyCamera = NativeRunDollyCamera,
	.PauseDollyCamera = NativePauseDollyCamera,
	.ResumeDollyCamera = NativeResumeDollyCamera,
	.SetDollyCameraMode = NativeSetDollyCameraMode,
	.SetDollyCameraPosition = NativeSetDollyCameraPosition,
	.SetDollyCameraCurve = NativeSetDollyCameraCurve,
	.SetDollyCameraLookPosition = NativeSetDollyCameraLookPosition,
	.SetDollyCameraLookUnit = NativeSetDollyCameraLookUnit,
	.SetDollyCameraLookCurve = NativeSetDollyCameraLookCurve,
	.SetDollyCameraRelativeMode = NativeSetDollyCameraRelativeMode,
	.SetVideoCapturingTimeOffset = NativeSetVideoCapturingTimeOffset,
	.SetCameraOffset = NativeSetCameraOffset,
	.SetDrawGround = NativeSetDrawGround,
	.SetDrawSky = NativeSetDrawSky,
	.SetDrawWater = NativeSetDrawWater,
	.SetDrawGroundDeferred = NativeSetDrawGroundDeferred,
	.SetDrawModelsDeferred = NativeSetDrawModelsDeferred,
	.SetAtmosphere = NativeSetAtmosphere,
	.SetSunDirection = NativeSetSunDirection,
	.SetSunLighting = NativeSetSunLighting,
	.SetWaterParams = NativeSetWaterParams,
	.SetMapShader = NativeSetMapShader,
	.SetMapShadingTexture = NativeSetMapShadingTexture,
	.SetSkyBoxTexture = NativeSetSkyBoxTexture,
	.SetMapRenderingParams = NativeSetMapRenderingParams,
	.SetLosViewColors = NativeSetLosViewColors,
	.SetDrawSelectionInfo = NativeSetDrawSelectionInfo,
	.SetShockFrontFactors = NativeSetShockFrontFactors,
	.SetCustomCommandDrawData = NativeSetCustomCommandDrawData,
	.SetLastMessagePosition = NativeSetLastMessagePosition,
	.LoadCmdColorsConfig = NativeLoadCmdColorsConfig,
	.LoadCtrlPanelConfig = NativeLoadCtrlPanelConfig,
	.LoadModelTextures = NativeLoadModelTextures,
	.ForceLayoutUpdate = NativeForceLayoutUpdate,
	.ForceTesselationUpdate = NativeForceTesselationUpdate,
	.SetAutoShowMetal = NativeSetAutoShowMetal,
	.SetUnitIconDraw = NativeSetUnitIconDraw,
	.SetUnitIcon = NativeSetUnitIcon,
	.SetUnitDefIcon = NativeSetUnitDefIcon,
	.SetUnitDefImage = NativeSetUnitDefImage,
	.SetCustomPaletteColor = NativeSetCustomPaletteColor,
	.SetUnitPaletteIndex = NativeSetUnitPaletteIndex,
	.SetFeaturePaletteIndex = NativeSetFeaturePaletteIndex,
	.SetEngineBuildSquareRendering = NativeSetEngineBuildSquareRendering,
	.SetFeatureNoDraw = NativeSetFeatureNoDraw,
	.SetFeatureEngineDrawMask = NativeSetFeatureEngineDrawMask,
	.SetFeatureAlwaysUpdateMatrix = NativeSetFeatureAlwaysUpdateMatrix,
	.SetFeatureFade = NativeSetFeatureFade,
	.SetNanoProjectileParams = NativeSetNanoProjectileParams,
	.PreloadFeatureDefModel = NativePreloadFeatureDefModel,
	.PreloadUnitDefModel = NativePreloadUnitDefModel,
	.SelectUnitMap = NativeSelectUnitMap,
	.DeselectUnitMap = NativeDeselectUnitMap,
	.DrawUnitCommands = NativeDrawUnitCommands,
	.SetWaterTexture = NativeSetWaterTexture,
	.GetWaterTexture = NativeGetWaterTexture,
	.SetUnitLuaDraw = NativeSetUnitLuaDraw,
};
