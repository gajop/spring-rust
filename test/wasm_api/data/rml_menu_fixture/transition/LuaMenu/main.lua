-- This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

local menuActivations = 0
local frames = 0
local startingGame = false

local gameStartScript = VFS.LoadFile("startscript.txt")
assert(gameStartScript, "RmlUi transition fixture could not load startscript.txt")

function Initialize()
	Spring.Echo("[RmlUiMenuTransition] LuaMenu initialized")
end

function ActivateMenu()
	menuActivations = menuActivations + 1
	frames = 0
	startingGame = false
	Spring.Echo("[RmlUiMenuTransition] menu activation " .. menuActivations)
end

function Update()
	frames = frames + 1

	if menuActivations >= 1 and menuActivations <= 2 and not startingGame and frames == 60 then
		startingGame = true
		Spring.Echo("[RmlUiMenuTransition] starting game cycle " .. menuActivations)
		Spring.Reload(gameStartScript)
	elseif menuActivations == 3 and frames == 60 then
		Spring.Echo("[RmlUiMenuTransition] completed two menu-game-menu cycles")
		Spring.Quit()
	end
end
