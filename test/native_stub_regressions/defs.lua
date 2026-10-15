-- This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
local defs=VFS.Include('gamedata/audit_base_defs.lua')
local function copy(t)
	local r={}
	for k,v in pairs(t) do r[k]=type(v)=='table' and copy(v) or v end
	return r
end
local base=defs.native_api_test_unit
defs.audit_wall=copy(defs.native_api_test_factory)
defs.audit_wall.builder=false
defs.audit_wall.footprintX=4
defs.audit_wall.footprintZ=4
defs.audit_wall.yardMap=string.rep('o',16)
base.maxVelocity=1
base.buildTime=100000
base.weapondefs.parity_weapon.turnRate=1234
for _,item in ipairs({{'audit_shield_a',.2},{'audit_shield_b',.7}}) do
	base.weapondefs[item[1]]={name=item[1],weapontype='Shield',shieldradius=100,
		shieldpower=100,shieldstartingpower=100,shieldalpha=item[2],shieldintercepttype=1}
	base.weapons[#base.weapons+1]={def=item[1]}
end
defs.audit_hovercraft=copy(base)
defs.audit_hovercraft.movementClass='HOVER1'
defs.audit_hovercraft.hoverAttack=false
defs.audit_hover_aircraft=copy(base)
defs.audit_hover_aircraft.canFly=true
defs.audit_hover_aircraft.movementClass=nil
defs.audit_hover_aircraft.hoverAttack=true
return defs
