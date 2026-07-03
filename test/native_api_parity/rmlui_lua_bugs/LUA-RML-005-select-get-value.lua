-- This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

-- LUA-RML-005: Element:GetValue should return values for select controls.
--
-- Before the fix, Element:GetValue only special-cased input and textarea tag
-- names. RmlUi select elements are also form controls, but they returned an
-- empty string through this Lua binding.
-- Copy this file into LuaUI/Widgets of a game with RmlUi enabled, or run the
-- native parity RmlUi harness, to reproduce the failure on an unfixed build.

function widget:GetInfo()
	return {
		name = "LUA-RML-005 select GetValue repro",
		desc = "Reproduces Element:GetValue returning an empty string for select controls",
		author = "RecoilEngine contributors",
		layer = 0,
		enabled = true,
	}
end

function widget:Initialize()
	assert(RmlUi ~= nil, "RmlUi is not available")

	local context = RmlUi.CreateContext("lua_rml_select_value_repro")
	assert(context ~= nil, "RmlUi.CreateContext returned nil")

	local ok, err = pcall(function()
		local document = context:CreateDocument()
		assert(document ~= nil, "Context:CreateDocument returned nil")
		document.inner_rml = [[
			<select id="choice">
				<option value="one" selected="">One</option>
			</select>
		]]
		document:UpdateDocument()

		local selectElement = document:GetElementById("choice")
		assert(selectElement ~= nil, "select element missing")
		assert(selectElement:GetValue() == "one", "select GetValue should return selected option value")
	end)

	pcall(function()
		context:UnloadAllDocuments()
	end)
	pcall(function()
		RmlUi.RemoveContext(context)
	end)

	assert(ok, err)
end
