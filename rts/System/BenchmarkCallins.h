/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <algorithm>
#include <array>
#include <chrono>
#include <cstdint>
#include <cstdlib>
#include <cstring>
#include <fstream>
#include <iomanip>
#include <limits>
#include <sstream>
#include <string>
#include <string_view>
#include <unordered_map>
#include <vector>

namespace spring::benchmark_callins {

using Clock = std::chrono::steady_clock;

// The first dispatches carry one-time work (unit creation, the draw matrix), so
// report a median over per-dispatch samples rather than a mean.
constexpr std::size_t kWarmupSamples = 1;
constexpr std::size_t kMinimumSamples = 32;

// Read once. End() consults this on every recorded sample, and a getenv scan
// there lands inside the timed region of every enclosing stage.
inline std::size_t ConfiguredMeasurementSamples()
{
	static const std::size_t samples = [] {
		const char* value = std::getenv("SPRING_NATIVE_BENCHMARK_REPEATS");
		if (value == nullptr)
			return kMinimumSamples;

		char* end = nullptr;
		const unsigned long parsed = std::strtoul(value, &end, 10);
		if (end == value || *end != '\0' || parsed == 0)
			return kMinimumSamples;
		return std::max<std::size_t>(kMinimumSamples, parsed);
	}();
	return samples;
}

// Stage rows decompose the one callin the run is already measuring. They are
// identified by an enumerator rather than a name so that entering a stage costs
// a clock read and nothing else: no allocation, no hashing, no name compare.
// Nothing here may allocate, because every stage sits inside the timed region
// of the callin row it is decomposing.
enum class Stage : std::uint8_t {
	NativeDispatch,
	CoreSelection,
	CoreAggregation,
	ModuleDispatch,
	Visibility,
	WasmtimeEntry,
	Count,
};

constexpr std::size_t kStageCount = static_cast<std::size_t>(Stage::Count);

constexpr std::string_view StageSuffix(Stage stage)
{
	switch (stage) {
		case Stage::NativeDispatch: return "native_dispatch";
		case Stage::CoreSelection: return "core_selection";
		case Stage::CoreAggregation: return "core_aggregation";
		case Stage::ModuleDispatch: return "module_dispatch";
		case Stage::Visibility: return "visibility";
		case Stage::WasmtimeEntry: return "wasmtime_entry";
		case Stage::Count: break;
	}
	return {};
}

struct Token {
	// Callin rows are keyed by a literal test name; stage rows by enumerator.
	// Both are trivially copyable, so a token never reaches the allocator.
	std::string_view test;
	Stage stage = Stage::Count;
	Clock::time_point start;
	bool active = false;
};

struct Stats {
	std::vector<uint64_t> samples;
};

inline bool IsEnabled()
{
	static const bool enabled = [] {
		const char* value = std::getenv("SPRING_NATIVE_BENCHMARK_CALLINS");
		if (value == nullptr)
			return false;
		const std::string_view setting(value);
		return setting == "1" || setting == "true" || setting == "TRUE" ||
			setting == "yes" || setting == "YES" || setting == "on" || setting == "ON";
	}();
	return enabled;
}

inline const std::string& ConfiguredBackend()
{
	static const std::string configured = [] {
		const char* value = std::getenv("SPRING_NATIVE_BENCHMARK_BACKEND");
		return value == nullptr ? std::string{} : std::string(value);
	}();
	return configured;
}

inline bool IsBackend(std::string_view backend)
{
	const std::string& configured = ConfiguredBackend();
	if (configured == backend)
		return true;
	// Both Wasm alternate hosts are reached through the same engine call sites
	// and therefore record the same "wasm" token. Flush labels the row with the
	// configured transport so Component-typed and Core remain distinct columns.
	return backend == "wasm" &&
		(configured == "wasm_rust_typed" || configured == "wasm_core");
}

inline bool IsCase(std::string_view benchmarkCase)
{
	static const std::string configured = [] {
		const char* value = std::getenv("SPRING_NATIVE_BENCHMARK_CASE");
		return value == nullptr ? std::string{} : std::string(value);
	}();
	return configured == benchmarkCase;
}

inline bool GameFrameReadsArgument()
{
	static const bool readsArgument = [] {
		const char* value = std::getenv("SPRING_NATIVE_BENCHMARK_CALLIN_VARIANT");
		if (value == nullptr)
			return false;
		return std::string_view(value) == "gameframe";
	}();
	return readsArgument;
}

inline bool IsVariant(std::string_view variant)
{
	static const std::string configured = [] {
		const char* value = std::getenv("SPRING_NATIVE_BENCHMARK_CALLIN_VARIANT");
		return value == nullptr ? std::string{} : std::string(value);
	}();
	return configured == variant;
}

inline bool StagesEnabled()
{
	static const bool enabled = [] {
		const char* value = std::getenv("SPRING_NATIVE_BENCHMARK_STAGES");
		if (value == nullptr)
			return false;
		const std::string_view setting(value);
		return setting == "1" || setting == "true" || setting == "TRUE" ||
			setting == "yes" || setting == "YES" || setting == "on" || setting == "ON";
	}();
	return enabled;
}

inline std::string_view GameFrameTestName()
{
	if (IsVariant("unimplemented"))
		return "callin_unimplemented";
	if (IsVariant("fourmodules"))
		return "callin_4modules";
	return GameFrameReadsArgument() ? "callin_gameframe" : "callin_empty";
}

inline std::unordered_map<std::string_view, Stats>& Samples()
{
	// Keys are string literals with static storage, so string_view is a stable
	// key and the hot path never builds a std::string.
	static std::unordered_map<std::string_view, Stats> samples;
	return samples;
}

// Stage samples live in a flat array indexed by enumerator: recording one is a
// push_back into a pre-reserved vector, with no key to hash.
inline std::array<Stats, kStageCount>& StageSamples()
{
	static std::array<Stats, kStageCount> samples = [] {
		std::array<Stats, kStageCount> entries;
		for (Stats& stats : entries)
			stats.samples.reserve(1024);
		return entries;
	}();
	return samples;
}

// The callin whose dispatch the stage rows decompose. Stages are only recorded
// while such a callin is being timed, so they can never be attributed to the
// wrong row.
inline std::string_view& ActiveStageTest()
{
	static std::string_view test;
	return test;
}

// Retained across dispatches so Flush can label the stage rows after the last
// callin has ended.
inline std::string_view& StageOwnerTest()
{
	static std::string_view test;
	return test;
}

inline bool IsTrackedTest(std::string_view test)
{
	if (IsVariant("unimplemented"))
		return test == "callin_unimplemented";
	if (IsVariant("fourmodules"))
		return test == "callin_4modules";
	if (IsVariant("consoleline"))
		return test == "callin_string";
	if (IsVariant("commandnotify"))
		return test == "callin_command";
	if (IsVariant("variable"))
		return test == "callin_string" || test == "callin_command" ||
			test == "callin_string_event" || test == "callin_command_event";
	// Update is dispatched unsynced-only. Its row comes from a dedicated run
	// against an unsynced guest; recording it elsewhere times an engine path
	// that reaches no module.
	if (IsVariant("update"))
		return test == "callin_update";
	return test == "callin_empty" || test == "callin_gameframe" ||
		test == "callin_drawworld" ||
		test == "callin_unitcreated" || test == "callin_unitpredamaged" ||
		test == "callin_allowunitcreation";
}

inline std::string_view EventTestName(std::string_view event)
{
	if (event == "GameFrame" && IsVariant("unimplemented"))
		return "callin_unimplemented";
	if (event == "DrawWorld") {
#ifdef HEADLESS
		return {};
#else
		return "callin_drawworld";
#endif
	}
	if (event == "UnitCreated")
		return "callin_unitcreated";
	if (event == "UnitPreDamaged")
		return "callin_unitpredamaged";
	if (event == "AllowUnitCreation")
		return "callin_allowunitcreation";
	if (event == "AddConsoleLine")
		return "callin_string";
	if (event == "CommandNotify")
		return "callin_command";
	return {};
}

// Walk a buffer larger than L3 cache so the next callin dispatch starts cold,
// matching real usage where a full frame of work runs between callins.
inline void EvictCache()
{
	constexpr std::size_t kBytes = 64 * 1024 * 1024;
	static std::vector<char> buffer(kBytes, 0);
	volatile char sink = 0;
	for (std::size_t i = 0; i < kBytes; i += 64)
		sink = buffer[i];
	(void)sink;
}

inline Token Begin(std::string_view backend, std::string_view test)
{
	// Lua's generic call wrapper sees source event names. Canonicalize them here
	// so new representative callins do not need one-off timing branches in the
	// large LuaHandle implementation.
	const std::string_view eventTest = EventTestName(test);
	if (!eventTest.empty())
		test = eventTest;
	if (!IsEnabled() || !IsBackend(backend) || !IsTrackedTest(test))
		return {};
	EvictCache();
	if (StagesEnabled()) {
		ActiveStageTest() = test;
		StageOwnerTest() = test;
	}
	return {
		.test = test,
		.start = Clock::now(),
		.active = true,
	};
}

inline Token BeginConfigured(std::string_view test)
{
	const std::string& backend = ConfiguredBackend();
	if (backend == "lua")
		return Begin("lua", test);
	if (backend == "native")
		return Begin("native", test);
	if (backend == "wasm" || backend == "wasm_rust_typed" || backend == "wasm_core")
		return Begin("wasm", test);
	return {};
}

// Diagnostic stage rows are opt-in: they nest inside the timed region of the
// callin row they decompose, so their own cost is charged to that row. A run
// that reports a headline callin number must therefore leave stages off, and a
// stage run reports only the decomposition.
inline Token BeginStage(Stage stage)
{
	// Armed by Begin() only while a tracked callin is being timed with stages
	// on, so entering a stage reads one flag and takes one clock sample.
	if (ActiveStageTest().empty())
		return {};
	return {
		.test = {},
		.stage = stage,
		.start = Clock::now(),
		.active = true,
	};
}

inline void End(Token token)
{
	if (!token.active)
		return;

	const auto elapsed = std::chrono::duration_cast<std::chrono::nanoseconds>(
		Clock::now() - token.start).count();
	const std::size_t sampleLimit = kWarmupSamples + ConfiguredMeasurementSamples();
	if (token.stage != Stage::Count) {
		Stats& stats = StageSamples()[static_cast<std::size_t>(token.stage)];
		if (stats.samples.size() < sampleLimit)
			stats.samples.push_back(static_cast<uint64_t>(elapsed));
		return;
	}

	Stats& stats = Samples()[token.test];
	if (stats.samples.size() < sampleLimit)
		stats.samples.push_back(static_cast<uint64_t>(elapsed));
	ActiveStageTest() = {};
}

// Cost of the clock reads that bracket a single dispatch. Reported alongside
// the row, not subtracted from it: it bounds how much of a small median is
// measurement bias.
inline double ClockOverheadNanoseconds()
{
	double best = std::numeric_limits<double>::max();
	for (int index = 0; index < 1000; ++index) {
		const auto start = Clock::now();
		const auto end = Clock::now();
		best = std::min(best, static_cast<double>(
			std::chrono::duration_cast<std::chrono::nanoseconds>(end - start).count()));
	}
	return best;
}

inline double Percentile(const std::vector<uint64_t>& sorted, double fraction)
{
	if (sorted.empty())
		return 0.0;
	const std::size_t index = static_cast<std::size_t>(
		fraction * static_cast<double>(sorted.size() - 1));
	return static_cast<double>(sorted[std::min(index, sorted.size() - 1)]);
}

inline void WriteRow(std::ofstream& file, std::string_view backend,
	std::string_view test, const Stats& sample, double clockOverhead, double fanout)
{
	if (sample.samples.empty())
		return;
	if (sample.samples.size() <= kWarmupSamples ||
		sample.samples.size() - kWarmupSamples < kMinimumSamples) {
		file << "{\"backend\":\"" << backend
			 << "\",\"test\":\"" << test
			 << "\",\"status\":\"unavailable\",\"iterations\":" << sample.samples.size()
			 << ",\"reason\":\"only " << sample.samples.size()
			 << " dispatches were recorded; a callin row needs at least "
			 << (kMinimumSamples + kWarmupSamples) << "\"}\n";
		return;
	}
	std::vector<uint64_t> sorted(
		sample.samples.begin() + kWarmupSamples, sample.samples.end());
	std::sort(sorted.begin(), sorted.end());
	const double median = Percentile(sorted, 0.5) * fanout;
	// Central 90%: per-dispatch samples always contain scheduling outliers.
	const double spread = (Percentile(sorted, 0.95) - Percentile(sorted, 0.05)) * fanout;
	file << "{\"backend\":\"" << backend
		 << "\",\"test\":\"" << test
		 << "\",\"status\":\"pass\",\"iterations\":" << sorted.size()
		 << ",\"medianNs\":" << std::fixed << std::setprecision(3) << median
		 << ",\"spreadNs\":" << spread
		 << ",\"p99Ns\":" << Percentile(sorted, 0.99) * fanout
		 << ",\"clockOverheadNs\":" << clockOverhead
		 << ",\"samplesNs\":[";
	for (std::size_t index = 0; index < sorted.size(); ++index) {
		if (index != 0)
			file << ',';
		file << sorted[index] * fanout;
	}
	file << "]"
		 << ",\"measurement\":\"engine callin boundary, cold cache; median of per-dispatch samples\"}\n";
}

inline void Flush()
{
	static bool flushed = false;
	if (flushed || !IsEnabled() || Samples().empty())
		return;

	const char* output = std::getenv("SPRING_NATIVE_PARITY_OUTPUT_DIR");
	const char* backend = std::getenv("SPRING_NATIVE_BENCHMARK_BACKEND");
	if (output == nullptr || backend == nullptr || *backend == '\0')
		return;

	std::ofstream file(
		std::string(output) + "/benchmark_" + backend + ".jsonl",
		std::ios::app
	);
	if (!file)
		return;

	const double clockOverhead = ClockOverheadNanoseconds();
	// Native modules are separate event clients, so a four-module run records
	// one token per client. Report the per-engine-event fan-out cost, matching
	// the single dispatch token used by Lua and Wasm.
	const double fanout = std::string_view(backend) == "native" &&
		IsVariant("fourmodules") ? 4.0 : 1.0;
	for (const auto& [test, sample] : Samples())
		WriteRow(file, backend, test, sample, clockOverhead, fanout);

	// Stage rows are named only here, where naming costs nothing.
	if (StagesEnabled() && !StageOwnerTest().empty()) {
		for (std::size_t index = 0; index < kStageCount; ++index) {
			const Stats& sample = StageSamples()[index];
			if (sample.samples.empty())
				continue;
			const std::string name = std::string(StageOwnerTest()) + "_" +
				std::string(StageSuffix(static_cast<Stage>(index)));
			WriteRow(file, backend, name, sample, clockOverhead, 1.0);
		}
	}
	file.flush();
	flushed = true;
}

} // namespace spring::benchmark_callins
