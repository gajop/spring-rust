/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <string_view>

#include "WasmEnvironment.h"

#if __has_include("../wasm/generated/WasmCoreGeneratedRegistry.h")
#include "../wasm/generated/WasmCoreGeneratedRegistry.h"
#define RECOIL_WASM_CORE_GENERATED_REGISTRY 1
#endif

namespace recoil::wasm::core {

struct ImportDescriptor {
	std::string_view module;
	std::string_view name;
	std::string_view signature;
	std::uint32_t environmentMask;
};

struct ImportLookup {
	std::string_view signature;
	std::uint32_t environmentMask = 0;
	bool found = false;
};

inline constexpr std::string_view UnitsInfoModule = "spring:units-info";
inline constexpr std::string_view UnitsQueryModule = "spring:units-query";
inline constexpr std::string_view UnitDefsModule = "spring:unit-defs";
inline constexpr std::string_view UnitsCommandsModule = "spring:units-commands";
inline constexpr std::string_view UnitsPiecesModule = "spring:units-pieces";
inline constexpr std::string_view UnitControlModule = "spring:unit-control";
inline constexpr std::string_view TerrainControlModule = "spring:terrain-control";
inline constexpr std::string_view GfxModule = "spring:gfx";
inline constexpr std::string_view ProfilingModule = "spring:profiling";
inline constexpr std::string_view MessagesModule = "spring:messages";
inline constexpr std::string_view RulesParamsModule = "spring:rules-params";
inline constexpr std::string_view TerrainModule = "spring:terrain";
inline constexpr std::string_view ConfigModule = "spring:config";
inline constexpr std::string_view BenchmarkModule = "spring:benchmark";

inline constexpr std::string_view GameFrameExport = "spring:callin/game-frame";
inline constexpr std::string_view GameFramePostExport = "spring:callin/game-frame-post";
inline constexpr std::string_view UpdateExport = "spring:callin/update";
inline constexpr std::string_view UnitCreatedExport = "spring:callin/unit-created";
inline constexpr std::string_view UnitPreDamagedExport = "spring:callin/unit-pre-damaged";
inline constexpr std::string_view AllowUnitCreationExport = "spring:callin/allow-unit-creation";
inline constexpr std::string_view DrawWorldExport = "spring:callin/draw-world";
inline constexpr std::string_view CallbackDispatchExport = "spring:callback/dispatch";

inline constexpr std::uint32_t AllEnvironmentMask =
	(1u << static_cast<std::uint32_t>(WasmEnvironment::RulesSynced)) |
	(1u << static_cast<std::uint32_t>(WasmEnvironment::RulesUnsynced)) |
	(1u << static_cast<std::uint32_t>(WasmEnvironment::GaiaSynced)) |
	(1u << static_cast<std::uint32_t>(WasmEnvironment::GaiaUnsynced)) |
	(1u << static_cast<std::uint32_t>(WasmEnvironment::UI));
inline constexpr std::uint32_t SyncedEnvironmentMask =
	(1u << static_cast<std::uint32_t>(WasmEnvironment::RulesSynced)) |
	(1u << static_cast<std::uint32_t>(WasmEnvironment::GaiaSynced));
inline constexpr std::uint32_t UnsyncedEnvironmentMask =
	(1u << static_cast<std::uint32_t>(WasmEnvironment::RulesUnsynced)) |
	(1u << static_cast<std::uint32_t>(WasmEnvironment::GaiaUnsynced)) |
	(1u << static_cast<std::uint32_t>(WasmEnvironment::UI));

inline constexpr ImportDescriptor kImports[] = {
	{UnitsInfoModule, "get-unit-def-id", "i32->i64", AllEnvironmentMask},
	{UnitsInfoModule, "get-unit-team", "i32->i64", AllEnvironmentMask},
	{UnitsInfoModule, "get-unit-is-dead", "i32->i64", AllEnvironmentMask},
	{UnitsInfoModule, "get-unit-experience", "i32->i64", AllEnvironmentMask},
	{UnitsInfoModule, "get-unit-position", "i32,i32,i32->i32", AllEnvironmentMask},
	{UnitsInfoModule, "get-unit-velocity", "i32,i32->i32", AllEnvironmentMask},
	{UnitsInfoModule, "get-unit-health", "i32,i32->i32", AllEnvironmentMask},

	{UnitsQueryModule, "valid-unit-id", "i32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-all-units", "i32,i32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-team-units", "i32,i32,i32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-team-unit-def-count", "i32,i32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-team-unit-count", "i32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-units-in-rectangle", "f32,f32,f32,f32,i32,i32,i32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-units-in-box", "f32,f32,f32,f32,f32,f32,i32,i32,i32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-units-in-sphere", "f32,f32,f32,f32,i32,i32,i32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-units-in-cylinder", "f32,f32,f32,i32,i32,i32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-unit-nearest-ally", "i32,f32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-unit-nearest-enemy", "i32,f32,i32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-unit-separation", "i32,i32,i32->i64", AllEnvironmentMask},

	{UnitDefsModule, "get-unit-def-name", "i32,i32,i32->i64", AllEnvironmentMask},
	{UnitDefsModule, "get-unit-def-human-name", "i32,i32,i32->i64", AllEnvironmentMask},

	{UnitsCommandsModule, "get-unit-command-count", "i32->i64", AllEnvironmentMask},
	{UnitsCommandsModule, "get-unit-commands", "i32,i32,i32,i32->i64", AllEnvironmentMask},
	// Synced order callouts borrow aligned guest i32/f32 arrays directly for the
	// duration of the native call. Synced Core memory is fixed/non-growable, so
	// these imports need no host vector or copy.
	{UnitsCommandsModule, "give-order", "i32,i32,i32,i32,i32->i64", SyncedEnvironmentMask},
	{UnitsCommandsModule, "give-order-to-unit-map", "i32,i32,i32,i32,i32,i32,i32->i64", SyncedEnvironmentMask},

	// Reviewed flat list<string> result. The descriptor table and packed bytes
	// are guest-owned; Core never materializes vector<string> or one allocation
	// per name.
	{UnitsPiecesModule, "get-unit-script-names-flat", "i32,i32,i32,i32,i32,i32->i32", AllEnvironmentMask},

	{UnitControlModule, "give-order-to-unit", "i32,i32,i32,i32,i32,i32->i64", SyncedEnvironmentMask},

	{TerrainControlModule, "set-height-map", "f32,f32,f32,f32->i64", SyncedEnvironmentMask},
	{TerrainControlModule, "level-height-map", "f32,f32,f32,f32,f32->i64", SyncedEnvironmentMask},
	{TerrainControlModule, "set-height-map-func", "i32,i32->i64", SyncedEnvironmentMask},
	{GfxModule, "vertex", "f32,f32,f32,f32,i32->i32", UnsyncedEnvironmentMask},
	{GfxModule, "begin-end", "i32,i32,i32->i32", UnsyncedEnvironmentMask},

	{ProfilingModule, "get-timer-micros", "->i64", AllEnvironmentMask},
	{ProfilingModule, "get-lua-mem-usage", "i32->i32", UnsyncedEnvironmentMask},
	{ProfilingModule, "get-synced-gc-info", "i32->i64", UnsyncedEnvironmentMask},

	// Message strings are copied into call-scoped NUL-terminated storage before
	// entering NativeInterface; short values stay on the stack. SendToUnsynced
	// retains its synced-only semantic policy.
	{MessagesModule, "echo", "i32,i32,i32,i32->i64", AllEnvironmentMask},
	{MessagesModule, "log", "i32,i32,i32,i32,i32->i64", AllEnvironmentMask},
	{MessagesModule, "send-message", "i32,i32->i64", AllEnvironmentMask},
	{MessagesModule, "send-message-to-player", "i32,i32,i32->i64", AllEnvironmentMask},
	{MessagesModule, "send-message-to-team", "i32,i32,i32->i64", AllEnvironmentMask},
	{MessagesModule, "send-message-to-ally-team", "i32,i32,i32->i64", AllEnvironmentMask},
	{MessagesModule, "send-message-to-spectators", "i32,i32->i64", AllEnvironmentMask},
	{MessagesModule, "send-public-chat", "i32,i32->i64", AllEnvironmentMask},
	{MessagesModule, "send-ally-chat", "i32,i32->i64", AllEnvironmentMask},
	{MessagesModule, "send-spectator-chat", "i32,i32->i64", AllEnvironmentMask},
	{MessagesModule, "send-private-chat", "i32,i32,i32->i64", AllEnvironmentMask},
	{MessagesModule, "send-commands", "i32,i32,i32,i32->i64", AllEnvironmentMask},
	{MessagesModule, "send-lua-menu-msg", "i32,i32->i64", AllEnvironmentMask},
	{MessagesModule, "send-skirmish-ai-message", "i32,i32,i32->i64", AllEnvironmentMask},
	{MessagesModule, "send-lua-ui-msg", "i32,i32,i32,i32->i64", AllEnvironmentMask},
	{MessagesModule, "send-lua-gaia-msg", "i32,i32->i64", AllEnvironmentMask},
	{MessagesModule, "send-lua-rules-msg", "i32,i32->i64", AllEnvironmentMask},
	{MessagesModule, "send-to-unsynced", "i32,i32->i64", SyncedEnvironmentMask},

	{RulesParamsModule, "set-unit-rules-param-f32", "i32,i32,i32,f32,i32->i64", SyncedEnvironmentMask},
	{RulesParamsModule, "get-unit-rules-param-f32", "i32,i32,i32->i64", AllEnvironmentMask},
	{TerrainModule, "get-ground-orig-height", "f32,f32->i64", AllEnvironmentMask},

	{ConfigModule, "get-log-sections-flat", "i32,i32,i32,i32,i32->i32", UnsyncedEnvironmentMask},

	{BenchmarkModule, "consume-string", "i32,i32->i64", AllEnvironmentMask},
	{BenchmarkModule, "consume-f32-list", "i32,i32->i64", AllEnvironmentMask},
};

inline ImportLookup LookupImport(std::string_view module, std::string_view name)
{
	for (const ImportDescriptor& import : kImports) {
		if (import.module == module && import.name == name)
			return {import.signature, import.environmentMask, true};
	}
#if defined(RECOIL_WASM_CORE_GENERATED_REGISTRY)
	if (const auto* import = generated_registry::Find(module, name); import != nullptr)
		return {import->signature, import->environmentMask, true};
#endif
	return {};
}

inline bool ImportAllowed(std::string_view module, std::string_view name,
	WasmEnvironment environment)
{
	const ImportLookup import = LookupImport(module, name);
	if (!import.found)
		return false;
	const std::uint32_t bit = 1u << static_cast<std::uint32_t>(environment);
	return (import.environmentMask & bit) != 0;
}

} // namespace recoil::wasm::core
