#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Selection API
// @see rts/Lua/LuaUnsyncedRead.cpp, LuaUnsyncedCtrl.cpp
//
// Unit selection queries and control (unsynced)
// ============================================================================

// Selection count by unit def
struct SelectionCounts {
	int32_t* unitDefIDs;
	uint32_t* counts;
	uint32_t uniqueCount;
};

struct SelectionCountsResult {
	const Error* error;
	SelectionCounts counts;
};

// Group info
struct GroupInfo {
	int32_t groupID;
	uint32_t unitCount;
};

// API structure
struct SelectionApi {
	// Query selection
	Int32Array (*GetSelectedUnits)();
	Int32Array (*GetSelectedUnitsSorted)();
	SelectionCountsResult (*GetSelectedUnitsCounts)();
	UInt32Result (*GetSelectedUnitsCount)();

	// Control selection (unsynced)
	BoolResult (*SelectUnit)(int32_t unitID, bool append);
	BoolResult (*SelectUnitArray)(const int32_t* unitIDs, uint32_t count, bool append);
	BoolResult (*DeselectUnit)(int32_t unitID);
	BoolResult (*DeselectUnitArray)(const int32_t* unitIDs, uint32_t count);

	// Groups
	Int32Array (*GetGroupList)();
	Int32Result (*GetSelectedGroup)();
	Int32Array (*GetGroupUnits)(int32_t groupID);
	Int32Result (*GetUnitGroup)(int32_t unitID);
	BoolResult (*SetUnitGroup)(int32_t unitID, int32_t groupID);
};

extern const SelectionApi SELECTION_API;

#ifdef __cplusplus
}
#endif
