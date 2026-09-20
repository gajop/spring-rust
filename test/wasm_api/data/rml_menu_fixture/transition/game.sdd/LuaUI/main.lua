-- This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

local frames = 0

function Initialize()
	Spring.Echo("[RmlUiMenuTransitionGame] game LuaUI initialized")
end

function Update()
	frames = frames + 1
	if frames == 30 then
		Spring.Echo("[RmlUiMenuTransitionGame] returning to menu")
		Spring.Reload("")
	end
end
