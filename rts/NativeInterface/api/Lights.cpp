/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "Lights.h"

#include "Rendering/GL/Light.h"
#include "Rendering/GL/LightHandler.h"
#include "Rendering/GlobalRendering.h"
#include "Rendering/Units/UnitDrawer.h"
#include "Game/Camera.h"
#include "Game/GlobalUnsynced.h"
#include "Lua/LuaUtils.h"
#include "Map/BaseGroundDrawer.h"
#include "Sim/Misc/GlobalSynced.h"
#include "Sim/Misc/QuadField.h"
#include "Sim/Misc/TeamHandler.h"
#include "Sim/Objects/SolidObject.h"
#include "Sim/Projectiles/Projectile.h"
#include "Sim/Projectiles/ProjectileHandler.h"
#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Map/ReadMap.h"

namespace {

static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Lighting system not available"
};

static const Error INVALID_LIGHT_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid light handle"
};

static const Error INVALID_TARGET_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid tracking target"
};

static GL::LightHandler* GetMapLightHandler()
{
	if (readMap == nullptr || readMap->GetGroundDrawer() == nullptr)
		return nullptr;
	return readMap->GetGroundDrawer()->GetLightHandler();
}

static GL::LightHandler* GetModelLightHandler()
{
	if (unitDrawer == nullptr)
		return nullptr;
	return unitDrawer->GetLightHandler();
}

static void ApplyLightParams(const LightParams& params, GL::Light& light)
{
	light.SetPosition(params.position);
	light.SetDirection(params.direction);
	light.SetAmbientColor(params.ambientColor);
	light.SetDiffuseColor(params.diffuseColor);
	light.SetSpecularColor(params.specularColor);
	light.SetIntensityWeight(params.intensityWeight);
	light.SetAttenuation(params.attenuation);
	light.SetAmbientDecayRate(params.ambientDecayRate);
	light.SetDiffuseDecayRate(params.diffuseDecayRate);
	light.SetSpecularDecayRate(params.specularDecayRate);
	light.SetDecayFunctionType(params.decayFunctionType);
	light.SetRadius(std::max(1.0f, params.radius));
	light.SetFOV(std::max(0.0f, std::min(180.0f, params.fov)));
	light.SetTTL(params.ttl);
	light.SetPriority(params.priority);
	light.SetIgnoreLOS(params.ignoreLOS);
	light.SetLocalSpace(params.localSpace);
}

static bool AddTrackingTarget(const AddLightTrackingTargetQuery* query, GL::LightHandler* handler, GL::Light* light)
{
	if (handler == nullptr || light == nullptr)
		return false;

	bool ret = false;

	if (query->trackUnit) {
		CUnit* unit = unitHandler.GetUnit(query->objectID);
		if (unit != nullptr) {
			if (query->enableTracking) {
				if (light->GetTrackObject() == nullptr) {
					light->AddDeathDependence(unit, DEPENDENCE_LIGHT);
					light->SetTrackObject(unit);
					light->SetTrackType(GL::Light::TRACK_TYPE_UNIT);
					ret = true;
				}
			} else {
				if (light->GetTrackObject() == unit) {
					light->DeleteDeathDependence(unit, DEPENDENCE_LIGHT);
					light->SetTrackObject(nullptr);
					ret = true;
				}
			}
		}
	} else {
		CProjectile* proj = projectileHandler.GetProjectileBySyncedID(query->objectID);
		if (proj != nullptr) {
			if (query->enableTracking) {
				if (light->GetTrackObject() == nullptr) {
					light->AddDeathDependence(proj, DEPENDENCE_LIGHT);
					light->SetTrackObject(proj);
					light->SetTrackType(GL::Light::TRACK_TYPE_PROJ);
					ret = true;
				}
			} else {
				if (light->GetTrackObject() == proj) {
					light->DeleteDeathDependence(proj, DEPENDENCE_LIGHT);
					light->SetTrackObject(nullptr);
					ret = true;
				}
			}
		}
	}

	return ret;
}

static void NativeAddMapLight(const AddMapLightQuery* query, AddMapLightResult* result)
{
	result->error = nullptr;
	result->lightHandle = static_cast<uint32_t>(-1);

	GL::LightHandler* handler = GetMapLightHandler();
	if (handler == nullptr) {
		// Lua returns -1 when dynamic map lighting is unavailable; this is a
		// normal result, not an API error.
		return;
	}

	GL::Light light;
	ApplyLightParams(query->params, light);
	result->lightHandle = handler->AddLight(light);
}

static void NativeAddModelLight(const AddModelLightQuery* query, AddModelLightResult* result)
{
	result->error = nullptr;
	result->lightHandle = static_cast<uint32_t>(-1);

	GL::LightHandler* handler = GetModelLightHandler();
	if (handler == nullptr) {
		// Lua returns -1 when dynamic model lighting is unavailable; this is a
		// normal result, not an API error.
		return;
	}

	GL::Light light;
	ApplyLightParams(query->params, light);
	result->lightHandle = handler->AddLight(light);
}

static void NativeUpdateMapLight(const UpdateMapLightQuery* query, UpdateMapLightResult* result)
{
	result->error = nullptr;
	result->success = false;

	GL::LightHandler* handler = GetMapLightHandler();
	if (handler == nullptr) {
		return;
	}

	GL::Light* light = handler->GetLight(query->lightHandle);
	if (light == nullptr) {
		return;
	}

	ApplyLightParams(query->params, *light);
	result->success = true;
}

static void NativeUpdateModelLight(const UpdateModelLightQuery* query, UpdateModelLightResult* result)
{
	result->error = nullptr;
	result->success = false;

	GL::LightHandler* handler = GetModelLightHandler();
	if (handler == nullptr) {
		return;
	}

	GL::Light* light = handler->GetLight(query->lightHandle);
	if (light == nullptr) {
		return;
	}

	ApplyLightParams(query->params, *light);
	result->success = true;
}

static void NativeSetMapLightTrackingState(const SetMapLightTrackingStateQuery* query, SetMapLightTrackingStateResult* result)
{
	result->error = nullptr;
	result->success = false;

	GL::LightHandler* handler = GetMapLightHandler();
	if (handler == nullptr) {
		return;
	}

	GL::Light* light = handler->GetLight(query->lightHandle);
	if (light == nullptr) {
		return;
	}

	AddLightTrackingTargetQuery trackQuery = {
		.lightHandle = query->lightHandle,
		.objectID = query->objectID,
		.trackUnit = query->trackUnit,
		.enableTracking = query->enableTracking
	};

	result->success = AddTrackingTarget(&trackQuery, handler, light);
	if (!result->success) {
		result->error = &INVALID_TARGET_ERROR;
	}
}

static void NativeSetModelLightTrackingState(const SetModelLightTrackingStateQuery* query, SetModelLightTrackingStateResult* result)
{
	result->error = nullptr;
	result->success = false;

	GL::LightHandler* handler = GetModelLightHandler();
	if (handler == nullptr) {
		return;
	}

	GL::Light* light = handler->GetLight(query->lightHandle);
	if (light == nullptr) {
		return;
	}

	AddLightTrackingTargetQuery trackQuery = {
		.lightHandle = query->lightHandle,
		.objectID = query->objectID,
		.trackUnit = query->trackUnit,
		.enableTracking = query->enableTracking
	};

	result->success = AddTrackingTarget(&trackQuery, handler, light);
	if (!result->success) {
		result->error = &INVALID_TARGET_ERROR;
	}
}

static void NativeAddLightTrackingTarget(const AddLightTrackingTargetQuery* query, AddLightTrackingTargetResult* result)
{
	result->error = nullptr;
	result->success = false;

	// Try map lights first, then model lights
	GL::LightHandler* handler = GetMapLightHandler();
	GL::Light* light = (handler != nullptr) ? handler->GetLight(query->lightHandle) : nullptr;

	if (light == nullptr) {
		handler = GetModelLightHandler();
		light = (handler != nullptr) ? handler->GetLight(query->lightHandle) : nullptr;
	}

	if (handler == nullptr || light == nullptr) {
		result->error = (handler == nullptr) ? &NOT_READY_ERROR : &INVALID_LIGHT_ERROR;
		return;
	}

	if (AddTrackingTarget(query, handler, light)) {
		result->success = true;
	} else {
		result->error = &INVALID_TARGET_ERROR;
	}
}

} // namespace

const LightsApi LIGHTS_API = {
	.AddMapLight = NativeAddMapLight,
	.AddModelLight = NativeAddModelLight,
	.UpdateMapLight = NativeUpdateMapLight,
	.UpdateModelLight = NativeUpdateModelLight,
	.SetMapLightTrackingState = NativeSetMapLightTrackingState,
	.SetModelLightTrackingState = NativeSetModelLightTrackingState,
	.AddLightTrackingTarget = NativeAddLightTrackingTarget,
};
