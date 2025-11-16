#include "Tracing.h"

#include "Game/TraceRay.h"
#include "Sim/Units/Unit.h"
#include "Sim/Features/Feature.h"
#include "Map/Ground.h"
#include "System/float3.h"

namespace {

// Scratch buffer for dynamic data
static thread_local char scratchBuffer[1024];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Tracing system not ready"
};

// General ray trace
static void NativeTraceRay(const TraceRayQuery* query, TraceRayResult* result)
{
	bufferPos = 0;

	const Ray& ray = query->ray;
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

	result->error = nullptr;
	result->hit = false;
	result->hitType = 0;
	result->hitID = -1;

	if (hitUnit != nullptr) {
		result->hit = true;
		result->hitType = 1; // Unit
		result->hitID = hitUnit->id;
		const float3 hitPos = origin + (dir * dist);
		result->hitPos.x = hitPos.x;
		result->hitPos.y = hitPos.y;
		result->hitPos.z = hitPos.z;
	} else if (hitFeature != nullptr) {
		result->hit = true;
		result->hitType = 2; // Feature
		result->hitID = hitFeature->id;
		const float3 hitPos = origin + (dir * dist);
		result->hitPos.x = hitPos.x;
		result->hitPos.y = hitPos.y;
		result->hitPos.z = hitPos.z;
	} else if (dist > 0.0f) {
		result->hit = true;
		result->hitType = 3; // Ground
		const float3 hitPos = origin + (dir * dist);
		result->hitPos.x = hitPos.x;
		result->hitPos.y = hitPos.y;
		result->hitPos.z = hitPos.z;

		// Get ground normal
		const float3 normal = CGround::GetNormal(hitPos.x, hitPos.z);
		result->hitNormal.x = normal.x;
		result->hitNormal.y = normal.y;
		result->hitNormal.z = normal.z;
	}
}

// Unit-only trace
static void NativeTraceRayUnits(const TraceRayUnitsQuery* query, TraceRayUnitsResult* result)
{
	bufferPos = 0;

	const Ray& ray = query->ray;
	const float3 origin(ray.origin.x, ray.origin.y, ray.origin.z);
	const float3 dir(ray.direction.x, ray.direction.y, ray.direction.z);

	CUnit* hitUnit = nullptr;
	CFeature* hitFeature = nullptr;

	const int traceFlags = Collision::NOFEATURES | Collision::NOGROUND;
	const float dist = TraceRay::TraceRay(origin, dir, ray.length, traceFlags, ray.allyTeamID, nullptr, hitUnit, hitFeature, nullptr);

	result->error = nullptr;
	result->hit = false;
	result->hitType = 0;
	result->hitID = -1;

	if (hitUnit != nullptr) {
		result->hit = true;
		result->hitType = 1; // Unit
		result->hitID = hitUnit->id;
		const float3 hitPos = origin + (dir * dist);
		result->hitPos.x = hitPos.x;
		result->hitPos.y = hitPos.y;
		result->hitPos.z = hitPos.z;
	}
}

// Feature-only trace
static void NativeTraceRayFeatures(const TraceRayFeaturesQuery* query, TraceRayFeaturesResult* result)
{
	bufferPos = 0;

	const Ray& ray = query->ray;
	const float3 origin(ray.origin.x, ray.origin.y, ray.origin.z);
	const float3 dir(ray.direction.x, ray.direction.y, ray.direction.z);

	CUnit* hitUnit = nullptr;
	CFeature* hitFeature = nullptr;

	const int traceFlags = Collision::NOUNITS | Collision::NOGROUND;
	const float dist = TraceRay::TraceRay(origin, dir, ray.length, traceFlags, ray.allyTeamID, nullptr, hitUnit, hitFeature, nullptr);

	result->error = nullptr;
	result->hit = false;
	result->hitType = 0;
	result->hitID = -1;

	if (hitFeature != nullptr) {
		result->hit = true;
		result->hitType = 2; // Feature
		result->hitID = hitFeature->id;
		const float3 hitPos = origin + (dir * dist);
		result->hitPos.x = hitPos.x;
		result->hitPos.y = hitPos.y;
		result->hitPos.z = hitPos.z;
	}
}

// Ground trace between positions
static void NativeTraceRayGroundBetweenPositions(const TraceRayGroundBetweenPositionsQuery* query, TraceRayGroundBetweenPositionsResult* result)
{
	bufferPos = 0;

	const float3 start(query->start.x, query->start.y, query->start.z);
	const float3 end(query->end.x, query->end.y, query->end.z);
	const float3 dir = (end - start).SafeNormalize();

	// Simple ground intersection at mid-point
	const float3 hitPos = (start + end) * 0.5f;

	result->error = nullptr;
	result->hit = true;
	result->hitPos.x = hitPos.x;
	result->hitPos.y = CGround::GetHeightReal(hitPos.x, hitPos.z);
	result->hitPos.z = hitPos.z;

	const float3 normal = CGround::GetNormal(hitPos.x, hitPos.z);
	result->hitNormal.x = normal.x;
	result->hitNormal.y = normal.y;
	result->hitNormal.z = normal.z;
}

// Ground trace in direction
static void NativeTraceRayGroundInDirection(const TraceRayGroundInDirectionQuery* query, TraceRayGroundInDirectionResult* result)
{
	bufferPos = 0;

	const float3 vStart(query->start.x, query->start.y, query->start.z);
	const float3 vDir(query->dir.x, query->dir.y, query->dir.z);
	const float3 hitPos = vStart + (vDir * query->length);

	result->error = nullptr;
	result->hit = true;
	result->hitPos.x = hitPos.x;
	result->hitPos.y = CGround::GetHeightReal(hitPos.x, hitPos.z);
	result->hitPos.z = hitPos.z;

	const float3 normal = CGround::GetNormal(hitPos.x, hitPos.z);
	result->hitNormal.x = normal.x;
	result->hitNormal.y = normal.y;
	result->hitNormal.z = normal.z;
}

} // namespace

const TracingApi TRACING_API = {
	.TraceRay = NativeTraceRay,
	.TraceRayUnits = NativeTraceRayUnits,
	.TraceRayFeatures = NativeTraceRayFeatures,
	.TraceRayGroundBetweenPositions = NativeTraceRayGroundBetweenPositions,
	.TraceRayGroundInDirection = NativeTraceRayGroundInDirection,
};
