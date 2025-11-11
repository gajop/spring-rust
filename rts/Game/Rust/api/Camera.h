#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Camera API
// @see rts/Lua/LuaUnsyncedRead.cpp, LuaUnsyncedCtrl.cpp
//
// Camera queries and control (unsynced)
// ============================================================================

// Camera state
struct CameraState {
	const char* name;  // "fps", "ta", "spring", "rot", "ov", etc.
	Float3 pos;
	Float3 dir;
	Float3 up;
	Float3 right;
	float fov;
	float rx;  // Rotation x
	float ry;  // Rotation y
	float rz;  // Rotation z
	float dist;
	float height;
	float angle;
};

struct CameraStateResult {
	const Error* error;
	CameraState state;
};

// Screen to world conversion
struct ScreenCoord {
	float x;
	float y;
};

struct WorldCoordResult {
	const Error* error;
	Float3 worldPos;
	bool valid;
};

// Trace ray
struct TraceRayResult {
	const Error* error;
	int32_t hitType;  // 0=none, 1=unit, 2=feature, 3=ground
	int32_t hitID;    // Unit or feature ID
	Float3 hitPos;
};

// API structure
struct CameraApi {
	// Query camera state
	StringArray (*GetCameraNames)();
	CameraStateResult (*GetCameraState)();
	Float3Result (*GetCameraPosition)();
	Float3Result (*GetCameraDirection)();
	FloatResult (*GetCameraFOV)();

	// Conversions
	WorldCoordResult (*WorldToScreenCoords)(Float3 worldPos);
	TraceRayResult (*TraceScreenRay)(float screenX, float screenY, bool onlyCoords);
	Float3Result (*GetPixelDir)(float screenX, float screenY);

	// Control (unsynced)
	BoolResult (*SetCameraState)(CameraState state, float transitionTime);
	BoolResult (*SetCameraTarget)(Float3 target, float transitionTime);
};

extern const CameraApi CAMERA_API;

#ifdef __cplusplus
}
#endif
