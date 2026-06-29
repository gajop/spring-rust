#include "Selection.h"

#include "Game/SelectedUnitsHandler.h"
#include "Game/UI/Groups/GroupHandler.h"
#include "Game/UI/Groups/Group.h"
#include "Game/GlobalUnsynced.h"
#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitDef.h"
#include "Sim/Units/UnitHandler.h"
#include <algorithm>
#include <unordered_map>

namespace {

// Scratch buffer
static thread_local char scratchBuffer[1024];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Selection system not ready" };
static const Error INVALID_UNIT_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid unit ID" };
static const Error INVALID_GROUP_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid group ID" };

static bool IsReady() { return (gu != nullptr); }

static void NativeGetSelectedUnits(const GetSelectedUnitsQuery* query, GetSelectedUnitsResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	int32_t* units = reinterpret_cast<int32_t*>(&scratchBuffer[bufferPos]);
	uint32_t count = 0;

	for (int unitID : selectedUnitsHandler.selectedUnits) {
		if (bufferPos + sizeof(int32_t) > sizeof(scratchBuffer)) break;
		units[count++] = unitID;
		bufferPos += sizeof(int32_t);
	}

	result->error = nullptr;
	result->units = units;
	result->count = count;
}

static void NativeGetSelectedUnitsSorted(const GetSelectedUnitsSortedQuery* query, GetSelectedUnitsSortedResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	int32_t* units = reinterpret_cast<int32_t*>(&scratchBuffer[bufferPos]);
	uint32_t count = 0;

	for (int unitID : selectedUnitsHandler.selectedUnits) {
		if (bufferPos + sizeof(int32_t) > sizeof(scratchBuffer)) break;
		units[count++] = unitID;
		bufferPos += sizeof(int32_t);
	}

	std::sort(units, units + count);

	result->error = nullptr;
	result->units = units;
	result->count = count;
}

static void NativeGetSelectedUnitsCounts(const GetSelectedUnitsCountsQuery* query, GetSelectedUnitsCountsResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	std::unordered_map<int32_t, uint32_t> countMap;

	for (int unitID : selectedUnitsHandler.selectedUnits) {
		const CUnit* unit = unitHandler.GetUnit(unitID);
		if (unit != nullptr) {
			countMap[unit->unitDef->id]++;
		}
	}

	int32_t* defIDs = reinterpret_cast<int32_t*>(&scratchBuffer[bufferPos]);
	size_t arraySize = countMap.size() * sizeof(int32_t);
	if (bufferPos + arraySize > sizeof(scratchBuffer)) { result->error = &NOT_READY_ERROR; return; }
	bufferPos += arraySize;

	uint32_t* counts = reinterpret_cast<uint32_t*>(&scratchBuffer[bufferPos]);
	arraySize = countMap.size() * sizeof(uint32_t);
	if (bufferPos + arraySize > sizeof(scratchBuffer)) { result->error = &NOT_READY_ERROR; return; }
	bufferPos += arraySize;

	uint32_t idx = 0;
	for (const auto& [defID, count] : countMap) {
		defIDs[idx] = defID;
		counts[idx] = count;
		idx++;
	}

	result->error = nullptr;
	result->counts.unitDefIDs = defIDs;
	result->counts.counts = counts;
	result->counts.uniqueCount = idx;
}

static void NativeGetSelectedUnitsCount(const GetSelectedUnitsCountQuery* query, GetSelectedUnitsCountResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	result->error = nullptr;
	result->count = static_cast<uint32_t>(selectedUnitsHandler.selectedUnits.size());
}

static void NativeSelectUnit(const SelectUnitQuery* query, SelectUnitResult* result) {
	bufferPos = 0;
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
	bufferPos = 0;
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
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) { result->error = &INVALID_UNIT_ERROR; return; }

	selectedUnitsHandler.RemoveUnit(const_cast<CUnit*>(unit));

	result->error = nullptr;
	result->success = true;
}

static void NativeDeselectUnitArray(const DeselectUnitArrayQuery* query, DeselectUnitArrayResult* result) {
	bufferPos = 0;
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
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	int32_t* groups = reinterpret_cast<int32_t*>(&scratchBuffer[bufferPos]);
	uint32_t count = 0;

	for (int g = 0; g < 10; g++) {
		const CGroup* group = uiGroupHandlers[gu->myTeam].GetGroup(g);
		if (group != nullptr && !group->units.empty()) {
			if (bufferPos + sizeof(int32_t) > sizeof(scratchBuffer)) break;
			groups[count++] = g;
			bufferPos += sizeof(int32_t);
		}
	}

	result->error = nullptr;
	result->groups = groups;
	result->count = count;
}

static void NativeGetSelectedGroup(const GetSelectedGroupQuery* query, GetSelectedGroupResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	result->error = nullptr;
	result->groupID = -1;  // GetDefaultGroup no longer available
}

static void NativeGetGroupUnits(const GetGroupUnitsQuery* query, GetGroupUnitsResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	if (query->groupID < 0 || query->groupID >= 10) { result->error = &INVALID_GROUP_ERROR; return; }

	const CGroup* group = uiGroupHandlers[gu->myTeam].GetGroup(query->groupID);
	if (group == nullptr) { result->error = &INVALID_GROUP_ERROR; return; }

	int32_t* units = reinterpret_cast<int32_t*>(&scratchBuffer[bufferPos]);
	uint32_t count = 0;

	for (int unitID : group->units) {
		if (bufferPos + sizeof(int32_t) > sizeof(scratchBuffer)) break;
		units[count++] = unitID;
		bufferPos += sizeof(int32_t);
	}

	result->error = nullptr;
	result->units = units;
	result->count = count;
}

static void NativeGetGroupUnitsSorted(const GetGroupUnitsSortedQuery* query, GetGroupUnitsSortedResult* result) {
	bufferPos = 0;
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

	const size_t groupBytes = unitsByDef.size() * sizeof(TeamUnitsByDef);
	if (bufferPos + groupBytes > sizeof(scratchBuffer)) { result->error = &NOT_READY_ERROR; return; }
	TeamUnitsByDef* groups = reinterpret_cast<TeamUnitsByDef*>(&scratchBuffer[bufferPos]);
	bufferPos += groupBytes;

	std::vector<int32_t> unitDefIDs;
	unitDefIDs.reserve(unitsByDef.size());
	for (const auto& [unitDefID, _] : unitsByDef) {
		unitDefIDs.push_back(unitDefID);
	}
	std::sort(unitDefIDs.begin(), unitDefIDs.end());

	uint32_t groupCount = 0;
	for (int32_t unitDefID : unitDefIDs) {
		auto& unitIDs = unitsByDef[unitDefID];
		std::sort(unitIDs.begin(), unitIDs.end());

		const size_t bytes = unitIDs.size() * sizeof(int32_t);
		if (bufferPos + bytes > sizeof(scratchBuffer)) { result->error = &NOT_READY_ERROR; return; }

		int32_t* units = reinterpret_cast<int32_t*>(&scratchBuffer[bufferPos]);
		for (uint32_t i = 0; i < unitIDs.size(); ++i) {
			units[i] = unitIDs[i];
		}
		bufferPos += bytes;

		TeamUnitsByDef& groupEntry = groups[groupCount++];
		groupEntry.unitDefID = unitDefID;
		groupEntry.units = units;
		groupEntry.count = unitIDs.size();
	}

	result->error = nullptr;
	result->groups = groups;
	result->count = groupCount;
}

static void NativeGetUnitGroup(const GetUnitGroupQuery* query, GetUnitGroupResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) { result->error = &INVALID_UNIT_ERROR; return; }

	result->error = nullptr;
	result->groupID = -1;  // Default: not in any group

	// Find which group contains this unit (units don't track their group directly)
	for (int g = 0; g < 10; g++) {
		const CGroup* group = uiGroupHandlers[unit->team].GetGroup(g);
		if (group != nullptr && group->units.count(unit->id) > 0) {
			result->groupID = g;
			return;
		}
	}
}

static void NativeSetUnitGroup(const SetUnitGroupQuery* query, SetUnitGroupResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) { result->error = &INVALID_UNIT_ERROR; return; }

	if (query->groupID >= 0 && query->groupID < 10) {
		// First remove from current group (if any)
		for (int g = 0; g < 10; g++) {
			CGroup* group = uiGroupHandlers[unit->team].GetGroup(g);
			if (group != nullptr && group->units.count(unit->id) > 0) {
				group->RemoveUnit(unit);
				break;
			}
		}
		// Then add to new group
		CGroup* newGroup = uiGroupHandlers[unit->team].GetGroup(query->groupID);
		if (newGroup != nullptr) {
			newGroup->AddUnit(unit);
		}
		result->error = nullptr;
		result->success = true;
	} else {
		// Remove from any group
		for (int g = 0; g < 10; g++) {
			CGroup* group = uiGroupHandlers[unit->team].GetGroup(g);
			if (group != nullptr && group->units.count(unit->id) > 0) {
				group->RemoveUnit(unit);
				break;
			}
		}
		result->error = nullptr;
		result->success = true;
	}
}

static void NativeGetGroupUnitsCount(const GetGroupUnitsCountQuery* query, GetGroupUnitsCountResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	if (query->groupID < 0 || query->groupID >= 10) { result->error = &INVALID_GROUP_ERROR; return; }

	const CGroup* group = uiGroupHandlers[gu->myTeam].GetGroup(query->groupID);
	if (group == nullptr) { result->error = &INVALID_GROUP_ERROR; return; }

	result->error = nullptr;
	result->count = static_cast<uint32_t>(group->units.size());
}

static void NativeGetGroupUnitsCounts(const GetGroupUnitsCountsQuery* query, GetGroupUnitsCountsResult* result) {
	bufferPos = 0;
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

	int32_t* defIDs = reinterpret_cast<int32_t*>(&scratchBuffer[bufferPos]);
	size_t arraySize = countMap.size() * sizeof(int32_t);
	if (bufferPos + arraySize > sizeof(scratchBuffer)) { result->error = &NOT_READY_ERROR; return; }
	bufferPos += arraySize;

	uint32_t* counts = reinterpret_cast<uint32_t*>(&scratchBuffer[bufferPos]);
	arraySize = countMap.size() * sizeof(uint32_t);
	if (bufferPos + arraySize > sizeof(scratchBuffer)) { result->error = &NOT_READY_ERROR; return; }
	bufferPos += arraySize;

	uint32_t idx = 0;
	for (const auto& [defID, count] : countMap) {
		defIDs[idx] = defID;
		counts[idx] = count;
		idx++;
	}

	result->error = nullptr;
	result->counts.unitDefIDs = defIDs;
	result->counts.counts = counts;
	result->counts.uniqueCount = idx;
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
