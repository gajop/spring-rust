#include "Camera.h"

#include "Game/Camera.h"
#include "Game/CameraHandler.h"
#include "Game/UI/MouseHandler.h"
#include "Game/UI/MiniMap.h"
#include "Game/TraceRay.h"
#include "Map/Ground.h"
#include "Rendering/GlobalRendering.h"
#include "Sim/Units/Unit.h"
#include "Sim/Features/Feature.h"
#include "System/float4.h"
#include "System/float3.h"

#include <algorithm>
#include <cmath>

namespace {

// Scratch buffer for dynamic data
static thread_local char scratchBuffer[1024];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Camera system not ready"
};

// Helper: check if ready
static bool IsReady()
{
	return (camera != nullptr) && (camHandler != nullptr);
}

// Query camera state
static void NativeGetCameraNames(const GetCameraNamesQuery* query, GetCameraNamesResult* result)
{
	bufferPos = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const auto& controllers = camHandler->GetControllers();

	// First, write all strings to scratch buffer
	const char** namePointers = reinterpret_cast<const char**>(&scratchBuffer[bufferPos]);
	size_t count = 0;
	size_t ptrArraySize = controllers.size() * sizeof(const char*);

	if (bufferPos + ptrArraySize > sizeof(scratchBuffer)) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	bufferPos += ptrArraySize;

	for (size_t i = 0; i < controllers.size(); ++i) {
		if (controllers[i] != nullptr) {
			std::string name = controllers[i]->GetName();
			char* strBuf = &scratchBuffer[bufferPos];
			size_t len = name.length();

			if (bufferPos + len + 1 > sizeof(scratchBuffer)) {
				result->error = &NOT_READY_ERROR;
				return;
			}

			memcpy(strBuf, name.c_str(), len + 1);
			namePointers[count++] = strBuf;
			bufferPos += len + 1;
		}
	}

	result->error = nullptr;
	result->names = namePointers;
	result->count = static_cast<uint32_t>(count);
}

static void NativeGetCameraState(const GetCameraStateQuery* query, GetCameraStateResult* result)
{
	bufferPos = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	// Copy camera name to scratch buffer
	std::string cameraName = camHandler->GetCurrentController().GetName();
	char* nameBuf = &scratchBuffer[bufferPos];
	size_t len = cameraName.length();

	if (bufferPos + len + 1 > sizeof(scratchBuffer)) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	memcpy(nameBuf, cameraName.c_str(), len + 1);
	bufferPos += len + 1;

	result->error = nullptr;
	result->state.name = nameBuf;

	// `CameraState` follows Spring.GetCameraState: px/py/pz and the
	// controller-specific fields belong to the active controller.  The actual
	// rendered camera position remains available through GetCameraPosition.
	// This distinction matters for overhead cameras, whose controller position
	// is the ground focus while the rendered camera is offset by height.
	CCameraController::StateMap camState;
	camHandler->GetState(camState);
	result->state.pos.x = camState["px"];
	result->state.pos.y = camState["py"];
	result->state.pos.z = camState["pz"];
	result->state.dir.x = camState["dx"];
	result->state.dir.y = camState["dy"];
	result->state.dir.z = camState["dz"];

	const float3& up = camera->GetUp();
	result->state.up.x = up.x;
	result->state.up.y = up.y;
	result->state.up.z = up.z;

	const float3& right = camera->GetRight();
	result->state.right.x = right.x;
	result->state.right.y = right.y;
	result->state.right.z = right.z;

	result->state.fov = camState["fov"];
	result->state.rx = camState["rx"];
	result->state.ry = camState["ry"];
	result->state.rz = camState["rz"];
	result->state.dist = camState["dist"];
	result->state.height = camState["height"];
	result->state.angle = camState["angle"];
}

static void NativeGetCameraPosition(const GetCameraPositionQuery* query, GetCameraPositionResult* result)
{
	bufferPos = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const float3& pos = camera->GetPos();
	result->error = nullptr;
	result->position.x = pos.x;
	result->position.y = pos.y;
	result->position.z = pos.z;
}

static void NativeGetCameraDirection(const GetCameraDirectionQuery* query, GetCameraDirectionResult* result)
{
	bufferPos = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const float3& dir = camera->GetDir();
	result->error = nullptr;
	result->direction.x = dir.x;
	result->direction.y = dir.y;
	result->direction.z = dir.z;
}

static void NativeGetCameraFOV(const GetCameraFOVQuery* query, GetCameraFOVResult* result)
{
	bufferPos = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->error = nullptr;
	result->fov = camera->GetVFOV();
}

// Conversions
static void NativeWorldToScreenCoords(const WorldToScreenCoordsQuery* query, WorldToScreenCoordsResult* result)
{
	bufferPos = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const float3 pos(query->worldPos.x, query->worldPos.y, query->worldPos.z);
	const float3 vpPos = camera->CalcViewPortCoordinates(pos);

	result->error = nullptr;
	result->screenPos.x = vpPos.x;
	result->screenPos.y = vpPos.y;
	result->screenPos.z = vpPos.z;
	result->valid = true;
}

static void NativeTraceScreenRay(const TraceScreenRayQuery* query, TraceScreenRayResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->hitType = 0;
	result->hitID = -1;
	result->hitPos = {};

	if (!IsReady() || globalRendering == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	// Keep the native query on the same coordinate contract as Lua's
	// Spring.TraceScreenRay. Convert to the renderer-space coordinate exactly
	// as the Lua implementation does before calling CalcPixelDir.
	if (!std::isfinite(query->screenX) || !std::isfinite(query->screenY))
		return;

	const int mx = static_cast<int>(query->screenX);
	const int my = static_cast<int>(query->screenY);
	const int wx = mx + globalRendering->viewPosX;
	const int wy = globalRendering->viewSizeY - 1 - my;

	// Match the Lua minimap path before applying the normal viewport bounds.
	// The minimap coordinates are relative to the same viewport as mx/my, while
	// GetMapPosition expects the full renderer coordinates wx/wy.
	if (query->options.useMinimap && minimap != nullptr && !minimap->GetMinimized()) {
		const int px = minimap->GetPosX() - globalRendering->viewPosX;
		const int py = minimap->GetPosY() - globalRendering->viewPosY;
		const int sx = minimap->GetSizeX();
		const int sy = minimap->GetSizeY();

		if ((mx >= px) && (mx < (px + sx)) && (my >= py) && (my < (py + sy))) {
			const float3 mapPos = minimap->GetMapPosition(wx, wy);
			if (!query->options.onlyCoords) {
				const CUnit* unit = minimap->GetSelectUnit(mapPos);
				if (unit != nullptr) {
					result->hitType = 1; // Unit
					result->hitID = unit->id;
					return;
				}
			}

			result->hitType = 3; // Ground
			result->hitPos.x = mapPos.x;
			result->hitPos.y = CGround::GetHeightReal(mapPos.x, mapPos.z, false);
			result->hitPos.z = mapPos.z;
			return;
		}
	}

	// Lua rejects coordinates outside the active viewport. In particular, do
	// not let an off-window mouse event turn into a ray at a clamped edge.
	if (mx < 0 || mx >= globalRendering->viewSizeX || my < 0 || my >= globalRendering->viewSizeY)
		return;

	const float rawRange = camera->GetFarPlaneDist() * 1.4f;
	const float badRange = rawRange - 300.0f;
	const float3 camPos = camera->GetPos();
	const float3 dir = camera->CalcPixelDir(wx, wy);

	// Trace against units, features, and ground. `onlyCoords` asks for the
	// ground position regardless of what stands on it, which is what a terrain
	// brush needs to paint under a unit; it maps onto GuiTraceRay's groundOnly.
	const CUnit* hitUnit = nullptr;
	const CFeature* hitFeature = nullptr;

	const float dist = TraceRay::GuiTraceRay(
		camPos, dir, rawRange, nullptr, hitUnit, hitFeature, true, query->options.onlyCoords, query->options.ignoreWater);
	const float planeDist = CGround::LinePlaneCol(camPos, dir, rawRange, query->options.heightOffset);
	const float3 tracePos = camPos + (dir * dist);
	const float3 planePos = camPos + (dir * planeDist);

	if (hitUnit != nullptr) {
		result->hitType = 1; // Unit
		result->hitID = hitUnit->id;
		result->hitPos.x = tracePos.x;
		result->hitPos.y = tracePos.y;
		result->hitPos.z = tracePos.z;
	} else if (hitFeature != nullptr) {
		result->hitType = 2; // Feature
		result->hitID = hitFeature->id;
		result->hitPos.x = tracePos.x;
		result->hitPos.y = tracePos.y;
		result->hitPos.z = tracePos.z;
	} else if ((dist < 0.0f || dist > badRange) && !query->options.includeSky) {
		// Lua reports no result when the ray misses the map and sky results were
		// not requested. A zero-distance ground hit remains valid.
		return;
	} else if (dist < 0.0f || dist > badRange) {
		// The compact native result has one position, while Lua returns both the
		// ray position and the custom-plane fallback. For a sky hit the fallback
		// is the useful position, so expose it here and use the explicit sky tag.
		result->hitType = 4; // Sky
		result->hitPos.x = planePos.x;
		result->hitPos.y = planePos.y;
		result->hitPos.z = planePos.z;
	} else {
		// Lua's Spring.TraceScreenRay treats a zero-distance terrain
		// intersection as ground. This matters when the camera is exactly on
		// a terrain surface: reporting a miss makes editor brushes disappear
		// even though their cursor has a valid map position.
		result->hitType = 3; // Ground
		result->hitPos.x = tracePos.x;
		result->hitPos.y = tracePos.y;
		result->hitPos.z = tracePos.z;
	}
}

static void NativeGetPixelDir(const GetPixelDirQuery* query, GetPixelDirResult* result)
{
	bufferPos = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const float3 dir = camera->CalcPixelDir(static_cast<int>(query->screenX), static_cast<int>(query->screenY));
	result->error = nullptr;
	result->direction.x = dir.x;
	result->direction.y = dir.y;
	result->direction.z = dir.z;
}

// Control (unsynced)
static void NativeSetCameraState(const SetCameraStateQuery* query, SetCameraStateResult* result)
{
	bufferPos = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	// The query is deliberately a complete controller state, just as the Lua
	// Spring.SetCameraState path is.  Start from the live map so fields not
	// represented in the compact C struct (for example `flipped`, velocities,
	// or the free-camera flags) are preserved instead of being zeroed.
	CCameraController::StateMap state = camHandler->GetState();
	state["fov"] = query->state.fov;
	state["px"] = query->state.pos.x;
	state["py"] = query->state.pos.y;
	state["pz"] = query->state.pos.z;
	state["dx"] = query->state.dir.x;
	state["dy"] = query->state.dir.y;
	state["dz"] = query->state.dir.z;
	state["rx"] = query->state.rx;
	state["ry"] = query->state.ry;
	state["rz"] = query->state.rz;
	state["height"] = query->state.height;
	state["angle"] = query->state.angle;
	state["dist"] = query->state.dist;

	camHandler->SetTransitionParams(query->transitionTimeFactor, query->transitionTimeExponent);
	const bool success = camHandler->SetState(state);
	camHandler->CameraTransition(std::max(0.0f, query->transitionTime));
	result->error = nullptr;
	result->success = success;
}

static void NativeSetCameraTarget(const SetCameraTargetQuery* query, SetCameraTargetResult* result)
{
	bufferPos = 0;

	if (query == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!IsReady() || mouse == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	float4 targetPos = {
		query->target.x,
		query->target.y,
		query->target.z,
		std::max(0.0f, query->options.hasTransitionTime ? query->options.transitionTime : 0.5f),
	};

	const float3 currentDir = camera->GetDir();
	const float3 targetDir = {
		query->options.hasDirX ? query->options.dirX : currentDir.x,
		query->options.hasDirY ? query->options.dirY : currentDir.y,
		query->options.hasDirZ ? query->options.dirZ : currentDir.z,
	};

	camHandler->GetCurrentController().SetPos(targetPos);
	camHandler->GetCurrentController().SetDir(targetDir);
	camHandler->CameraTransition(targetPos.w);

	result->error = nullptr;
	result->success = true;
}

} // namespace

const CameraApi CAMERA_API = {
	.GetCameraNames = NativeGetCameraNames,
	.GetCameraState = NativeGetCameraState,
	.GetCameraPosition = NativeGetCameraPosition,
	.GetCameraDirection = NativeGetCameraDirection,
	.GetCameraFOV = NativeGetCameraFOV,

	.WorldToScreenCoords = NativeWorldToScreenCoords,
	.TraceScreenRay = NativeTraceScreenRay,
	.GetPixelDir = NativeGetPixelDir,

	.SetCameraState = NativeSetCameraState,
	.SetCameraTarget = NativeSetCameraTarget,
};
