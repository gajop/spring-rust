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

// Queries
struct GetSelectedUnitsQuery { uint8_t _unused; };
struct GetSelectedUnitsResult { const Error* error; int32_t* units; uint32_t count; };

struct GetSelectedUnitsSortedQuery { uint8_t _unused; };
struct GetSelectedUnitsSortedResult { const Error* error; int32_t* units; uint32_t count; };

struct GetSelectedUnitsCountsQuery { uint8_t _unused; };
struct GetSelectedUnitsCountsResult { const Error* error; SelectionCounts counts; };

struct GetSelectedUnitsCountQuery { uint8_t _unused; };
struct GetSelectedUnitsCountResult { const Error* error; uint32_t count; };

struct SelectUnitQuery { int32_t unitID; bool append; };
struct SelectUnitResult { const Error* error; bool success; };

struct SelectUnitArrayQuery { const int32_t* unitIDs; uint32_t count; bool append; };
struct SelectUnitArrayResult { const Error* error; bool success; };

struct DeselectUnitQuery { int32_t unitID; };
struct DeselectUnitResult { const Error* error; bool success; };

struct DeselectUnitArrayQuery { const int32_t* unitIDs; uint32_t count; };
struct DeselectUnitArrayResult { const Error* error; bool success; };

struct GetGroupListQuery { uint8_t _unused; };
struct GetGroupListResult { const Error* error; int32_t* groups; uint32_t count; };

struct GetSelectedGroupQuery { uint8_t _unused; };
struct GetSelectedGroupResult { const Error* error; int32_t groupID; };

struct GetGroupUnitsQuery { int32_t groupID; };
struct GetGroupUnitsResult { const Error* error; int32_t* units; uint32_t count; };

struct GetUnitGroupQuery { int32_t unitID; };
struct GetUnitGroupResult { const Error* error; int32_t groupID; };

struct SetUnitGroupQuery { int32_t unitID; int32_t groupID; };
struct SetUnitGroupResult { const Error* error; bool success; };

// API structure
struct SelectionApi {
	void (*GetSelectedUnits)(const GetSelectedUnitsQuery* query, GetSelectedUnitsResult* result);
	void (*GetSelectedUnitsSorted)(const GetSelectedUnitsSortedQuery* query, GetSelectedUnitsSortedResult* result);
	void (*GetSelectedUnitsCounts)(const GetSelectedUnitsCountsQuery* query, GetSelectedUnitsCountsResult* result);
	void (*GetSelectedUnitsCount)(const GetSelectedUnitsCountQuery* query, GetSelectedUnitsCountResult* result);
	void (*SelectUnit)(const SelectUnitQuery* query, SelectUnitResult* result);
	void (*SelectUnitArray)(const SelectUnitArrayQuery* query, SelectUnitArrayResult* result);
	void (*DeselectUnit)(const DeselectUnitQuery* query, DeselectUnitResult* result);
	void (*DeselectUnitArray)(const DeselectUnitArrayQuery* query, DeselectUnitArrayResult* result);
	void (*GetGroupList)(const GetGroupListQuery* query, GetGroupListResult* result);
	void (*GetSelectedGroup)(const GetSelectedGroupQuery* query, GetSelectedGroupResult* result);
	void (*GetGroupUnits)(const GetGroupUnitsQuery* query, GetGroupUnitsResult* result);
	void (*GetUnitGroup)(const GetUnitGroupQuery* query, GetUnitGroupResult* result);
	void (*SetUnitGroup)(const SetUnitGroupQuery* query, SetUnitGroupResult* result);
};

extern const SelectionApi SELECTION_API;

#ifdef __cplusplus
}
#endif
