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

// Alternate native C++ host for the Spring Core-Wasm ABI. It owns one isolated
// Wasmtime store/linker/instance per guest and exposes only generated Spring
// imports. WasmInterfaceSystem keeps normal module identity/order bookkeeping;
// this class owns execution state and the low-level fast call paths.
class WasmCoreHost {
public:
	static bool Enabled();
	static bool Load(std::string moduleName, const std::vector<std::uint8_t>& moduleBytes,
		NativeInterface* nativeInterface, WasmEnvironment environment,
		const WasmRuntime& runtime, WasmModuleIdentity& identity, std::string& error);
	static void Unload(std::string_view moduleName);
	static void UnloadAll();
	static bool AnyActive();

	// NativeInterfaceEventClient dispatch seam. Returns true when the callin is
	// a Core ABI callin understood by at least one active Core guest. `error` is
	// populated on guest failure/trap; synced integration can treat that as fatal.
	static bool DispatchCallin(std::string_view name, const void* query, void* result,
		std::string& error);

	~WasmCoreHost();
	WasmCoreHost(const WasmCoreHost&) = delete;
	WasmCoreHost& operator=(const WasmCoreHost&) = delete;

private:
	struct Backend;
	WasmCoreHost(std::string moduleName, WasmEnvironment environment,
		std::unique_ptr<Backend> backend);

	bool Invoke(std::string_view name, const void* query, void* result, std::string& error);
	bool InvokeGameFrame(const void* query, std::string& error);
	bool InvokeGameFramePost(const void* query, std::string& error);
	bool InvokeUpdate(const void* query, std::string& error);
	bool InvokeUnitCreated(const void* query, std::string& error);
	bool InvokeUnitPreDamaged(const void* query, void* result, std::string& error);
	bool InvokeAllowUnitCreation(const void* query, void* result, std::string& error);
	bool InvokeDrawWorld(std::string& error);
	bool CallFailed(std::string error);

	std::string moduleName;
	WasmEnvironment environment;
	std::unique_ptr<Backend> backend;
};
