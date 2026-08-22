/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreUnitsQueryBorrowedBindings.h"

#include <bit>
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

enum class BorrowResult {
	Ok,
	InvalidArgument,
	OutOfBounds,
	NotAvailable,
};

BorrowResult BorrowI32Slice(HostState* state, std::uint32_t pointer, std::uint32_t count,
	std::span<const std::int32_t>& values)
{
	if constexpr (std::endian::native != std::endian::little)
		return BorrowResult::NotAvailable;
	if (count == 0) {
		values = {};
		return BorrowResult::Ok;
	}
	if ((pointer & (alignof(std::int32_t) - 1u)) != 0)
		return BorrowResult::InvalidArgument;
	const std::uint64_t bytes64 = static_cast<std::uint64_t>(count) * sizeof(std::int32_t);
	if (bytes64 > std::numeric_limits<std::size_t>::max())
		return BorrowResult::InvalidArgument;
	std::span<const std::uint8_t> bytes;
	if (!state->memory.View(pointer, static_cast<std::size_t>(bytes64), bytes))
		return BorrowResult::OutOfBounds;
	values = std::span<const std::int32_t>(
		reinterpret_cast<const std::int32_t*>(bytes.data()), count);
	return BorrowResult::Ok;
}

std::int32_t BorrowStatus(BorrowResult result)
{
	switch (result) {
		case BorrowResult::Ok: return 0;
		case BorrowResult::InvalidArgument: return static_cast<std::int32_t>(Status::InvalidArgument);
		case BorrowResult::OutOfBounds: return static_cast<std::int32_t>(Status::OutOfBounds);
		case BorrowResult::NotAvailable: return static_cast<std::int32_t>(Status::NotAvailable);
	}
	return static_cast<std::int32_t>(Status::Internal);
}

void ReturnList(wasmtime_val_raw_t* slots, HostState* state, const Error* nativeError,
	const std::int32_t* values, std::uint32_t count,
	std::uint32_t output, std::uint32_t capacity)
{
	const std::int32_t errorCode = NativeErrorCode(nativeError);
	if (errorCode != 0) {
		slots[0].i64 = static_cast<std::int64_t>(PackU32(0, errorCode));
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
	if (!state->memory.WriteI32SliceLE(output,
			std::span<const std::int32_t>(values, count))) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return;
	}
	slots[0].i64 = static_cast<std::int64_t>(PackU32(count, 0));
}

bool WriteFloat3(HostState* state, std::uint32_t output, const Float3& value)
{
	std::span<std::uint8_t> bytes;
	if (!state->memory.MutableView(output, 12u, bytes))
		return false;
	WireWriter writer(bytes);
	return writer.F32(value.x) && writer.F32(value.y) && writer.F32(value.z) && writer.Finish(4);
}

wasm_trap_t* GetTeamUnitsByDefs(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitsQuery == nullptr ||
		state->native->unitsQuery->GetTeamUnitsByDefs == nullptr)
		return Trap("GetTeamUnitsByDefs Core binding is unavailable");
	if (slots == nullptr || slotCount != 5)
		return Trap("GetTeamUnitsByDefs Core ABI signature mismatch");
	ImportGuard guard(state, 6);
	if (!guard.Ok())
		return Trap(guard.Error());

	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);

	const std::uint32_t input = static_cast<std::uint32_t>(slots[1].i32);
	const std::uint32_t inputCount = static_cast<std::uint32_t>(slots[2].i32);
	const std::uint32_t output = static_cast<std::uint32_t>(slots[3].i32);
	const std::uint32_t capacity = static_cast<std::uint32_t>(slots[4].i32);
	std::span<const std::int32_t> unitDefIDs;
	const BorrowResult borrowed = BorrowI32Slice(state, input, inputCount, unitDefIDs);
	if (borrowed != BorrowResult::Ok) {
		slots[0].i64 = static_cast<std::int64_t>(PackU32(0, BorrowStatus(borrowed)));
		return nullptr;
	}
	const std::uint64_t outputBytes = static_cast<std::uint64_t>(capacity) * sizeof(std::int32_t);
	if (outputBytes > std::numeric_limits<std::size_t>::max() ||
		!state->memory.Contains(output, static_cast<std::size_t>(outputBytes))) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return nullptr;
	}

	GetTeamUnitsByDefsQuery query{};
	query.teamID = slots[0].i32;
	query.unitDefIDs = unitDefIDs.empty() ? nullptr : unitDefIDs.data();
	query.defCount = inputCount;
	GetTeamUnitsByDefsResult result{};
	state->native->unitsQuery->GetTeamUnitsByDefs(&query, &result);
	ReturnList(slots, state, result.error, result.units, result.count, output, capacity);
	return nullptr;
}

template<typename Query, typename Result>
wasm_trap_t* Centroid(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount,
	void (*callback)(const Query*, Result*), const char* unavailable)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitsQuery == nullptr ||
		callback == nullptr)
		return Trap(unavailable);
	if (slots == nullptr || slotCount != 3)
		return Trap("unit centroid Core ABI signature mismatch");
	ImportGuard guard(state, 4);
	if (!guard.Ok())
		return Trap(guard.Error());

	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);

	const std::uint32_t input = static_cast<std::uint32_t>(slots[0].i32);
	const std::uint32_t count = static_cast<std::uint32_t>(slots[1].i32);
	const std::uint32_t output = static_cast<std::uint32_t>(slots[2].i32);
	std::span<const std::int32_t> unitIDs;
	const BorrowResult borrowed = BorrowI32Slice(state, input, count, unitIDs);
	if (borrowed != BorrowResult::Ok) {
		slots[0].i32 = BorrowStatus(borrowed);
		return nullptr;
	}
	if (!state->memory.Contains(output, 12u)) {
		slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
		return nullptr;
	}

	Query query{};
	query.unitIDs = unitIDs.empty() ? nullptr : unitIDs.data();
	query.count = count;
	Result result{};
	callback(&query, &result);
	const std::int32_t nativeError = NativeErrorCode(result.error);
	if (nativeError != 0) {
		slots[0].i32 = nativeError;
		return nullptr;
	}
	if (!WriteFloat3(state, output, result.centroid)) {
		slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
		return nullptr;
	}
	slots[0].i32 = 0;
	return nullptr;
}

wasm_trap_t* GetUnitArrayCentroid(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	return Centroid<GetUnitArrayCentroidQuery, GetUnitArrayCentroidResult>(
		environment, caller, slots, slotCount,
		state != nullptr && state->native != nullptr && state->native->unitsQuery != nullptr
			? state->native->unitsQuery->GetUnitArrayCentroid : nullptr,
		"GetUnitArrayCentroid Core binding is unavailable");
}

wasm_trap_t* GetUnitMapCentroid(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	return Centroid<GetUnitMapCentroidQuery, GetUnitMapCentroidResult>(
		environment, caller, slots, slotCount,
		state != nullptr && state->native != nullptr && state->native->unitsQuery != nullptr
			? state->native->unitsQuery->GetUnitMapCentroid : nullptr,
		"GetUnitMapCentroid Core binding is unavailable");
}

bool Define(wasmtime_linker_t* linker, const char* name, wasm_functype_t* type,
	wasmtime_func_unchecked_callback_t callback, HostState* state, std::string& error)
{
	wasmtime_error_t* defineError = wasmtime_linker_define_func_unchecked(
		linker, "spring:units-query", 18,
		name, std::char_traits<char>::length(name), type, callback, state, nullptr);
	wasm_functype_delete(type);
	if (defineError == nullptr)
		return true;
	error = ErrorMessage(defineError);
	return false;
}

} // namespace

bool RegisterUnitsQueryBorrowedImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error)
{
	if (linker == nullptr || state == nullptr || state->native == nullptr ||
		state->native->unitsQuery == nullptr) {
		error = "cannot register borrowed UnitsQuery Core imports without linker/host/API";
		return false;
	}
	const wasm_valkind_t i64Result[] = {WASM_I64};
	const wasm_valkind_t i32Result[] = {WASM_I32};
	const wasm_valkind_t byDefs[] = {WASM_I32, WASM_I32, WASM_I32, WASM_I32, WASM_I32};
	const wasm_valkind_t centroid[] = {WASM_I32, WASM_I32, WASM_I32};
	if (!Define(linker, "get-team-units-by-defs",
			MakeFuncType(byDefs, 5, i64Result, 1), GetTeamUnitsByDefs, state, error))
		return false;
	if (!Define(linker, "get-unit-array-centroid",
			MakeFuncType(centroid, 3, i32Result, 1), GetUnitArrayCentroid, state, error))
		return false;
	return Define(linker, "get-unit-map-centroid",
		MakeFuncType(centroid, 3, i32Result, 1), GetUnitMapCentroid, state, error);
}

#endif

} // namespace recoil::wasm::core
