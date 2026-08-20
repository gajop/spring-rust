/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreHost.h"

bool WasmCoreHost::DispatchModule(WasmCoreHost* host, WasmCoreCallin callin,
	const void* query, void* result, std::string& error)
{
	if (host == nullptr) {
		error = "Core Wasm module handle is null";
		return false;
	}
	if (callin == WasmCoreCallin::Invalid) {
		error = "unsupported Core Wasm callin";
		return false;
	}
	return host->Invoke(callin, query, result, error);
}
