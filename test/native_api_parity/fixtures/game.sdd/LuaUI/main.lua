local Common = VFS.Include("LuaRules/Utilities/native_api_parity_common.lua")
local GeneratedTests = VFS.Include("LuaRules/Utilities/generated_api_tests.lua")
local outputPath = Common.outputDir() .. "/widget.jsonl"
local sentInventory = false
local ranGeneratedTests = false
local ranRmlUiTests = false
local fixtureIDs = {}

local function record(name, payload)
	payload.context = "widget"
	Common.setTestName(payload, name)
	Common.appendJsonLine(outputPath, payload)
end

local function recordInventory()
	if sentInventory then
		return
	end
	sentInventory = true
	record("context_inventory", { functions = Common.springFunctionInventory() })
end

local function runGeneratedTests()
	if ranGeneratedTests then
		return
	end
	ranGeneratedTests = true
	Common.runPortableReadOnlyTests("widget", GeneratedTests, record, function(encoded)
		Spring.InvokeNativeModule(encoded)
	end, fixtureIDs)
end

local function assertEqual(label, actual, expected)
	if actual ~= expected then
		error(label .. ": actual=" .. tostring(actual) .. ", expected=" .. tostring(expected), 0)
	end
end

local function assertNear(label, actual, expected)
	if math.abs(actual - expected) > 0.001 then
		error(label .. ": actual=" .. tostring(actual) .. ", expected=" .. tostring(expected), 0)
	end
end

local function assertContains(label, actual, expected)
	if type(actual) ~= "string" or not string.find(actual, expected, 1, true) then
		error(label .. ": actual=" .. tostring(actual) .. ", expected substring=" .. tostring(expected), 0)
	end
end

local function assertNotContains(label, actual, unexpected)
	if type(actual) == "string" and string.find(actual, unexpected, 1, true) then
		error(label .. ": actual=" .. tostring(actual) .. ", unexpected substring=" .. tostring(unexpected), 0)
	end
end

local function expectElement(label, element)
	if element == nil then
		error(label .. " should exist", 0)
	end
	return element
end

local function expectCast(label, value)
	if value == nil then
		error(label .. " cast failed", 0)
	end
	return value
end

local function tableLength(values)
	local count = 0
	for _ in pairs(values) do
		count = count + 1
	end
	return count
end

local function withRmlContext(label, fn)
	local name = "native_api_parity_lua_rml_" .. label
	local existing = RmlUi.GetContext(name)
	if existing ~= nil then
		RmlUi.RemoveContext(existing)
	end

	local context = RmlUi.CreateContext(name)
	if context == nil then
		error("RmlUi.CreateContext(" .. name .. ") returned nil", 0)
	end

	local ok, err = pcall(fn, context)
	pcall(function()
		context:UnloadAllDocuments()
	end)
	pcall(function()
		RmlUi.RemoveContext(context)
	end)
	if not ok then
		error(err, 0)
	end
end

local function createDocument(context)
	local document = context:CreateDocument()
	if document == nil then
		error("Context:CreateDocument returned nil", 0)
	end
	return document
end

local function runRmlCheck(name, fn)
	local ok, err = pcall(fn)
	record(name, {
		status = ok and "pass" or "fail",
		message = ok and "" or tostring(err),
	})
end

local function runRmlUiTests()
	if ranRmlUiTests or not Common.enableRenderingTests() then
		return
	end
	ranRmlUiTests = true

	runRmlCheck("lua_rml_available", function()
		if RmlUi == nil then
			error("RmlUi table is not available", 0)
		end
		assertEqual("RmlUi version type", type(RmlUi.version), "string")
	end)

	if RmlUi == nil then
		return
	end

	runRmlCheck("lua_rml_lifecycle_behavior", function()
		withRmlContext("lifecycle", function(context)
			local document = createDocument(context)
			assertEqual("document context", document.context, context)
			assertEqual("document initial title", document.title, "")
			document.title = "Lua Rml Behavior"
			assertEqual("document title after set", document.title, "Lua Rml Behavior")
			document:Show()
			assertEqual("document visible after show", document:IsVisible(), true)
			document:Hide()
			assertEqual("document visible after hide", document:IsVisible(), false)
			context:UnloadDocument(document)
		end)
	end)

	runRmlCheck("lua_rml_context_document_extra_behavior", function()
		withRmlContext("context_document_extra", function(context)
			context.dimensions = RmlUi.Vector2i.new(320, 240)
			assertEqual("context dimensions x", context.dimensions.x, 320)
			assertEqual("context dimensions y", context.dimensions.y, 240)
			context.dp_ratio = 1.5
			assertNear("context dp_ratio", context.dp_ratio, 1.5)

			context:ActivateTheme("native-api-parity-theme", true)
			assertEqual("theme active", context:IsThemeActive("native-api-parity-theme"), true)
			context:ActivateTheme("native-api-parity-theme", false)
			assertEqual("theme inactive", context:IsThemeActive("native-api-parity-theme"), false)
			assertEqual("path requests initially empty", tableLength(RmlUi.GetDocumentPathRequests("native-api-parity.rml")), 0)
			RmlUi.ClearDocumentPathRequests("native-api-parity.rml")
			assertEqual("path requests after clear", tableLength(RmlUi.GetDocumentPathRequests("native-api-parity.rml")), 0)

			local document = createDocument(context)
			document.id = "extra-document"
			document.title = "Lua Extra Document"
			document:Show(RmlUi.RmlModalFlag.Modal, RmlUi.RmlFocusFlag.Document)
			assertEqual("document modal after show", document.modal, true)
			assertEqual("context get document", context:GetDocument("extra-document"), document)
			assertEqual("document url type", type(document.url), "string")

			local text = expectElement("text node", document:AppendChild(document:CreateTextNode("hello")))
			local textElement = expectCast("text node cast", RmlUi.Element.As.ElementText(text))
			assertEqual("text node initial text", textElement.text, "hello")
			textElement.text = "world"
			assertEqual("text node updated text", textElement.text, "world")

			local closeDocument = createDocument(context)
			closeDocument.id = "close-document"
			closeDocument:Show()
			assertEqual("close document lookup before close", context:GetDocument("close-document"), closeDocument)
			closeDocument:Close()
			context:Update()
			assertEqual("close document lookup after close", context:GetDocument("close-document"), nil)
		end)
	end)

	runRmlCheck("lua_rml_dom_behavior", function()
		withRmlContext("dom", function(context)
			local document = createDocument(context)
			document.inner_rml = [[
				<div id="container" class="panel primary" data-owner="lua">
					<span id="alpha" class="chip hot" data-role="primary">A</span>
					<button id="beta" class="chip">B</button>
				</div>
			]]
			document:UpdateDocument()

			local alpha = expectElement("alpha", document:GetElementById("alpha"))
			local beta = expectElement("beta", document:QuerySelector("#beta"))
			assertEqual("alpha selector", document:QuerySelector("span.hot"), alpha)
			assertEqual("alpha tag", alpha.tag_name, "span")
			assertEqual("alpha id", alpha.id, "alpha")
			assertEqual("alpha data-role", alpha:GetAttribute("data-role"), "primary")
			alpha:RemoveAttribute("data-role")
			assertEqual("alpha data-role removed", alpha:HasAttribute("data-role"), false)
			alpha:SetClass("selected", true)
			assertEqual("alpha selected class", alpha:IsClassSet("selected"), true)
			assertEqual("alpha matches selected", alpha:Matches("span.selected"), true)
			assertEqual("beta id", beta.id, "beta")
		end)
	end)

	runRmlCheck("lua_rml_child_mutation_behavior", function()
		withRmlContext("children", function(context)
			local document = createDocument(context)
			local parentPtr = document:CreateElement("div")
			local parent = expectElement("parent", document:AppendChild(parentPtr))
			parent.id = "parent"

			local first = expectElement("first", parent:AppendChild(document:CreateElement("p")))
			first.id = "first"
			local third = expectElement("third", parent:AppendChild(document:CreateElement("p")))
			third.id = "third"

			local second = expectElement("second", parent:InsertBefore(document:CreateElement("p"), third))
			second.id = "second"
			assertEqual("child count after insert", tableLength(parent.child_nodes), 3)
			assertEqual("first child id", parent:GetChild(0).id, "first")
			assertEqual("second child id", parent:GetChild(1).id, "second")
			assertEqual("third child id", parent:GetChild(2).id, "third")
			assertEqual("parent first_child", parent.first_child, first)
			assertEqual("parent last_child", parent.last_child, third)
			assertEqual("first next_sibling", first.next_sibling, second)
			assertEqual("third previous_sibling", third.previous_sibling, second)
			assertEqual("second parent_node", second.parent_node, parent)
			assertEqual("parent has children", parent:HasChildNodes(), true)

			local removedSecond = parent:ReplaceChild(document:CreateElement("section"), second)
			local replacement = expectElement("replacement", parent:QuerySelector("section"))
			replacement.id = "replacement"
			assertEqual("second detached", document:QuerySelector("#second"), nil)
			assertEqual("replacement child id", parent:GetChild(1).id, "replacement")
			assertEqual("removed second reattaches", expectElement("removed second", parent:AppendChild(removedSecond)).id, "second")

			local removed = parent:RemoveChild(third)
			assertEqual("third removed", document:QuerySelector("#third"), nil)
			local reattached = expectElement("reattached third", parent:AppendChild(removed))
			assertEqual("reattached third", reattached.id, "third")

			local clone = expectElement("clone", parent:AppendChild(first:Clone()))
			clone.id = "first-clone"
			assertEqual("clone id", parent:GetChild(4).id, "first-clone")
		end)
	end)

	runRmlCheck("lua_rml_element_property_behavior", function()
		withRmlContext("properties", function(context)
			local document = createDocument(context)
			document.inner_rml = [[<div id="box" class="initial" data-count="7"><span id="child">child</span></div>]]
			document:UpdateDocument()

			local box = expectElement("box", document:GetElementById("box"))
			assertEqual("box class_name before", box.class_name, "initial")
			box.class_name = "initial selected"
			assertEqual("box selected class", box:IsClassSet("selected"), true)
			assertEqual("attributes count includes id/class/data", tableLength(box.attributes) >= 3, true)
			assertEqual("attributes data-count", box.attributes["data-count"], "7")
			box.style["display"] = "block"
			assertEqual("style display set", box.style["display"], "block")
			box.style["display"] = nil
			if box.style["display"] == "block" then
				error("style display should be removed", 0)
			end
			assertEqual("owner document", box.owner_document, document)
			assertEqual("address type", type(box.address), "string")
			assertEqual("visible property type", type(box.visible), "boolean")
		end)
	end)

	runRmlCheck("lua_rml_event_behavior", function()
		withRmlContext("events", function(context)
			local document = createDocument(context)
			document.inner_rml = [[<button id="event-button">B</button>]]
			document:UpdateDocument()

			local button = expectElement("event button", document:GetElementById("event-button"))
			local calls = 0
			local eventLog = ""
			button:AddEventListener("click", function(event, element, ownerDocument)
				calls = calls + 1
				local parameters = event.parameters
				eventLog = table.concat({
					event.type,
					tostring(element.id == "event-button"),
					tostring(ownerDocument.title == "Lua Event Document"),
					parameters.kind,
					tostring(parameters.count),
					tostring(parameters.flag),
				}, ":")
			end)
			document.title = "Lua Event Document"
			assertEqual("dispatch event", button:DispatchEvent("click", { kind = "synthetic", count = 3, flag = true }), true)
			assertEqual("event callback count", calls, 1)
			assertEqual("event callback log", eventLog, "click:true:true:synthetic:3:true")
		end)
	end)

	runRmlCheck("lua_rml_form_control_behavior", function()
		withRmlContext("forms", function(context)
			local document = createDocument(context)
			document.inner_rml = [[
				<input id="input" value="abcdef" />
				<textarea id="text">hello world</textarea>
				<select id="select">
					<option value="one" selected="">One</option>
				</select>
			]]
			document:UpdateDocument()
			document:Show()

			local input = expectElement("input", document:GetElementById("input"))
			local inputControl = expectCast("input", RmlUi.Element.As.ElementFormControlInput(input))
			assertEqual("input value", input:GetValue(), "abcdef")
			input:Focus()
			inputControl:SetSelection(1, 4)
			local inputStart, inputEnd, inputText = inputControl:GetSelection()
			assertEqual("input selection start", inputStart, 1)
			assertEqual("input selection end", inputEnd, 4)
			assertEqual("input selected text", inputText, "bcd")

			local textarea = expectElement("textarea", document:GetElementById("text"))
			local textareaControl = expectCast("textarea", RmlUi.Element.As.ElementFormControlTextArea(textarea))
			assertEqual("textarea value", textarea:GetValue(), "hello world")
			textarea:Focus()
			textareaControl:SetSelection(0, 5)
			local textStart, textEnd, text = textareaControl:GetSelection()
			assertEqual("textarea selection start", textStart, 0)
			assertEqual("textarea selection end", textEnd, 5)
			assertEqual("textarea selected text", text, "hello")

			local select = expectElement("select", document:GetElementById("select"))
			local selectControl = expectCast("select", RmlUi.Element.As.ElementFormControlSelect(select))
			assertEqual("select value", select:GetValue(), "one")
			assertEqual("select selection", selectControl.selection, 0)
			assertEqual("select option value", selectControl.options[0].value, "one")
			selectControl.selection = 0
			assertEqual("select selection after set", selectControl.selection, 0)

			local baseControl = expectCast("input as base control", RmlUi.Element.As.ElementFormControl(input))
			baseControl.name = "lua_input"
			baseControl.value = "updated"
			assertEqual("base control name", baseControl.name, "lua_input")
			assertEqual("base control value", input:GetValue(), "updated")
			baseControl.disabled = true
			assertEqual("base control disabled", baseControl.disabled, true)
		end)
	end)

	runRmlCheck("lua_rml_data_model_binding_behavior", function()
		withRmlContext("data_model_binding", function(context)
			local clickLog = {}
			local model = context:OpenDataModel("native_api_parity_lua_model", {
				title = "Alpha",
				field = "First",
				flag = false,
				showDetails = true,
				nested = {
					label = "Nested A",
				},
				items = {
					{ name = "One", count = 1 },
					{ name = "Two", count = 2 },
				},
				record_click = function(event, payload, observedTitle)
					clickLog[#clickLog + 1] = table.concat({
						event.type,
						tostring(payload),
						tostring(observedTitle),
					}, ":")
				end,
			})
			if model == nil then
				error("Context:OpenDataModel returned nil", 0)
			end

			local document = createDocument(context)

			local function refresh()
				context:Update()
			end

			local function renderText()
				refresh()
				return document.inner_rml
			end

			document.inner_rml = [[
				<div id="root" data-model="native_api_parity_lua_model">
					<span id="title">{{title}}</span>
					<span id="nested">{{nested.label}}</span>
					<span id="count">{{items.size}}</span>
					<div id="details" data-if="showDetails">details visible</div>
					<input id="field-input" type="text" data-value="field" />
					<span id="field-text">{{field}}</span>
					<input id="flag-input" type="checkbox" value="yes" data-checked="flag" />
					<span class="row" data-for="item : items">{{item.name}}={{item.count}};</span>
					<button id="model-event" data-event-click="record_click('payload', title)">Click</button>
				</div>
			]]
			document:Show()
			document:UpdateDocument()

			local text = renderText()
			assertContains("initial title binding", text, "Alpha")
			assertContains("initial nested binding", text, "Nested A")
			assertEqual("initial array size binding", expectElement("count", document:GetElementById("count")).inner_rml, "2")
			assertContains("initial first row", text, "One=1")
			assertContains("initial second row", text, "Two=2")
			assertContains("initial conditional", text, "details visible")
			assertEqual("initial conditional visible", expectElement("details", document:GetElementById("details")).visible, true)
			local fieldInput = expectCast("field input", RmlUi.Element.As.ElementFormControl(expectElement("field input", document:GetElementById("field-input"))))
			assertEqual("initial data-value input", fieldInput.value, "First")
			local flagInput = expectCast("flag input", RmlUi.Element.As.ElementFormControlInput(expectElement("flag input", document:GetElementById("flag-input"))))
			assertEqual("initial data-checked checkbox", flagInput.checked, false)

			local eventButton = expectElement("model event button", document:GetElementById("model-event"))
			assertEqual("model event dispatch", eventButton:DispatchEvent("click"), true)
			assertEqual("model event callback count", #clickLog, 1)
			assertEqual("model event callback args", clickLog[1], "click:payload:Alpha")

			model.title = "Beta"
			model.field = "Second"
			model.flag = true
			model.nested.label = "Nested B"
			model.items[2].name = "Deux"
			model.items[2].count = 22
			model.showDetails = false

			text = renderText()
			assertContains("updated title binding", text, "Beta")
			assertContains("updated nested binding", text, "Nested B")
			assertContains("updated second row", text, "Deux=22")
			assertEqual("details element hidden by data-if", expectElement("details", document:GetElementById("details")).visible, false)
			assertEqual("updated data-value input", fieldInput.value, "Second")
			assertEqual("updated data-value text", expectElement("field text", document:GetElementById("field-text")).inner_rml, "Second")
			assertEqual("updated data-checked checkbox", flagInput.checked, true)

			assertEqual("data-value change dispatch", fieldInput:DispatchEvent("change", { value = "Third" }), true)
			text = renderText()
			assertEqual("data-value change updated model", model.field, "Third")
			assertEqual("data-value change updated text", expectElement("field text", document:GetElementById("field-text")).inner_rml, "Third")
			assertEqual("data-value change updated input", fieldInput.value, "Third")

			assertEqual("data-checked change dispatch", flagInput:DispatchEvent("change", {
				["data-binding-override-value"] = false,
				value = "",
				checked = false,
			}), true)
			text = renderText()
			assertEqual("data-checked change updated model", model.flag, false)
			assertEqual("data-checked change updated checkbox", flagInput.checked, false)

			local raw = model:__GetTable()
			raw.title = "Gamma"
			text = renderText()
			assertContains("raw mutation without dirty keeps previous title", text, "Beta")
			assertNotContains("raw mutation without dirty not rendered", text, "Gamma")

			model:__SetDirty("title")
			text = renderText()
			assertContains("manual dirty renders raw title", text, "Gamma")

			raw.items[#raw.items + 1] = { name = "Three", count = 3 }
			text = renderText()
			assertNotContains("raw append without dirty not rendered", text, "Three=3")

			model:__SetDirty("items")
			text = renderText()
			assertContains("manual dirty after raw append rendered", text, "Three=3")
			assertEqual("manual dirty after raw append size", expectElement("count", document:GetElementById("count")).inner_rml, "3")

			assertEqual("root new key rejected", pcall(function()
				model.unbound = true
			end), false)
			assertEqual("root function replacement rejected", pcall(function()
				model.record_click = function() end
			end), false)

			context:RemoveDataModel("native_api_parity_lua_model")
		end)
	end)

	runRmlCheck("lua_rml_stylesheet_append_behavior", function()
		withRmlContext("stylesheet", function(context)
			local document = createDocument(context)
			document:AppendToStyleSheet("body { color: rgb(255, 0, 0); }")
			document:AppendToStyleSheet(".chip { display: block; }")
			document:UpdateDocument()

			local malformedDocument = createDocument(context)
			malformedDocument:AppendToStyleSheet("body { color: ")
		end)
	end)

	runRmlCheck("lua_rml_scrollbar_teardown_behavior", function()
		withRmlContext("scrollbar_teardown", function(context)
			context.dimensions = RmlUi.Vector2i.new(320, 240)

			local document = createDocument(context)
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

			local host = expectElement("scroll host", document:GetElementById("host"))
			assertEqual("scroll host overflows vertically", host.scroll_height > host.client_height, true)
			host.scroll_top = 16
			assertEqual("scroll top changed", host.scroll_top > 0, true)

			document.inner_rml = [[<div id="replacement">replacement</div>]]
			document:UpdateDocument()
			context:Update()

			assertEqual("replacement element exists", document:GetElementById("replacement") ~= nil, true)
		end)
	end)

	runRmlCheck("lua_rml_debug_context_removal_behavior", function()
		local context = RmlUi.CreateContext("native_api_parity_lua_rml_debug_remove")
		if context == nil then
			error("RmlUi.CreateContext(debug_remove) returned nil", 0)
		end
		RmlUi.SetDebugContext(context)
		context:UnloadAllDocuments()
		RmlUi.RemoveContext(context)
	end)
end

function NativeApiParityResult(stream, encodedPayload)
	local path = Common.outputDir() .. "/" .. stream .. ".jsonl"
	local file, err = io.open(path, "a")
	if not file then
		Spring.Echo("native_api_parity: failed to open " .. path .. ": " .. tostring(err))
		return
	end
	file:write(encodedPayload)
	file:write("\n")
	file:close()
end

function NativeApiParityFixture(unitID, featureID, unitDefID, featureDefID, weaponDefID, projectileID, pieceProjectileID, teamID, allyTeamID, groundDecalID)
	fixtureIDs = {
		unitID = unitID,
		featureID = featureID,
		unitDefID = unitDefID,
		featureDefID = featureDefID,
		weaponDefID = weaponDefID,
		projectileID = projectileID,
		pieceProjectileID = pieceProjectileID,
		teamID = teamID,
		allyTeamID = allyTeamID,
		groundDecalID = groundDecalID,
	}
	ranGeneratedTests = false
end

function GameSetup()
	return true, true
end

function Initialize()
	recordInventory()
	record("game_frame_initial", { value = Spring.GetGameFrame() })
	record("my_player", { playerID = Spring.GetMyPlayerID(), teamID = Spring.GetMyTeamID(), allyTeamID = Spring.GetMyAllyTeamID() })
	Spring.SendCommands("setmaxspeed 1000", "setminspeed 1")
end

function GameFrame(frame)
	if frame == 4 then
		recordInventory()
		runRmlUiTests()
		runGeneratedTests()
		record("game_frame", { value = Spring.GetGameFrame() })
		record("visible_units", { count = #(Spring.GetVisibleUnits() or {}) })
	elseif frame == 20 then
		runGeneratedTests()
		local options = Spring.GetModOptions() or {}
		local processTest = tostring(options.native_api_parity_process_test or "")
		if processTest ~= "reload" and processTest ~= "restart" then
			Spring.SendCommands("quitforce")
		end
	end
end
