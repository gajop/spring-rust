-- This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

-- LUA-RML-003: Context:UnloadDocument should accept Lua-created documents.
--
-- Before the fix, the binding exposed the raw Rml::Context::UnloadDocument
-- overload and sol could not match the RmlUi.Document userdata returned by
-- Context:CreateDocument.
-- Copy this file into LuaUI/Widgets of a game with RmlUi enabled, or run the
-- native parity RmlUi harness, to reproduce the failure on an unfixed build.

function widget:GetInfo()
	return {
		name = "LUA-RML-003 UnloadDocument repro",
		desc = "Reproduces Context:UnloadDocument rejecting Lua document userdata",
		author = "RecoilEngine contributors",
		layer = 0,
		enabled = true,
	}
end

function widget:Initialize()
	assert(RmlUi ~= nil, "RmlUi is not available")

	local context = RmlUi.CreateContext("lua_rml_unload_document_repro")
	assert(context ~= nil, "RmlUi.CreateContext returned nil")

	local ok, err = pcall(function()
		local document = context:CreateDocument()
		assert(document ~= nil, "Context:CreateDocument returned nil")
		context:UnloadDocument(document)
	end)

	pcall(function()
		context:UnloadAllDocuments()
	end)
	pcall(function()
		RmlUi.RemoveContext(context)
	end)

	assert(ok, err)
end
