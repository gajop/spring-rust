/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <string>
#include <string_view>
#include <vector>

#include "WasmEnvironment.h"
#include "WasmRuntime.h"

struct WasmModuleDescriptor {
	std::string name;
	std::string source;
	WasmEnvironment environment = WasmEnvironment::RulesSynced;
	std::uint32_t order = 0;
	std::string interfaceVersion = std::string(RECOIL_WASM_INTERFACE_VERSION_NUMBER);
	std::vector<std::uint8_t> bytes;
	std::string archive;
};

// A deliberately small, archive-friendly declaration format.  The engine's
// VFS layer can provide the bytes later without changing this metadata parser:
//
//   module(name, path, rules-synced, 0, 1.0.0)
//
// The fifth field is optional for compatibility with early development
// manifests; omission means the current host interface version.  Content
// packages should write it explicitly so an incompatible package fails with a
// useful version diagnostic instead of reaching Component instantiation.
//
// `path` is content-relative and is never treated as a host-OS capability.
struct WasmModuleDeclaration {
	std::string name;
	std::string path;
	WasmEnvironment environment = WasmEnvironment::RulesSynced;
	std::uint32_t order = 0;
	std::string interfaceVersion = "1.0.0";
	// Filled by the multi-archive loader; the text format itself stays
	// content-relative and does not trust archive paths from module authors.
	std::string archive;
};

struct WasmManifestSource {
	std::string archive;
	std::string text;
};

class WasmModuleManifest {
public:
	static bool Parse(std::string_view text,
		std::vector<WasmModuleDeclaration>& declarations, std::string& error);
};
