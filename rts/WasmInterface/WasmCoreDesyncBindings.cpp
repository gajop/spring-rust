/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

// The `spring:desync` group deliberately exposes nondeterministic host data to
// synced guests. Reading a wall clock from synced code makes the simulation
// diverge between clients, which is why the same timers are unsynced-only in
// `spring:profiling`.
//
// The group exists because debugging and benchmarking synced code needs a
// clock, and a guest that wants to trade determinism for that is entitled to.
// The module name is the warning label: nothing here can be imported by
// accident, and an import list is easy to audit for it.
//
// This is a sync hazard only. It grants no sandbox escape, no OS authority and
// no hidden game state, so it is not a safety, security or visibility
// exception -- do not add anything to this group that would be.

#include "WasmCoreDesyncBindings.h"

#include <bit>
#include <cstdint>
#include <string>

#include "WasmCoreGeneratedSupport.h"

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)
namespace {

using generated::ImportGuard;
using generated::Trap;

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
	GetTimerMicrosQuery query{};
	GetTimerMicrosResult result{};
	state->native->profiling->GetTimerMicros(&query, &result);
	if (result.error != nullptr)
		return Trap("desync get-timer-micros unexpectedly returned a native error");
	slots[0].i64 = static_cast<std::int64_t>(result.timer);
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
	const wasm_valkind_t twoI64TwoI32[] = {WASM_I64, WASM_I64, WASM_I32, WASM_I32};
	if (!Define(linker, "get-timer", MakeFuncType(nullptr, 0, i64, 1),
			GetTimer, state, error) ||
		!Define(linker, "get-timer-micros", MakeFuncType(nullptr, 0, i64, 1),
			GetTimerMicros, state, error))
		return false;
	return Define(linker, "diff-timers", MakeFuncType(twoI64TwoI32, 4, i64, 1),
		DiffTimers, state, error);
}

#endif

} // namespace recoil::wasm::core
