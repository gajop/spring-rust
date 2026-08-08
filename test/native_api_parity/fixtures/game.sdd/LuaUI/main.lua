local Common = VFS.Include("LuaRules/Utilities/native_api_parity_common.lua")
local GeneratedTests = VFS.Include("LuaRules/Utilities/generated_api_tests.lua")
local outputPath = Common.outputDir() .. "/widget.jsonl"
local sentInventory = false
local ranGeneratedTests = false
local ranRmlUiTests = false
local ranRmlSurfaceApiTest = false
local ranRmlElementSurfaceApiTest = false
local ranGlStateQueries = false
local ranGlStateMutations = false
local ranGlFixedImmediate = false
local ranGlImmediatePrimitives = false
local ranGlShaderUniforms = false
local ranGlTextureResources = false
local ranGlListsQueries = false
local ranGlAtlas = false
local ranGlFbo = false
local ranGlResourceHandles = false
local ranGlUserdata = false
local ranGlFonts = false
local ranGlMiniMap = false
local ranGlObjectDrawing = false
local ranScriptKillTest = false
local fixtureIDs = {}
local renderFixtureIDs

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

local function normalizeRmlSurfaceValue(value)
	local valueType = type(value)
	if valueType == "nil" then
		return { type = "nil" }
	end
	if valueType == "boolean" or valueType == "number" or valueType == "string" then
		return value
	end
	if valueType == "table" then
		return { type = "table", count = tableLength(value) }
	end
	-- RmlUi userdata contains process-local pointers.  The native side records
	-- the same observable category and separately validates that the handle is
	-- live before producing it.
	return { type = valueType }
end

local function rmlSurfaceCall(results, apiName, fn, normalizer)
	local values = { n = 0 }
	local function capture(...)
		values.n = select("#", ...)
		for index = 1, values.n do
			values[index] = select(index, ...)
		end
	end
	local ok, err = pcall(function()
		capture(fn())
	end)
	if not ok then
		error(apiName .. ": " .. tostring(err), 0)
	end
	local normalized = {}
	for index = 1, values.n do
		normalized[index] = normalizer and normalizer(values[index]) or normalizeRmlSurfaceValue(values[index])
	end
	results[apiName] = { n = values.n, values = normalized }
	return unpack(values, 1, values.n)
end

local function runRmlSurfaceApiTest()
	if ranRmlSurfaceApiTest or not Common.enableRenderingTests() or RmlUi == nil then
		return
	end
	ranRmlSurfaceApiTest = true
	local testName = "rml.global_context_document"
	local result = {}
	local contextName = "native_api_parity_surface_global_context_document"

	local context = rmlSurfaceCall(result, "RmlUi.CreateContext", function()
		return RmlUi.CreateContext(contextName)
	end)
	if context == nil then
		error("RmlUi.CreateContext returned nil", 0)
	end
	rmlSurfaceCall(result, "RmlUi.GetContext", function()
		return RmlUi.GetContext(contextName)
	end)
	rmlSurfaceCall(result, "RmlUi.AddTranslationString", function()
		return RmlUi.AddTranslationString("native_api_parity_surface_key", "surface translation")
	end)
	rmlSurfaceCall(result, "RmlUi.ClearTranslations", function()
		return RmlUi.ClearTranslations()
	end)
	rmlSurfaceCall(result, "RmlUi.LoadFontFace", function()
		return RmlUi.LoadFontFace("native_api_parity_missing_font.ttf", false)
	end)
	rmlSurfaceCall(result, "RmlUi.RegiserEventType", function()
		return RmlUi.RegisterEventType("native_api_parity_surface_event", true, true)
	end)
	rmlSurfaceCall(result, "RmlUi.SetMouseCursorAlias", function()
		return RmlUi.SetMouseCursorAlias("native-api-parity-surface", "Arrow")
	end)
	rmlSurfaceCall(result, "RmlUi.SetDebugContext", function()
		return RmlUi.SetDebugContext(context)
	end)
	rmlSurfaceCall(result, "RmlUi.GetDocumentPathRequests", function()
		return RmlUi.GetDocumentPathRequests("native-api-parity-surface.rml")
	end)
	rmlSurfaceCall(result, "RmlUi.ClearDocumentPathRequests", function()
		return RmlUi.ClearDocumentPathRequests("native-api-parity-surface.rml")
	end)
	rmlSurfaceCall(result, "RmlUi.Vector2i.new", function()
		return RmlUi.Vector2i.new(12, 34)
	end, function(value)
		return { type = "vector2i", x = value.x, y = value.y }
	end)
	rmlSurfaceCall(result, "RmlUi.Vector2f.new", function()
		return RmlUi.Vector2f.new(12.5, 34.5)
	end, function(value)
		return { type = "vector2f", x = value.x, y = value.y }
	end)

	rmlSurfaceCall(result, "RmlUi.Context.AddEventListener", function()
		return context:AddEventListener("click", "return true", false)
	end)
	rmlSurfaceCall(result, "RmlUi.Context.EnableMouseCursor", function()
		return context:EnableMouseCursor(false)
	end)
	rmlSurfaceCall(result, "RmlUi.Context.ActivateTheme", function()
		return context:ActivateTheme("native-api-parity-surface-theme", false)
	end)
	rmlSurfaceCall(result, "RmlUi.Context.IsThemeActive", function()
		return context:IsThemeActive("native-api-parity-surface-theme")
	end)
	rmlSurfaceCall(result, "RmlUi.Context.ProcessMouseMove", function()
		return context:ProcessMouseMove(1, 1, 0)
	end)
	rmlSurfaceCall(result, "RmlUi.Context.ProcessMouseButtonDown", function()
		return context:ProcessMouseButtonDown(0, 0)
	end)
	rmlSurfaceCall(result, "RmlUi.Context.ProcessMouseButtonUp", function()
		return context:ProcessMouseButtonUp(0, 0)
	end)
	rmlSurfaceCall(result, "RmlUi.Context.ProcessMouseWheel", function()
		return context:ProcessMouseWheel(RmlUi.Vector2f.new(0, 1), 0)
	end)
	rmlSurfaceCall(result, "RmlUi.Context.ProcessMouseLeave", function()
		return context:ProcessMouseLeave()
	end)
	rmlSurfaceCall(result, "RmlUi.Context.IsMouseInteracting", function()
		return context:IsMouseInteracting()
	end)
	rmlSurfaceCall(result, "RmlUi.Context.ProcessKeyDown", function()
		return context:ProcessKeyDown(65, 0)
	end)
	rmlSurfaceCall(result, "RmlUi.Context.ProcessKeyUp", function()
		return context:ProcessKeyUp(65, 0)
	end)
	rmlSurfaceCall(result, "RmlUi.Context.ProcessTextInput", function()
		return context:ProcessTextInput("x")
	end)

	local document = rmlSurfaceCall(result, "RmlUi.Context.CreateDocument", function()
		return context:CreateDocument()
	end)
	if document == nil then
		error("Context:CreateDocument returned nil", 0)
	end
	document.id = "native-api-parity-surface-document"
	rmlSurfaceCall(result, "RmlUi.Context.GetDocument", function()
		return context:GetDocument("native-api-parity-surface-document")
	end)
	rmlSurfaceCall(result, "RmlUi.Context.LoadDocument", function()
		return context:LoadDocument("native_api_parity_missing_surface_document.rml", {})
	end)
	rmlSurfaceCall(result, "RmlUi.Context.OpenDataModel", function()
		return context:OpenDataModel("native_api_parity_surface_model", { value = "surface" })
	end, function(value)
		return { type = "data_model", fields = tableLength(value) }
	end)
	rmlSurfaceCall(result, "RmlUi.Context.GetElementAtPoint", function()
		return context:GetElementAtPoint(RmlUi.Vector2f.new(1, 1))
	end)
	rmlSurfaceCall(result, "RmlUi.Context.PullDocumentToFront", function()
		return context:PullDocumentToFront(document)
	end)
	rmlSurfaceCall(result, "RmlUi.Context.PushDocumentToBack", function()
		return context:PushDocumentToBack(document)
	end)

	rmlSurfaceCall(result, "RmlUi.Document.AppendToStyleSheet", function()
		return document:AppendToStyleSheet("body { color: rgb(1, 2, 3); }")
	end)
	rmlSurfaceCall(result, "RmlUi.Document.CreateElement", function()
		return document:CreateElement("div")
	end)
	rmlSurfaceCall(result, "RmlUi.Document.CreateTextNode", function()
		return document:CreateTextNode("surface")
	end)
	rmlSurfaceCall(result, "RmlUi.Document.LoadInlineScript", function()
		return document:LoadInlineScript("return true", "native-api-parity-surface", 1)
	end)
	rmlSurfaceCall(result, "RmlUi.Document.LoadExternalScript", function()
		return document:LoadExternalScript("native_api_parity_missing_surface.js")
	end)
	rmlSurfaceCall(result, "RmlUi.Document.ReloadStyleSheet", function()
		return document:ReloadStyleSheet(false)
	end)
	rmlSurfaceCall(result, "RmlUi.Document.Show", function()
		return document:Show()
	end)
	rmlSurfaceCall(result, "RmlUi.Document.Hide", function()
		return document:Hide()
	end)
	rmlSurfaceCall(result, "RmlUi.Document.PullToFront", function()
		return document:PullToFront()
	end)
	rmlSurfaceCall(result, "RmlUi.Document.PushToBack", function()
		return document:PushToBack()
	end)
	rmlSurfaceCall(result, "RmlUi.Document.UpdateDocument", function()
		return document:UpdateDocument()
	end)

	-- Keep the remaining cleanup calls in the same order on both sides.
	local closeDocument = context:CreateDocument()
	if closeDocument == nil then
		error("close document creation failed", 0)
	end
	rmlSurfaceCall(result, "RmlUi.Document.Close", function()
		return closeDocument:Close()
	end)
	local unloadDocument = context:CreateDocument()
	if unloadDocument == nil then
		error("unload document creation failed", 0)
	end
	rmlSurfaceCall(result, "RmlUi.Context.UnloadDocument", function()
		return context:UnloadDocument(unloadDocument)
	end)
	rmlSurfaceCall(result, "RmlUi.Context.RemoveDataModel", function()
		return context:RemoveDataModel("native_api_parity_surface_model")
	end)
	rmlSurfaceCall(result, "RmlUi.Context.Update", function()
		return context:Update()
	end)
	rmlSurfaceCall(result, "RmlUi.Context.Render", function()
		return context:Render()
	end)
	rmlSurfaceCall(result, "RmlUi.Context.UnloadAllDocuments", function()
		return context:UnloadAllDocuments()
	end)
	rmlSurfaceCall(result, "RmlUi.RemoveContext", function()
		return RmlUi.RemoveContext(context)
	end)

	local payload = { status = "pass", result = result, context = "widget" }
	Common.setTestName(payload, testName)
	record(testName, payload)
	if Common.mode() == "native" then
		Spring.InvokeNativeModule(Common.encode(payload))
	end
end

local function roundedGlValue(value)
	if type(value) == "number" then
		if value > 1e30 then
			return "inf"
		elseif value < -1e30 then
			return "-inf"
		elseif value ~= value then
			return "nan"
		end
		return math.floor(value * 100000 + 0.5) / 100000
	end
	return value
end

local function captureGlValues(...)
	local values = { n = select("#", ...) }
	for index = 1, values.n do
		values[index] = roundedGlValue(select(index, ...))
	end
	return values
end

local function glCall(result, name, fn)
	local values = captureGlValues(fn())
	local normalized = {}
	for index = 1, values.n do
		normalized[index] = values[index]
	end
	result[name] = { n = values.n, values = normalized }
end

local function runGlStateSurfaceApiTest()
	if ranGlStateQueries or not Common.enableRenderingTests() then
		return
	end
	ranGlStateQueries = true
	if gl == nil then
		error("gl table is not available in rendering parity test", 0)
	end

	local queryResult = {}
	glCall(queryResult, "gl.HasExtension", function()
		return gl.HasExtension("GL_NATIVE_API_PARITY_NOT_AN_EXTENSION")
	end)
	glCall(queryResult, "gl.GetNumber", function()
		return gl.GetNumber(0x0BA2, 4)
	end)
	glCall(queryResult, "gl.GetString", function()
		return gl.GetString(0x1F02)
	end)
	glCall(queryResult, "gl.GetViewSizes", gl.GetViewSizes)
	glCall(queryResult, "gl.GetViewRange", function()
		return gl.GetViewRange()
	end)
	glCall(queryResult, "gl.GetShadowMapParams", gl.GetShadowMapParams)
	glCall(queryResult, "gl.GetAtmosphere", gl.GetAtmosphere)
	glCall(queryResult, "gl.GetSun", gl.GetSun)
	glCall(queryResult, "gl.GetWaterRendering", function()
		return gl.GetWaterRendering("absorb")
	end)
	glCall(queryResult, "gl.GetMapRendering", function()
		return gl.GetMapRendering("voidWater")
	end)
	glCall(queryResult, "gl.GetScreenViewTrans", gl.GetScreenViewTrans)

	local payload = { status = "pass", result = queryResult, context = "widget" }
	Common.setTestName(payload, "gl.state_queries")
	record("gl.state_queries", payload)
	if Common.mode() == "native" then
		Spring.InvokeNativeModule(Common.encode(payload))
	end
end

local function runGlStateMutationSurfaceApiTest()
	if ranGlStateMutations or not Common.enableRenderingTests() then
		return
	end
	ranGlStateMutations = true
	local result = {}
	local function void(name, fn)
		fn()
		result[name] = { n = 0, values = {} }
	end

	void("gl.ResetState", gl.ResetState)
	void("gl.ResetMatrices", gl.ResetMatrices)
	void("gl.MatrixMode", function() gl.MatrixMode(GL.PROJECTION) end)
	void("gl.LoadIdentity", gl.LoadIdentity)
	void("gl.Translate", function() gl.Translate(1, 2, 3) end)
	void("gl.Scale", function() gl.Scale(2, 3, 4) end)
	void("gl.Rotate", function() gl.Rotate(15, 0, 1, 0) end)
	void("gl.PushMatrix", gl.PushMatrix)
	void("gl.PopMatrix", gl.PopMatrix)
	void("gl.Ortho", function() gl.Ortho(-1, 1, -1, 1, 0.1, 100) end)
	void("gl.Frustum", function() gl.Frustum(-0.1, 0.1, -0.1, 0.1, 0.1, 100) end)
	void("gl.LoadMatrix", function()
		gl.LoadMatrix(1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)
	end)
	void("gl.MultMatrix", function()
		gl.MultMatrix(1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 2, 3, 1)
	end)
	glCall(result, "gl.GetMatrixData", function()
		return gl.GetMatrixData(GL.PROJECTION)
	end)
	void("gl.MatrixMode.restore", function() gl.MatrixMode(GL.MODELVIEW) end)
	void("gl.ResetMatrices.restore", gl.ResetMatrices)
	void("gl.DepthTest", function() gl.DepthTest(GL.LEQUAL) end)
	void("gl.DepthMask", function() gl.DepthMask(false) end)
	void("gl.Culling", function() gl.Culling(true) end)
	void("gl.Blending", function() gl.Blending(true) end)
	void("gl.BlendFunc", function() gl.BlendFunc(GL.SRC_ALPHA, GL.ONE_MINUS_SRC_ALPHA) end)
	void("gl.BlendEquation", function() gl.BlendEquation(GL.FUNC_ADD) end)
	void("gl.ColorMask", function() gl.ColorMask(true, false, true, false) end)
	void("gl.AlphaToCoverage", function() gl.AlphaToCoverage(false, true) end)
	void("gl.StencilTest", function() gl.StencilTest(false) end)
	void("gl.Scissor", function() gl.Scissor(0, 0, 64, 64) end)
	void("gl.Viewport", function() gl.Viewport(0, 0, 64, 64) end)
	void("gl.LineWidth", function() gl.LineWidth(2) end)
	void("gl.PointSize", function() gl.PointSize(3) end)
	void("gl.Fog", function() gl.Fog(false) end)
	void("gl.Lighting", function() gl.Lighting(false) end)
	void("gl.ResetState.restore", gl.ResetState)

	local payload = { status = "pass", result = result, context = "widget" }
	Common.setTestName(payload, "gl.state_mutations")
	record("gl.state_mutations", payload)
	if Common.mode() == "native" then
		Spring.InvokeNativeModule(Common.encode(payload))
	end
end

local function runGlImmediatePrimitivesSurfaceApiTest()
	if ranGlImmediatePrimitives or not Common.enableRenderingTests() then
		return
	end
	ranGlImmediatePrimitives = true
	local result = {}
	local function void(name, fn)
		fn()
		result[name] = { n = 0, values = {} }
	end

	void("gl.Clear", function()
		gl.Clear(GL.COLOR_BUFFER_BIT, 0.1, 0.2, 0.3, 0.4)
	end)
	void("gl.BeginEnd", function()
		gl.BeginEnd(GL.TRIANGLES, function()
			gl.Color(0.2, 0.3, 0.4, 0.5)
			gl.SecondaryColor(0.6, 0.7, 0.8)
			gl.Normal(0, 1, 0)
			gl.TexCoord(0.1, 0.2, 0.3, 0.4)
			gl.MultiTexCoord(1, 0.2, 0.3, 0.4, 0.5)
			gl.FogCoord(0.6)
			gl.EdgeFlag(false)
			gl.Vertex(0, 0, 0, 1)
			gl.Vertex(1, 0, 0, 1)
			gl.Vertex(0, 1, 0, 1)
		end)
	end)
	void("gl.Shape", function()
		gl.Shape(GL.TRIANGLES, {
			{ v = { 0, 0, 0 }, n = { 0, 1, 0 }, t = { 0, 0 }, c = { 1, 0, 0, 1 } },
			{ v = { 1, 0, 0 }, n = { 0, 1, 0 }, t = { 1, 0 }, c = { 0, 1, 0, 1 } },
			{ v = { 0, 1, 0 }, n = { 0, 1, 0 }, t = { 0, 1 }, c = { 0, 0, 1, 1 } },
		})
	end)
	void("gl.Rect", function() gl.Rect(-1, -1, 1, 1) end)
	void("gl.TexRect", function() gl.TexRect(-1, -1, 1, 1, 0.1, 0.2, 0.9, 0.8) end)
	void("gl.Billboard", gl.Billboard)
	void("gl.PushPopMatrix", function()
		gl.PushPopMatrix(GL.MODELVIEW, function()
			gl.Translate(1, 2, 3)
		end)
	end)
	void("gl.UnsafeState", function()
		gl.UnsafeState(GL.BLEND, true, function() end)
	end)
	void("gl.Flush", gl.Flush)
	void("gl.Finish", gl.Finish)

	-- Read back the mutable current attributes after the immediate-mode calls;
	-- this makes the comparison validate GL state rather than only void arity.
	gl.Color(0.11, 0.22, 0.33, 0.44)
	gl.Normal(0.55, 0.66, 0.77)
	gl.TexCoord(0.12, 0.23, 0.34, 0.45)
	gl.SecondaryColor(0.56, 0.67, 0.78)
	gl.FogCoord(0.89)
	gl.EdgeFlag(false)
	glCall(result, "gl.GetNumber.currentColor", function()
		return gl.GetNumber(0x0B00, 4)
	end)
	glCall(result, "gl.GetNumber.currentNormal", function()
		return gl.GetNumber(0x0B02, 3)
	end)
	glCall(result, "gl.GetNumber.currentTexCoord", function()
		return gl.GetNumber(0x0B03, 4)
	end)
	glCall(result, "gl.GetNumber.currentSecondaryColor", function()
		return gl.GetNumber(0x8459, 4)
	end)
	glCall(result, "gl.GetNumber.currentFogCoord", function()
		return gl.GetNumber(0x8453, 1)
	end)
	glCall(result, "gl.GetNumber.edgeFlag", function()
		return gl.GetNumber(0x0B43, 1)
	end)

	void("gl.ResetState.restore", gl.ResetState)
	local payload = { status = "pass", result = result, context = "widget" }
	Common.setTestName(payload, "gl.immediate_primitives")
	record("gl.immediate_primitives", payload)
	if Common.mode() == "native" then
		Spring.InvokeNativeModule(Common.encode(payload))
	end
end

local function runGlShaderUniformSurfaceApiTest()
	if ranGlShaderUniforms or not Common.enableRenderingTests() then
		return
	end
	ranGlShaderUniforms = true
	local result = {}
	local function void(name, fn)
		fn()
		result[name] = { n = 0, values = {} }
	end

	local vertexShader = [[
		#version 120
		uniform float u_scalar;
		uniform float u_floatArray[2];
		uniform vec2 u_vector;
		uniform int u_int;
		uniform int u_intArray[2];
		uniform mat4 u_matrix;
		void main() {
			float offset = u_scalar + u_vector.x + float(u_int) +
				u_floatArray[0] + float(u_intArray[0]) + u_matrix[0][0];
			gl_Position = gl_Vertex + vec4(offset * 0.0001);
		}
	]]
	local fragmentShader = [[
		#version 120
		uniform float u_scalar;
		uniform float u_floatArray[2];
		uniform vec2 u_vector;
		uniform int u_int;
		uniform int u_intArray[2];
		uniform mat4 u_matrix;
		uniform vec4 u_color;
		void main() {
			float offset = u_scalar + u_vector.y + float(u_int) +
				u_floatArray[1] + float(u_intArray[1]) + u_matrix[1][1];
			gl_FragColor = u_color + vec4(offset * 0.0001);
		}
	]]

	local shaderID, rawShaderID = gl.CreateShader({
		vertex = vertexShader,
		fragment = fragmentShader,
	})
	if shaderID == nil or rawShaderID == nil or shaderID <= 0 or rawShaderID <= 0 then
		error("gl.CreateShader did not return valid handles", 0)
	end
	-- Shader and GL program handles are process-local; compare validity, not IDs.
	result["gl.CreateShader"] = { n = 2, values = { true, true } }

	glCall(result, "gl.GetShaderLog", gl.GetShaderLog)
	glCall(result, "gl.UseShader", function()
		return gl.UseShader(shaderID)
	end)
	glCall(result, "gl.GetNumber.currentProgram", function()
		local program = gl.GetNumber(0x8B8D, 1)
		return program > 0
	end)

	local locations = {}
	for _, name in ipairs({ "u_scalar", "u_floatArray", "u_vector", "u_int", "u_intArray", "u_matrix", "u_color" }) do
		local location = gl.GetUniformLocation(shaderID, name)
		locations[name] = location
		result["gl.GetUniformLocation." .. name] = {
			n = 1,
			values = { location },
		}
	end

	local activeUniforms = gl.GetActiveUniforms(shaderID)
	local normalizedUniforms = {}
	for _, uniform in ipairs(activeUniforms or {}) do
		normalizedUniforms[#normalizedUniforms + 1] = {
			uniform.name,
			uniform.type,
			uniform.length,
			uniform.size,
			uniform.location,
		}
	end
	table.sort(normalizedUniforms, function(left, right)
		return left[1] < right[1]
	end)
	result["gl.GetActiveUniforms"] = { n = 1, values = { normalizedUniforms } }

	void("gl.Uniform", function()
		gl.Uniform(locations.u_scalar, 1.25)
		gl.Uniform("u_vector", 2.0, 3.0)
		gl.Uniform("u_color", 0.1, 0.2, 0.3, 0.4)
	end)
	void("gl.UniformInt", function()
		gl.UniformInt("u_int", 7)
	end)
	void("gl.UniformArray", function()
		gl.UniformArray("u_floatArray", 2, { 1.5, 2.5 })
		gl.UniformArray("u_intArray", 1, { 3, 4 })
	end)
	void("gl.UniformMatrix", function()
		gl.UniformMatrix("u_matrix",
			1, 0, 0, 0,
			0, 1, 0, 0,
			0, 0, 1, 0,
			0, 0, 0, 1)
	end)
	void("gl.ActiveShader", function()
		gl.ActiveShader(shaderID, function()
			gl.Uniform("u_scalar", 1.5)
		end)
	end)
	void("gl.SetGeometryShaderParameter", function()
		gl.SetGeometryShaderParameter(shaderID, 0x8DDC, GL.TRIANGLES)
	end)
	void("gl.SetTesselationShaderParameter", function()
		gl.SetTesselationShaderParameter(0x8E72, 3)
	end)

	glCall(result, "gl.GetEngineUniformBufferDef", function()
		return gl.GetEngineUniformBufferDef(0)
	end)
	glCall(result, "gl.GetEngineModelUniformDataDef", gl.GetEngineModelUniformDataDef)
	glCall(result, "gl.GetEngineModelUniformDataSize", gl.GetEngineModelUniformDataSize)

	glCall(result, "gl.UseShader.restore", function()
		return gl.UseShader(0)
	end)
	local deleted = gl.DeleteShader(shaderID)
	result["gl.DeleteShader"] = { n = 1, values = { deleted } }
	glCall(result, "gl.UseShader.invalid", function()
		return gl.UseShader(shaderID)
	end)

	local payload = { status = "pass", result = result, context = "widget" }
	Common.setTestName(payload, "gl.shader_uniforms")
	record("gl.shader_uniforms", payload)
	if Common.mode() == "native" then
		Spring.InvokeNativeModule(Common.encode(payload))
	end
end

local function runGlTextureResourceSurfaceApiTest()
	if ranGlTextureResources or not Common.enableRenderingTests() then
		return
	end
	ranGlTextureResources = true
	local result = {}
	local function void(name, fn)
		fn()
		result[name] = { n = 0, values = {} }
	end

	local textureParams = {
		format = GL.RGBA8,
		min_filter = GL.LINEAR,
		mag_filter = GL.LINEAR,
		wrap_s = GL.CLAMP_TO_EDGE,
		wrap_t = GL.CLAMP_TO_EDGE,
		wrap_r = GL.CLAMP_TO_EDGE,
	}
	local textureName = gl.CreateTexture(4, 4, textureParams)
	if type(textureName) ~= "string" or textureName == "" then
		error("gl.CreateTexture did not return a texture name", 0)
	end
	result["gl.CreateTexture"] = { n = 1, values = { true } }

	local textureInfo = gl.TextureInfo(textureName)
	if type(textureInfo) ~= "table" or textureInfo.xsize ~= 4 or textureInfo.ysize ~= 4 or textureInfo.zsize ~= 0 or textureInfo.id <= 0 then
		error("gl.TextureInfo returned unexpected texture metadata", 0)
	end
	result["gl.TextureInfo"] = {
		n = 1,
		values = {{
			xsize = textureInfo.xsize,
			ysize = textureInfo.ysize,
			zsize = textureInfo.zsize,
			target = textureInfo.target,
		}},
	}
	result["gl.TextureInfo.idValid"] = { n = 1, values = { textureInfo.id > 0 } }

	glCall(result, "gl.Texture", function()
		return gl.Texture(textureName)
	end)
	void("gl.ChangeTextureParams", function()
		gl.ChangeTextureParams(textureName, {
			min_filter = GL.NEAREST,
			mag_filter = GL.NEAREST,
			wrap_s = GL.CLAMP_TO_EDGE,
			wrap_t = GL.CLAMP_TO_EDGE,
			wrap_r = GL.CLAMP_TO_EDGE,
		})
	end)
	glCall(result, "gl.GetNumber.textureBinding", function()
		return gl.GetNumber(0x8069, 1) > 0
	end)
	glCall(result, "gl.Texture.disable", function()
		return gl.Texture(false)
	end)
	void("gl.CopyToTexture", function()
		gl.CopyToTexture(textureName, 0, 0, 0, 0, 1, 1, GL.TEXTURE_2D, 0)
	end)
	void("gl.GenerateMipmap", function()
		gl.GenerateMipmap(textureName)
	end)
	void("gl.BindImageTexture", function()
		gl.BindImageTexture(0, textureName, 0, 0, GL.READ_WRITE, GL.RGBA8)
	end)

	void("gl.Clear", function()
		gl.Clear(GL.COLOR_BUFFER_BIT, 0.21, 0.31, 0.41, 0.51)
	end)
	glCall(result, "gl.ReadPixels", function()
		return gl.ReadPixels(0, 0, 1, 1, GL.RGBA)
	end)
	glCall(result, "gl.SaveImage", function()
		return gl.SaveImage(0, 0, 1, 1, "native_api_parity_texture.png", {
			alpha = false,
			yflip = false,
			grayscale16bit = false,
		})
	end)

	local fboTextureName = gl.CreateTexture(4, 4, {
		format = GL.RGBA8,
		min_filter = GL.LINEAR,
		mag_filter = GL.LINEAR,
		wrap_s = GL.CLAMP_TO_EDGE,
		wrap_t = GL.CLAMP_TO_EDGE,
		fbo = true,
		fboDepth = true,
	})
	if type(fboTextureName) ~= "string" or fboTextureName == "" then
		error("gl.CreateTexture(fbo) did not return a texture name", 0)
	end
	result["gl.CreateTexture.fbo"] = { n = 1, values = { true } }
	void("gl.RenderToTexture", function()
		gl.RenderToTexture(fboTextureName, function()
			gl.Clear(GL.COLOR_BUFFER_BIT, 0.21, 0.31, 0.41, 0.51)
		end)
	end)
	glCall(result, "gl.DeleteTextureFBO", function()
		return gl.DeleteTextureFBO(fboTextureName)
	end)
	glCall(result, "gl.DeleteTexture.fbo", function()
		return gl.DeleteTexture(fboTextureName)
	end)
	glCall(result, "gl.DeleteTexture", function()
		return gl.DeleteTexture(textureName)
	end)

	local payload = { status = "pass", result = result, context = "widget" }
	Common.setTestName(payload, "gl.texture_resources")
	record("gl.texture_resources", payload)
	if Common.mode() == "native" then
		Spring.InvokeNativeModule(Common.encode(payload))
	end
end

local function runGlListsQuerySurfaceApiTest()
	if ranGlListsQueries or not Common.enableRenderingTests() then
		return
	end
	ranGlListsQueries = true
	local result = {}
	local function void(name, fn)
		fn()
		result[name] = { n = 0, values = {} }
	end

	void("gl.ResetState", gl.ResetState)
	local listIndex = gl.CreateList(function()
		gl.BeginEnd(GL.TRIANGLES, function()
			gl.Color(0.17, 0.27, 0.37, 0.47)
			gl.Vertex(0, 0, 0)
			gl.Vertex(1, 0, 0)
			gl.Vertex(0, 1, 0)
		end)
	end)
	if type(listIndex) ~= "number" or listIndex <= 0 then
		error("gl.CreateList did not return a valid list index", 0)
	end
	result["gl.CreateList"] = { n = 1, values = { true } }
	void("gl.CallList", function()
		gl.CallList(listIndex)
	end)
	void("gl.DeleteList", function()
		gl.DeleteList(listIndex)
	end)

	local query = gl.CreateQuery()
	if query == nil then
		error("gl.CreateQuery did not return a query", 0)
	end
	void("gl.RunQuery", function()
		gl.RunQuery(query, function()
			gl.BeginEnd(GL.TRIANGLES, function()
				gl.Vertex(0, 0, 0)
				gl.Vertex(1, 0, 0)
				gl.Vertex(0, 1, 0)
			end)
		end)
	end)
	glCall(result, "gl.GetQuery", function()
		return gl.GetQuery(query)
	end)
	void("gl.DeleteQuery", function()
		gl.DeleteQuery(query)
	end)

	local payload = { status = "pass", result = result, context = "widget" }
	Common.setTestName(payload, "gl.lists_queries")
	record("gl.lists_queries", payload)
	if Common.mode() == "native" then
		Spring.InvokeNativeModule(Common.encode(payload))
	end
end

local function normalizeAtlasEntries(textures)
	local names = {}
	for name in pairs(textures or {}) do
		names[#names + 1] = name
	end
	table.sort(names)
	local entries = {}
	for _, name in ipairs(names) do
		local coords = textures[name]
		entries[#entries + 1] = {
			name,
			roundedGlValue(coords[1]),
			roundedGlValue(coords[2]),
			roundedGlValue(coords[3]),
			roundedGlValue(coords[4]),
		}
	end
	return entries
end

local function runGlAtlasSurfaceApiTest()
	if ranGlAtlas or not Common.enableRenderingTests() then
		return
	end
	ranGlAtlas = true
	local result = {}
	local function void(name, fn)
		fn()
		result[name] = { n = 0, values = {} }
	end

	void("gl.ResetState", gl.ResetState)
	local textureName = gl.CreateTexture(4, 4, {
		format = GL.RGBA8,
		min_filter = GL.LINEAR,
		mag_filter = GL.LINEAR,
		wrap_s = GL.CLAMP_TO_EDGE,
		wrap_t = GL.CLAMP_TO_EDGE,
	})
	if type(textureName) ~= "string" or textureName == "" then
		error("atlas source texture was not created", 0)
	end
	local atlasName = gl.CreateTextureAtlas(256, 256, 0)
	if type(atlasName) ~= "string" or atlasName == "" then
		error("gl.CreateTextureAtlas did not return an atlas name", 0)
	end
	result["gl.CreateTexture"] = { n = 1, values = { true } }
	result["gl.CreateTextureAtlas"] = { n = 1, values = { true } }
	void("gl.AddAtlasTexture", function()
		-- With no third argument Lua uses the source texture name as the
		-- sub-atlas name, matching the native two-string surface exactly.
		gl.AddAtlasTexture(atlasName, textureName)
	end)
	glCall(result, "gl.FinalizeTextureAtlas", function()
		return gl.FinalizeTextureAtlas(atlasName)
	end)
	glCall(result, "gl.GetAtlasTexture", function()
		return gl.GetAtlasTexture(atlasName, textureName)
	end)
	glCall(result, "gl.DeleteTextureAtlas", function()
		return gl.DeleteTextureAtlas(atlasName)
	end)
	glCall(result, "gl.DeleteTexture", function()
		return gl.DeleteTexture(textureName)
	end)

	glCall(result, "gl.GetEngineAtlasTextures", function()
		return normalizeAtlasEntries(gl.GetEngineAtlasTextures("$explosions"))
	end)
	local globalNames = gl.GetGlobalTexNames() or {}
	table.sort(globalNames)
	result["gl.GetGlobalTexNames"] = { n = 1, values = { globalNames } }
	if #globalNames > 0 then
		glCall(result, "gl.GetGlobalTexCoords", function()
			return gl.GetGlobalTexCoords(globalNames[1])
		end)
	else
		result["gl.GetGlobalTexCoords"] = { n = 0, values = {} }
	end

	local payload = { status = "pass", result = result, context = "widget" }
	Common.setTestName(payload, "gl.atlas")
	record("gl.atlas", payload)
	if Common.mode() == "native" then
		Spring.InvokeNativeModule(Common.encode(payload))
	end
end

local function runGlFboSurfaceApiTest()
	if ranGlFbo or not Common.enableRenderingTests() then
		return
	end
	ranGlFbo = true
	local result = {}
	local function void(name, fn)
		fn()
		result[name] = { n = 0, values = {} }
	end

	void("gl.ResetState", gl.ResetState)
	local sourceTexture = gl.CreateTexture(4, 4, {
		format = GL.RGBA8,
		min_filter = GL.LINEAR,
		mag_filter = GL.LINEAR,
		wrap_s = GL.CLAMP_TO_EDGE,
		wrap_t = GL.CLAMP_TO_EDGE,
	})
	local destinationTexture = gl.CreateTexture(4, 4, {
		format = GL.RGBA8,
		min_filter = GL.LINEAR,
		mag_filter = GL.LINEAR,
		wrap_s = GL.CLAMP_TO_EDGE,
		wrap_t = GL.CLAMP_TO_EDGE,
	})
	if type(sourceTexture) ~= "string" or type(destinationTexture) ~= "string" then
		error("FBO source textures were not created", 0)
	end
	local depthRbo = gl.CreateRBO(4, 4, { format = GL.DEPTH_COMPONENT24 })
	if depthRbo == nil or not depthRbo.valid then
		error("gl.CreateRBO did not return a valid RBO", 0)
	end
	result["gl.CreateTexture"] = { n = 1, values = { true } }
	result["gl.CreateRBO"] = { n = 1, values = { true } }
	result["RBO.valid"] = { n = 1, values = { depthRbo.valid } }
	result["RBO.target"] = { n = 1, values = { depthRbo.target } }
	result["RBO.format"] = { n = 1, values = { depthRbo.format } }
	result["RBO.xsize"] = { n = 1, values = { depthRbo.xsize } }
	result["RBO.ysize"] = { n = 1, values = { depthRbo.ysize } }
	result["RBO.samples"] = { n = 1, values = { depthRbo.samples } }

	local sourceFbo = gl.CreateFBO({
		color0 = sourceTexture,
		depth = depthRbo,
		drawbuffers = { GL.COLOR_ATTACHMENT0 },
	})
	local destinationFbo = gl.CreateFBO({
		color0 = destinationTexture,
		drawbuffers = { GL.COLOR_ATTACHMENT0 },
	})
	if sourceFbo == nil or destinationFbo == nil then
		error("gl.CreateFBO did not return FBOs", 0)
	end
	result["gl.CreateFBO"] = { n = 1, values = { true } }
	local sourceValid, sourceStatus = gl.IsValidFBO(sourceFbo)
	local destinationValid, destinationStatus = gl.IsValidFBO(destinationFbo)
	result["gl.IsValidFBO"] = {
		n = 4,
		values = { sourceValid, sourceStatus, destinationValid, destinationStatus },
	}
	destinationFbo.color0 = sourceTexture
	destinationFbo.drawbuffers = { GL.COLOR_ATTACHMENT0 }
	destinationFbo.readbuffer = GL.COLOR_ATTACHMENT0
	local dynamicValid, dynamicStatus = gl.IsValidFBO(destinationFbo)
	result["FBO.dynamic attachment keys"] = {
		n = 2,
		values = { dynamicValid, dynamicStatus },
	}

	local previousFbo = gl.RawBindFBO(sourceFbo)
	result["gl.RawBindFBO"] = { n = 1, values = { previousFbo } }
	void("gl.RawBindFBO.restore", function()
		gl.RawBindFBO(nil)
	end)

	local cleared
	void("gl.ActiveFBO", function()
		gl.ActiveFBO(sourceFbo, true, function()
			gl.Clear(GL.COLOR_BUFFER_BIT, 0.17, 0.27, 0.37, 0.47)
			cleared = gl.ClearAttachmentFBO(GL.FRAMEBUFFER, "color0", 0.21, 0.31, 0.41, 0.51)
		end)
	end)
	result["gl.ClearAttachmentFBO"] = { n = 1, values = { cleared } }
	void("gl.BlitFBO", function()
		gl.BlitFBO(
			sourceFbo, 0, 0, 4, 4,
			destinationFbo, 0, 0, 4, 4,
			GL.COLOR_BUFFER_BIT, GL.NEAREST
		)
	end)

	void("gl.DeleteFBO", function()
		gl.DeleteFBO(sourceFbo)
		gl.DeleteFBO(destinationFbo)
	end)
	void("gl.DeleteRBO", function()
		gl.DeleteRBO(depthRbo)
	end)
	void("gl.DeleteTexture", function()
		gl.DeleteTexture(sourceTexture)
		gl.DeleteTexture(destinationTexture)
	end)

	local payload = { status = "pass", result = result, context = "widget" }
	Common.setTestName(payload, "gl.fbo")
	record("gl.fbo", payload)
	if Common.mode() == "native" then
		Spring.InvokeNativeModule(Common.encode(payload))
	end
end

local function runGlResourceHandleSurfaceApiTest()
	if ranGlResourceHandles or not Common.enableRenderingTests() then
		return
	end
	ranGlResourceHandles = true

	local result = {}
	local vao = gl.GetVAO()
	result["gl.GetVAO"] = { n = 1, values = { vao ~= nil } }
	if vao == nil then
		error("gl.GetVAO did not return a VAO", 0)
	end

	local vbo = gl.GetVBO(GL.ARRAY_BUFFER, false)
	result["gl.GetVBO"] = { n = 2, values = { vbo ~= nil, GL.ARRAY_BUFFER } }
	if vbo == nil then
		error("gl.GetVBO did not return a VBO", 0)
	end

	local function void(name, fn)
		fn()
		result[name] = { n = 0, values = {} }
	end
	void("gl.GetVAO.Delete", function() vao:Delete() end)
	void("gl.GetVBO.Delete", function() vbo:Delete() end)
	void("gl.SwapBuffers", gl.SwapBuffers)

	local payload = { status = "pass", result = result, context = "widget" }
	Common.setTestName(payload, "gl.resource_handles")
	record("gl.resource_handles", payload)
	if Common.mode() == "native" then
		Spring.InvokeNativeModule(Common.encode(payload))
	end
end

local function runGlUserdataSurfaceApiTest()
	if ranGlUserdata or not Common.enableRenderingTests() then
		return
	end
	local ids = renderFixtureIDs
	if ids == nil or ids.unitID == nil or ids.featureID == nil or ids.unitDefID == nil or ids.featureDefID == nil then
		return
	end
	-- Engine transform/upload tables are populated after the first rendered
	-- world update.  Run before the unsynced generated tests mutate the live
	-- fixture, so the instance-data methods exercise the same live objects as
	-- the later object-drawing test.
	if Spring.GetGameFrame() < 2 then
		return
	end
	-- The instance-data APIs intentionally address objects owned by the
	-- renderer's model-drawer lists.  This fixture is created after renderer
	-- initialization; verify that the IDs delivered from synced Lua are in
	-- those lists before invoking the userdata methods.
	local function contains(values, wanted)
		for _, value in ipairs(values or {}) do
			if value == wanted then
				return true
			end
		end
		return false
	end
	if not contains(Spring.GetRenderUnits(0xFF, false), ids.unitID)
		or not contains(Spring.GetRenderFeatures(0xFF, false), ids.featureID)
	then
		error("gl.userdata fixture is not renderer-managed", 0)
	end
	ranGlUserdata = true

	local result = {}
	local function void(name, fn)
		fn()
		result[name] = { n = 0, values = {} }
	end
	local function values(name, fn)
		glCall(result, name, fn)
	end

	-- Exercise both table-defined and element-array VBO layouts.  The data
	-- paths below deliberately include an attribute-only upload and a
	-- non-default source range so the native implementation cannot merely
	-- validate handles and ignore the interleaved layout.
	local vertexVbo = gl.GetVBO(GL.ARRAY_BUFFER, false)
	local instanceVbo = gl.GetVBO(GL.ARRAY_BUFFER, false)
	local copyVbo = gl.GetVBO(GL.ARRAY_BUFFER, false)
	local indexVbo = gl.GetVBO(GL.ELEMENT_ARRAY_BUFFER, false)
	local uniformVbo = gl.GetVBO(GL.UNIFORM_BUFFER, false)
	local engineInstanceVbo = gl.GetVBO(GL.ARRAY_BUFFER, false)
	local matrixVbo = gl.GetVBO(GL.UNIFORM_BUFFER, false)
	if vertexVbo == nil or instanceVbo == nil or copyVbo == nil or indexVbo == nil or uniformVbo == nil or engineInstanceVbo == nil or matrixVbo == nil then
		error("gl.GetVBO did not return all userdata handles", 0)
	end

	vertexVbo:Define(3, {
		{ id = 0, type = GL.FLOAT, size = 2 },
		{ id = 1, type = GL.FLOAT, size = 4 },
	})
	copyVbo:Define(3, {
		{ id = 0, type = GL.FLOAT, size = 2 },
		{ id = 1, type = GL.FLOAT, size = 4 },
	})
	instanceVbo:Define(2, { { id = 2, type = GL.FLOAT, size = 4 } })
	indexVbo:Define(3, GL.UNSIGNED_INT)
	uniformVbo:Define(2, 2)
	engineInstanceVbo:Define(1, { { id = 0, type = GL.UNSIGNED_INT, size = 4 } })
	matrixVbo:Define(1, { { id = 0, type = GL.FLOAT_MAT4, size = 1 } })
	result["VBO.Define"] = { n = 0, values = {} }

	values("VBO.GetBufferSize", function()
		return vertexVbo:GetBufferSize()
	end)
	values("VBO.GetID", function()
		return vertexVbo:GetID() > 0
	end)
	local fullData = {
		1, 2, 3, 4, 5, 6,
		7, 8, 9, 10, 11, 12,
		13, 14, 15, 16, 17, 18,
	}
	values("VBO.Upload", function()
		return vertexVbo:Upload(fullData)
	end)
	values("VBO.Upload.attribute", function()
		return vertexVbo:Upload({ 99, 20, 21, 22, 23, 88 }, 1, 1, 2, 5)
	end)
	values("VBO.Download", function()
		return vertexVbo:Download(-1, 0, 3, false)
	end)
	values("VBO.Download.attribute", function()
		return vertexVbo:Download(1, 1, 1, false)
	end)

	values("VBO.CopyTo", function()
		local _, _, gpuSize = vertexVbo:GetBufferSize()
		return vertexVbo:CopyTo(copyVbo, gpuSize)
	end)
	values("VBO.CopyTo.Download", function()
		return copyVbo:Download(-1, 0, 3, true)
	end)
	void("VBO.DumpDefinition", function() vertexVbo:DumpDefinition() end)
	void("VBO.Clear", function() vertexVbo:Clear() end)
	values("VBO.Clear.Download", function()
		return vertexVbo:Download(-1, 0, 1, false)
	end)

	values("VBO.BindBufferRange", function()
		return uniformVbo:BindBufferRange(6, 0, 1, GL.UNIFORM_BUFFER)
	end)
	values("VBO.UnbindBufferRange", function()
		return uniformVbo:UnbindBufferRange(6, 0, 1, GL.UNIFORM_BUFFER)
	end)

	local modelVertexVbo = gl.GetVBO(GL.ARRAY_BUFFER, false)
	local modelIndexVbo = gl.GetVBO(GL.ELEMENT_ARRAY_BUFFER, false)
	if modelVertexVbo == nil or modelIndexVbo == nil then
		error("gl.GetVBO did not return model VBO handles", 0)
	end
	values("VBO.ModelsVBO", function()
		return modelVertexVbo:ModelsVBO(), modelIndexVbo:ModelsVBO()
	end)

	values("VBO.InstanceDataFromUnitDefIDs", function()
		local single = engineInstanceVbo:InstanceDataFromUnitDefIDs(ids.unitDefID, 0, ids.teamID, 0)
		local tableValue = engineInstanceVbo:InstanceDataFromUnitDefIDs({ ids.unitDefID }, 0, ids.teamID, 0)
		return single, tableValue
	end)
	values("VBO.InstanceDataFromUnitDefIDs.Download", function()
		return engineInstanceVbo:Download(0, 0, 1, false)
	end)
	values("VBO.InstanceDataFromFeatureDefIDs", function()
		local single = engineInstanceVbo:InstanceDataFromFeatureDefIDs(ids.featureDefID, 0, ids.teamID, 0)
		local tableValue = engineInstanceVbo:InstanceDataFromFeatureDefIDs({ ids.featureDefID }, 0, ids.teamID, 0)
		return single, tableValue
	end)
	values("VBO.InstanceDataFromFeatureDefIDs.Download", function()
		return engineInstanceVbo:Download(0, 0, 1, false)
	end)
	values("VBO.InstanceDataFromUnitIDs", function()
		local single = engineInstanceVbo:InstanceDataFromUnitIDs(ids.unitID, 0, 0)
		local tableValue = engineInstanceVbo:InstanceDataFromUnitIDs({ ids.unitID }, 0, 0)
		return single, tableValue
	end)
	values("VBO.InstanceDataFromUnitIDs.Download", function()
		return engineInstanceVbo:Download(0, 0, 1, false)
	end)
	values("VBO.InstanceDataFromFeatureIDs", function()
		local single = engineInstanceVbo:InstanceDataFromFeatureIDs(ids.featureID, 0, 0)
		local tableValue = engineInstanceVbo:InstanceDataFromFeatureIDs({ ids.featureID }, 0, 0)
		return single, tableValue
	end)
	values("VBO.InstanceDataFromFeatureIDs.Download", function()
		return engineInstanceVbo:Download(0, 0, 1, false)
	end)
	values("VBO.InstanceData.Download", function()
		return engineInstanceVbo:Download(0, 0, 1, false)
	end)

	if ids.projectileID ~= nil then
		values("VBO.MatrixDataFromProjectileIDs", function()
			local single = matrixVbo:MatrixDataFromProjectileIDs(ids.projectileID, 0, 0)
			local tableValue = matrixVbo:MatrixDataFromProjectileIDs({ ids.projectileID }, 0, 0)
			return single, tableValue
		end)
		values("VBO.MatrixData.Download", function()
			return matrixVbo:Download(-1, 0, 1, false)
		end)
	end

	local drawVao = gl.GetVAO()
	local submissionVao = gl.GetVAO()
	if drawVao == nil or submissionVao == nil then
		error("gl.GetVAO did not return all userdata handles", 0)
	end
	void("VAO.AttachVertexBuffer", function() drawVao:AttachVertexBuffer(vertexVbo) end)
	void("VAO.AttachInstanceBuffer", function() drawVao:AttachInstanceBuffer(instanceVbo) end)
	void("VAO.AttachIndexBuffer", function() drawVao:AttachIndexBuffer(indexVbo) end)
	void("VAO.DrawArrays", function() drawVao:DrawArrays(GL.TRIANGLES, 3, 0, 0, 0) end)
	void("VAO.DrawElements", function() drawVao:DrawElements(GL.TRIANGLES, 3, 0, 0, 0, 0) end)

	submissionVao:AttachVertexBuffer(modelVertexVbo)
	submissionVao:AttachIndexBuffer(modelIndexVbo)
	void("VAO.ClearSubmission", function() submissionVao:ClearSubmission() end)
	values("VAO.AddUnitsToSubmission", function()
		return submissionVao:AddUnitsToSubmission(ids.unitID)
	end)
	values("VAO.AddFeaturesToSubmission", function()
		return submissionVao:AddFeaturesToSubmission({ ids.featureID })
	end)
	values("VAO.AddUnitDefsToSubmission", function()
		return submissionVao:AddUnitDefsToSubmission(ids.unitDefID)
	end)
	values("VAO.AddFeatureDefsToSubmission", function()
		return submissionVao:AddFeatureDefsToSubmission({ ids.featureDefID })
	end)
	void("VAO.RemoveFromSubmission", function() submissionVao:RemoveFromSubmission(1) end)
	void("VAO.Submit", function() submissionVao:Submit() end)
	submissionVao:ClearSubmission()

	void("VAO.Delete", function()
		drawVao:Delete()
		submissionVao:Delete()
	end)
	void("VBO.Delete", function()
		vertexVbo:Delete()
		instanceVbo:Delete()
		copyVbo:Delete()
		indexVbo:Delete()
		uniformVbo:Delete()
		engineInstanceVbo:Delete()
		matrixVbo:Delete()
		modelVertexVbo:Delete()
		modelIndexVbo:Delete()
	end)

	local payload = {
		status = "pass",
		result = result,
		context = "widget",
		fixture = {
			unitID = ids.unitID,
			featureID = ids.featureID,
			unitDefID = ids.unitDefID,
			featureDefID = ids.featureDefID,
			projectileID = ids.projectileID,
			teamID = ids.teamID,
		},
		userdataFixture = {
			unitID = ids.unitID,
			featureID = ids.featureID,
			unitDefID = ids.unitDefID,
			featureDefID = ids.featureDefID,
			projectileID = ids.projectileID,
			teamID = ids.teamID,
		},
	}
	Common.setTestName(payload, "gl.userdata")
	record("gl.userdata", payload)
	if Common.mode() == "native" then
		Spring.InvokeNativeModule(Common.encode(payload))
	end
end

local function runGlFontSurfaceApiTest()
	if ranGlFonts or not Common.enableRenderingTests() then
		return
	end
	ranGlFonts = true
	local result = {}
	local function void(name, fn)
		fn()
		result[name] = { n = 0, values = {} }
	end

	void("gl.ResetState", gl.ResetState)
	void("gl.Color", function()
		gl.Color(0.31, 0.41, 0.51, 0.61)
	end)
	void("gl.BeginText", gl.BeginText)
	void("gl.Text", function()
		gl.Text("Native parity", 8, 8, 12, "")
	end)
	void("gl.EndText", gl.EndText)
	glCall(result, "gl.GetTextWidth", function()
		return gl.GetTextWidth("Native parity")
	end)
	glCall(result, "gl.GetTextHeight", function()
		return gl.GetTextHeight("Native parity")
	end)

	local font = gl.LoadFont("fonts/FreeSansBold.otf", 12, 2, 15)
	if font == nil then
		error("gl.LoadFont did not return a font", 0)
	end
	result["gl.LoadFont"] = { n = 1, values = { true } }
	glCall(result, "gl.LoadFont.info", function()
		return font.path, font.family, font.style, font.size, font.height,
			font.lineheight,
			font.descender, font.outlinewidth, font.outlineweight,
			font.texturewidth, font.textureheight
	end)
	glCall(result, "LuaFont.GetTextWidth", function()
		return font:GetTextWidth("Native parity")
	end)
	glCall(result, "LuaFont.GetTextHeight", function()
		return font:GetTextHeight("Native parity")
	end)
	glCall(result, "LuaFont.WrapText", function()
		return font:WrapText("one two three four", 20, 100, 12)
	end)
	void("LuaFont.SetTextColor", function()
		font:SetTextColor({ 0.11, 0.22, 0.33, 0.44 })
	end)
	void("LuaFont.SetOutlineColor", function()
		font:SetOutlineColor(0.55, 0.66, 0.77, 0.88)
	end)
	void("LuaFont.SetAutoOutlineColor", function()
		font:SetAutoOutlineColor(true)
	end)
	void("LuaFont.Begin", function()
		font:Begin(false)
	end)
	void("LuaFont.Print", function()
		font:Print("Native parity", 8, 8, 12, "")
	end)
	void("LuaFont.PrintWorld", function()
		font:PrintWorld("Native parity", 0, 0, 0, 12, "")
	end)
	void("LuaFont.End", function()
		font:End()
	end)
	void("LuaFont.SubmitBuffered", function()
		font:SubmitBuffered(true, false)
	end)
	void("LuaFont.BindTexture", function()
		font:BindTexture()
	end)
	void("gl.DeleteFont", function()
		gl.DeleteFont(font)
	end)

	glCall(result, "gl.AddFallbackFont", function()
		return gl.AddFallbackFont("fonts/FreeSansBold.otf")
	end)
	void("gl.ClearFallbackFonts", gl.ClearFallbackFonts)
	void("gl.ResetState.restore", gl.ResetState)

	local payload = { status = "pass", result = result, context = "widget" }
	Common.setTestName(payload, "gl.fonts")
	record("gl.fonts", payload)
	if Common.mode() == "native" then
		Spring.InvokeNativeModule(Common.encode(payload))
	end
end

local function runGlMiniMapSurfaceApiTest()
	if ranGlMiniMap or not Common.enableRenderingTests() then
		return
	end
	ranGlMiniMap = true
	local result = {}
	local function void(name, fn)
		fn()
		result[name] = { n = 0, values = {} }
	end

	local px, py, sx, sy = Spring.GetMiniMapGeometry()
	if px == nil then
		error("Spring.GetMiniMapGeometry returned no minimap", 0)
	end
	void("gl.ConfigMiniMap", function()
		gl.ConfigMiniMap(px, py, sx, sy)
	end)
	void("gl.SlaveMiniMap", function()
		gl.SlaveMiniMap(true)
	end)
	void("gl.DrawMiniMap", function()
		gl.DrawMiniMap(true)
	end)
	void("gl.DrawMiniMap.custom", function()
		gl.DrawMiniMap(false)
	end)
	void("gl.SlaveMiniMap.restore", function()
		gl.SlaveMiniMap(false)
	end)
	void("gl.ConfigMiniMap.restore", function()
		gl.ConfigMiniMap(px, py, sx, sy)
	end)

	local payload = { status = "pass", result = result, context = "widget" }
	Common.setTestName(payload, "gl.minimap")
	record("gl.minimap", payload)
	if Common.mode() == "native" then
		Spring.InvokeNativeModule(Common.encode(payload))
	end
end

local function runGlObjectDrawingSurfaceApiTest()
	if ranGlObjectDrawing or not Common.enableRenderingTests() then
		return
	end
	local ids = fixtureIDs
	if ids.unitID == nil or ids.featureID == nil or ids.unitDefID == nil or ids.featureDefID == nil then
		return
	end
	ranGlObjectDrawing = true

	local pieceID = 0
	local pieceMap = Spring.GetUnitPieceMap(ids.unitID) or {}
	for _, candidate in pairs(pieceMap) do
		pieceID = candidate
		break
	end

	local result = {}
	local function void(name, fn)
		fn()
		result[name] = { n = 0, values = {} }
	end
	local function matrix(name)
		glCall(result, name, function()
			return gl.GetMatrixData(GL.MODELVIEW)
		end)
	end
	local function resetMatrices()
		gl.ResetMatrices()
	end

	void("gl.Unit", function()
		gl.Unit(ids.unitID, false, nil, true, true)
	end)
	void("gl.Unit.rawDraw", function()
		gl.Unit(ids.unitID, true, nil, true, true)
	end)
	void("gl.UnitRaw", function()
		gl.UnitRaw(ids.unitID, false, nil, true, true)
	end)
	void("gl.UnitRaw.rawDraw", function()
		gl.UnitRaw(ids.unitID, true, nil, true, true)
	end)
	void("gl.Feature", function()
		gl.Feature(ids.featureID, false, nil, true)
	end)
	void("gl.Feature.rawDraw", function()
		gl.Feature(ids.featureID, true, nil, true)
	end)
	void("gl.FeatureRaw", function()
		gl.FeatureRaw(ids.featureID, false, nil, true)
	end)
	void("gl.FeatureRaw.rawDraw", function()
		gl.FeatureRaw(ids.featureID, true, nil, true)
	end)
	void("gl.UnitTextures.push", function() gl.UnitTextures(ids.unitID, true) end)
	void("gl.UnitTextures.pop", function() gl.UnitTextures(ids.unitID, false) end)
	void("gl.FeatureTextures.push", function() gl.FeatureTextures(ids.featureID, true) end)
	void("gl.FeatureTextures.pop", function() gl.FeatureTextures(ids.featureID, false) end)
	void("gl.UnitShape", function()
		gl.UnitShape(ids.unitDefID, ids.teamID, true, false, true)
	end)
	void("gl.UnitShapeTextures.push", function() gl.UnitShapeTextures(ids.unitDefID, true) end)
	void("gl.UnitShapeTextures.pop", function() gl.UnitShapeTextures(ids.unitDefID, false) end)
	void("gl.FeatureShape", function()
		gl.FeatureShape(ids.featureDefID, ids.teamID, true, false, true)
	end)
	void("gl.FeatureShapeTextures.push", function() gl.FeatureShapeTextures(ids.featureDefID, true) end)
	void("gl.FeatureShapeTextures.pop", function() gl.FeatureShapeTextures(ids.featureDefID, false) end)

	resetMatrices()
	void("gl.UnitMultMatrix", function() gl.UnitMultMatrix(ids.unitID) end)
	matrix("gl.UnitMultMatrix.matrix")
	resetMatrices()
	void("gl.UnitPiece", function() gl.UnitPiece(ids.unitID, pieceID) end)
	resetMatrices()
	void("gl.UnitPieceMatrix", function() gl.UnitPieceMatrix(ids.unitID, pieceID) end)
	matrix("gl.UnitPieceMatrix.matrix")
	resetMatrices()
	void("gl.UnitPieceMultMatrix", function() gl.UnitPieceMultMatrix(ids.unitID, pieceID) end)
	matrix("gl.UnitPieceMultMatrix.matrix")
	void("gl.FeatureMultMatrix", function() gl.FeatureMultMatrix(ids.featureID) end)
	matrix("gl.FeatureMultMatrix.matrix")
	resetMatrices()
	void("gl.FeaturePiece", function() gl.FeaturePiece(ids.featureID, pieceID) end)
	resetMatrices()
	void("gl.FeaturePieceMatrix", function() gl.FeaturePieceMatrix(ids.featureID, pieceID) end)
	matrix("gl.FeaturePieceMatrix.matrix")
	resetMatrices()
	void("gl.FeaturePieceMultMatrix", function() gl.FeaturePieceMultMatrix(ids.featureID, pieceID) end)
	matrix("gl.FeaturePieceMultMatrix.matrix")

	void("gl.DrawGroundCircle", function()
		gl.DrawGroundCircle(64, 0, 64, 12, 16)
	end)
	if ids.weaponDefID ~= nil then
		void("gl.DrawGroundCircle.ballistic", function()
			gl.DrawGroundCircle(64, 0, 64, 12, 16, 0.1, 0.5, ids.weaponDefID)
		end)
	end
	void("gl.DrawGroundQuad", function()
		gl.DrawGroundQuad(0, 0, 128, 128)
	end)
	void("gl.DrawGroundQuad.textured", function()
		gl.DrawGroundQuad(0, 0, 128, 128, nil, 0.1, 0.2, 0.8, 0.9)
	end)

	local listIndex = gl.CreateList(function()
		gl.BeginEnd(GL.TRIANGLES, function()
			gl.Vertex(0, 0, 0)
			gl.Vertex(1, 0, 0)
			gl.Vertex(0, 1, 0)
		end)
	end)
	if listIndex == nil then
		error("gl.CreateList returned nil for object drawing", 0)
	end
	void("gl.DrawListAtUnit", function()
		gl.DrawListAtUnit(ids.unitID, listIndex, true, 1, 1, 1, 0, 0, 1, 0)
	end)
	void("gl.DeleteList", function() gl.DeleteList(listIndex) end)

	local callbackMatrix
	void("gl.DrawFuncAtUnit", function()
		gl.DrawFuncAtUnit(ids.unitID, true, function()
			local values = captureGlValues(gl.GetMatrixData(GL.MODELVIEW))
			callbackMatrix = {}
			for index = 1, values.n do
				callbackMatrix[index] = values[index]
			end
			gl.Color(0.21, 0.31, 0.41, 0.51)
		end)
	end)
	local callbackValues = callbackMatrix or {}
	result["gl.DrawFuncAtUnit.matrix"] = {
		n = #callbackValues,
		values = callbackValues,
	}

	resetMatrices()
	local payload = {
		status = "pass",
		result = result,
		context = "widget",
		fixture = {
			unitID = ids.unitID,
			featureID = ids.featureID,
			unitDefID = ids.unitDefID,
			featureDefID = ids.featureDefID,
			weaponDefID = ids.weaponDefID,
			teamID = ids.teamID,
			pieceID = pieceID,
		},
	}
	Common.setTestName(payload, "gl.object_drawing")
	record("gl.object_drawing", payload)
	if Common.mode() == "native" then
		Spring.InvokeNativeModule(Common.encode(payload))
	end
end

local function runGlFixedImmediateSurfaceApiTest()
	if ranGlFixedImmediate or not Common.enableRenderingTests() then
		return
	end
	ranGlFixedImmediate = true
	local result = {}
	local function void(name, fn)
		fn()
		result[name] = { n = 0, values = {} }
	end

	-- Exercise the overloaded Lua state callouts through the equivalent native
	-- query shapes.  The fixed-state values are flattened so the comparison is
	-- about the GL state, not Lua table key representation.
	void("gl.ResetState", gl.ResetState)
	void("gl.DepthTest", function() gl.DepthTest(GL.GREATER) end)
	void("gl.Culling", function() gl.Culling(true) end)
	void("gl.Blending", function() gl.Blending(GL.SRC_ALPHA, GL.ONE_MINUS_SRC_ALPHA) end)
	void("gl.BlendFuncSeparate", function()
		gl.BlendFuncSeparate(GL.SRC_ALPHA, GL.ONE_MINUS_SRC_ALPHA, GL.ONE, GL.ZERO)
	end)
	void("gl.BlendEquationSeparate", function()
		gl.BlendEquationSeparate(GL.FUNC_ADD, GL.FUNC_REVERSE_SUBTRACT)
	end)
	void("gl.ColorMask", function() gl.ColorMask(true, false, true, false) end)
	void("gl.AlphaTest", function() gl.AlphaTest(GL.LESS, 0.25) end)
	void("gl.AlphaToCoverage", function() gl.AlphaToCoverage(false, true) end)
	void("gl.DepthClamp", function() gl.DepthClamp(true) end)
	void("gl.LogicOp", function() gl.LogicOp(GL.XOR) end)
	void("gl.ShadeModel", function() gl.ShadeModel(GL.FLAT) end)
	void("gl.Scissor", function() gl.Scissor(4, 5, 64, 32) end)
	void("gl.LineStipple", function() gl.LineStipple(2, 0xAAAA) end)
	void("gl.PointSprite", function() gl.PointSprite(true) end)
	void("gl.PolygonMode", function() gl.PolygonMode(GL.FRONT_AND_BACK, GL.FILL) end)
	void("gl.PolygonOffset", function() gl.PolygonOffset(1.25, 2.5) end)
	void("gl.StencilTest", function() gl.StencilTest(true) end)
	void("gl.StencilFunc", function() gl.StencilFunc(GL.LEQUAL, 1, 0xFF) end)
	void("gl.StencilFuncSeparate", function()
		gl.StencilFuncSeparate(GL.FRONT_AND_BACK, GL.GEQUAL, 2, 0x7F)
	end)
	void("gl.StencilMask", function() gl.StencilMask(0xF0) end)
	void("gl.StencilMaskSeparate", function() gl.StencilMaskSeparate(GL.FRONT_AND_BACK, 0x0F) end)
	void("gl.StencilOp", function() gl.StencilOp(GL.KEEP, GL.REPLACE, GL.INVERT) end)
	void("gl.StencilOpSeparate", function()
		gl.StencilOpSeparate(GL.FRONT_AND_BACK, GL.KEEP, GL.ZERO, GL.REPLACE)
	end)
	void("gl.ClipDistance", function() gl.ClipDistance(0, true) end)
	void("gl.ClipPlane", function() gl.ClipPlane(1, 1, 0, 0, 0) end)
	void("gl.PointParameter", function() gl.PointParameter(0.5, 0.25, 0.125, 1, 64, 8) end)
	void("gl.Light", function() gl.Light(0, GL.DIFFUSE, 1, 0.5, 0.25, 1) end)
	void("gl.Material", function()
		gl.Material({ ambient = { 0.1, 0.2, 0.3, 1 } })
	end)
	void("gl.TexEnv", function()
		gl.TexEnv(GL.TEXTURE_ENV, GL.TEXTURE_ENV_COLOR, 0.1, 0.2, 0.3, 0.4)
	end)
	void("gl.MultiTexEnv", function()
		gl.MultiTexEnv(1, GL.TEXTURE_ENV, GL.TEXTURE_ENV_MODE, GL.MODULATE)
	end)
	void("gl.TexGen", function()
		gl.TexGen(GL.S, GL.TEXTURE_GEN_MODE, GL.OBJECT_LINEAR)
	end)
	void("gl.MultiTexGen", function()
		gl.MultiTexGen(1, GL.T, GL.TEXTURE_GEN_MODE, GL.EYE_LINEAR)
	end)
	void("gl.PushAttrib", function() gl.PushAttrib(GL.ENABLE_BIT) end)
	void("gl.PopAttrib", gl.PopAttrib)
	void("gl.MemoryBarrier", function() gl.MemoryBarrier(0) end)
	void("gl.DispatchCompute", function() gl.DispatchCompute(1, 1, 1, 0) end)
	void("gl.ActiveTexture", function()
		gl.ActiveTexture(1, function()
			gl.TexEnv(GL.TEXTURE_ENV, GL.TEXTURE_ENV_MODE, GL.MODULATE)
		end)
	end)
	void("gl.ObjectLabel", function() gl.ObjectLabel(GL.TEXTURE, 0, "native-api-parity") end)
	void("gl.PushDebugGroup", function() gl.PushDebugGroup(1, "native-api-parity") end)
	void("gl.PopDebugGroup", gl.PopDebugGroup)

	glCall(result, "gl.GetFixedState.blending", function()
		local enabled, state = gl.GetFixedState("blending")
		return enabled, state.GL_BLEND_SRC_RGB, state.GL_BLEND_SRC_ALPHA,
			state.GL_BLEND_DST_RGB, state.GL_BLEND_DST_ALPHA,
			state.GL_BLEND_EQUATION_RGB, state.GL_BLEND_EQUATION_ALPHA
	end)
	glCall(result, "gl.GetFixedState.depth", function()
		local enabled, write, state = gl.GetFixedState("depth")
		return enabled, write, state.GL_DEPTH_FUNC
	end)
	glCall(result, "gl.GetFixedState.culling", function()
		local enabled, state = gl.GetFixedState("culling")
		return enabled, state.GL_CULL_FACE_MODE
	end)
	glCall(result, "gl.GetFixedState.colorMask", function()
		local state = gl.GetFixedState("colorMask")
		return state.GL_COLOR_WRITEMASK_R, state.GL_COLOR_WRITEMASK_G,
			state.GL_COLOR_WRITEMASK_B, state.GL_COLOR_WRITEMASK_A
	end)
	glCall(result, "gl.GetFixedState.alphaTest", function()
		local enabled, state = gl.GetFixedState("alphaTest")
		return enabled, state.GL_ALPHA_TEST_FUNC, state.GL_ALPHA_TEST_REF
	end)
	glCall(result, "gl.GetFixedState.lineWidth", function()
		return gl.GetFixedState("lineWidth")
	end)
	glCall(result, "gl.GetFixedState.pointSize", function()
		return gl.GetFixedState("pointSize")
	end)

	void("gl.ResetState.restore", gl.ResetState)
	local payload = { status = "pass", result = result, context = "widget" }
	Common.setTestName(payload, "gl.fixed_immediate")
	record("gl.fixed_immediate", payload)
	if Common.mode() == "native" then
		Spring.InvokeNativeModule(Common.encode(payload))
	end
end

local function runRmlUiTests()
	if ranRmlUiTests or not Common.enableRenderingTests() then
		return
	end
	ranRmlUiTests = true
	runRmlSurfaceApiTest()

	local function runRmlElementSurfaceApiTest()
		if ranRmlElementSurfaceApiTest then
			return
		end
		ranRmlElementSurfaceApiTest = true

		local testName = "rml.element_form_event"
		local result = {}
		local context = RmlUi.CreateContext("native_api_parity_surface_element_form")
		if context == nil then
			error("RmlUi.CreateContext for element surface returned nil", 0)
		end
		local document = context:CreateDocument()
		if document == nil then
			error("element surface document creation failed", 0)
		end
		document.inner_rml = [[
			<form id="surface-form">
				<div id="container" class="panel primary">
					<span id="alpha" class="chip hot">A</span>
					<button id="beta" class="chip">B</button>
				</div>
				<input id="input" value="abcdef" />
				<textarea id="textarea">hello world</textarea>
				<select id="select"><option value="one">One</option></select>
				<tabset id="tabs"></tabset>
			</form>
		]]
		document:UpdateDocument()

		local container = expectElement("surface container", document:GetElementById("container"))
		local alpha = expectElement("surface alpha", document:GetElementById("alpha"))
		local beta = expectElement("surface beta", document:GetElementById("beta"))
		local input = expectElement("surface input", document:GetElementById("input"))
		local textarea = expectElement("surface textarea", document:GetElementById("textarea"))
		local select = expectElement("surface select", document:GetElementById("select"))
		local form = expectCast("surface form", RmlUi.Element.As.ElementForm(document:GetElementById("surface-form")))

		rmlSurfaceCall(result, "RmlUi.Element.AddEventListener", function()
			return beta:AddEventListener("click", function(event)
				rmlSurfaceCall(result, "RmlUi.Element.ProcessDefaultAction", function()
					return beta:ProcessDefaultAction(event)
				end)
			end)
		end)
		rmlSurfaceCall(result, "RmlUi.Element.AppendChild", function()
			return container:AppendChild(document:CreateElement("p"))
		end)
		rmlSurfaceCall(result, "RmlUi.Element.ArePseudoCLassesSet", function()
			return container:ArePseudoClassesSet({ "panel", "primary" })
		end)
		rmlSurfaceCall(result, "RmlUi.Element.Blur", function()
			return alpha:Blur()
		end)
		rmlSurfaceCall(result, "RmlUi.Element.Click", function()
			return beta:Click()
		end)
		rmlSurfaceCall(result, "RmlUi.Element.Clone", function()
			return alpha:Clone()
		end)
		rmlSurfaceCall(result, "RmlUi.Element.Closest", function()
			return alpha:Closest(".panel")
		end)
		rmlSurfaceCall(result, "RmlUi.Element.DispatchEvent", function()
			return beta:DispatchEvent("click")
		end)
		rmlSurfaceCall(result, "RmlUi.Element.Focus", function()
			return input:Focus()
		end)
		rmlSurfaceCall(result, "RmlUi.Element.GetActivePseudoCLasses", function()
			return container:GetActivePseudoClasses()
		end)
		rmlSurfaceCall(result, "RmlUi.Element.GetAttribute", function()
			return container:GetAttribute("class")
		end)
	rmlSurfaceCall(result, "RmlUi.Element.GetChild", function()
			return container:GetChild(0)
		end)
		rmlSurfaceCall(result, "RmlUi.Element.GetElementById", function()
			return document:GetElementById("alpha")
		end)
		rmlSurfaceCall(result, "RmlUi.Element.GetElementsByClassName", function()
			return container:GetElementsByClassName("chip")
		end)
		rmlSurfaceCall(result, "RmlUi.Element.GetElementsByTagName", function()
			return container:GetElementsByTagName("span")
		end)
		rmlSurfaceCall(result, "RmlUi.Element.GetValue", function()
			return input:GetValue()
		end)
		rmlSurfaceCall(result, "RmlUi.Element.HasAttribute", function()
			return container:HasAttribute("class")
		end)
		rmlSurfaceCall(result, "RmlUi.Element.HasChildNodes", function()
			return container:HasChildNodes()
		end)
		rmlSurfaceCall(result, "RmlUi.Element.InsertBefore", function()
			return container:InsertBefore(document:CreateElement("i"), beta)
		end)
		rmlSurfaceCall(result, "RmlUi.Element.IsClassSet", function()
			return alpha:IsClassSet("chip")
		end)
		rmlSurfaceCall(result, "RmlUi.Element.IsPointWithinElement", function()
			return container:IsPointWithinElement(RmlUi.Vector2f.new(1.0, 1.0))
		end)
		rmlSurfaceCall(result, "RmlUi.Element.IsPseudoClassSet", function()
			return alpha:IsPseudoClassSet("hover")
		end)
		rmlSurfaceCall(result, "RmlUi.Element.IsVisible", function()
			return container:IsVisible()
		end)
		rmlSurfaceCall(result, "RmlUi.Element.Matches", function()
			return alpha:Matches("span.chip")
		end)
		rmlSurfaceCall(result, "RmlUi.Element.QuerySelector", function()
			return container:QuerySelector("#beta")
		end)
		rmlSurfaceCall(result, "RmlUi.Element.QuerySelectorAll", function()
			return container:QuerySelectorAll(".chip")
		end)
		rmlSurfaceCall(result, "RmlUi.Element.RemoveAttribute", function()
			return alpha:RemoveAttribute("class")
		end)
		rmlSurfaceCall(result, "RmlUi.Element.RemoveChild", function()
			return container:RemoveChild(beta)
		end)
		local replacement = document:CreateElement("em")
		rmlSurfaceCall(result, "RmlUi.Element.ReplaceChild", function()
			return container:ReplaceChild(replacement, alpha)
		end)
		rmlSurfaceCall(result, "RmlUi.Element.ScrollIntoView", function()
			return container:ScrollIntoView(true)
		end)
		rmlSurfaceCall(result, "RmlUi.Element.SetAttribute", function()
			return container:SetAttribute("data-surface", "native-api-parity")
		end)
		rmlSurfaceCall(result, "RmlUi.Element.SetClass", function()
			return container:SetClass("selected", true)
		end)
		rmlSurfaceCall(result, "RmlUi.Element.SetPseudoClass", function()
			return container:SetPseudoClass("hover", true)
		end)

		rmlSurfaceCall(result, "RmlUi.ElementForm.Submit", function()
			return form:Submit("surface", "value")
		end)

		local inputControl = expectCast("surface input control", RmlUi.Element.As.ElementFormControlInput(input))
		rmlSurfaceCall(result, "RmlUi.ElementFormControlInput.SetSelection", function()
			return inputControl:SetSelection(1, 4)
		end)
		rmlSurfaceCall(result, "RmlUi.ElementFormControlInput.GetSelection", function()
			return inputControl:GetSelection()
		end)
		rmlSurfaceCall(result, "RmlUi.ElementFormControlInput.Select", function()
			return inputControl:Select()
		end)

		local selectControl = expectCast("surface select control", RmlUi.Element.As.ElementFormControlSelect(select))
		rmlSurfaceCall(result, "RmlUi.ElementFormControlSelect.Add", function()
			return selectControl:Add(document:CreateElement("option"))
		end)
		rmlSurfaceCall(result, "RmlUi.ElementFormControlSelect.Remove", function()
			return selectControl:Remove(1)
		end)
		rmlSurfaceCall(result, "RmlUi.ElementFormControlSelect.RemoveAll", function()
			return selectControl:RemoveAll()
		end)

		local textareaControl = expectCast("surface textarea control", RmlUi.Element.As.ElementFormControlTextArea(textarea))
		rmlSurfaceCall(result, "RmlUi.ElementFormControlTextArea.SetSelection", function()
			return textareaControl:SetSelection(0, 5)
		end)
		rmlSurfaceCall(result, "RmlUi.ElementFormControlTextArea.GetSelection", function()
			return textareaControl:GetSelection()
		end)
		rmlSurfaceCall(result, "RmlUi.ElementFormControlTextArea.Select", function()
			return textareaControl:Select()
		end)

		local tabSet = expectCast("surface tab set", RmlUi.Element.As.ElementTabSet(document:GetElementById("tabs")))
		rmlSurfaceCall(result, "RmlUi.ElementTabSet.SetPanel", function()
			return tabSet:SetPanel(0, "<div>panel</div>")
		end)
		rmlSurfaceCall(result, "RmlUi.ElementTabSet.SetTab", function()
			return tabSet:SetTab(0, "Tab")
		end)
		rmlSurfaceCall(result, "RmlUi.ElementTabSet.RemoveTab", function()
			return tabSet:RemoveTab(0)
		end)

		local model = context:OpenDataModel("native_api_parity_surface_element_model", { value = "surface" })
		if model == nil then
			error("surface data model creation failed", 0)
		end
		rmlSurfaceCall(result, "RmlUi.SolLuaDataModel.__SetDirty", function()
			return model:__SetDirty("value")
		end)

		context:UnloadAllDocuments()
		RmlUi.RemoveContext(context)

		local payload = { status = "pass", result = result, context = "widget" }
		Common.setTestName(payload, testName)
		record(testName, payload)
		if Common.mode() == "native" then
			Spring.InvokeNativeModule(Common.encode(payload))
		end

		local listenerType = type(RmlUi.EventListener)
		local canConstructListener = pcall(function()
			return RmlUi.EventListener()
		end)
		if listenerType ~= "table" or canConstructListener then
			error("RmlUi.EventListener should remain an abstract no-constructor type", 0)
		end
		local listenerPayload = {
			status = "pass",
			listenerType = listenerType,
			constructible = canConstructListener,
			context = "widget",
		}
		Common.setTestName(listenerPayload, "rml.event_listener_nonconstructible")
		record("rml.event_listener_nonconstructible", listenerPayload)
	end

	runRmlElementSurfaceApiTest()

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

function NativeApiParityRenderFixture(unitID, featureID, unitDefID, featureDefID, projectileID, teamID)
	renderFixtureIDs = {
		unitID = unitID,
		featureID = featureID,
		unitDefID = unitDefID,
		featureDefID = featureDefID,
		projectileID = projectileID,
		teamID = teamID,
	}
end

function GameSetup()
	return true, true
end

function DrawScreen(viewSizeX, viewSizeY)
	runGlStateSurfaceApiTest()
	runGlStateMutationSurfaceApiTest()
	runGlImmediatePrimitivesSurfaceApiTest()
	runGlShaderUniformSurfaceApiTest()
	runGlTextureResourceSurfaceApiTest()
	runGlListsQuerySurfaceApiTest()
	runGlAtlasSurfaceApiTest()
	runGlFboSurfaceApiTest()
	runGlResourceHandleSurfaceApiTest()
	runGlUserdataSurfaceApiTest()
	runGlFontSurfaceApiTest()
	runGlMiniMapSurfaceApiTest()
	runGlObjectDrawingSurfaceApiTest()
	runGlFixedImmediateSurfaceApiTest()
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
	if frame == 19 and not ranScriptKillTest and Script and Script.Kill then
		ranScriptKillTest = true
		local ok, returnCount = pcall(function()
			return select("#", Script.Kill("native_api_parity Script.Kill"))
		end)
		record("script.kill", {
		called = ok,
		returnCount = ok and returnCount or -1,
	})
	end
end

VFS.Include("LuaUI/callin_ui_trace.lua")
