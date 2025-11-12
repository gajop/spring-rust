#include "UnitsCommands.h"

namespace {

// Scratch buffer
static thread_local char scratchBuffer[8192];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_IMPLEMENTED_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "UnitsCommands API not yet fully implemented" };
static const Error INVALID_UNIT_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid unit ID" };

// Command queue management - complex Lua/C++ integration
static void NativeGetUnitCommandCount(const GetUnitCommandCountQuery* query, GetUnitCommandCountResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->count = 0;
}

static void NativeGetUnitCommands(const GetUnitCommandsQuery* query, GetUnitCommandsResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->commands = nullptr;
	result->count = 0;
}

static void NativeGetUnitCurrentCommand(const GetUnitCurrentCommandQuery* query, GetUnitCurrentCommandResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->hasCommand = false;
}

static void NativeGetFactoryCounts(const GetFactoryCountsQuery* query, GetFactoryCountsResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->info.totalCount = 0;
	result->info.currentCount = 0;
	result->info.unitDefIDs = nullptr;
	result->info.counts = nullptr;
	result->info.uniqueCount = 0;
}

static void NativeGetFactoryCommandCount(const GetFactoryCommandCountQuery* query, GetFactoryCommandCountResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->count = 0;
}

static void NativeGetFactoryCommands(const GetFactoryCommandsQuery* query, GetFactoryCommandsResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->commands = nullptr;
	result->count = 0;
}

static void NativeGetFactoryBuggerOff(const GetFactoryBuggerOffQuery* query, GetFactoryBuggerOffResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->isBuggingOff = false;
	result->buggerOffPos.x = 0.0f;
	result->buggerOffPos.y = 0.0f;
	result->buggerOffPos.z = 0.0f;
	result->buggerOffRadius = 0.0f;
}

static void NativeGetCommandQueue(const GetCommandQueueQuery* query, GetCommandQueueResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->commands = nullptr;
	result->count = 0;
}

static void NativeGetFullBuildQueue(const GetFullBuildQueueQuery* query, GetFullBuildQueueResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->entries = nullptr;
	result->count = 0;
}

static void NativeGetRealBuildQueue(const GetRealBuildQueueQuery* query, GetRealBuildQueueResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->unitDefIDs = nullptr;
	result->count = 0;
}

static void NativeGetUnitCmdDescs(const GetUnitCmdDescsQuery* query, GetUnitCmdDescsResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->cmdDescs = nullptr;
	result->count = 0;
}

static void NativeFindUnitCmdDesc(const FindUnitCmdDescQuery* query, FindUnitCmdDescResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->found = false;
}

} // namespace

const UnitsCommandsApi UNITS_COMMANDS_API = {
	.GetUnitCommandCount = NativeGetUnitCommandCount,
	.GetUnitCommands = NativeGetUnitCommands,
	.GetUnitCurrentCommand = NativeGetUnitCurrentCommand,
	.GetFactoryCounts = NativeGetFactoryCounts,
	.GetFactoryCommandCount = NativeGetFactoryCommandCount,
	.GetFactoryCommands = NativeGetFactoryCommands,
	.GetFactoryBuggerOff = NativeGetFactoryBuggerOff,
	.GetCommandQueue = NativeGetCommandQueue,
	.GetFullBuildQueue = NativeGetFullBuildQueue,
	.GetRealBuildQueue = NativeGetRealBuildQueue,
	.GetUnitCmdDescs = NativeGetUnitCmdDescs,
	.FindUnitCmdDesc = NativeFindUnitCmdDesc,
};
