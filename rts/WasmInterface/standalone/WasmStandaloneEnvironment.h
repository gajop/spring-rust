/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <memory>
#include <string>
#include <string_view>
#include <vector>

enum class WasmEnvironment : std::uint8_t;
enum class WasmCoreCallin : std::uint16_t;
struct NativeInterface;
class WasmInterfaceSystem;
class NativeInterfaceEventClient;
class SharedLib;

class WasmStandaloneEnvironment {
public:
	~WasmStandaloneEnvironment();

	WasmStandaloneEnvironment(const WasmStandaloneEnvironment&) = delete;
	WasmStandaloneEnvironment& operator=(const WasmStandaloneEnvironment&) = delete;

	static std::unique_ptr<WasmStandaloneEnvironment> Create();

	bool LoadManifest(const std::string& manifestPath);
	bool TryLoadNativeDLL(const std::string& pathStem);

	void EnsureEventClient();
	void RemoveEventClient();
	void ActivateMenu(const std::string& message);
	void ActivateGame();

	void Update();
	bool HasModules(WasmEnvironment environment) const;
	bool HasNativeModule() const { return m_nativeModuleLoaded; }
	WasmInterfaceSystem* GetWasmSystem() const;
	NativeInterface* GetNativeInterface() const;

private:
	WasmStandaloneEnvironment();

	std::unique_ptr<NativeInterface> m_nativeInterface;
	std::unique_ptr<WasmInterfaceSystem> m_wasmSystem;
	std::unique_ptr<NativeInterfaceEventClient> m_eventClient;
	std::unique_ptr<SharedLib> m_sharedLib;
	bool m_nativeModuleLoaded = false;
};
