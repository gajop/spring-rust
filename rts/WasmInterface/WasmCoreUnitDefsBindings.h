/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <string>

#include "WasmCoreBindings.h"

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)

// Representative raw-byte string bindings. Core ABI strings are byte strings,
// matching the engine/Lua contract; no UTF-8 conversion occurs at this layer.
bool RegisterUnitDefsImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);

#endif

} // namespace recoil::wasm::core
