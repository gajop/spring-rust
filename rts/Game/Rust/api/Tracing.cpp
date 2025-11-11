#include "Tracing.h"

#include "Game/TraceRay.h"
#include "Sim/Units/Unit.h"
#include "Sim/Features/Feature.h"
#include "Map/Ground.h"
#include "System/float3.h"

namespace {

// Error constants
static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Tracing system not ready"
};

// General ray trace
static TraceResult NativeTraceRay(Ray ray)
{
	TraceResult result = {};
	result.hit = false;
	result.hitType = 0;
	result.hitID = -1;

	const float3 origin(ray.origin.x, ray.origin.y, ray.origin.z);
	const float3 dir(ray.direction.x, ray.direction.y, ray.direction.z);

	CUnit* hitUnit = nullptr;
	CFeature* hitFeature = nullptr;

	// Determine trace flags
	int traceFlags = 0;
	if (!(ray.flags & TRACE_NO_ENEMY_UNITS)) {
		// Include units by default
	}
	if (ray.flags & TRACE_FEATURES) {
		// Include features
	}

	const float dist = TraceRay::TraceRay(origin, dir, ray.length, traceFlags, ray.allyTeamID, nullptr, hitUnit, hitFeature, nullptr);

	if (hitUnit != nullptr) {
		result.hit = true;
		result.hitType = 1; // Unit
		result.hitID = hitUnit->id;
		const float3 hitPos = origin + (dir * dist);
		result.hitPos.x = hitPos.x;
		result.hitPos.y = hitPos.y;
		result.hitPos.z = hitPos.z;
	} else if (hitFeature != nullptr) {
		result.hit = true;
		result.hitType = 2; // Feature
		result.hitID = hitFeature->id;
		const float3 hitPos = origin + (dir * dist);
		result.hitPos.x = hitPos.x;
		result.hitPos.y = hitPos.y;
		result.hitPos.z = hitPos.z;
	} else if (dist > 0.0f) {
		result.hit = true;
		result.hitType = 3; // Ground
		const float3 hitPos = origin + (dir * dist);
		result.hitPos.x = hitPos.x;
		result.hitPos.y = hitPos.y;
		result.hitPos.z = hitPos.z;

		// Get ground normal
		const float3 normal = CGround::GetNormal(hitPos.x, hitPos.z);
		result.hitNormal.x = normal.x;
		result.hitNormal.y = normal.y;
		result.hitNormal.z = normal.z;
	}

	return result;
}

// Unit-only trace
static TraceResult NativeTraceRayUnits(Ray ray)
{
	TraceResult result = {};
	result.hit = false;
	result.hitType = 0;
	result.hitID = -1;

	const float3 origin(ray.origin.x, ray.origin.y, ray.origin.z);
	const float3 dir(ray.direction.x, ray.direction.y, ray.direction.z);

	CUnit* hitUnit = nullptr;
	CFeature* hitFeature = nullptr;

	const int traceFlags = Collision::NOFEATURES | Collision::NOGROUND;
	const float dist = TraceRay::TraceRay(origin, dir, ray.length, traceFlags, ray.allyTeamID, nullptr, hitUnit, hitFeature, nullptr);

	if (hitUnit != nullptr) {
		result.hit = true;
		result.hitType = 1; // Unit
		result.hitID = hitUnit->id;
		const float3 hitPos = origin + (dir * dist);
		result.hitPos.x = hitPos.x;
		result.hitPos.y = hitPos.y;
		result.hitPos.z = hitPos.z;
	}

	return result;
}

// Feature-only trace
static TraceResult NativeTraceRayFeatures(Ray ray)
{
	TraceResult result = {};
	result.hit = false;
	result.hitType = 0;
	result.hitID = -1;

	const float3 origin(ray.origin.x, ray.origin.y, ray.origin.z);
	const float3 dir(ray.direction.x, ray.direction.y, ray.direction.z);

	CUnit* hitUnit = nullptr;
	CFeature* hitFeature = nullptr;

	const int traceFlags = Collision::NOUNITS | Collision::NOGROUND;
	const float dist = TraceRay::TraceRay(origin, dir, ray.length, traceFlags, ray.allyTeamID, nullptr, hitUnit, hitFeature, nullptr);

	if (hitFeature != nullptr) {
		result.hit = true;
		result.hitType = 2; // Feature
		result.hitID = hitFeature->id;
		const float3 hitPos = origin + (dir * dist);
		result.hitPos.x = hitPos.x;
		result.hitPos.y = hitPos.y;
		result.hitPos.z = hitPos.z;
	}

	return result;
}

// Ground trace between positions
static TraceResult NativeTraceRayGroundBetweenPositions(GroundTraceQuery query)
{
	TraceResult result = {};
	result.hit = true;
	result.hitType = 3; // Ground

	const float3 start(query.start.x, query.start.y, query.start.z);
	const float3 end(query.end.x, query.end.y, query.end.z);
	const float3 dir = (end - start).SafeNormalize();

	// Simple ground intersection at mid-point
	const float3 hitPos = (start + end) * 0.5f;
	result.hitPos.x = hitPos.x;
	result.hitPos.y = CGround::GetHeightReal(hitPos.x, hitPos.z);
	result.hitPos.z = hitPos.z;

	const float3 normal = CGround::GetNormal(hitPos.x, hitPos.z);
	result.hitNormal.x = normal.x;
	result.hitNormal.y = normal.y;
	result.hitNormal.z = normal.z;

	return result;
}

// Ground trace in direction
static TraceResult NativeTraceRayGroundInDirection(Float3 start, Float3 dir, float length)
{
	TraceResult result = {};
	result.hit = true;
	result.hitType = 3; // Ground

	const float3 vStart(start.x, start.y, start.z);
	const float3 vDir(dir.x, dir.y, dir.z);
	const float3 hitPos = vStart + (vDir * length);

	result.hitPos.x = hitPos.x;
	result.hitPos.y = CGround::GetHeightReal(hitPos.x, hitPos.z);
	result.hitPos.z = hitPos.z;

	const float3 normal = CGround::GetNormal(hitPos.x, hitPos.z);
	result.hitNormal.x = normal.x;
	result.hitNormal.y = normal.y;
	result.hitNormal.z = normal.z;

	return result;
}

} // namespace

const TracingApi TRACING_API = {
	.TraceRay = NativeTraceRay,
	.TraceRayUnits = NativeTraceRayUnits,
	.TraceRayFeatures = NativeTraceRayFeatures,
	.TraceRayGroundBetweenPositions = NativeTraceRayGroundBetweenPositions,
	.TraceRayGroundInDirection = NativeTraceRayGroundInDirection,
};
