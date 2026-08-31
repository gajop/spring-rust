/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <memory>
#include <string>

#include "NativeInterface/NativeInterface.h"
#include "WasmCoreAbi.h"
#include "WasmEnvironment.h"
#include "WasmResources.h"

class NativeUnitScriptBackend;

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)

struct HostState {
	NativeInterface* native = nullptr;
	NativeUnitScriptBackend* cusBackend = nullptr;
	Memory memory;
	WasmExecutionBudget* budget = nullptr;
	WasmEnvironment environment = WasmEnvironment::RulesSynced;
	std::uint32_t maxValueNodes = 1u << 20;
	bool fixedMemory = false;
	wasmtime_context_t* context = nullptr;
	wasmtime_func_t callbackDispatch{};
	bool callbackDispatchBound = false;
	std::shared_ptr<bool> alive = std::make_shared<bool>(true);

	~HostState() {
		if (alive) {
			*alive = false;
		}
	}
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

#if __has_include("../wasm/generated/WasmCoreGeneratedDynamicOutputBindings.h")
#define RECOIL_WASM_CORE_GENERATED_DYNAMIC_OUTPUT_BINDINGS 1
namespace generated {
bool RegisterGeneratedDynamicOutputImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
}
#endif

#if __has_include("../wasm/generated/WasmCoreGeneratedDynamicInputBindings.h")
#define RECOIL_WASM_CORE_GENERATED_DYNAMIC_INPUT_BINDINGS 1
namespace generated {
bool RegisterGeneratedDynamicInputImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
}
#endif

#if __has_include("../wasm/generated/WasmCoreGeneratedVariableBindings.h")
#define RECOIL_WASM_CORE_GENERATED_VARIABLE_BINDINGS 1
namespace generated {
bool RegisterGeneratedVariableImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
}
#endif

#if __has_include("../wasm/generated/WasmCoreGeneratedBorrowedBindings.h")
#define RECOIL_WASM_CORE_GENERATED_BORROWED_BINDINGS 1
namespace generated {
bool RegisterGeneratedBorrowedImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
}
#endif

#if __has_include("../wasm/generated/WasmCoreGeneratedVariableIoBindings.h")
#define RECOIL_WASM_CORE_GENERATED_VARIABLE_IO_BINDINGS 1
namespace generated {
bool RegisterGeneratedVariableIoImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
}
#endif

bool RegisterFastImports(wasmtime_linker_t* linker, HostState* state, std::string& error);
bool RegisterUnitsInfoVariableImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterUnitsQueryImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterUnitsQueryBorrowedImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterUnitDefsImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterUnitsCommandsImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterCobScriptImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterUnitsPiecesImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterUnitControlImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterTerrainControlImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterSystemControlImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterMathExtraImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterTerrainReadImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterGfxImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterGfxResourceImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterVfsImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterRmlUiImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterProfilingImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterMessagesImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterRulesParamsImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterConfigImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterBenchmarkImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterDesyncImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool BindGuestMemory(HostState& state, wasmtime_context_t* context,
	const wasmtime_instance_t& instance, std::string& error);

class InstanceBindings {
public:
	explicit InstanceBindings(NativeInterface* nativeInterface,
		WasmExecutionBudget* executionBudget = nullptr, bool fixedMemory = false,
		WasmEnvironment environment = WasmEnvironment::RulesSynced,
		std::uint32_t maxValueNodes = 1u << 20,
		NativeUnitScriptBackend* cusBackend = nullptr)
	{
		host.native = nativeInterface;
		host.cusBackend = cusBackend;
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
		// Register the legacy fast fallbacks first. Generated bindings use the
		// canonical wire ABI and must shadow any legacy registration for the
		// same import; otherwise a pointer-based generated call can be decoded
		// by an older direct-flag handler.
		if (host.environment != WasmEnvironment::UI &&
			!RegisterFastImports(linker, &host, error))
			return false;
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
#if defined(RECOIL_WASM_CORE_GENERATED_DYNAMIC_OUTPUT_BINDINGS)
		if (!generated::RegisterGeneratedDynamicOutputImports(linker, &host, error))
			return false;
#endif
#if defined(RECOIL_WASM_CORE_GENERATED_DYNAMIC_INPUT_BINDINGS)
		if (!generated::RegisterGeneratedDynamicInputImports(linker, &host, error))
			return false;
#endif
#if defined(RECOIL_WASM_CORE_GENERATED_VARIABLE_BINDINGS)
		if (!generated::RegisterGeneratedVariableImports(linker, &host, error))
			return false;
#endif
#if defined(RECOIL_WASM_CORE_GENERATED_BORROWED_BINDINGS)
		if (!generated::RegisterGeneratedBorrowedImports(linker, &host, error))
			return false;
#endif
#if defined(RECOIL_WASM_CORE_GENERATED_VARIABLE_IO_BINDINGS)
		if (!generated::RegisterGeneratedVariableIoImports(linker, &host, error))
			return false;
#endif
		if (!RegisterUnitsInfoVariableImports(linker, &host, error))
			return false;
		if (!RegisterUnitsQueryImports(linker, &host, error))
			return false;
		if (!RegisterUnitsQueryBorrowedImports(linker, &host, error))
			return false;
		if (!RegisterUnitDefsImports(linker, &host, error))
			return false;
		if (!RegisterUnitsCommandsImports(linker, &host, error))
			return false;
		if (!RegisterCobScriptImports(linker, &host, error))
			return false;
		if (!RegisterUnitsPiecesImports(linker, &host, error))
			return false;
		if (!RegisterUnitControlImports(linker, &host, error))
			return false;
		if (!RegisterTerrainControlImports(linker, &host, error))
			return false;
		if (!RegisterSystemControlImports(linker, &host, error))
			return false;
		if (!RegisterMathExtraImports(linker, &host, error))
			return false;
		if (!RegisterTerrainReadImports(linker, &host, error))
			return false;
		if (!RegisterGfxImports(linker, &host, error))
			return false;
		if (!RegisterGfxResourceImports(linker, &host, error))
			return false;
		if (!RegisterVfsImports(linker, &host, error))
			return false;
		if (!RegisterRmlUiImports(linker, &host, error))
			return false;
		if (!RegisterProfilingImports(linker, &host, error))
			return false;
		if (!RegisterMessagesImports(linker, &host, error))
			return false;
		if (!RegisterRulesParamsImports(linker, &host, error))
			return false;
		if (!RegisterConfigImports(linker, &host, error))
			return false;
		if (!RegisterBenchmarkImports(linker, &host, error))
			return false;
		return RegisterDesyncImports(linker, &host, error);
	}

	bool Bind(wasmtime_context_t* context, const wasmtime_instance_t& instance,
		std::string& error);

	// Argument marshalling lives with the dispatch plan in WasmCoreHost, which
	// calls the resolved function directly. This class only resolves and holds
	// the exports.
	const RawExport& GameFrameExport() const { return gameFrame.Raw(); }
	const RawExport& GameFramePostExport() const { return gameFramePost.Raw(); }
	const RawExport& UpdateExport() const { return update; }
	const RawExport& UnitCreatedExport() const { return unitCreated; }
	const RawExport& UnitPreDamagedExport() const { return unitPreDamaged; }
	const RawExport& AllowUnitCreationExport() const { return allowUnitCreation; }
	const RawExport& DrawWorldExport() const { return drawWorld; }

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
