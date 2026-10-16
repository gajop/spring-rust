-- This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

local controls = {}

function controls:GetInfo()
	return {
		name = "Game controls",
		desc = "Pause and screenshots without RTS key bindings",
		author = "gajop",
		license = "GPL-v2-or-later",
		before = {"all"},
		enabled = true,
	}
end

function controls:Initialize()
	Spring.SendCommands({
		"unbindall",
		"unbind enter chat",
		"bind Any+pause pause",
		"bind Any+f12 screenshot",
		"bind Ctrl+q reloadforce",
	})
end

function controls:KeyPress(key, mods, isRepeat)
	local command
	if key == Spring.GetKeyCode("pause") then
		command = "pause"
	elseif key == Spring.GetKeyCode("f12") then
		command = "screenshot"
	else
		return false
	end
	if not isRepeat then
		Spring.SendCommands(command)
	end
	return true
end

function controls:Shutdown()
	Spring.SendCommands("keyreload")
end

return controls
