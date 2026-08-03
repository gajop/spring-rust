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
local GeneratedTests = VFS.Include("LuaRules/Utilities/generated_api_tests.lua")
local outputPath
local sentInventory = false
local ranGeneratedTests = false
local fixtureIDs = {}

local function record(name, payload)
	if outputPath == nil then
		outputPath = Common.outputDir() .. "/widget.jsonl"
	end
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

function widget:NativeApiParityFixture(unitID, featureID, unitDefID, featureDefID, weaponDefID, projectileID, pieceProjectileID, teamID, allyTeamID, groundDecalID)
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
