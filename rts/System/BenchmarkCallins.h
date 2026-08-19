/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <algorithm>
#include <chrono>
#include <cstdint>
#include <cstdlib>
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

struct Token {
	std::string test;
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

inline bool IsBackend(std::string_view backend)
{
	static const std::string configured = [] {
		const char* value = std::getenv("SPRING_NATIVE_BENCHMARK_BACKEND");
		return value == nullptr ? std::string{} : std::string(value);
	}();
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

inline std::string_view GameFrameTestName()
{
	if (IsVariant("unimplemented"))
		return "callin_unimplemented";
	if (IsVariant("fourmodules"))
		return "callin_4modules";
	return GameFrameReadsArgument() ? "callin_gameframe" : "callin_empty";
}

inline std::unordered_map<std::string, Stats>& Samples()
{
	static std::unordered_map<std::string, Stats> samples;
	return samples;
}

inline bool IsTrackedTest(std::string_view test)
{
	if (IsVariant("unimplemented"))
		return test == "callin_unimplemented";
	if (IsVariant("fourmodules"))
		return test == "callin_4modules";
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
	return {};
}

inline Token Begin(std::string_view backend, std::string_view test)
{
	if (!IsEnabled() || !IsBackend(backend) || !IsTrackedTest(test))
		return {};
	return {
		.test = std::string(test),
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
	Samples()[token.test].samples.push_back(static_cast<uint64_t>(elapsed));
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
	for (const auto& [test, sample] : Samples()) {
		if (sample.samples.empty())
			continue;
		// Native modules are separate event clients, so a four-module run
		// records one token per client. Report the per-engine-event fan-out
		// cost, matching the single dispatch token used by Lua and Wasm.
		const double fanout = std::string_view(backend) == "native" &&
			IsVariant("fourmodules") ? 4.0 : 1.0;
		if (sample.samples.size() <= kWarmupSamples ||
			sample.samples.size() - kWarmupSamples < kMinimumSamples) {
			file << "{\"backend\":\"" << backend
				 << "\",\"test\":\"" << test
				 << "\",\"status\":\"unavailable\",\"iterations\":" << sample.samples.size()
				 << ",\"reason\":\"only " << sample.samples.size()
				 << " dispatches were recorded; a callin row needs at least "
				 << (kMinimumSamples + kWarmupSamples) << "\"}\n";
			continue;
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
			 << ",\"measurement\":\"actual engine callin boundary; median of per-dispatch samples\"}\n";
	}
	file.flush();
	flushed = true;
}

} // namespace spring::benchmark_callins
