/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreBindings.h"

#include <exception>

#include "WasmCoreGeneratedSupport.h"
#include "WasmCoreHost.h"
#include "WasmInterface/jobs/WasmSyncedJobs.h"

namespace recoil::wasm::core {
#if defined(RECOIL_WASMTIME_AVAILABLE)
namespace {

wasm_trap_t* Submit(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t count)
{
	auto* state = static_cast<HostState*>(environment);
	if (!state || !state->host || !slots || (count != 4 && count != 6 && count != 7))
		return generated::Trap("Wasm job submission ABI or host is unavailable");
	generated::ImportGuard guard(state, 1);
	if (!guard.Ok()) return generated::Trap(guard.Error());
	const auto mode = count == 7 ? static_cast<WasmJobInstanceMode>(slots[6].i32) : WasmJobInstanceMode::Fresh;
	const std::size_t start = count == 4 ? 0 : 2;
	const auto workerBytes = start != 0 ? static_cast<std::uint32_t>(slots[1].i32) : 0;
	const auto namePointer = static_cast<std::uint32_t>(slots[start].i32);
	const auto nameBytes = static_cast<std::uint32_t>(slots[start + 1].i32);
	const auto inputPointer = static_cast<std::uint32_t>(slots[start + 2].i32);
	const auto inputBytes = static_cast<std::uint32_t>(slots[start + 3].i32);
	if (mode > WasmJobInstanceMode::Reuse || workerBytes > 256 || nameBytes == 0 || nameBytes > 256 || inputBytes > WasmSyncedJobs::MAX_INPUT_BYTES) {
		slots[0].i64 = PackU32(0, static_cast<std::int32_t>(Status::InvalidArgument));
		return nullptr;
	}
	std::span<const std::uint8_t> name, input, worker;
	if (!state->memory.View(namePointer, nameBytes, name) ||
		!state->memory.View(inputPointer, inputBytes, input) ||
		(start != 0 && !state->memory.View(static_cast<std::uint32_t>(slots[0].i32), workerBytes, worker))) {
		slots[0].i64 = PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds));
		return nullptr;
	}
	if (!guard.Charge(workerBytes + nameBytes + inputBytes)) return generated::Trap(guard.Error());
	try {
		slots[0].i64 = state->host->SubmitJob(
			std::string(reinterpret_cast<const char*>(name.data()), name.size()), input,
			worker.empty() ? std::string_view{} : std::string_view(reinterpret_cast<const char*>(worker.data()), worker.size()), mode);
	} catch (...) {
		return generated::Trap("Host failed to allocate Wasm job submission");
	}
	return nullptr;
}

wasm_trap_t* RunBatch(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t count)
{
	auto* state = static_cast<HostState*>(environment);
	if (!state || !state->host || !slots || (count != 8 && count != 9))
		return generated::Trap("Wasm batch ABI or host is unavailable");
	generated::ImportGuard guard(state, 1);
	if (!guard.Ok()) return generated::Trap(guard.Error());
	const auto workerBytes = static_cast<std::uint32_t>(slots[1].i32);
	const auto nameBytes = static_cast<std::uint32_t>(slots[3].i32);
	const auto inputBytes = static_cast<std::uint32_t>(slots[5].i32);
	const auto outputPointer = static_cast<std::uint32_t>(slots[6].i32);
	const auto outputBytes = static_cast<std::uint32_t>(slots[7].i32);
	const auto mode = count == 9 ? static_cast<WasmJobInstanceMode>(slots[8].i32) : WasmJobInstanceMode::Fresh;
	if (mode > WasmJobInstanceMode::Reuse || workerBytes > 256 || nameBytes == 0 || nameBytes > 256 ||
		inputBytes > WasmJobBatch::MAX_WIRE_BYTES || outputBytes > WasmJobBatch::MAX_WIRE_BYTES) {
		slots[0].i64 = PackU32(0, static_cast<std::int32_t>(Status::InvalidArgument));
		return nullptr;
	}
	std::span<const std::uint8_t> worker, name, input;
	if (!state->memory.View(static_cast<std::uint32_t>(slots[0].i32), workerBytes, worker) ||
		!state->memory.View(static_cast<std::uint32_t>(slots[2].i32), nameBytes, name) ||
		!state->memory.View(static_cast<std::uint32_t>(slots[4].i32), inputBytes, input) ||
		!state->memory.Contains(outputPointer, outputBytes)) {
		slots[0].i64 = PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds));
		return nullptr;
	}
	// Charge reserved output as well as copied input BEFORE allocation/execution.
	if (!guard.Charge(std::uint64_t(workerBytes) + nameBytes + inputBytes + outputBytes))
		return generated::Trap(guard.Error());
	try {
		std::vector<WasmJobBatchInput> inputs;
		std::size_t requiredCapacity;
		if (!WasmJobBatch::Decode(input, inputs, requiredCapacity)) {
			slots[0].i64 = PackU32(0, static_cast<std::int32_t>(Status::InvalidArgument));
			return nullptr;
		}
		if (outputBytes < requiredCapacity) {
			slots[0].i64 = PackU32(requiredCapacity, static_cast<std::int32_t>(Status::BufferOverflow));
			return nullptr;
		}
		std::vector<WasmJobResult> results;
		const auto status = state->host->RunJobBatch(
			std::string(reinterpret_cast<const char*>(name.data()), name.size()), std::move(inputs), results,
			worker.empty() ? std::string_view{} : std::string_view(reinterpret_cast<const char*>(worker.data()), worker.size()), mode);
		if (status != 0) {
			slots[0].i64 = PackU32(0, status);
			return nullptr;
		}
		const auto wire = WasmJobBatch::Encode(results);
		if (wire.size() > outputBytes || !state->memory.Write(outputPointer, wire.data(), wire.size()))
			return generated::Trap("Cannot copy Wasm batch results");
		slots[0].i64 = PackU32(wire.size(), 0);
	} catch (const std::exception& failure) {
		state->host->FailJobBatch(std::current_exception());
		return generated::Trap(failure.what());
	} catch (...) {
		state->host->FailJobBatch(std::current_exception());
		return generated::Trap("Host failed to execute Wasm batch");
	}
	return nullptr;
}

} // namespace

bool RegisterJobsImports(wasmtime_linker_t* linker, HostState* state, std::string& error)
{
	const wasm_valkind_t params[] = {
		WASM_I32, WASM_I32, WASM_I32, WASM_I32, WASM_I32, WASM_I32, WASM_I32, WASM_I32, WASM_I32
	};
	const wasm_valkind_t results[] = {WASM_I64};
	const struct {
		std::string_view name;
		std::size_t arity;
		wasmtime_func_unchecked_callback_t callback;
	} imports[] = {
		{"submit", 4, Submit},
		{"submit-to", 6, Submit},
		{"run-batch-to", 8, RunBatch},
		{"submit-to-with-mode", 7, Submit},
		{"run-batch-to-with-mode", 9, RunBatch},
	};
	for (const auto& [name, arity, callback]: imports) {
		auto* type = MakeFuncType(params, arity, results, 1);
		auto* failure = wasmtime_linker_define_func_unchecked(linker, "spring:jobs", 11,
			name.data(), name.size(), type, callback, state, nullptr);
		wasm_functype_delete(type);
		if (failure) {
			error = ErrorMessage(failure);
			return false;
		}
	}
	return true;
}
#endif
} // namespace recoil::wasm::core
