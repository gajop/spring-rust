/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreProfilingBindings.h"

#include <array>
#include <bit>
#include <cstdint>
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

void ReturnF32(wasmtime_val_raw_t* slots, float value, const Error* error)
{
	slots[0].i64 = static_cast<std::int64_t>(PackU32(
		std::bit_cast<std::uint32_t>(value), NativeErrorCode(error)));
}

bool WriteF32Values(HostState* state, std::uint32_t output, std::span<const float> values)
{
	std::span<std::uint8_t> bytes;
	if (!state->memory.MutableView(output, values.size() * sizeof(float), bytes))
		return false;
	WireWriter writer(bytes);
	for (float value : values) {
		if (!writer.F32(value))
			return false;
	}
	return writer.Finish(4);
}

bool WriteU64Value(HostState* state, std::uint32_t output, std::uint64_t value)
{
	std::span<std::uint8_t> bytes;
	if (!state->memory.MutableView(output, sizeof(value), bytes))
		return false;
	WireWriter writer(bytes);
	return writer.U64(value) && writer.Finish(8);
}

wasm_trap_t* GetTimer(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->profiling == nullptr ||
		state->native->profiling->GetTimer == nullptr)
		return Trap("GetTimer Core binding is unavailable");
	if (slots == nullptr || slotCount != 2)
		return Trap("GetTimer Core ABI signature mismatch");
	ImportGuard guard(state, 2);
	if (!guard.Ok())
		return Trap(guard.Error());
	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);
	const auto output = static_cast<std::uint32_t>(slots[1].i32);
	if (!state->memory.Contains(output, sizeof(std::uint64_t))) {
		slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
		return nullptr;
	}
	GetTimerQuery query{};
	GetTimerResult result{};
	state->native->profiling->GetTimer(&query, &result);
	if (result.error != nullptr) {
		slots[0].i32 = result.error->code;
		return nullptr;
	}
	if (!WriteU64Value(state, output, result.timer))
		return Trap("GetTimer Core output range changed unexpectedly");
	slots[0].i32 = 0;
	return nullptr;
}

wasm_trap_t* GetTimerMicros(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->profiling == nullptr ||
		state->native->profiling->GetTimerMicros == nullptr)
		return Trap("GetTimerMicros Core binding is unavailable");
	if (slots == nullptr || slotCount != 2)
		return Trap("GetTimerMicros Core ABI signature mismatch");
	ImportGuard guard(state, 2);
	if (!guard.Ok())
		return Trap(guard.Error());
	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);
	const auto output = static_cast<std::uint32_t>(slots[1].i32);
	if (!state->memory.Contains(output, sizeof(std::uint64_t))) {
		slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
		return nullptr;
	}
	GetTimerMicrosQuery query{};
	GetTimerMicrosResult result{};
	state->native->profiling->GetTimerMicros(&query, &result);
	if (result.error != nullptr) {
		slots[0].i32 = result.error->code;
		return nullptr;
	}
	if (!WriteU64Value(state, output, result.timer))
		return Trap("GetTimerMicros Core output range changed unexpectedly");
	slots[0].i32 = 0;
	return nullptr;
}

wasm_trap_t* DiffTimers(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->profiling == nullptr ||
		state->native->profiling->DiffTimers == nullptr)
		return Trap("DiffTimers Core binding is unavailable");
	if (slots == nullptr || slotCount != 4)
		return Trap("DiffTimers Core ABI signature mismatch");
	ImportGuard guard(state, 5);
	if (!guard.Ok())
		return Trap(guard.Error());
	DiffTimersQuery query{};
	query.endTimer = static_cast<std::uint64_t>(slots[0].i64);
	query.startTimer = static_cast<std::uint64_t>(slots[1].i64);
	query.options.returnMs = slots[2].i32 != 0;
	query.options.fromMicroSecs = slots[3].i32 != 0;
	DiffTimersResult result{};
	state->native->profiling->DiffTimers(&query, &result);
	ReturnF32(slots, result.seconds, result.error);
	return nullptr;
}

wasm_trap_t* GetFrameTimer(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->profiling == nullptr ||
		state->native->profiling->GetFrameTimer == nullptr)
		return Trap("GetFrameTimer Core binding is unavailable");
	if (slots == nullptr || slotCount != 2)
		return Trap("GetFrameTimer Core ABI signature mismatch");
	ImportGuard guard(state, 2);
	if (!guard.Ok())
		return Trap(guard.Error());
	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);
	const auto output = static_cast<std::uint32_t>(slots[1].i32);
	if (!state->memory.Contains(output, sizeof(std::uint64_t))) {
		slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
		return nullptr;
	}
	GetFrameTimerQuery query{slots[0].i32 != 0};
	GetFrameTimerResult result{};
	state->native->profiling->GetFrameTimer(&query, &result);
	if (result.error != nullptr) {
		slots[0].i32 = result.error->code;
		return nullptr;
	}
	if (!WriteU64Value(state, output, result.timer))
		return Trap("GetFrameTimer Core output range changed unexpectedly");
	slots[0].i32 = 0;
	return nullptr;
}

wasm_trap_t* GetDrawSeconds(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->profiling == nullptr ||
		state->native->profiling->GetDrawSeconds == nullptr)
		return Trap("GetDrawSeconds Core binding is unavailable");
	if (slots == nullptr || slotCount != 1)
		return Trap("GetDrawSeconds Core ABI signature mismatch");
	ImportGuard guard(state, 2);
	if (!guard.Ok())
		return Trap(guard.Error());
	GetDrawSecondsQuery query{};
	GetDrawSecondsResult result{};
	state->native->profiling->GetDrawSeconds(&query, &result);
	ReturnF32(slots, result.seconds, result.error);
	return nullptr;
}

wasm_trap_t* GetLuaMemUsage(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->profiling == nullptr ||
		state->native->profiling->GetLuaMemUsage == nullptr)
		return Trap("GetLuaMemUsage Core binding is unavailable");
	if (slots == nullptr || slotCount != 2)
		return Trap("GetLuaMemUsage Core ABI signature mismatch");
	ImportGuard guard(state, 4);
	if (!guard.Ok())
		return Trap(guard.Error());
	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);
	const std::uint32_t output = static_cast<std::uint32_t>(slots[1].i32);
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
	if (!WriteF32Values(state, output, values)) {
		slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
		return nullptr;
	}
	slots[0].i32 = 0;
	return nullptr;
}

wasm_trap_t* GetVidMemUsage(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->profiling == nullptr ||
		state->native->profiling->GetVidMemUsage == nullptr)
		return Trap("GetVidMemUsage Core binding is unavailable");
	if (slots == nullptr || slotCount != 2)
		return Trap("GetVidMemUsage Core ABI signature mismatch");
	ImportGuard guard(state, 2);
	if (!guard.Ok())
		return Trap(guard.Error());
	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);
	const std::uint32_t output = static_cast<std::uint32_t>(slots[1].i32);
	if (!state->memory.Contains(output, 2u * sizeof(float))) {
		slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
		return nullptr;
	}
	GetVidMemUsageQuery query{};
	GetVidMemUsageResult result{};
	state->native->profiling->GetVidMemUsage(&query, &result);
	const std::int32_t nativeError = NativeErrorCode(result.error);
	if (nativeError != 0) {
		slots[0].i32 = nativeError;
		return nullptr;
	}
	const float values[] = {result.usedMB, result.availableMB};
	if (!WriteF32Values(state, output, values)) {
		slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
		return nullptr;
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
	ReturnF32(slots, result.gcKB, result.error);
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
	if (linker == nullptr || state == nullptr || state->native == nullptr ||
		state->native->profiling == nullptr) {
		error = "cannot register Profiling Core imports without linker/host/native API";
		return false;
	}
	const wasm_valkind_t i32[] = {WASM_I32};
	const wasm_valkind_t twoI32[] = {WASM_I32, WASM_I32};
	const wasm_valkind_t i64[] = {WASM_I64};
	const wasm_valkind_t twoI64TwoI32[] = {WASM_I64, WASM_I64, WASM_I32, WASM_I32};
	if (!Define(linker, "get-timer", MakeFuncType(twoI32, 2, i32, 1),
			GetTimer, state, error) ||
		!Define(linker, "get-timer-micros", MakeFuncType(twoI32, 2, i32, 1),
			GetTimerMicros, state, error) ||
		!Define(linker, "diff-timers", MakeFuncType(twoI64TwoI32, 4, i64, 1),
			DiffTimers, state, error) ||
		!Define(linker, "get-frame-timer", MakeFuncType(twoI32, 2, i32, 1),
			GetFrameTimer, state, error) ||
		!Define(linker, "get-draw-seconds", MakeFuncType(i32, 1, i64, 1),
			GetDrawSeconds, state, error) ||
		!Define(linker, "get-lua-mem-usage", MakeFuncType(twoI32, 2, i32, 1),
			GetLuaMemUsage, state, error) ||
		!Define(linker, "get-vid-mem-usage", MakeFuncType(twoI32, 2, i32, 1),
			GetVidMemUsage, state, error))
		return false;
	return Define(linker, "get-synced-gc-info",
		MakeFuncType(i32, 1, i64, 1), GetSyncedGCInfo, state, error);
}

#endif

} // namespace recoil::wasm::core
