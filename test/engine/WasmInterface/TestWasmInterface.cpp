/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include <catch_amalgamated.hpp>

#include <algorithm>
#include <chrono>
#include <cstdint>
#include <string>
#include <vector>

#include "ComponentHostFixture.h"
#include "ComponentComplexFixture.h"
#include "ComponentAllocatorFixture.h"
#include "ComponentSemanticFixture.h"
#include "ComponentRustFixture.h"
#include "ComponentValueFixture.h"
#include "NativeInterface/NativeInterface.h"
#include "NativeInterface/NativeInterfaceWasmAdapter.h"
#include "WasmInterface/WasmEnvironment.h"
#include "WasmInterface/WasmHost.h"
#include "WasmInterface/WasmModule.h"
#include "WasmInterface/WasmResources.h"
#include "WasmInterface/WasmRuntime.h"

namespace {
	void BenchmarkGetTeamUnitCount(const GetTeamUnitCountQuery* query,
		GetTeamUnitCountResult* result)
	{
		result->error = nullptr;
		result->count = static_cast<std::uint32_t>(query->teamID + 10);
	}

	std::vector<std::uint8_t> WrapNestedComponent(const std::vector<std::uint8_t>& child)
	{
		// Component section 4 contains a complete nested component.  Keeping the
		// encoder here makes the boundary fixture independent of a host-side
		// wasm-tools installation and lets the validator see real recursive type
		// structure rather than merely a max-depth setting.
		std::vector<std::uint8_t> result = {
			0x00, 'a', 's', 'm', 0x0d, 0x00, 0x01, 0x00, 0x04,
		};
		std::size_t size = child.size();
		while (true) {
			std::uint8_t byte = static_cast<std::uint8_t>(size & 0x7f);
			size >>= 7;
			if (size != 0)
				byte |= 0x80;
			result.push_back(byte);
			if (size == 0)
				break;
		}
		result.insert(result.end(), child.begin(), child.end());
		return result;
	}

#if defined(__SANITIZE_ADDRESS__)
constexpr bool kBuildUsesAsan = true;
#elif defined(__clang__)
#if __has_feature(address_sanitizer)
constexpr bool kBuildUsesAsan = true;
#else
constexpr bool kBuildUsesAsan = false;
#endif
#else
constexpr bool kBuildUsesAsan = false;
#endif

} // namespace

TEST_CASE("Wasm environments keep synced and unsynced worlds separate")
{
	WasmEnvironment environment = WasmEnvironment::RulesSynced;
	CHECK(WasmEnvironmentMatrix::Parse("rules-synced", environment));
	CHECK(environment == WasmEnvironment::RulesSynced);
	CHECK(WasmEnvironmentMatrix::Policy(environment).synced);
	CHECK(WasmEnvironmentMatrix::IsRuntimeEnabled(environment));

	CHECK(WasmEnvironmentMatrix::Parse("rules-unsynced", environment));
	CHECK_FALSE(WasmEnvironmentMatrix::Policy(environment).synced);
	CHECK(WasmEnvironmentMatrix::HasModule(environment, "unsynced_read"));
	CHECK_FALSE(WasmEnvironmentMatrix::HasModule(environment, "synced_ctrl"));
	CHECK(WasmEnvironmentMatrix::IsRuntimeEnabled(WasmEnvironment::UI));
	CHECK_FALSE(WasmEnvironmentMatrix::Policy(WasmEnvironment::UI).synced);
	CHECK_FALSE(WasmEnvironmentMatrix::Policy(WasmEnvironment::UI).permitsSimulationMutation);
	CHECK(WasmEnvironmentMatrix::HasModule(WasmEnvironment::UI, "units_info"));
	CHECK(WasmEnvironmentMatrix::HasModule(WasmEnvironment::UI, "unit_control"));
}

TEST_CASE("Wasm resources reject foreign and stale handles")
{
	WasmResourceTable resources(1);
	const WasmHandle handle = resources.Insert(1, "texture");
	REQUIRE(handle != 0);
	CHECK(resources.Insert(1, "texture") == 0);
	CHECK(resources.Validate(handle, 1, "texture"));
	CHECK_FALSE(resources.Validate(handle, 2, "texture"));
	CHECK_FALSE(resources.Validate(handle, 1, "buffer"));
	REQUIRE(resources.Drop(handle, 1, "texture"));
	CHECK_FALSE(resources.Validate(handle, 1, "texture"));
	const WasmHandle reused = resources.Insert(1, "texture");
	CHECK(reused != handle);
}

TEST_CASE("Wasm budget accounts guest, host and result work")
{
	WasmExecutionBudget budget(3, 4, 8);
	CHECK(budget.ChargeGuest(2));
	CHECK_FALSE(budget.ChargeGuest(2));
	CHECK(budget.ChargeHost(4));
	CHECK_FALSE(budget.ChargeHost(1));
	CHECK(budget.CheckResultSize(8));
	CHECK_FALSE(budget.CheckResultSize(9));
}

TEST_CASE("Wasm callback policy controls host re-entry")
{
	WasmCallbackRegistry callbacks;
	const WasmCallbackID denied = callbacks.Register({false}, [](const auto&) { return true; });
	const WasmCallbackID allowed = callbacks.Register({true}, [](const auto&) { return true; });
	bool reentryAllowed = false;
	CHECK_FALSE(callbacks.Invoke(denied, {}, true, reentryAllowed));
	CHECK(callbacks.Invoke(denied, {}, false, reentryAllowed));
	CHECK_FALSE(reentryAllowed);
	CHECK(callbacks.Invoke(allowed, {}, true, reentryAllowed));
	CHECK(reentryAllowed);
}

TEST_CASE("Wasm callback re-entry permission remains active through nested callbacks")
{
	WasmExecutionBudget budget(100, 100, 100);
	CHECK(budget.CallbackReentryAllowed());

	REQUIRE(budget.EnterCallback(false));
	CHECK_FALSE(budget.CallbackReentryAllowed());
	CHECK_FALSE(budget.EnterImport(budget.CallbackReentryAllowed()));

	budget.LeaveCallback();
	CHECK(budget.CallbackReentryAllowed());
	REQUIRE(budget.EnterCallback(true));
	CHECK(budget.CallbackReentryAllowed());
	REQUIRE(budget.EnterImport(budget.CallbackReentryAllowed()));
	budget.LeaveImport();
	budget.LeaveCallback();
	CHECK(budget.CallbackReentryAllowed());
}

TEST_CASE("Wasm module cleanup runs before backend teardown")
{
	const std::vector<std::uint8_t> validCore = {
		0x00, 'a', 's', 'm', 0x01, 0x00, 0x00, 0x00,
	};
	WasmRuntime runtime;
	WasmModule module(9, {
		.name = "cleanup-fixture",
		.source = "cleanup-fixture.wasm",
		.environment = WasmEnvironment::RulesSynced,
		.bytes = validCore,
	}, runtime);
	std::string error;
	REQUIRE(module.Initialize(error));
	std::vector<int> cleanupOrder;
	REQUIRE(module.RegisterCleanup([&cleanupOrder]() { cleanupOrder.push_back(1); }));
	REQUIRE(module.RegisterCleanup([&cleanupOrder]() { cleanupOrder.push_back(2); }));
	module.Shutdown();
	CHECK(cleanupOrder == std::vector<int>{2, 1});
	module.Shutdown();
	CHECK(cleanupOrder == std::vector<int>{2, 1});
}

TEST_CASE("Wasm fault cleanup is isolated across multiple instances")
{
	const std::vector<std::uint8_t> validCore = {
		0x00, 'a', 's', 'm', 0x01, 0x00, 0x00, 0x00,
	};
	WasmRuntime runtime;
	WasmModule first(10, {
		.name = "fault-first",
		.source = "fault-first.wasm",
		.environment = WasmEnvironment::RulesUnsynced,
		.bytes = validCore,
	}, runtime);
	WasmModule second(11, {
		.name = "fault-second",
		.source = "fault-second.wasm",
		.environment = WasmEnvironment::RulesUnsynced,
		.bytes = validCore,
	}, runtime);
	std::string error;
	REQUIRE(first.Initialize(error));
	REQUIRE(second.Initialize(error));
	std::vector<int> cleanup;
	REQUIRE(first.RegisterCleanup([&cleanup]() { cleanup.push_back(1); }));
	REQUIRE(second.RegisterCleanup([&cleanup]() { cleanup.push_back(2); }));
	first.Fault("synthetic unsynced trap");
	CHECK(first.State() == WasmModuleState::Faulted);
	CHECK(second.State() == WasmModuleState::Running);
	first.Shutdown();
	CHECK(cleanup == std::vector<int>{1});
	CHECK(second.State() == WasmModuleState::Running);
	second.Shutdown();
	CHECK(cleanup == std::vector<int>{1, 2});
}

TEST_CASE("Wasm validation fails closed for invalid and oversized inputs")
{
	WasmRuntimeConfig config;
	config.maxModuleBytes = 8;
	WasmRuntime runtime(config);

	const std::vector<std::uint8_t> validCore = {0x00, 'a', 's', 'm', 0x01, 0x00, 0x00, 0x00};
	CHECK(runtime.ValidateModule(validCore, WasmEnvironment::RulesSynced, "rules-synced").valid);
	CHECK_FALSE(runtime.ValidateModule({}, WasmEnvironment::RulesSynced, "rules-synced").valid);
	CHECK(runtime.ValidateModule(validCore, WasmEnvironment::UI, "ui").valid);

	const std::vector<std::uint8_t> oversized(9, 0);
	CHECK_FALSE(runtime.ValidateModule(oversized, WasmEnvironment::RulesSynced, "rules-synced").valid);

	const std::vector<std::uint8_t> component = {
		0x00, 'a', 's', 'm', 0x0d, 0x00, 0x01, 0x00,
	};
	CHECK(runtime.ValidateModule(component, WasmEnvironment::RulesSynced, "rules-synced").valid);
	const std::vector<std::uint8_t> validComponent(
		wasm_component_fixture::kComponentHostFixture,
		wasm_component_fixture::kComponentHostFixture +
			wasm_component_fixture::kComponentHostFixtureSize);
	WasmRuntime componentRuntime;
	CHECK(componentRuntime.ValidateModule(validComponent, WasmEnvironment::RulesSynced,
		"rules-synced").valid);
	WasmRuntimeConfig nestingConfig;
	nestingConfig.maxComponentNesting = 0;
	WasmRuntime nestingRuntime(nestingConfig);
	const auto nestingResult = nestingRuntime.ValidateModule(validComponent,
		WasmEnvironment::RulesSynced, "rules-synced");
	CHECK_FALSE(nestingResult.valid);
	CHECK(nestingResult.error.find("nesting exceeds configured maximum") != std::string::npos);
	const auto pathologicalComponent = WrapNestedComponent(WrapNestedComponent(validComponent));
	WasmRuntimeConfig pathologicalConfig;
	pathologicalConfig.maxComponentNesting = 1;
	WasmRuntime pathologicalRuntime(pathologicalConfig);
	const auto pathologicalResult = pathologicalRuntime.ValidateModule(pathologicalComponent,
		WasmEnvironment::RulesSynced, "rules-synced");
	CHECK_FALSE(pathologicalResult.valid);
	CHECK(pathologicalResult.error.find("nesting exceeds configured maximum") != std::string::npos);
	std::vector<std::uint8_t> incompatibleVersion = validComponent;
	const std::string componentVersion = "@1.0.0";
	const auto versionBegin = std::search(incompatibleVersion.begin(), incompatibleVersion.end(),
		componentVersion.begin(), componentVersion.end());
	REQUIRE(versionBegin != incompatibleVersion.end());
	*(versionBegin + 1) = '2';
	CHECK_FALSE(componentRuntime.ValidateModule(incompatibleVersion,
		WasmEnvironment::RulesSynced, "rules-synced").valid);
	const auto incompatibleInterface = componentRuntime.ValidateModule(validComponent,
		WasmEnvironment::RulesSynced, "rules-synced", "2.0.0");
	CHECK_FALSE(incompatibleInterface.valid);
	CHECK(incompatibleInterface.error.find("interface version") != std::string::npos);
	std::vector<std::uint8_t> deniedComponent = validComponent;
	const std::string deniedPrefix = "recoil:";
	const auto deniedPrefixBegin = std::search(deniedComponent.begin(), deniedComponent.end(),
		deniedPrefix.begin(), deniedPrefix.end());
	REQUIRE(deniedPrefixBegin != deniedComponent.end());
	*deniedPrefixBegin = 'e';
	CHECK_FALSE(componentRuntime.ValidateModule(deniedComponent, WasmEnvironment::RulesSynced,
		"rules-synced").valid);
	std::vector<std::uint8_t> unknownComponent = validComponent;
	const std::string knownComponentModule = "units-query";
	const auto knownComponentModuleBegin = std::search(unknownComponent.begin(),
		unknownComponent.end(), knownComponentModule.begin(), knownComponentModule.end());
	REQUIRE(knownComponentModuleBegin != unknownComponent.end());
	*knownComponentModuleBegin = 'x';
	CHECK_FALSE(componentRuntime.ValidateModule(unknownComponent,
		WasmEnvironment::RulesSynced, "rules-synced").valid);
	std::vector<std::uint8_t> unknownComponentFunction = validComponent;
	const std::string knownComponentFunction = "get-team-unit-count";
	const auto knownComponentFunctionBegin = std::search(unknownComponentFunction.begin(),
		unknownComponentFunction.end(), knownComponentFunction.begin(), knownComponentFunction.end());
	REQUIRE(knownComponentFunctionBegin != unknownComponentFunction.end());
	*(knownComponentFunctionBegin + knownComponentFunction.find("count")) = 'x';
	const auto unknownFunctionResult = componentRuntime.ValidateModule(unknownComponentFunction,
		WasmEnvironment::RulesSynced, "rules-synced");
	CHECK_FALSE(unknownFunctionResult.valid);
	CHECK((unknownFunctionResult.error.find("unknown function") != std::string::npos ||
		unknownFunctionResult.error.find("invalid Wasm Component Model binary") != std::string::npos));

	WasmRuntimeConfig policyConfig;
	policyConfig.maxMemoryPages = 1;
	policyConfig.maxTableElements = 4;
	policyConfig.maxExports = 0;
	WasmRuntime policyRuntime(policyConfig);
	const std::vector<std::uint8_t> deniedImport = {
		0x00, 'a', 's', 'm', 0x01, 0x00, 0x00, 0x00,
		0x02, 0x09, 0x01, 0x04, 'e', 'v', 'i', 'l', 0x01, 'x', 0x00, 0x00,
	};
	CHECK_FALSE(policyRuntime.ValidateModule(deniedImport, WasmEnvironment::RulesSynced, "rules-synced").valid);
	const std::vector<std::uint8_t> supportedCoreImport = {
		0x00, 'a', 's', 'm', 0x01, 0x00, 0x00, 0x00,
		0x02, 0x12, 0x01, 0x06, 's', 'p', 'r', 'i', 'n', 'g',
		0x07, 'a', 'd', 'd', '-', 'i', '3', '2', 0x00, 0x00,
	};
	CHECK(policyRuntime.ValidateModule(supportedCoreImport, WasmEnvironment::RulesSynced,
		"rules-synced").valid);
	const std::vector<std::uint8_t> unknownCoreImport = {
		0x00, 'a', 's', 'm', 0x01, 0x00, 0x00, 0x00,
		0x02, 0x0f, 0x01, 0x06, 's', 'p', 'r', 'i', 'n', 'g',
		0x04, 'e', 'v', 'i', 'l', 0x00, 0x00,
	};
	const auto unknownCoreResult = policyRuntime.ValidateModule(unknownCoreImport,
		WasmEnvironment::RulesSynced, "rules-synced");
	CHECK_FALSE(unknownCoreResult.valid);
	CHECK(unknownCoreResult.error.find("unknown or unsupported Wasm core import") !=
		std::string::npos);
	CHECK_FALSE(policyRuntime.ValidateModule(validComponent, WasmEnvironment::RulesSynced,
		"rules-synced").valid);

	const std::vector<std::uint8_t> oversizedMemory = {
		0x00, 'a', 's', 'm', 0x01, 0x00, 0x00, 0x00,
		0x05, 0x04, 0x01, 0x00, 0x80, 0x08,
	};
	CHECK_FALSE(policyRuntime.ValidateModule(oversizedMemory, WasmEnvironment::RulesSynced, "rules-synced").valid);
	const std::vector<std::uint8_t> oversizedTable = {
		0x00, 'a', 's', 'm', 0x01, 0x00, 0x00, 0x00,
		0x04, 0x05, 0x01, 0x70, 0x01, 0x00, 0x05,
	};
	CHECK_FALSE(policyRuntime.ValidateModule(oversizedTable, WasmEnvironment::RulesSynced, "rules-synced").valid);
	const std::vector<std::uint8_t> oneExport = {
		0x00, 'a', 's', 'm', 0x01, 0x00, 0x00, 0x00,
		0x07, 0x07, 0x01, 0x03, 'r', 'u', 'n', 0x00, 0x00,
	};
	CHECK_FALSE(policyRuntime.ValidateModule(oneExport, WasmEnvironment::RulesSynced, "rules-synced").valid);
	CHECK_FALSE(policyRuntime.CanDeserializeAot("module", "runtime"));
	WasmRuntimeConfig aotConfig;
	aotConfig.allowAotDeserialization = true;
	WasmRuntime aotRuntime(aotConfig);
	CHECK_FALSE(aotRuntime.CanDeserializeAot("valid-hash", "matching-runtime"));
	CHECK_FALSE(aotRuntime.CanDeserializeAot("tampered-hash", "matching-runtime"));
	CHECK(componentRuntime.ConfigurationIdentity().find("wasmtime=42.0.1") != std::string::npos);
	CHECK(componentRuntime.ConfigurationIdentity().find("nan-canonicalization=1") != std::string::npos);
	CHECK(componentRuntime.ConfigurationIdentity() != policyRuntime.ConfigurationIdentity());
	CHECK_FALSE(WasmEnvironmentMatrix::HasModule(WasmEnvironment::RulesSynced, "gfx"));
	CHECK(WasmEnvironmentMatrix::HasModule(WasmEnvironment::RulesUnsynced, "gfx"));
}

TEST_CASE("Wasm module instantiates through the pinned backend")
{
	const std::vector<std::uint8_t> validCore = {
		0x00, 'a', 's', 'm', 0x01, 0x00, 0x00, 0x00,
	};
	WasmRuntimeConfig runtimeConfig;
	runtimeConfig.resultBytesLimit = 4;
	WasmRuntime runtime(runtimeConfig);
	REQUIRE(runtime.IsAvailable());

	WasmModule module(1, {
		.name = "backend-fixture",
		.source = "test",
		.environment = WasmEnvironment::RulesSynced,
		.bytes = validCore,
	}, runtime);
	std::string error;
	REQUIRE(module.Initialize(error));
	CHECK(module.State() == WasmModuleState::Running);
	CHECK_FALSE(module.Callin("missing", {}, error));
	CHECK_FALSE(error.empty());
	WasmValue semanticResult;
	error.clear();
	CHECK_FALSE(module.Callin("missing", {WasmValue::String("12345")}, semanticResult, error));
	CHECK(error == "Wasm callin arguments exceed the configured byte limit");
	module.Shutdown();
	CHECK(module.State() == WasmModuleState::Stopped);
}

TEST_CASE("Wasm core guest calls the owned host adapter")
{
	class ScalarAdapter final : public WasmHostAdapter {
	public:
		bool Callout(std::string_view module, std::string_view function,
			const std::vector<WasmValue>& arguments, WasmValue& result,
			std::string& error) override
		{
			if (module != "spring" || function != "add-i32" || arguments.size() != 1) {
				error = "unexpected scalar host call";
				return false;
			}
			const auto* value = std::get_if<std::int64_t>(&arguments.front().storage);
			if (value == nullptr) {
				error = "scalar host argument has the wrong type";
				return false;
			}
			result = WasmValue::I64(*value + 1);
			return true;
		}
	};

	const std::vector<std::uint8_t> moduleBytes = {
		0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x06, 0x01, 0x60,
		0x01, 0x7f, 0x01, 0x7f, 0x02, 0x12, 0x01, 0x06, 0x73, 0x70, 0x72, 0x69,
		0x6e, 0x67, 0x07, 0x61, 0x64, 0x64, 0x2d, 0x69, 0x33, 0x32, 0x00, 0x00,
		0x03, 0x02, 0x01, 0x00, 0x07, 0x07, 0x01, 0x03, 0x72, 0x75, 0x6e, 0x00,
		0x01, 0x0a, 0x08, 0x01, 0x06, 0x00, 0x20, 0x00, 0x10, 0x00, 0x0b,
	};
	ScalarAdapter adapter;
	WasmRuntime runtime;
	WasmModule module(2, {
		.name = "host-scalar",
		.source = "host_scalar.wat",
		.environment = WasmEnvironment::RulesSynced,
		.bytes = moduleBytes,
	}, runtime, &adapter);
	std::string error;
	REQUIRE(module.Initialize(error));
	std::vector<std::uint64_t> results;
	REQUIRE(module.Callin("run", {41}, results, error));
	REQUIRE(results.size() == 1);
	CHECK(results.front() == 42);
}

TEST_CASE("Wasm callouts enforce the result byte limit")
{
	class LargeResultAdapter final : public WasmHostAdapter {
	public:
		bool Callout(std::string_view, std::string_view, const std::vector<WasmValue>&,
			WasmValue& result, std::string&) override
		{
			result = WasmValue::String("12345");
			return true;
		}
	};

	const std::vector<std::uint8_t> validCore = {
		0x00, 'a', 's', 'm', 0x01, 0x00, 0x00, 0x00,
	};
	WasmRuntimeConfig config;
	config.resultBytesLimit = 4;
	WasmRuntime runtime(config);
	LargeResultAdapter adapter;
	WasmModule module(7, {
		.name = "result-limit",
		.source = "result-limit.wasm",
		.environment = WasmEnvironment::RulesSynced,
		.bytes = validCore,
	}, runtime, &adapter);
	std::string error;
	REQUIRE(module.Initialize(error));
	WasmValue result;
	CHECK_FALSE(module.InvokeCallout("test", "large", {}, result, error));
	CHECK(error == "Wasm callout result exceeds the configured byte limit");
	CHECK(module.State() == WasmModuleState::Running);
}

TEST_CASE("Wasm callout re-entry is denied across the complete host boundary")
{
	class ReentrantAdapter final : public WasmHostAdapter {
	public:
		bool Callout(std::string_view, std::string_view, const std::vector<WasmValue>&,
			WasmValue&, std::string& error) override
		{
			error = "unexpected non-owner-aware host call";
			return false;
		}

		bool Callout(WasmModule& owner, std::string_view, std::string_view,
			const std::vector<WasmValue>&, WasmValue&, std::string& error) override
		{
			WasmValue nestedResult;
			std::string nestedError;
			CHECK_FALSE(owner.InvokeCallout("nested", "call", {}, nestedResult, nestedError));
			CHECK(nestedError == "Wasm import re-entry denied");
			error = "outer host call failed after the nested call was denied";
			return false;
		}
	};

	const std::vector<std::uint8_t> validCore = {
		0x00, 'a', 's', 'm', 0x01, 0x00, 0x00, 0x00,
	};
	WasmRuntime runtime;
	ReentrantAdapter adapter;
	WasmModule module(32, {
		.name = "re-entry-fixture",
		.source = "re-entry-fixture.wasm",
		.environment = WasmEnvironment::RulesSynced,
		.bytes = validCore,
	}, runtime, &adapter);
	std::string error;
	REQUIRE(module.Initialize(error));
	WasmValue result;
	CHECK_FALSE(module.InvokeCallout("outer", "call", {}, result, error));
	CHECK(error.find("outer host call failed") != std::string::npos);
	CHECK(module.State() == WasmModuleState::Running);
}

TEST_CASE("Wasm fuel exhaustion faults a core instance")
{
	const std::vector<std::uint8_t> loopingModule = {
		0x00, 'a', 's', 'm', 0x01, 0x00, 0x00, 0x00, 0x01, 0x04, 0x01, 0x60,
		0x00, 0x00, 0x03, 0x02, 0x01, 0x00, 0x07, 0x08, 0x01, 0x04, 'l', 'o',
		'o', 'p', 0x00, 0x00, 0x0a, 0x09, 0x01, 0x07, 0x00, 0x03, 0x40, 0x0c,
		0x00, 0x0b, 0x0b,
	};
	WasmRuntimeConfig config;
	config.instructionFuel = 100;
	WasmRuntime runtime(config);
	WasmModule module(8, {
		.name = "fuel-limit",
		.source = "fuel-limit.wasm",
		.environment = WasmEnvironment::RulesSynced,
		.bytes = loopingModule,
	}, runtime);
	std::string error;
	REQUIRE(module.Initialize(error));
	CHECK_FALSE(module.Callin("loop", {}, error));
	CHECK(module.State() == WasmModuleState::Faulted);
	CHECK_FALSE(module.FaultReason().empty());
}

TEST_CASE("Wasm Component Model imports lower through the host adapter")
{
	class ComponentAdapter final : public WasmHostAdapter {
	public:
		bool Callout(std::string_view module, std::string_view function,
			const std::vector<WasmValue>& arguments, WasmValue& result,
			std::string& error) override
		{
			if (module != "units_query" || function != "GetTeamUnitCount" ||
				arguments.size() != 1) {
				error = "unexpected Component Model host call";
				return false;
			}
			const auto* teamID = std::get_if<std::int64_t>(&arguments.front().storage);
			if (teamID == nullptr) {
				error = "Component Model host argument has the wrong type";
				return false;
			}
			result = WasmValue::U64(static_cast<std::uint64_t>(*teamID + 10));
			return true;
		}
	};

	ComponentAdapter adapter;
	WasmRuntime runtime;
	WasmModule module(3, {
		.name = "component-host-fixture",
		.source = "component-host-fixture.wasm",
		.environment = WasmEnvironment::RulesSynced,
		.bytes = std::vector<std::uint8_t>(wasm_component_fixture::kComponentHostFixture,
			wasm_component_fixture::kComponentHostFixture +
				wasm_component_fixture::kComponentHostFixtureSize),
	}, runtime, &adapter);
	std::string error;
	const bool initialized = module.Initialize(error);
	INFO("Component Model initialization error: " << error);
	REQUIRE(initialized);
	std::vector<std::uint64_t> results;
	REQUIRE(module.Callin("run", {7}, results, error));
	REQUIRE(results.size() == 1);
	CHECK(results.front() == 17);
	WasmValue semanticResult;
	REQUIRE(module.Callin("run", {WasmValue::U64(7)}, semanticResult, error));
	const auto* semanticValue = std::get_if<std::uint64_t>(&semanticResult.storage);
	REQUIRE(semanticValue != nullptr);
	CHECK(*semanticValue == 17);
}

TEST_CASE("Wasm Component Model bounds host value shapes before lifting")
{
	class ComponentAdapter final : public WasmHostAdapter {
	public:
		bool Callout(std::string_view, std::string_view,
			const std::vector<WasmValue>&, WasmValue&, std::string& error) override
		{
			called = true;
			error = "host adapter should not receive an over-budget value";
			return false;
		}

		bool called = false;
	};

	ComponentAdapter adapter;
	WasmRuntimeConfig config;
	config.maxValueNodes = 0;
	WasmRuntime runtime(config);
	WasmModule module(30, {
		.name = "component-value-shape-limit",
		.source = "component-value-shape-limit.wasm",
		.environment = WasmEnvironment::RulesSynced,
		.bytes = std::vector<std::uint8_t>(wasm_component_fixture::kComponentHostFixture,
			wasm_component_fixture::kComponentHostFixture +
				wasm_component_fixture::kComponentHostFixtureSize),
	}, runtime, &adapter);
	std::string error;
	REQUIRE(module.Initialize(error));
	std::vector<std::uint64_t> results;
	CHECK_FALSE(module.Callin("run", {7}, results, error));
	CHECK(error.find("node count exceeds") != std::string::npos);
	CHECK_FALSE(adapter.called);
	CHECK(module.State() == WasmModuleState::Faulted);
}

TEST_CASE("Wasm Component Model canonical result lowering consumes guest fuel")
{
	class ComponentAdapter final : public WasmHostAdapter {
	public:
		bool Callout(std::string_view module, std::string_view function,
			const std::vector<WasmValue>& arguments, WasmValue& result,
			std::string& error) override
		{
			if (module != "units_query" || function != "GetQueryResult" ||
				arguments.size() != 2) {
				error = "unexpected fuel-boundary Component Model host call";
				return false;
			}
			result = WasmValue::Record({
				{"text", WasmValue::String("fuel-boundary")},
				{"bytes", WasmValue::Bytes({1, 2, 3, 4})},
			});
			return true;
		}
	};

	WasmRuntimeConfig config;
	config.instructionFuel = 1;
	config.allowUnregisteredComponentFunctionsForTesting = true;
	WasmRuntime runtime(config);
	ComponentAdapter adapter;
	WasmModule module(33, {
		.name = "canonical-fuel-fixture",
		.source = "canonical-fuel-fixture.wasm",
		.environment = WasmEnvironment::RulesSynced,
		.bytes = std::vector<std::uint8_t>(wasm_component_fixture::kComponentComplexFixture,
			wasm_component_fixture::kComponentComplexFixture +
				wasm_component_fixture::kComponentComplexFixtureSize),
	}, runtime, &adapter);
	std::string error;
	REQUIRE(module.Initialize(error));
	std::vector<std::uint64_t> results;
	CHECK_FALSE(module.Callin("run", {0}, results, error));
	CHECK(module.State() == WasmModuleState::Faulted);
	CHECK((error.find("fuel") != std::string::npos ||
		error.find("Fuel") != std::string::npos));
}

TEST_CASE("Wasm transport performance gates")
{
	using Clock = std::chrono::steady_clock;
	using Nanoseconds = std::chrono::nanoseconds;
	constexpr std::size_t scalarIterations = 1000;
	constexpr std::size_t componentIterations = 100;
	constexpr std::size_t callbackIterations = 10000;
	constexpr std::size_t gfxIterations = 5000;

	// Keep each operation to a small, explicit fraction of the frame budget.
	// The scalar/callback paths are intended for hot simulation work at 30 Hz;
	// the shaped Gfx path is intended for a 120 Hz UI/render frame.  The
	// Component Model result path gets a larger allowance because it copies a
	// string, a byte list, and a record, but it is still a measured budget rather
	// than a process-liveness timeout.
	constexpr auto simulationFrameBudget = Nanoseconds(33'000'000);
	constexpr auto uiFrameBudget = Nanoseconds(8'000'000);
	// These are measured non-ASAN budgets, each derived from a small fraction
	// of the frame it can consume.  They are intentionally close to the
	// observed values (scalar <= 2 us, shaped Component result <= 6 us,
	// callback <= 0.5 us, and Gfx <= 1 us) rather than hang-detection limits.
	constexpr auto scalarBudget = simulationFrameBudget / 16'500;
	constexpr auto componentBudget = simulationFrameBudget / 5'500;
	constexpr auto callbackBudget = uiFrameBudget / 16'000;
	constexpr auto gfxBudget = uiFrameBudget / 8'000;

	WasmRuntimeConfig benchmarkConfig;
	benchmarkConfig.instructionFuel = 1'000'000'000;
	benchmarkConfig.hostWorkLimit = 1'000'000'000;
	benchmarkConfig.allowUnregisteredComponentFunctionsForTesting = true;
	WasmRuntime benchmarkRuntime(benchmarkConfig);

	class ScalarAdapter final : public WasmHostAdapter {
	public:
		bool Callout(std::string_view, std::string_view,
			const std::vector<WasmValue>& arguments, WasmValue& result,
			std::string& error) override
		{
			if (arguments.size() != 1) {
				error = "benchmark scalar arity mismatch";
				return false;
			}
			const auto* value = std::get_if<std::int64_t>(&arguments.front().storage);
			if (value == nullptr) {
				error = "benchmark scalar type mismatch";
				return false;
			}
			result = WasmValue::I64(*value + 1);
			return true;
		}
	};

	const std::vector<std::uint8_t> scalarBytes = {
		0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x06, 0x01, 0x60,
		0x01, 0x7f, 0x01, 0x7f, 0x02, 0x12, 0x01, 0x06, 0x73, 0x70, 0x72, 0x69,
		0x6e, 0x67, 0x07, 0x61, 0x64, 0x64, 0x2d, 0x69, 0x33, 0x32, 0x00, 0x00,
		0x03, 0x02, 0x01, 0x00, 0x07, 0x07, 0x01, 0x03, 0x72, 0x75, 0x6e, 0x00,
		0x01, 0x0a, 0x08, 0x01, 0x06, 0x00, 0x20, 0x00, 0x10, 0x00, 0x0b,
	};
	ScalarAdapter scalarAdapter;
	WasmModule scalarModule(34, {
		.name = "benchmark-scalar",
		.source = "benchmark-scalar.wasm",
		.environment = WasmEnvironment::RulesSynced,
		.bytes = scalarBytes,
	}, benchmarkRuntime, &scalarAdapter);
	std::string error;
	REQUIRE(scalarModule.Initialize(error));
	auto scalarStart = Clock::now();
	std::vector<std::uint64_t> scalarResults;
	bool scalarSucceeded = true;
	for (std::size_t index = 0; index < scalarIterations; ++index) {
		if (!scalarModule.Callin("run", {41}, scalarResults, error) ||
			scalarResults.size() != 1) {
			scalarSucceeded = false;
			break;
		}
	}
	const auto scalarElapsed = Clock::now() - scalarStart;
	REQUIRE(scalarSucceeded);
	const auto scalarPerCall = scalarElapsed / scalarIterations;
	INFO("scalar core callin: " <<
		std::chrono::duration_cast<Nanoseconds>(scalarPerCall).count() <<
		" ns/call (budget " << scalarBudget.count() << " ns)");
	if constexpr (kBuildUsesAsan) {
		WARN("ASAN build: performance budget is reported but not enforced");
	} else {
		CHECK(scalarPerCall <= scalarBudget);
	}

	class ComponentAdapter final : public WasmHostAdapter {
	public:
		bool Callout(std::string_view, std::string_view,
			const std::vector<WasmValue>&, WasmValue& result,
			std::string&) override
		{
			result = WasmValue::Record({
				{"text", WasmValue::String("benchmark")},
				{"bytes", WasmValue::Bytes({1, 2, 3, 4, 5, 6, 7, 8})},
			});
			return true;
		}
	};
	ComponentAdapter componentAdapter;
	WasmModule componentModule(35, {
		.name = "benchmark-component",
		.source = "benchmark-component.wasm",
		.environment = WasmEnvironment::RulesSynced,
		.bytes = std::vector<std::uint8_t>(wasm_component_fixture::kComponentComplexFixture,
			wasm_component_fixture::kComponentComplexFixture +
				wasm_component_fixture::kComponentComplexFixtureSize),
	}, benchmarkRuntime, &componentAdapter);
	REQUIRE(componentModule.Initialize(error));
	auto componentStart = Clock::now();
	bool componentSucceeded = true;
	for (std::size_t index = 0; index < componentIterations; ++index) {
		std::vector<std::uint64_t> results;
		if (!componentModule.Callin("run", {0}, results, error) || results.size() != 1) {
			componentSucceeded = false;
			break;
		}
	}
	const auto componentElapsed = Clock::now() - componentStart;
	REQUIRE(componentSucceeded);
	const auto componentPerCall = componentElapsed / componentIterations;
	INFO("string/list/record Component callin: " <<
		std::chrono::duration_cast<Nanoseconds>(componentPerCall).count() <<
		" ns/call (budget " << componentBudget.count() << " ns)");
	if constexpr (kBuildUsesAsan) {
		WARN("ASAN build: performance budget is reported but not enforced");
	} else {
		CHECK(componentPerCall <= componentBudget);
	}

	WasmCallbackRegistry callbacks;
	const WasmCallbackID callbackID = callbacks.Register({true},
		[](const std::vector<std::uint64_t>&) { return true; });
	REQUIRE(callbackID != 0);
	auto callbackStart = Clock::now();
	bool callbacksSucceeded = true;
	for (std::size_t index = 0; index < callbackIterations; ++index) {
		bool reentryAllowed = false;
		callbacksSucceeded = callbacks.Invoke(callbackID, {index}, false, reentryAllowed) &&
			reentryAllowed && callbacksSucceeded;
	}
	const auto callbackElapsed = Clock::now() - callbackStart;
	const auto callbackPerCall = callbackElapsed / callbackIterations;
	CHECK(callbacksSucceeded);
	INFO("re-entrant callback registry dispatch: " <<
		std::chrono::duration_cast<Nanoseconds>(callbackPerCall).count() <<
		" ns/call (budget " << callbackBudget.count() << " ns)");
	if constexpr (kBuildUsesAsan) {
		WARN("ASAN build: performance budget is reported but not enforced");
	} else {
		CHECK(callbackPerCall <= callbackBudget);
	}

	class GfxAdapter final : public WasmHostAdapter {
	public:
		bool Callout(std::string_view module, std::string_view function,
			const std::vector<WasmValue>&, WasmValue& result,
			std::string& error) override
		{
			if (module != "gfx" || function != "GetMatrixData") {
				error = "benchmark Gfx shape mismatch";
				return false;
			}
			result = WasmValue::List({
				WasmValue::F64(1), WasmValue::F64(0), WasmValue::F64(0), WasmValue::F64(0),
				WasmValue::F64(0), WasmValue::F64(1), WasmValue::F64(0), WasmValue::F64(0),
				WasmValue::F64(0), WasmValue::F64(0), WasmValue::F64(1), WasmValue::F64(0),
				WasmValue::F64(0), WasmValue::F64(0), WasmValue::F64(0), WasmValue::F64(1),
			});
			return true;
		}
	};
	GfxAdapter gfxAdapter;
	WasmModule gfxModule(36, {
		.name = "benchmark-gfx-shape",
		.source = "benchmark-gfx-shape.wasm",
		.environment = WasmEnvironment::RulesUnsynced,
		.bytes = {0x00, 'a', 's', 'm', 0x01, 0x00, 0x00, 0x00},
	}, benchmarkRuntime, &gfxAdapter);
	REQUIRE(gfxModule.Initialize(error));
	auto gfxStart = Clock::now();
	bool gfxSucceeded = true;
	for (std::size_t index = 0; index < gfxIterations; ++index) {
		WasmValue result;
		if (!gfxModule.InvokeCallout("gfx", "GetMatrixData", {}, result, error)) {
			gfxSucceeded = false;
			break;
		}
	}
	const auto gfxElapsed = Clock::now() - gfxStart;
	const auto gfxPerCall = gfxElapsed / gfxIterations;
	REQUIRE(gfxSucceeded);
	INFO("Gfx-shaped list callout: " <<
		std::chrono::duration_cast<Nanoseconds>(gfxPerCall).count() <<
		" ns/call (budget " << gfxBudget.count() << " ns)");
	if constexpr (kBuildUsesAsan) {
		WARN("ASAN build: performance budget is reported but not enforced");
	} else {
		CHECK(gfxPerCall <= gfxBudget);
	}
}

TEST_CASE("Wasm generated adapter transport performance")
{
	using Clock = std::chrono::steady_clock;
	using Nanoseconds = std::chrono::nanoseconds;
	constexpr std::size_t iterations = 100;
	constexpr auto simulationFrameBudget = Nanoseconds(33'000'000);
	constexpr auto generatedAdapterBudget = simulationFrameBudget / 4'000;

	UnitsQueryApi unitsQuery{};
	unitsQuery.GetTeamUnitCount = BenchmarkGetTeamUnitCount;
	NativeInterface nativeInterface{};
	nativeInterface.unitsQuery = &unitsQuery;
	NativeInterfaceWasmAdapter adapter(&nativeInterface);
	WasmRuntimeConfig runtimeConfig;
	runtimeConfig.instructionFuel = 1'000'000'000;
	runtimeConfig.hostWorkLimit = 1'000'000'000;
	WasmRuntime runtime(runtimeConfig);
	WasmModule module(37, {
		.name = "generated-adapter-performance",
		.source = "generated-adapter-performance.wasm",
		.environment = WasmEnvironment::RulesSynced,
		.bytes = std::vector<std::uint8_t>(wasm_component_fixture::kComponentRustFixture,
			wasm_component_fixture::kComponentRustFixture +
				wasm_component_fixture::kComponentRustFixtureSize),
	}, runtime, &adapter);
	std::string error;
	REQUIRE(module.Initialize(error));

	const auto start = Clock::now();
	bool succeeded = true;
	for (std::size_t index = 0; index < iterations; ++index) {
		std::vector<std::uint64_t> results;
		if (!module.Callin("run", {7}, results, error) ||
			results.size() != 1 || results.front() != 17) {
			succeeded = false;
			break;
		}
	}
	const auto elapsed = Clock::now() - start;
	REQUIRE(succeeded);
	const auto perCall = elapsed / iterations;
	INFO("generated NativeInterface Component call: " <<
		std::chrono::duration_cast<Nanoseconds>(perCall).count() <<
		" ns/call (budget " << generatedAdapterBudget.count() << " ns)");
	if constexpr (kBuildUsesAsan) {
		WARN("ASAN build: performance budget is reported but not enforced");
	} else {
		CHECK(perCall <= generatedAdapterBudget);
	}
}

TEST_CASE("Wasm Component Model runs a Rust wit-bindgen guest")
{
	class ComponentAdapter final : public WasmHostAdapter {
	public:
		bool Callout(std::string_view module, std::string_view function,
			const std::vector<WasmValue>& arguments, WasmValue& result,
			std::string& error) override
		{
			if (module != "units_query" || function != "GetTeamUnitCount" ||
				arguments.size() != 1) {
				error = "unexpected Rust Component Model host call";
				return false;
			}
			const auto* teamID = std::get_if<std::int64_t>(&arguments.front().storage);
			if (teamID == nullptr) {
				error = "Rust Component Model host argument has the wrong type";
				return false;
			}
			result = WasmValue::U64(static_cast<std::uint64_t>(*teamID + 10));
			return true;
		}
	};

	ComponentAdapter adapter;
	WasmRuntime runtime;
	WasmModule module(4, {
		.name = "rust-component-host-fixture",
		.source = "rust-component-host-fixture.wasm",
		.environment = WasmEnvironment::RulesSynced,
		.bytes = std::vector<std::uint8_t>(wasm_component_fixture::kComponentRustFixture,
			wasm_component_fixture::kComponentRustFixture +
				wasm_component_fixture::kComponentRustFixtureSize),
	}, runtime, &adapter);
	std::string error;
	const bool initialized = module.Initialize(error);
	INFO("Rust Component Model initialization error: " << error);
	REQUIRE(initialized);
	std::vector<std::uint64_t> results;
	REQUIRE(module.Callin("run", {7}, results, error));
	REQUIRE(results.size() == 1);
	CHECK(results.front() == 17);
}

TEST_CASE("Wasm Component Model lowers string list and record values")
{
	class ComponentAdapter final : public WasmHostAdapter {
	public:
		bool Callout(std::string_view module, std::string_view function,
			const std::vector<WasmValue>& arguments, WasmValue& result,
			std::string& error) override
		{
			if (module != "units_query" || function != "GetQueryResult" ||
				arguments.size() != 2) {
				error = "unexpected complex Component Model host call";
				return false;
			}
			const auto* input = std::get_if<std::string>(&arguments[0].storage);
			const auto* values = std::get_if<std::vector<std::uint8_t>>(
				&arguments[1].storage);
			if (input == nullptr || values == nullptr || *input != "hello" ||
				*values != std::vector<std::uint8_t>{1, 2, 3}) {
				error = "complex Component Model host arguments have the wrong shape";
				return false;
			}
			result = WasmValue::Record({
				{"text", WasmValue::String(*input)},
				{"bytes", WasmValue::Bytes(*values)},
			});
			return true;
		}
	};

	ComponentAdapter adapter;
	WasmRuntimeConfig runtimeConfig;
	runtimeConfig.allowUnregisteredComponentFunctionsForTesting = true;
	WasmRuntime runtime(runtimeConfig);
	WasmModule module(5, {
		.name = "complex-component-host-fixture",
		.source = "complex-component-host-fixture.wasm",
		.environment = WasmEnvironment::RulesSynced,
		.bytes = std::vector<std::uint8_t>(wasm_component_fixture::kComponentComplexFixture,
			wasm_component_fixture::kComponentComplexFixture +
				wasm_component_fixture::kComponentComplexFixtureSize),
	}, runtime, &adapter);
	std::string error;
	const bool initialized = module.Initialize(error);
	INFO("Complex Component Model initialization error: " << error);
	REQUIRE(initialized);
	std::vector<std::uint64_t> results;
	REQUIRE(module.Callin("run", {0}, results, error));
	REQUIRE(results.size() == 1);
	CHECK(results.front() == 8);
}

TEST_CASE("Wasm Component Model lowers options enums flags and results")
{
	class ComponentAdapter final : public WasmHostAdapter {
	public:
		bool Callout(std::string_view module, std::string_view function,
			const std::vector<WasmValue>& arguments, WasmValue& result,
			std::string& error) override
		{
			if (module != "units_query" || function != "GetSemantic" ||
				arguments.size() != 3) {
				error = "unexpected semantic Component Model host call";
				return false;
			}
			const auto* maybe = std::get_if<std::string>(&arguments[0].storage);
			const auto* color = std::get_if<std::string>(&arguments[1].storage);
			const auto* flags = std::get_if<WasmValueList>(&arguments[2].storage);
			if (maybe == nullptr || color == nullptr || flags == nullptr || *maybe != "x" ||
				*color != "blue" || flags->size() != 1 ||
				std::get<std::string>((*flags)[0].storage) != "fast") {
				error = "semantic Component Model host arguments have the wrong shape";
				return false;
			}
			result = WasmValue::Record({
				{"maybe", WasmValue::String(*maybe)},
				{"color", WasmValue::String(*color)},
				{"feature-flags", WasmValue::List({WasmValue::String("fast")})},
				{"outcome", WasmValue::Record({
					{"ok", WasmValue::Bool(true)},
					{"value", WasmValue::U64(41)},
				})},
			});
			return true;
		}
	};

	ComponentAdapter adapter;
	WasmRuntimeConfig runtimeConfig;
	runtimeConfig.allowUnregisteredComponentFunctionsForTesting = true;
	WasmRuntime runtime(runtimeConfig);
	WasmModule module(6, {
		.name = "semantic-component-host-fixture",
		.source = "semantic-component-host-fixture.wasm",
		.environment = WasmEnvironment::RulesSynced,
		.bytes = std::vector<std::uint8_t>(wasm_component_fixture::kComponentSemanticFixture,
			wasm_component_fixture::kComponentSemanticFixture +
				wasm_component_fixture::kComponentSemanticFixtureSize),
	}, runtime, &adapter);
	std::string error;
	const bool initialized = module.Initialize(error);
	INFO("Semantic Component Model initialization error: " << error);
	REQUIRE(initialized);
	std::vector<std::uint64_t> results;
	REQUIRE(module.Callin("run", {0}, results, error));
	REQUIRE(results.size() == 1);
	CHECK(results.front() == 48);
}

TEST_CASE("Wasm Component Model lowers variants and owned resources")
{
	class ComponentAdapter final : public WasmHostAdapter {
	public:
		bool Callout(std::string_view module, std::string_view function,
			const std::vector<WasmValue>& arguments, WasmValue& result,
			std::string& error) override
		{
			if (module != "units_query" || function != "EchoChoice" ||
				arguments.size() != 1) {
				error = "unexpected variant Component Model host call";
				return false;
			}
			const auto* variant = std::get_if<WasmValueVariant>(&arguments.front().storage);
			if (variant == nullptr || !variant->HasValue() ||
				variant->discriminant != "number" ||
				!std::holds_alternative<std::uint64_t>(variant->value->storage)) {
				error = "variant Component Model host argument has the wrong shape";
				return false;
			}
			result = arguments.front();
			return true;
		}
	};

	ComponentAdapter adapter;
	WasmRuntimeConfig runtimeConfig;
	runtimeConfig.allowUnregisteredComponentFunctionsForTesting = true;
	WasmRuntime runtime(runtimeConfig);
	WasmModule module(31, {
		.name = "value-component-host-fixture",
		.source = "value-component-host-fixture.wasm",
		.environment = WasmEnvironment::RulesSynced,
		.bytes = std::vector<std::uint8_t>(wasm_component_fixture::kComponentValueFixture,
			wasm_component_fixture::kComponentValueFixture +
				wasm_component_fixture::kComponentValueFixtureSize),
	}, runtime, &adapter);
	std::string error;
	const bool initialized = module.Initialize(error);
	INFO("Variant/resource Component Model initialization error: " << error);
	REQUIRE(initialized);

	WasmValue variantResult;
	REQUIRE(module.Callin("recoil:spring-api/resource-fixture@1.0.0/echo-choice",
		{WasmValue::Variant("number", WasmValue::U64(123))}, variantResult, error));
	const auto* variantEnvelope = std::get_if<WasmValueRecord>(&variantResult.storage);
	REQUIRE(variantEnvelope != nullptr);
	CHECK(std::get<bool>(variantEnvelope->at("ok").storage));
	const auto* variant = std::get_if<WasmValueVariant>(
		&variantEnvelope->at("value").storage);
	REQUIRE(variant != nullptr);
	CHECK(variant->discriminant == "number");
	REQUIRE(variant->value != nullptr);
	CHECK(std::get<std::uint64_t>(variant->value->storage) == 123);

	WasmValue token;
	REQUIRE(module.Callin("recoil:spring-api/resource-fixture@1.0.0/make-token", {}, token, error));
	const auto* resource = std::get_if<WasmValueResource>(&token.storage);
	REQUIRE(resource != nullptr);
	REQUIRE(resource->handle != 0);
	CHECK(resource->family == "component");
	CHECK(resource->owned);
	CHECK(module.Resources().Validate(resource->handle, module.InstanceID(), "component"));

	WasmValue rejectedPair;
	CHECK_FALSE(module.Callin(
		"recoil:spring-api/resource-fixture@1.0.0/consume-pair",
		{WasmValue::Record({
			{"token", token},
			{"required", WasmValue::String("wrong type")},
		})}, rejectedPair, error));
	CHECK(error.find("unsigned integer") != std::string::npos);
	// Lowering the first owned field cloned a transfer candidate, but the
	// later field failure must not consume the source resource.
	CHECK(module.Resources().Validate(resource->handle, module.InstanceID(), "component"));

	WasmValue consumed;
	REQUIRE(module.Callin("recoil:spring-api/resource-fixture@1.0.0/consume-token", {token}, consumed, error));
	CHECK(std::get<std::uint64_t>(consumed.storage) == 73);
	CHECK_FALSE(module.Resources().Validate(resource->handle, module.InstanceID(), "component"));
	WasmValue staleResult;
	CHECK_FALSE(module.Callin("recoil:spring-api/resource-fixture@1.0.0/consume-token",
		{token}, staleResult, error));
	CHECK(error.find("stale") != std::string::npos);
}
