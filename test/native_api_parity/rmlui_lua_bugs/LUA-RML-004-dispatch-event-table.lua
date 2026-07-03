-- This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

-- LUA-RML-004: Element:DispatchEvent should accept a Lua parameter table.
--
-- Before the fix, the binding exposed the C++ Rml::Dictionary parameter
-- directly. Sol could not convert a Lua table to Rml::Dictionary, so the
-- documented Element:DispatchEvent("event", { ... }) form failed.
-- Copy this file into LuaUI/Widgets of a game with RmlUi enabled, or run the
-- native parity RmlUi harness, to reproduce the failure on an unfixed build.

function widget:GetInfo()
	return {
		name = "LUA-RML-004 DispatchEvent table repro",
		desc = "Reproduces Element:DispatchEvent rejecting Lua parameter tables",
		author = "RecoilEngine contributors",
		layer = 0,
		enabled = true,
	}
end

function widget:Initialize()
	assert(RmlUi ~= nil, "RmlUi is not available")

	local context = RmlUi.CreateContext("lua_rml_dispatch_event_repro")
	assert(context ~= nil, "RmlUi.CreateContext returned nil")

	local ok, err = pcall(function()
		local document = context:CreateDocument()
		assert(document ~= nil, "Context:CreateDocument returned nil")
		document.inner_rml = [[<button id="target">Target</button>]]
		document:UpdateDocument()

		local target = document:GetElementById("target")
		assert(target ~= nil, "target element missing")

		local sawParameters = false
		target:AddEventListener("custom-repro", function(event)
			sawParameters =
				event.parameters.kind == "synthetic" and
				event.parameters.count == 3 and
				event.parameters.flag == true
		end)

		assert(target:DispatchEvent("custom-repro", { kind = "synthetic", count = 3, flag = true }))
		assert(sawParameters, "listener did not receive dispatched parameters")
	end)

	pcall(function()
		context:UnloadAllDocuments()
	end)
	pcall(function()
		RmlUi.RemoveContext(context)
	end)

	assert(ok, err)
end
