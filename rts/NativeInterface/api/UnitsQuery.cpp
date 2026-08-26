#include "UnitsQuery.h"
#include "NativeInterface/ResultStorage.h"

#include "NativeInterface/WasmUiVisibility.h"

#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitDef.h"
#include "Sim/Units/UnitDefHandler.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Misc/GlobalSynced.h"
#include "Sim/Misc/TeamHandler.h"
#include "Sim/Misc/QuadField.h"
#include "Game/GameHelper.h"
#include "System/float3.h"
#include "Rendering/Units/UnitDrawer.h"

#include <algorithm>
#include <limits>
#include <vector>

namespace {

// Static errors
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Unit system not ready" };
static const Error INVALID_UNIT_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid unit ID" };
static const Error INVALID_TEAM_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid team ID" };
static const Error INVALID_ALLYTEAM_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid ally team ID" };
static const Error INVALID_DEFS_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Unit definition IDs are required" };
static const Error RESULT_BUFFER_ERROR = { .code = ERROR_BUFFER_OVERFLOW, .message = "Unit query result buffer allocation failed" };

static thread_local NativeResultStorage resultStorage;

template <typename T>
static T* AllocateResult(size_t count, const Error*& error)
{
	T* result = resultStorage.Allocate<T>(count);
	if (count != 0 && result == nullptr)
		error = &RESULT_BUFFER_ERROR;
	return result;
}

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

static bool UnitMatchesAllegiance(const CUnit* unit, int32_t allegiance)
{
	if (unit == nullptr)
		return false;
	if (allegiance >= 0)
		return unit->team == allegiance;
	return true;
}

static bool UnitInPlanes(const CUnit* unit, const PlanesQuery& planes)
{
	if (unit == nullptr)
		return false;

	for (uint32_t i = 0; i < planes.planeCount && i < 6; ++i) {
		const Float4& plane = planes.planes[i];
		const float distance = (unit->pos.x * plane.x) +
			(unit->pos.y * plane.y) +
			(unit->pos.z * plane.z) + plane.w;
		if ((distance - unit->radius) > 0.0f)
			return false;
	}

	return true;
}

// Validation
static void NativeValidUnitID(const ValidUnitIDQuery* query, ValidUnitIDResult* result)
{
	resultStorage.Reset();
	result->error = nullptr;
	result->valid = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}
	result->valid = (WasmUiVisibility::FindUnit(query->unitID,
		WasmUiVisibility::UnitAccess::Visible) != nullptr);
}

// Get all units
static void NativeGetAllUnits(const GetAllUnitsQuery* query, GetAllUnitsResult* result)
{
	resultStorage.Reset();
	result->error = nullptr;
	result->units = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const auto& activeUnits = unitHandler.GetActiveUnits();
	int32_t* units = AllocateResult<int32_t>(activeUnits.size(), result->error);
	if (result->error != nullptr)
		return;
	uint32_t count = 0;
	for (const CUnit* unit : activeUnits) {
		if (unit != nullptr && WasmUiVisibility::UnitPasses(unit,
			WasmUiVisibility::UnitAccess::Visible))
			units[count++] = unit->id;
	}

	result->units = (count == 0) ? nullptr : units;
	result->count = count;
}

// Get units by team
static void NativeGetTeamUnits(const GetTeamUnitsQuery* query, GetTeamUnitsResult* result)
{
	resultStorage.Reset();
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

	const bool allied = !WasmUiVisibility::Active() ||
		(WasmUiVisibility::ReadTeam() >= 0 &&
		teamHandler.AlliedTeams(query->teamID, WasmUiVisibility::ReadTeam()));
	const auto& teamUnits = unitHandler.GetUnitsByTeam(query->teamID);
	int32_t* units = AllocateResult<int32_t>(teamUnits.size(), result->error);
	if (result->error != nullptr)
		return;
	uint32_t count = 0;
	for (const CUnit* unit : teamUnits) {
		if (unit != nullptr && (allied || WasmUiVisibility::IsUnitVisible(unit)))
			units[count++] = unit->id;
	}

	result->units = (count == 0) ? nullptr : units;
	result->count = count;
}

static void NativeGetTeamUnitsSorted(const GetTeamUnitsSortedQuery* query, GetTeamUnitsSortedResult* result)
{
	resultStorage.Reset();
	result->error = nullptr;
	result->groups = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!teamHandler.IsValidTeam(query->teamID)) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}
	const bool allied = !WasmUiVisibility::Active() ||
		(WasmUiVisibility::ReadTeam() >= 0 &&
		teamHandler.AlliedTeams(query->teamID, WasmUiVisibility::ReadTeam()));

	size_t groupCount = 0;
	size_t totalUnitCount = 0;
	for (unsigned int i = 0, n = unitDefHandler->NumUnitDefs(); i < n; ++i) {
		const int unitDefID = i + 1;
		const auto& unitsByDef = unitHandler.GetUnitsByTeamAndDef(query->teamID, unitDefID);
		if (unitsByDef.empty())
			continue;

		size_t visibleCount = 0;
		for (const CUnit* unit : unitsByDef) {
			if (!allied && !WasmUiVisibility::IsUnitVisible(unit))
				continue;
			++visibleCount;
		}
		if (visibleCount == 0)
			continue;

		++groupCount;
		if (visibleCount > std::numeric_limits<size_t>::max() - totalUnitCount) {
			result->error = &RESULT_BUFFER_ERROR;
			return;
		}
		totalUnitCount += visibleCount;
	}

	if (totalUnitCount > std::numeric_limits<size_t>::max() / sizeof(int32_t) ||
		groupCount > (std::numeric_limits<size_t>::max() - totalUnitCount * sizeof(int32_t) - alignof(std::max_align_t)) / sizeof(TeamUnitsByDef) ||
		!resultStorage.ReserveAdditional(groupCount * sizeof(TeamUnitsByDef) + totalUnitCount * sizeof(int32_t) + alignof(std::max_align_t))) {
		result->error = &RESULT_BUFFER_ERROR;
		return;
	}
	TeamUnitsByDef* groups = AllocateResult<TeamUnitsByDef>(groupCount, result->error);
	if (result->error != nullptr)
		return;
	uint32_t groupIndex = 0;
	for (unsigned int i = 0, n = unitDefHandler->NumUnitDefs(); i < n; ++i) {
		const int unitDefID = i + 1;
		const auto& unitsByDef = unitHandler.GetUnitsByTeamAndDef(query->teamID, unitDefID);
		if (unitsByDef.empty())
			continue;

		size_t visibleCount = 0;
		for (const CUnit* unit : unitsByDef) {
			if (!allied && !WasmUiVisibility::IsUnitVisible(unit))
				continue;
			++visibleCount;
		}
		if (visibleCount == 0)
			continue;

		int32_t* units = AllocateResult<int32_t>(visibleCount, result->error);
		if (result->error != nullptr)
			return;
		size_t unitIndex = 0;
		for (const CUnit* unit : unitsByDef) {
			if (!allied && !WasmUiVisibility::IsUnitVisible(unit))
				continue;
			units[unitIndex++] = unit->id;
		}
		groups[groupIndex++] = {
			.unitDefID = unitDefID,
			.units = units,
			.count = static_cast<uint32_t>(visibleCount),
		};
	}

	result->groups = (groupCount == 0) ? nullptr : groups;
	result->count = static_cast<uint32_t>(groupCount);
}

static void NativeGetTeamUnitsCounts(const GetTeamUnitsCountsQuery* query, GetTeamUnitsCountsResult* result)
{
	resultStorage.Reset();
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
	const bool allied = !WasmUiVisibility::Active() ||
		(WasmUiVisibility::ReadTeam() >= 0 &&
		teamHandler.AlliedTeams(query->teamID, WasmUiVisibility::ReadTeam()));

	// Count units by def
	std::unordered_map<int32_t, uint32_t> defCounts;
	for (const CUnit* unit : unitHandler.GetUnitsByTeam(query->teamID)) {
		if (unit != nullptr && (allied || WasmUiVisibility::IsUnitVisible(unit))) {
			defCounts[unit->unitDef->id]++;
		}
	}

	UnitDefCount* counts = AllocateResult<UnitDefCount>(defCounts.size(), result->error);
	if (result->error != nullptr)
		return;
	size_t count = 0;
	for (const auto& [defID, defCount] : defCounts)
		counts[count++] = { .unitDefID = defID, .count = defCount };

	result->counts = (count == 0) ? nullptr : counts;
	result->count = static_cast<uint32_t>(count);
}

static void NativeGetTeamUnitsByDefs(const GetTeamUnitsByDefsQuery* query, GetTeamUnitsByDefsResult* result)
{
	resultStorage.Reset();
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
	const bool allied = !WasmUiVisibility::Active() ||
		(WasmUiVisibility::ReadTeam() >= 0 &&
		teamHandler.AlliedTeams(query->teamID, WasmUiVisibility::ReadTeam()));

	if (query->defCount != 0 && query->unitDefIDs == nullptr) {
		result->error = &INVALID_DEFS_ERROR;
		return;
	}
	std::vector<int32_t> requestedDefs;
	if (query->defCount != 0)
		requestedDefs.assign(query->unitDefIDs, query->unitDefIDs + query->defCount);
	std::sort(requestedDefs.begin(), requestedDefs.end());
	requestedDefs.erase(std::unique(requestedDefs.begin(), requestedDefs.end()), requestedDefs.end());

	const auto& teamUnits = unitHandler.GetUnitsByTeam(query->teamID);
	size_t matchingCount = 0;
	for (const int32_t unitDefID : requestedDefs) {
		for (const CUnit* unit : teamUnits) {
			if (unit != nullptr && (allied || WasmUiVisibility::IsUnitVisible(unit)) &&
				unit->unitDef->id == unitDefID) {
				++matchingCount;
			}
		}
	}

	int32_t* units = AllocateResult<int32_t>(matchingCount, result->error);
	if (result->error != nullptr)
		return;
	uint32_t count = 0;
	for (const int32_t unitDefID : requestedDefs) {
		for (const CUnit* unit : teamUnits) {
			if (unit != nullptr && (allied || WasmUiVisibility::IsUnitVisible(unit)) &&
				unit->unitDef->id == unitDefID) {
				units[count++] = unit->id;
			}
		}
	}

	result->units = (count == 0) ? nullptr : units;
	result->count = count;
}

static void NativeGetTeamUnitDefCount(const GetTeamUnitDefCountQuery* query, GetTeamUnitDefCountResult* result)
{
	resultStorage.Reset();
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
	const bool allied = !WasmUiVisibility::Active() ||
		(WasmUiVisibility::ReadTeam() >= 0 &&
		teamHandler.AlliedTeams(query->teamID, WasmUiVisibility::ReadTeam()));

	uint32_t count = 0;
	for (const CUnit* unit : unitHandler.GetUnitsByTeam(query->teamID)) {
		if (unit != nullptr && (allied || WasmUiVisibility::IsUnitTyped(unit)) &&
			unit->unitDef->id == query->unitDefID) {
			count++;
		}
	}

	result->count = count;
}

static void NativeGetTeamUnitCount(const GetTeamUnitCountQuery* query, GetTeamUnitCountResult* result)
{
	resultStorage.Reset();
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
	const bool allied = !WasmUiVisibility::Active() ||
		(WasmUiVisibility::ReadTeam() >= 0 &&
		teamHandler.AlliedTeams(query->teamID, WasmUiVisibility::ReadTeam()));

	result->count = allied ? unitHandler.NumUnitsByTeam(query->teamID) : std::count_if(
		unitHandler.GetUnitsByTeam(query->teamID).begin(),
		unitHandler.GetUnitsByTeam(query->teamID).end(), [](const CUnit* unit) {
			return WasmUiVisibility::IsUnitVisible(unit);
		});
}

// Spatial queries
static void NativeGetUnitsInRectangle(const GetUnitsInRectangleQuery* query, GetUnitsInRectangleResult* result)
{
	resultStorage.Reset();
	result->error = nullptr;
	result->units = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const float3 mins(query->xmin, 0.0f, query->zmin);
	const float3 maxs(query->xmax, 0.0f, query->zmax);

	QuadFieldQuery qfq;
	quadField.GetUnitsExact(qfq, mins, maxs);
	const size_t capacity = (qfq.units == nullptr) ? 0 : qfq.units->size();
	int32_t* units = AllocateResult<int32_t>(capacity, result->error);
	if (result->error != nullptr)
		return;
	uint32_t count = 0;
	if (qfq.units != nullptr) {
		for (const CUnit* unit : *(qfq.units)) {
			if (UnitMatchesAllegiance(unit, query->allegiance) &&
				WasmUiVisibility::UnitPasses(unit, WasmUiVisibility::UnitAccess::Visible))
				units[count++] = unit->id;
		}
	}

	result->units = (count == 0) ? nullptr : units;
	result->count = count;
}

static void NativeGetUnitsInBox(const GetUnitsInBoxQuery* query, GetUnitsInBoxResult* result)
{
	resultStorage.Reset();
	result->error = nullptr;
	result->units = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const float3 mins(query->xmin, query->ymin, query->zmin);
	const float3 maxs(query->xmax, query->ymax, query->zmax);

	QuadFieldQuery qfq;
	quadField.GetUnitsExact(qfq, mins, maxs);
	const size_t capacity = (qfq.units == nullptr) ? 0 : qfq.units->size();
	int32_t* units = AllocateResult<int32_t>(capacity, result->error);
	if (result->error != nullptr)
		return;
	uint32_t count = 0;
	if (qfq.units != nullptr) {
		for (const CUnit* unit : *(qfq.units)) {
			if (UnitMatchesAllegiance(unit, query->allegiance) &&
				WasmUiVisibility::UnitPasses(unit, WasmUiVisibility::UnitAccess::Visible)) {
				const float3& pos = unit->pos;
				if (pos.x >= mins.x && pos.x <= maxs.x &&
					pos.y >= mins.y && pos.y <= maxs.y &&
					pos.z >= mins.z && pos.z <= maxs.z)
					units[count++] = unit->id;
			}
		}
	}

	result->units = (count == 0) ? nullptr : units;
	result->count = count;
}

static void NativeGetUnitsInPlanes(const GetUnitsInPlanesQuery* query, GetUnitsInPlanesResult* result)
{
	resultStorage.Reset();
	result->error = nullptr;
	result->units = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const auto& activeUnits = unitHandler.GetActiveUnits();
	int32_t* units = AllocateResult<int32_t>(activeUnits.size(), result->error);
	if (result->error != nullptr)
		return;
	uint32_t count = 0;
	for (const CUnit* unit : activeUnits) {
		if (UnitMatchesAllegiance(unit, query->allegiance) &&
			WasmUiVisibility::UnitPasses(unit, WasmUiVisibility::UnitAccess::Visible) &&
			UnitInPlanes(unit, query->planes))
			units[count++] = unit->id;
	}

	result->units = (count == 0) ? nullptr : units;
	result->count = count;
}

static void NativeGetUnitsInSphere(const GetUnitsInSphereQuery* query, GetUnitsInSphereResult* result)
{
	resultStorage.Reset();
	result->error = nullptr;
	result->units = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const float3 center(query->x, query->y, query->z);
	const float radiusSq = query->radius * query->radius;

	QuadFieldQuery qfq;
	quadField.GetUnitsExact(qfq, center, query->radius);
	const size_t capacity = (qfq.units == nullptr) ? 0 : qfq.units->size();
	int32_t* units = AllocateResult<int32_t>(capacity, result->error);
	if (result->error != nullptr)
		return;
	uint32_t count = 0;
	if (qfq.units != nullptr) {
		for (const CUnit* unit : *(qfq.units)) {
			if (UnitMatchesAllegiance(unit, query->allegiance) &&
				WasmUiVisibility::UnitPasses(unit, WasmUiVisibility::UnitAccess::Visible)) {
				const float distSq = unit->pos.SqDistance(center);
				if (distSq <= radiusSq)
					units[count++] = unit->id;
			}
		}
	}

	result->units = (count == 0) ? nullptr : units;
	result->count = count;
}

static void NativeGetUnitsInCylinder(const GetUnitsInCylinderQuery* query, GetUnitsInCylinderResult* result)
{
	resultStorage.Reset();
	result->error = nullptr;
	result->units = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const float3 center(query->x, 0.0f, query->z);
	const float radiusSq = query->radius * query->radius;

	QuadFieldQuery qfq;
	quadField.GetUnitsExact(qfq, center, query->radius);
	const size_t capacity = (qfq.units == nullptr) ? 0 : qfq.units->size();
	int32_t* units = AllocateResult<int32_t>(capacity, result->error);
	if (result->error != nullptr)
		return;
	uint32_t count = 0;
	if (qfq.units != nullptr) {
		for (const CUnit* unit : *(qfq.units)) {
			if (UnitMatchesAllegiance(unit, query->allegiance) &&
				WasmUiVisibility::UnitPasses(unit, WasmUiVisibility::UnitAccess::Visible)) {
				const float3& pos = unit->pos;
				const float dx = pos.x - center.x;
				const float dz = pos.z - center.z;
				const float distXZSq = dx * dx + dz * dz;

				if (distXZSq <= radiusSq)
					units[count++] = unit->id;
			}
		}
	}

	result->units = (count == 0) ? nullptr : units;
	result->count = count;
}

// Centroid calculations
static void NativeGetUnitArrayCentroid(const GetUnitArrayCentroidQuery* query, GetUnitArrayCentroidResult* result)
{
	resultStorage.Reset();
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
		const CUnit* unit = WasmUiVisibility::FindUnit(query->unitIDs[i], WasmUiVisibility::UnitAccess::Visible);
		if (unit != nullptr) {
			centroid += unit->midPos;
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
	resultStorage.Reset();
	result->error = nullptr;
	result->unitID = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = WasmUiVisibility::FindUnit(query->unitID, WasmUiVisibility::UnitAccess::Ally);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const CUnit* target = CGameHelper::GetClosestFriendlyUnit(unit, unit->pos, query->range, unit->allyteam);
	if (target != nullptr && WasmUiVisibility::IsUnitVisible(target)) {
		result->unitID = target->id;
	}
}

static void NativeGetUnitNearestEnemy(const GetUnitNearestEnemyQuery* query, GetUnitNearestEnemyResult* result)
{
	resultStorage.Reset();
	result->error = nullptr;
	result->unitID = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = WasmUiVisibility::FindUnit(query->unitID, WasmUiVisibility::UnitAccess::Ally);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const CUnit* target = query->options.useLOS
		? CGameHelper::GetClosestEnemyUnit(unit, unit->pos, query->range, unit->allyteam)
		: CGameHelper::GetClosestEnemyUnitNoLosTest(unit, unit->pos, query->range, unit->allyteam, query->options.sphereDistTest, query->options.checkSightDist);
	if (target != nullptr && WasmUiVisibility::IsUnitVisible(target)) {
		result->unitID = target->id;
	}
}

static void NativeGetClosestEnemyUnit(const GetClosestEnemyUnitQuery* query, GetClosestEnemyUnitResult* result)
{
	resultStorage.Reset();
	result->error = nullptr;
	result->unitID = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!teamHandler.IsValidAllyTeam(query->allyTeamID)) {
		result->error = &INVALID_ALLYTEAM_ERROR;
		return;
	}
	if (!WasmUiVisibility::IsLosPerspectiveAllowed(query->allyTeamID)) {
		result->error = &INVALID_ALLYTEAM_ERROR;
		return;
	}

	const float3 pos(query->pos.x, query->pos.y, query->pos.z);
	const CUnit* unit =
		query->options.useLOS
			? CGameHelper::GetClosestEnemyUnit(nullptr, pos, query->range, query->allyTeamID)
			: CGameHelper::GetClosestEnemyUnitNoLosTest(nullptr, pos, query->range, query->allyTeamID, query->options.sphereDistTest, query->options.checkSightDist);

	if (unit != nullptr && WasmUiVisibility::IsUnitVisible(unit))
		result->unitID = unit->id;
}

// Separation
static void NativeGetUnitSeparation(const GetUnitSeparationQuery* query, GetUnitSeparationResult* result)
{
	resultStorage.Reset();
	result->error = nullptr;
	result->separation = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit1 = WasmUiVisibility::FindUnit(query->unitID1, WasmUiVisibility::UnitAccess::Visible);
	const CUnit* unit2 = WasmUiVisibility::FindUnit(query->unitID2, WasmUiVisibility::UnitAccess::Visible);

	if (unit1 == nullptr || unit2 == nullptr ||
		!WasmUiVisibility::IsUnitInLos(unit1) || !WasmUiVisibility::IsUnitInLos(unit2)) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (query->options.positional) {
		result->separation = unit1->pos.distance(unit2->pos);
	} else {
		// Collision volume based distance
		const float radSum = unit1->radius + unit2->radius;
		const float dist = unit1->pos.distance(unit2->pos);
		result->separation = std::max(0.0f, dist - radSum);
	}
}

static void NativeGetRenderUnits(const GetRenderUnitsQuery* query, GetRenderUnitsResult* result)
{
	resultStorage.Reset();
	result->error = nullptr;
	result->units = nullptr;
	result->count = 0;

	if (!IsReady() || unitDrawer == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const auto& renderUnits = unitDrawer->GetUnsortedUnits();
	if (renderUnits.empty())
		return;

	int32_t* units = AllocateResult<int32_t>(renderUnits.size(), result->error);
	if (result->error != nullptr)
		return;
	uint32_t count = 0;
	for (const CUnit* unit : renderUnits) {
		if (!WasmUiVisibility::IsUnitVisible(unit) || (unit->drawFlag & query->drawMask) == 0)
			continue;

		units[count++] = unit->id;
	}

	result->units = (count == 0) ? nullptr : units;
	(void)query->sendMask;
	result->count = count;
}

static void NativeGetRenderUnitsDrawFlagChanged(const GetRenderUnitsDrawFlagChangedQuery* query, GetRenderUnitsDrawFlagChangedResult* result)
{
	resultStorage.Reset();
	result->error = nullptr;
	result->units = nullptr;
	result->count = 0;

	if (!IsReady() || unitDrawer == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const auto& renderUnits = unitDrawer->GetUnsortedUnits();
	int32_t* units = AllocateResult<int32_t>(renderUnits.size(), result->error);
	if (result->error != nullptr)
		return;
	uint32_t count = 0;
	for (const CUnit* u : renderUnits) {
		if (!WasmUiVisibility::IsUnitVisible(u) || u->previousDrawFlag == u->drawFlag)
			continue;

		units[count++] = u->id;
	}

	result->units = (count == 0) ? nullptr : units;
	(void)query->sendMask;
	result->count = count;
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
	.GetClosestEnemyUnit = NativeGetClosestEnemyUnit,

	.GetUnitSeparation = NativeGetUnitSeparation,
	.GetRenderUnits = NativeGetRenderUnits,
	.GetRenderUnitsDrawFlagChanged = NativeGetRenderUnitsDrawFlagChanged,
};
