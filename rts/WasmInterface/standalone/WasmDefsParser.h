/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <string>

class LuaParser;

/// Attempt to generate game definitions via a wasm module instead of
/// gamedata/defs.lua.  If a wasm defs manifest exists and the module
/// successfully produces a Lua table literal, the result is wrapped in a
/// LuaParser that callers can use identically to the Lua-based parser.
///
/// The wasm module receives the same limited API subset as menu/intro
/// (VFS, config, platform, etc.) so it can read TDF/unit files and mod
/// options.  It implements the GenerateDefs callin and returns a Lua
/// source string like "return { UnitDefs = {...}, ... }".
///
/// Returns nullptr when no wasm defs module is present or when the
/// module does not produce a result.  Throws content_error on module
/// failure.
LuaParser* TryWasmDefsParser(std::string& errorOut);
