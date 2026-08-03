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
	Float3 pos;        // controller px/py/pz (the Lua state map), not rendered position
	Float3 dir;        // controller dx/dy/dz
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

// Queries
struct GetCameraNamesQuery {
	uint8_t _unused;
};

struct GetCameraNamesResult {
	const Error* error;
	const char** names;
	uint32_t count;
};

struct GetCameraStateQuery {
	bool useTable;
};

struct GetCameraStateResult {
	const Error* error;
	CameraState state;
};

struct GetCameraPositionQuery { uint8_t _unused; };

struct GetCameraPositionResult {
	const Error* error;
	Float3 position;
};

struct GetCameraDirectionQuery { uint8_t _unused; };

struct GetCameraDirectionResult {
	const Error* error;
	Float3 direction;
};

struct GetCameraFOVQuery { uint8_t _unused; };

struct GetCameraFOVResult {
	const Error* error;
	float fov;
};

struct WorldToScreenCoordsQuery {
	Float3 worldPos;
};

struct WorldToScreenCoordsResult {
	const Error* error;
	Float3 screenPos;
	bool valid;
};

struct TraceScreenRayQuery {
	// Same numeric arguments as Spring.TraceScreenRay. The implementation
	// performs the same conversion to the renderer's pixel-ray coordinates;
	// callers should pass the value they would pass to the Lua function.
	float screenX;
	float screenY;
	bool onlyCoords;
	bool useMinimap;
	bool includeSky;
	bool ignoreWater;
	float heightOffset;
};

struct TraceScreenRayResult {
	const Error* error;
	int32_t hitType;  // 0=none, 1=unit, 2=feature, 3=ground, 4=sky
	int32_t hitID;    // Unit or feature ID
	Float3 hitPos;    // Trace position, or heightOffset plane for sky
};

struct GetPixelDirQuery {
	float screenX;
	float screenY;
};

struct GetPixelDirResult {
	const Error* error;
	Float3 direction;
};

struct SetCameraStateQuery {
	CameraState state;
	float transitionTime;
	float transitionTimeFactor;
	float transitionTimeExponent;
};

struct SetCameraStateResult {
	const Error* error;
	bool success;
};

struct SetCameraTargetQuery {
	Float3 target;
	float transitionTime;
	bool hasTransitionTime;
	float dirX;
	bool hasDirX;
	float dirY;
	bool hasDirY;
	float dirZ;
	bool hasDirZ;
};

struct SetCameraTargetResult {
	const Error* error;
	bool success;
};

// API structure
struct CameraApi {
	void (*GetCameraNames)(
		const GetCameraNamesQuery* query,
		GetCameraNamesResult* result
	);

	void (*GetCameraState)(
		const GetCameraStateQuery* query,
		GetCameraStateResult* result
	);

	void (*GetCameraPosition)(
		const GetCameraPositionQuery* query,
		GetCameraPositionResult* result
	);

	void (*GetCameraDirection)(
		const GetCameraDirectionQuery* query,
		GetCameraDirectionResult* result
	);

	void (*GetCameraFOV)(
		const GetCameraFOVQuery* query,
		GetCameraFOVResult* result
	);

	void (*WorldToScreenCoords)(
		const WorldToScreenCoordsQuery* query,
		WorldToScreenCoordsResult* result
	);

	void (*TraceScreenRay)(
		const TraceScreenRayQuery* query,
		TraceScreenRayResult* result
	);

	void (*GetPixelDir)(
		const GetPixelDirQuery* query,
		GetPixelDirResult* result
	);

	void (*SetCameraState)(
		const SetCameraStateQuery* query,
		SetCameraStateResult* result
	);

	void (*SetCameraTarget)(
		const SetCameraTargetQuery* query,
		SetCameraTargetResult* result
	);
};

extern const CameraApi CAMERA_API;

#ifdef __cplusplus
}
#endif
