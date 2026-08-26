#include "Selection.h"
#include "NativeInterface/ResultStorage.h"

#include "Game/SelectedUnitsHandler.h"
#include "Game/UI/Groups/GroupHandler.h"
#include "Game/UI/Groups/Group.h"
#include "Game/GlobalUnsynced.h"
#include "Sim/Misc/GlobalSynced.h"
#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitDef.h"
#include "Sim/Units/UnitHandler.h"
#include <algorithm>
#include <limits>
#include <unordered_map>
#include <vector>

namespace {

static thread_local NativeResultStorage resultStorage;

// Static errors
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Selection system not ready" };
static const Error INVALID_UNIT_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid unit ID" };
static const Error INVALID_GROUP_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid group ID" };
static const Error RESULT_BUFFER_ERROR = { .code = ERROR_BUFFER_OVERFLOW, .message = "Selection result buffer allocation failed" };

static bool IsReady() { return (gu != nullptr); }

static void NativeGetSelectedUnits(const GetSelectedUnitsQuery* query, GetSelectedUnitsResult* result) {
	resultStorage.Reset();
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	const size_t count = selectedUnitsHandler.selectedUnits.size();
	int32_t* units = resultStorage.Allocate<int32_t>(count);
	if (count != 0 && units == nullptr) {
		result->error = &RESULT_BUFFER_ERROR;
		result->units = nullptr;
		result->count = 0;
		return;
	}
	if (count != 0)
		std::copy(selectedUnitsHandler.selectedUnits.begin(), selectedUnitsHandler.selectedUnits.end(), units);

	result->error = nullptr;
	result->units = units;
	result->count = static_cast<uint32_t>(count);
}

static void NativeGetSelectedUnitsSorted(const GetSelectedUnitsSortedQuery* query, GetSelectedUnitsSortedResult* result) {
	resultStorage.Reset();
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	const size_t count = selectedUnitsHandler.selectedUnits.size();
	int32_t* units = resultStorage.Allocate<int32_t>(count);
	if (count != 0 && units == nullptr) {
		result->error = &RESULT_BUFFER_ERROR;
		result->units = nullptr;
		result->count = 0;
		return;
	}
	if (count != 0) {
		std::copy(selectedUnitsHandler.selectedUnits.begin(), selectedUnitsHandler.selectedUnits.end(), units);
		std::sort(units, units + count);
	}

	result->error = nullptr;
	result->units = units;
	result->count = static_cast<uint32_t>(count);
}

static void NativeGetSelectedUnitsCounts(const GetSelectedUnitsCountsQuery* query, GetSelectedUnitsCountsResult* result) {
	resultStorage.Reset();
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	std::unordered_map<int32_t, uint32_t> countMap;

	for (int unitID : selectedUnitsHandler.selectedUnits) {
		const CUnit* unit = unitHandler.GetUnit(unitID);
		if (unit != nullptr) {
			countMap[unit->unitDef->id]++;
		}
	}

	const size_t count = countMap.size();
	constexpr size_t parallelBytes = sizeof(int32_t) + sizeof(uint32_t);
	if (count > (std::numeric_limits<size_t>::max() - alignof(std::max_align_t)) / parallelBytes ||
		!resultStorage.ReserveAdditional(count * parallelBytes + alignof(std::max_align_t))) {
		result->error = &RESULT_BUFFER_ERROR;
		result->counts = {};
		return;
	}
	int32_t* unitDefIDs = resultStorage.Allocate<int32_t>(count);
	uint32_t* counts = resultStorage.Allocate<uint32_t>(count);
	if ((count != 0 && unitDefIDs == nullptr) || (count != 0 && counts == nullptr)) {
		result->error = &RESULT_BUFFER_ERROR;
		result->counts = {};
		return;
	}
	size_t index = 0;
	for (const auto& [defID, count] : countMap) {
		unitDefIDs[index] = defID;
		counts[index] = count;
		++index;
	}

	result->error = nullptr;
	result->counts.unitDefIDs = unitDefIDs;
	result->counts.counts = counts;
	result->counts.uniqueCount = static_cast<uint32_t>(count);
}

static void NativeGetSelectedUnitsCount(const GetSelectedUnitsCountQuery* query, GetSelectedUnitsCountResult* result) {
	resultStorage.Reset();
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	result->error = nullptr;
	result->count = static_cast<uint32_t>(selectedUnitsHandler.selectedUnits.size());
}

static void NativeSelectUnit(const SelectUnitQuery* query, SelectUnitResult* result) {
	resultStorage.Reset();
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) { result->error = &INVALID_UNIT_ERROR; return; }

	if (!query->append) {
		selectedUnitsHandler.ClearSelected();
	}
	selectedUnitsHandler.AddUnit(const_cast<CUnit*>(unit));

	result->error = nullptr;
	result->success = true;
}

static void NativeSelectUnitArray(const SelectUnitArrayQuery* query, SelectUnitArrayResult* result) {
	resultStorage.Reset();
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	if (!query->append) {
		selectedUnitsHandler.ClearSelected();
	}

	for (uint32_t i = 0; i < query->count; i++) {
		const CUnit* unit = unitHandler.GetUnit(query->unitIDs[i]);
		if (unit != nullptr) {
			selectedUnitsHandler.AddUnit(const_cast<CUnit*>(unit));
		}
	}

	result->error = nullptr;
	result->success = true;
}

static void NativeDeselectUnit(const DeselectUnitQuery* query, DeselectUnitResult* result) {
	resultStorage.Reset();
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) { result->error = &INVALID_UNIT_ERROR; return; }

	selectedUnitsHandler.RemoveUnit(const_cast<CUnit*>(unit));

	result->error = nullptr;
	result->success = true;
}

static void NativeDeselectUnitArray(const DeselectUnitArrayQuery* query, DeselectUnitArrayResult* result) {
	resultStorage.Reset();
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	for (uint32_t i = 0; i < query->count; i++) {
		const CUnit* unit = unitHandler.GetUnit(query->unitIDs[i]);
		if (unit != nullptr) {
			selectedUnitsHandler.RemoveUnit(const_cast<CUnit*>(unit));
		}
	}

	result->error = nullptr;
	result->success = true;
}

static void NativeGetGroupList(const GetGroupListQuery* query, GetGroupListResult* result) {
	resultStorage.Reset();
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	int32_t* groups = resultStorage.Allocate<int32_t>(10);
	if (groups == nullptr) { result->error = &RESULT_BUFFER_ERROR; return; }
	uint32_t count = 0;
	for (int g = 0; g < 10; g++) {
		const CGroup* group = uiGroupHandlers[gu->myTeam].GetGroup(g);
		if (group != nullptr && !group->units.empty())
			groups[count++] = g;
	}

	result->error = nullptr;
	result->groups = (count == 0) ? nullptr : groups;
	result->count = count;
}

static void NativeGetSelectedGroup(const GetSelectedGroupQuery* query, GetSelectedGroupResult* result) {
	resultStorage.Reset();
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	result->error = nullptr;
	result->groupID = -1;  // GetDefaultGroup no longer available
}

static void NativeGetGroupUnits(const GetGroupUnitsQuery* query, GetGroupUnitsResult* result) {
	resultStorage.Reset();
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	if (query->groupID < 0 || query->groupID >= 10) { result->error = &INVALID_GROUP_ERROR; return; }

	const CGroup* group = uiGroupHandlers[gu->myTeam].GetGroup(query->groupID);
	if (group == nullptr) { result->error = &INVALID_GROUP_ERROR; return; }

	const size_t count = group->units.size();
	int32_t* units = resultStorage.Allocate<int32_t>(count);
	if (count != 0 && units == nullptr) {
		result->error = &RESULT_BUFFER_ERROR;
		result->units = nullptr;
		result->count = 0;
		return;
	}
	if (count != 0)
		std::copy(group->units.begin(), group->units.end(), units);

	result->error = nullptr;
	result->units = units;
	result->count = static_cast<uint32_t>(count);
}

static void NativeGetGroupUnitsSorted(const GetGroupUnitsSortedQuery* query, GetGroupUnitsSortedResult* result) {
	resultStorage.Reset();
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	result->groups = nullptr;
	result->count = 0;

	if (query->groupID < 0 || query->groupID >= 10) { result->error = &INVALID_GROUP_ERROR; return; }

	const CGroup* group = uiGroupHandlers[gu->myTeam].GetGroup(query->groupID);
	if (group == nullptr) { result->error = &INVALID_GROUP_ERROR; return; }

	std::unordered_map<int32_t, std::vector<int32_t>> unitsByDef;

	for (int unitID : group->units) {
		const CUnit* unit = unitHandler.GetUnit(unitID);
		if (unit != nullptr && unit->unitDef != nullptr) {
			unitsByDef[unit->unitDef->id].push_back(unitID);
		}
	}

	if (unitsByDef.empty()) {
		result->error = nullptr;
		return;
	}

	std::vector<int32_t> unitDefIDs;
	unitDefIDs.reserve(unitsByDef.size());
	for (const auto& [unitDefID, _] : unitsByDef) {
		unitDefIDs.push_back(unitDefID);
	}
	std::sort(unitDefIDs.begin(), unitDefIDs.end());

	size_t totalUnitCount = 0;
	for (const auto& [_, unitIDs] : unitsByDef) {
		if (unitIDs.size() > std::numeric_limits<size_t>::max() - totalUnitCount) {
			result->error = &RESULT_BUFFER_ERROR;
			return;
		}
		totalUnitCount += unitIDs.size();
	}
	const size_t groupCount = unitDefIDs.size();
	if (totalUnitCount > std::numeric_limits<size_t>::max() / sizeof(int32_t)) {
		result->error = &RESULT_BUFFER_ERROR;
		return;
	}
	const size_t unitBytes = totalUnitCount * sizeof(int32_t);
	if (groupCount > (std::numeric_limits<size_t>::max() - unitBytes - alignof(std::max_align_t)) / sizeof(TeamUnitsByDef) ||
		!resultStorage.ReserveAdditional(groupCount * sizeof(TeamUnitsByDef) + unitBytes + alignof(std::max_align_t))) {
		result->error = &RESULT_BUFFER_ERROR;
		return;
	}
	TeamUnitsByDef* groups = resultStorage.Allocate<TeamUnitsByDef>(groupCount);
	if (groups == nullptr) {
		result->error = &RESULT_BUFFER_ERROR;
		return;
	}
	uint32_t groupIndex = 0;
	for (int32_t unitDefID : unitDefIDs) {
		auto& unitIDs = unitsByDef[unitDefID];
		std::sort(unitIDs.begin(), unitIDs.end());

		int32_t* resultUnitIDs = resultStorage.Allocate<int32_t>(unitIDs.size());
		if (!unitIDs.empty() && resultUnitIDs == nullptr) {
			result->error = &RESULT_BUFFER_ERROR;
			return;
		}
		if (!unitIDs.empty())
			std::copy(unitIDs.begin(), unitIDs.end(), resultUnitIDs);
		groups[groupIndex++] = {
			.unitDefID = unitDefID,
			.units = resultUnitIDs,
			.count = static_cast<uint32_t>(unitIDs.size()),
		};
	}

	result->error = nullptr;
	result->groups = groups;
	result->count = static_cast<uint32_t>(groupCount);
}

static void NativeGetUnitGroup(const GetUnitGroupQuery* query, GetUnitGroupResult* result) {
	resultStorage.Reset();
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) { result->error = &INVALID_UNIT_ERROR; return; }

	result->error = nullptr;
	// Lua returns no value when the unit has no group; the parity-facing
	// native result represents that same absence as zero.
	result->groupID = 0;

	if (unit->team != gu->myTeam)
		return;

	const CGroup* group = unit->GetGroup();
	if (group != nullptr)
		result->groupID = group->id;
}

static void NativeSetUnitGroup(const SetUnitGroupQuery* query, SetUnitGroupResult* result) {
	resultStorage.Reset();
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	result->error = nullptr;
	result->success = true;

	if (gs->noHelperAIs)
		return;

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) { result->error = &INVALID_UNIT_ERROR; return; }

	if (query->groupID == -1) {
		unit->SetGroup(nullptr);
		return;
	}

	if (!uiGroupHandlers[gu->myTeam].HasGroup(query->groupID))
		return;

	unit->SetGroup(uiGroupHandlers[gu->myTeam].GetGroup(query->groupID));
}

static void NativeGetGroupUnitsCount(const GetGroupUnitsCountQuery* query, GetGroupUnitsCountResult* result) {
	resultStorage.Reset();
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	if (query->groupID < 0 || query->groupID >= 10) { result->error = &INVALID_GROUP_ERROR; return; }

	const CGroup* group = uiGroupHandlers[gu->myTeam].GetGroup(query->groupID);
	if (group == nullptr) { result->error = &INVALID_GROUP_ERROR; return; }

	result->error = nullptr;
	result->count = static_cast<uint32_t>(group->units.size());
}

static void NativeGetGroupUnitsCounts(const GetGroupUnitsCountsQuery* query, GetGroupUnitsCountsResult* result) {
	resultStorage.Reset();
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	if (query->groupID < 0 || query->groupID >= 10) { result->error = &INVALID_GROUP_ERROR; return; }

	const CGroup* group = uiGroupHandlers[gu->myTeam].GetGroup(query->groupID);
	if (group == nullptr) { result->error = &INVALID_GROUP_ERROR; return; }

	std::unordered_map<int32_t, uint32_t> countMap;

	for (int unitID : group->units) {
		const CUnit* unit = unitHandler.GetUnit(unitID);
		if (unit != nullptr) {
			countMap[unit->unitDef->id]++;
		}
	}

	const size_t count = countMap.size();
	constexpr size_t parallelBytes = sizeof(int32_t) + sizeof(uint32_t);
	if (count > (std::numeric_limits<size_t>::max() - alignof(std::max_align_t)) / parallelBytes ||
		!resultStorage.ReserveAdditional(count * parallelBytes + alignof(std::max_align_t))) {
		result->error = &RESULT_BUFFER_ERROR;
		result->counts = {};
		return;
	}
	int32_t* unitDefIDs = resultStorage.Allocate<int32_t>(count);
	uint32_t* counts = resultStorage.Allocate<uint32_t>(count);
	if ((count != 0 && unitDefIDs == nullptr) || (count != 0 && counts == nullptr)) {
		result->error = &RESULT_BUFFER_ERROR;
		result->counts = {};
		return;
	}
	size_t index = 0;
	for (const auto& [defID, count] : countMap) {
		unitDefIDs[index] = defID;
		counts[index] = count;
		++index;
	}

	result->error = nullptr;
	result->counts.unitDefIDs = unitDefIDs;
	result->counts.counts = counts;
	result->counts.uniqueCount = static_cast<uint32_t>(count);
}

} // namespace

const SelectionApi SELECTION_API = {
	.GetSelectedUnits = NativeGetSelectedUnits,
	.GetSelectedUnitsSorted = NativeGetSelectedUnitsSorted,
	.GetSelectedUnitsCounts = NativeGetSelectedUnitsCounts,
	.GetSelectedUnitsCount = NativeGetSelectedUnitsCount,
	.SelectUnit = NativeSelectUnit,
	.SelectUnitArray = NativeSelectUnitArray,
	.DeselectUnit = NativeDeselectUnit,
	.DeselectUnitArray = NativeDeselectUnitArray,
	.GetGroupList = NativeGetGroupList,
	.GetSelectedGroup = NativeGetSelectedGroup,
	.GetGroupUnits = NativeGetGroupUnits,
	.GetGroupUnitsSorted = NativeGetGroupUnitsSorted,
	.GetGroupUnitsCount = NativeGetGroupUnitsCount,
	.GetGroupUnitsCounts = NativeGetGroupUnitsCounts,
	.GetUnitGroup = NativeGetUnitGroup,
	.SetUnitGroup = NativeSetUnitGroup,
};
