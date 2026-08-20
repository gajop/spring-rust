/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <string>

#include "WasmCoreBindings.h"

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)

// Runtime bindings needed by the shared Lua/native/Component/Core benchmark
// suite.  The timer/message calls are benchmark instrumentation; RulesParams
// and Terrain calls are ordinary NativeInterface operations and are timed just
// like their Component counterparts.
bool RegisterBenchmarkImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);

#endif

} // namespace recoil::wasm::core
