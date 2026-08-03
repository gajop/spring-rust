function gadget:GetInfo()
	return {
		name = "Native API Parity",
		desc = "Runs Lua/native parity checks for a small engine fixture",
		author = "Spring",
		layer = 0,
		enabled = true,
	}
end

if not gadgetHandler:IsSyncedCode() then
	local Common = VFS.Include("LuaRules/Utilities/native_api_parity_common.lua")
	local GeneratedTests = VFS.Include("LuaRules/Utilities/generated_api_tests.lua")
	local sentInventory = false
	local ranGeneratedTests = false
	local fixtureIDs = {}
	local groundDecalID
	local parityOptions = Spring.GetModOptions() or {}
	local processTest = tostring(parityOptions.native_api_parity_process_test or "")
	local processStage = tostring(parityOptions.native_api_parity_process_stage or "initial")
	local function unitPayload(caseIndex, ids, values)
		values.case = caseIndex
		values.unitID = ids.unitID
		return values
	end
	local function groundPayload(caseIndex, _, values)
		values.case = caseIndex
		return values
	end
	local allHooks = VFS.Include("LuaRules/Utilities/native_api_parity_custom_hooks.lua")({
			unitPayload = unitPayload,
			groundPayload = groundPayload,
		})
	local function customUnsyncedTests(processOnly)
		local selected = {}
		for _, hook in ipairs(allHooks) do
			local normalTest = hook.name == "map_model_lights_lifecycle"
				or hook.name == "camera_state_roundtrip"
				or hook.name == "set_atmosphere_params"
				or hook.name == "set_sun_lighting_params"
				or hook.name == "set_water_params"
				or hook.name == "set_map_rendering_params"
				or hook.name == "preload_sound_item_missing"
				or hook.name == "set_window_geometry"
				or hook.name == "set_window_minimized"
				or hook.name == "set_window_maximized"
				or hook.name == "yield"
			local processTestMatch = processOnly
				and processStage == "initial"
				and hook.name == processTest
			if (not processOnly and normalTest) or processTestMatch then
				selected[#selected + 1] = hook
			end
		end
		return selected
	end
	local CustomTests = customUnsyncedTests()
	local ProcessTests = customUnsyncedTests(true)

	local function ensureGroundDecal()
		if groundDecalID == nil or groundDecalID <= 0 or Spring.GetGroundDecalType(groundDecalID) == nil then
			groundDecalID = Spring.CreateGroundDecal()
		end
		fixtureIDs.groundDecalID = groundDecalID
		fixtureIDs.decalID = groundDecalID
	end

	local function forward(stream, payload)
		if Script.LuaUI.NativeApiParityResult then
			Script.LuaUI.NativeApiParityResult(stream, payload)
		end
	end

	local function sendInventory()
		if sentInventory then
			return
		end
		sentInventory = true
		forward("unsynced_gadget", Common.encode({
			context = "unsynced_gadget",
			name = "context_inventory",
			functions = Common.springFunctionInventory(),
		}))
	end

	local function record(name, payload)
		payload.context = "unsynced_gadget"
		Common.setTestName(payload, name)
		forward("unsynced_gadget", Common.encode(payload))
	end

	local function runGeneratedTests()
		if processStage == "resume" then
			return
		end
		if ranGeneratedTests then
			return
		end
		ranGeneratedTests = true
		Common.runPortableReadOnlyTests("unsynced_gadget", GeneratedTests, record, function(encoded)
			Spring.InvokeNativeModule(encoded)
		end, fixtureIDs, function()
			-- Some rendering tests are destructive by design.  Give each random
			-- case a valid decal fixture instead of reusing the ID destroyed by the
			-- preceding case.
			ensureGroundDecal()
		end)
		Common.runCustomTests("unsynced_gadget", CustomTests, GeneratedTests, record, function(encoded)
			Spring.InvokeNativeModule(encoded)
		end, fixtureIDs)
	end

	function gadget:Initialize()
		-- Keep rendering-sensitive camera checks independent of the physical
		-- pointer position of the process that launched the fixture.  WarpMouse
		-- uses the same bottom-origin coordinates as the camera APIs under test.
		local viewSizeX, viewSizeY = Spring.GetViewGeometry()
		Spring.WarpMouse(viewSizeX * 0.5, viewSizeY * 0.5)

		-- Ground decals are an unsynced rendering fixture.  Create one after the
		-- engine has initialized the unsynced decal drawer, then pass its ID to
		-- the widget/native runner alongside the synced fixture IDs.
		ensureGroundDecal()
		forward("unsynced_gadget", Common.encode({
			context = "unsynced_gadget",
			name = "game_frame_initial",
			value = Spring.GetGameFrame(),
		}))
	end

	function gadget:RecvFromSynced(name, ...)
		if name == "native_api_parity_fixture" then
			local unitID, featureID, unitDefID, featureDefID, weaponDefID, projectileID, pieceProjectileID, teamID, allyTeamID = ...
			fixtureIDs = {
				unitID = unitID,
				featureID = featureID,
				unitDefID = unitDefID,
				featureDefID = featureDefID,
				weaponDefID = weaponDefID,
				projectileID = projectileID,
				pieceProjectileID = pieceProjectileID,
				teamID = teamID,
				allyTeamID = allyTeamID,
				groundDecalID = groundDecalID,
			}
			if Script.LuaUI.NativeApiParityFixture then
				Script.LuaUI.NativeApiParityFixture(unitID, featureID, unitDefID, featureDefID, weaponDefID, projectileID, pieceProjectileID, teamID, allyTeamID, groundDecalID)
			end
			return
		elseif name == "native_api_parity_result" then
			local stream, encodedPayload = ...
			forward(stream, encodedPayload)
		end
	end

	local function runProcessTests()
		if processTest == "" or processStage ~= "initial" then
			return
		end
		Common.runCustomTests("unsynced_gadget", ProcessTests, GeneratedTests, record, function(encoded)
			Spring.InvokeNativeModule(encoded)
		end, fixtureIDs)
	end

	function gadget:GameFrame(frame)
		if frame == 1 and processStage == "resume"
			and (processTest == "reload" or processTest == "restart")
		then
			local payload = {
				case = 1,
				called = true,
				returnCount = 0,
				reloaded = true,
			}
			record(processTest, payload)
			if Common.mode() == "native" then
				Spring.InvokeNativeModule(Common.encode(payload))
			end
			Spring.Quit()
			return
		end
		if frame == 4 then
			-- Depending on renderer startup timing, CreateGroundDecal can be
			-- unavailable during gadget Initialize.  Retry once the first game
			-- frame has been reached before deciding which decal tests can run.
			ensureGroundDecal()
			sendInventory()
			runGeneratedTests()
		end
		if frame == 20 then
			sendInventory()
			runGeneratedTests()
			runProcessTests()
			forward("unsynced_gadget", Common.encode({
				context = "unsynced_gadget",
				name = "game_frame",
				value = Spring.GetGameFrame(),
			}))
			if processTest == "reload" or processTest == "restart" then
				-- The process-control API is expected to reload the valid fixture
				-- script.  The resumed fixture quits at frame one.
			else
				Spring.SendCommands("quitforce")
			end
		end
	end

	return
end

local Common = VFS.Include("LuaRules/Utilities/native_api_parity_common.lua")
local GeneratedTests = VFS.Include("LuaRules/Utilities/generated_api_tests.lua")
local syncedParityOptions = Spring.GetModOptions() or {}
local syncedProcessStage = tostring(syncedParityOptions.native_api_parity_process_stage or "initial")

local function options()
	return Spring.GetModOptions() or {}
end

local rngState = tonumber(options().native_api_parity_seed) or 1
local caseCount = tonumber(options().native_api_parity_cases) or 1
local selectedTestsOption = tostring(options().native_api_parity_tests or "")
local selectedTestPrefix = tonumber(options().native_api_parity_test_prefix or "")
local selectedTests = {}
for testName in string.gmatch(selectedTestsOption, "[^,]+") do
	selectedTests[testName] = true
end

local function selectedTest(testName)
	return selectedTestsOption == "" and selectedTestPrefix == nil or selectedTests[testName] == true
end

local function rand01()
	rngState = (1103515245 * rngState + 12345) % 2147483648
	return rngState / 2147483648
end

local function randFloat(minValue, maxValue)
	return minValue + (maxValue - minValue) * rand01()
end

local function randInt(minValue, maxValue)
	return math.floor(randFloat(minValue, maxValue + 1))
end

local function rounded(value)
	return math.floor(value * 1000 + 0.5) / 1000
end

local function send(name, payload)
	Common.setTestName(payload, name)
	payload.context = "synced_gadget"
	local encoded = Common.encode(payload)
	SendToUnsynced("native_api_parity_result", "synced_gadget", encoded)
	if Common.mode() == "native" then
		Spring.InvokeNativeModule(encoded)
	end
end

local function sendInventory()
	local encoded = Common.encode({
		context = "synced_gadget",
		name = "context_inventory",
		functions = Common.springFunctionInventory(),
	})
	SendToUnsynced("native_api_parity_result", "synced_gadget", encoded)
end

local function requestNativeSet(name, payload, luaFallback)
	Common.setTestName(payload, name)
	payload.context = "synced_gadget"
	if Common.mode() == "native" then
		Spring.InvokeNativeModule(Common.encode(payload))
	elseif luaFallback then
		luaFallback()
	end
end

local function unitPayload(caseIndex, ids, values)
	values.case = caseIndex
	values.unitID = ids.unitID
	return values
end

local function featurePayload(caseIndex, ids, values)
	values.case = caseIndex
	values.featureID = ids.featureID
	-- Diagnostic fixture state: keep the object transform alongside piece
	-- queries so cross-process differences can be attributed to the feature
	-- itself versus LocalModelPiece emission data.  These fields are not part
	-- of any declared comparison contract.
	local x, y, z = Spring.GetFeaturePosition(ids.featureID)
	values.fixtureFeaturePosX = x
	values.fixtureFeaturePosY = y
	values.fixtureFeaturePosZ = z
	values.fixtureFeatureInputX = ids.featureInputX
	values.fixtureFeatureInputZ = ids.featureInputZ
	values.fixtureFeatureFacing = ids.featureFacing
	local frontX, frontY, frontZ = Spring.GetFeatureDirection(ids.featureID)
	values.fixtureFeatureFrontX = frontX
	values.fixtureFeatureFrontY = frontY
	values.fixtureFeatureFrontZ = frontZ
	return values
end

local function projectilePayload(caseIndex, ids, values)
	values.case = caseIndex
	values.projectileID = ids.projectileID
	return values
end

local function pieceProjectilePayload(caseIndex, ids, values)
	values.case = caseIndex
	values.projectileID = ids.pieceProjectileID
	values.isPiece = true
	return values
end

local function groundPayload(caseIndex, _, values)
	values.case = caseIndex
	return values
end

local function objectPayload(caseIndex, ids, values)
	values.case = caseIndex
	values.unitID = ids.unitID
	values.featureID = ids.featureID
	values.unitDefID = ids.unitDefID
	values.featureDefID = ids.featureDefID
	values.teamID = ids.teamID
	values.groundX = ids.groundX
	values.groundY = 96
	values.groundZ = ids.groundZ
	values.x = ids.groundX
	values.y = 96
	values.z = ids.groundZ
	return values
end

local function randomPosition()
	return {
		x = rounded(randFloat(880, 1180)),
		y = 96,
		z = rounded(randFloat(880, 1180)),
	}
end

local function randomGroundPoint()
	return {
		x = rounded(randFloat(880, 1180)),
		z = rounded(randFloat(880, 1180)),
	}
end

local function randomVelocity()
	return {
		x = rounded(randFloat(-2, 2)),
		y = rounded(randFloat(-0.25, 1)),
		z = rounded(randFloat(-2, 2)),
	}
end

local function randomFlatVelocity()
	return {
		x = rounded(randFloat(-2, 2)),
		y = 0,
		z = rounded(randFloat(-2, 2)),
	}
end

local function randomDirection()
	local angle = randFloat(0, math.pi * 2)
	local frontX = rounded(math.sin(angle))
	local frontZ = rounded(math.cos(angle))
	return {
		frontX = frontX,
		frontY = 0,
		frontZ = frontZ,
		rightX = rounded(math.cos(angle)),
		rightY = 0,
		rightZ = rounded(-math.sin(angle)),
	}
end

local function randomResource()
	if randInt(0, 1) == 0 then
		return "metal"
	end
	return "energy"
end

local function resolveLuaFunction(name)
	local tableName, functionName = name:match("^([^.]+)%.(.+)$")
	if tableName == nil or _G[tableName] == nil then
		error("cannot resolve Lua API function " .. tostring(name))
	end
	local func = _G[tableName][functionName]
	if type(func) ~= "function" then
		error("Lua API value is not a function: " .. tostring(name))
	end
	return func
end

local function generatedParamValue(param, ids)
	if param.fixed ~= nil then
		return param.fixed
	end
	if param.fixture then
		return ids[param.fixture]
	end
	if param.generator == "unit_circle" then
		local angle = randFloat(0, math.pi * 2)
		return { x = rounded(math.sin(angle)), z = rounded(math.cos(angle)) }
	end
	if param.generator == "map_position" then
		return randomPosition()
	end
	if param.generator == "map_point" then
		return randomGroundPoint()
	end
	if param.generator == "unit_velocity" then
		return randomVelocity()
	end
	if param.generator == "flat_velocity" then
		return randomFlatVelocity()
	end
	if param.generator == "unit_orientation" then
		return randomDirection()
	end
	if param.type == "bool" then
		return randInt(0, 1) == 1
	end
	if param.type == "enum" then
		return param.values[randInt(1, #param.values)]
	end
	if param.type == "string" then
		return param.value or ""
	end
	if param.type == "i32" then
		return randInt(param.range[1], param.range[2])
	end
	if param.type == "f32" then
		return rounded(randFloat(param.range[1], param.range[2]))
	end
	return nil
end

local function generatedMake(test)
	return function(ids)
		local values = {}
		for name, param in pairs(test.params or {}) do
			local value = generatedParamValue(param, ids)
			if param.expands_to and type(value) == "table" then
				for index, field in ipairs(param.expands_to) do
					local sourceField = param.expands_from and param.expands_from[index] or field
					values[field] = value[sourceField]
				end
			else
				values[name] = value
			end
		end
		return values
	end
end

local function generatedArg(spec, ids, value)
	if type(spec) == "table" then
		local resolved = {}
		for key, item in pairs(spec) do
			resolved[key] = generatedArg(item, ids, value)
		end
		return resolved
	end
	if type(spec) ~= "string" then
		return spec
	end
	if spec == "nil" then
		return nil
	end
	local fixtureKey = spec:match("^fixture%.(.+)$")
	if fixtureKey then
		return ids[fixtureKey]
	end
	local valueKey = spec:match("^value%.(.+)$")
	if valueKey then
		return value[valueKey]
	end
	return value[spec]
end

local function generatedReturnValue(returnSpec, returns)
	if type(returnSpec) == "string" then
		return returnSpec, nil
	end

	local field = returnSpec.field
	local value = returns[returnSpec.index or 1]
	for _, key in ipairs(returnSpec.path or {}) do
		if type(value) ~= "table" then
			return field, nil
		end
		value = value[key]
	end
	if returnSpec.transform == "table_keys" then
		local keys = {}
		if type(value) == "table" then
			for key in pairs(value) do
				keys[#keys + 1] = tostring(key)
			end
			table.sort(keys)
		end
		value = keys
	elseif returnSpec.transform == "table_string_int_pairs" then
		local pairsList = {}
		if type(value) == "table" then
			for key, item in pairs(value) do
				if type(key) == "string" and type(item) == "number" then
					pairsList[#pairsList + 1] = { name = key, pieceNum = item }
				end
			end
			table.sort(pairsList, function(a, b) return a.name < b.name end)
		end
		value = pairsList
	elseif returnSpec.transform == "table_int_keys" then
		local keys = {}
		if type(value) == "table" then
			for key in pairs(value) do
				if type(key) == "number" then
					keys[#keys + 1] = key
				end
			end
			table.sort(keys)
		end
		value = keys
	elseif returnSpec.transform == "table_count" then
		local count = 0
		if type(value) == "table" then
			for key in pairs(value) do
				if type(key) == "number" then
					count = count + 1
				end
			end
		end
		value = count
	elseif returnSpec.transform == "table_int_values" then
		local values = {}
		if type(value) == "table" then
			for _, item in ipairs(value) do
				values[#values + 1] = item
			end
		end
		value = values
	elseif returnSpec.transform == "table_nonempty" then
		local nonempty = false
		if type(value) == "table" then
			for _ in pairs(value) do
				nonempty = true
				break
			end
		end
		value = nonempty
	elseif returnSpec.transform == "return_count" then
		value = returns.n or 0
	elseif returnSpec.transform == "truthy" then
		value = value ~= nil and value ~= false
	elseif returnSpec.transform == "valid_id" then
		value = type(value) == "number" and value >= 0
	elseif returnSpec.transform == "string_len" then
		value = type(value) == "string" and #value or 0
	elseif returnSpec.transform == "nil_to_minus_one" then
		if value == nil then
			value = -1
		end
	elseif returnSpec.transform == "false_to_minus_one" then
		if value == nil or value == false then
			value = -1
		end
	elseif returnSpec.transform == "build_status_can_build" then
		value = type(value) == "number" and value >= 2
	elseif returnSpec.transform == "nested_unit_ids" then
		local unitIDs = {}
		if type(value) == "table" then
			for _, units in pairs(value) do
				if type(units) == "table" then
					for _, unitID in ipairs(units) do
						unitIDs[#unitIDs + 1] = unitID
					end
				end
			end
			table.sort(unitIDs)
		end
		value = unitIDs
	elseif returnSpec.transform == "unit_def_unit_groups" then
		local groups = {}
		if type(value) == "table" then
			for unitDefID, units in pairs(value) do
				if unitDefID ~= "n" and type(units) == "table" then
					local unitIDs = {}
					for _, unitID in ipairs(units) do
						unitIDs[#unitIDs + 1] = unitID
					end
					table.sort(unitIDs)
					groups[#groups + 1] = {
						unitDefID = tonumber(unitDefID),
						unitIDs = unitIDs,
					}
				end
			end
			table.sort(groups, function(a, b)
				return a.unitDefID < b.unitDefID
			end)
		end
		value = groups
	elseif returnSpec.transform == "unit_def_counts" then
		local counts = {}
		if type(value) == "table" then
			for unitDefID, count in pairs(value) do
				if unitDefID ~= "n" then
					counts[#counts + 1] = {
						unitDefID = tonumber(unitDefID),
						count = count,
					}
				end
			end
			table.sort(counts, function(a, b)
				return a.unitDefID < b.unitDefID
			end)
		end
		value = counts
	elseif returnSpec.transform == "start_positions" then
		local positions = {}
		if type(value) == "table" then
			for teamID, pos in pairs(value) do
				if type(pos) == "table" then
					positions[#positions + 1] = {
						teamID = tonumber(teamID),
						x = pos[1],
						y = pos[2],
						z = pos[3],
					}
				end
			end
			table.sort(positions, function(a, b)
				return a.teamID < b.teamID
			end)
		end
		value = positions
	end
	return field, value
end

local function packReturns(...)
	local packed = { n = select("#", ...) }
	for index = 1, packed.n do
		packed[index] = select(index, ...)
	end
	return packed
end

local function invokeGeneratedRuntime(runtime, ids, value)
	if runtime.table then
		local tableValue = _G[runtime.table]
		if type(tableValue) ~= "table" then
			error("Lua API value is not a table: " .. tostring(runtime.table))
		end
		local key = generatedArg(runtime.key, ids, value)
		if key ~= nil then
			tableValue = tableValue[key]
		end
		return { tableValue }
	end

	local func = resolveLuaFunction(runtime.call)
	local args = {}
	local argCount = 0
	for _, argSpec in ipairs(runtime.args or {}) do
		argCount = argCount + 1
		args[argCount] = generatedArg(argSpec, ids, value)
	end
	return packReturns(func(unpack(args, 1, argCount)))
end

local function appendGeneratedReadback(readback, runtime, returns)
	for index, returnSpec in ipairs(runtime.returns or {}) do
		local field, returnValue = generatedReturnValue(returnSpec, returns)
		if type(returnSpec) == "string" then
			returnValue = returns[index]
		end
		readback[field] = returnValue
	end
end

local function hasOnlyReturnCountResults(runtime)
	local returns = runtime.returns
	if type(returns) ~= "table" or #returns == 0 or #(runtime.post or {}) ~= 0 then
		return false
	end

	for _, returnSpec in ipairs(returns) do
		if type(returnSpec) ~= "table" or returnSpec.transform ~= "return_count" then
			return false
		end
	end
	return true
end

local function generatedGet(test)
	return function(ids, value)
		local runtime = test.lua_runtime
		local returns
		if test.native_only and Common.mode() == "native" then
			-- Native-only mutators are applied by the native setter callback
			-- before this readback.  Do not invoke their Lua call a second time.
			returns = { n = 0 }
		elseif Common.mode() == "native" and test.readonly and hasOnlyReturnCountResults(runtime) then
			-- Write-only Lua controls are represented as readonly parity tests
			-- because their generated readback is just the Lua return count.  The
			-- native check invokes the Rust control call separately; invoking the
			-- Lua control here as well would apply the side effect twice in native
			-- mode (and can consume synced RNG or otherwise change later checks).
			returns = { n = 0 }
		else
			returns = invokeGeneratedRuntime(runtime, ids, value)
		end
		local readback = {}
		appendGeneratedReadback(readback, runtime, returns)
		for _, postRuntime in ipairs(runtime.post or {}) do
			appendGeneratedReadback(readback, postRuntime, invokeGeneratedRuntime(postRuntime, ids, value))
		end
		for _, field in ipairs(test.order_insensitive_fields or {}) do
			if type(readback[field]) == "table" then
				table.sort(readback[field])
			end
		end
		return readback
	end
end

local function generatedSet(test)
	return function(ids, value)
		local runtime = test.lua_runtime
		local setters = runtime.set
		if setters == nil then
			return
		end

		if setters.call ~= nil then
			setters = { setters }
		end

		for _, setter in ipairs(setters) do
			local func = resolveLuaFunction(setter.call)
			local args = {}
			local argCount = 0
			for _, argSpec in ipairs(setter.args or {}) do
				argCount = argCount + 1
				args[argCount] = generatedArg(argSpec, ids, value)
			end
			func(unpack(args, 1, argCount))
		end
	end
end

local Fixture = {}

function Fixture.create()
	local teamID = 0
	local baseX = randFloat(880, 1180)
	local baseZ = randFloat(880, 1180)
	local unitID = Spring.CreateUnit("native_api_test_unit", baseX, 96, baseZ, randInt(0, 3), teamID, false, false)
	-- Lua unit scripts are created lazily.  The piece/script parity checks need
	-- the same CLuaUnitScript piece map that a real script-backed unit has.
	if unitID and Spring.UnitScript and Spring.UnitScript.CreateScript then
		Spring.UnitScript.CreateScript(unitID, {})
	end
	local featureInputX = baseX + randFloat(24, 80)
	local featureInputZ = baseZ + randFloat(24, 80)
	local featureFacing = randInt(0, 3)
	local featureID = Spring.CreateFeature("native_api_test_feature", featureInputX, 96, featureInputZ, featureFacing, teamID)
	local unitDefID = Spring.GetUnitDefID(unitID)
	local unitDef = UnitDefs[unitDefID]
	local weaponEntry = unitDef and unitDef.weapons and (unitDef.weapons[1] or unitDef.weapons[0])
	local weaponDefID = weaponEntry and weaponEntry.weaponDef
	local projectileID = weaponDefID and Spring.SpawnProjectile(weaponDefID, {
		pos = {baseX, 96, baseZ},
		speed = {0, 0, 0},
		["end"] = {baseX + 128, 96, baseZ + 128},
		owner = unitID,
		team = teamID,
		ttl = 10000,
		gravity = 0,
	})
	local existingProjectiles = {}
	for _, id in ipairs(Spring.GetAllProjectiles(false, false) or {}) do
		existingProjectiles[id] = true
	end
	local pieceProjectileID
	if unitID and Spring.UnitScript and Spring.UnitScript.CallAsUnit and SFX then
		Spring.UnitScript.CallAsUnit(unitID, function()
			Spring.UnitScript.Explode(1, SFX.EXPLODE + SFX.NO_CEG_TRAIL)
		end)
		for _, id in ipairs(Spring.GetAllProjectiles(false, false) or {}) do
			if not existingProjectiles[id] then
				local isWeapon, isPiece = Spring.GetProjectileType(id)
				if isPiece and not isWeapon then
					pieceProjectileID = id
					break
				end
			end
		end
	end
	return {
		teamID = teamID,
		allyTeamID = 0,
		unitID = unitID,
		unitDefID = unitDefID,
		featureID = featureID,
		featureInputX = rounded(featureInputX),
		featureInputZ = rounded(featureInputZ),
		featureFacing = featureFacing,
		featureDefID = Spring.GetFeatureDefID(featureID),
		weaponDefID = weaponDefID,
		projectileID = projectileID,
		pieceProjectileID = pieceProjectileID,
		groundX = rounded(baseX),
		groundZ = rounded(baseZ),
	}
end

function Fixture.destroy(ids)
	if ids.unitID then
		for _, id in ipairs(Spring.GetAllProjectiles(false, false) or {}) do
			if Spring.GetProjectileOwnerID(id) == ids.unitID then
				Spring.DeleteProjectile(id)
			end
		end
	end
	if ids.unitID and (not Spring.ValidUnitID or Spring.ValidUnitID(ids.unitID)) then
		-- Test fixtures are owned by this case.  Use Lua's immediate-cleanup
		-- option so the next randomized case cannot observe a dead fixture.
		Spring.DestroyUnit(ids.unitID, false, true, nil, true)
	end
	if ids.featureID and (not Spring.ValidFeatureID or Spring.ValidFeatureID(ids.featureID)) then
		Spring.DestroyFeature(ids.featureID)
	end
end

local CustomHooks = VFS.Include("LuaRules/Utilities/native_api_parity_custom_hooks.lua")
local TEST_HOOKS = CustomHooks({
	randFloat = randFloat,
	randInt = randInt,
	rounded = rounded,
	randomPosition = randomPosition,
	randomGroundPoint = randomGroundPoint,
	randomVelocity = randomVelocity,
	randomFlatVelocity = randomFlatVelocity,
	randomDirection = randomDirection,
	randomResource = randomResource,
	unitPayload = unitPayload,
	featurePayload = featurePayload,
	groundPayload = groundPayload,
	objectPayload = objectPayload,
})

local function buildGeneratedTests(hooks)
	local hookByID = {}
	local seen = {}
	local function hasUnsupportedRequirement(metadata)
		for _, requirement in ipairs(metadata.requires or {}) do
			if requirement ~= "unit" and requirement ~= "feature" and requirement ~= "projectile" and requirement ~= "piece_projectile" and requirement ~= "ground_point" then
				return true
			end
		end
		return false
	end
	for _, hook in ipairs(hooks) do
		hookByID[hook.name] = hook
	end

	local tests = {}
	for index, metadata in ipairs(GeneratedTests) do
		if selectedTestPrefix ~= nil and index <= selectedTestPrefix then
			selectedTests[metadata.id] = true
		end
		local context = metadata.context or "synced_gadget"
		if not selectedTest(metadata.id) then
			seen[metadata.id] = true
		elseif context ~= "synced_gadget" then
			seen[metadata.id] = true
		elseif metadata.requires_rendering and not Common.enableRenderingTests() then
			seen[metadata.id] = true
		elseif hasUnsupportedRequirement(metadata) then
			seen[metadata.id] = true
		else
			local hook = hookByID[metadata.id]

			local test = {}
			for key, value in pairs(metadata) do
				test[key] = value
			end
			test.name = metadata.id
			if hook ~= nil then
				for key, value in pairs(hook) do
					test[key] = value
				end
			elseif metadata.lua_runtime ~= nil then
				test.readonly = metadata.kind == "readonly"
				if metadata.requires and metadata.requires[1] == "unit" then
					test.payload = unitPayload
				elseif metadata.requires and metadata.requires[1] == "feature" then
					test.payload = featurePayload
				elseif metadata.requires and metadata.requires[1] == "projectile" then
					test.payload = projectilePayload
				elseif metadata.requires and metadata.requires[1] == "piece_projectile" then
					test.payload = pieceProjectilePayload
				else
					test.payload = groundPayload
				end
				test.make = generatedMake(metadata)
				test.get = generatedGet(test)
				test.set = generatedSet(test)
			else
				error("generated parity metadata has no Lua runtime hook for " .. metadata.id)
			end
			tests[#tests + 1] = test
			seen[metadata.id] = true
		end
	end

	for _, hook in ipairs(hooks) do
		if selectedTest(hook.name) and not seen[hook.name] then
			error("Lua runtime hook has no generated parity metadata for " .. hook.name)
		end
	end

	return tests
end

local TESTS = buildGeneratedTests(TEST_HOOKS)
local persistentFixture
local ranDeferredSyncedChecks = false

local function mergePayload(caseIndex, ids, test, value, readback)
	local payload = test.payload(caseIndex, ids, readback)
	for key, fieldValue in pairs(value) do
		if payload[key] == nil then
			payload[key] = fieldValue
		end
	end
	return payload
end

local function runOneTest(test, ids, caseIndex)
	if test.native_only then
		local value = test.make(ids)
		if Common.mode() == "native" then
			local payload = test.payload(caseIndex, ids, value)
			requestNativeSet("set_native_" .. test.name, payload)
		end
		-- In Lua mode this invokes the Lua mutator.  In native mode generatedGet
		-- returns its declared zero-return shape after the native setter above.
		send(test.name, mergePayload(caseIndex, ids, test, value, test.get(ids, value)))
		return
	end

	-- A few synced controls are destructive or one-shot (for example
	-- KillTeam and GameOver).  In the native run, invoke the native operation
	-- before the Lua readback instead of first invoking the Lua operation and
	-- accidentally masking the native state transition.  The Lua baseline
	-- still exercises the Lua call, while the native baseline exercises the
	-- corresponding Rust call and emits the same pair of comparison rows.
	if Common.mode() == "native" and test.nativeFirst then
		local nativeValue = test.make(ids)
		local function invokeNativeSet()
			requestNativeSet("set_native_" .. test.name, test.payload(caseIndex, ids, nativeValue), function()
				test.set(ids, nativeValue)
			end)
		end
		if test.nativeSet == nil then
			error("native-first parity hook has no native setter for " .. test.name)
		end
		test.nativeSet(ids, nativeValue, invokeNativeSet)
		-- requestNativeSet annotates the transport object in-place.  Do not let
		-- that annotation become the ordinary comparison row's display name.
		nativeValue.name = nil
		nativeValue.testName = nil
		local readback = test.get(ids, nativeValue)
		local payload = mergePayload(caseIndex, ids, test, nativeValue, readback)
		send(test.name, payload)
		local nativePayload = mergePayload(caseIndex, ids, test, nativeValue, readback)
		nativePayload.name = "set_native_" .. test.name
		send("native_" .. test.name, nativePayload)
		return
	end

	local luaValue = test.make(ids)
	if test.readonly then
		local ok, readback = pcall(test.get, ids, luaValue)
		if ok then
			send(test.name, mergePayload(caseIndex, ids, test, luaValue, readback))
		elseif test.expect_error then
			send(test.name, mergePayload(caseIndex, ids, test, luaValue, { error = true }))
		else
			error(readback)
		end
		return
	end

	test.set(ids, luaValue)
	send(test.name, mergePayload(caseIndex, ids, test, luaValue, test.get(ids, luaValue)))

	local nativeValue = test.make(ids)
	local function invokeNativeSet()
		requestNativeSet("set_native_" .. test.name, test.payload(caseIndex, ids, nativeValue), function()
			test.set(ids, nativeValue)
		end)
	end
	if Common.mode() == "native" and test.nativeSet ~= nil then
		test.nativeSet(ids, nativeValue, invokeNativeSet)
	else
		invokeNativeSet()
	end
	send("native_" .. test.name, mergePayload(caseIndex, ids, test, nativeValue, test.get(ids, nativeValue)))
end

local function runSyncedChecks()
	sendInventory()

	for caseIndex = 1, caseCount do
		local ids = Fixture.create()

		for _, test in ipairs(TESTS) do
			if not test.deferred then
				runOneTest(test, ids, caseIndex)
			end
		end

		Fixture.destroy(ids)
	end

end

local function runDeferredSyncedChecks()
	if ranDeferredSyncedChecks then
		return
	end
	ranDeferredSyncedChecks = true

	for caseIndex = 1, caseCount do
		local ids = Fixture.create()
		for _, test in ipairs(TESTS) do
			if test.deferred and (not test.singleCase or caseIndex == caseCount) then
				runOneTest(test, ids, caseIndex)
			end
		end
		Fixture.destroy(ids)
	end

	send("complete", {})
end

function gadget:GameFrame(frame)
	if syncedProcessStage == "resume" then
		return
	end
	if frame == 1 then
		persistentFixture = Fixture.create()
		SendToUnsynced(
			"native_api_parity_fixture",
			persistentFixture.unitID,
			persistentFixture.featureID,
			persistentFixture.unitDefID,
			persistentFixture.featureDefID,
			persistentFixture.weaponDefID,
			persistentFixture.projectileID,
			persistentFixture.pieceProjectileID,
			persistentFixture.teamID,
			persistentFixture.allyTeamID
		)
	end
	if frame == 2 then
		runSyncedChecks()
	end
	if frame == 18 then
		runDeferredSyncedChecks()
	end
	if frame == 20 and persistentFixture ~= nil then
		Fixture.destroy(persistentFixture)
		persistentFixture = nil
	end
end
