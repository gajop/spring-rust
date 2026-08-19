/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <memory>
#include <string>
#include <string_view>
#include <vector>

#include "WasmEnvironment.h"
#include "WasmRuntime.h"

struct NativeInterface;

// Native C++ Wasmtime/Core host used by the end-to-end transport benchmark and
// as the staging backend for the shipping Core ABI. It deliberately mirrors
// WasmTypedHost's alternate-host seam so the existing NativeInterface event
// conversion and benchmark timing do not need another special path.
class WasmCoreHost {
public:
	static bool Enabled();
	static bool Load(std::string moduleName, const std::vector<std::uint8_t>& moduleBytes,
		NativeInterface* nativeInterface, WasmEnvironment environment, std::string& error);
	static void Unload(std::string_view moduleName);
	static void UnloadAll();
	static bool AnyActive();

	// Returns true when this transport owns the named callin. An empty error is
	// success; a non-empty error means the guest trapped/faulted and remains
	// owned by this transport rather than falling through to another instance.
	static bool DispatchCallin(std::string_view name, const void* query, void* result,
		std::string& error);

	~WasmCoreHost();
	WasmCoreHost(const WasmCoreHost&) = delete;
	WasmCoreHost& operator=(const WasmCoreHost&) = delete;

private:
	struct Backend;
	WasmCoreHost(std::string moduleName, WasmEnvironment environment,
		std::unique_ptr<Backend> backend);

	bool InvokeGameFrame(const void* query, std::string& error);

	std::string moduleName;
	WasmEnvironment environment;
	std::unique_ptr<Backend> backend;
};
