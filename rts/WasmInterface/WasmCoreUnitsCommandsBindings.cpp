/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreUnitsCommandsBindings.h"

#include <cstdint>
#include <limits>
#include <span>

#include "WasmCoreGeneratedSupport.h"
#include "WasmCoreWire.h"

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)
namespace {

using generated::ImportGuard;
using generated::Trap;

std::int32_t NativeErrorCode(const Error* error)
{
	return error == nullptr ? 0 : error->code;
}

bool RequiredCommandBytes(const GetUnitCommandsResult& result, std::uint32_t& required)
{
	std::uint64_t bytes = 4; // command count
	if (result.count != 0 && result.commands == nullptr)
		return false;
	for (std::uint32_t index = 0; index < result.count; ++index) {
		const CommandFFI& command = result.commands[index];
		if (command.paramCount != 0 && command.params == nullptr)
			return false;
		// cmdID, options-as-u32, tag, aiCommandID, timeOut, paramCount.
		bytes += 24;
		bytes += static_cast<std::uint64_t>(command.paramCount) * sizeof(float);
		if (bytes > std::numeric_limits<std::uint32_t>::max())
			return false;
	}
	required = static_cast<std::uint32_t>(bytes);
	return true;
}

wasm_trap_t* GetUnitCommands(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitsCommands == nullptr ||
		state->native->unitsCommands->GetUnitCommands == nullptr)
		return Trap("GetUnitCommands Core binding is unavailable");
	if (slots == nullptr || slotCount != 4)
		return Trap("GetUnitCommands Core ABI signature mismatch");

	ImportGuard guard(state, 6);
	if (!guard.Ok())
		return Trap(guard.Error());

	const std::int32_t unitID = slots[0].i32;
	const std::uint32_t maxCommands = static_cast<std::uint32_t>(slots[1].i32);
	const std::uint32_t output = static_cast<std::uint32_t>(slots[2].i32);
	const std::uint32_t capacity = static_cast<std::uint32_t>(slots[3].i32);

	GetUnitCommandsQuery query{unitID, maxCommands};
	GetUnitCommandsResult result{};
	state->native->unitsCommands->GetUnitCommands(&query, &result);
	const std::int32_t nativeError = NativeErrorCode(result.error);
	if (nativeError != 0) {
		slots[0].i64 = static_cast<std::int64_t>(PackU32(0, nativeError));
		return nullptr;
	}

	std::uint32_t required = 0;
	if (!RequiredCommandBytes(result, required)) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::Internal)));
		return nullptr;
	}
	if (required > capacity) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(required, static_cast<std::int32_t>(Status::BufferOverflow)));
		return nullptr;
	}

	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);
	std::span<std::uint8_t> bytes;
	if (!state->memory.MutableView(output, required, bytes)) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(required, static_cast<std::int32_t>(Status::OutOfBounds)));
		return nullptr;
	}

	WireWriter writer(bytes);
	if (!writer.U32(result.count))
		return Trap("GetUnitCommands Core wire buffer unexpectedly too small");
	for (std::uint32_t index = 0; index < result.count; ++index) {
		const CommandFFI& command = result.commands[index];
		if (!writer.I32(command.cmdID) ||
			!writer.U32(static_cast<std::uint32_t>(command.options)) ||
			!writer.I32(command.tag) ||
			!writer.I32(command.aiCommandID) ||
			!writer.F32(command.timeOut) ||
			!writer.U32(command.paramCount))
			return Trap("GetUnitCommands Core wire encoding failed");
		for (std::uint32_t param = 0; param < command.paramCount; ++param) {
			if (!writer.F32(command.params[param]))
				return Trap("GetUnitCommands Core parameter encoding failed");
		}
	}
	if (!writer.Finish())
		return Trap("GetUnitCommands Core wire size mismatch");

	slots[0].i64 = static_cast<std::int64_t>(PackU32(required, 0));
	return nullptr;
}

wasm_trap_t* GetUnitCommandCount(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitsCommands == nullptr ||
		state->native->unitsCommands->GetUnitCommandCount == nullptr)
		return Trap("GetUnitCommandCount Core binding is unavailable");
	if (slots == nullptr || slotCount != 1)
		return Trap("GetUnitCommandCount Core ABI signature mismatch");

	ImportGuard guard(state, 2);
	if (!guard.Ok())
		return Trap(guard.Error());

	GetUnitCommandCountQuery query{slots[0].i32};
	GetUnitCommandCountResult result{};
	state->native->unitsCommands->GetUnitCommandCount(&query, &result);
	slots[0].i64 = static_cast<std::int64_t>(PackU32(result.count,
		NativeErrorCode(result.error)));
	return nullptr;
}

bool Define(wasmtime_linker_t* linker, const char* name, wasm_functype_t* type,
	wasmtime_func_unchecked_callback_t callback, HostState* state, std::string& error)
{
	wasmtime_error_t* defineError = wasmtime_linker_define_func_unchecked(
		linker, "spring:units-commands", 21,
		name, std::char_traits<char>::length(name), type, callback, state, nullptr);
	wasm_functype_delete(type);
	if (defineError == nullptr)
		return true;
	error = ErrorMessage(defineError);
	return false;
}

} // namespace

bool RegisterUnitsCommandsImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error)
{
	if (linker == nullptr || state == nullptr || state->native == nullptr ||
		state->native->unitsCommands == nullptr) {
		error = "cannot register UnitsCommands Core imports without linker/host/API";
		return false;
	}
	const wasm_valkind_t i64Result[] = {WASM_I64};
	const wasm_valkind_t countParams[] = {WASM_I32};
	if (!Define(linker, "get-unit-command-count",
			MakeFuncType(countParams, 1, i64Result, 1),
			GetUnitCommandCount, state, error))
		return false;
	const wasm_valkind_t commandParams[] = {WASM_I32, WASM_I32, WASM_I32, WASM_I32};
	return Define(linker, "get-unit-commands",
		MakeFuncType(commandParams, 4, i64Result, 1), GetUnitCommands, state, error);
}

#endif

} // namespace recoil::wasm::core
