/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstddef>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

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

public:
	static NativeInterfaceSystem* s_instance;

private:
	// Pimpl implementation
	class Impl;
	std::unique_ptr<Impl> pImpl;
};
