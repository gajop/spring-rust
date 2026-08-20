/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <string>

#include "NativeInterface/NativeInterface.h"
#include "WasmCoreAbi.h"
#include "WasmEnvironment.h"
#include "WasmResources.h"

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)

struct HostState {
	NativeInterface* native = nullptr;
	Memory memory;
	WasmExecutionBudget* budget = nullptr;
	WasmEnvironment environment = WasmEnvironment::RulesSynced;
	std::uint32_t maxValueNodes = 1u << 20;
	bool fixedMemory = false;
	wasmtime_func_t callbackDispatch{};
	bool callbackDispatchBound = false;
};

#if __has_include("../wasm/generated/WasmCoreGeneratedBindings.h")
#define RECOIL_WASM_CORE_GENERATED_BINDINGS 1
namespace generated {
bool RegisterGeneratedImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
}
#endif

#if __has_include("../wasm/generated/WasmCoreGeneratedOptionBindings.h")
#define RECOIL_WASM_CORE_GENERATED_OPTION_BINDINGS 1
namespace generated {
bool RegisterGeneratedOptionImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
}
#endif

#if __has_include("../wasm/generated/WasmCoreGeneratedVariableOutputBindings.h")
#define RECOIL_WASM_CORE_GENERATED_VARIABLE_OUTPUT_BINDINGS 1
namespace generated {
bool RegisterGeneratedVariableOutputImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
}
#endif

bool RegisterFastImports(wasmtime_linker_t* linker, HostState* state, std::string& error);
bool RegisterUnitsQueryImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterUnitDefsImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterUnitsCommandsImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterUnitsPiecesImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterUnitControlImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterTerrainControlImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterGfxImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterProfilingImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterMessagesImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterConfigImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterBenchmarkImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool BindGuestMemory(HostState& state, wasmtime_context_t* context,
	const wasmtime_instance_t& instance, std::string& error);

class InstanceBindings {
public:
	explicit InstanceBindings(NativeInterface* nativeInterface,
		WasmExecutionBudget* executionBudget = nullptr, bool fixedMemory = false,
		WasmEnvironment environment = WasmEnvironment::RulesSynced,
		std::uint32_t maxValueNodes = 1u << 20)
	{
		host.native = nativeInterface;
		host.budget = executionBudget;
		host.fixedMemory = fixedMemory;
		host.environment = environment;
		host.maxValueNodes = maxValueNodes;
		if (fixedMemory)
			host.memory.MarkStable();
	}

	bool RegisterImports(wasmtime_linker_t* linker, std::string& error)
	{
		if (linker == nullptr) {
			error = "cannot register Core imports without a linker";
			return false;
		}

		wasmtime_linker_allow_shadowing(linker, true);
#if defined(RECOIL_WASM_CORE_GENERATED_BINDINGS)
		if (!generated::RegisterGeneratedImports(linker, &host, error))
			return false;
#endif
#if defined(RECOIL_WASM_CORE_GENERATED_OPTION_BINDINGS)
		if (!generated::RegisterGeneratedOptionImports(linker, &host, error))
			return false;
#endif
#if defined(RECOIL_WASM_CORE_GENERATED_VARIABLE_OUTPUT_BINDINGS)
		if (!generated::RegisterGeneratedVariableOutputImports(linker, &host, error))
			return false;
#endif
		if (host.environment != WasmEnvironment::UI &&
			!RegisterFastImports(linker, &host, error))
			return false;
		if (!RegisterUnitsQueryImports(linker, &host, error))
			return false;
		if (!RegisterUnitDefsImports(linker, &host, error))
			return false;
		if (!RegisterUnitsCommandsImports(linker, &host, error))
			return false;
		if (!RegisterUnitsPiecesImports(linker, &host, error))
			return false;
		if (!RegisterUnitControlImports(linker, &host, error))
			return false;
		if (!RegisterTerrainControlImports(linker, &host, error))
			return false;
		if (!RegisterGfxImports(linker, &host, error))
			return false;
		if (!RegisterProfilingImports(linker, &host, error))
			return false;
		if (!RegisterMessagesImports(linker, &host, error))
			return false;
		if (!RegisterConfigImports(linker, &host, error))
			return false;
		return RegisterBenchmarkImports(linker, &host, error);
	}

	bool Bind(wasmtime_context_t* context, const wasmtime_instance_t& instance,
		std::string& error);

	bool GameFrame(wasmtime_context_t* context, std::int32_t frame, std::string& error) const;
	bool GameFramePost(wasmtime_context_t* context, std::int32_t frame, std::string& error) const;
	bool Update(wasmtime_context_t* context, float deltaSeconds, std::string& error) const;
	bool UnitCreated(wasmtime_context_t* context, std::int32_t unitID,
		std::int32_t unitDefID, std::int32_t unitTeam, std::int32_t builderID,
		std::string& error) const;
	bool UnitPreDamaged(wasmtime_context_t* context, std::int32_t unitID,
		std::int32_t unitDefID, std::int32_t unitTeam, float damage, bool paralyzer,
		std::int32_t weaponDefID, std::int32_t projectileID, std::int32_t attackerID,
		std::int32_t attackerDefID, std::int32_t attackerTeam,
		float& newDamage, float& impulseMult, std::string& error) const;
	bool AllowUnitCreation(wasmtime_context_t* context, std::int32_t unitDefID,
		std::int32_t builderID, std::int32_t builderTeam, bool hasBuildInfo,
		float buildX, float buildY, float buildZ, std::int32_t buildFacing,
		bool& allow, bool& dropOrder, std::string& error) const;
	bool DrawWorld(wasmtime_context_t* context, std::string& error) const;

	bool HasGameFrame() const { return gameFrame.Present(); }
	bool HasGameFramePost() const { return gameFramePost.Present(); }
	bool HasUpdate() const { return update.Present(); }
	bool HasUnitCreated() const { return unitCreated.Present(); }
	bool HasUnitPreDamaged() const { return unitPreDamaged.Present(); }
	bool HasAllowUnitCreation() const { return allowUnitCreation.Present(); }
	bool HasDrawWorld() const { return drawWorld.Present(); }

	HostState& Host() { return host; }

private:
	HostState host;
	I32ToVoidExport gameFrame;
	I32ToVoidExport gameFramePost;
	RawExport update;
	RawExport unitCreated;
	RawExport unitPreDamaged;
	RawExport allowUnitCreation;
	RawExport drawWorld;
};

#endif

} // namespace recoil::wasm::core
