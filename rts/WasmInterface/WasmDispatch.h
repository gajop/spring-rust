/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <string>
#include <vector>

#include "WasmModule.h"

struct WasmCallinEvent {
	std::string name;
	std::vector<std::uint64_t> arguments;
};

class WasmDispatch {
public:
	static bool Dispatch(WasmModule& module, const WasmCallinEvent& event, std::string& error);
};
