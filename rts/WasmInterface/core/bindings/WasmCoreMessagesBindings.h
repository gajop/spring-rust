/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <string>

#include "WasmCoreBindings.h"

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)

bool RegisterMessagesImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);

// SendLuaRulesMsg as seen by the caller's environment. Unsynced modules send
// over the network like Lua; synced modules run on every client, so theirs is
// delivered locally.
using SendLuaRulesMsgFunction = void (*)(const SendLuaRulesQuery*, SendLuaRulesResult*);
SendLuaRulesMsgFunction SendLuaRulesMsgFor(const HostState* state);

#endif

} // namespace recoil::wasm::core
