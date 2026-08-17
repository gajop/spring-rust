/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <chrono>
#include <cstdint>
#include <cstdlib>
#include <fstream>
#include <iomanip>
#include <sstream>
#include <string>
#include <string_view>
#include <unordered_map>

namespace spring::benchmark_callins {

using Clock = std::chrono::steady_clock;

struct Token {
	std::string test;
	Clock::time_point start;
	bool active = false;
};

struct Stats {
	uint64_t count = 0;
	uint64_t totalNanoseconds = 0;
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
	return configured == backend;
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
	return test == "callin_empty" || test == "callin_gameframe" ||
		test == "callin_update" || test == "callin_drawworld" ||
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
	auto& sample = Samples()[token.test];
	sample.count++;
	sample.totalNanoseconds += static_cast<uint64_t>(elapsed);
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

	for (const auto& [test, sample] : Samples()) {
		if (sample.count == 0)
			continue;
		const double perCallbackNanoseconds = static_cast<double>(sample.totalNanoseconds) /
			static_cast<double>(sample.count);
		// Native modules are separate event clients, so a four-module run
		// records one token per client. Report the per-engine-event fan-out
		// cost, matching the single dispatch token used by Lua and Wasm.
		const double fanout = std::string_view(backend) == "native" &&
			IsVariant("fourmodules") ? 4.0 : 1.0;
		const double meanNanoseconds = perCallbackNanoseconds * fanout;
		file << "{\"backend\":\"" << backend
			 << "\",\"test\":\"" << test
			 << "\",\"status\":\"pass\",\"iterations\":" << sample.count
			 << ",\"meanNs\":" << std::fixed << std::setprecision(3) << meanNanoseconds
			 << ",\"totalMeanNs\":" << sample.totalNanoseconds
			 << ",\"measurement\":\"actual engine callin boundary\"}\n";
	}
	file.flush();
	flushed = true;
}

} // namespace spring::benchmark_callins
