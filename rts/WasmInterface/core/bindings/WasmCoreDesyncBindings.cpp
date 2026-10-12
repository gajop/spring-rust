/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

// The `spring:desync` group deliberately exposes nondeterministic host data to
// synced guests. Reading a wall clock from synced code makes the simulation
// diverge between clients, which is why the same timers are unsynced-only in
// `spring:profiling`.
//
// The import namespace remains visible so module validation/configuration does
// not depend on a process-local environment variable. Actual clock reads in a
// synced environment are disabled by default and require the same explicit
// diagnostic opt-in as the Component path: SPRING_ENABLE_SYNCED_TIMERS=1.
//
// This is a sync hazard only. It grants no sandbox escape, no OS authority and
// no hidden game state, so it is not a safety, security or visibility
// exception -- do not add anything to this group that would be.

#include "WasmCoreDesyncBindings.h"

#include <bit>
#include <array>
#include <cstdint>
#include <string>

#include "System/SyncedTiming.h"
#include "WasmCoreGeneratedSupport.h"

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)
namespace {

using generated::ImportGuard;
using generated::Trap;

bool TimerAllowed(const HostState* state)
{
	const bool synced = state != nullptr &&
		WasmEnvironmentMatrix::Policy(state->environment).synced;
	return spring::synced_timing::IsAllowed(
		synced, true, spring::synced_timing::IsEnabled());
}

wasm_trap_t* GetTimer(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->profiling == nullptr ||
		state->native->profiling->GetTimer == nullptr)
		return Trap("desync get-timer Core binding is unavailable");
	if (slots == nullptr || slotCount != 1)
		return Trap("desync get-timer Core ABI signature mismatch");
	ImportGuard guard(state, 1);
	if (!guard.Ok())
		return Trap(guard.Error());
	if (!TimerAllowed(state)) {
		// Legacy scalar ABI has no status slot. Return deterministic zero so
		// already-built profiling guests can degrade without losing their rules.
		slots[0].i64 = 0;
		return nullptr;
	}
	GetTimerQuery query{};
	GetTimerResult result{};
	state->native->profiling->GetTimer(&query, &result);
	if (result.error != nullptr)
		return Trap("desync get-timer unexpectedly returned a native error");
	slots[0].i64 = static_cast<std::int64_t>(result.timer);
	return nullptr;
}

wasm_trap_t* GetTimerMicros(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->profiling == nullptr ||
		state->native->profiling->GetTimerMicros == nullptr)
		return Trap("desync get-timer-micros Core binding is unavailable");
	if (slots == nullptr || slotCount != 1)
		return Trap("desync get-timer-micros Core ABI signature mismatch");
	ImportGuard guard(state, 1);
	if (!guard.Ok())
		return Trap(guard.Error());
	if (!TimerAllowed(state)) {
		slots[0].i64 = 0;
		return nullptr;
	}
	GetTimerMicrosQuery query{};
	GetTimerMicrosResult result{};
	state->native->profiling->GetTimerMicros(&query, &result);
	if (result.error != nullptr)
		return Trap("desync get-timer-micros unexpectedly returned a native error");
	slots[0].i64 = static_cast<std::int64_t>(result.timer);
	return nullptr;
}

template<bool Micros>
wasm_trap_t* TryGetTimer(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || slots == nullptr || slotCount != 1)
		return Trap("desync checked timer Core ABI signature mismatch");
	ImportGuard guard(state, 9);
	if (!guard.Ok())
		return Trap(guard.Error());
	const auto pointer = static_cast<std::uint32_t>(slots[0].i32);
	const auto status = [&]() -> std::int32_t {
		if (!TimerAllowed(state) || state->native == nullptr || state->native->profiling == nullptr)
			return static_cast<std::int32_t>(Status::NotAvailable);
		if (!state->memory.Contains(pointer, sizeof(std::uint64_t)))
			return static_cast<std::int32_t>(Status::OutOfBounds);
		std::uint64_t timer = 0;
		if constexpr (Micros) {
			if (state->native->profiling->GetTimerMicros == nullptr)
				return static_cast<std::int32_t>(Status::NotAvailable);
			GetTimerMicrosQuery query{};
			GetTimerMicrosResult result{};
			state->native->profiling->GetTimerMicros(&query, &result);
			if (result.error != nullptr) return result.error->code;
			timer = result.timer;
		} else {
			if (state->native->profiling->GetTimer == nullptr)
				return static_cast<std::int32_t>(Status::NotAvailable);
			GetTimerQuery query{};
			GetTimerResult result{};
			state->native->profiling->GetTimer(&query, &result);
			if (result.error != nullptr) return result.error->code;
			timer = result.timer;
		}
		std::array<std::uint8_t, 8> bytes{};
		for (std::size_t i = 0; i < bytes.size(); ++i) bytes[i] = static_cast<std::uint8_t>(timer >> (8 * i));
		return static_cast<std::int32_t>(state->memory.Write(pointer, bytes.data(), bytes.size()) ? Status::Ok : Status::OutOfBounds);
	}();
	slots[0].i32 = status;
	return nullptr;
}

wasm_trap_t* DiffTimers(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->profiling == nullptr ||
		state->native->profiling->DiffTimers == nullptr)
		return Trap("desync diff-timers Core binding is unavailable");
	if (slots == nullptr || slotCount != 4)
		return Trap("desync diff-timers Core ABI signature mismatch");
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
	slots[0].i64 = static_cast<std::int64_t>(PackU32(
		std::bit_cast<std::uint32_t>(result.seconds),
		result.error == nullptr ? 0 : result.error->code));
	return nullptr;
}

bool Define(wasmtime_linker_t* linker, const char* name, wasm_functype_t* type,
	wasmtime_func_unchecked_callback_t callback, HostState* state, std::string& error)
{
	wasmtime_error_t* defineError = wasmtime_linker_define_func_unchecked(
		linker, "spring:desync", 13,
		name, std::char_traits<char>::length(name), type, callback, state, nullptr);
	wasm_functype_delete(type);
	if (defineError == nullptr)
		return true;
	error = ErrorMessage(defineError);
	return false;
}

} // namespace

bool RegisterDesyncImports(wasmtime_linker_t* linker, HostState* state, std::string& error)
{
	if (linker == nullptr || state == nullptr || state->native == nullptr ||
		state->native->profiling == nullptr) {
		error = "cannot register desync Core imports without linker/host/native API";
		return false;
	}
	const wasm_valkind_t i64[] = {WASM_I64};
	const wasm_valkind_t i32[] = {WASM_I32};
	const wasm_valkind_t twoI64TwoI32[] = {WASM_I64, WASM_I64, WASM_I32, WASM_I32};
	if (!Define(linker, "get-timer", MakeFuncType(nullptr, 0, i64, 1),
			GetTimer, state, error) ||
		!Define(linker, "get-timer-micros", MakeFuncType(nullptr, 0, i64, 1),
			GetTimerMicros, state, error) ||
		!Define(linker, "try-get-timer", MakeFuncType(i32, 1, i32, 1),
			TryGetTimer<false>, state, error) ||
		!Define(linker, "try-get-timer-micros", MakeFuncType(i32, 1, i32, 1),
			TryGetTimer<true>, state, error))
		return false;
	return Define(linker, "diff-timers", MakeFuncType(twoI64TwoI32, 4, i64, 1),
		DiffTimers, state, error);
}

#endif

} // namespace recoil::wasm::core
