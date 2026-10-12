/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreBindings.h"

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
	if (!state || !state->host || !slots || (count != 4 && count != 6))
		return generated::Trap("Wasm job submission ABI or host is unavailable");
	generated::ImportGuard guard(state, 1);
	if (!guard.Ok()) return generated::Trap(guard.Error());
	const std::size_t start = count == 6 ? 2 : 0;
	const auto workerBytes = count == 6 ? static_cast<std::uint32_t>(slots[1].i32) : 0;
	const auto namePointer = static_cast<std::uint32_t>(slots[start].i32);
	const auto nameBytes = static_cast<std::uint32_t>(slots[start + 1].i32);
	const auto inputPointer = static_cast<std::uint32_t>(slots[start + 2].i32);
	const auto inputBytes = static_cast<std::uint32_t>(slots[start + 3].i32);
	if (workerBytes > 256 || nameBytes == 0 || nameBytes > 256 || inputBytes > WasmSyncedJobs::MAX_INPUT_BYTES) {
		slots[0].i64 = PackU32(0, static_cast<std::int32_t>(Status::InvalidArgument));
		return nullptr;
	}
	std::span<const std::uint8_t> name, input, worker;
	if (!state->memory.View(namePointer, nameBytes, name) ||
		!state->memory.View(inputPointer, inputBytes, input) ||
		(count == 6 && !state->memory.View(static_cast<std::uint32_t>(slots[0].i32), workerBytes, worker))) {
		slots[0].i64 = PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds));
		return nullptr;
	}
	if (!guard.Charge(workerBytes + nameBytes + inputBytes)) return generated::Trap(guard.Error());
	try {
		slots[0].i64 = state->host->SubmitJob(
			std::string(reinterpret_cast<const char*>(name.data()), name.size()), input,
			worker.empty() ? std::string_view{} : std::string_view(reinterpret_cast<const char*>(worker.data()), worker.size()));
	} catch (...) {
		return generated::Trap("Host failed to allocate Wasm job submission");
	}
	return nullptr;
}

} // namespace

bool RegisterJobsImports(wasmtime_linker_t* linker, HostState* state, std::string& error)
{
	const wasm_valkind_t params[] = {WASM_I32, WASM_I32, WASM_I32, WASM_I32};
	const wasm_valkind_t results[] = {WASM_I64};
	auto* type = MakeFuncType(params, 4, results, 1);
	auto* failure = wasmtime_linker_define_func_unchecked(linker, "spring:jobs", 11,
		"submit", 6, type, Submit, state, nullptr);
	wasm_functype_delete(type);
	if (failure) {
		error = ErrorMessage(failure);
		return false;
	}
	const wasm_valkind_t sixParams[] = {WASM_I32, WASM_I32, WASM_I32, WASM_I32, WASM_I32, WASM_I32};
	type = MakeFuncType(sixParams, 6, results, 1);
	failure = wasmtime_linker_define_func_unchecked(linker, "spring:jobs", 11,
		"submit-to", 9, type, Submit, state, nullptr);
	wasm_functype_delete(type);
	if (!failure) return true;
	error = ErrorMessage(failure);
	return false;
}
#endif
} // namespace recoil::wasm::core
