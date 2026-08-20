/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <string_view>
#include <vector>

#include "WasmEnvironment.h"
#include "WasmRuntime.h"

namespace recoil::wasm::core {

// Validate a Spring Core-ABI module before Wasmtime compilation. This is
// deliberately stricter than the legacy core fixture path in WasmRuntime:
// imports are an exact generated capability set and synced memory/table limits
// must be fixed so growth cannot depend on local host resource availability.
WasmValidationResult ValidateModule(const std::vector<std::uint8_t>& bytes,
	WasmEnvironment environment, std::string_view interfaceVersion,
	const WasmRuntimeConfig& config);

} // namespace recoil::wasm::core
