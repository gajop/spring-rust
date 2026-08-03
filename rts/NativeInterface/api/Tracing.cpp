#include "Tracing.h"

#include "Game/TraceRay.h"
#include "Game/GlobalUnsynced.h"
#include "Sim/Units/Unit.h"
#include "Sim/Features/Feature.h"
#include "Sim/Misc/CollisionHandler.h"
#include "Sim/Misc/QuadField.h"
#include "Map/Ground.h"
#include "System/float3.h"
#include "System/Sync/SyncChecker.h"

#include <algorithm>
#include <cstring>
#include <tuple>
#include <unordered_set>
#include <vector>

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

static const Error INVALID_ARGUMENT_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid tracing argument"
};

static thread_local std::vector<TraceRayHit> traceHitStorage;

static bool NativeTraceIsSynced()
{
#ifdef SYNCCHECK
	return CSyncChecker::InSyncedCode();
#else
	return false;
#endif
}

static bool NativeTraceHasFullRead()
{
	return NativeTraceIsSynced() || (gu == nullptr) || gu->spectatingFullView;
}

static bool NativeTraceUnitVisible(const CUnit* unit)
{
	if (NativeTraceHasFullRead() || gu->myAllyTeam < 0)
		return NativeTraceHasFullRead();

	if (unit->allyteam == gu->myAllyTeam)
		return true;

	return (unit->losStatus[gu->myAllyTeam] & LOS_INLOS) != 0;
}

static bool NativeTraceFeatureVisible(const CFeature* feature)
{
	if (NativeTraceHasFullRead())
		return true;
	if (gu == nullptr || gu->myAllyTeam < 0)
		return false;

	return feature->IsInLosForAllyTeam(gu->myAllyTeam);
}

static bool IsTraceTypeValid(const char* type)
{
	return type != nullptr && (
		std::strcmp(type, "unit") == 0 ||
		std::strcmp(type, "feature") == 0 ||
		std::strcmp(type, "both") == 0
	);
}

static void NativeTraceRayAllHits(const float3& pos, const float3& dir, float maxLength, const char* type, const Error*& error, TraceRayHit*& hits, uint32_t& count)
{
	traceHitStorage.clear();
	error = nullptr;
	hits = nullptr;
	count = 0;

	if (!IsTraceTypeValid(type)) {
		error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const bool testUnits = (std::strcmp(type, "unit") == 0 || std::strcmp(type, "both") == 0);
	const bool testFeatures = (std::strcmp(type, "feature") == 0 || std::strcmp(type, "both") == 0);

	QuadFieldQuery qfQuery;
	quadField.GetQuadsOnRay(qfQuery, pos, dir, maxLength);

	std::unordered_set<int> testedUnitIDs;
	std::unordered_set<int> testedFeatureIDs;
	std::vector<std::tuple<float, int, int>> foundHits;

	for (const int quadIdx : *qfQuery.quads) {
		const CQuadField::Quad& quad = quadField.GetQuad(quadIdx);

		if (testUnits) {
			for (const CUnit* unit : quad.units) {
				if (!unit->HasCollidableStateBit(CSolidObject::CSTATE_BIT_QUADMAPRAYS))
					continue;
				if (!testedUnitIDs.insert(unit->id).second)
					continue;
				if (!NativeTraceUnitVisible(unit))
					continue;

				CollisionQuery collisionQuery;
				if (!CCollisionHandler::DetectHit(unit, unit->GetTransformMatrix(true), pos, pos + dir * maxLength, &collisionQuery, true))
					continue;

				const float hitLength = collisionQuery.GetHitPosDist(pos, dir);
				if (hitLength <= maxLength)
					foundHits.emplace_back(hitLength, unit->id, 1);
			}
		}

		if (testFeatures) {
			for (const CFeature* feature : quad.features) {
				if (!feature->HasCollidableStateBit(CSolidObject::CSTATE_BIT_QUADMAPRAYS))
					continue;
				if (!testedFeatureIDs.insert(feature->id).second)
					continue;
				if (!NativeTraceFeatureVisible(feature))
					continue;

				CollisionQuery collisionQuery;
				if (!CCollisionHandler::DetectHit(feature, feature->GetTransformMatrix(true), pos, pos + dir * maxLength, &collisionQuery, true))
					continue;

				const float hitLength = collisionQuery.GetHitPosDist(pos, dir);
				if (hitLength <= maxLength)
					foundHits.emplace_back(hitLength, feature->id, 2);
			}
		}
	}

	std::stable_sort(foundHits.begin(), foundHits.end(), [](const auto& lhs, const auto& rhs) {
		return std::get<0>(lhs) < std::get<0>(rhs);
	});

	traceHitStorage.reserve(foundHits.size());
	for (const auto& [hitLength, objectID, objectType] : foundHits) {
		traceHitStorage.push_back({
			.hitLength = hitLength,
			.objectID = objectID,
			.objectType = objectType,
		});
	}

	hits = traceHitStorage.empty() ? nullptr : traceHitStorage.data();
	count = static_cast<uint32_t>(traceHitStorage.size());
}

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

static void NativeTraceRayInDirection(const TraceRayInDirectionQuery* query, TraceRayInDirectionResult* result)
{
	bufferPos = 0;

	if (query == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		result->hits = nullptr;
		result->count = 0;
		return;
	}

	const float3 pos(query->pos.x, query->pos.y, query->pos.z);
	const float3 dir(query->dir.x, query->dir.y, query->dir.z);
	const float maxLength = query->hasMaxLength ? query->maxLength : 999999.0f;

	NativeTraceRayAllHits(pos, dir, maxLength, query->type, result->error, result->hits, result->count);
}

static void NativeTraceRayBetweenPositions(const TraceRayBetweenPositionsQuery* query, TraceRayBetweenPositionsResult* result)
{
	bufferPos = 0;

	if (query == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		result->hits = nullptr;
		result->count = 0;
		return;
	}

	const float3 start(query->start.x, query->start.y, query->start.z);
	const float3 end(query->end.x, query->end.y, query->end.z);
	const auto [dir, length] = (end - start).GetNormalized();

	NativeTraceRayAllHits(start, dir, length, query->type, result->error, result->hits, result->count);
}

// Ground trace between positions
static void NativeTraceRayGroundBetweenPositions(const TraceRayGroundBetweenPositionsQuery* query, TraceRayGroundBetweenPositionsResult* result)
{
	bufferPos = 0;

	result->error = nullptr;
	result->hit = false;
	result->hitLength = -1.0f;
	result->hitPos = {};
	result->hitNormal = {};

	if (query == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const float3 start(query->start.x, query->start.y, query->start.z);
	const float3 end(query->end.x, query->end.y, query->end.z);
	const auto [dir, length] = (end - start).GetNormalized();
	const bool testWater = query->hasTestWater ? query->testWater : true;
	const float hitLength = CGround::LineGroundWaterCol(start, dir, length, testWater, NativeTraceIsSynced());

	if (hitLength < 0.0f)
		return;

	const float3 hitPos = start + dir * hitLength;
	result->hit = true;
	result->hitLength = hitLength;
	result->hitPos.x = hitPos.x;
	result->hitPos.y = hitPos.y;
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

	result->error = nullptr;
	result->hit = false;
	result->hitLength = -1.0f;
	result->hitPos = {};
	result->hitNormal = {};

	if (query == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const float3 vStart(query->start.x, query->start.y, query->start.z);
	const float3 vDir = float3(query->dir.x, query->dir.y, query->dir.z).SafeNormalize();
	const float maxLength = query->hasLength ? query->length : 999999.0f;
	const bool testWater = query->hasTestWater ? query->testWater : true;
	const float hitLength = CGround::LineGroundWaterCol(vStart, vDir, maxLength, testWater, NativeTraceIsSynced());

	if (hitLength < 0.0f)
		return;

	const float3 hitPos = vStart + (vDir * hitLength);
	result->hit = true;
	result->hitLength = hitLength;
	result->hitPos.x = hitPos.x;
	result->hitPos.y = hitPos.y;
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
	.TraceRayInDirection = NativeTraceRayInDirection,
	.TraceRayBetweenPositions = NativeTraceRayBetweenPositions,
	.TraceRayGroundBetweenPositions = NativeTraceRayGroundBetweenPositions,
	.TraceRayGroundInDirection = NativeTraceRayGroundInDirection,
};
