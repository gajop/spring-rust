/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreUnitDefsBindings.h"

#include <cstdint>
#include <cstring>
#include <limits>
#include <string_view>

#include "WasmCoreGeneratedSupport.h"

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)
namespace {

using generated::ImportGuard;
using generated::Trap;

std::int32_t NativeErrorCode(const Error* error)
{
	return error == nullptr ? 0 : error->code;
}

bool PrepareByteBuffer(HostState* state, wasmtime_caller_t* caller,
	std::uint32_t output, std::uint32_t capacity, std::string& error)
{
	if (!generated::EnsureMemory(state, caller, error))
		return false;
	return state->memory.Contains(output, static_cast<std::size_t>(capacity));
}

void ReturnBytes(wasmtime_val_raw_t* slots, HostState* state, const Error* nativeError,
	const char* value, std::uint32_t output, std::uint32_t capacity)
{
	const std::int32_t errorCode = NativeErrorCode(nativeError);
	if (errorCode != 0) {
		slots[0].i64 = static_cast<std::int64_t>(PackU32(0, errorCode));
		return;
	}
	if (value == nullptr) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::Internal)));
		return;
	}
	const std::size_t length = std::strlen(value);
	if (length > std::numeric_limits<std::uint32_t>::max()) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::BufferOverflow)));
		return;
	}
	const std::uint32_t required = static_cast<std::uint32_t>(length);
	if (!generated::CheckResultBytes(state, required) || required > capacity) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(required, static_cast<std::int32_t>(Status::BufferOverflow)));
		return;
	}
	if (required != 0 && !state->memory.Write(output, value, required)) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return;
	}
	slots[0].i64 = static_cast<std::int64_t>(PackU32(required, 0));
}

wasm_trap_t* GetUnitDefName(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitDefs == nullptr ||
		state->native->unitDefs->GetUnitDefName == nullptr)
		return Trap("GetUnitDefName Core binding is unavailable");
	if (slots == nullptr || slotCount != 3)
		return Trap("GetUnitDefName Core ABI signature mismatch");
	ImportGuard guard(state, 4);
	if (!guard.Ok())
		return Trap(guard.Error());
	const std::uint32_t output = static_cast<std::uint32_t>(slots[1].i32);
	const std::uint32_t capacity = static_cast<std::uint32_t>(slots[2].i32);
	std::string memoryError;
	if (!PrepareByteBuffer(state, caller, output, capacity, memoryError)) {
		if (!memoryError.empty())
			return Trap(memoryError);
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return nullptr;
	}
	GetUnitDefNameQuery query{slots[0].i32};
	GetUnitDefNameResult result{};
	state->native->unitDefs->GetUnitDefName(&query, &result);
	ReturnBytes(slots, state, result.error, result.name, output, capacity);
	return nullptr;
}

wasm_trap_t* GetUnitDefHumanName(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitDefs == nullptr ||
		state->native->unitDefs->GetUnitDefHumanName == nullptr)
		return Trap("GetUnitDefHumanName Core binding is unavailable");
	if (slots == nullptr || slotCount != 3)
		return Trap("GetUnitDefHumanName Core ABI signature mismatch");
	ImportGuard guard(state, 4);
	if (!guard.Ok())
		return Trap(guard.Error());
	const std::uint32_t output = static_cast<std::uint32_t>(slots[1].i32);
	const std::uint32_t capacity = static_cast<std::uint32_t>(slots[2].i32);
	std::string memoryError;
	if (!PrepareByteBuffer(state, caller, output, capacity, memoryError)) {
		if (!memoryError.empty())
			return Trap(memoryError);
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return nullptr;
	}
	GetUnitDefHumanNameQuery query{slots[0].i32};
	GetUnitDefHumanNameResult result{};
	state->native->unitDefs->GetUnitDefHumanName(&query, &result);
	ReturnBytes(slots, state, result.error, result.humanName, output, capacity);
	return nullptr;
}

bool Define(wasmtime_linker_t* linker, const char* name,
	wasmtime_func_unchecked_callback_t callback, HostState* state, std::string& error)
{
	const wasm_valkind_t params[] = {WASM_I32, WASM_I32, WASM_I32};
	const wasm_valkind_t results[] = {WASM_I64};
	wasm_functype_t* type = MakeFuncType(params, 3, results, 1);
	wasmtime_error_t* defineError = wasmtime_linker_define_func_unchecked(
		linker, "spring:unit-defs", 16, name, std::char_traits<char>::length(name),
		type, callback, state, nullptr);
	wasm_functype_delete(type);
	if (defineError == nullptr)
		return true;
	error = ErrorMessage(defineError);
	return false;
}

} // namespace

bool RegisterUnitDefsImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error)
{
	if (linker == nullptr || state == nullptr || state->native == nullptr) {
		error = "cannot register UnitDefs Core imports without linker/host/API";
		return false;
	}
	if (state->native->unitDefs == nullptr)
		return true;
	return Define(linker, "get-unit-def-name", GetUnitDefName, state, error) &&
		Define(linker, "get-unit-def-human-name", GetUnitDefHumanName, state, error);
}

#endif

} // namespace recoil::wasm::core
