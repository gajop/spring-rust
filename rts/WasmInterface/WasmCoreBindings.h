/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <string>

#include "NativeInterface/NativeInterface.h"
#include "WasmCoreAbi.h"

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)

struct HostState {
	NativeInterface* native = nullptr;
	Memory memory;
};

bool RegisterFastImports(wasmtime_linker_t* linker, HostState* state, std::string& error);
bool BindGuestMemory(HostState& state, wasmtime_context_t* context,
	const wasmtime_instance_t& instance, std::string& error);

// Per-instance fast-path state. All import/export resolution and signature
// checking happens during RegisterImports/Bind. Frame-time dispatch performs
// no string lookup, allocation, or semantic value conversion.
class InstanceBindings {
public:
	explicit InstanceBindings(NativeInterface* nativeInterface)
	{
		host.native = nativeInterface;
	}

	bool RegisterImports(wasmtime_linker_t* linker, std::string& error)
	{
		return RegisterFastImports(linker, &host, error);
	}

	bool Bind(wasmtime_context_t* context, const wasmtime_instance_t& instance,
		std::string& error);
	bool GameFrame(wasmtime_context_t* context, std::int32_t frame, std::string& error) const
	{
		return gameFrame.Call(context, frame, error);
	}
	bool HasGameFrame() const { return gameFrame.Present(); }
	HostState& Host() { return host; }

private:
	HostState host;
	I32ToVoidExport gameFrame;
};

#endif

} // namespace recoil::wasm::core
