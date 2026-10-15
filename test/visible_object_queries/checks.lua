-- This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
local stage, draws, failed, started = 1, 0, false, false
local function signature(ids)
	table.sort(ids)
	return table.concat(ids, ',')
end
local function check()
	local w,h = Spring.GetViewGeometry()
	local queries = {
		function() return Spring.GetVisibleUnits(-1, 0, true) end,
		function() return Spring.GetVisibleUnits(-1, 0, false) end,
		function() return Spring.GetVisibleUnits(-2, 0, true) end,
		function() return Spring.GetVisibleFeatures(-1, 0, true, true) end,
		function() return Spring.GetUnitsInScreenRectangle(0, 0, w, h, -1) end,
		function() return Spring.GetFeaturesInScreenRectangle(0, 0, w, h) end,
	}
	local nonempty = 0
	for _,query in ipairs(queries) do
		local reference
		for _,mode in ipairs({1, 2, 0}) do
			Spring.SetConfigInt('SparseMapVisibilityQuery', mode, true)
			local ids = query() or {}
			local actual = signature(ids)
			if reference then assert(actual == reference, 'query strategies returned different IDs') end
			reference = actual
			if #ids > 0 then nonempty = nonempty+1 end
		end
	end
	assert(nonempty >= 12, 'visibility comparisons were vacuous')
end
local function camera()
	local dist = stage == 1 and 1500 or math.min(50000, Game.mapSizeX * 1.5)
	Spring.SetCameraState({name='spring',mode=2,px=Game.mapSizeX/2,py=0,pz=Game.mapSizeZ/2,
		dist=dist,rx=math.acos(-math.sin(math.rad(75))),ry=math.pi,fov=45},0)
	draws = 0
end
function Update()
	if started or Spring.GetGameFrame() < 5 or Spring.GetGameRulesParam('visibility_ready') ~= 1 then return end
	started = true
	for _,id in ipairs(Spring.GetAllUnits()) do Spring.SetUnitEngineDrawMask(id, 0) end
	Spring.SendCommands('pause 1', 'maxviewrange 65536', 'shadows 0', 'mapborder 0')
	Spring.SetDrawGround(false)
	camera()
end
function DrawScreen()
	if not started or failed then return end
	draws = draws+1
	if draws < 5 then return end
	local ok,err = pcall(check)
	if not ok then
		failed = true
		Spring.Echo('VISIBLE_OBJECT_QUERIES_FAIL '..tostring(err))
		Spring.SendCommands('quitforce')
		return
	end
	if stage == 1 then stage=2;camera();return end
	Spring.Echo('VISIBLE_OBJECT_QUERIES_PASS')
	failed=true
	Spring.SendCommands('quitforce')
end
