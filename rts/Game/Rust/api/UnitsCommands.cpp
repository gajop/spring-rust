#include "UnitsCommands.h"

namespace {

static const Error NOT_IMPLEMENTED_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "UnitsCommands API not yet fully implemented"
};

// Command queue management - complex Lua/C++ integration
static CommandResult NativeGetUnitCurrentCommand(int32_t unitID)
{
	CommandResult result = {};
	result.hasCommand = false;
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

static CommandArray NativeGetUnitCommands(int32_t unitID, uint32_t maxCommands)
{
	CommandArray result = {};
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

static CommandDescriptionArray NativeGetUnitCmdDescs(int32_t unitID)
{
	CommandDescriptionArray result = {};
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

} // namespace

const UnitsCommandsApi UNITS_COMMANDS_API = {
	.GetUnitCurrentCommand = NativeGetUnitCurrentCommand,
	.GetUnitCommands = NativeGetUnitCommands,
	.GetUnitCmdDescs = NativeGetUnitCmdDescs,
};
