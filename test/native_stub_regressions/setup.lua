-- This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
function gadget:GetInfo() return {name='Non-vacuous native audit',layer=0,enabled=true} end
if not gadgetHandler:IsSyncedCode() then return end
local function send(...)
	local fields={...}
	for i,v in ipairs(fields) do if type(v)=='boolean' then fields[i]=v and 1 or 0 end end
	Spring.InvokeNativeModule(table.concat(fields,' '))
end
local factory
local function run(frame)
	if frame == 35 then
		assert(Spring.GetUnitIsBuilding(factory), 'factory must start construction for a nonzero observation')
		send('factory',factory,1)
		Spring.SetGameRulesParam('audit_synced_ready',1,{public=true})
		return
	end
	if frame ~= 5 then return end
	local unit=assert(Spring.CreateUnit('native_api_test_unit',1000,96,1000,0,0))
	local unitCalls={
		QueryWeapon=function() return 1 end,
		AimFromWeapon=function() return 1 end,
	}
	Spring.UnitScript.CreateScript(unit,unitCalls)
	factory=assert(Spring.CreateUnit('native_api_test_factory',1800,96,1800,0,0))
	Spring.UnitScript.CreateScript(factory,{
		Activate=function() Spring.UnitScript.SetUnitValue(COB.INBUILDSTANCE,1) end,
		QueryBuildInfo=function() return 1 end,
	})
	assert(Spring.CreateUnit('audit_wall',1800,96,2600,0,0))
	local def=UnitDefNames.native_api_test_unit.id
	local function move(x,z,terrain,objects,center)
		local expected=Spring.TestMoveOrder(def,x,96,z,1,0,0,terrain,objects,center)
		send('move',def,x,96,z,1,0,0,terrain,objects,center,expected)
		return expected
	end
	assert(move(2400,2400,true,true,true), 'movement fixture needs an open square')
	assert(not move(1800,2600,true,true,true), 'movement fixture needs a blocked square')
	assert(move(1800,2600,true,false,true), 'object-testing option must affect result')
	move(1800,2600,true,true,false)
	send('move',-1,0,0,0,0,0,0,true,true,false,false)
	Spring.SetUnitShieldState(unit,2,true,42)
	Spring.SetUnitShieldState(unit,3,false,77)
	local _,powerA=Spring.GetUnitShieldState(unit,2)
	local _,powerB=Spring.GetUnitShieldState(unit,3)
	assert(powerA==42 and powerB==77,'fixture needs distinct real shields in slots 2 and 3')
	for _,slot in ipairs({-1,0,1,2,3,99}) do
		local enabled,power=Spring.GetUnitShieldState(unit,slot)
		local present=enabled~=nil
		local alpha=power==42 and .2 or .7
		send('shield',unit,slot,present,present and enabled~=0 and enabled~=false or false,power or 0,alpha)
	end
	send('turn',unit,1,WeaponDefs[UnitDefs[def].weapons[1].weaponDef].turnRate)
	send('hover',UnitDefNames.audit_hovercraft.id,true)
	send('hover',UnitDefNames.audit_hover_aircraft.id,false)
	Spring.GiveOrderToUnit(factory,-def,{},0)
	send('factory',factory,Spring.GetUnitIsBuilding(factory) and 1 or 0)
	send('options')
	send('log')
	local pieces=Spring.GetUnitPieceList(unit)
	Spring.UnitScript.CallAsUnit(unit,function() Spring.UnitScript.Explode(#pieces,2) end)
	local found=false
	for _,projectile in ipairs(Spring.GetProjectilesInRectangle(0,0,Game.mapSizeX,Game.mapSizeZ,false,false)) do
		if Spring.GetPieceProjectileName(projectile) then
			send('piece',projectile,#pieces-1)
			found=true
		end
	end
	assert(found,'piece projectile fixture must produce an actual piece')
end
function gadget:GameFrame(frame)
	local ok,err=pcall(run,frame)
	if not ok then
		Spring.Echo('NO_STUBS_FAIL fixture '..tostring(err))
		Spring.SetGameRulesParam('audit_failed',1,{public=true})
	end
end
