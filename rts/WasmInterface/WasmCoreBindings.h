/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <string>

#include "NativeInterface/NativeInterface.h"
#include "WasmCoreAbi.h"

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)

// Per-instance state passed directly to unchecked host callbacks. The pointer
// is stable for the full Store/instance lifetime. Generated callbacks never do
// a function-name lookup or construct a semantic value tree.
struct HostState {
	NativeInterface* native = nullptr;
	Memory memory;
};

// Registers the first production Core ABI slice. This is deliberately a
// generated-shaped API: each NativeInterface function becomes one statically
// typed raw callback. The generator can emit the same pattern for the complete
// API once this slice is benchmarked end to end.
bool RegisterFastImports(wasmtime_linker_t* linker, HostState* state, std::string& error);

// Cache the guest memory immediately after instantiation. Imports reached from
// a start function can bind it lazily through wasmtime_caller_t; this call
// removes that lookup from all normal steady-state imports.
bool BindGuestMemory(HostState& state, wasmtime_context_t* context,
	const wasmtime_instance_t& instance, std::string& error);

#endif

} // namespace recoil::wasm::core
