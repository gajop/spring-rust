/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmEnvironment.h"

#include <algorithm>

#include "wasm/generated/WasmCalloutRegistry.h"

namespace {
	constexpr WasmEnvironmentPolicy ENVIRONMENT_POLICIES[] = {
		{WasmEnvironment::RulesSynced, "rules-synced", true, true, true},
		{WasmEnvironment::RulesUnsynced, "rules-unsynced", false, true, false},
		{WasmEnvironment::GaiaSynced, "gaia-synced", true, true, true},
		{WasmEnvironment::GaiaUnsynced, "gaia-unsynced", false, true, false},
		{WasmEnvironment::UI, "ui", false, true, false},
		{WasmEnvironment::Menu, "menu", false, true, false},
		{WasmEnvironment::Intro, "intro", false, true, false},
	};
	constexpr WasmEnvironmentPolicy INVALID_POLICY = {
		WasmEnvironment::UI, "invalid", false, false, false,
	};

}

const WasmEnvironmentPolicy& WasmEnvironmentMatrix::Policy(WasmEnvironment environment)
{
	const auto iter = std::find_if(std::begin(ENVIRONMENT_POLICIES), std::end(ENVIRONMENT_POLICIES),
		[environment](const auto& policy) { return policy.environment == environment; });
	return iter == std::end(ENVIRONMENT_POLICIES) ? INVALID_POLICY : *iter;
}

const char* WasmEnvironmentMatrix::Name(WasmEnvironment environment)
{
	return Policy(environment).name;
}

bool WasmEnvironmentMatrix::Parse(std::string_view name, WasmEnvironment& environment)
{
	for (const auto& policy : ENVIRONMENT_POLICIES) {
		if (name == policy.name) {
			environment = policy.environment;
			return true;
		}
	}
	return false;
}

bool WasmEnvironmentMatrix::IsRuntimeEnabled(WasmEnvironment environment)
{
	return Policy(environment).runtimeEnabled;
}

bool WasmEnvironmentMatrix::HasModule(WasmEnvironment environment, std::string_view module)
{
	if (!IsRuntimeEnabled(environment))
		return false;
	module = recoil::wasm::generated::CanonicalModule(module);
	const std::uint32_t environmentBit = 1u << static_cast<std::uint32_t>(environment);
	for (const auto& descriptor : recoil::wasm::generated::kCallouts) {
		if (module == descriptor.module && (descriptor.environmentMask & environmentBit) != 0)
			return true;
	}
	return false;
}
