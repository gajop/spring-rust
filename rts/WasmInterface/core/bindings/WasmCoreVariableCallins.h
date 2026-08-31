/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <string>

#include "WasmCoreAbi.h"

struct AddConsoleLineQuery;
struct BoolCallinResult;
struct CommandNotifyQuery;
class WasmExecutionBudget;

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)

// All variable engine->guest callin paths use the same per-thread guard. This
// includes hand-specialized and generated scratch serializers. A nested event
// must never overwrite a scratch region while an outer guest still borrows it.
bool& VariableCallinScratchInUse();

// Variable-size engine -> guest callins use one guest-owned scratch region.
// The region is negotiated once at bind time through
// `spring:callin/scratch-info() -> i64` (offset in low 32 bits, capacity in
// high 32 bits). The hot path then consists of one bounded serialization into
// guest memory followed by exactly one unchecked host->guest call.
class VariableCallinBindings {
public:
	bool Bind(wasmtime_context_t* context, const wasmtime_instance_t& instance,
		Memory& memory, std::string& error);

	bool HasAddConsoleLine() const { return addConsoleLine.Present(); }
	bool HasCommandNotify() const { return commandNotify.Present(); }
	bool AnyPresent() const { return HasAddConsoleLine() || HasCommandNotify(); }

	bool AddConsoleLine(wasmtime_context_t* context, WasmExecutionBudget& budget,
		Memory& memory, const AddConsoleLineQuery& query, BoolCallinResult& result,
		std::string& error) const;
	bool CommandNotify(wasmtime_context_t* context, WasmExecutionBudget& budget,
		Memory& memory, const CommandNotifyQuery& query, BoolCallinResult& result,
		std::string& error) const;

	std::uint32_t ScratchOffset() const { return scratchOffset; }
	std::uint32_t ScratchCapacity() const { return scratchCapacity; }

private:
	bool CallBool(wasmtime_context_t* context, const RawExport& function,
		std::uint32_t usedBytes, BoolCallinResult& result, std::string& error) const;

	RawExport scratchInfo;
	RawExport addConsoleLine;
	RawExport commandNotify;
	std::uint32_t scratchOffset = 0;
	std::uint32_t scratchCapacity = 0;
	mutable bool scratchInUse = false;
};

#endif

} // namespace recoil::wasm::core
