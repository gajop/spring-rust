/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreUnitsInfoVariableBindings.h"

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

bool ValidateListOutput(HostState* state, std::uint32_t output, std::uint32_t capacity)
{
	const std::uint64_t bytes64 = static_cast<std::uint64_t>(capacity) * sizeof(std::int32_t);
	return bytes64 <= std::numeric_limits<std::size_t>::max() &&
		state->memory.Contains(output, static_cast<std::size_t>(bytes64));
}

void ReturnList(wasmtime_val_raw_t* slots, HostState* state, const Error* error,
	const std::int32_t* values, std::uint32_t count,
	std::uint32_t output, std::uint32_t capacity)
{
	const std::int32_t nativeError = NativeErrorCode(error);
	if (nativeError != 0) {
		slots[0].i64 = static_cast<std::int64_t>(PackU32(0, nativeError));
		return;
	}
	if (count > capacity) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(count, static_cast<std::int32_t>(Status::BufferOverflow)));
		return;
	}
	if (count != 0 && values == nullptr) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::Internal)));
		return;
	}
	if (!state->memory.WriteI32SliceLE(output, std::span<const std::int32_t>(values, count))) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return;
	}
	slots[0].i64 = static_cast<std::int64_t>(PackU32(count, 0));
}

bool WriteBool(HostState* state, std::uint32_t output, bool value)
{
	std::span<std::uint8_t> bytes;
	if (!state->memory.MutableView(output, 4u, bytes))
		return false;
	WireWriter writer(bytes);
	return writer.U32(value ? 1u : 0u) && writer.Finish(4);
}

wasm_trap_t* GetUnitNanoPieces(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitsInfo == nullptr ||
		state->native->unitsInfo->GetUnitNanoPieces == nullptr)
		return Trap("GetUnitNanoPieces Core binding is unavailable");
	if (slots == nullptr || slotCount != 3)
		return Trap("GetUnitNanoPieces Core ABI signature mismatch");
	ImportGuard guard(state, 4);
	if (!guard.Ok())
		return Trap(guard.Error());

	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);
	const std::uint32_t output = static_cast<std::uint32_t>(slots[1].i32);
	const std::uint32_t capacity = static_cast<std::uint32_t>(slots[2].i32);
	if (!ValidateListOutput(state, output, capacity)) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return nullptr;
	}

	GetUnitNanoPiecesQuery query{slots[0].i32};
	GetUnitNanoPiecesResult result{};
	state->native->unitsInfo->GetUnitNanoPieces(&query, &result);
	ReturnList(slots, state, result.error, result.pieces, result.count, output, capacity);
	return nullptr;
}

wasm_trap_t* GetUnitIsTransporting(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitsInfo == nullptr ||
		state->native->unitsInfo->GetUnitIsTransporting == nullptr)
		return Trap("GetUnitIsTransporting Core binding is unavailable");
	if (slots == nullptr || slotCount != 4)
		return Trap("GetUnitIsTransporting Core ABI signature mismatch");
	ImportGuard guard(state, 5);
	if (!guard.Ok())
		return Trap(guard.Error());

	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);
	const std::uint32_t output = static_cast<std::uint32_t>(slots[1].i32);
	const std::uint32_t capacity = static_cast<std::uint32_t>(slots[2].i32);
	const std::uint32_t stateOutput = static_cast<std::uint32_t>(slots[3].i32);
	if (!ValidateListOutput(state, output, capacity) || !state->memory.Contains(stateOutput, 4u)) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return nullptr;
	}

	GetUnitIsTransportingQuery query{slots[0].i32};
	GetUnitIsTransportingResult result{};
	state->native->unitsInfo->GetUnitIsTransporting(&query, &result);
	const std::int32_t nativeError = NativeErrorCode(result.error);
	if (nativeError != 0) {
		slots[0].i64 = static_cast<std::int64_t>(PackU32(0, nativeError));
		return nullptr;
	}
	if (!WriteBool(state, stateOutput, result.isTransporting)) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return nullptr;
	}
	ReturnList(slots, state, nullptr, result.unitIDs, result.count, output, capacity);
	return nullptr;
}

bool Define(wasmtime_linker_t* linker, const char* name, wasm_functype_t* type,
	wasmtime_func_unchecked_callback_t callback, HostState* state, std::string& error)
{
	wasmtime_error_t* defineError = wasmtime_linker_define_func_unchecked(
		linker, "spring:units-info", 17,
		name, std::char_traits<char>::length(name), type, callback, state, nullptr);
	wasm_functype_delete(type);
	if (defineError == nullptr)
		return true;
	error = ErrorMessage(defineError);
	return false;
}

} // namespace

bool RegisterUnitsInfoVariableImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error)
{
	if (linker == nullptr || state == nullptr || state->native == nullptr) {
		error = "cannot register variable UnitsInfo Core imports without linker/host/API";
		return false;
	}
	if (state->native->unitsInfo == nullptr)
		return true;
	const wasm_valkind_t nanoParams[] = {WASM_I32, WASM_I32, WASM_I32};
	const wasm_valkind_t transportParams[] = {WASM_I32, WASM_I32, WASM_I32, WASM_I32};
	const wasm_valkind_t result[] = {WASM_I64};
	if (!Define(linker, "get-unit-nano-pieces",
			MakeFuncType(nanoParams, 3, result, 1), GetUnitNanoPieces, state, error))
		return false;
	return Define(linker, "get-unit-is-transporting",
		MakeFuncType(transportParams, 4, result, 1), GetUnitIsTransporting, state, error);
}

#endif

} // namespace recoil::wasm::core
