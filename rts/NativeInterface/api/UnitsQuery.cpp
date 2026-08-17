#include "UnitsQuery.h"

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

namespace {

// Scratch buffer
static thread_local char scratchBuffer[1024];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Unit system not ready" };
static const Error INVALID_UNIT_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid unit ID" };
static const Error INVALID_TEAM_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid team ID" };
static const Error INVALID_ALLYTEAM_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid ally team ID" };
static const Error BUFFER_OVERFLOW_ERROR = { .code = ERROR_BUFFER_OVERFLOW, .message = "Buffer overflow" };

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
	bufferPos = 0;
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
		if (unit != nullptr && WasmUiVisibility::UnitPasses(unit,
			WasmUiVisibility::UnitAccess::Visible) && count < maxUnits) {
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

	const bool allied = !WasmUiVisibility::Active() ||
		(WasmUiVisibility::ReadTeam() >= 0 &&
		teamHandler.AlliedTeams(query->teamID, WasmUiVisibility::ReadTeam()));
	for (const CUnit* unit : unitHandler.GetUnitsByTeam(query->teamID)) {
		if (unit != nullptr && (allied || WasmUiVisibility::IsUnitVisible(unit)) && count < maxUnits) {
			units[count++] = unit->id;
		}
	}

	result->units = units;
	result->count = count;
	bufferPos += count * sizeof(int32_t);
}

static void NativeGetTeamUnitsSorted(const GetTeamUnitsSortedQuery* query, GetTeamUnitsSortedResult* result)
{
	bufferPos = 0;
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

	size_t groupCapacity = 0;
	for (unsigned int i = 0, n = unitDefHandler->NumUnitDefs(); i < n; ++i) {
		const auto& unitsByDef = unitHandler.GetUnitsByTeamAndDef(query->teamID, i + 1);
		if (!unitsByDef.empty() && (!WasmUiVisibility::Active() || allied ||
			std::any_of(unitsByDef.begin(), unitsByDef.end(), [](const CUnit* unit) {
				return WasmUiVisibility::IsUnitVisible(unit);
			})))
			groupCapacity++;
	}

	if (groupCapacity == 0)
		return;

	const size_t groupBytes = groupCapacity * sizeof(TeamUnitsByDef);
	if (groupBytes > sizeof(scratchBuffer)) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	TeamUnitsByDef* groups = reinterpret_cast<TeamUnitsByDef*>(scratchBuffer);
	bufferPos = groupBytes;

	for (unsigned int i = 0, n = unitDefHandler->NumUnitDefs(); i < n; ++i) {
		const int unitDefID = i + 1;
		const auto& unitsByDef = unitHandler.GetUnitsByTeamAndDef(query->teamID, unitDefID);
		if (unitsByDef.empty())
			continue;

		if (result->count >= groupCapacity) {
			result->error = &BUFFER_OVERFLOW_ERROR;
			return;
		}

		const size_t visibleCount = allied ? unitsByDef.size() : std::count_if(
			unitsByDef.begin(), unitsByDef.end(), [](const CUnit* unit) {
				return WasmUiVisibility::IsUnitVisible(unit);
			});
		if (visibleCount == 0)
			continue;
		const size_t bytes = visibleCount * sizeof(int32_t);
		if (bufferPos + bytes > sizeof(scratchBuffer)) {
			result->error = &BUFFER_OVERFLOW_ERROR;
			return;
		}

		int32_t* units = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
		uint32_t unitCount = 0;
		for (const CUnit* unit: unitsByDef) {
			if (!allied && !WasmUiVisibility::IsUnitVisible(unit))
				continue;
			units[unitCount++] = unit->id;
		}
		bufferPos += bytes;

		TeamUnitsByDef& group = groups[result->count++];
		group.unitDefID = unitDefID;
		group.units = units;
		group.count = unitCount;
	}

	result->groups = groups;
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
	const bool allied = !WasmUiVisibility::Active() ||
		(WasmUiVisibility::ReadTeam() >= 0 &&
		teamHandler.AlliedTeams(query->teamID, WasmUiVisibility::ReadTeam()));

	std::vector<int32_t> requestedDefs(query->unitDefIDs, query->unitDefIDs + query->defCount);
	std::sort(requestedDefs.begin(), requestedDefs.end());
	requestedDefs.erase(std::unique(requestedDefs.begin(), requestedDefs.end()), requestedDefs.end());

	// Use scratch buffer for array
	int32_t* units = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t count = 0;
	const size_t maxUnits = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);

	for (const int32_t unitDefID: requestedDefs) {
		for (const CUnit* unit : unitHandler.GetUnitsByTeam(query->teamID)) {
			if (unit != nullptr && (allied || WasmUiVisibility::IsUnitVisible(unit)) &&
				unit->unitDef->id == unitDefID && count < maxUnits) {
				units[count++] = unit->id;
			}
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
	bufferPos = 0;
	result->error = nullptr;
	result->units = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const float3 mins(query->xmin, 0.0f, query->zmin);
	const float3 maxs(query->xmax, 0.0f, query->zmax);

	// Use scratch buffer for array
	int32_t* units = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t count = 0;
	const size_t maxUnits = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);

	QuadFieldQuery qfq;
	quadField.GetUnitsExact(qfq, mins, maxs);
	if (qfq.units != nullptr) {
		for (const CUnit* unit : *(qfq.units)) {
			if (UnitMatchesAllegiance(unit, query->allegiance) &&
				WasmUiVisibility::UnitPasses(unit, WasmUiVisibility::UnitAccess::Visible) &&
				count < maxUnits) {
				units[count++] = unit->id;
			}
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

	const float3 mins(query->xmin, query->ymin, query->zmin);
	const float3 maxs(query->xmax, query->ymax, query->zmax);

	// Use scratch buffer for array
	int32_t* units = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t count = 0;
	const size_t maxUnits = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);

	QuadFieldQuery qfq;
	quadField.GetUnitsExact(qfq, mins, maxs);
	if (qfq.units != nullptr) {
		for (const CUnit* unit : *(qfq.units)) {
			if (UnitMatchesAllegiance(unit, query->allegiance) &&
				WasmUiVisibility::UnitPasses(unit, WasmUiVisibility::UnitAccess::Visible)) {
				const float3& pos = unit->pos;
				if (pos.x >= mins.x && pos.x <= maxs.x &&
					pos.y >= mins.y && pos.y <= maxs.y &&
					pos.z >= mins.z && pos.z <= maxs.z &&
					count < maxUnits) {
					units[count++] = unit->id;
				}
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

	for (const CUnit* unit : unitHandler.GetActiveUnits()) {
		if (UnitMatchesAllegiance(unit, query->allegiance) &&
			WasmUiVisibility::UnitPasses(unit, WasmUiVisibility::UnitAccess::Visible) &&
			UnitInPlanes(unit, query->planes) && count < maxUnits) {
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

	const float3 center(query->x, query->y, query->z);
	const float radiusSq = query->radius * query->radius;

	// Use scratch buffer for array
	int32_t* units = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t count = 0;
	const size_t maxUnits = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);

	QuadFieldQuery qfq;
	quadField.GetUnitsExact(qfq, center, query->radius);
	if (qfq.units != nullptr) {
		for (const CUnit* unit : *(qfq.units)) {
			if (UnitMatchesAllegiance(unit, query->allegiance) &&
				WasmUiVisibility::UnitPasses(unit, WasmUiVisibility::UnitAccess::Visible)) {
				const float distSq = unit->pos.SqDistance(center);
				if (distSq <= radiusSq && count < maxUnits) {
					units[count++] = unit->id;
				}
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

	const float3 center(query->x, 0.0f, query->z);
	const float radiusSq = query->radius * query->radius;

	// Use scratch buffer for array
	int32_t* units = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t count = 0;
	const size_t maxUnits = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);

	QuadFieldQuery qfq;
	quadField.GetUnitsExact(qfq, center, query->radius);
	if (qfq.units != nullptr) {
		for (const CUnit* unit : *(qfq.units)) {
			if (UnitMatchesAllegiance(unit, query->allegiance) &&
				WasmUiVisibility::UnitPasses(unit, WasmUiVisibility::UnitAccess::Visible)) {
				const float3& pos = unit->pos;
				const float dx = pos.x - center.x;
				const float dz = pos.z - center.z;
				const float distXZSq = dx * dx + dz * dz;

				if (distXZSq <= radiusSq && count < maxUnits) {
					units[count++] = unit->id;
				}
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
	bufferPos = 0;
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
	bufferPos = 0;
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
	bufferPos = 0;
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
	bufferPos = 0;
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
	bufferPos = 0;
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

	int32_t* out = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t count = 0;
	const size_t maxCount = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);

	for (const CUnit* unit : renderUnits) {
		if (count >= maxCount)
			break;
		if (!WasmUiVisibility::IsUnitVisible(unit) || (unit->drawFlag & query->drawMask) == 0)
			continue;

		out[count++] = unit->id;
	}

	result->units = out;
	bufferPos += count * sizeof(int32_t);
	(void)query->sendMask;
	result->count = count;
}

static void NativeGetRenderUnitsDrawFlagChanged(const GetRenderUnitsDrawFlagChangedQuery* query, GetRenderUnitsDrawFlagChangedResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->units = nullptr;
	result->count = 0;

	if (!IsReady() || unitDrawer == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const auto& renderUnits = unitDrawer->GetUnsortedUnits();
	int32_t* out = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t count = 0;
	const size_t maxCount = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);

	for (const CUnit* u : renderUnits) {
		if (count >= maxCount)
			break;

		if (!WasmUiVisibility::IsUnitVisible(u) || u->previousDrawFlag == u->drawFlag)
			continue;

		out[count++] = u->id;
	}

	result->units = out;
	bufferPos += count * sizeof(int32_t);
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
