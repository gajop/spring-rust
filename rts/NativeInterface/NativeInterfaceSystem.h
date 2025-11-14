/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

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

	void Reload();

	// Special events (called from Lua)
	void HandleLuaMsg(int playerID, int script, int mode, const std::vector<std::uint8_t>& data);
	void HandleLuaCall(const char* msg);

public:
	static NativeInterfaceSystem* s_instance;

private:
	// Pimpl implementation
	class Impl;
	std::unique_ptr<Impl> pImpl;
};
