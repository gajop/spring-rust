-- This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
function gadget:GetInfo()
	return {name = 'Visibility query fixture', layer = 0, enabled = true}
end
if not gadgetHandler:IsSyncedCode() then return end
function gadget:Initialize()
	for i = 0, 319 do
		local x = i == 0 and Game.mapSizeX/2 or (.1 + (i%18)/22) * Game.mapSizeX
		local z = i == 0 and Game.mapSizeZ/2 or (.1 + math.floor(i/18)/22) * Game.mapSizeZ
		local unit = assert(Spring.CreateUnit('native_api_test_unit', x, 0, z, 0, 0))
		Spring.MoveCtrl.Enable(unit)
		Spring.MoveCtrl.SetPosition(unit, x, 0, z)
		Spring.SetUnitSensorRadius(unit, 'los', 500)
		assert(Spring.CreateFeature('native_api_test_feature', x+20, 0, z+20))
	end
	Spring.SetGameRulesParam('visibility_ready', 1, {public=true})
end
