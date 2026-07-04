-- This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

-- LUA-RML-008: Removing the active RmlUi debug context should not crash.
--
-- Before the fix, RmlUi.SetDebugContext(context) installed the debugger hook
-- document into the target context. Context:UnloadAllDocuments() or
-- RmlUi.RemoveContext(context) could then destroy that hook without first
-- detaching the debugger, so an ASAN build reported a use-after-poison in
-- Rml::DebuggerPlugin::SetContext() from RmlGui::Update().
-- Copy this file into LuaUI/Widgets of a game with RmlUi enabled, or run the
-- native parity RmlUi harness, to reproduce the failure on an unfixed build.

function widget:GetInfo()
	return {
		name = "LUA-RML-008 Debug context removal repro",
		desc = "Reproduces RmlUi debug context removal crashing under ASAN",
		author = "RecoilEngine contributors",
		layer = 0,
		enabled = true,
	}
end

function widget:Initialize()
	assert(RmlUi ~= nil, "RmlUi is not available")

	local context = RmlUi.CreateContext("lua_rml_debug_context_removal_repro")
	assert(context ~= nil, "RmlUi.CreateContext returned nil")

	RmlUi.SetDebugContext(context)
	context:UnloadAllDocuments()
	RmlUi.RemoveContext(context)
end
