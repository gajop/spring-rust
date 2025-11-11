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

// Command structure
struct Command {
	int32_t cmdID;
	uint8_t options;  // Bitfield: shift, alt, ctrl, etc.
	int32_t tag;      // User-defined tag
	int32_t aiCommandID;
	float timeOut;

	// Parameters (interpretation depends on cmdID)
	float* params;
	uint32_t paramCount;
};

struct CommandResult {
	const Error* error;
	Command command;
	bool hasCommand;
};

// Command array
struct CommandArray {
	const Error* error;
	Command* commands;
	uint32_t count;
};

// Command description
struct CommandDescription {
	int32_t cmdID;
	int32_t action;          // Command action type
	const char* type;        // "icon", "iconMap", etc.
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
	StringArray params;
};

struct CommandDescriptionResult {
	const Error* error;
	CommandDescription cmdDesc;
};

struct CommandDescriptionArray {
	const Error* error;
	CommandDescription* cmdDescs;
	uint32_t count;
};

// Factory queue info
struct FactoryQueueInfo {
	uint32_t totalCount;      // Total units in queue
	uint32_t currentCount;    // Currently being built
	int32_t* unitDefIDs;
	uint32_t* counts;
	uint32_t uniqueCount;
};

struct FactoryQueueInfoResult {
	const Error* error;
	FactoryQueueInfo info;
};

// Build queue
struct BuildQueueEntry {
	int32_t unitDefID;
	uint32_t numOrdered;
};

struct BuildQueue {
	const Error* error;
	BuildQueueEntry* entries;
	uint32_t count;
};

// API structure
struct UnitsCommandsApi {
	// Command count
	UInt32Result (*GetUnitCommandCount)(int32_t unitID);

	// Get commands
	CommandArray (*GetUnitCommands)(int32_t unitID, uint32_t maxCommands);
	CommandResult (*GetUnitCurrentCommand)(int32_t unitID);

	// Factory commands
	FactoryQueueInfoResult (*GetFactoryCounts)(int32_t unitID);
	UInt32Result (*GetFactoryCommandCount)(int32_t unitID);
	CommandArray (*GetFactoryCommands)(int32_t unitID, uint32_t maxCommands);

	// Factory bugger-off
	BoolResult (*GetFactoryBuggerOff)(int32_t unitID, bool* isBuggingOff, Float3* buggerOffPos, float* buggerOffRadius);

	// Command queue
	CommandArray (*GetCommandQueue)(int32_t unitID, uint32_t maxCommands);

	// Build queue
	BuildQueue (*GetFullBuildQueue)(int32_t unitID);
	Int32Array (*GetRealBuildQueue)(int32_t unitID);

	// Command descriptions
	CommandDescriptionArray (*GetUnitCmdDescs)(int32_t unitID);
	CommandDescriptionResult (*FindUnitCmdDesc)(int32_t unitID, int32_t cmdID);
};

extern const UnitsCommandsApi UNITS_COMMANDS_API;

#ifdef __cplusplus
}
#endif
