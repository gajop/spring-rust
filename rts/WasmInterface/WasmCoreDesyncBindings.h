/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <string>

#include "WasmCoreBindings.h"

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)

// Imports that are available in every environment, synced included, and whose
// results are NOT deterministic across clients. See WasmCoreDesyncBindings.cpp
// for the policy behind the name.
bool RegisterDesyncImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);

#endif

} // namespace recoil::wasm::core
