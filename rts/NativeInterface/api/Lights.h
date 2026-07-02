/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Lights API (unsynced)
// @see rts/Lua/LuaUnsyncedCtrl.cpp (AddMapLight, AddModelLight, etc.)
// ============================================================================

struct LightParams {
	float position[3];
	float direction[3];
	float ambientColor[3];
	float diffuseColor[3];
	float specularColor[3];
	float intensityWeight[3];
	float attenuation[3];
	float ambientDecayRate[3];
	float diffuseDecayRate[3];
	float specularDecayRate[3];
	float decayFunctionType[3];
	float radius;
	float fov;
	uint32_t ttl;
	uint32_t priority;
	bool ignoreLOS;
	bool localSpace;
};

struct AddMapLightQuery { LightParams params; };
struct AddMapLightResult { const Error* error; uint32_t lightHandle; };

struct AddModelLightQuery { LightParams params; };
struct AddModelLightResult { const Error* error; uint32_t lightHandle; };

struct UpdateMapLightQuery { uint32_t lightHandle; LightParams params; };
struct UpdateMapLightResult { const Error* error; bool success; };

struct UpdateModelLightQuery { uint32_t lightHandle; LightParams params; };
struct UpdateModelLightResult { const Error* error; bool success; };

struct AddLightTrackingTargetQuery { uint32_t lightHandle; int32_t objectID; bool trackUnit; bool enableTracking; };
struct AddLightTrackingTargetResult { const Error* error; bool success; };

struct SetMapLightTrackingStateQuery { uint32_t lightHandle; int32_t objectID; bool enableTracking; bool trackUnit; };
struct SetMapLightTrackingStateResult { const Error* error; bool success; };

struct SetModelLightTrackingStateQuery { uint32_t lightHandle; int32_t objectID; bool enableTracking; bool trackUnit; };
struct SetModelLightTrackingStateResult { const Error* error; bool success; };

struct LightsApi {
	void (*AddMapLight)(const AddMapLightQuery* query, AddMapLightResult* result);
	void (*AddModelLight)(const AddModelLightQuery* query, AddModelLightResult* result);
	void (*UpdateMapLight)(const UpdateMapLightQuery* query, UpdateMapLightResult* result);
	void (*UpdateModelLight)(const UpdateModelLightQuery* query, UpdateModelLightResult* result);
	void (*SetMapLightTrackingState)(const SetMapLightTrackingStateQuery* query, SetMapLightTrackingStateResult* result);
	void (*SetModelLightTrackingState)(const SetModelLightTrackingStateQuery* query, SetModelLightTrackingStateResult* result);
	void (*AddLightTrackingTarget)(const AddLightTrackingTargetQuery* query, AddLightTrackingTargetResult* result);
};

extern const LightsApi LIGHTS_API;

#ifdef __cplusplus
}
#endif
