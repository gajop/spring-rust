/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <string>
#include <string_view>
#include <vector>

#include "WasmEnvironment.h"

// A deliberately small, archive-friendly declaration format.  The engine's
// VFS layer can provide the bytes later without changing this metadata parser:
//
//   module(name, path, rules-synced, 0)
//
// `path` is content-relative and is never treated as a host-OS capability.
struct WasmModuleDeclaration {
	std::string name;
	std::string path;
	WasmEnvironment environment = WasmEnvironment::RulesSynced;
	std::uint32_t order = 0;
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
