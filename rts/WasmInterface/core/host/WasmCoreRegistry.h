/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <string_view>

#include "WasmEnvironment.h"
#include "WasmCoreRegistryPolicy.h"

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
inline constexpr std::string_view CobScriptModule = "spring:cob-script";
inline constexpr std::string_view UnitsPiecesModule = "spring:units-pieces";
inline constexpr std::string_view UnitControlModule = "spring:unit-control";
inline constexpr std::string_view TerrainControlModule = "spring:terrain-control";
inline constexpr std::string_view SystemControlModule = "spring:system-control";
inline constexpr std::string_view MathExtraModule = "spring:math-extra";
inline constexpr std::string_view GfxModule = "spring:gfx";
inline constexpr std::string_view RmlUiModule = "spring:rml-ui";
inline constexpr std::string_view ProfilingModule = "spring:profiling";
inline constexpr std::string_view MessagesModule = "spring:messages";
inline constexpr std::string_view RulesParamsModule = "spring:rules-params";
inline constexpr std::string_view TerrainModule = "spring:terrain";
inline constexpr std::string_view ConfigModule = "spring:config";
inline constexpr std::string_view BenchmarkModule = "spring:benchmark";
inline constexpr std::string_view CusModule = "spring:cus";
// Nondeterministic by design and available to synced guests; see
// WasmCoreDesyncBindings.cpp.
inline constexpr std::string_view DesyncModule = "spring:desync";

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
	{UnitsInfoModule, "get-unit-nano-pieces", "i32,i32,i32->i64", AllEnvironmentMask},
	{UnitsInfoModule, "get-unit-is-transporting", "i32,i32,i32,i32->i64", AllEnvironmentMask},

	{UnitsQueryModule, "valid-unit-id", "i32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-all-units", "i32,i32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-team-units", "i32,i32,i32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-team-units-by-defs", "i32,i32,i32,i32,i32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-team-unit-def-count", "i32,i32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-team-unit-count", "i32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-unit-array-centroid", "i32,i32,i32->i32", AllEnvironmentMask},
	{UnitsQueryModule, "get-unit-map-centroid", "i32,i32,i32->i32", AllEnvironmentMask},
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
	{CobScriptModule, "call-cob-script", "i32,i32,i32,i32,i32,i32,i32,i32,i32,i32->i64", SyncedEnvironmentMask},
	{CusModule, "attach", "i32,i32,i32,i32->i64", SyncedEnvironmentMask},
	{CusModule, "operation", "i32,i32,i32,i32,i32,i32,i32,f32,f32,f32->i64", SyncedEnvironmentMask},
	{CusModule, "animation-active", "i32,i32,i32,i32,i32->i64", SyncedEnvironmentMask},

	// Reviewed flat list<string> result. The descriptor table and packed bytes
	// are guest-owned; Core never materializes vector<string> or one allocation
	// per name.
	{UnitsPiecesModule, "get-unit-script-names-flat", "i32,i32,i32,i32,i32,i32->i32", AllEnvironmentMask},

	{UnitControlModule, "give-order-to-unit", "i32,i32,i32,i32,i32,i32->i64", SyncedEnvironmentMask},

	{TerrainControlModule, "set-height-map", "f32,f32,f32,f32->i64", SyncedEnvironmentMask},
	{TerrainControlModule, "level-height-map", "f32,f32,f32,f32,f32->i64", SyncedEnvironmentMask},
	{TerrainControlModule, "set-height-map-func", "i32,i32->i64", SyncedEnvironmentMask},
	{TerrainControlModule, "set-original-height-map-func", "i32,i32->i64", SyncedEnvironmentMask},
	{TerrainControlModule, "set-smooth-mesh-func", "i32,i32->i64", SyncedEnvironmentMask},
	{SystemControlModule, "call-as-team", "i32,i32,i32->i64", SyncedEnvironmentMask},
	{MathExtraModule, "normalize", "f32,f32,f32,i32->i32", AllEnvironmentMask},

	{GfxModule, "vertex", "f32,f32,f32,f32,i32->i32", UnsyncedEnvironmentMask},
	{GfxModule, "begin-end", "i32,i32,i32->i32", UnsyncedEnvironmentMask},
	{GfxModule, "active-fbo", "i32,i32,i32,i32,i32->i32", UnsyncedEnvironmentMask},
	{GfxModule, "active-shader", "i32,i32,i32->i32", UnsyncedEnvironmentMask},
	{GfxModule, "create-list", "i32,i32->i64", UnsyncedEnvironmentMask},
	{GfxModule, "draw-func-at-unit", "i32,i32,i32,i32->i32", UnsyncedEnvironmentMask},
	{GfxModule, "push-pop-matrix", "i32,i32->i32", UnsyncedEnvironmentMask},
	{GfxModule, "render-to-texture", "i32,i32,i32,i32->i32", UnsyncedEnvironmentMask},
	{GfxModule, "run-query", "i32,i32,i32->i32", UnsyncedEnvironmentMask},
	{GfxModule, "unsafe-state", "i32,i32,i32,i32->i32", UnsyncedEnvironmentMask},
	{GfxModule, "create-texture", "i32,i32,i32,i32,i32,i32->i64", UnsyncedEnvironmentMask},
	{GfxModule, "create-texture-atlas", "i32,i32,i32,i32,i32->i64", UnsyncedEnvironmentMask},

	// RmlUi event transports retain a guest callback across the native call and
	// are registered by hand in WasmCoreRmlUiBindings.cpp. The generic lowering
	// cannot describe the retained callback triple, so the generated registry
	// omits them and these are the only signatures a guest can import.
	{RmlUiModule, "context-add-event-listener", "i64,i32,i32,i32,i32,i32,i32,i32->i32", UnsyncedEnvironmentMask},
	{RmlUiModule, "element-add-event-listener", "i64,i32,i32,i32,i32,i32,i32,i32->i32", UnsyncedEnvironmentMask},
	{RmlUiModule, "data-model-bind-event", "i64,i32,i32,i32,i32,i32,i32,i64,i32->i32", UnsyncedEnvironmentMask},
	{RmlUiModule, "data-model-unbind-event", "i64->i64", UnsyncedEnvironmentMask},
	{RmlUiModule, "data-model-current-event", "i32->i32", UnsyncedEnvironmentMask},
	{RmlUiModule, "data-model-current-value", "i64,i32,i32,i32->i32", UnsyncedEnvironmentMask},
	{RmlUiModule, "event-listener-on-attach", "i64,i64->i64", UnsyncedEnvironmentMask},
	{RmlUiModule, "event-listener-on-detach", "i64,i64->i64", UnsyncedEnvironmentMask},
	{RmlUiModule, "event-listener-process-event", "i64,i64->i64", UnsyncedEnvironmentMask},

	// Profiling reads are intentionally unsynced-only. GetTimerMicros was
	// historically marked all-environment despite using spring_now(); allowing a
	// synced guest to branch on that value would break deterministic simulation.
	{ProfilingModule, "get-timer", "->i64", UnsyncedEnvironmentMask},
	{ProfilingModule, "get-timer-micros", "->i64", UnsyncedEnvironmentMask},
	{ProfilingModule, "diff-timers", "i64,i64,i32,i32->i64", UnsyncedEnvironmentMask},
	{ProfilingModule, "get-frame-timer", "i32->i64", UnsyncedEnvironmentMask},
	{ProfilingModule, "get-draw-seconds", "->i64", UnsyncedEnvironmentMask},
	{ProfilingModule, "get-lua-mem-usage", "i32,i32->i32", UnsyncedEnvironmentMask},
	{ProfilingModule, "get-vid-mem-usage", "i32,i32->i32", UnsyncedEnvironmentMask},
	{ProfilingModule, "get-synced-gc-info", "i32->i64", UnsyncedEnvironmentMask},

	// Message strings are copied into call-scoped NUL-terminated storage before
	// entering NativeInterface. The legacy table retains its source masks here;
	// WasmCoreRegistryPolicy normalizes ordinary messages to unsynced/UI and
	// withholds SendCommands before validation returns the descriptor.
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

	{TerrainModule, "is-pos-in-map", "f32,f32->i64", AllEnvironmentMask},
	{TerrainModule, "get-ground-height", "f32,f32->i64", AllEnvironmentMask},
	{TerrainModule, "get-ground-orig-height", "f32,f32->i64", AllEnvironmentMask},
	{TerrainModule, "get-smooth-mesh-height", "f32,f32->i64", AllEnvironmentMask},
	{TerrainModule, "get-water-plane-level", "->i64", AllEnvironmentMask},
	{TerrainModule, "get-water-level", "f32,f32->i64", AllEnvironmentMask},
	{TerrainModule, "get-ground-normal", "f32,f32,i32,i32->i32", AllEnvironmentMask},
	{TerrainModule, "get-ground-extremes", "i32->i32", AllEnvironmentMask},
	{TerrainModule, "get-height-map-size", "i32->i32", AllEnvironmentMask},
	{TerrainModule, "get-ground-blocked", "f32,f32,f32,f32->i64", AllEnvironmentMask},
	{TerrainModule, "get-grass", "f32,f32->i64", AllEnvironmentMask},

	{ConfigModule, "get-log-sections-flat", "i32,i32,i32,i32,i32->i32", UnsyncedEnvironmentMask},

	// The desync group trades determinism for a clock in synced code. The same
	// timers stay unsynced-only under spring:profiling.
	{DesyncModule, "get-timer", "->i64", AllEnvironmentMask},
	{DesyncModule, "get-timer-micros", "->i64", AllEnvironmentMask},
	{DesyncModule, "diff-timers", "i64,i64,i32,i32->i64", AllEnvironmentMask},
	{BenchmarkModule, "consume-string", "i32,i32->i64", AllEnvironmentMask},
	{BenchmarkModule, "consume-f32-list", "i32,i32->i64", AllEnvironmentMask},
};

inline ImportLookup LookupImport(std::string_view module, std::string_view name)
{
	for (const ImportDescriptor& import : kImports) {
		if (import.module != module || import.name != name)
			continue;
		if (!registry_policy::HandwrittenImportAllowed(module, name))
			return {};
		return {
			import.signature,
			registry_policy::HandwrittenEnvironmentMask(module, name, import.environmentMask),
			true,
		};
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
