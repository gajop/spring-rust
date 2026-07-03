local Common = VFS.Include("LuaRules/Utilities/native_api_parity_common.lua")
local GeneratedTests = VFS.Include("LuaRules/Utilities/generated_api_tests.lua")
local outputPath = Common.outputDir() .. "/widget.jsonl"
local sentInventory = false
local ranGeneratedTests = false
local ranRmlUiTests = false
local fixtureIDs = {}

local function record(name, payload)
	payload.context = "widget"
	payload.name = name
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

function NativeApiParityFixture(unitID, featureID, unitDefID, featureDefID, weaponDefID, teamID, allyTeamID)
	fixtureIDs = {
		unitID = unitID,
		featureID = featureID,
		unitDefID = unitDefID,
		featureDefID = featureDefID,
		weaponDefID = weaponDefID,
		teamID = teamID,
		allyTeamID = allyTeamID,
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
		Spring.SendCommands("quitforce")
	end
end
