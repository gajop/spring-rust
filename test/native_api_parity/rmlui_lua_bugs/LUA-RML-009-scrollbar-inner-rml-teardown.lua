-- This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

-- LUA-RML-009: Replacing RML containing a live scrollbar should not crash.
--
-- With vendored RmlUi 6.2 at 2230d1a6, this Lua-visible sequence trips ASAN:
-- create a block with real overflow, let RmlUi create scrollbar widgets, then
-- replace the document inner_rml. RmlUi destroys the scrollable subtree, and
-- WidgetScroll::~WidgetScroll tries to remove event listeners from scrollbar
-- child elements whose EventDispatchers have already been destroyed.
-- The observed ASAN path is:
-- Element::SetInnerRML -> Element::~Element -> ElementScroll::~ElementScroll
-- -> WidgetScroll::~WidgetScroll -> EventDispatcher::DetachEvent.
-- Copy this file into LuaUI/Widgets of a game with RmlUi enabled, or run the
-- native parity RmlUi harness, to reproduce the failure on an unfixed ASAN
-- build.

function widget:GetInfo()
	return {
		name = "LUA-RML-009 Scrollbar inner_rml teardown repro",
		desc = "Reproduces RmlUi WidgetScroll teardown crashing under ASAN",
		author = "RecoilEngine contributors",
		layer = 0,
		enabled = true,
	}
end

function widget:Initialize()
	assert(RmlUi ~= nil, "RmlUi is not available")

	local context = RmlUi.CreateContext("lua_rml_scrollbar_teardown_repro")
	assert(context ~= nil, "RmlUi.CreateContext returned nil")
	context.dimensions = RmlUi.Vector2i.new(320, 240)

	local ok, err = pcall(function()
		local document = context:CreateDocument()
		assert(document ~= nil, "Context:CreateDocument returned nil")
		document:AppendToStyleSheet([[
			#host {
				display: block;
				position: absolute;
				left: 0px;
				top: 0px;
				width: 80px;
				height: 80px;
				overflow: auto;
			}
			#large {
				display: block;
				width: 320px;
				height: 320px;
			}
		]])
		document.inner_rml = [[
			<div id="host">
				<div id="large">large content</div>
			</div>
		]]
		document:Show()
		document:UpdateDocument()
		context:Update()

		local host = document:GetElementById("host")
		assert(host ~= nil, "scroll host should exist")
		assert(host.scroll_height > host.client_height, "scroll host should overflow vertically")
		host.scroll_top = 16
		assert(host.scroll_top > 0, "scroll host should scroll vertically")

		document.inner_rml = [[<div id="replacement">replacement</div>]]
		document:UpdateDocument()
		context:Update()
		assert(document:GetElementById("replacement") ~= nil, "replacement element should exist")
	end)

	pcall(function()
		context:UnloadAllDocuments()
	end)
	pcall(function()
		RmlUi.RemoveContext(context)
	end)

	assert(ok, err)
end
