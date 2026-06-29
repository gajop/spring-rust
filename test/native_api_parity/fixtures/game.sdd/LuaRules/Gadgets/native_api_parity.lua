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
		payload.name = name
		forward("unsynced_gadget", Common.encode(payload))
	end

	local function runGeneratedTests()
		if ranGeneratedTests then
			return
		end
		ranGeneratedTests = true
		Common.runPortableReadOnlyTests("unsynced_gadget", GeneratedTests, record, function(encoded)
			Spring.InvokeNativeModule(encoded)
		end, fixtureIDs)
	end

	function gadget:Initialize()
		forward("unsynced_gadget", Common.encode({
			context = "unsynced_gadget",
			name = "game_frame_initial",
			value = Spring.GetGameFrame(),
		}))
	end

	function gadget:RecvFromSynced(name, ...)
		if name == "native_api_parity_fixture" then
			local unitID, featureID, unitDefID, featureDefID, weaponDefID, teamID, allyTeamID = ...
			fixtureIDs = {
				unitID = unitID,
				featureID = featureID,
				unitDefID = unitDefID,
				featureDefID = featureDefID,
				weaponDefID = weaponDefID,
				teamID = teamID,
				allyTeamID = allyTeamID,
			}
			if Script.LuaUI.NativeApiParityFixture then
				Script.LuaUI.NativeApiParityFixture(unitID, featureID, unitDefID, featureDefID, weaponDefID, teamID, allyTeamID)
			end
			return
		elseif name == "native_api_parity_result" then
			local stream, encodedPayload = ...
			forward(stream, encodedPayload)
		end
	end

	function gadget:GameFrame(frame)
		if frame == 4 then
			sendInventory()
			runGeneratedTests()
		end
		if frame == 20 then
			sendInventory()
			runGeneratedTests()
			forward("unsynced_gadget", Common.encode({
				context = "unsynced_gadget",
				name = "game_frame",
				value = Spring.GetGameFrame(),
			}))
			Spring.SendCommands("quitforce")
		end
	end

	return
end

local Common = VFS.Include("LuaRules/Utilities/native_api_parity_common.lua")
local GeneratedTests = VFS.Include("LuaRules/Utilities/generated_api_tests.lua")

local function options()
	return Spring.GetModOptions() or {}
end

local rngState = tonumber(options().native_api_parity_seed) or 1
local caseCount = tonumber(options().native_api_parity_cases) or 1

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
	payload.name = name
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
	payload.name = name
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
	return values
end

local function groundPayload(caseIndex, _, values)
	values.case = caseIndex
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
				for _, field in ipairs(param.expands_to) do
					values[field] = value[field]
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
	elseif returnSpec.transform == "table_nonempty" then
		local nonempty = false
		if type(value) == "table" then
			for _ in pairs(value) do
				nonempty = true
				break
			end
		end
		value = nonempty
	elseif returnSpec.transform == "truthy" then
		value = value ~= nil and value ~= false
	elseif returnSpec.transform == "string_len" then
		value = type(value) == "string" and #value or 0
	elseif returnSpec.transform == "nil_to_minus_one" then
		if value == nil then
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

local function generatedGet(test)
	return function(ids, value)
		local runtime = test.lua_runtime
		local returns
		if runtime.table then
			local tableValue = _G[runtime.table]
			if type(tableValue) ~= "table" then
				error("Lua API value is not a table: " .. tostring(runtime.table))
			end
			local key = generatedArg(runtime.key, ids, value)
			if key ~= nil then
				tableValue = tableValue[key]
			end
			returns = { tableValue }
		else
			local func = resolveLuaFunction(runtime.call)
			local args = {}
			for _, argSpec in ipairs(runtime.args or {}) do
				args[#args + 1] = generatedArg(argSpec, ids, value)
			end
			returns = { func(unpack(args)) }
		end
		local readback = {}
		for index, returnSpec in ipairs(runtime.returns or {}) do
			local field, returnValue = generatedReturnValue(returnSpec, returns)
			if type(returnSpec) == "string" then
				returnValue = returns[index]
			end
			readback[field] = returnValue
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
			for _, argSpec in ipairs(setter.args or {}) do
				args[#args + 1] = generatedArg(argSpec, ids, value)
			end
			func(unpack(args))
		end
	end
end

local Fixture = {}

function Fixture.create()
	local teamID = 0
	local baseX = randFloat(880, 1180)
	local baseZ = randFloat(880, 1180)
	local unitID = Spring.CreateUnit("native_api_test_unit", baseX, 96, baseZ, randInt(0, 3), teamID, false, false)
	local featureID = Spring.CreateFeature("native_api_test_feature", baseX + randFloat(24, 80), 96, baseZ + randFloat(24, 80), randInt(0, 3), teamID)
	local unitDefID = Spring.GetUnitDefID(unitID)
	local unitDef = UnitDefs[unitDefID]
	local weaponEntry = unitDef and unitDef.weapons and (unitDef.weapons[1] or unitDef.weapons[0])
	local weaponDefID = weaponEntry and weaponEntry.weaponDef
	return {
		teamID = teamID,
		allyTeamID = 0,
		unitID = unitID,
		unitDefID = unitDefID,
		featureID = featureID,
		featureDefID = Spring.GetFeatureDefID(featureID),
		weaponDefID = weaponDefID,
		groundX = rounded(baseX),
		groundZ = rounded(baseZ),
	}
end

function Fixture.destroy(ids)
	if ids.unitID and (not Spring.ValidUnitID or Spring.ValidUnitID(ids.unitID)) then
		Spring.DestroyUnit(ids.unitID, false, true)
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
})

local function buildGeneratedTests(hooks)
	local hookByID = {}
	local seen = {}
	local function hasUnsupportedRequirement(metadata)
		for _, requirement in ipairs(metadata.requires or {}) do
			if requirement ~= "unit" and requirement ~= "feature" and requirement ~= "ground_point" then
				return true
			end
		end
		return false
	end
	for _, hook in ipairs(hooks) do
		hookByID[hook.name] = hook
	end

	local tests = {}
	for _, metadata in ipairs(GeneratedTests) do
		local context = metadata.context or "synced_gadget"
		if context ~= "synced_gadget" then
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
				else
					test.payload = groundPayload
				end
				test.make = generatedMake(metadata)
				test.get = generatedGet(metadata)
				test.set = generatedSet(metadata)
			else
				error("generated parity metadata has no Lua runtime hook for " .. metadata.id)
			end
			tests[#tests + 1] = test
			seen[metadata.id] = true
		end
	end

	for _, hook in ipairs(hooks) do
		if not seen[hook.name] then
			error("Lua runtime hook has no generated parity metadata for " .. hook.name)
		end
	end

	return tests
end

local TESTS = buildGeneratedTests(TEST_HOOKS)
local persistentFixture

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
	local luaValue = test.make(ids)
	if test.readonly then
		send(test.name, mergePayload(caseIndex, ids, test, luaValue, test.get(ids, luaValue)))
		return
	end

	test.set(ids, luaValue)
	send(test.name, mergePayload(caseIndex, ids, test, luaValue, test.get(ids, luaValue)))

	local nativeValue = test.make(ids)
	requestNativeSet("set_native_" .. test.name, test.payload(caseIndex, ids, nativeValue), function()
		test.set(ids, nativeValue)
	end)
	send("native_" .. test.name, mergePayload(caseIndex, ids, test, nativeValue, test.get(ids, nativeValue)))
end

local function runSyncedChecks()
	sendInventory()

	for caseIndex = 1, caseCount do
		local ids = Fixture.create()

		for _, test in ipairs(TESTS) do
			runOneTest(test, ids, caseIndex)
		end

		Fixture.destroy(ids)
	end

	send("complete", {})
end

function gadget:GameFrame(frame)
	if frame == 1 then
		persistentFixture = Fixture.create()
		SendToUnsynced(
			"native_api_parity_fixture",
			persistentFixture.unitID,
			persistentFixture.featureID,
			persistentFixture.unitDefID,
			persistentFixture.featureDefID,
			persistentFixture.weaponDefID,
			persistentFixture.teamID,
			persistentFixture.allyTeamID
		)
	end
	if frame == 2 then
		runSyncedChecks()
	end
	if frame == 20 and persistentFixture ~= nil then
		Fixture.destroy(persistentFixture)
		persistentFixture = nil
	end
end
