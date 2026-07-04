-- This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

-- LUA-RML-010: Lua RmlUi data-model arrays should expose the reserved
-- `<array>.size` child, matching RmlUi's native C++ array bindings.
--
-- Before the fix, a SolLua table-backed array worked with data-for iteration
-- because the variable definition's Size() method was used directly, but
-- expressions such as `{{items.size}}` and `data-if="items.size == 2"` resolved
-- to an empty variable. RmlUi's bundled samples use `.size` in expressions, so
-- Lua models need to support the same behavior.
-- Copy this file into LuaUI/Widgets of a game with RmlUi enabled, or run the
-- native parity RmlUi harness, to reproduce the failure on an unfixed build.

function widget:GetInfo()
	return {
		name = "LUA-RML-010 Data model array size repro",
		desc = "Reproduces Lua RmlUi data-model arrays missing the .size child",
		author = "RecoilEngine contributors",
		layer = 0,
		enabled = true,
	}
end

function widget:Initialize()
	assert(RmlUi ~= nil, "RmlUi is not available")

	local context = RmlUi.CreateContext("lua_rml_data_model_size_repro")
	assert(context ~= nil, "RmlUi.CreateContext returned nil")

	local ok, err = pcall(function()
		local model = context:OpenDataModel("lua_rml_array_size_model", {
			items = {
				{ name = "One" },
				{ name = "Two" },
			},
		})
		assert(model ~= nil, "Context:OpenDataModel returned nil")

		local document = context:CreateDocument()
		assert(document ~= nil, "Context:CreateDocument returned nil")
		document.inner_rml = [[
			<div data-model="lua_rml_array_size_model">
				<span id="size-text">{{items.size}}</span>
				<span id="size-if" data-if="items.size == 2">two items</span>
			</div>
		]]
		document:Show()
		document:UpdateDocument()
		context:Update()

		local sizeText = document:GetElementById("size-text")
		assert(sizeText ~= nil, "size text element should exist")
		assert(sizeText.inner_rml == "2", "items.size text should render as 2, got: " .. tostring(sizeText.inner_rml))

		local sizeIf = document:GetElementById("size-if")
		assert(sizeIf ~= nil, "size-if element should exist")
		assert(sizeIf.visible == true, "data-if using items.size should be visible")
	end)

	pcall(function()
		context:UnloadAllDocuments()
	end)
	pcall(function()
		RmlUi.RemoveContext(context)
	end)

	assert(ok, err)
end
