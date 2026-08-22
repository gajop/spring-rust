/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreCobScriptBindings.h"

#include <bit>
#include <cstdint>
#include <limits>
#include <span>

#include "WasmCoreGeneratedSupport.h"
#include "WasmCoreGuestInput.h"

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)
namespace {

using generated::ImportGuard;
using generated::Trap;

template<typename T>
Status BorrowInputSlice(HostState* state, std::uint32_t pointer, std::uint32_t count,
	std::span<const T>& values)
{
	if constexpr (std::endian::native != std::endian::little)
		return Status::NotAvailable;
	if (count == 0) {
		values = {};
		return Status::Ok;
	}
	if ((pointer % alignof(T)) != 0)
		return Status::InvalidArgument;
	const std::uint64_t bytes64 = static_cast<std::uint64_t>(count) * sizeof(T);
	if (bytes64 > std::numeric_limits<std::size_t>::max())
		return Status::InvalidArgument;
	std::span<const std::uint8_t> bytes;
	if (!state->memory.View(pointer, static_cast<std::size_t>(bytes64), bytes))
		return Status::OutOfBounds;
	values = std::span<const T>(reinterpret_cast<const T*>(bytes.data()), count);
	return Status::Ok;
}

wasm_trap_t* CallCOBScript(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->syncedCtrl == nullptr ||
		state->native->syncedCtrl->cobScript == nullptr ||
		state->native->syncedCtrl->cobScript->CallCOBScript == nullptr)
		return Trap("CallCOBScript Core binding is unavailable");
	if (slots == nullptr || slotCount != 10)
		return Trap("CallCOBScript Core ABI signature mismatch");

	ImportGuard guard(state, 12);
	if (!guard.Ok())
		return Trap(guard.Error());
	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);

	const std::int32_t unitID = slots[0].i32;
	const std::int32_t funcID = slots[1].i32;
	const std::uint32_t namePointer = static_cast<std::uint32_t>(slots[2].i32);
	const std::uint32_t nameLength = static_cast<std::uint32_t>(slots[3].i32);
	const std::uint32_t retArgs = static_cast<std::uint32_t>(slots[4].i32);
	const std::uint32_t argsPointer = static_cast<std::uint32_t>(slots[5].i32);
	const std::uint32_t argCount = static_cast<std::uint32_t>(slots[6].i32);
	const std::uint32_t retValuesPointer = static_cast<std::uint32_t>(slots[7].i32);
	const std::uint32_t retValuesCapacity = static_cast<std::uint32_t>(slots[8].i32);
	const std::uint32_t retCountPointer = static_cast<std::uint32_t>(slots[9].i32);

	// This call may mutate synced state, so reject insufficient output capacity
	// before invoking native code rather than relying on a probe/retry call.
	if (retValuesCapacity < retArgs) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackI32(0, static_cast<std::int32_t>(Status::BufferOverflow)));
		return nullptr;
	}
	const std::uint64_t capacityBytes64 =
		static_cast<std::uint64_t>(retValuesCapacity) * sizeof(std::int32_t);
	if (capacityBytes64 > std::numeric_limits<std::size_t>::max() ||
		!state->memory.Contains(retValuesPointer, static_cast<std::size_t>(capacityBytes64)) ||
		!state->memory.Contains(retCountPointer, sizeof(std::uint32_t))) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackI32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return nullptr;
	}

	GuestCString<> funcName;
	if (funcID < 0) {
		if (!guard.Charge(nameLength))
			return Trap(guard.Error());
		if (!funcName.Read(state->memory, namePointer, nameLength)) {
			slots[0].i64 = static_cast<std::int64_t>(
				PackI32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
			return nullptr;
		}
	}

	std::span<const std::int32_t> args;
	const Status argsStatus = BorrowInputSlice(state, argsPointer, argCount, args);
	if (argsStatus != Status::Ok) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackI32(0, static_cast<std::int32_t>(argsStatus)));
		return nullptr;
	}

	CobFunctionRef function{};
	function.name = (funcID < 0) ? funcName.c_str() : nullptr;
	function.id = funcID;
	CallCOBScriptQuery query{};
	query.unitID = unitID;
	query.func = function;
	query.retArgs = retArgs;
	query.args = args.data();
	query.argCount = argCount;
	CallCOBScriptResult result{};
	state->native->syncedCtrl->cobScript->CallCOBScript(&query, &result);
	const std::int32_t errorCode = generated::NativeErrorCode(result.error);
	if (errorCode != 0) {
		slots[0].i64 = static_cast<std::int64_t>(PackI32(result.retCode, errorCode));
		return nullptr;
	}
	if (result.retCount > retArgs || result.retCount > retValuesCapacity ||
		(result.retCount != 0 && result.retValues == nullptr)) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackI32(result.retCode, static_cast<std::int32_t>(Status::Internal)));
		return nullptr;
	}

	const std::uint64_t resultBytes64 =
		static_cast<std::uint64_t>(result.retCount) * sizeof(std::int32_t);
	if (resultBytes64 > std::numeric_limits<std::size_t>::max() ||
		!generated::CheckResultBytes(state, static_cast<std::size_t>(resultBytes64))) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackI32(result.retCode, static_cast<std::int32_t>(Status::BufferOverflow)));
		return nullptr;
	}
	if (!guard.Charge(resultBytes64))
		return Trap(guard.Error());
	if (result.retCount != 0 && !state->memory.Write(retValuesPointer, result.retValues,
			static_cast<std::size_t>(resultBytes64)))
		return Trap("CallCOBScript Core result range changed unexpectedly");
	if (!state->memory.Write(retCountPointer, &result.retCount, sizeof(result.retCount)))
		return Trap("CallCOBScript Core result-count range changed unexpectedly");

	slots[0].i64 = static_cast<std::int64_t>(PackI32(result.retCode, 0));
	return nullptr;
}

bool Define(wasmtime_linker_t* linker, const char* name, wasm_functype_t* type,
	wasmtime_func_unchecked_callback_t callback, HostState* state, std::string& error)
{
	constexpr char moduleName[] = "spring:cob-script";
	wasmtime_error_t* defineError = wasmtime_linker_define_func_unchecked(
		linker, moduleName, sizeof(moduleName) - 1,
		name, std::char_traits<char>::length(name), type, callback, state, nullptr);
	wasm_functype_delete(type);
	if (defineError == nullptr)
		return true;
	error = ErrorMessage(defineError);
	return false;
}

} // namespace

bool RegisterCobScriptImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error)
{
	if (linker == nullptr || state == nullptr || state->native == nullptr ||
		state->native->syncedCtrl == nullptr || state->native->syncedCtrl->cobScript == nullptr) {
		error = "cannot register COB-script Core imports without linker/host/API";
		return false;
	}
	const wasm_valkind_t params[] = {
		WASM_I32, WASM_I32, WASM_I32, WASM_I32, WASM_I32,
		WASM_I32, WASM_I32, WASM_I32, WASM_I32, WASM_I32,
	};
	const wasm_valkind_t result[] = {WASM_I64};
	return Define(linker, "call-cob-script", MakeFuncType(params, 10, result, 1),
		CallCOBScript, state, error);
}

#endif

} // namespace recoil::wasm::core
