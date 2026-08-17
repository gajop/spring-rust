/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include <catch_amalgamated.hpp>

#include <cstddef>
#include <cstdint>
#include <string>
#include <string_view>
#include <vector>

#include "ComponentAggregationFalseFixture.h"
#include "ComponentAggregationTrueFixture.h"
#include "System/SyncedTiming.h"
#include "WasmInterface/WasmInterfaceSystem.h"
#include "generated/WasmCallinRegistry.h"

namespace {
	std::vector<std::uint8_t> MinimalCoreModule()
	{
		return {
			0x00, 'a', 's', 'm', 0x01, 0x00, 0x00, 0x00,
		};
	}
}

TEST_CASE("Wasm interface system keeps multiple instances deterministic")
{
	WasmInterfaceSystem system;
	REQUIRE(system.Runtime().IsAvailable());
	std::string error;

	REQUIRE(system.LoadModule({
		.name = "later",
		.source = "later.wasm",
		.environment = WasmEnvironment::RulesSynced,
		.order = 4,
		.bytes = MinimalCoreModule(),
	}, error));
	REQUIRE(system.LoadModule({
		.name = "earlier",
		.source = "earlier.wasm",
		.environment = WasmEnvironment::RulesSynced,
		.order = 2,
		.bytes = MinimalCoreModule(),
	}, error));
	REQUIRE(system.LoadModule({
		.name = "local",
		.source = "local.wasm",
		.environment = WasmEnvironment::RulesUnsynced,
		.order = 0,
		.bytes = MinimalCoreModule(),
	}, error));

	CHECK(system.ModuleCount() == 3);
	const auto configuration = system.SyncedConfiguration();
	REQUIRE(configuration.size() == 2);
	CHECK(configuration[0].find("earlier|rules-synced|2|") == 0);
	CHECK(configuration[1].find("later|rules-synced|4|") == 0);

	CHECK_FALSE(system.LoadModule({
		.name = "earlier",
		.source = "duplicate.wasm",
		.environment = WasmEnvironment::RulesSynced,
		.bytes = MinimalCoreModule(),
	}, error));
	CHECK(system.ModuleCount() == 3);
	REQUIRE(system.LoadModule({
		.name = "ui-module",
		.source = "ui.wasm",
		.environment = WasmEnvironment::UI,
		.bytes = MinimalCoreModule(),
	}, error));
	CHECK(system.ModuleCount() == 4);

	CHECK(system.UnloadModule("earlier"));
	CHECK_FALSE(system.UnloadModule("earlier"));
	CHECK(system.ModuleCount() == 3);
	system.UnloadAll();
	CHECK(system.ModuleCount() == 0);
}

TEST_CASE("Wasm synced messages are a one-way no-op without unsynced modules")
{
	WasmInterfaceSystem system;
	REQUIRE(system.Runtime().IsAvailable());
	std::string error;

	CHECK(system.DispatchSyncedMessage("native-api-direct-message", error));
	CHECK(error.empty());
	CHECK(system.ModuleCount() == 0);
}

TEST_CASE("synced timing requires an explicit opt-in value")
{
	CHECK_FALSE(spring::synced_timing::IsAllowed(true, true, false));
	CHECK(spring::synced_timing::IsAllowed(true, true, true));
	CHECK(spring::synced_timing::IsAllowed(true, false, false));
	CHECK(spring::synced_timing::IsAllowed(false, true, false));

	CHECK_FALSE(spring::synced_timing::IsEnabledSetting(""));
	CHECK_FALSE(spring::synced_timing::IsEnabledSetting("0"));
	CHECK_FALSE(spring::synced_timing::IsEnabledSetting("enabled"));
	CHECK(spring::synced_timing::IsEnabledSetting("1"));
	CHECK(spring::synced_timing::IsEnabledSetting("true"));
	CHECK(spring::synced_timing::IsEnabledSetting("TRUE"));
	CHECK(spring::synced_timing::IsEnabledSetting("yes"));
	CHECK(spring::synced_timing::IsEnabledSetting("YES"));
	CHECK(spring::synced_timing::IsEnabledSetting("on"));
	CHECK(spring::synced_timing::IsEnabledSetting("ON"));
}

TEST_CASE("Wasm Allow callins are synced-only and deny by default")
{
	constexpr std::uint32_t syncedEnvironmentMask = 1u <<
		static_cast<std::uint32_t>(WasmEnvironment::RulesSynced) |
		1u << static_cast<std::uint32_t>(WasmEnvironment::GaiaSynced);

	for (const auto& descriptor : recoil::wasm::generated::kCallins) {
		if (std::string_view(descriptor.name).starts_with("Allow")) {
			CHECK((descriptor.environmentMask & ~syncedEnvironmentMask) == 0);
			if (std::string_view(descriptor.result) == "BoolCallinResult")
				CHECK(std::string_view(descriptor.aggregation) == "and-false");
		}
	}
}

TEST_CASE("Wasm callin aggregation matches Lua defaults across components")
{
	const auto booleanResult = [](bool value) {
		return WasmValue::Record({{"value", WasmValue::Bool(value)}});
	};
	const auto readBoolean = [](const WasmValue& value) {
		const auto* record = std::get_if<WasmValueRecord>(&value.storage);
		REQUIRE(record != nullptr);
		const auto iter = record->find("value");
		REQUIRE(iter != record->end());
		const auto* boolean = std::get_if<bool>(&iter->second.storage);
		REQUIRE(boolean != nullptr);
		return *boolean;
	};

	WasmValue aggregate;
	std::string error;
	bool haveResult = false;
	CHECK(WasmInterfaceSystem::AggregateCallinResult("and-false", booleanResult(true),
		haveResult, aggregate, error));
	CHECK(readBoolean(aggregate));
	CHECK(WasmInterfaceSystem::AggregateCallinResult("and-false", booleanResult(false),
		haveResult, aggregate, error));
	CHECK_FALSE(readBoolean(aggregate));
	CHECK(WasmInterfaceSystem::AggregateCallinResult("and-false", booleanResult(true),
		haveResult, aggregate, error));
	CHECK_FALSE(readBoolean(aggregate));

	haveResult = false;
	aggregate = WasmValue::Unit();
	CHECK(WasmInterfaceSystem::AggregateCallinResult("or-true", booleanResult(false),
		haveResult, aggregate, error));
	CHECK_FALSE(readBoolean(aggregate));
	CHECK(WasmInterfaceSystem::AggregateCallinResult("or-true", booleanResult(true),
		haveResult, aggregate, error));
	CHECK(readBoolean(aggregate));

	// No component result leaves the engine's native default in force.
	haveResult = false;
	aggregate = WasmValue::Unit();
	CHECK_FALSE(haveResult);
	CHECK(aggregate.IsUnit());

	WasmInterfaceSystem system;
	REQUIRE(system.Runtime().IsAvailable());
	WasmValue result;
	CHECK_FALSE(system.DispatchCallin("AllowCommand", {},
		WasmEnvironment::RulesUnsynced, result, error));
	CHECK(error.find("not available") != std::string::npos);
	CHECK_FALSE(system.DispatchCallin("AllowCommand", {}, WasmEnvironment::UI, result, error));
	error.clear();
	CHECK(system.DispatchCallin("AllowCommand", {}, WasmEnvironment::RulesSynced,
		result, error));
	CHECK(error.empty());
	CHECK(result.IsUnit());
}

TEST_CASE("Wasm callin aggregation combines compiled components")
{
	const auto command = WasmValue::Record({
		{"unit-id", WasmValue::I64(1)},
		{"unit-def-id", WasmValue::I64(2)},
		{"unit-team", WasmValue::I64(3)},
		{"command", WasmValue::Record({
			{"id", WasmValue::I64(4)},
			{"time-out", WasmValue::I64(0)},
			{"page-index", WasmValue::U64(0)},
			{"num-params", WasmValue::U64(0)},
			{"tag", WasmValue::U64(0)},
			{"options", WasmValue::U64(0)},
			{"params", WasmValue::List({})},
		})},
		{"player-num", WasmValue::I64(5)},
		{"from-synced", WasmValue::Bool(true)},
		{"from-lua", WasmValue::Bool(false)},
	});
	const auto readAllow = [](const WasmValue& value) {
		const auto* record = std::get_if<WasmValueRecord>(&value.storage);
		REQUIRE(record != nullptr);
		const auto iter = record->find("value");
		REQUIRE(iter != record->end());
		const auto* boolean = std::get_if<bool>(&iter->second.storage);
		REQUIRE(boolean != nullptr);
		return *boolean;
	};
	const auto copyFixture = [](const std::uint8_t* bytes, std::size_t size) {
		return std::vector<std::uint8_t>(bytes, bytes + size);
	};

	WasmInterfaceSystem system;
	REQUIRE(system.Runtime().IsAvailable());
	std::string error;
	REQUIRE(system.LoadModule({
		.name = "allow-false",
		.source = "allow-false.component.wasm",
		.environment = WasmEnvironment::RulesSynced,
		.order = 0,
		.bytes = copyFixture(wasm_component_fixture::kComponentAggregationFalseFixture,
			wasm_component_fixture::kComponentAggregationFalseFixtureSize),
	}, error));
	REQUIRE(system.LoadModule({
		.name = "allow-true",
		.source = "allow-true.component.wasm",
		.environment = WasmEnvironment::RulesSynced,
		.order = 1,
		.bytes = copyFixture(wasm_component_fixture::kComponentAggregationTrueFixture,
			wasm_component_fixture::kComponentAggregationTrueFixtureSize),
	}, error));

	WasmValue result;
	REQUIRE(system.DispatchCallin("AllowCommand", {command},
		WasmEnvironment::RulesSynced, result, error));
	CHECK(error.empty());
	CHECK_FALSE(readAllow(result));

	REQUIRE(system.UnloadModule("allow-false"));
	result = WasmValue::Unit();
	error.clear();
	REQUIRE(system.DispatchCallin("AllowCommand", {command},
		WasmEnvironment::RulesSynced, result, error));
	CHECK(error.empty());
	CHECK(readAllow(result));
}

TEST_CASE("Wasm manifest loading is atomic across multiple declarations")
{
	WasmInterfaceSystem system;
	REQUIRE(system.Runtime().IsAvailable());
	std::string error;

	const auto provider = [](std::string_view path, std::vector<std::uint8_t>& bytes,
		std::string& providerError) {
		if (path == "missing.wasm") {
			providerError = "fixture module is missing";
			return false;
		}
		bytes = MinimalCoreModule();
		return true;
	};

	const std::string manifest =
		"module(alpha, alpha.wasm, rules-synced, 1)\n"
		"module(beta, beta.wasm, rules-synced, 0)\n";
	REQUIRE(system.LoadManifest(manifest, provider, error));
	CHECK(system.ModuleCount() == 2);

	const std::string brokenManifest =
		"module(gamma, gamma.wasm, rules-synced, 2)\n"
		"module(delta, missing.wasm, rules-synced, 3)\n";
	CHECK_FALSE(system.LoadManifest(brokenManifest, provider, error));
	CHECK(system.ModuleCount() == 2);
	const auto configuration = system.SyncedConfiguration();
	REQUIRE(configuration.size() == 2);
	CHECK(configuration[0].find("beta|rules-synced|0|") == 0);
	CHECK(configuration[1].find("alpha|rules-synced|1|") == 0);
}

TEST_CASE("Wasm manifest loading merges archives deterministically and atomically")
{
	WasmInterfaceSystem system;
	REQUIRE(system.Runtime().IsAvailable());
	std::string error;
	std::vector<std::string> requests;
	const auto provider = [&requests](std::string_view archive, std::string_view path,
		std::vector<std::uint8_t>& bytes, std::string& providerError) {
		requests.push_back(std::string(archive) + ":" + std::string(path));
		if (archive == "archive-b" && path == "missing.wasm") {
			providerError = "fixture module is missing";
			return false;
		}
		bytes = MinimalCoreModule();
		return true;
	};

	const std::vector<WasmManifestSource> sources = {
		{"archive-b", "module(beta, beta.wasm, rules-synced, 1)\n"},
		{"archive-a", "module(alpha, alpha.wasm, rules-synced, 1)\n"},
	};
	REQUIRE(system.LoadManifests(sources, provider, error));
	CHECK(requests == std::vector<std::string>{"archive-b:beta.wasm", "archive-a:alpha.wasm"});
	const auto configuration = system.SyncedConfiguration();
	REQUIRE(configuration.size() == 2);
	CHECK(configuration[0].find("alpha|rules-synced|1|archive-a|") == 0);
	CHECK(configuration[1].find("beta|rules-synced|1|archive-b|") == 0);

	requests.clear();
	const std::vector<WasmManifestSource> brokenSources = {
		{"archive-c", "module(gamma, gamma.wasm, rules-synced, 2)\n"},
		{"archive-b", "module(delta, missing.wasm, rules-synced, 3)\n"},
	};
	CHECK_FALSE(system.LoadManifests(brokenSources, provider, error));
	CHECK(system.ModuleCount() == 2);
	CHECK(requests == std::vector<std::string>{"archive-c:gamma.wasm", "archive-b:missing.wasm"});

	const std::vector<WasmManifestSource> duplicateSources = {
		{"archive-c", "module(alpha, other.wasm, rules-synced, 2)\n"},
	};
	CHECK_FALSE(system.LoadManifests(duplicateSources, provider, error));
	CHECK(error.find("duplicate") != std::string::npos);
	CHECK(error.find("alpha") != std::string::npos);
	CHECK(system.ModuleCount() == 2);
}
