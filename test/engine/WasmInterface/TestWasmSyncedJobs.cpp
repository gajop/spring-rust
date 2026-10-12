/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include <catch_amalgamated.hpp>

#include <algorithm>
#include <array>
#include <iostream>
#include <mutex>
#include <chrono>
#include <cstdlib>
#include <fstream>
#include <stdexcept>
#include <string>
#include <vector>

#include "WasmInterface/jobs/WasmJobProgram.h"
#include "WasmInterface/jobs/WasmSyncedJobs.h"
#include "WasmCoreAbi.h"
#include "WasmRuntime.h"
#include "System/Misc/SpringTime.h"
#include "System/Threading/ThreadPool.h"

#if defined(RECOIL_WASM_TSAN_TEST) && defined(RECOIL_WASMTIME_AVAILABLE)
// Test-link interception keeps the production runtime/settings untouched.
// Rust's uninstrumented Rayon ownership transfers obscure native queue races
// from TSan. Synchronous compilation avoids those transfers. A lifecycle lock
// makes the uninstrumented VM allocator's reuse of unmapped addresses visible
// to TSan; guest execution, native queues/results, limits and fuel are unchanged.
// This does not instrument the Rust library or JIT-generated guest instructions.
static std::mutex tsanVmLifecycle;
extern "C" wasm_engine_t* __real_wasm_engine_new_with_config(wasm_config_t*);
extern "C" wasm_engine_t* __wrap_wasm_engine_new_with_config(wasm_config_t* config)
{
	wasmtime_config_parallel_compilation_set(config, false);
	return __real_wasm_engine_new_with_config(config);
}
extern "C" wasmtime_error_t* __real_wasmtime_linker_instantiate(const wasmtime_linker_t*,
	wasmtime_context_t*, const wasmtime_module_t*, wasmtime_instance_t*, wasm_trap_t**);
extern "C" wasmtime_error_t* __wrap_wasmtime_linker_instantiate(const wasmtime_linker_t* linker,
	wasmtime_context_t* context, const wasmtime_module_t* module, wasmtime_instance_t* instance, wasm_trap_t** trap)
{
	std::lock_guard lock(tsanVmLifecycle);
	return __real_wasmtime_linker_instantiate(linker, context, module, instance, trap);
}
extern "C" void __real_wasmtime_store_delete(wasmtime_store_t*);
extern "C" void __wrap_wasmtime_store_delete(wasmtime_store_t* store)
{
	std::lock_guard lock(tsanVmLifecycle);
	__real_wasmtime_store_delete(store);
}
#endif

namespace {
InitSpringTime springTime;
struct Cores { Cores() { Threading::DetectCores(); } } cores;

std::vector<std::uint8_t> ReadFixture()
{
	const char* fixture = std::getenv("WASM_JOB_FIXTURE");
	std::ifstream stream(fixture ? fixture : WASM_JOB_FIXTURE, std::ios::binary);
	REQUIRE(stream.good());
	const std::string wat((std::istreambuf_iterator<char>(stream)), {});
	wasm_byte_vec_t bytes;
	auto* error = wasmtime_wat2wasm(wat.data(), wat.size(), &bytes);
	REQUIRE(error == nullptr);
	std::vector<std::uint8_t> result(bytes.data, bytes.data + bytes.size);
	wasm_byte_vec_delete(&bytes);
	return result;
}

std::vector<std::uint8_t> Input(std::uint32_t seed, std::uint32_t work, std::size_t bytes = 16)
{
	std::vector<std::uint8_t> input(bytes, 99);
	for (int i = 0; i < 4; ++i) {
		input[i] = seed >> (8 * i);
		input[4 + i] = work >> (8 * i);
	}
	return input;
}

std::uint32_t Word(const std::vector<std::uint8_t>& bytes, std::size_t offset)
{
	return std::uint32_t(bytes[offset]) | (std::uint32_t(bytes[offset + 1]) << 8) |
		(std::uint32_t(bytes[offset + 2]) << 16) | (std::uint32_t(bytes[offset + 3]) << 24);
}

std::uint32_t Work(std::uint32_t seed, std::uint32_t count)
{
	while (count--) seed = seed * 1664525u + 1013904223u;
	return seed;
}

std::shared_ptr<WasmJobProgram> Program(WasmRuntime& runtime)
{
	std::string error;
	auto program = WasmJobProgram::Compile(runtime, ReadFixture(), error);
	INFO(error);
	REQUIRE(program);
	return program;
}

struct StopThreads { ~StopThreads() { ThreadPool::SetThreadCount(1); } };
}

TEST_CASE("Wasm jobs have fresh memory/globals, fixed limits and no engine imports")
{
	WasmRuntimeConfig config;
	config.instructionFuel = 100000;
	WasmRuntime runtime(config);
	auto program = Program(runtime);
	for (int i = 0; i < 2; ++i) {
		auto result = program->Run("work", Input(12, 100));
		REQUIRE(result.status == WasmJobStatus::Ok);
		REQUIRE(result.bytes.size() == 12);
		CHECK(Word(result.bytes, 0) == Work(12, 100));
		CHECK(Word(result.bytes, 4) == 1); // no globals carried between jobs
		CHECK(Word(result.bytes, 8) == 99);
	}
	const auto floats = program->Run("float-work", std::vector<std::uint8_t>{0,0,0,0,0,0,128,63});
	REQUIRE(floats.status == WasmJobStatus::Ok);
	CHECK(Word(floats.bytes, 0) == 0x7fc00000u); // canonical NaN, inherited runtime settings
	CHECK(Word(floats.bytes, 4) == 0x3faccccdu); // (1 * 1.25) + 0.1 in f32
	CHECK(program->Run("missing", Input(0, 0)).status == WasmJobStatus::InvalidExport);
	CHECK(program->Run("SPRING_ENV_MASK", Input(0, 0)).status == WasmJobStatus::InvalidExport);
	CHECK(program->Run("trap", Input(0, 0)).status == WasmJobStatus::Trapped);
	CHECK(program->Run("host-call", Input(0, 0)).status == WasmJobStatus::Trapped);
	CHECK(program->Run("bad-output", Input(0, 0)).status == WasmJobStatus::InvalidOutput);
	CHECK(program->Run("work", Input(0, 0, 65537)).status == WasmJobStatus::InputTooLarge);
	auto growth = program->Run("grow", Input(0, 0));
	REQUIRE(growth.status == WasmJobStatus::Ok);
	CHECK(Word(growth.bytes, 0) == 0xffffffffu);
}

TEST_CASE("Accepted Wasm job programs own their runtime engine after the source runtime is unloaded")
{
	std::shared_ptr<WasmJobProgram> program;
	{
		WasmRuntime runtime;
		program = Program(runtime);
	}
	const auto result = program->Run("work", Input(9, 10));
	REQUIRE(result.status == WasmJobStatus::Ok);
	CHECK(Word(result.bytes, 0) == Work(9, 10));
}

TEST_CASE("Wasm jobs have the configured deterministic fuel budget on every invocation")
{
	WasmRuntimeConfig config;
	config.instructionFuel = 1000;
	WasmRuntime runtime(config);
	auto program = Program(runtime);
	for (int i = 0; i < 2; ++i) {
		CHECK(program->Run("loop", Input(0, 0)).status == WasmJobStatus::Trapped);
		CHECK(program->Run("work", Input(17, 5)).status == WasmJobStatus::Ok);
		CHECK(program->Run("work", Input(17, 100000)).status == WasmJobStatus::Trapped);
	}
}

TEST_CASE("Wasm job checksums retain the entire copied input without rescanning payloads")
{
	WasmRuntime runtime;
	auto program = Program(runtime);
	int owner = 0;
	WasmSyncedJobs first, identical, different;
	first.BeginFrame(0);
	identical.BeginFrame(0);
	different.BeginFrame(0);
	std::vector<std::uint8_t> input(WasmSyncedJobs::MAX_INPUT_BYTES, 0);
	REQUIRE(first.Submit(&owner, program, "work", input) == 1);
	REQUIRE(identical.Submit(&owner, program, "work", input) == 1);
	const auto hash = first.StateHash(&owner);
	CHECK(identical.StateHash(&owner) == hash);
	input.back() = 1;
	REQUIRE(different.Submit(&owner, program, "work", input) == 1);
	CHECK(different.StateHash(&owner) != hash);
	CHECK(first.StateHash(&owner) == hash); // caller mutation cannot alter the digest
}

TEST_CASE("Wasm jobs merge globally in submission order at the next frame across thread counts")
{
	StopThreads stop;
	WasmRuntimeConfig config;
	config.instructionFuel = 100000000;
	WasmRuntime runtime(config);
	auto program = Program(runtime);
	std::vector<std::vector<std::uint8_t>> expected;
	std::uint64_t expectedHash = 0;
	int owners[2]{};
	for (const int threads: {1, 2, 4}) {
		ThreadPool::SetThreadCount(threads);
		WasmSyncedJobs queue;
		CHECK(queue.Submit(owners, program, "work", Input(0, 0)) == 0);
		REQUIRE(queue.BeginFrame(0).empty());
		for (int i = 0; i < 24; ++i) {
			auto input = Input(i, i == 0 ? 1000000 : 100 + i);
			REQUIRE(queue.Submit(&owners[i % 2], program, "work", input) == i + 1);
			input[0] = 200; // the queue must own the input snapshot
		}
		const auto hash = queue.StateHash(&owners[0]);
		queue.EndFrame();
		CHECK(queue.StateHash(&owners[0]) == hash); // readiness is absent from state
		CHECK(queue.Submit(owners, program, "work", Input(0, 0)) == 0);
		if (threads == 1) expectedHash = hash;
		CHECK(hash == expectedHash);
		auto results = queue.BeginFrame(1);
		REQUIRE(results.size() == 24);
		for (int i = 0; i < 24; ++i) {
			CHECK(results[i].id == i + 1);
			CHECK(results[i].submittedFrame == 0);
			CHECK(results[i].owner == &owners[i % 2]);
			REQUIRE(results[i].result.status == WasmJobStatus::Ok);
			CHECK(Word(results[i].result.bytes, 0) == Work(i, i == 0 ? 1000000 : 100 + i));
			if (threads == 1) expected.push_back(results[i].result.bytes);
			CHECK(results[i].result.bytes == expected[i]);
		}
		queue.EndFrame();
		CHECK(queue.BeginFrame(2).empty());
	}
}

TEST_CASE("Wasm job admission is bounded and teardown drains workers before removing an owner")
{
	StopThreads stop;
	ThreadPool::SetThreadCount(4);
	WasmRuntime runtime;
	auto program = Program(runtime);
	int ownerA = 0, ownerB = 0;
	WasmSyncedJobs queue;
	queue.BeginFrame(0);
	CHECK(queue.Submit(&ownerA, program, "work", Input(1, 1000000)) == 1);
	CHECK(queue.Submit(&ownerB, program, "work", Input(2, 10)) == 2);
	queue.EndFrame();
	queue.CancelOwner(&ownerA);
	auto remaining = queue.BeginFrame(1);
	REQUIRE(remaining.size() == 1);
	CHECK(remaining[0].owner == &ownerB);
	CHECK(remaining[0].id == 2);
	for (std::size_t i = 0; i < WasmSyncedJobs::MAX_JOBS_PER_FRAME; ++i)
		REQUIRE(queue.Submit(&ownerB, program, "work", Input(i, 0)) != 0);
	CHECK(queue.Submit(&ownerB, program, "work", Input(0, 0)) == 0);
	queue.Reset();
	queue.BeginFrame(0);
	const auto large = Input(0, 0, WasmSyncedJobs::MAX_INPUT_BYTES);
	for (int i = 0; i < 16; ++i)
		REQUIRE(queue.Submit(&ownerA, program, "work", large) != 0);
	CHECK(queue.Submit(&ownerA, program, "work", Input(0, 0)) == 0);
	queue.Reset();
	queue.BeginFrame(0);
	CHECK(queue.Submit(&ownerA, program, "work", Input(0, 0, WasmSyncedJobs::MAX_INPUT_BYTES + 1)) == 0);
	CHECK(queue.Submit(&ownerA, program, std::string(257, 'x'), Input(0, 0)) == 0);
}

TEST_CASE("Wasm guest failures retain their IDs and ordered next-frame delivery")
{
	StopThreads stop;
	ThreadPool::SetThreadCount(4);
	WasmRuntimeConfig config;
	config.instructionFuel = 1000;
	WasmRuntime runtime(config);
	auto program = Program(runtime);
	int owner = 0;
	WasmSyncedJobs queue;
	queue.BeginFrame(0);
	CHECK(queue.Submit(&owner, program, "loop", Input(0, 0)) == 1);
	CHECK(queue.Submit(&owner, program, "missing", Input(0, 0)) == 2);
	CHECK(queue.Submit(&owner, program, "work", Input(17, 5)) == 3);
	queue.EndFrame();
	CHECK_THROWS_AS(queue.BeginFrame(2), std::logic_error);
	auto results = queue.BeginFrame(1);
	REQUIRE(results.size() == 3);
	for (std::size_t i = 0; i < results.size(); ++i) CHECK(results[i].id == i + 1);
	CHECK(results[0].result.status == WasmJobStatus::Trapped);
	CHECK(results[0].result.bytes.empty());
	CHECK(results[1].result.status == WasmJobStatus::InvalidExport);
	CHECK(results[1].result.bytes.empty());
	REQUIRE(results[2].result.status == WasmJobStatus::Ok);
	CHECK(Word(results[2].result.bytes, 0) == Work(17, 5));
	// A result handler can submit the next batch; errors don't consume extra
	// IDs and don't prevent valid work from continuing at the fixed boundary.
	CHECK(queue.Submit(&owner, program, "work", Input(18, 5)) == 4);
	queue.EndFrame();
	results = queue.BeginFrame(2);
	REQUIRE(results.size() == 1);
	CHECK(results[0].id == 4);
	CHECK(results[0].submittedFrame == 1);
	REQUIRE(results[0].result.status == WasmJobStatus::Ok);
	CHECK(Word(results[0].result.bytes, 0) == Work(18, 5));
}

TEST_CASE("Wasm shutdown drains before pool recreation and resets IDs across cached task-group reuse")
{
	StopThreads stop;
	WasmRuntime runtime;
	auto program = Program(runtime);
	int owner = 0;
	WasmSyncedJobs queue;
	// More than the engine's 256 pooled background groups: a reset that leaves
	// a group in use cannot hide behind allocating a different slot next time.
	for (int reload = 0; reload < 270; ++reload) {
		ThreadPool::SetThreadCount(4);
		REQUIRE(queue.BeginFrame(0).empty());
		for (int i = 0; i < 8; ++i)
			REQUIRE(queue.Submit(&owner, program, "work", Input(i, 1000)) == i + 1);
		queue.EndFrame();
		queue.Reset(); // SpringApp does this BEFORE stopping its thread pool.
		ThreadPool::SetThreadCount(1);
		REQUIRE(queue.BeginFrame(0).empty());
		REQUIRE(queue.Submit(&owner, program, "work", Input(19, 5)) == 1);
		queue.EndFrame();
		const auto results = queue.BeginFrame(1);
		REQUIRE(results.size() == 1);
		REQUIRE(results[0].result.status == WasmJobStatus::Ok);
		CHECK(Word(results[0].result.bytes, 0) == Work(19, 5));
		queue.Reset();
	}
}

// Only this local benchmark can obtain the runtime's native engine. The guest
TEST_CASE("Local job queues progress without simulation and isolate IDs and quotas")
{
	WasmRuntimeConfig config;
	config.instructionFuel = 1000000;
	WasmRuntime runtime(config);
	auto program = Program(runtime);
	StopThreads stop;
	for (const int threads: {1, 4}) {
		ThreadPool::SetThreadCount(threads);
		WasmSyncedJobs synced;
		synced.BeginFrame(0);
		const auto syncHash = synced.StateHash(nullptr);
		WasmSyncedJobs local(WasmSyncedJobs::Mode::Update);
		int owner = 0;
		REQUIRE(local.Submit(&owner, program, "work", Input(1, 100)) == 1); // initialization
		REQUIRE(local.BeginUpdate().empty());
		local.EndUpdate();
		REQUIRE(local.Submit(&owner, program, "work", Input(2, 100)) == 2); // input/draw between updates
		auto results = local.BeginUpdate();
		REQUIRE(results.size() == 1);
		CHECK(results.front().id == 1);
		CHECK(Word(results.front().result.bytes, 0) == Work(1, 100));
		REQUIRE(local.Submit(&owner, program, "work", Input(3, 100)) == 3); // result callback
		REQUIRE(local.Submit(&owner, program, "work", Input(4, 100)) == 4); // Update callback
		local.EndUpdate();
		results = local.BeginUpdate();
		REQUIRE(results.size() == 3);
		for (std::size_t i = 0; i < results.size(); ++i) {
			CHECK(results[i].id == i + 2);
			CHECK(results[i].submittedFrame == 1);
			CHECK(Word(results[i].result.bytes, 0) == Work(i + 2, 100));
		}
		local.EndUpdate();
		CHECK(synced.StateHash(nullptr) == syncHash);
		for (unsigned i = 0; i < WasmSyncedJobs::MAX_JOBS_PER_FRAME; ++i)
			REQUIRE(local.Submit(&owner, program, "work", Input(i, 0)) != 0);
		CHECK(local.Submit(&owner, program, "work", Input(0, 0)) == 0);
		CHECK(synced.Submit(&owner, program, "work", Input(0, 0)) == 1);
		local.Reset();
		CHECK(local.Submit(&owner, program, "work", Input(1, 0)) == 1);
		local.EndUpdate();
		local.CancelOwner(&owner);
		CHECK(local.BeginUpdate().empty());
		CHECK_THROWS_AS(synced.BeginUpdate(), std::logic_error);
		CHECK_THROWS_AS(synced.EndUpdate(), std::logic_error);
	}
}

// Only this local benchmark can obtain the runtime's native engine. The guest
// never sees it. Both inline and isolated work use exactly this engine/config.
class WasmJobBenchmark {
public:
	static wasm_engine_t* Engine(const WasmRuntime& runtime)
	{
		return static_cast<wasm_engine_t*>(runtime.BackendEngine());
	}
};

TEST_CASE("Measure Wasm job setup, copies, dispatch and inline work", "[.benchmark]")
{
	using namespace recoil::wasm::core;
	using Clock = std::chrono::steady_clock;
	const auto ns = [](auto start) {
		return std::chrono::duration_cast<std::chrono::nanoseconds>(Clock::now() - start).count();
	};
	const auto median = [](std::vector<std::uint64_t> values) {
		std::sort(values.begin(), values.end());
		return values[values.size() / 2];
	};
	StopThreads stop;
	for (const std::uint64_t fuel: {0ull, 1000000000ull}) {
		WasmRuntimeConfig config;
		config.instructionFuel = fuel;
		WasmRuntime runtime(config);
		const auto compileStart = Clock::now();
		auto program = Program(runtime);
		const auto compileNs = ns(compileStart);
		auto* engine = WasmJobBenchmark::Engine(runtime);
		const auto bytes = ReadFixture();
		wasmtime_module_t* rawModule = nullptr;
		REQUIRE(wasmtime_module_new(engine, bytes.data(), bytes.size(), &rawModule) == nullptr);
		std::unique_ptr<wasmtime_module_t, decltype(&wasmtime_module_delete)> module(rawModule, wasmtime_module_delete);
		std::unique_ptr<wasmtime_store_t, decltype(&wasmtime_store_delete)> store(
			wasmtime_store_new(engine, nullptr, nullptr), wasmtime_store_delete);
		std::unique_ptr<wasmtime_linker_t, decltype(&wasmtime_linker_delete)> linker(wasmtime_linker_new(engine), wasmtime_linker_delete);
		REQUIRE(wasmtime_linker_define_unknown_imports_as_traps(linker.get(), module.get()) == nullptr);
		auto* context = wasmtime_store_context(store.get());
		if (fuel) REQUIRE(wasmtime_context_set_fuel(context, fuel) == nullptr);
		wasmtime_instance_t instance{};
		wasm_trap_t* trap = nullptr;
		REQUIRE(wasmtime_linker_instantiate(linker.get(), context, module.get(), &instance, &trap) == nullptr);
		REQUIRE(trap == nullptr);
		Memory memory;
		std::string error;
		REQUIRE(memory.BindFromInstance(context, instance, error));
		RawExport work, inlineBatch;
		const wasm_valkind_t params[] = {WASM_I32, WASM_I32};
		const wasm_valkind_t results[] = {WASM_I64};
		REQUIRE(work.Resolve(context, instance, "pure-work", 9, params, results, false, error));
		REQUIRE(inlineBatch.Resolve(context, instance, "inline-batch", 12, params, results, false, error));
		for (const std::uint32_t iterations: {0u, 1000u, 100000u}) {
			for (const std::size_t payload: {64u, 4096u, 65536u}) {
				constexpr int samples = 301;
				auto input = Input(7, iterations, payload);
				for (int i = 0; i < 4; ++i) input[8 + i] = samples >> (8 * i);
				std::array<wasmtime_val_raw_t, 2> slots{};
				REQUIRE(memory.Write(1024, input.data(), input.size()));
				if (fuel) REQUIRE(wasmtime_context_set_fuel(context, fuel) == nullptr);
				slots[0].i32 = 1024;
				slots[1].i32 = input.size();
				const auto inlineStart = Clock::now();
				REQUIRE(inlineBatch.Call(context, slots.data(), slots.size(), error));
				const auto inlineNs = ns(inlineStart) / samples;
				std::vector<std::uint8_t> inlineOutput(12);
				REQUIRE(memory.Read(32, inlineOutput.data(), inlineOutput.size()));
				REQUIRE(Word(inlineOutput, 0) == Work(7, iterations));
				std::vector<std::uint64_t> total, setup, inputCopy, dispatch, outputCopy;
				for (int i = 0; i < samples + 10; ++i) {
					WasmJobTiming timing;
					const auto start = Clock::now();
					const auto result = program->Run("pure-work", input, &timing);
					const auto totalNs = ns(start);
					REQUIRE(result.status == WasmJobStatus::Ok);
					REQUIRE(result.bytes == inlineOutput);
					if (i < 10) continue;
					total.push_back(totalNs);
					setup.push_back(timing.instantiateNs);
					inputCopy.push_back(timing.inputCopyNs);
					dispatch.push_back(timing.dispatchNs);
					outputCopy.push_back(timing.outputCopyNs);
				}
				std::cout << "WASM_JOB_BENCH {\"kind\":\"isolated\",\"fuel\":" << fuel
					<< ",\"iterations\":" << iterations << ",\"input_bytes\":" << payload
					<< ",\"output_bytes\":12,\"samples\":" << samples << ",\"compile_ns\":" << compileNs
					<< ",\"inline_ns\":" << inlineNs << ",\"total_ns\":" << median(total)
					<< ",\"instantiate_ns\":" << median(setup) << ",\"input_copy_ns\":" << median(inputCopy)
					<< ",\"dispatch_ns\":" << median(dispatch) << ",\"output_copy_ns\":" << median(outputCopy)
					<< "}\n";
			}
		}
		// Copy both ways at realistic buffer sizes, with no computation.
		for (const std::size_t payload: {64u, 4096u, 65536u}) {
			const auto input = Input(7, 0, payload);
			std::vector<std::uint64_t> inputCopy, outputCopy;
			for (int i = 0; i < 101; ++i) {
				WasmJobTiming timing;
				const auto result = program->Run("echo", input, &timing);
				REQUIRE(result.status == WasmJobStatus::Ok);
				REQUIRE(result.bytes == input);
				inputCopy.push_back(timing.inputCopyNs);
				outputCopy.push_back(timing.outputCopyNs);
			}
			std::cout << "WASM_JOB_BENCH {\"kind\":\"copies\",\"fuel\":" << fuel
				<< ",\"bytes_each_way\":" << payload << ",\"input_copy_ns\":" << median(inputCopy)
				<< ",\"output_copy_ns\":" << median(outputCopy) << "}\n";
		}
		for (const int threads: {1, 2, 4, 8}) {
			ThreadPool::SetThreadCount(threads);
			for (const std::uint32_t iterations: {0u, 1000u, 100000u}) {
				std::vector<std::uint64_t> admission, seal, barrier, total;
				for (int repeat = 0; repeat < 11; ++repeat) {
					WasmSyncedJobs queue;
					int owner = 0;
					const auto input = Input(7, iterations, 4096);
					queue.BeginFrame(0);
					const auto start = Clock::now();
					for (int i = 0; i < 64; ++i)
						REQUIRE(queue.Submit(&owner, program, "pure-work", input) != 0);
					const auto admitted = Clock::now();
					queue.EndFrame();
					const auto sealed = Clock::now();
					auto delivery = queue.BeginFrame(1);
					const auto finished = Clock::now();
					REQUIRE(delivery.size() == 64);
					for (const auto& item: delivery) REQUIRE(item.result.status == WasmJobStatus::Ok);
					const auto delta = [](auto a, auto b) { return std::chrono::duration_cast<std::chrono::nanoseconds>(b - a).count() / 64; };
					admission.push_back(delta(start, admitted));
					seal.push_back(delta(admitted, sealed));
					barrier.push_back(delta(sealed, finished));
					total.push_back(delta(start, finished));
				}
				std::cout << "WASM_JOB_BENCH {\"kind\":\"queue\",\"fuel\":" << fuel
					<< ",\"workers\":" << threads << ",\"iterations\":" << iterations
					<< ",\"jobs_per_batch\":64,\"input_bytes\":4096,\"admission_ns\":" << median(admission)
					<< ",\"seal_ns\":" << median(seal) << ",\"barrier_ns\":" << median(barrier)
					<< ",\"total_ns\":" << median(total) << "}\n";
			}
		}
	}
}
