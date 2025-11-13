#include "UnitsCommands.h"

#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Units/CommandAI/CommandAI.h"
#include "Sim/Units/CommandAI/FactoryCAI.h"
#include "Sim/Units/CommandAI/Command.h"
#include "Sim/Units/CommandAI/CommandQueue.h"
#include "Sim/Misc/GlobalSynced.h"
#include <cstring>
#include <algorithm>

namespace {

// Scratch buffer
static thread_local char scratchBuffer[8192];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Game not ready" };
static const Error INVALID_UNIT_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid unit ID" };
static const Error NO_COMMAND_AI_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Unit has no command AI" };
static const Error BUFFER_OVERFLOW_ERROR = { .code = ERROR_BUFFER_OVERFLOW, .message = "Buffer overflow" };

static bool IsReady() {
	return (gs != nullptr);
}

// Helper to allocate from scratch buffer
template<typename T>
static T* AllocateArray(size_t count) {
	size_t needed = count * sizeof(T);
	if (bufferPos + needed > sizeof(scratchBuffer)) {
		return nullptr;
	}
	T* ptr = reinterpret_cast<T*>(&scratchBuffer[bufferPos]);
	bufferPos += needed;
	return ptr;
}

// Helper to copy string to scratch buffer
static const char* CopyString(const std::string& str) {
	size_t len = str.length() + 1;
	if (bufferPos + len > sizeof(scratchBuffer)) {
		return nullptr;
	}
	char* ptr = &scratchBuffer[bufferPos];
	memcpy(ptr, str.c_str(), len);
	bufferPos += len;
	return ptr;
}

// Helper to convert Command to API Command
static bool ConvertCommand(const ::Command& cmd, Command& outCmd) {
	outCmd.cmdID = cmd.GetID(false);
	outCmd.options = cmd.GetOpts();
	outCmd.tag = cmd.GetTag();
	outCmd.aiCommandID = cmd.GetID(true);
	outCmd.timeOut = static_cast<float>(cmd.GetTimeOut());

	uint32_t paramCount = cmd.GetNumParams();
	if (paramCount > 0) {
		outCmd.params = AllocateArray<float>(paramCount);
		if (outCmd.params == nullptr) {
			return false;
		}
		for (uint32_t i = 0; i < paramCount; ++i) {
			outCmd.params[i] = cmd.GetParam(i);
		}
	} else {
		outCmd.params = nullptr;
	}
	outCmd.paramCount = paramCount;

	return true;
}

static void NativeGetUnitCommandCount(const GetUnitCommandCountQuery* query, GetUnitCommandCountResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (unit->commandAI == nullptr) {
		result->error = &NO_COMMAND_AI_ERROR;
		return;
	}

	result->count = static_cast<uint32_t>(unit->commandAI->commandQue.size());
}

static void NativeGetUnitCommands(const GetUnitCommandsQuery* query, GetUnitCommandsResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->commands = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (unit->commandAI == nullptr) {
		result->error = &NO_COMMAND_AI_ERROR;
		return;
	}

	const CCommandQueue& queue = unit->commandAI->commandQue;
	uint32_t count = std::min(static_cast<uint32_t>(queue.size()), query->maxCommands);

	if (count > 0) {
		result->commands = AllocateArray<Command>(count);
		if (result->commands == nullptr) {
			result->error = &BUFFER_OVERFLOW_ERROR;
			return;
		}

		for (uint32_t i = 0; i < count; ++i) {
			if (!ConvertCommand(queue[i], result->commands[i])) {
				result->error = &BUFFER_OVERFLOW_ERROR;
				result->count = i;
				return;
			}
		}
	}

	result->count = count;
}

static void NativeGetUnitCurrentCommand(const GetUnitCurrentCommandQuery* query, GetUnitCurrentCommandResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->hasCommand = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (unit->commandAI == nullptr) {
		result->error = &NO_COMMAND_AI_ERROR;
		return;
	}

	const CCommandQueue& queue = unit->commandAI->commandQue;
	if (!queue.empty()) {
		if (!ConvertCommand(queue.front(), result->command)) {
			result->error = &BUFFER_OVERFLOW_ERROR;
			return;
		}
		result->hasCommand = true;
	}
}

static void NativeGetFactoryCounts(const GetFactoryCountsQuery* query, GetFactoryCountsResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->info.totalCount = 0;
	result->info.currentCount = 0;
	result->info.unitDefIDs = nullptr;
	result->info.counts = nullptr;
	result->info.uniqueCount = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const CFactoryCAI* factoryCAI = dynamic_cast<const CFactoryCAI*>(unit->commandAI);
	if (factoryCAI == nullptr) {
		return; // Not a factory, return zero counts
	}

	const auto& buildOptions = factoryCAI->buildOptions;
	if (buildOptions.empty()) {
		return;
	}

	// Allocate arrays
	result->info.unitDefIDs = AllocateArray<int32_t>(buildOptions.size());
	result->info.counts = AllocateArray<uint32_t>(buildOptions.size());
	if (result->info.unitDefIDs == nullptr || result->info.counts == nullptr) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	// Fill arrays
	uint32_t idx = 0;
	uint32_t totalCount = 0;
	for (const auto& pair : buildOptions) {
		result->info.unitDefIDs[idx] = -pair.first; // Build commands use negative IDs
		result->info.counts[idx] = pair.second;
		totalCount += pair.second;
		idx++;
	}

	result->info.uniqueCount = idx;
	result->info.totalCount = totalCount;
	result->info.currentCount = (unit->beingBuilt) ? 0 : 1; // Simplified
}

static void NativeGetFactoryCommandCount(const GetFactoryCommandCountQuery* query, GetFactoryCommandCountResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const CFactoryCAI* factoryCAI = dynamic_cast<const CFactoryCAI*>(unit->commandAI);
	if (factoryCAI == nullptr) {
		return; // Not a factory
	}

	result->count = static_cast<uint32_t>(factoryCAI->commandQue.size());
}

static void NativeGetFactoryCommands(const GetFactoryCommandsQuery* query, GetFactoryCommandsResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->commands = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const CFactoryCAI* factoryCAI = dynamic_cast<const CFactoryCAI*>(unit->commandAI);
	if (factoryCAI == nullptr) {
		return; // Not a factory
	}

	const CCommandQueue& queue = factoryCAI->commandQue;
	uint32_t count = std::min(static_cast<uint32_t>(queue.size()), query->maxCommands);

	if (count > 0) {
		result->commands = AllocateArray<Command>(count);
		if (result->commands == nullptr) {
			result->error = &BUFFER_OVERFLOW_ERROR;
			return;
		}

		for (uint32_t i = 0; i < count; ++i) {
			if (!ConvertCommand(queue[i], result->commands[i])) {
				result->error = &BUFFER_OVERFLOW_ERROR;
				result->count = i;
				return;
			}
		}
	}

	result->count = count;
}

static void NativeGetFactoryBuggerOff(const GetFactoryBuggerOffQuery* query, GetFactoryBuggerOffResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->isBuggingOff = false;
	result->buggerOffPos.x = 0.0f;
	result->buggerOffPos.y = 0.0f;
	result->buggerOffPos.z = 0.0f;
	result->buggerOffRadius = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	// BuggerOff is a complex feature not easily accessible, return default
	// Would need to track internal factory state
}

static void NativeGetCommandQueue(const GetCommandQueueQuery* query, GetCommandQueueResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->commands = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (unit->commandAI == nullptr) {
		result->error = &NO_COMMAND_AI_ERROR;
		return;
	}

	const CCommandQueue& queue = unit->commandAI->commandQue;
	uint32_t count = std::min(static_cast<uint32_t>(queue.size()), query->maxCommands);

	if (count > 0) {
		result->commands = AllocateArray<Command>(count);
		if (result->commands == nullptr) {
			result->error = &BUFFER_OVERFLOW_ERROR;
			return;
		}

		for (uint32_t i = 0; i < count; ++i) {
			if (!ConvertCommand(queue[i], result->commands[i])) {
				result->error = &BUFFER_OVERFLOW_ERROR;
				result->count = i;
				return;
			}
		}
	}

	result->count = count;
}

static void NativeGetFullBuildQueue(const GetFullBuildQueueQuery* query, GetFullBuildQueueResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->entries = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const CFactoryCAI* factoryCAI = dynamic_cast<const CFactoryCAI*>(unit->commandAI);
	if (factoryCAI == nullptr) {
		return; // Not a factory
	}

	const auto& buildOptions = factoryCAI->buildOptions;
	if (buildOptions.empty()) {
		return;
	}

	result->entries = AllocateArray<BuildQueueEntry>(buildOptions.size());
	if (result->entries == nullptr) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	uint32_t idx = 0;
	for (const auto& pair : buildOptions) {
		result->entries[idx].unitDefID = -pair.first;
		result->entries[idx].numOrdered = pair.second;
		idx++;
	}

	result->count = idx;
}

static void NativeGetRealBuildQueue(const GetRealBuildQueueQuery* query, GetRealBuildQueueResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->unitDefIDs = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const CFactoryCAI* factoryCAI = dynamic_cast<const CFactoryCAI*>(unit->commandAI);
	if (factoryCAI == nullptr) {
		return; // Not a factory
	}

	const CCommandQueue& queue = factoryCAI->commandQue;
	uint32_t buildCount = 0;

	// Count build commands
	for (const auto& cmd : queue) {
		if (cmd.GetID() < 0) { // Build commands have negative IDs
			buildCount++;
		}
	}

	if (buildCount == 0) {
		return;
	}

	result->unitDefIDs = AllocateArray<int32_t>(buildCount);
	if (result->unitDefIDs == nullptr) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	uint32_t idx = 0;
	for (const auto& cmd : queue) {
		if (cmd.GetID() < 0) {
			result->unitDefIDs[idx++] = cmd.GetID();
		}
	}

	result->count = idx;
}

static void NativeGetUnitCmdDescs(const GetUnitCmdDescsQuery* query, GetUnitCmdDescsResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->cmdDescs = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (unit->commandAI == nullptr) {
		result->error = &NO_COMMAND_AI_ERROR;
		return;
	}

	const auto& possibleCmds = unit->commandAI->GetPossibleCommands();
	if (possibleCmds.empty()) {
		return;
	}

	result->cmdDescs = AllocateArray<CommandDescription>(possibleCmds.size());
	if (result->cmdDescs == nullptr) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	for (size_t i = 0; i < possibleCmds.size(); ++i) {
		const SCommandDescription* desc = possibleCmds[i];
		CommandDescription& outDesc = result->cmdDescs[i];

		outDesc.cmdID = desc->id;
		outDesc.action = desc->action;
		outDesc.type = CopyString(desc->type);
		outDesc.name = CopyString(desc->name);
		outDesc.tooltip = CopyString(desc->tooltip);
		outDesc.texture = CopyString(desc->iconname);
		outDesc.cursor = CopyString(desc->mouseicon);
		outDesc.queueing = desc->queueing;
		outDesc.hidden = desc->hidden;
		outDesc.disabled = desc->disabled;
		outDesc.showUnique = desc->showUnique;
		outDesc.onlyTexture = desc->onlyTexture;

		if (!desc->params.empty()) {
			outDesc.params = AllocateArray<const char*>(desc->params.size());
			if (outDesc.params == nullptr) {
				result->error = &BUFFER_OVERFLOW_ERROR;
				result->count = i;
				return;
			}
			for (size_t j = 0; j < desc->params.size(); ++j) {
				outDesc.params[j] = CopyString(desc->params[j]);
				if (outDesc.params[j] == nullptr) {
					result->error = &BUFFER_OVERFLOW_ERROR;
					result->count = i;
					return;
				}
			}
			outDesc.paramCount = desc->params.size();
		} else {
			outDesc.params = nullptr;
			outDesc.paramCount = 0;
		}

		if (outDesc.type == nullptr || outDesc.name == nullptr || outDesc.tooltip == nullptr ||
		    outDesc.texture == nullptr || outDesc.cursor == nullptr) {
			result->error = &BUFFER_OVERFLOW_ERROR;
			result->count = i;
			return;
		}
	}

	result->count = possibleCmds.size();
}

static void NativeFindUnitCmdDesc(const FindUnitCmdDescQuery* query, FindUnitCmdDescResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->found = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (unit->commandAI == nullptr) {
		result->error = &NO_COMMAND_AI_ERROR;
		return;
	}

	const auto& possibleCmds = unit->commandAI->GetPossibleCommands();
	for (const SCommandDescription* desc : possibleCmds) {
		if (desc->id == query->cmdID) {
			CommandDescription& outDesc = result->cmdDesc;

			outDesc.cmdID = desc->id;
			outDesc.action = desc->action;
			outDesc.type = CopyString(desc->type);
			outDesc.name = CopyString(desc->name);
			outDesc.tooltip = CopyString(desc->tooltip);
			outDesc.texture = CopyString(desc->iconname);
			outDesc.cursor = CopyString(desc->mouseicon);
			outDesc.queueing = desc->queueing;
			outDesc.hidden = desc->hidden;
			outDesc.disabled = desc->disabled;
			outDesc.showUnique = desc->showUnique;
			outDesc.onlyTexture = desc->onlyTexture;

			if (!desc->params.empty()) {
				outDesc.params = AllocateArray<const char*>(desc->params.size());
				if (outDesc.params == nullptr) {
					result->error = &BUFFER_OVERFLOW_ERROR;
					return;
				}
				for (size_t j = 0; j < desc->params.size(); ++j) {
					outDesc.params[j] = CopyString(desc->params[j]);
					if (outDesc.params[j] == nullptr) {
						result->error = &BUFFER_OVERFLOW_ERROR;
						return;
					}
				}
				outDesc.paramCount = desc->params.size();
			} else {
				outDesc.params = nullptr;
				outDesc.paramCount = 0;
			}

			if (outDesc.type == nullptr || outDesc.name == nullptr || outDesc.tooltip == nullptr ||
			    outDesc.texture == nullptr || outDesc.cursor == nullptr) {
				result->error = &BUFFER_OVERFLOW_ERROR;
				return;
			}

			result->found = true;
			return;
		}
	}
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
