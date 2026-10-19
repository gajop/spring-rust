/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */
#pragma once

#include <cstdint>
#include <string>
#include <unordered_map>
#include <vector>
#include "WasmCoreAbi.h"

namespace recoil::wasm::core {

// Export aliases give the embedder access to otherwise hidden state, without
// injecting guest instructions or changing original indices/exports. Validate
// the original module first; its content hash remains the module identity.
struct StateExports {
	std::string prefix;
	std::uint32_t functions = 0;
	std::uint32_t globals = 0;
	std::uint32_t tables = 0;
	static bool Prepare(const std::vector<std::uint8_t>& original,
		std::vector<std::uint8_t>& compiled, StateExports& exports, std::string& error);
	std::string Name(char kind, std::uint32_t index) const;
};

#if defined(RECOIL_WASMTIME_AVAILABLE)
class StateInspector {
public:
	bool Bind(wasmtime_context_t* context, const wasmtime_instance_t& instance,
		const StateExports& exports, std::string& error);
	// Canonical LE bytes: exact numeric bits, globals/table index order, null
	// references or original module function indices. Never serialize pointers,
	// store IDs, padding or the process-local iteration order of this map.
	bool Snapshot(std::vector<std::uint8_t>& bytes, std::string& error) const;
private:
	bool Value(const wasmtime_val_t& value, std::vector<std::uint8_t>& bytes,
		std::string& error) const;
	wasmtime_context_t* context = nullptr;
	std::vector<wasmtime_global_t> globals;
	std::vector<wasmtime_table_t> tables;
	std::unordered_map<void*, std::uint32_t> functions;
};
#endif
} // namespace recoil::wasm::core
