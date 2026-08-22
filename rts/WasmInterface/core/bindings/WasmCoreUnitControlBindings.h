/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <string>

#include "WasmCoreBindings.h"

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)

bool RegisterUnitControlImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);

#endif

} // namespace recoil::wasm::core
