#include "UnitsQuery.h"

#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Misc/GlobalSynced.h"
#include "Sim/Misc/TeamHandler.h"
#include "System/float3.h"

namespace {

// Scratch buffer
static thread_local char scratchBuffer[8192];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Unit system not ready" };
static const Error INVALID_UNIT_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid unit ID" };
static const Error INVALID_TEAM_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid team ID" };

// Helper: check if ready
static bool IsReady()
{
	return (gs != nullptr);
}

// Helper: check if unit matches filter
static bool UnitMatchesFilter(const CUnit* unit, const UnitFilterParams& filter)
{
	if (unit == nullptr) return false;

	switch (filter.filter) {
		case UNIT_FILTER_ALL:
			return true;
		case UNIT_FILTER_TEAM:
			return unit->team == filter.teamID;
		case UNIT_FILTER_ALLYTEAM:
			return teamHandler.AllyTeam(unit->team) == filter.allyTeamID;
		default:
			return true;
	}
}

// Validation
static void NativeValidUnitID(const ValidUnitIDQuery* query, ValidUnitIDResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->valid = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}
	result->valid = unitHandler.IsValidUnit(query->unitID);
}

// Get all units
static void NativeGetAllUnits(const GetAllUnitsQuery* query, GetAllUnitsResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->units = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	// Use scratch buffer for array
	int32_t* units = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t count = 0;
	const size_t maxUnits = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);

	for (const CUnit* unit : unitHandler.GetActiveUnits()) {
		if (unit != nullptr && count < maxUnits) {
			units[count++] = unit->id;
		}
	}

	result->units = units;
	result->count = count;
	bufferPos += count * sizeof(int32_t);
}

// Get units by team
static void NativeGetTeamUnits(const GetTeamUnitsQuery* query, GetTeamUnitsResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->units = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!teamHandler.IsValidTeam(query->teamID)) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	// Use scratch buffer for array
	int32_t* units = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t count = 0;
	const size_t maxUnits = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);

	for (const CUnit* unit : unitHandler.GetUnitsByTeam(query->teamID)) {
		if (unit != nullptr && count < maxUnits) {
			units[count++] = unit->id;
		}
	}

	result->units = units;
	result->count = count;
	bufferPos += count * sizeof(int32_t);
}

static void NativeGetTeamUnitsSorted(const GetTeamUnitsSortedQuery* query, GetTeamUnitsSortedResult* result)
{
	// For now, return same as GetTeamUnits
	// Full sorting by def would require more complex structure
	GetTeamUnitsQuery q = { .teamID = query->teamID };
	GetTeamUnitsResult r;
	NativeGetTeamUnits(&q, &r);
	result->error = r.error;
	result->units = r.units;
	result->count = r.count;
}

static void NativeGetTeamUnitsCounts(const GetTeamUnitsCountsQuery* query, GetTeamUnitsCountsResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->counts = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!teamHandler.IsValidTeam(query->teamID)) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	// Count units by def
	std::unordered_map<int32_t, uint32_t> defCounts;
	for (const CUnit* unit : unitHandler.GetUnitsByTeam(query->teamID)) {
		if (unit != nullptr) {
			defCounts[unit->unitDef->id]++;
		}
	}

	// Use scratch buffer for array
	UnitDefCount* counts = reinterpret_cast<UnitDefCount*>(scratchBuffer + bufferPos);
	uint32_t count = 0;
	const size_t maxCounts = (sizeof(scratchBuffer) - bufferPos) / sizeof(UnitDefCount);

	for (const auto& [defID, defCount] : defCounts) {
		if (count < maxCounts) {
			counts[count].unitDefID = defID;
			counts[count].count = defCount;
			count++;
		}
	}

	result->counts = counts;
	result->count = count;
	bufferPos += count * sizeof(UnitDefCount);
}

static void NativeGetTeamUnitsByDefs(const GetTeamUnitsByDefsQuery* query, GetTeamUnitsByDefsResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->units = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!teamHandler.IsValidTeam(query->teamID)) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	// Build set of requested defs
	std::unordered_set<int32_t> requestedDefs(query->unitDefIDs, query->unitDefIDs + query->defCount);

	// Use scratch buffer for array
	int32_t* units = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t count = 0;
	const size_t maxUnits = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);

	for (const CUnit* unit : unitHandler.GetUnitsByTeam(query->teamID)) {
		if (unit != nullptr && requestedDefs.count(unit->unitDef->id) > 0 && count < maxUnits) {
			units[count++] = unit->id;
		}
	}

	result->units = units;
	result->count = count;
	bufferPos += count * sizeof(int32_t);
}

static void NativeGetTeamUnitDefCount(const GetTeamUnitDefCountQuery* query, GetTeamUnitDefCountResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!teamHandler.IsValidTeam(query->teamID)) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	uint32_t count = 0;
	for (const CUnit* unit : unitHandler.GetUnitsByTeam(query->teamID)) {
		if (unit != nullptr && unit->unitDef->id == query->unitDefID) {
			count++;
		}
	}

	result->count = count;
}

static void NativeGetTeamUnitCount(const GetTeamUnitCountQuery* query, GetTeamUnitCountResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!teamHandler.IsValidTeam(query->teamID)) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	result->count = unitHandler.NumUnitsByTeam(query->teamID);
}

// Spatial queries
static void NativeGetUnitsInRectangle(const GetUnitsInRectangleQuery* query, GetUnitsInRectangleResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->units = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const float3 mins(query->rect.minX, 0.0f, query->rect.minZ);
	const float3 maxs(query->rect.maxX, 0.0f, query->rect.maxZ);

	// Use scratch buffer for array
	int32_t* units = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t count = 0;
	const size_t maxUnits = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);

	const auto& foundUnits = quadField.GetUnitsExact(mins, maxs);
	for (const CUnit* unit : foundUnits) {
		if (UnitMatchesFilter(unit, query->filter) && count < maxUnits) {
			units[count++] = unit->id;
		}
	}

	result->units = units;
	result->count = count;
	bufferPos += count * sizeof(int32_t);
}

static void NativeGetUnitsInBox(const GetUnitsInBoxQuery* query, GetUnitsInBoxResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->units = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const float3 mins(query->box.min.x, query->box.min.y, query->box.min.z);
	const float3 maxs(query->box.max.x, query->box.max.y, query->box.max.z);

	// Use scratch buffer for array
	int32_t* units = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t count = 0;
	const size_t maxUnits = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);

	const auto& foundUnits = quadField.GetUnitsExact(mins, maxs);
	for (const CUnit* unit : foundUnits) {
		if (UnitMatchesFilter(unit, query->filter)) {
			const float3& pos = unit->pos;
			if (pos.x >= mins.x && pos.x <= maxs.x &&
				pos.y >= mins.y && pos.y <= maxs.y &&
				pos.z >= mins.z && pos.z <= maxs.z &&
				count < maxUnits) {
				units[count++] = unit->id;
			}
		}
	}

	result->units = units;
	result->count = count;
	bufferPos += count * sizeof(int32_t);
}

static void NativeGetUnitsInPlanes(const GetUnitsInPlanesQuery* query, GetUnitsInPlanesResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->units = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	// Use scratch buffer for array
	int32_t* units = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t count = 0;
	const size_t maxUnits = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);

	// Simplified - would need proper frustum culling
	for (const CUnit* unit : unitHandler.GetActiveUnits()) {
		if (UnitMatchesFilter(unit, query->filter) && count < maxUnits) {
			units[count++] = unit->id;
		}
	}

	result->units = units;
	result->count = count;
	bufferPos += count * sizeof(int32_t);
}

static void NativeGetUnitsInSphere(const GetUnitsInSphereQuery* query, GetUnitsInSphereResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->units = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const float3 center(query->sphere.center.x, query->sphere.center.y, query->sphere.center.z);
	const float radiusSq = query->sphere.radius * query->sphere.radius;

	// Use scratch buffer for array
	int32_t* units = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t count = 0;
	const size_t maxUnits = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);

	const auto& foundUnits = quadField.GetUnitsExact(center, query->sphere.radius);
	for (const CUnit* unit : foundUnits) {
		if (UnitMatchesFilter(unit, query->filter)) {
			const float distSq = unit->pos.SqDistance(center);
			if (distSq <= radiusSq && count < maxUnits) {
				units[count++] = unit->id;
			}
		}
	}

	result->units = units;
	result->count = count;
	bufferPos += count * sizeof(int32_t);
}

static void NativeGetUnitsInCylinder(const GetUnitsInCylinderQuery* query, GetUnitsInCylinderResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->units = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const float3 center(query->cylinder.center.x, query->cylinder.center.y, query->cylinder.center.z);
	const float radiusSq = query->cylinder.radius * query->cylinder.radius;
	const float halfHeight = query->cylinder.height * 0.5f;

	// Use scratch buffer for array
	int32_t* units = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t count = 0;
	const size_t maxUnits = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);

	const auto& foundUnits = quadField.GetUnitsExact(center, query->cylinder.radius);
	for (const CUnit* unit : foundUnits) {
		if (UnitMatchesFilter(unit, query->filter)) {
			const float3& pos = unit->pos;
			const float dx = pos.x - center.x;
			const float dz = pos.z - center.z;
			const float distXZSq = dx * dx + dz * dz;
			const float dy = std::abs(pos.y - center.y);

			if (distXZSq <= radiusSq && dy <= halfHeight && count < maxUnits) {
				units[count++] = unit->id;
			}
		}
	}

	result->units = units;
	result->count = count;
	bufferPos += count * sizeof(int32_t);
}

// Centroid calculations
static void NativeGetUnitArrayCentroid(const GetUnitArrayCentroidQuery* query, GetUnitArrayCentroidResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->centroid.x = 0.0f;
	result->centroid.y = 0.0f;
	result->centroid.z = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (query->count == 0 || query->unitIDs == nullptr) {
		return;
	}

	float3 centroid(0.0f, 0.0f, 0.0f);
	uint32_t validCount = 0;

	for (uint32_t i = 0; i < query->count; i++) {
		const CUnit* unit = unitHandler.GetUnit(query->unitIDs[i]);
		if (unit != nullptr) {
			centroid += unit->pos;
			validCount++;
		}
	}

	if (validCount > 0) {
		centroid /= static_cast<float>(validCount);
	}

	result->centroid.x = centroid.x;
	result->centroid.y = centroid.y;
	result->centroid.z = centroid.z;
}

static void NativeGetUnitMapCentroid(const GetUnitMapCentroidQuery* query, GetUnitMapCentroidResult* result)
{
	// Same as array centroid for now
	GetUnitArrayCentroidQuery q = { .unitIDs = query->unitIDs, .count = query->count };
	GetUnitArrayCentroidResult r;
	NativeGetUnitArrayCentroid(&q, &r);
	result->error = r.error;
	result->centroid = r.centroid;
}

// Nearest unit
static void NativeGetUnitNearestAlly(const GetUnitNearestAllyQuery* query, GetUnitNearestAllyResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->unitID = -1; // No unit found

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const float3 position(query->pos.x, query->pos.y, query->pos.z);
	float minDistSq = query->radius * query->radius;

	const auto& foundUnits = quadField.GetUnitsExact(position, query->radius);
	for (const CUnit* unit : foundUnits) {
		if (unit != nullptr) {
			const float distSq = unit->pos.SqDistance(position);
			if (distSq < minDistSq) {
				minDistSq = distSq;
				result->unitID = unit->id;
			}
		}
	}
}

static void NativeGetUnitNearestEnemy(const GetUnitNearestEnemyQuery* query, GetUnitNearestEnemyResult* result)
{
	// Same as ally for now - would need ally/enemy filtering
	GetUnitNearestAllyQuery q = { .pos = query->pos, .radius = query->radius };
	GetUnitNearestAllyResult r;
	NativeGetUnitNearestAlly(&q, &r);
	result->error = r.error;
	result->unitID = r.unitID;
}

// Separation
static void NativeGetUnitSeparation(const GetUnitSeparationQuery* query, GetUnitSeparationResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->separation = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit1 = unitHandler.GetUnit(query->unitID1);
	const CUnit* unit2 = unitHandler.GetUnit(query->unitID2);

	if (unit1 == nullptr || unit2 == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (query->positional) {
		result->separation = unit1->pos.distance(unit2->pos);
	} else {
		// Collision volume based distance
		const float radSum = unit1->radius + unit2->radius;
		const float dist = unit1->pos.distance(unit2->pos);
		result->separation = std::max(0.0f, dist - radSum);
	}
}

} // namespace

const UnitsQueryApi UNITS_QUERY_API = {
	.ValidUnitID = NativeValidUnitID,

	.GetAllUnits = NativeGetAllUnits,

	.GetTeamUnits = NativeGetTeamUnits,
	.GetTeamUnitsSorted = NativeGetTeamUnitsSorted,
	.GetTeamUnitsCounts = NativeGetTeamUnitsCounts,
	.GetTeamUnitsByDefs = NativeGetTeamUnitsByDefs,
	.GetTeamUnitDefCount = NativeGetTeamUnitDefCount,
	.GetTeamUnitCount = NativeGetTeamUnitCount,

	.GetUnitsInRectangle = NativeGetUnitsInRectangle,
	.GetUnitsInBox = NativeGetUnitsInBox,
	.GetUnitsInPlanes = NativeGetUnitsInPlanes,
	.GetUnitsInSphere = NativeGetUnitsInSphere,
	.GetUnitsInCylinder = NativeGetUnitsInCylinder,

	.GetUnitArrayCentroid = NativeGetUnitArrayCentroid,
	.GetUnitMapCentroid = NativeGetUnitMapCentroid,

	.GetUnitNearestAlly = NativeGetUnitNearestAlly,
	.GetUnitNearestEnemy = NativeGetUnitNearestEnemy,

	.GetUnitSeparation = NativeGetUnitSeparation,
};
