function widget:GetInfo()
	return {
		name = "Native API Parity Widget",
		desc = "Records unsynced widget API results for native parity tests",
		author = "Spring",
		layer = 0,
		enabled = true,
	}
end

local Common = VFS.Include("LuaRules/Utilities/native_api_parity_common.lua")
local outputPath
local sentInventory = false

local function record(name, payload)
	if outputPath == nil then
		outputPath = Common.outputDir() .. "/widget.jsonl"
	end
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

function widget:Initialize()
	outputPath = Common.outputDir() .. "/widget.jsonl"
	recordInventory()
	record("game_frame_initial", { value = Spring.GetGameFrame() })
	record("my_player", { playerID = Spring.GetMyPlayerID(), teamID = Spring.GetMyTeamID(), allyTeamID = Spring.GetMyAllyTeamID() })
	Spring.SendCommands("setmaxspeed 1000", "setminspeed 1")
end

function widget:GameFrame(frame)
	if frame == 4 then
		recordInventory()
		record("game_frame", { value = Spring.GetGameFrame() })
		record("visible_units", { count = #(Spring.GetVisibleUnits() or {}) })
	elseif frame == 20 then
		Spring.SendCommands("quitforce")
	end
end
