-- This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
-- Test overlay only, included by the isolated LuaRules/draw.lua.
local updates, pausedFrame, pausedUpdates = 0, nil, 0
local started, previousGameStart = false, GameStart
function GameStart(...)
	started = true
	if previousGameStart then previousGameStart(...) end
end
local previousUpdate = Update
function Update()
	if previousUpdate then previousUpdate() end
	if Spring.IsReplay() or not started then return end
	updates = updates + 1
	if updates == 2 then
		Spring.SendCommands("pause 1")
	end
	local _, _, paused = Spring.GetGameSpeed()
	if not paused then return end
	pausedUpdates = pausedUpdates + 1
	if pausedUpdates == 1 then
		pausedFrame = Spring.GetGameFrame()
		Spring.Echo("JOBS_PAUSE_STABLE_BEGIN " .. pausedFrame)
	elseif pausedUpdates == 200 then
		local frame = Spring.GetGameFrame()
		Spring.Echo("JOBS_PAUSE_STABLE_END " .. frame)
		if frame == pausedFrame then Spring.Echo("JOBS_PAUSE_PASS") else Spring.Echo("JOBS_PAUSE_FAIL") end
		Spring.SendCommands("pause 0")
	end
end
