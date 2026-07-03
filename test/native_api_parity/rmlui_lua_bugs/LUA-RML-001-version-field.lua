-- This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

-- LUA-RML-001: RmlUi.version should be a string field.
--
-- Before the fix, the binding exposed Rml::GetVersion as a readonly property,
-- so Lua saw RmlUi.version as a function instead of the documented string.
-- Copy this file into LuaUI/Widgets of a game with RmlUi enabled, or run the
-- native parity RmlUi harness, to reproduce the failure on an unfixed build.

function widget:GetInfo()
	return {
		name = "LUA-RML-001 version field repro",
		desc = "Reproduces RmlUi.version being exposed as a function",
		author = "RecoilEngine contributors",
		layer = 0,
		enabled = true,
	}
end

function widget:Initialize()
	assert(RmlUi ~= nil, "RmlUi is not available")
	assert(type(RmlUi.version) == "string", "RmlUi.version should be a string, got " .. type(RmlUi.version))
end
