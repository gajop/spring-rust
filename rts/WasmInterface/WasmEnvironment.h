/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <array>
#include <cstdint>
#include <string_view>

enum class WasmEnvironment : std::uint8_t {
	RulesSynced,
	RulesUnsynced,
	GaiaSynced,
	GaiaUnsynced,
	UI,
};

struct WasmEnvironmentPolicy {
	WasmEnvironment environment;
	const char* name;
	bool synced;
	bool runtimeEnabled;
	bool permitsSimulationMutation;
};

class WasmEnvironmentMatrix {
public:
	static constexpr std::array<WasmEnvironment, 5> All() {
		return {WasmEnvironment::RulesSynced, WasmEnvironment::RulesUnsynced,
			WasmEnvironment::GaiaSynced, WasmEnvironment::GaiaUnsynced,
			WasmEnvironment::UI};
	}

	static const WasmEnvironmentPolicy& Policy(WasmEnvironment environment);
	static const char* Name(WasmEnvironment environment);
	static bool Parse(std::string_view name, WasmEnvironment& environment);

	// UI remains described by the generated model but is intentionally disabled
	// until its Lua visibility/LOS semantics pass parity.
	static bool IsRuntimeEnabled(WasmEnvironment environment);

	// Return whether a generated C/WIT API module belongs in this world.
	static bool HasModule(WasmEnvironment environment, std::string_view module);
};
