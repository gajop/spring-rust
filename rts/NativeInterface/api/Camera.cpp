#include "Camera.h"

#include "Game/Camera.h"
#include "Game/CameraHandler.h"
#include "Game/UI/MouseHandler.h"
#include "Game/TraceRay.h"
#include "Sim/Units/Unit.h"
#include "Sim/Features/Feature.h"
#include "System/float4.h"
#include "System/float3.h"

#include <algorithm>

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

	const float3& pos = camera->GetPos();
	result->state.pos.x = pos.x;
	result->state.pos.y = pos.y;
	result->state.pos.z = pos.z;

	const float3& dir = camera->GetDir();
	result->state.dir.x = dir.x;
	result->state.dir.y = dir.y;
	result->state.dir.z = dir.z;

	const float3& up = camera->GetUp();
	result->state.up.x = up.x;
	result->state.up.y = up.y;
	result->state.up.z = up.z;

	const float3& right = camera->GetRight();
	result->state.right.x = right.x;
	result->state.right.y = right.y;
	result->state.right.z = right.z;

	result->state.fov = camera->GetVFOV();

	const float3& rot = camera->GetRot();
	result->state.rx = rot.x;
	result->state.ry = rot.y;
	result->state.rz = rot.z;

	// Controller-specific state (simplified)
	CCameraController::StateMap camState;
	camHandler->GetState(camState);

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

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	// Get pixel direction
	const float3 dir = camera->CalcPixelDir(static_cast<int>(query->screenX), static_cast<int>(query->screenY));
	const float3& pos = camera->GetPos();
	(void)query->useMinimap;
	(void)query->includeSky;
	(void)query->heightOffset;

	// Trace against units, features, and ground. `onlyCoords` asks for the
	// ground position regardless of what stands on it, which is what a terrain
	// brush needs to paint under a unit; it maps onto GuiTraceRay's groundOnly.
	const CUnit* hitUnit = nullptr;
	const CFeature* hitFeature = nullptr;

	const float dist = TraceRay::GuiTraceRay(
		pos, dir, 9999999.0f, nullptr, hitUnit, hitFeature, false, query->onlyCoords, query->ignoreWater);

	result->error = nullptr;
	result->hitType = 0; // No hit
	result->hitID = -1;

	if (hitUnit != nullptr) {
		result->hitType = 1; // Unit
		result->hitID = hitUnit->id;
		const float3 hitPos = pos + (dir * dist);
		result->hitPos.x = hitPos.x;
		result->hitPos.y = hitPos.y;
		result->hitPos.z = hitPos.z;
	} else if (hitFeature != nullptr) {
		result->hitType = 2; // Feature
		result->hitID = hitFeature->id;
		const float3 hitPos = pos + (dir * dist);
		result->hitPos.x = hitPos.x;
		result->hitPos.y = hitPos.y;
		result->hitPos.z = hitPos.z;
	} else if (dist > 0.0f) {
		result->hitType = 3; // Ground
		const float3 hitPos = pos + (dir * dist);
		result->hitPos.x = hitPos.x;
		result->hitPos.y = hitPos.y;
		result->hitPos.z = hitPos.z;
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

// Control (unsynced) - simplified implementations
static void NativeSetCameraState(const SetCameraStateQuery* query, SetCameraStateResult* result)
{
	bufferPos = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	// Simplified: just return false as camera state setting requires
	// complex controller-specific logic
	(void)query->state;
	(void)query->transitionTime;
	(void)query->transitionTimeFactor;
	(void)query->transitionTimeExponent;
	result->error = nullptr;
	result->success = false;
}

static void NativeSetCameraTarget(const SetCameraTargetQuery* query, SetCameraTargetResult* result)
{
	bufferPos = 0;

	if (!IsReady() || mouse == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	float4 targetPos = {
		query->target.x,
		query->target.y,
		query->target.z,
		std::max(0.0f, query->transitionTime),
	};

	const float3 targetDir = camera->GetDir();

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
