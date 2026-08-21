/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

// The Core UnitsPieces binding reads model-owned piece names straight off the
// simulation globals (`unitHandler`, `gs`) so it can skip the NativeInterface
// scratch copy. These Wasm transport tests deliberately link no simulation, so
// register that one group as a no-op rather than pulling the sim in. Every
// other Core group is linked and exercised normally.

#include "WasmInterface/WasmCoreBindings.h"

namespace recoil::wasm::core {

bool RegisterUnitsPiecesImports(wasmtime_linker_t* /*linker*/, HostState* /*state*/,
	std::string& /*error*/)
{
	return true;
}

} // namespace recoil::wasm::core

// Keep focused Core callback execution coverage in this already-linked tiny
// test translation unit instead of growing the large aggregate test sources.
#include "TestWasmCoreCallbacks.h"
