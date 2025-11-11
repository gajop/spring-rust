#include "UnitsQuery.h"

#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Misc/GlobalSynced.h"
#include "Sim/Misc/TeamHandler.h"
#include "System/float3.h"

namespace {

// Error constants
static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Unit system not ready"
};

static const Error INVALID_UNIT_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid unit ID"
};

static const Error INVALID_TEAM_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid team ID"
};

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
static BoolResult NativeValidUnitID(int32_t unitID)
{
	BoolResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}
	result.value = unitHandler.IsValidUnit(unitID);
	return result;
}

// Get all units
static Int32Array NativeGetAllUnits()
{
	Int32Array result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Use static storage - valid for call duration only
	static thread_local std::vector<int32_t> units;
	units.clear();

	for (const CUnit* unit : unitHandler.GetActiveUnits()) {
		if (unit != nullptr) {
			units.push_back(unit->id);
		}
	}

	result.data = units.data();
	result.length = static_cast<uint32_t>(units.size());
	return result;
}

// Get units by team
static Int32Array NativeGetTeamUnits(int32_t teamID)
{
	Int32Array result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidTeam(teamID)) {
		result.error = &INVALID_TEAM_ERROR;
		return result;
	}

	// Use static storage - valid for call duration only
	static thread_local std::vector<int32_t> units;
	units.clear();

	for (const CUnit* unit : unitHandler.GetUnitsByTeam(teamID)) {
		if (unit != nullptr) {
			units.push_back(unit->id);
		}
	}

	result.data = units.data();
	result.length = static_cast<uint32_t>(units.size());
	return result;
}

static Int32Array NativeGetTeamUnitsSorted(int32_t teamID)
{
	// For now, return same as GetTeamUnits
	// Full sorting by def would require more complex structure
	return NativeGetTeamUnits(teamID);
}

static UnitDefCountsResult NativeGetTeamUnitsCounts(int32_t teamID)
{
	UnitDefCountsResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidTeam(teamID)) {
		result.error = &INVALID_TEAM_ERROR;
		return result;
	}

	// Use static storage - valid for call duration only
	static thread_local std::vector<UnitDefCount> counts;
	counts.clear();

	// Count units by def
	std::unordered_map<int32_t, uint32_t> defCounts;
	for (const CUnit* unit : unitHandler.GetUnitsByTeam(teamID)) {
		if (unit != nullptr) {
			defCounts[unit->unitDef->id]++;
		}
	}

	for (const auto& [defID, count] : defCounts) {
		UnitDefCount udc;
		udc.unitDefID = defID;
		udc.count = count;
		counts.push_back(udc);
	}

	result.counts = counts.data();
	result.countCount = static_cast<uint32_t>(counts.size());
	return result;
}

static Int32Array NativeGetTeamUnitsByDefs(int32_t teamID, const int32_t* unitDefIDs, uint32_t count)
{
	Int32Array result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidTeam(teamID)) {
		result.error = &INVALID_TEAM_ERROR;
		return result;
	}

	// Build set of requested defs
	std::unordered_set<int32_t> requestedDefs(unitDefIDs, unitDefIDs + count);

	// Use static storage - valid for call duration only
	static thread_local std::vector<int32_t> units;
	units.clear();

	for (const CUnit* unit : unitHandler.GetUnitsByTeam(teamID)) {
		if (unit != nullptr && requestedDefs.count(unit->unitDef->id) > 0) {
			units.push_back(unit->id);
		}
	}

	result.data = units.data();
	result.length = static_cast<uint32_t>(units.size());
	return result;
}

static UInt32Result NativeGetTeamUnitDefCount(int32_t teamID, int32_t unitDefID)
{
	UInt32Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidTeam(teamID)) {
		result.error = &INVALID_TEAM_ERROR;
		return result;
	}

	uint32_t count = 0;
	for (const CUnit* unit : unitHandler.GetUnitsByTeam(teamID)) {
		if (unit != nullptr && unit->unitDef->id == unitDefID) {
			count++;
		}
	}

	result.value = count;
	return result;
}

static UInt32Result NativeGetTeamUnitCount(int32_t teamID)
{
	UInt32Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidTeam(teamID)) {
		result.error = &INVALID_TEAM_ERROR;
		return result;
	}

	result.value = unitHandler.NumUnitsByTeam(teamID);
	return result;
}

// Spatial queries
static Int32Array NativeGetUnitsInRectangle(RectangleQuery query, UnitFilterParams filter)
{
	Int32Array result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Use static storage - valid for call duration only
	static thread_local std::vector<int32_t> units;
	units.clear();

	const float3 mins(query.minX, 0.0f, query.minZ);
	const float3 maxs(query.maxX, 0.0f, query.maxZ);

	const auto& foundUnits = quadField.GetUnitsExact(mins, maxs);
	for (const CUnit* unit : foundUnits) {
		if (UnitMatchesFilter(unit, filter)) {
			units.push_back(unit->id);
		}
	}

	result.data = units.data();
	result.length = static_cast<uint32_t>(units.size());
	return result;
}

static Int32Array NativeGetUnitsInBox(BoxQuery query, UnitFilterParams filter)
{
	Int32Array result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Use static storage - valid for call duration only
	static thread_local std::vector<int32_t> units;
	units.clear();

	const float3 mins(query.min.x, query.min.y, query.min.z);
	const float3 maxs(query.max.x, query.max.y, query.max.z);

	const auto& foundUnits = quadField.GetUnitsExact(mins, maxs);
	for (const CUnit* unit : foundUnits) {
		if (UnitMatchesFilter(unit, filter)) {
			const float3& pos = unit->pos;
			if (pos.x >= mins.x && pos.x <= maxs.x &&
				pos.y >= mins.y && pos.y <= maxs.y &&
				pos.z >= mins.z && pos.z <= maxs.z) {
				units.push_back(unit->id);
			}
		}
	}

	result.data = units.data();
	result.length = static_cast<uint32_t>(units.size());
	return result;
}

static Int32Array NativeGetUnitsInPlanes(PlanesQuery query, UnitFilterParams filter)
{
	Int32Array result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Use static storage - valid for call duration only
	static thread_local std::vector<int32_t> units;
	units.clear();

	// Simplified - would need proper frustum culling
	for (const CUnit* unit : unitHandler.GetActiveUnits()) {
		if (UnitMatchesFilter(unit, filter)) {
			units.push_back(unit->id);
		}
	}

	result.data = units.data();
	result.length = static_cast<uint32_t>(units.size());
	return result;
}

static Int32Array NativeGetUnitsInSphere(SphereQuery query, UnitFilterParams filter)
{
	Int32Array result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Use static storage - valid for call duration only
	static thread_local std::vector<int32_t> units;
	units.clear();

	const float3 center(query.center.x, query.center.y, query.center.z);
	const float radiusSq = query.radius * query.radius;

	const auto& foundUnits = quadField.GetUnitsExact(center, query.radius);
	for (const CUnit* unit : foundUnits) {
		if (UnitMatchesFilter(unit, filter)) {
			const float distSq = unit->pos.SqDistance(center);
			if (distSq <= radiusSq) {
				units.push_back(unit->id);
			}
		}
	}

	result.data = units.data();
	result.length = static_cast<uint32_t>(units.size());
	return result;
}

static Int32Array NativeGetUnitsInCylinder(CylinderQuery query, UnitFilterParams filter)
{
	Int32Array result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Use static storage - valid for call duration only
	static thread_local std::vector<int32_t> units;
	units.clear();

	const float3 center(query.center.x, query.center.y, query.center.z);
	const float radiusSq = query.radius * query.radius;
	const float halfHeight = query.height * 0.5f;

	const auto& foundUnits = quadField.GetUnitsExact(center, query.radius);
	for (const CUnit* unit : foundUnits) {
		if (UnitMatchesFilter(unit, filter)) {
			const float3& pos = unit->pos;
			const float dx = pos.x - center.x;
			const float dz = pos.z - center.z;
			const float distXZSq = dx * dx + dz * dz;
			const float dy = std::abs(pos.y - center.y);

			if (distXZSq <= radiusSq && dy <= halfHeight) {
				units.push_back(unit->id);
			}
		}
	}

	result.data = units.data();
	result.length = static_cast<uint32_t>(units.size());
	return result;
}

// Centroid calculations
static Float3Result NativeGetUnitArrayCentroid(const int32_t* unitIDs, uint32_t count)
{
	Float3Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (count == 0 || unitIDs == nullptr) {
		result.value.x = 0.0f;
		result.value.y = 0.0f;
		result.value.z = 0.0f;
		return result;
	}

	float3 centroid(0.0f, 0.0f, 0.0f);
	uint32_t validCount = 0;

	for (uint32_t i = 0; i < count; i++) {
		const CUnit* unit = unitHandler.GetUnit(unitIDs[i]);
		if (unit != nullptr) {
			centroid += unit->pos;
			validCount++;
		}
	}

	if (validCount > 0) {
		centroid /= static_cast<float>(validCount);
	}

	result.value.x = centroid.x;
	result.value.y = centroid.y;
	result.value.z = centroid.z;
	return result;
}

static Float3Result NativeGetUnitMapCentroid(const int32_t* unitIDs, uint32_t count)
{
	// Same as array centroid for now
	return NativeGetUnitArrayCentroid(unitIDs, count);
}

// Nearest unit
static Int32Result NativeGetUnitNearestAlly(Float3 pos, float radius)
{
	Int32Result result = {};
	result.value = -1; // No unit found

	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const float3 position(pos.x, pos.y, pos.z);
	float minDistSq = radius * radius;

	const auto& foundUnits = quadField.GetUnitsExact(position, radius);
	for (const CUnit* unit : foundUnits) {
		if (unit != nullptr) {
			const float distSq = unit->pos.SqDistance(position);
			if (distSq < minDistSq) {
				minDistSq = distSq;
				result.value = unit->id;
			}
		}
	}

	return result;
}

static Int32Result NativeGetUnitNearestEnemy(Float3 pos, float radius)
{
	// Same as ally for now - would need ally/enemy filtering
	return NativeGetUnitNearestAlly(pos, radius);
}

// Separation
static FloatResult NativeGetUnitSeparation(int32_t unitID1, int32_t unitID2, bool positional, bool checkMap)
{
	FloatResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CUnit* unit1 = unitHandler.GetUnit(unitID1);
	const CUnit* unit2 = unitHandler.GetUnit(unitID2);

	if (unit1 == nullptr || unit2 == nullptr) {
		result.error = &INVALID_UNIT_ERROR;
		return result;
	}

	if (positional) {
		result.value = unit1->pos.distance(unit2->pos);
	} else {
		// Collision volume based distance
		const float radSum = unit1->radius + unit2->radius;
		const float dist = unit1->pos.distance(unit2->pos);
		result.value = std::max(0.0f, dist - radSum);
	}

	return result;
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
