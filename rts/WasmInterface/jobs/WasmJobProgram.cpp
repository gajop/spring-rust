/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmJobProgram.h"
#include "WasmJobProfile.h"

#include <array>
#include <chrono>
#include <cstdlib>
#include <mutex>
#include <string_view>

#include "WasmCoreAbi.h"
#include "WasmCoreValidation.h"
#include "WasmRuntime.h"
#include "System/Threading/ThreadPool.h"

struct WasmJobProgram::Backend {
	WasmRuntimeConfig config;
	std::string identity;
#if defined(RECOIL_WASMTIME_AVAILABLE)
	struct Instance {
		std::unique_ptr<wasmtime_store_t, decltype(&wasmtime_store_delete)> store{nullptr, wasmtime_store_delete};
		wasmtime_instance_t handle{};
		wasmtime_context_t* context = nullptr;
		recoil::wasm::core::Memory memory;
		recoil::wasm::core::RawExport buffer, capacity, work;
	};
	struct Slot {
		std::mutex mutex;
		Instance instance;
		std::string exportName;
	};
	struct Executable {
		wasm_engine_t* engine = nullptr;
		wasmtime_module_t* module = nullptr;
		wasmtime_linker_t* linker = nullptr;
		wasmtime_instance_pre_t* pre = nullptr;

		void Link()
		{
			// Resolve imports once. Workers have no host capabilities.
			linker = wasmtime_linker_new(engine);
			if (!linker)
				return;
			auto* error = wasmtime_linker_define_unknown_imports_as_traps(linker, module);
			if (!error)
				error = wasmtime_linker_instantiate_pre(linker, module, &pre);
			if (error)
				wasmtime_error_delete(error);
		}

		WasmJobStatus Instantiate(Instance& instance, const WasmRuntimeConfig& config, std::string& diagnostic) const
		{
			using namespace recoil::wasm::core;
			instance.store.reset(wasmtime_store_new(engine, nullptr, nullptr));
			if (!instance.store) {
				diagnostic = "Cannot create job store";
				return WasmJobStatus::RuntimeFailure;
			}
			wasmtime_store_limiter(instance.store.get(), static_cast<std::int64_t>(config.maxMemoryPages) * 65536,
				config.maxTableElements, 1, 1, 1);
			instance.context = wasmtime_store_context(instance.store.get());
			if (config.instructionFuel != 0) {
				if (auto* error = wasmtime_context_set_fuel(instance.context, config.instructionFuel)) {
					diagnostic = ErrorMessage(error);
					return WasmJobStatus::RuntimeFailure;
				}
			}
			wasm_trap_t* trap = nullptr;
			wasmtime_error_t* instantiateError = nullptr;
			if (pre != nullptr) {
				instantiateError = wasmtime_instance_pre_instantiate(pre, instance.context, &instance.handle, &trap);
			} else {
				std::unique_ptr<wasmtime_linker_t, decltype(&wasmtime_linker_delete)> localLinker(
					wasmtime_linker_new(engine), wasmtime_linker_delete);
				if (!localLinker) {
					diagnostic = "Cannot create job linker";
					return WasmJobStatus::RuntimeFailure;
				}
				if (auto* error = wasmtime_linker_define_unknown_imports_as_traps(localLinker.get(), module)) {
					diagnostic = ErrorMessage(error);
					return WasmJobStatus::RuntimeFailure;
				}
				instantiateError = wasmtime_linker_instantiate(localLinker.get(), instance.context, module, &instance.handle, &trap);
			}
			if (instantiateError) {
				diagnostic = ErrorMessage(instantiateError);
				if (trap)
					diagnostic += TrapMessage(trap);
				return WasmJobStatus::RuntimeFailure;
			}
			if (trap) {
				diagnostic = TrapMessage(trap);
				return WasmJobStatus::Trapped;
			}
			return WasmJobStatus::Ok;
		}

		~Executable()
		{
			if (pre) wasmtime_instance_pre_delete(pre);
			if (linker) wasmtime_linker_delete(linker);
			if (module) wasmtime_module_delete(module);
			if (engine) wasm_engine_delete(engine);
		}
	};
	Executable fresh;
	bool canonicalizeAll = false;
	std::once_flag pooledOnce;
	std::unique_ptr<Executable> pooled;
	std::once_flag reusableOnce;
	std::unique_ptr<std::array<Slot, ThreadPool::MAX_THREADS>> reusable;

	const Executable* Pooled()
	{
		std::call_once(pooledOnce, [&] {
			auto executable = std::make_unique<Executable>();
			executable->engine = static_cast<wasm_engine_t*>(WasmRuntime::NewPooledJobEngine(config, canonicalizeAll));
			if (!executable->engine)
				return;
			// Only deserialize this engine's own compiled code. Allocation policy
			// can change without recompiling or accepting an external artifact.
			wasm_byte_vec_t bytes{};
			if (auto* error = wasmtime_module_serialize(fresh.module, &bytes)) {
				wasmtime_error_delete(error);
				return;
			}
			auto* error = wasmtime_module_deserialize(executable->engine,
				reinterpret_cast<const std::uint8_t*>(bytes.data), bytes.size, &executable->module);
			wasm_byte_vec_delete(&bytes);
			if (error) {
				wasmtime_error_delete(error);
				return;
			}
			executable->Link();
			pooled = std::move(executable);
		});
		return pooled.get();
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
	state->fresh.engine = wasmtime_engine_clone(moduleEngine);
	state->canonicalizeAll = moduleEngine != runtimeEngine ||
		runtime.NanCanonicalization() == WasmNanCanonicalization::All;
	state->identity = std::move(identity) + "|" + runtime.ConfigurationIdentity() +
		(moduleEngine != runtimeEngine ? "|nan-canonicalization-fallback=all" : "");
	state->fresh.module = wasmtime_module_clone(static_cast<wasmtime_module_t*>(compiledModule));
	// Diagnostic baseline: resolving the imports separately for each job.
	const char* pre = std::getenv("SPRING_WASM_JOB_INSTANCE_PRE");
	if (pre == nullptr || std::string_view(pre) != "0")
		state->fresh.Link();
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
	bool reused = false;
	auto* module = static_cast<wasmtime_module_t*>(runtime.CompileModule(engine, compileBytes, error, reused));
	if (module == nullptr)
		return nullptr;
	auto program = FromCompiled(runtime, module, validated.identity.sha512, engine);
	wasmtime_module_delete(module);
	return program;
#else
	error = "Wasmtime is unavailable";
	return nullptr;
#endif
}

WasmJobResult WasmJobProgram::Run(std::string_view exportName,
	std::span<const std::uint8_t> input, WasmJobTiming* timing, std::size_t maxOutputBytes, WasmJobInstanceMode mode) const
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
	Backend::Slot* cached = nullptr;
	std::unique_lock<std::mutex> lease;
	if (mode == WasmJobInstanceMode::Reuse) {
		std::call_once(backend->reusableOnce, [&] {
			backend->reusable = std::make_unique<std::array<Backend::Slot, ThreadPool::MAX_THREADS>>();
		});
		auto& slot = (*backend->reusable)[ThreadPool::GetThreadNum()];
		// Foreground and background workers may share an index. Contended jobs
		// use a private instance rather than waiting or sharing mutable state.
		lease = std::unique_lock(slot.mutex, std::try_to_lock);
		if (lease.owns_lock()) {
			cached = &slot;
			if (slot.exportName != exportName) {
				slot.instance = {};
				slot.exportName = exportName;
			}
		}
	}
	// On any failure the checked-out store is destroyed. Only a completed
	// job returns an instance to its slot, after copying the result bytes.
	auto instance = cached ? std::move(cached->instance) : Backend::Instance{};
	WASM_JOB_PROFILE_TEARDOWN(stages);
	std::string error;
	if (!instance.store) {
		const auto* pooled = mode == WasmJobInstanceMode::Pooled ? backend->Pooled() : nullptr;
		auto status = pooled ? pooled->Instantiate(instance, backend->config, error)
			: backend->fresh.Instantiate(instance, backend->config, error);
		// Unsupported modules/platforms and local pool capacity fall back to the
		// ordinary allocator. Both allocators give Wasm the same initial state.
		if (pooled && status == WasmJobStatus::RuntimeFailure) {
			instance = {};
			error.clear();
			status = backend->fresh.Instantiate(instance, backend->config, error);
		}
		if (status != WasmJobStatus::Ok)
			return fail(status, std::move(error));
		const wasm_valkind_t i32[] = {WASM_I32};
		const wasm_valkind_t twoI32[] = {WASM_I32, WASM_I32};
		const wasm_valkind_t i64[] = {WASM_I64};
		if (!instance.memory.BindFromInstance(instance.context, instance.handle, error) ||
			!instance.buffer.Resolve(instance.context, instance.handle, "SPRING_JOB_INPUT_BUFFER", 23, {}, i32, false, error) ||
			!instance.capacity.Resolve(instance.context, instance.handle, "SPRING_JOB_INPUT_CAPACITY", 25, {}, i32, false, error) ||
			!instance.work.Resolve(instance.context, instance.handle, exportName.data(), exportName.size(), twoI32, i64, false, error))
			return fail(WasmJobStatus::InvalidExport, std::move(error));
	} else if (backend->config.instructionFuel != 0) {
		// Core validation forbids a Wasm start function, so there is no
		// instantiation fuel to carry over to the next call.
		if (auto* failure = wasmtime_context_set_fuel(instance.context, backend->config.instructionFuel))
			return fail(WasmJobStatus::RuntimeFailure, ErrorMessage(failure));
	}
	auto* context = instance.context;
	auto& memory = instance.memory;
	std::array<wasmtime_val_raw_t, 2> slots{};
	if (!instance.buffer.Call(context, slots.data(), 1, error))
		return fail(WasmJobStatus::Trapped, error);
	const auto inputPointer = static_cast<std::uint32_t>(slots[0].i32);
	if (!instance.capacity.Call(context, slots.data(), 1, error))
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
	if (!instance.work.Call(context, slots.data(), slots.size(), error))
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
	if (cached)
		cached->instance = std::move(instance);
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
