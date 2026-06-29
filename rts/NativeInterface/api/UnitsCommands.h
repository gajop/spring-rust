#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Units Commands API
// @see rts/Lua/LuaSyncedRead.cpp
//
// Unit command queue and command description queries
// ============================================================================

// Command structure (FFI version to avoid conflicts with engine Command)
struct CommandFFI {
	int32_t cmdID;
	uint8_t options;  // Bitfield: shift, alt, ctrl, etc.
	int32_t tag;      // User-defined tag
	int32_t aiCommandID;
	float timeOut;

	// Parameters (interpretation depends on cmdID)
	float* params;
	uint32_t paramCount;
};

// Command description
struct CommandDescription {
	int32_t cmdID;
	const char* action;      // Command action binding name
	int32_t type;            // CMDTYPE_* from Command.h
	const char* name;
	const char* tooltip;
	const char* texture;
	const char* cursor;
	bool queueing;
	bool hidden;
	bool disabled;
	bool showUnique;
	bool onlyTexture;

	// Custom params
	const char** params;
	uint32_t paramCount;
};

// Factory queue info
struct FactoryQueueInfo {
	uint32_t totalCount;      // Total units in queue
	uint32_t currentCount;    // Currently being built
	int32_t* unitDefIDs;
	uint32_t* counts;
	uint32_t uniqueCount;
};

// Build queue entry
struct BuildQueueEntry {
	int32_t unitDefID;
	uint32_t numOrdered;
};

// Queries
struct GetUnitCommandCountQuery { int32_t unitID; };
struct GetUnitCommandCountResult { const Error* error; uint32_t count; };

struct GetUnitCommandsQuery { int32_t unitID; uint32_t maxCommands; };
struct GetUnitCommandsResult { const Error* error; CommandFFI* commands; uint32_t count; };

struct GetUnitCurrentCommandQuery { int32_t unitID; int32_t cmdIndex; };
struct GetUnitCurrentCommandResult { const Error* error; CommandFFI command; bool hasCommand; };

struct GetFactoryCountsQuery { int32_t unitID; int32_t count; bool addCmds; };
struct GetFactoryCountsResult { const Error* error; FactoryQueueInfo info; };

struct GetFactoryCommandCountQuery { int32_t unitID; };
struct GetFactoryCommandCountResult { const Error* error; uint32_t count; };

struct GetFactoryCommandsQuery { int32_t unitID; uint32_t maxCommands; };
struct GetFactoryCommandsResult { const Error* error; CommandFFI* commands; uint32_t count; };

struct GetFactoryBuggerOffQuery { int32_t unitID; };
struct GetFactoryBuggerOffResult { const Error* error; bool isBuggingOff; Float3 buggerOffPos; float buggerOffRadius; };

struct GetCommandQueueQuery { int32_t unitID; uint32_t maxCommands; };
struct GetCommandQueueResult { const Error* error; CommandFFI* commands; uint32_t count; };

struct GetFullBuildQueueQuery { int32_t unitID; };
struct GetFullBuildQueueResult { const Error* error; BuildQueueEntry* entries; uint32_t count; };

struct GetRealBuildQueueQuery { int32_t unitID; };
struct GetRealBuildQueueResult { const Error* error; int32_t* unitDefIDs; uint32_t count; };

struct GetUnitCmdDescsQuery { int32_t unitID; };
struct GetUnitCmdDescsResult { const Error* error; CommandDescription* cmdDescs; uint32_t count; };

struct FindUnitCmdDescQuery { int32_t unitID; int32_t cmdID; };
struct FindUnitCmdDescResult { const Error* error; int32_t cmdIndex; bool found; };

struct GetCommandParamsQuery { const CommandFFI* command; };
struct GetCommandParamsResult { const Error* error; float* params; uint32_t count; };

struct GiveOrderQuery { int32_t cmdID; float* params; uint32_t paramCount; uint32_t options; int32_t timeout; };
struct GiveOrderResult { const Error* error; bool success; };

struct GiveOrderToUnitMapQuery {
	const int32_t* unitIDs;
	uint32_t count;
	int32_t cmdID;
	float* params;
	uint32_t paramCount;
	uint32_t options;
	int32_t timeout;
};

struct GiveOrderToUnitMapResult { const Error* error; int32_t unitsOrdered; };

struct GiveOrderArrayToUnitMapQuery {
	const int32_t* unitIDs;
	uint32_t unitCount;
	const CommandFFI* commands;
	uint32_t commandCount;
};

struct GiveOrderArrayToUnitMapResult { const Error* error; int32_t unitsOrdered; };

// API structure
struct UnitsCommandsApi {
	void (*GetUnitCommandCount)(const GetUnitCommandCountQuery* query, GetUnitCommandCountResult* result);
	void (*GetUnitCommands)(const GetUnitCommandsQuery* query, GetUnitCommandsResult* result);
	void (*GetUnitCurrentCommand)(const GetUnitCurrentCommandQuery* query, GetUnitCurrentCommandResult* result);
	void (*GetFactoryCounts)(const GetFactoryCountsQuery* query, GetFactoryCountsResult* result);
	void (*GetFactoryCommandCount)(const GetFactoryCommandCountQuery* query, GetFactoryCommandCountResult* result);
	void (*GetFactoryCommands)(const GetFactoryCommandsQuery* query, GetFactoryCommandsResult* result);
	void (*GetFactoryBuggerOff)(const GetFactoryBuggerOffQuery* query, GetFactoryBuggerOffResult* result);
	void (*GetCommandQueue)(const GetCommandQueueQuery* query, GetCommandQueueResult* result);
	void (*GetFullBuildQueue)(const GetFullBuildQueueQuery* query, GetFullBuildQueueResult* result);
	void (*GetRealBuildQueue)(const GetRealBuildQueueQuery* query, GetRealBuildQueueResult* result);
	void (*GetUnitCmdDescs)(const GetUnitCmdDescsQuery* query, GetUnitCmdDescsResult* result);
	void (*FindUnitCmdDesc)(const FindUnitCmdDescQuery* query, FindUnitCmdDescResult* result);
	void (*GetCommandParams)(const GetCommandParamsQuery* query, GetCommandParamsResult* result);
	void (*GiveOrder)(const GiveOrderQuery* query, GiveOrderResult* result);
	void (*GiveOrderToUnitMap)(const GiveOrderToUnitMapQuery* query, GiveOrderToUnitMapResult* result);
	void (*GiveOrderArrayToUnitMap)(const GiveOrderArrayToUnitMapQuery* query, GiveOrderArrayToUnitMapResult* result);
};

extern const UnitsCommandsApi UNITS_COMMANDS_API;

#ifdef __cplusplus
}
#endif
