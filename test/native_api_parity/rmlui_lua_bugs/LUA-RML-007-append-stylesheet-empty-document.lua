-- This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

-- LUA-RML-007: Document:AppendToStyleSheet should work on a new document.
--
-- Before the fix, a Lua-created document without an existing stylesheet
-- container crashed when AppendToStyleSheet tried to combine the new stylesheet
-- with *document:GetStyleSheetContainer(). Malformed stylesheet text should also
-- be ignored rather than dereferencing a null parsed stylesheet.
-- Copy this file into LuaUI/Widgets of a game with RmlUi enabled, or run the
-- native parity RmlUi harness, to reproduce the failure on an unfixed build.

function widget:GetInfo()
	return {
		name = "LUA-RML-007 AppendToStyleSheet repro",
		desc = "Reproduces Document:AppendToStyleSheet crashing on new documents",
		author = "RecoilEngine contributors",
		layer = 0,
		enabled = true,
	}
end

function widget:Initialize()
	assert(RmlUi ~= nil, "RmlUi is not available")

	local context = RmlUi.CreateContext("lua_rml_append_stylesheet_repro")
	assert(context ~= nil, "RmlUi.CreateContext returned nil")

	local ok, err = pcall(function()
		local document = context:CreateDocument()
		assert(document ~= nil, "Context:CreateDocument returned nil")
		document:AppendToStyleSheet("body { color: rgb(255, 0, 0); }")
		document:AppendToStyleSheet(".chip { display: block; }")

		local malformedDocument = context:CreateDocument()
		assert(malformedDocument ~= nil, "Context:CreateDocument returned nil for malformed case")
		malformedDocument:AppendToStyleSheet("body { color: ")
	end)

	pcall(function()
		context:UnloadAllDocuments()
	end)
	pcall(function()
		RmlUi.RemoveContext(context)
	end)

	assert(ok, err)
end
