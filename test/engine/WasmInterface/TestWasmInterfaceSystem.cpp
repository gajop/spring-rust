/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include <catch_amalgamated.hpp>

#include <cstdint>
#include <string>
#include <vector>

#include "WasmInterface/WasmInterfaceSystem.h"

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
	CHECK_FALSE(system.LoadModule({
		.name = "ui-module",
		.source = "ui.wasm",
		.environment = WasmEnvironment::UI,
		.bytes = MinimalCoreModule(),
	}, error));
	CHECK(system.ModuleCount() == 3);

	CHECK(system.UnloadModule("earlier"));
	CHECK_FALSE(system.UnloadModule("earlier"));
	CHECK(system.ModuleCount() == 2);
	system.UnloadAll();
	CHECK(system.ModuleCount() == 0);
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
