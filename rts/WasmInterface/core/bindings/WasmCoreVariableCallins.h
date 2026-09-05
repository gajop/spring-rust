/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <string>
#include <vector>

#include "WasmCoreAbi.h"

struct AddConsoleLineQuery;
struct BoolCallinResult;
struct CommandNotifyQuery;
class WasmExecutionBudget;

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)

// All variable engine->guest callin paths share one per-thread scratch region,
// covering both hand-specialized and generated serializers.
//
// A callout made from inside a callin can make the engine raise another callin
// synchronously, so these nest. `used` is how many bytes the innermost active
// callin's payload occupies, which is what a nested callin has to preserve.
struct VariableCallinScratchState {
	bool inUse = false;
	std::uint32_t used = 0;
};

VariableCallinScratchState& VariableCallinScratch();

// Keeps an outer callin's scratch payload intact across a nested one.
//
// The region is a single fixed buffer, so a nested callin necessarily writes
// over the payload the outer callin is still borrowing as a slice. Rejecting
// the nested callin would silently drop the event, so the outer bytes are saved
// on entry and put back before control returns to the outer guest frame.
class ScratchReentryScope {
public:
	ScratchReentryScope(Memory& memory, std::uint32_t offset);
	~ScratchReentryScope();

	ScratchReentryScope(const ScratchReentryScope&) = delete;
	ScratchReentryScope& operator=(const ScratchReentryScope&) = delete;

	// Records the payload size a further nested callin would have to preserve.
	void SetUsed(std::size_t used);

private:
	Memory& memory;
	std::uint32_t offset;
	bool previousInUse;
	std::uint32_t previousUsed;
	std::vector<std::uint8_t> saved;
};

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

};

#endif

} // namespace recoil::wasm::core
