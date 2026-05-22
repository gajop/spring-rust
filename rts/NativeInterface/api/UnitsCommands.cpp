#include "UnitsCommands.h"

#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Units/CommandAI/CommandAI.h"
#include "Sim/Units/CommandAI/FactoryCAI.h"
#include "Sim/Units/CommandAI/Command.h"
#include "Sim/Units/CommandAI/CommandQueue.h"
#include "Sim/Misc/GlobalSynced.h"
#include "Game/SelectedUnitsHandler.h"
#include "Game/GlobalUnsynced.h"
#include "Lua/LuaConfig.h"
#include <cstring>
#include <algorithm>
#include <vector>

namespace {

// Scratch buffer
static thread_local char scratchBuffer[64 * 1024];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Game not ready" };
static const Error INVALID_UNIT_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid unit ID" };
static const Error NO_COMMAND_AI_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Unit has no command AI" };
static const Error BUFFER_OVERFLOW_ERROR = { .code = ERROR_BUFFER_OVERFLOW, .message = "Buffer overflow" };
static const Error NOT_IMPLEMENTED_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Command issuing not implemented" };
static const Error ORDERS_BLOCKED_ERROR = { .code = ERROR_PERMISSION_DENIED, .message = "Command issuing not allowed" };

static bool IsReady() {
	return (gs != nullptr);
}

static bool CanIssueOrders()
{
	if (!IsReady())
		return false;

	if (gs->PreSimFrame())
		return false;

	if (gs->noHelperAIs)
		return false;

	// Mirrors CSelectedUnitsHandler::SendCommandsToUnits guard
	if (gu->spectating && gs->godMode == 0)
		return false;

	return true;
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

// Helper to convert CMDTYPE_* int to string
static const char* CmdTypeToString(int type) {
	switch (type) {
		case 0:  return "icon";
		case 5:  return "iconMode";
		case 10: return "iconMap";
		case 11: return "iconArea";
		case 12: return "iconUnit";
		case 13: return "iconUnitOrMap";
		case 14: return "iconFront";
		case 16: return "iconUnitOrArea";
		case 17: return "next";
		case 18: return "prev";
		case 19: return "iconUnitFeatureOrArea";
		case 20: return "iconBuilding";
		case 21: return "custom";
		case 22: return "iconUnitOrRectangle";
		case 23: return "number";
		default: return "icon";
	}
}

// Helper to convert engine Command to FFI CommandFFI
static bool ConvertCommand(const ::Command& cmd, CommandFFI& outCmd) {
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

static bool BuildCommand(const CommandFFI& ffi, Command& outCmd)
{
	if (ffi.paramCount > MAX_COMMAND_PARAMS) {
		return false;
	}

	outCmd = Command(ffi.cmdID);
	outCmd.SetOpts(ffi.options);
	outCmd.SetTag(ffi.tag);
	outCmd.SetAICmdID(ffi.aiCommandID);

	if (ffi.timeOut > 0.0f && ffi.timeOut < static_cast<float>(INT_MAX)) {
		outCmd.SetTimeOut(static_cast<int>(ffi.timeOut));
	}

	for (uint32_t i = 0; i < ffi.paramCount; ++i) {
		outCmd.PushParam(ffi.params[i]);
	}

	return true;
}

static bool BuildCommandSimple(int32_t cmdID, uint32_t options, const float* params, uint32_t paramCount, int32_t timeout, Command& outCmd)
{
	CommandFFI ffi{};
	ffi.cmdID = cmdID;
	ffi.options = static_cast<uint8_t>(options);
	ffi.tag = 0;
	ffi.aiCommandID = 0;
	ffi.timeOut = static_cast<float>(timeout);
	ffi.params = const_cast<float*>(params);
	ffi.paramCount = paramCount;
	return BuildCommand(ffi, outCmd);
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
		result->commands = AllocateArray<CommandFFI>(count);
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

	const CFactoryCAI* factoryCAI = dynamic_cast<const CFactoryCAI*>(unit->commandAI);
	const CCommandQueue& queue = (factoryCAI == nullptr) ? unit->commandAI->commandQue : factoryCAI->newUnitCommands;
	int cmdIndex = query->cmdIndex;
	if (cmdIndex > 0) {
		cmdIndex -= 1;
	} else {
		cmdIndex = static_cast<int>(queue.size()) + cmdIndex;
	}

	if (cmdIndex >= 0 && cmdIndex < static_cast<int>(queue.size())) {
		if (!ConvertCommand(queue[cmdIndex], result->command)) {
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

	const CCommandQueue& commandQue = factoryCAI->commandQue;
	int count = query->count;
	if (count < 0) {
		count = static_cast<int>(commandQue.size());
	}

	// Use command queue to count commands
	std::map<int, uint32_t> cmdCounts;
	int processed = 0;
	for (const auto& cmd : commandQue) {
		if (processed >= count) break;
		const int id = cmd.GetID(false);
		if (!query->addCmds && id >= 0) {
			continue; // skip non-build commands when addCmds=false
		}
		cmdCounts[id] += 1;
		processed++;
	}

	if (cmdCounts.empty()) {
		return;
	}

	// Allocate arrays
	result->info.unitDefIDs = AllocateArray<int32_t>(cmdCounts.size());
	result->info.counts = AllocateArray<uint32_t>(cmdCounts.size());
	if (result->info.unitDefIDs == nullptr || result->info.counts == nullptr) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	// Fill arrays
	uint32_t idx = 0;
	uint32_t totalCount = 0;
	for (const auto& pair : cmdCounts) {
		result->info.unitDefIDs[idx] = pair.first;
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
		result->commands = AllocateArray<CommandFFI>(count);
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
		result->commands = AllocateArray<CommandFFI>(count);
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
		outDesc.action = 0; // action string not mapped to int currently
		outDesc.type = CmdTypeToString(desc->type);
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
	result->cmdIndex = 0;
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

	for (int i = 0; i < static_cast<int>(possibleCmds.size()); ++i) {
		const SCommandDescription* desc = possibleCmds[i];
		if (desc->id == query->cmdID) {
			result->cmdIndex = i + CMD_INDEX_OFFSET;
			result->found = true;
			return;
		}
	}
}

static void NativeGiveOrder(const GiveOrderQuery* query, GiveOrderResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!CanIssueOrders()) {
		result->error = &ORDERS_BLOCKED_ERROR;
		return;
	}

	Command cmd;
	if (!BuildCommandSimple(query->cmdID, query->options, query->params, query->paramCount, query->timeout, cmd)) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	selectedUnitsHandler.GiveCommand(cmd);
	result->success = true;
}

static void NativeGiveOrderToUnitMap(const GiveOrderToUnitMapQuery* query, GiveOrderToUnitMapResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->unitsOrdered = 0;

	if (!CanIssueOrders()) {
		result->error = &ORDERS_BLOCKED_ERROR;
		return;
	}

	Command cmd;
	if (!BuildCommandSimple(query->cmdID, query->options, query->params, query->paramCount, query->timeout, cmd)) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	std::vector<int> unitIDs;
	unitIDs.reserve(query->count);

	for (uint32_t i = 0; i < query->count; ++i) {
		CUnit* unit = unitHandler.GetUnit(query->unitIDs[i]);
		if (unit != nullptr && !unit->noSelect) {
			unitIDs.push_back(unit->id);
		}
	}

	if (!unitIDs.empty()) {
		selectedUnitsHandler.SendCommandsToUnits(unitIDs, {cmd});
		result->unitsOrdered = static_cast<int32_t>(unitIDs.size());
	}
}

static void NativeGiveOrderArrayToUnitMap(const GiveOrderArrayToUnitMapQuery* query, GiveOrderArrayToUnitMapResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->unitsOrdered = 0;

	if (!CanIssueOrders()) {
		result->error = &ORDERS_BLOCKED_ERROR;
		return;
	}

	std::vector<int> unitIDs;
	unitIDs.reserve(query->unitCount);

	for (uint32_t i = 0; i < query->unitCount; ++i) {
		CUnit* unit = unitHandler.GetUnit(query->unitIDs[i]);
		if (unit != nullptr && !unit->noSelect) {
			unitIDs.push_back(unit->id);
		}
	}

	std::vector<Command> commands;
	commands.reserve(query->commandCount);

	for (uint32_t i = 0; i < query->commandCount; ++i) {
		Command cmd;
		if (!BuildCommand(query->commands[i], cmd)) {
			result->error = &BUFFER_OVERFLOW_ERROR;
			return;
		}
		commands.push_back(cmd);
	}

	if (!unitIDs.empty() && !commands.empty()) {
		selectedUnitsHandler.SendCommandsToUnits(unitIDs, commands, false);
		result->unitsOrdered = static_cast<int32_t>(unitIDs.size());
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
	.GiveOrder = NativeGiveOrder,
	.GiveOrderToUnitMap = NativeGiveOrderToUnitMap,
	.GiveOrderArrayToUnitMap = NativeGiveOrderArrayToUnitMap,
};
