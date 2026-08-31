/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstddef>
#include <cstdint>
#include <memory>
#include <string>
#include <string_view>
#include <vector>

struct WasmModuleDescriptor;
class WasmInterfaceSystem;

/**
 * NativeInterfaceSystem - Manager for native module integration
 *
 * This class:
 * - Manages DLL loading/unloading
 * - Creates and owns the NativeInterfaceEventClient
 * - Provides interface for Lua message handling
 * - Uses pimpl pattern for fast compilation
 */
class NativeInterfaceSystem {
public:
	NativeInterfaceSystem();
	~NativeInterfaceSystem();

	// Request a module reload. Once a module is active, the reload is delayed
	// until Update() so no native callback can unload its own code or data.
	void Reload();
	// Called by CGame after eventHandler.Update(), when native callbacks have
	// returned and it is safe to unload a module.
	void Update();
	// Advance native and Core-Wasm CUS schedulers once per deterministic
	// simulation frame.
	void Tick(std::uint32_t frame);
	// Give a native module first refusal on keyboard input before RmlUi performs
	// focus traversal. Returning true consumes the event.
	bool KeyPress(int keyCode, int scanCode, bool isRepeat);
	bool KeyRelease(int keyCode, int scanCode);
	// Clear the pending duplicate-dispatch guard when Rml/Lua consumes the
	// event before it reaches the CEventHandler native client.
	void CancelKeyPressPreDispatch();
	void CancelKeyReleasePreDispatch();

	// Special events (called from Lua)
	void HandleLuaMsg(int playerID, int script, int mode, const std::vector<std::uint8_t>& data);
	void HandleLuaCall(const char* msg, size_t msgLength, bool synced);
	// One-way synced-to-unsynced Wasm message delivery. Unsynced receiver
	// failures are reported to the caller for logging but never feed back into
	// the simulation result.
	bool DispatchWasmSyncedMessage(std::string_view message, std::string& error);

	// Wasm module lifecycle is explicit so game/map discovery can feed the same
	// instance registry without exposing runtime internals to CGame.
	bool LoadWasmModule(WasmModuleDescriptor descriptor, std::string& error);
	// Load an optional content-relative manifest through the selected VFS
	// namespace. Missing manifests are intentional; malformed manifests or
	// modules fail closed.
	bool LoadWasmManifest(std::string_view manifestPath, std::string_view vfsModes,
		std::string& error);
	bool UnloadWasmModule(const std::string& moduleName);
	void UnloadAllWasmModules();
	WasmInterfaceSystem* GetWasmInterfaceSystem();

	// Attach the active native module's CUS instance to an engine unit.  The
	// module must register its instance before calling this entry point; the
	// host creates the adapter and invokes its Create callin synchronously.
	bool AttachCusScript(int unitID, std::uint32_t instanceID, std::uint64_t capabilities);

public:
	static NativeInterfaceSystem* s_instance;

private:
	// Pimpl implementation
	class Impl;
	std::unique_ptr<Impl> pImpl;
};
