local Common = VFS.Include("LuaRules/Utilities/native_api_parity_common.lua")
local GeneratedTests = VFS.Include("LuaRules/Utilities/generated_api_tests.lua")
local outputPath = Common.outputDir() .. "/widget.jsonl"
local sentInventory = false
local ranGeneratedTests = false

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
		runGeneratedTests()
		record("game_frame", { value = Spring.GetGameFrame() })
		record("visible_units", { count = #(Spring.GetVisibleUnits() or {}) })
	elseif frame == 20 then
		Spring.SendCommands("quitforce")
	end
end
