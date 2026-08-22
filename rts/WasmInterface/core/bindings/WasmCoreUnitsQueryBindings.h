/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <string>

#include "WasmCoreBindings.h"

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)

// Variable-size and spatial UnitsQuery bindings. Kept in a separate translation
// unit from the tiny UnitsInfo hot slice so both code size and benchmarks remain
// attributable while they share the exact same HostState/memory/budget rules.
bool RegisterUnitsQueryImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);

#endif

} // namespace recoil::wasm::core
