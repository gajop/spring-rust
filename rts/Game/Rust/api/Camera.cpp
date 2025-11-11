#include "Camera.h"

#include "Game/Camera.h"
#include "Game/CameraHandler.h"
#include "Game/TraceRay.h"
#include "System/float3.h"
#include <vector>
#include <string>

namespace {

// Error constants
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
static StringArray NativeGetCameraNames()
{
	StringArray result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Use static storage - valid for call duration only
	static thread_local std::vector<const char*> cameraNames;
	static thread_local std::vector<std::string> cameraStrings;

	cameraNames.clear();
	cameraStrings.clear();

	const auto& controllers = camHandler->GetControllers();
	for (size_t i = 0; i < controllers.size(); ++i) {
		if (controllers[i] != nullptr) {
			cameraStrings.push_back(controllers[i]->GetName());
		}
	}

	for (const auto& str : cameraStrings) {
		cameraNames.push_back(str.c_str());
	}

	result.data = cameraNames.data();
	result.length = static_cast<uint32_t>(cameraNames.size());
	return result;
}

static CameraStateResult NativeGetCameraState()
{
	CameraStateResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Use static storage for name
	static thread_local std::string cameraName;
	cameraName = camHandler->GetCurrentController().GetName();

	result.state.name = cameraName.c_str();

	const float3& pos = camera->GetPos();
	result.state.pos.x = pos.x;
	result.state.pos.y = pos.y;
	result.state.pos.z = pos.z;

	const float3& dir = camera->GetDir();
	result.state.dir.x = dir.x;
	result.state.dir.y = dir.y;
	result.state.dir.z = dir.z;

	const float3& up = camera->GetUp();
	result.state.up.x = up.x;
	result.state.up.y = up.y;
	result.state.up.z = up.z;

	const float3& right = camera->GetRight();
	result.state.right.x = right.x;
	result.state.right.y = right.y;
	result.state.right.z = right.z;

	result.state.fov = camera->GetVFOV();

	const float3& rot = camera->GetRot();
	result.state.rx = rot.x;
	result.state.ry = rot.y;
	result.state.rz = rot.z;

	// Controller-specific state (simplified)
	CCameraController::StateMap camState;
	camHandler->GetState(camState);

	result.state.dist = camState["dist"];
	result.state.height = camState["height"];
	result.state.angle = camState["angle"];

	return result;
}

static Float3Result NativeGetCameraPosition()
{
	Float3Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const float3& pos = camera->GetPos();
	result.value.x = pos.x;
	result.value.y = pos.y;
	result.value.z = pos.z;
	return result;
}

static Float3Result NativeGetCameraDirection()
{
	Float3Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const float3& dir = camera->GetDir();
	result.value.x = dir.x;
	result.value.y = dir.y;
	result.value.z = dir.z;
	return result;
}

static FloatResult NativeGetCameraFOV()
{
	FloatResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	result.value = camera->GetVFOV();
	return result;
}

// Conversions
static WorldCoordResult NativeWorldToScreenCoords(Float3 worldPos)
{
	WorldCoordResult result = {};
	result.valid = false;

	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const float3 pos(worldPos.x, worldPos.y, worldPos.z);
	const float3 vpPos = camera->CalcViewPortCoordinates(pos);

	result.worldPos.x = vpPos.x;
	result.worldPos.y = vpPos.y;
	result.worldPos.z = vpPos.z;
	result.valid = true;

	return result;
}

static TraceRayResult NativeTraceScreenRay(float screenX, float screenY, bool onlyCoords)
{
	TraceRayResult result = {};
	result.hitType = 0; // No hit
	result.hitID = -1;

	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Get pixel direction
	const float3 dir = camera->CalcPixelDir(static_cast<int>(screenX), static_cast<int>(screenY));
	const float3& pos = camera->GetPos();

	// Simplified: trace against units, features, and ground
	const CUnit* hitUnit = nullptr;
	const CFeature* hitFeature = nullptr;

	const float dist = TraceRay::GuiTraceRay(pos, dir, 9999999.0f, nullptr, hitUnit, hitFeature, false, false, true);

	if (hitUnit != nullptr) {
		result.hitType = 1; // Unit
		result.hitID = hitUnit->id;
		const float3 hitPos = pos + (dir * dist);
		result.hitPos.x = hitPos.x;
		result.hitPos.y = hitPos.y;
		result.hitPos.z = hitPos.z;
	} else if (hitFeature != nullptr) {
		result.hitType = 2; // Feature
		result.hitID = hitFeature->id;
		const float3 hitPos = pos + (dir * dist);
		result.hitPos.x = hitPos.x;
		result.hitPos.y = hitPos.y;
		result.hitPos.z = hitPos.z;
	} else if (dist > 0.0f) {
		result.hitType = 3; // Ground
		const float3 hitPos = pos + (dir * dist);
		result.hitPos.x = hitPos.x;
		result.hitPos.y = hitPos.y;
		result.hitPos.z = hitPos.z;
	}

	return result;
}

static Float3Result NativeGetPixelDir(float screenX, float screenY)
{
	Float3Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const float3 dir = camera->CalcPixelDir(static_cast<int>(screenX), static_cast<int>(screenY));
	result.value.x = dir.x;
	result.value.y = dir.y;
	result.value.z = dir.z;
	return result;
}

// Control (unsynced) - simplified implementations
static BoolResult NativeSetCameraState(CameraState state, float transitionTime)
{
	BoolResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Simplified: just return false as camera state setting requires
	// complex controller-specific logic
	result.value = false;
	return result;
}

static BoolResult NativeSetCameraTarget(Float3 target, float transitionTime)
{
	BoolResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Simplified: not implemented
	result.value = false;
	return result;
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
