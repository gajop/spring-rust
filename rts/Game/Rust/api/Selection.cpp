#include "Selection.h"

#include "Game/SelectedUnitsHandler.h"
#include "Game/UI/Groups/GroupHandler.h"
#include "Game/UI/Groups/Group.h"
#include "Game/GlobalUnsynced.h"
#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include <vector>
#include <unordered_map>

namespace {

// Error constants
static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Selection system not ready"
};

static const Error INVALID_GROUP_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid group ID"
};

// Helper: check if ready
static bool IsReady()
{
	return (gu != nullptr);
}

// Query selection
static Int32Array NativeGetSelectedUnits()
{
	Int32Array result = {};

	// Use static storage - valid for call duration only
	static thread_local std::vector<int32_t> units;
	units.clear();

	for (int unitID : selectedUnitsHandler.selectedUnits) {
		units.push_back(unitID);
	}

	result.data = units.data();
	result.length = static_cast<uint32_t>(units.size());
	return result;
}

static Int32Array NativeGetSelectedUnitsSorted()
{
	Int32Array result = {};

	// For sorted, we'd need to implement by unitDefID
	// Simplified: just return regular list
	return NativeGetSelectedUnits();
}

static SelectionCountsResult NativeGetSelectedUnitsCounts()
{
	SelectionCountsResult result = {};

	// Use static storage - valid for call duration only
	static thread_local std::vector<int32_t> defIDs;
	static thread_local std::vector<uint32_t> counts;
	static thread_local std::unordered_map<int, uint32_t> defCounts;

	defIDs.clear();
	counts.clear();
	defCounts.clear();

	// Count by unit def ID
	for (int unitID : selectedUnitsHandler.selectedUnits) {
		const CUnit* unit = unitHandler.GetUnit(unitID);
		if (unit != nullptr) {
			defCounts[unit->unitDef->id]++;
		}
	}

	// Convert to arrays
	for (const auto& pair : defCounts) {
		defIDs.push_back(pair.first);
		counts.push_back(pair.second);
	}

	result.counts.unitDefIDs = defIDs.data();
	result.counts.counts = counts.data();
	result.counts.uniqueCount = static_cast<uint32_t>(defIDs.size());
	return result;
}

static UInt32Result NativeGetSelectedUnitsCount()
{
	UInt32Result result = {};
	result.value = static_cast<uint32_t>(selectedUnitsHandler.selectedUnits.size());
	return result;
}

// Control selection (unsynced)
static BoolResult NativeSelectUnit(int32_t unitID, bool append)
{
	BoolResult result = {};

	if (!append) {
		selectedUnitsHandler.ClearSelected();
	}

	selectedUnitsHandler.AddUnit(unitHandler.GetUnit(unitID));
	result.value = true;
	return result;
}

static BoolResult NativeSelectUnitArray(const int32_t* unitIDs, uint32_t count, bool append)
{
	BoolResult result = {};
	if (unitIDs == nullptr) {
		result.value = false;
		return result;
	}

	if (!append) {
		selectedUnitsHandler.ClearSelected();
	}

	for (uint32_t i = 0; i < count; ++i) {
		selectedUnitsHandler.AddUnit(unitHandler.GetUnit(unitIDs[i]));
	}

	result.value = true;
	return result;
}

static BoolResult NativeDeselectUnit(int32_t unitID)
{
	BoolResult result = {};
	selectedUnitsHandler.RemoveUnit(unitHandler.GetUnit(unitID));
	result.value = true;
	return result;
}

static BoolResult NativeDeselectUnitArray(const int32_t* unitIDs, uint32_t count)
{
	BoolResult result = {};
	if (unitIDs == nullptr) {
		result.value = false;
		return result;
	}

	for (uint32_t i = 0; i < count; ++i) {
		selectedUnitsHandler.RemoveUnit(unitHandler.GetUnit(unitIDs[i]));
	}

	result.value = true;
	return result;
}

// Groups
static Int32Array NativeGetGroupList()
{
	Int32Array result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Use static storage - valid for call duration only
	static thread_local std::vector<int32_t> groups;
	groups.clear();

	const std::vector<CGroup>& groupList = uiGroupHandlers[gu->myTeam].GetGroups();
	for (const CGroup& group : groupList) {
		if (!group.units.empty()) {
			groups.push_back(group.id);
		}
	}

	result.data = groups.data();
	result.length = static_cast<uint32_t>(groups.size());
	return result;
}

static Int32Result NativeGetSelectedGroup()
{
	Int32Result result = {};
	result.value = selectedUnitsHandler.GetSelectedGroup();
	return result;
}

static Int32Array NativeGetGroupUnits(int32_t groupID)
{
	Int32Array result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CGroupHandler& groupHandler = uiGroupHandlers[gu->myTeam];
	if (!groupHandler.HasGroup(groupID)) {
		result.error = &INVALID_GROUP_ERROR;
		return result;
	}

	const CGroup* group = groupHandler.GetGroup(groupID);
	if (group == nullptr) {
		result.error = &INVALID_GROUP_ERROR;
		return result;
	}

	// Use static storage - valid for call duration only
	static thread_local std::vector<int32_t> units;
	units.clear();

	for (int unitID : group->units) {
		units.push_back(unitID);
	}

	result.data = units.data();
	result.length = static_cast<uint32_t>(units.size());
	return result;
}

static Int32Result NativeGetUnitGroup(int32_t unitID)
{
	Int32Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CUnit* unit = unitHandler.GetUnit(unitID);
	if (unit == nullptr || unit->team != gu->myTeam) {
		result.value = -1;
		return result;
	}

	const CGroup* group = unit->GetGroup();
	result.value = (group != nullptr) ? group->id : -1;
	return result;
}

static BoolResult NativeSetUnitGroup(int32_t unitID, int32_t groupID)
{
	BoolResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	CUnit* unit = unitHandler.GetUnit(unitID);
	if (unit == nullptr || unit->team != gu->myTeam) {
		result.value = false;
		return result;
	}

	CGroupHandler& groupHandler = uiGroupHandlers[gu->myTeam];

	// Remove from current group
	if (unit->GetGroup() != nullptr) {
		unit->GetGroup()->RemoveUnit(unit);
	}

	// Add to new group (if valid)
	if (groupID >= 0 && groupHandler.HasGroup(groupID)) {
		CGroup* group = groupHandler.GetGroup(groupID);
		if (group != nullptr) {
			group->AddUnit(unit);
		}
	}

	result.value = true;
	return result;
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
	.GetUnitGroup = NativeGetUnitGroup,
	.SetUnitGroup = NativeSetUnitGroup,
};
