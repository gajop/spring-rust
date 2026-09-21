#include "UnitsCommands.h"

#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Units/UnitTypes/Factory.h"
#include "Sim/Units/CommandAI/CommandAI.h"
#include "Sim/Units/CommandAI/FactoryCAI.h"
#include "Sim/Units/CommandAI/Command.h"
#include "Sim/Units/CommandAI/CommandQueue.h"
#include "Sim/Misc/GlobalSynced.h"
#include "Game/SelectedUnitsHandler.h"
#include "Game/GlobalUnsynced.h"
#include "Lua/LuaConfig.h"
#include <algorithm>
#include <map>
#include <vector>

namespace {

static thread_local std::vector<CommandFFI> commandStorage;
static thread_local std::vector<std::vector<float>> commandParameterStorage;
static thread_local std::vector<int32_t> integerStorage;
static thread_local std::vector<uint32_t> countStorage;
static thread_local std::vector<BuildQueueEntry> buildQueueStorage;
static thread_local std::vector<CommandDescription> commandDescriptionStorage;
static thread_local std::vector<std::vector<std::string>> descriptionStringStorage;
static thread_local std::vector<std::vector<const char*>> descriptionParameterStorage;
static thread_local std::vector<float> parameterStorage;

// Static errors
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Game not ready" };
static const Error INVALID_UNIT_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid unit ID" };
static const Error NOT_FACTORY_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Unit is not a factory" };
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

// Helper to convert engine Command to FFI CommandFFI
static bool ConvertCommand(const ::Command& cmd, CommandFFI& outCmd) {
	outCmd.cmdID = cmd.GetID(false);
	outCmd.options = cmd.GetOpts();
	outCmd.tag = cmd.GetTag();
	outCmd.aiCommandID = cmd.GetID(true);
	outCmd.timeOut = static_cast<float>(cmd.GetTimeOut());

	uint32_t paramCount = cmd.GetNumParams();
	if (paramCount > 0) {
		commandParameterStorage.emplace_back();
		auto& params = commandParameterStorage.back();
		params.resize(paramCount);
		outCmd.params = params.data();
		for (uint32_t i = 0; i < paramCount; ++i) {
			outCmd.params[i] = cmd.GetParam(i);
		}
	} else {
		outCmd.params = nullptr;
	}
	outCmd.paramCount = paramCount;

	return true;
}

static void BeginCommandStorage(size_t count)
{
	commandStorage.clear();
	commandParameterStorage.clear();
	commandStorage.resize(count);
	commandParameterStorage.reserve(count);
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
	BeginCommandStorage(count);

	if (count > 0) {
		for (uint32_t i = 0; i < count; ++i) {
			if (!ConvertCommand(queue[i], commandStorage[i])) {
				result->count = i;
				return;
			}
		}
		result->commands = commandStorage.data();
	}

	result->count = count;
}

static void NativeGetUnitCurrentCommand(const GetUnitCurrentCommandQuery* query, GetUnitCurrentCommandResult* result)
{
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
		BeginCommandStorage(1);
		if (!ConvertCommand(queue[cmdIndex], commandStorage[0])) {
			return;
		}
		result->command = commandStorage[0];
		result->hasCommand = true;
	}
}

static void NativeGetFactoryCounts(const GetFactoryCountsQuery* query, GetFactoryCountsResult* result)
{
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

	integerStorage.clear();
	countStorage.clear();
	integerStorage.reserve(cmdCounts.size());
	countStorage.reserve(cmdCounts.size());

	// Fill arrays
	uint32_t idx = 0;
	uint32_t totalCount = 0;
	for (const auto& pair : cmdCounts) {
		integerStorage.push_back(pair.first);
		countStorage.push_back(pair.second);
		totalCount += pair.second;
		idx++;
	}

	result->info.unitDefIDs = integerStorage.data();
	result->info.counts = countStorage.data();
	result->info.uniqueCount = idx;
	result->info.totalCount = totalCount;
	result->info.currentCount = (unit->beingBuilt) ? 0 : 1; // Simplified
}

static void NativeGetFactoryCommandCount(const GetFactoryCommandCountQuery* query, GetFactoryCommandCountResult* result)
{
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
	BeginCommandStorage(count);

	if (count > 0) {
		for (uint32_t i = 0; i < count; ++i) {
			if (!ConvertCommand(queue[i], commandStorage[i])) {
				result->count = i;
				return;
			}
		}
		result->commands = commandStorage.data();
	}

	result->count = count;
}

static void NativeGetFactoryBuggerOff(const GetFactoryBuggerOffQuery* query, GetFactoryBuggerOffResult* result)
{
	result->error = nullptr;
	result->perform = false;
	result->offset = 0.0f;
	result->radius = 0.0f;
	result->relHeading = 0;
	result->spherical = false;
	result->forced = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const CFactory* factory = dynamic_cast<const CFactory*>(unit);
	if (factory == nullptr) {
		result->error = &NOT_FACTORY_ERROR;
		return;
	}

	result->perform = factory->boPerform;
	result->offset = factory->boOffset;
	result->radius = factory->boRadius;
	result->relHeading = factory->boRelHeading;
	result->spherical = factory->boSherical;
	result->forced = factory->boForced;
}

static void NativeGetCommandQueue(const GetCommandQueueQuery* query, GetCommandQueueResult* result)
{
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
	BeginCommandStorage(count);

	if (count > 0) {
		for (uint32_t i = 0; i < count; ++i) {
			if (!ConvertCommand(queue[i], commandStorage[i])) {
				result->count = i;
				return;
			}
		}
		result->commands = commandStorage.data();
	}

	result->count = count;
}

static void NativeGetFullBuildQueue(const GetFullBuildQueueQuery* query, GetFullBuildQueueResult* result)
{
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

	buildQueueStorage.clear();
	buildQueueStorage.reserve(buildOptions.size());

	uint32_t idx = 0;
	for (const auto& pair : buildOptions) {
		buildQueueStorage.push_back({
			.unitDefID = -pair.first,
			.numOrdered = static_cast<uint32_t>(pair.second),
		});
		idx++;
	}

	result->entries = buildQueueStorage.data();
	result->count = idx;
}

static void NativeGetRealBuildQueue(const GetRealBuildQueueQuery* query, GetRealBuildQueueResult* result)
{
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

	integerStorage.clear();
	integerStorage.reserve(buildCount);

	uint32_t idx = 0;
	for (const auto& cmd : queue) {
		if (cmd.GetID() < 0) {
			integerStorage.push_back(cmd.GetID());
			idx++;
		}
	}

	result->unitDefIDs = integerStorage.data();
	result->count = idx;
}

static void NativeGetUnitCmdDescs(const GetUnitCmdDescsQuery* query, GetUnitCmdDescsResult* result)
{
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
	commandDescriptionStorage.clear();
	descriptionStringStorage.clear();
	descriptionParameterStorage.clear();
	if (possibleCmds.empty()) {
		return;
	}

	commandDescriptionStorage.resize(possibleCmds.size());
	descriptionStringStorage.reserve(possibleCmds.size());
	descriptionParameterStorage.reserve(possibleCmds.size());

	for (size_t i = 0; i < possibleCmds.size(); ++i) {
		const SCommandDescription* desc = possibleCmds[i];
		CommandDescription& outDesc = commandDescriptionStorage[i];
		auto& strings = descriptionStringStorage.emplace_back();
		strings.reserve(5 + desc->params.size());
		auto storeString = [&strings](const std::string& value) {
			strings.push_back(value);
			return strings.back().c_str();
		};

		outDesc.cmdID = desc->id;
		outDesc.action = storeString(desc->action);
		outDesc.type = desc->type;
		outDesc.name = storeString(desc->name);
		outDesc.tooltip = storeString(desc->tooltip);
		outDesc.texture = storeString(desc->iconname);
		outDesc.cursor = storeString(desc->mouseicon);
		outDesc.queueing = desc->queueing;
		outDesc.hidden = desc->hidden;
		outDesc.disabled = desc->disabled;
		outDesc.showUnique = desc->showUnique;
		outDesc.onlyTexture = desc->onlyTexture;

		if (!desc->params.empty()) {
			auto& params = descriptionParameterStorage.emplace_back();
			params.reserve(desc->params.size());
			for (const auto& param : desc->params) {
				strings.push_back(param);
				params.push_back(strings.back().c_str());
			}
			outDesc.params = params.data();
			outDesc.paramCount = desc->params.size();
		} else {
			outDesc.params = nullptr;
			outDesc.paramCount = 0;
		}

	}

	result->cmdDescs = commandDescriptionStorage.data();
	result->count = possibleCmds.size();
}

static void NativeFindUnitCmdDesc(const FindUnitCmdDescQuery* query, FindUnitCmdDescResult* result)
{
	result->error = nullptr;
	result->cmdIndex = -1;
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

static void NativeGetCommandParams(const GetCommandParamsQuery* query, GetCommandParamsResult* result)
{
	result->error = nullptr;
	result->params = nullptr;
	result->count = 0;

	if (query->command == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const CommandFFI& command = *query->command;
	if (command.paramCount == 0 || command.params == nullptr) {
		return;
	}

	parameterStorage.assign(command.params, command.params + command.paramCount);
	result->params = parameterStorage.data();
	result->count = command.paramCount;
}

static void NativeGiveOrder(const GiveOrderQuery* query, GiveOrderResult* result)
{
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
	.GetCommandParams = NativeGetCommandParams,
	.GiveOrder = NativeGiveOrder,
	.GiveOrderToUnitMap = NativeGiveOrderToUnitMap,
	.GiveOrderArrayToUnitMap = NativeGiveOrderArrayToUnitMap,
};
