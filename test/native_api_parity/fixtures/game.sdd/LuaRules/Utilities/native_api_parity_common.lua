local M = {}

local rngState = 1

function M.encode(value)
	local t = type(value)
	if t == "nil" then
		return "null"
	elseif t == "boolean" or t == "number" then
		return tostring(value)
	elseif t == "string" then
		return string.format("%q", value)
	end

	local parts = {}
	local isArray = true
	local n = 0
	for k in pairs(value) do
		if type(k) ~= "number" then
			isArray = false
			break
		end
		n = math.max(n, k)
	end

	if isArray then
		for i = 1, n do
			parts[#parts + 1] = M.encode(value[i])
		end
		return "[" .. table.concat(parts, ",") .. "]"
	end

	for k, v in pairs(value) do
		parts[#parts + 1] = M.encode(tostring(k)) .. ":" .. M.encode(v)
	end
	return "{" .. table.concat(parts, ",") .. "}"
end

function M.appendJsonLine(path, row)
	local file, err = io.open(path, "a")
	if not file then
		Spring.Echo("native_api_parity: failed to open " .. path .. ": " .. tostring(err))
		return false
	end
	file:write(M.encode(row))
	file:write("\n")
	file:close()
	return true
end

function M.springFunctionInventory()
	local names = {}
	for name, value in pairs(Spring) do
		if type(value) == "function" then
			names[#names + 1] = name
		end
	end
	table.sort(names)
	return names
end

function M.initRandom(seed, offset)
	rngState = ((tonumber(seed) or 1) + (tonumber(offset) or 0)) % 2147483648
	if rngState == 0 then
		rngState = 1
	end
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

local function options()
	return Spring.GetModOptions() or {}
end

function M.caseCount()
	return tonumber(options().native_api_parity_cases) or 1
end

function M.seed()
	return tonumber(options().native_api_parity_seed) or 1
end

function M.fixtureIDs()
	return {
		teamID = Spring.GetMyTeamID and Spring.GetMyTeamID() or 0,
		allyTeamID = Spring.GetMyAllyTeamID and Spring.GetMyAllyTeamID() or 0,
		playerID = Spring.GetMyPlayerID and Spring.GetMyPlayerID() or 0,
	}
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

local function generatedMake(test, ids)
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
	elseif returnSpec.transform == "truthy" then
		value = value ~= nil and value ~= false
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

local function generatedGet(test, ids, value)
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

local function canRunPortableReadOnly(test)
	if test.kind ~= "readonly" or test.lua_runtime == nil or test.requires_rendering then
		return false
	end
	if #(test.requires or {}) ~= 0 then
		return false
	end
	if test.lua_runtime.call then
		local tableName, functionName = test.lua_runtime.call:match("^([^.]+)%.(.+)$")
		if tableName == "Spring" and type(Spring[functionName]) ~= "function" then
			return false
		end
	end
	if test.lua_runtime.table and type(_G[test.lua_runtime.table]) ~= "table" then
		return false
	end
	return true
end

function M.runPortableReadOnlyTests(context, generatedTests, record, invokeNative)
	M.initRandom(M.seed(), context == "widget" and 20 or 10)
	local ids = M.fixtureIDs()
	for caseIndex = 1, M.caseCount() do
		for _, test in ipairs(generatedTests) do
			if canRunPortableReadOnly(test) then
				local value = generatedMake(test, ids)
				local ok, readback = pcall(generatedGet, test, ids, value)
				if ok then
					local payload = { case = caseIndex }
					for key, fieldValue in pairs(value) do
						payload[key] = fieldValue
					end
					for key, fieldValue in pairs(readback) do
						payload[key] = fieldValue
					end
					record(test.id, payload)
					if M.mode() == "native" and invokeNative then
						payload.context = context
						payload.name = test.id
						invokeNative(M.encode(payload))
					end
				end
			end
		end
	end
end

function M.outputDir()
	local opts = Spring.GetModOptions() or {}
	return opts.native_api_parity_output_dir or "native_api_parity"
end

function M.mode()
	local opts = Spring.GetModOptions() or {}
	return opts.native_api_parity_mode or "lua"
end

function M.enableRenderingTests()
	local opts = Spring.GetModOptions() or {}
	return tostring(opts.native_api_parity_enable_rendering_tests or "0") == "1"
end

return M
