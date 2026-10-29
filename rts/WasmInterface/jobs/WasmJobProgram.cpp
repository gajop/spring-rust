/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmJobProgram.h"
#include "WasmJobProfile.h"

#include <algorithm>
#include <array>
#include <chrono>
#include <limits>

#include "WasmCoreAbi.h"
#include "WasmCoreValidation.h"
#include "WasmRuntime.h"

struct WasmJobProgram::Backend {
	WasmRuntimeConfig config;
	std::string identity;
#if defined(RECOIL_WASMTIME_AVAILABLE)
	wasm_engine_t* engine = nullptr;
	wasmtime_module_t* module = nullptr;
	~Backend() {
		if (module) wasmtime_module_delete(module);
		if (engine) wasm_engine_delete(engine);
	}
#endif
};

WasmJobProgram::WasmJobProgram(std::unique_ptr<Backend> backend): backend(std::move(backend)) {}
WasmJobProgram::~WasmJobProgram() = default;
const std::string& WasmJobProgram::Identity() const { return backend->identity; }

std::shared_ptr<WasmJobProgram> WasmJobProgram::FromCompiled(const WasmRuntime& runtime, void* compiledModule, std::string identity, void* engine)
{
#if defined(RECOIL_WASMTIME_AVAILABLE)
	if (compiledModule == nullptr || !runtime.IsAvailable() || runtime.Config().allowThreads)
		return nullptr;
	auto* runtimeEngine = static_cast<wasm_engine_t*>(runtime.BackendEngine());
	auto* moduleEngine = engine != nullptr ? static_cast<wasm_engine_t*>(engine) : runtimeEngine;
	auto state = std::make_unique<Backend>();
	state->config = runtime.Config();
	state->engine = wasmtime_engine_clone(moduleEngine);
	state->identity = std::move(identity) + "|" + runtime.ConfigurationIdentity() +
		(moduleEngine != runtimeEngine ? "|nan-canonicalization-fallback=all" : "");
	state->module = wasmtime_module_clone(static_cast<wasmtime_module_t*>(compiledModule));
	return std::shared_ptr<WasmJobProgram>(new WasmJobProgram(std::move(state)));
#else
	return nullptr;
#endif
}

std::shared_ptr<WasmJobProgram> WasmJobProgram::Compile(const WasmRuntime& runtime,
	const std::vector<std::uint8_t>& bytes, std::string& error)
{
	if (!runtime.IsAvailable() || runtime.Config().allowThreads) {
		error = "Jobs require an available deterministic runtime without Wasm threads";
		return nullptr;
	}
	const auto validated = recoil::wasm::core::ValidateModule(bytes,
		WasmEnvironment::RulesSynced, RECOIL_WASM_INTERFACE_VERSION_NUMBER, runtime.Config());
	if (!validated.valid) {
		error = validated.error;
		return nullptr;
	}
#if defined(RECOIL_WASMTIME_AVAILABLE)
	// Jobs run synced kernels: the same NaN canonicalization as a synced module.
	std::shared_ptr<const std::vector<std::uint8_t>> rewritten;
	std::string note;
	bool fallback = false;
	void* engine = runtime.PrepareSyncedModule(bytes, validated.identity.sha512, rewritten, note, fallback);
	const auto& compileBytes = rewritten != nullptr ? *rewritten : bytes;
	wasmtime_module_t* module = nullptr;
	if (auto* failure = wasmtime_module_new(static_cast<wasm_engine_t*>(engine),
			compileBytes.data(), compileBytes.size(), &module)) {
		error = recoil::wasm::core::ErrorMessage(failure);
		return nullptr;
	}
	auto program = FromCompiled(runtime, module, validated.identity.sha512, engine);
	wasmtime_module_delete(module);
	return program;
#else
	error = "Wasmtime is unavailable";
	return nullptr;
#endif
}

WasmJobResult WasmJobProgram::Run(std::string_view exportName,
	std::span<const std::uint8_t> input, WasmJobTiming* timing, std::size_t maxOutputBytes) const
{
	WasmJobResult result;
#if defined(RECOIL_WASMTIME_AVAILABLE)
	using namespace recoil::wasm::core;
	using Clock = std::chrono::steady_clock;
	const auto now = [timing] { return timing ? Clock::now() : Clock::time_point{}; };
	const auto start = now();
	const auto elapsed = [](auto a, auto b) {
		return std::chrono::duration_cast<std::chrono::nanoseconds>(b - a).count();
	};
	const auto fail = [&result](WasmJobStatus status, std::string diagnostic) {
		result.status = status;
		result.diagnostic = std::move(diagnostic);
		return result;
	};
	WASM_JOB_PROFILE_START(stages);
	WASM_JOB_PROFILE_STAGE(stages, "Wasm::Jobs::Setup");
	std::unique_ptr<wasmtime_store_t, decltype(&wasmtime_store_delete)> store(
		wasmtime_store_new(backend->engine, nullptr, nullptr), wasmtime_store_delete);
	std::unique_ptr<wasmtime_linker_t, decltype(&wasmtime_linker_delete)> linker(
		nullptr, wasmtime_linker_delete);
	WASM_JOB_PROFILE_TEARDOWN(stages);
	if (!store)
		return fail(WasmJobStatus::RuntimeFailure, "Cannot create job store");
	wasmtime_store_limiter(store.get(), static_cast<std::int64_t>(backend->config.maxMemoryPages) * 65536,
		backend->config.maxTableElements, 1, 1, 1);
	auto* context = wasmtime_store_context(store.get());
	// The fresh per-job budget includes helper exports as well as computation.
	if (backend->config.instructionFuel != 0) {
		if (auto* error = wasmtime_context_set_fuel(context, backend->config.instructionFuel))
			return fail(WasmJobStatus::RuntimeFailure, ErrorMessage(error));
	}
	linker.reset(wasmtime_linker_new(backend->engine));
	if (!linker)
		return fail(WasmJobStatus::RuntimeFailure, "Cannot create job linker");
	// Workers have NO engine, clock, RNG or submission capabilities. A source
	// rules module can import them, but calling any import in a job traps.
	if (auto* error = wasmtime_linker_define_unknown_imports_as_traps(linker.get(), backend->module))
		return fail(WasmJobStatus::RuntimeFailure, ErrorMessage(error));
	wasmtime_instance_t instance{};
	wasm_trap_t* trap = nullptr;
	if (auto* error = wasmtime_linker_instantiate(linker.get(), context, backend->module, &instance, &trap)) {
		std::string diagnostic = ErrorMessage(error);
		if (trap) diagnostic += TrapMessage(trap);
		return fail(WasmJobStatus::RuntimeFailure, std::move(diagnostic));
	}
	if (trap)
		return fail(WasmJobStatus::Trapped, TrapMessage(trap));
	Memory memory;
	std::string error;
	RawExport buffer, capacity, work;
	const wasm_valkind_t i32[] = {WASM_I32};
	const wasm_valkind_t twoI32[] = {WASM_I32, WASM_I32};
	const wasm_valkind_t i64[] = {WASM_I64};
	if (!memory.BindFromInstance(context, instance, error) ||
		!buffer.Resolve(context, instance, "SPRING_JOB_INPUT_BUFFER", 23, {}, i32, false, error) ||
		!capacity.Resolve(context, instance, "SPRING_JOB_INPUT_CAPACITY", 25, {}, i32, false, error) ||
		!work.Resolve(context, instance, exportName.data(), exportName.size(), twoI32, i64, false, error))
		return fail(WasmJobStatus::InvalidExport, error);
	std::array<wasmtime_val_raw_t, 2> slots{};
	if (!buffer.Call(context, slots.data(), 1, error))
		return fail(WasmJobStatus::Trapped, error);
	const auto inputPointer = static_cast<std::uint32_t>(slots[0].i32);
	if (!capacity.Call(context, slots.data(), 1, error))
		return fail(WasmJobStatus::Trapped, error);
	const auto inputCapacity = static_cast<std::uint32_t>(slots[0].i32);
	if (input.size() > inputCapacity || !memory.Contains(inputPointer, input.size()))
		return fail(WasmJobStatus::InputTooLarge, "Input exceeds job scratch buffer");
	const auto instantiated = now();
	WASM_JOB_PROFILE_STAGE(stages, "Wasm::Jobs::InputCopy");
	if (!memory.Write(inputPointer, input.data(), input.size()))
		return fail(WasmJobStatus::InputTooLarge, "Cannot copy job input");
	const auto copied = now();
	slots[0].i32 = static_cast<std::int32_t>(inputPointer);
	slots[1].i32 = static_cast<std::int32_t>(input.size());
	WASM_JOB_PROFILE_STAGE(stages, "Wasm::Jobs::Execute");
	if (!work.Call(context, slots.data(), slots.size(), error))
		return fail(WasmJobStatus::Trapped, error);
	const auto dispatched = now();
	WASM_JOB_PROFILE_STAGE(stages, "Wasm::Jobs::OutputCopy");
	const auto packed = static_cast<std::uint64_t>(slots[0].i64);
	const auto outputPointer = static_cast<std::uint32_t>(packed);
	const auto outputBytes = static_cast<std::uint32_t>(packed >> 32);
	if (outputBytes > maxOutputBytes || outputBytes > backend->config.resultBytesLimit || outputBytes > 1024u * 1024u ||
		!memory.Contains(outputPointer, outputBytes))
		return fail(WasmJobStatus::InvalidOutput, "Job output range or byte limit exceeded");
	result.bytes.resize(outputBytes);
	if (!memory.Read(outputPointer, result.bytes.data(), outputBytes))
		return fail(WasmJobStatus::InvalidOutput, "Cannot copy job result");
	const auto finished = now();
	if (timing) {
		timing->instantiateNs = elapsed(start, instantiated);
		timing->inputCopyNs = elapsed(instantiated, copied);
		timing->dispatchNs = elapsed(copied, dispatched);
		timing->outputCopyNs = elapsed(dispatched, finished);
	}
#else
	result.status = WasmJobStatus::RuntimeFailure;
	result.diagnostic = "Wasmtime is unavailable";
#endif
	return result;
}
