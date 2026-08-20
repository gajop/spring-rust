/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreProfilingBindings.h"

#include <array>
#include <bit>
#include <cstdint>

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

wasm_trap_t* GetLuaMemUsage(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->profiling == nullptr ||
		state->native->profiling->GetLuaMemUsage == nullptr)
		return Trap("GetLuaMemUsage Core binding is unavailable");
	if (slots == nullptr || slotCount != 1)
		return Trap("GetLuaMemUsage Core ABI signature mismatch");
	ImportGuard guard(state, 4);
	if (!guard.Ok())
		return Trap(guard.Error());
	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);
	const std::uint32_t output = static_cast<std::uint32_t>(slots[0].i32);
	if (!state->memory.Contains(output, 8u * sizeof(float))) {
		slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
		return nullptr;
	}
	GetLuaMemUsageQuery query{};
	GetLuaMemUsageResult result{};
	state->native->profiling->GetLuaMemUsage(&query, &result);
	const std::int32_t nativeError = NativeErrorCode(result.error);
	if (nativeError != 0) {
		slots[0].i32 = nativeError;
		return nullptr;
	}
	const std::array<float, 8> values = {
		result.handleAllocedKB,
		result.handleAllocsK,
		result.globalAllocedKB,
		result.globalAllocsK,
		result.unsyncedAllocedKB,
		result.unsyncedAllocsK,
		result.syncedAllocedKB,
		result.syncedAllocsK,
	};
	// Fixed f32 records use Wasm's little-endian representation. All supported
	// production hosts are little-endian today; use per-value bit writes so the
	// ABI remains explicit and portable if that changes.
	for (std::size_t index = 0; index < values.size(); ++index) {
		const std::uint32_t bits = std::bit_cast<std::uint32_t>(values[index]);
		const std::uint8_t encoded[4] = {
			static_cast<std::uint8_t>(bits),
			static_cast<std::uint8_t>(bits >> 8),
			static_cast<std::uint8_t>(bits >> 16),
			static_cast<std::uint8_t>(bits >> 24),
		};
		if (!state->memory.Write(output + static_cast<std::uint32_t>(index * 4), encoded, 4)) {
			slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
			return nullptr;
		}
	}
	slots[0].i32 = 0;
	return nullptr;
}

wasm_trap_t* GetSyncedGCInfo(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->profiling == nullptr ||
		state->native->profiling->GetSyncedGCInfo == nullptr)
		return Trap("GetSyncedGCInfo Core binding is unavailable");
	if (slots == nullptr || slotCount != 1)
		return Trap("GetSyncedGCInfo Core ABI signature mismatch");
	ImportGuard guard(state, 2);
	if (!guard.Ok())
		return Trap(guard.Error());
	GetSyncedGCInfoQuery query{slots[0].i32 != 0};
	GetSyncedGCInfoResult result{};
	state->native->profiling->GetSyncedGCInfo(&query, &result);
	slots[0].i64 = static_cast<std::int64_t>(PackU32(
		std::bit_cast<std::uint32_t>(result.gcKB), NativeErrorCode(result.error)));
	return nullptr;
}

bool Define(wasmtime_linker_t* linker, const char* name, wasm_functype_t* type,
	wasmtime_func_unchecked_callback_t callback, HostState* state, std::string& error)
{
	wasmtime_error_t* defineError = wasmtime_linker_define_func_unchecked(
		linker, "spring:profiling", 16,
		name, std::char_traits<char>::length(name), type, callback, state, nullptr);
	wasm_functype_delete(type);
	if (defineError == nullptr)
		return true;
	error = ErrorMessage(defineError);
	return false;
}

} // namespace

bool RegisterProfilingImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error)
{
	if (linker == nullptr || state == nullptr || state->native == nullptr) {
		error = "cannot register Profiling Core imports without linker/host/native API";
		return false;
	}
	const wasm_valkind_t i32[] = {WASM_I32};
	const wasm_valkind_t i64[] = {WASM_I64};
	if (!Define(linker, "get-lua-mem-usage",
			MakeFuncType(i32, 1, i32, 1), GetLuaMemUsage, state, error))
		return false;
	return Define(linker, "get-synced-gc-info",
		MakeFuncType(i32, 1, i64, 1), GetSyncedGCInfo, state, error);
}

#endif

} // namespace recoil::wasm::core
