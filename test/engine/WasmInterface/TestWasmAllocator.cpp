/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include <catch_amalgamated.hpp>

#include <cstdint>
#include <string>
#include <string_view>
#include <vector>

#include "ComponentAllocatorFixture.h"
#include "WasmInterface/WasmHost.h"
#include "WasmInterface/WasmModule.h"
#include "WasmInterface/WasmRuntime.h"

namespace {

class ComponentAdapter final : public WasmHostAdapter {
public:
	bool Callout(std::string_view module, std::string_view function,
		const std::vector<WasmValue>& arguments, WasmValue& result,
		std::string& error) override
	{
		if (module != "units_query" || function != "GetBytes" || !arguments.empty()) {
			error = "unexpected allocator-boundary Component Model host call";
			return false;
		}
		++calls;
		result = WasmValue::Bytes({1, 2, 3, 4});
		return true;
	}

	std::size_t calls = 0;
};

WasmModuleDescriptor AllocatorDescriptor()
{
	return {
		.name = "allocator-reentry-fixture",
		.source = "allocator-reentry-fixture.wasm",
		.environment = WasmEnvironment::RulesSynced,
		.bytes = std::vector<std::uint8_t>(wasm_component_fixture::kComponentAllocatorFixture,
			wasm_component_fixture::kComponentAllocatorFixture +
				wasm_component_fixture::kComponentAllocatorFixtureSize),
	};
}

void CheckAllocatorFault(std::string_view exportName, WasmRuntimeConfig config,
	bool expectFuelError = false)
{
	config.allowUnregisteredComponentFunctionsForTesting = true;

	ComponentAdapter adapter;
	WasmRuntime runtime(config);
	WasmModule module(36, AllocatorDescriptor(), runtime, &adapter);
	std::string error;
	REQUIRE(module.Initialize(error));

	WasmValue result;
	CHECK_FALSE(module.Callin(exportName, {}, result, error));
	CHECK(adapter.calls == 1);
	CHECK(module.State() == WasmModuleState::Faulted);
	CHECK_FALSE(error.empty());
	if (expectFuelError) {
		CHECK((error.find("fuel") != std::string::npos ||
			error.find("Fuel") != std::string::npos));
	}
}

} // namespace

TEST_CASE("Wasm canonical allocator cannot re-enter a Spring import")
{
	CheckAllocatorFault("recoil:spring-api/allocator-fixture@1.0.0/probe", {});
}

TEST_CASE("Wasm canonical allocator trap faults the Component call")
{
	CheckAllocatorFault("recoil:spring-api/allocator-fixture@1.0.0/trap", {});
}

TEST_CASE("Wasm canonical allocator burns fuel under the Component guard")
{
	WasmRuntimeConfig config;
	config.instructionFuel = 10'000;
	CheckAllocatorFault("recoil:spring-api/allocator-fixture@1.0.0/burn-fuel",
		config, true);
}
