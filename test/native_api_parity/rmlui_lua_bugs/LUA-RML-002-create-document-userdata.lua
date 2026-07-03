-- This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

-- LUA-RML-002: Context:CreateDocument should return RmlUi.Document userdata.
--
-- Before the fix, CreateDocument returned the base Rml::ElementDocument
-- userdata. Lua callers received a value that could not be indexed as the
-- documented RmlUi.Document type.
-- Copy this file into LuaUI/Widgets of a game with RmlUi enabled, or run the
-- native parity RmlUi harness, to reproduce the failure on an unfixed build.

function widget:GetInfo()
	return {
		name = "LUA-RML-002 CreateDocument userdata repro",
		desc = "Reproduces Context:CreateDocument returning the wrong Lua userdata",
		author = "RecoilEngine contributors",
		layer = 0,
		enabled = true,
	}
end

function widget:Initialize()
	assert(RmlUi ~= nil, "RmlUi is not available")

	local context = RmlUi.CreateContext("lua_rml_create_document_repro")
	assert(context ~= nil, "RmlUi.CreateContext returned nil")

	local ok, err = pcall(function()
		local document = context:CreateDocument()
		assert(document ~= nil, "Context:CreateDocument returned nil")
		document.title = "created from Lua"
		assert(document.title == "created from Lua", "created document should expose RmlUi.Document properties")
	end)

	pcall(function()
		context:UnloadAllDocuments()
	end)
	pcall(function()
		RmlUi.RemoveContext(context)
	end)

	assert(ok, err)
end
