-- This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

-- LUA-RML-006: ElementFormControlSelect.options should be a field.
--
-- Before the fix, options was documented as a RmlUi.SelectOptionsProxy field
-- but bound as a method. Lua code using selectControl.options[0] therefore
-- attempted to index a function.
-- Copy this file into LuaUI/Widgets of a game with RmlUi enabled, or run the
-- native parity RmlUi harness, to reproduce the failure on an unfixed build.

function widget:GetInfo()
	return {
		name = "LUA-RML-006 select options field repro",
		desc = "Reproduces ElementFormControlSelect.options being exposed as a function",
		author = "RecoilEngine contributors",
		layer = 0,
		enabled = true,
	}
end

function widget:Initialize()
	assert(RmlUi ~= nil, "RmlUi is not available")

	local context = RmlUi.CreateContext("lua_rml_select_options_repro")
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

		local selectControl = RmlUi.Element.As.ElementFormControlSelect(selectElement)
		assert(selectControl ~= nil, "select cast failed")
		assert(type(selectControl.options) ~= "function", "selectControl.options should be a proxy field")
		assert(selectControl.options[0].value == "one", "first option value should be available through options[0]")
	end)

	pcall(function()
		context:UnloadAllDocuments()
	end)
	pcall(function()
		RmlUi.RemoveContext(context)
	end)

	assert(ok, err)
end
