-- This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

local controls = {}

function controls:GetInfo()
	return {
		name = "Game controls",
		desc = "Shared chat, pause, speed and screenshot keys",
		author = "gajop",
		license = "GPL-v2-or-later",
		before = {"all"},
		enabled = true,
	}
end

function controls:Initialize(keepRtsBindings)
	if not keepRtsBindings then
		Spring.SendCommands({
			"unbindall",
			"unbind enter chat",
			"bind Any+tab edit_complete",
			"bind Any+backspace edit_backspace",
			"bind Any+delete edit_delete",
			"bind Any+home edit_home",
			"bind Alt+left edit_home",
			"bind Any+end edit_end",
			"bind Alt+right edit_end",
			"bind Any+up edit_prev_line",
			"bind Any+down edit_next_line",
			"bind Any+left edit_prev_char",
			"bind Any+right edit_next_char",
			"bind Ctrl+left edit_prev_word",
			"bind Ctrl+right edit_next_word",
			"bind Any+enter edit_return",
			"bind Any+escape edit_escape",
			"bind Ctrl+v pastetext",
		})
	end
	Spring.SendCommands({
		"bind Any+enter chat",
		"bind Any+pause pause",
		"bind Any+f12 screenshot",
		"bind Alt+= speedup",
		"bind Alt++ speedup",
		"bind Alt+- slowdown",
		"bind Alt+numpad+ speedup",
		"bind Alt+numpad- slowdown",
		"bind Alt+insert speedup",
		"bind Alt+delete slowdown",
		"bind Ctrl+q reloadforce",
	})
end

function controls:KeyPress(key, mods, isRepeat)
	local command
	if key == Spring.GetKeyCode("pause") then
		command = "pause"
	elseif key == Spring.GetKeyCode("f12") then
		command = "screenshot"
	elseif mods.alt and not mods.ctrl and not mods.meta then
		if key == Spring.GetKeyCode("=") or key == Spring.GetKeyCode("+")
			or key == Spring.GetKeyCode("numpad+") or key == Spring.GetKeyCode("insert") then
			command = "speedup"
		elseif key == Spring.GetKeyCode("-") or key == Spring.GetKeyCode("numpad-")
			or key == Spring.GetKeyCode("delete") then
			command = "slowdown"
		else
			return false
		end
	else
		return false
	end
	if not isRepeat or command == "speedup" or command == "slowdown" then
		Spring.SendCommands(command)
	end
	return true
end

function controls:Shutdown()
	Spring.SendCommands("keyreload")
end

return controls
