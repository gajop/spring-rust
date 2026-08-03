include "constants.lua"

local empty_root_piece = piece("empty_root_piece")

function script.Create()
end

function script.Killed(recentDamage, maxHealth)
	return 0
end
