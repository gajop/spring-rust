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

class WasmCoreHost {
public:
	static bool Enabled();
	static bool Load(std::string moduleName, const std::vector<std::uint8_t>& moduleBytes,
		NativeInterface* nativeInterface, WasmEnvironment environment,
		const WasmRuntime& runtime, WasmModuleIdentity& identity, std::string& error);
	static void Unload(std::string_view moduleName);
	static void UnloadAll();
	static bool AnyActive();

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
