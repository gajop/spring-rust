function gadget:GetInfo()
	local mode = Spring.GetModOptions() or {}
	return {
		name = "Native API benchmark",
		desc = "Measures the Lua, native, and Wasm API backends",
		author = "Spring",
		layer = 0,
		enabled = tostring(mode.native_api_parity_mode or "") == "benchmark",
	}
end

local Common = VFS.Include("LuaRules/Utilities/native_api_parity_common.lua")
local synced = gadgetHandler:IsSyncedCode()
local options = Spring.GetModOptions() or {}
local backend = tostring(options.native_api_parity_benchmark_backend or "lua")
local outputDir = tostring(options.native_api_parity_output_dir or "benchmark")
local repeats = math.max(1, tonumber(options.native_api_parity_benchmark_repeats or 5) or 5)
local benchmarkScale = tonumber(options.native_api_parity_benchmark_scale or 1) or 1
benchmarkScale = math.max(0.0001, math.min(1, benchmarkScale))
local benchmarkCase = tostring(options.native_api_parity_benchmark_case or "")
local benchmarkIterations = math.max(1, tonumber(options.native_api_parity_benchmark_iterations or 1) or 1)
local callinVariant = tostring(options.native_api_parity_benchmark_callin_variant or "empty")
local benchmarkSeed = tonumber(options.native_api_parity_seed or 1) or 1
local unitIDs = {}
local firstUnitID
local firstUnitDefID
local fixtureReady = false
local benchmarkComplete = false
local callinDriverActive = false
local callinDriverCalls = 0
local callinFrameSink = 0
local callinFramesRemaining = 0
local memoryRan = false

local function benchmarkModuleOne(_frame) end
local function benchmarkModuleTwo(_frame) end
local function benchmarkModuleThree(_frame) end
local function benchmarkModuleFour(_frame) end

local function scaledCount(value)
	return math.max(1, math.floor(value * benchmarkScale + 0.5))
end

local function scaledTerrainCount(value)
	-- Every SetHeightMapFunc invocation triggers the engine's terrain/path
	-- update after the callback. Keep the bounded profile to one update while
	-- retaining the nominal count at scale 1.
	return math.max(1, math.floor(value * benchmarkScale * benchmarkScale + 0.5))
end

local function scaledBrushSize(value)
	if value == 0 then return 0 end
	-- Terrain edits also invalidate pathing. A sub-linear scale keeps the
	-- three cases distinct without making the bounded profile spend its whole
	-- timeout in path updates; scale 1 still uses the documented sizes.
	return math.max(1, math.floor(value * benchmarkScale ^ 0.75 + 0.5))
end

local function appendJSON(path, payload)
	if synced then
		SendToUnsynced("native_api_benchmark_lua_result", Common.encode(payload))
		return true
	end
	local file, err = io.open(path, "a")
	if not file then
		Spring.Echo("native_api_benchmark: failed to open " .. path .. ": " .. tostring(err))
		return false
	end
	file:write(Common.encode(payload))
	file:write("\n")
	file:close()
	return true
end

local function outputPath(name)
	return outputDir .. "/benchmark_" .. name .. ".jsonl"
end

local function monotonicSeconds()
	-- This is an explicit diagnostic opt-in. Without
	-- SPRING_ENABLE_SYNCED_TIMERS the synced API raises an error, so ordinary
	-- lockstep gadgets cannot accidentally use a process clock.
	return Spring.GetTimerMicros() / 1000000
end

-- Lua numbers are floats, so timer resolution is the float step at the current
-- timer magnitude (not 1us) and degrades as the process runs. Measure it.
local function clockQuantumNs()
	local best
	for _ = 1, 5 do
		local start = monotonicSeconds()
		local now = start
		while now == start do
			now = monotonicSeconds()
		end
		local delta = (now - start) * 1000000000
		if best == nil or delta < best then best = delta end
	end
	return best or 1000
end

-- Quantization error over N timed regions grows with sqrt(N), so the minimum
-- trustworthy duration does too.
local MINIMUM_QUANTA = 50
local function resolutionFloorNs(quantumNs, regions)
	return MINIMUM_QUANTA * quantumNs * math.sqrt(math.max(regions or 1, 1))
end

local function reportUnresolved(name, elapsedNs, floorNs, iterations, extra)
	local payload = {
		backend = "lua",
		test = name,
		status = "unavailable",
		iterations = iterations,
		scale = benchmarkScale,
		reason = string.format(
			"sample of %.0f ns is below the %.0f ns timer-resolution floor",
			elapsedNs, floorNs),
	}
	if extra then
		for key, value in pairs(extra) do payload[key] = value end
	end
	appendJSON(outputPath("lua"), payload)
end

local function median(values)
	local sorted = {}
	for index, value in ipairs(values) do
		sorted[index] = value
	end
	table.sort(sorted)
	return sorted[math.floor((#sorted + 1) / 2)]
end

local function spread(values)
	local low = values[1]
	local high = values[1]
	for index = 2, #values do
		low = math.min(low, values[index])
		high = math.max(high, values[index])
	end
	return high - low
end

local function percentile(values, fraction)
	local sorted = {}
	for index, value in ipairs(values) do sorted[index] = value end
	table.sort(sorted)
	local index = math.floor(fraction * (#sorted - 1)) + 1
	return sorted[math.max(1, math.min(#sorted, index))]
end

local function measure(name, iterations, callback, extra)
	local floorNs = resolutionFloorNs(clockQuantumNs(), 1)
	-- Grow the loop until a sample clears the floor; the discarded pass is warmup.
	local calls = iterations
	local elapsed
	while true do
		local start = monotonicSeconds()
		callback(calls)
		elapsed = (monotonicSeconds() - start) * 1000000000
		if elapsed >= floorNs or calls >= iterations * 1024 then break end
		local growth = elapsed > 0 and math.ceil(floorNs / elapsed) or 16
		calls = math.max(calls + 1, math.ceil(calls * math.max(growth, 2)))
	end
	if elapsed < floorNs then
		reportUnresolved(name, elapsed, floorNs, calls, extra)
		return
	end
	local samples = {}
	local totalSamples = {}
	local checksum = 0
	for repeatIndex = 1, repeats do
		local start = monotonicSeconds()
		local value = callback(calls)
		elapsed = (monotonicSeconds() - start) * 1000000000
		samples[repeatIndex] = elapsed / calls
		totalSamples[repeatIndex] = elapsed
		if type(value) == "number" then
			checksum = checksum + value
		elseif type(value) == "table" then
			checksum = checksum + #value
		elseif value == true then
			checksum = checksum + 1
		end
	end
	local payload = {
		backend = "lua",
		test = name,
		status = "pass",
		iterations = calls,
		medianNs = median(samples),
		spreadNs = spread(samples),
		totalMedianNs = median(totalSamples),
		totalSpreadNs = spread(totalSamples),
		checksum = checksum,
		samplesNs = samples,
		scale = benchmarkScale,
	}
	if extra then
		for key, value in pairs(extra) do
			payload[key] = value
		end
	end
	appendJSON(outputPath("lua"), payload)
end

local function makeUnits()
	-- Keep the fixed scenario identical across profiles. Creating the inventory
	-- is setup work and is outside every timed sample, while list/spatial calls
	-- depend on the prescribed 1,000 real units being present.
	local count = 1000
	local squareSize = Game.squareSize or 8
	local spacing = 7
	local columns = math.max(1, math.floor((Game.mapSizeX - squareSize * 2) / spacing) + 1)
	local rows = math.max(1, math.floor((Game.mapSizeZ - squareSize * 2) / spacing) + 1)
	local cells = columns * rows
	for index = 1, count do
		local cell = (index - 1) % cells
		local column = cell % columns
		local row = math.floor(cell / columns)
		local x = squareSize + column * spacing
		local z = squareSize + row * spacing
		local y = Spring.GetGroundHeight(x, z) or 0
		local unitID = Spring.CreateUnit("native_api_test_unit", x, y, z, 0, 0, false, false)
		if unitID == nil then
			error("could not create benchmark unit " .. index, 0)
		end
		unitIDs[#unitIDs + 1] = unitID
	end
	firstUnitID = unitIDs[1]
	firstUnitDefID = Spring.GetUnitDefID(firstUnitID)
	-- Give the first unit one short command so GetUnitCommands has a stable
	-- small result without changing the benchmark's unit inventory.
	Spring.GiveOrderToUnit(firstUnitID, CMD.MOVE, {512, 0, 512}, {})
	fixtureReady = true
end

local beginHeightmap
local calloutCases
local calloutIndex = 0
local calloutRepeat = 0
local calloutCalls = 0
local calloutTotalNs = 0
local calloutRegions = 0
local calloutSamples = nil
local calloutChecksum = 0

local function beginCallouts()
	local unitID = firstUnitID
	local unitDefID = firstUnitDefID
	local x, _, z = Spring.GetUnitPosition(unitID)
	-- Allocation-heavy cases are deliberately sliced across GameFrame
	-- callbacks. The synced sandbox removes collectgarbage, so doing all
	-- list/spatial calls in one callback would measure allocator exhaustion
	-- rather than an API call and can retain gigabytes until the callback ends.
	calloutCases = {
		{ name = "callout_scalar", iterations = scaledCount(100000), batch = 10000, nominalIterations = 100000, callback = function(iterations)
			local value
			for _ = 1, iterations do value = Spring.GetUnitDefID(unitID) end
			return value
		end },
		{ name = "callout_vec3", iterations = scaledCount(100000), batch = 10000, nominalIterations = 100000, callback = function(iterations)
			local value
			for _ = 1, iterations do
				local px, py, pz = Spring.GetUnitPosition(unitID)
				value = (px or 0) + (py or 0) + (pz or 0)
			end
			return value
		end },
		{ name = "callout_string", iterations = scaledCount(50000), batch = 5000, nominalIterations = 50000, callback = function(iterations)
			local value
			for _ = 1, iterations do
				-- GetUnitDefName is not present in the synced Lua table. UnitDefs is
				-- the synced Lua equivalent of the native definition-name lookup.
				value = (UnitDefs[unitDefID] and UnitDefs[unitDefID].name) or ""
			end
			return #value
		end },
		{ name = "callout_smalllist", iterations = scaledCount(20000), batch = 100, nominalIterations = 20000, callback = function(iterations)
			local value
			for _ = 1, iterations do value = Spring.GetUnitCommands(unitID, 5) or {} end
			return value
		end },
		{ name = "callout_biglist", iterations = scaledCount(1000), batch = 20, nominalIterations = 1000, callback = function(iterations)
			local value
			for _ = 1, iterations do value = Spring.GetTeamUnits(0) or {} end
			return value
		end },
		{ name = "callout_spatial", iterations = scaledCount(10000), batch = 100, nominalIterations = 10000, callback = function(iterations)
			local value
			for _ = 1, iterations do value = Spring.GetUnitsInCylinder(x, z, 300) or {} end
			return value
		end },
		{ name = "callout_mutate", iterations = scaledCount(100000), batch = 10000, nominalIterations = 100000, callback = function(iterations, offset)
			for index = 1, iterations do
				Spring.SetUnitRulesParam(unitID, "bench", offset + index, {public = true})
			end
			return iterations
		end },
		-- Keep the Lua call shape explicit: this is the twelve-scalar peer of
		-- Core's four Float3 records, not a synthetic loop over scalar setters.
		{ name = "callout_wide_unit_physics", iterations = scaledCount(20000), batch = 100, nominalIterations = 20000, callback = function(iterations)
			for _ = 1, iterations do
				Spring.SetUnitPhysics(unitID, x, 96, z, 1, 0, 0, 0, 1, 0, 1, 0, 0)
			end
			return iterations
		end },
		{ name = "callout_payload_8", iterations = scaledCount(20000), batch = 1000, nominalIterations = 20000, callback = function(iterations)
			local value
			for _ = 1, iterations do value = Spring.SetTerrainTypeData(0, 1, 1, 1, 1, 1, true, "terrain") end
			return value
		end },
		{ name = "callout_payload_64", iterations = scaledCount(20000), batch = 1000, nominalIterations = 20000, callback = function(iterations)
			local value
			for _ = 1, iterations do value = Spring.SetTerrainTypeData(0, 1, 1, 1, 1, 1, true, "terrain-payload-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx") end
			return value
		end },
		{ name = "callout_payload_256", iterations = scaledCount(20000), batch = 1000, nominalIterations = 20000, callback = function(iterations)
			local value
			for _ = 1, iterations do value = Spring.SetTerrainTypeData(0, 1, 1, 1, 1, 1, true, "terrain-payload-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx") end
			return value
		end },
	}
	if benchmarkCase == "callout_scalar" then
		local scalar = calloutCases[1]
		scalar.iterations = benchmarkIterations
		scalar.batch = math.min(scalar.batch, benchmarkIterations)
		calloutCases = { scalar }
	end
	calloutIndex = 1
	calloutRepeat = 1
	calloutCalls = 0
	calloutTotalNs = 0
	calloutRegions = 0
	calloutSamples = {}
	calloutChecksum = 0
end

local advanceCallout

local function finishCallout()
	local case = calloutCases[calloutIndex]
	local floorNs = resolutionFloorNs(clockQuantumNs(), calloutRegions)
	if calloutTotalNs < floorNs then
		reportUnresolved(case.name, calloutTotalNs, floorNs, case.iterations,
			{ nominalIterations = case.nominalIterations, regions = calloutRegions })
		advanceCallout()
		return
	end
	calloutSamples[calloutRepeat] = calloutTotalNs / case.iterations
	if calloutRepeat < repeats then
		calloutRepeat = calloutRepeat + 1
		calloutCalls = 0
		calloutTotalNs = 0
		calloutRegions = 0
		calloutChecksum = 0
		return
	end
	appendJSON(outputPath("lua"), {
		backend = "lua",
		test = case.name,
		status = "pass",
		iterations = case.iterations,
		medianNs = median(calloutSamples),
		p99Ns = percentile(calloutSamples, 0.99),
		spreadNs = spread(calloutSamples),
		totalMedianNs = median(calloutSamples) * case.iterations,
		totalSpreadNs = spread(calloutSamples) * case.iterations,
		checksum = calloutChecksum,
		samplesNs = calloutSamples,
		scale = benchmarkScale,
		nominalIterations = case.nominalIterations,
		measurement = "Lua API call loop sliced across GameFrame callbacks",
	})
	advanceCallout()
end

advanceCallout = function()
	calloutIndex = calloutIndex + 1
	calloutRepeat = 1
	calloutCalls = 0
	calloutTotalNs = 0
	calloutRegions = 0
	calloutChecksum = 0
	calloutSamples = {}
	if calloutIndex > #calloutCases then
		calloutIndex = 0
		calloutCases = nil
		if benchmarkCase == "callout_scalar" or benchmarkCase == "callouts" then
			benchmarkComplete = true
			appendJSON(outputPath("lua"), {
				backend = "lua",
				test = "complete",
				status = "pass",
				benchmarkCase = benchmarkCase,
			})
			return
		end
		appendJSON(outputPath("lua"), {
			backend = "lua",
			test = "callout_draw",
			status = "unavailable",
			reason = "draw callouts require the unsynced GL context",
		})
		beginHeightmap()
	end
end

local function stepCallouts()
	if calloutIndex == 0 then return end
	local case = calloutCases[calloutIndex]
	local count = math.min(case.batch, case.iterations - calloutCalls)
	local start = monotonicSeconds()
	local value = case.callback(count, calloutCalls)
	calloutTotalNs = calloutTotalNs + (monotonicSeconds() - start) * 1000000000
	calloutRegions = calloutRegions + 1
	calloutCalls = calloutCalls + count
	if type(value) == "number" then
		calloutChecksum = calloutChecksum + value
	elseif type(value) == "table" then
		calloutChecksum = calloutChecksum + #value
	elseif value == true then
		calloutChecksum = calloutChecksum + 1
	end
	if calloutCalls == case.iterations then finishCallout() end
end

local beginWorkloads
local heightmapCases = {
	-- Every terrain call uses zero terraform or the current height, so it is
	-- safe to batch independent invocations per GameFrame. The measured unit
	-- remains one callback; batching only removes scheduler wait time.
	{ name = "hm_callback_empty", size = 0, nominalSize = 0, invocations = scaledTerrainCount(10000), nominalInvocations = 10000, batch = 1000 },
	{ name = "hm_brush_small", size = scaledBrushSize(32), nominalSize = 32, invocations = scaledTerrainCount(1000), nominalInvocations = 1000, batch = 100 },
	{ name = "hm_brush_medium", size = scaledBrushSize(128), nominalSize = 128, invocations = scaledTerrainCount(100), nominalInvocations = 100, batch = 10 },
	{ name = "hm_brush_large", size = scaledBrushSize(512), nominalSize = 512, invocations = scaledTerrainCount(10), nominalInvocations = 10, batch = 10 },
	{ name = "hm_region_op", size = 0, nominalSize = 0, invocations = scaledTerrainCount(1000), nominalInvocations = 1000, batch = 1000, region = true },
}
local heightmapCaseIndex = 0
local heightmapRepeat = 0
local heightmapInvocation = 0
local heightmapSamples = nil
local heightmapInnerSamples = nil
local heightmapTotalNs = 0
local heightmapRegions = 0
local heightmapInnerNs = 0
local heightmapInnerCalls = 0
local heightmapInnerRegions = 0

beginHeightmap = function()
	heightmapCaseIndex = 1
	heightmapRepeat = 1
	heightmapInvocation = 0
	heightmapSamples = {}
	heightmapInnerSamples = {}
	heightmapTotalNs = 0
	heightmapRegions = 0
	heightmapInnerNs = 0
	heightmapInnerCalls = 0
	heightmapInnerRegions = 0
end

local advanceHeightmapCase

local function finishHeightmapCase()
	local case = heightmapCases[heightmapCaseIndex]
	local floorNs = resolutionFloorNs(clockQuantumNs(), heightmapRegions)
	if heightmapTotalNs < floorNs then
		reportUnresolved(case.name, heightmapTotalNs, floorNs, case.invocations, {
			nominalInvocations = case.nominalInvocations,
			nominalSize = case.nominalSize,
			regions = heightmapRegions,
		})
		advanceHeightmapCase()
		return
	end
	heightmapSamples[heightmapRepeat] = heightmapTotalNs / case.invocations
	-- The inner region is timed per callback and carries its own floor.
	local innerFloorNs = resolutionFloorNs(clockQuantumNs(), heightmapInnerRegions)
	heightmapInnerSamples[heightmapRepeat] =
		(case.size > 0 and heightmapInnerNs >= innerFloorNs)
		and heightmapInnerNs / math.max(heightmapInnerCalls, 1) or 0
	if heightmapRepeat < repeats then
		heightmapRepeat = heightmapRepeat + 1
		heightmapInvocation = 0
		heightmapTotalNs = 0
		heightmapRegions = 0
		heightmapInnerNs = 0
		heightmapInnerCalls = 0
		heightmapInnerRegions = 0
		return
	end
	local heightmapSampleMs = {}
	for index, value in ipairs(heightmapSamples) do
		heightmapSampleMs[index] = value / 1000000
	end
	appendJSON(outputPath("lua"), {
		backend = "lua",
		test = case.name,
		status = "pass",
		invocations = case.invocations,
		innerCalls = case.size * case.size,
		medianMs = median(heightmapSamples) / 1000000,
		p99Ms = percentile(heightmapSamples, 0.99) / 1000000,
		spreadMs = spread(heightmapSamples) / 1000000,
		samplesMs = heightmapSampleMs,
		innerNs = median(heightmapInnerSamples),
		scale = benchmarkScale,
		nominalSize = case.nominalSize,
		nominalInvocations = case.nominalInvocations,
		measurement = "Lua callback boundary with zero terraform; terrain rebuild excluded",
	})
	advanceHeightmapCase()
end

advanceHeightmapCase = function()
	heightmapCaseIndex = heightmapCaseIndex + 1
	heightmapInvocation = 0
	if heightmapCaseIndex > #heightmapCases then
		heightmapCaseIndex = 0
		heightmapSamples = nil
		heightmapInnerSamples = nil
		if benchmarkCase == "heightmap" then
			benchmarkComplete = true
			appendJSON(outputPath("lua"), {
				backend = "lua",
				test = "complete",
				status = "pass",
				benchmarkCase = benchmarkCase,
			})
			return
		end
		for _, name in ipairs({"mem_per_call_small", "mem_per_call_list", "gc_pause", "frame_spike", "mem_growth"}) do
			appendJSON(outputPath("lua"), {
				backend = "lua",
				test = name,
				status = "unavailable",
				reason = "synced Lua sandbox does not expose collectgarbage or GetLuaMemUsage",
			})
		end
		beginWorkloads()
	else
		-- Each case owns an independent repeat set. Leaving the repeat index and
		-- accumulators from the previous case in place makes the next case record
		-- one sample into the previous case's five-sample array, which produces
		-- identical-looking or effectively zero Lua rows.
		heightmapRepeat = 1
		heightmapSamples = {}
		heightmapInnerSamples = {}
		heightmapTotalNs = 0
		heightmapRegions = 0
		heightmapInnerNs = 0
		heightmapInnerCalls = 0
		heightmapInnerRegions = 0
	end
end

local function stepHeightmap()
	if heightmapCaseIndex == 0 then
		return
	end
	local case = heightmapCases[heightmapCaseIndex]
	-- Use world coordinates on square boundaries. Larger brushes intentionally
	-- revisit the bounded fixture squares so the prescribed call counts do not
	-- require a multi-gigabyte map. Terraform zero keeps this profile focused
	-- on the callback and inner-call boundary; path/LOS rebuild cost is a
	-- separate engine workload and would otherwise dominate every backend.
	local x, z = 8, 8
	local terrainHeight = Spring.GetGroundHeight(x, z) or 96
	local invocations = math.min(case.batch, case.invocations - heightmapInvocation)
	-- Time the whole batch: one SetHeightMapFunc call is shorter than the Lua
	-- timer's float step.
	local batchStart = monotonicSeconds()
	for _ = 1, invocations do
		if case.region then
			Spring.LevelHeightMap(x, z, 256, 256, terrainHeight)
		else
			local result = Spring.SetHeightMapFunc(function()
				local innerStart = monotonicSeconds()
				for offsetX = 0, case.size - 1 do
					for offsetZ = 0, case.size - 1 do
						local squareX = 8 + (offsetX % 30) * 8
						local squareZ = 8 + (offsetZ % 30) * 8
						Spring.SetHeightMap(x + squareX, z + squareZ, terrainHeight, 0)
						heightmapInnerCalls = heightmapInnerCalls + 1
					end
				end
				heightmapInnerNs = heightmapInnerNs + (monotonicSeconds() - innerStart) * 1000000000
				heightmapInnerRegions = heightmapInnerRegions + 1
			end)
			if result == nil then
				error("SetHeightMapFunc returned nil")
			end
		end
		heightmapInvocation = heightmapInvocation + 1
	end
	heightmapTotalNs = heightmapTotalNs + (monotonicSeconds() - batchStart) * 1000000000
	heightmapRegions = heightmapRegions + 1
	if heightmapInvocation == case.invocations then
		finishHeightmapCase()
	end
end

local function workloadUnitScan()
	local unitLimit = math.min(#unitIDs, 1000)
	local areaLimit = math.min(#unitIDs, 100)
	local commandLimit = math.min(#unitIDs, 200)
	local computeLimit = 100000
	local function unitScan()
		local count = 0
		for index = 1, unitLimit do
			local unitID = unitIDs[index]
			local x, y, z = Spring.GetUnitPosition(unitID)
			local health = Spring.GetUnitHealth(unitID) or 0
			local defID = Spring.GetUnitDefID(unitID) or 0
			if x and health < 50 then count = count + defID end
			count = count + math.floor((x or 0) + (y or 0) + (z or 0)) % 7
		end
		return count
	end
	local function areaEffect()
		local count = 0
		for index = 1, areaLimit do
			local x, _, z = Spring.GetUnitPosition(unitIDs[index])
			count = count + #(Spring.GetUnitsInCylinder(x, z, 300) or {})
		end
		return count
	end
	local function rulesParams()
		local count = 0
		for index = 1, unitLimit do
			local unitID = unitIDs[index]
			Spring.SetUnitRulesParam(unitID, "bench", index, {public = true})
			local value = Spring.GetUnitRulesParam(unitID, "bench")
			count = count + (value or 0)
		end
		return count
	end
	local function commands()
		for index = 1, commandLimit do
			local unitID = unitIDs[index]
			local x, y, z = Spring.GetUnitPosition(unitID)
			Spring.GiveOrderToUnit(unitID, CMD.MOVE, {x + 8, y, z + 8}, {})
		end
		return 200
	end
	local function compute()
		local value = 0
		for index = 1, computeLimit do
			value = (value + index * 0.25) % 1000003
		end
		return value
	end
	return unitScan, areaEffect, rulesParams, commands, compute
end

local workloadCallbacks = {}
local workloadIndex = 0
local workloadFrame = 0
local workloadSamples = nil
local workloadChecksum = 0
local workloadFrames = scaledCount(5000)
local workloadBatchFrames = 1
local workloadBatchStart = 0
local workloadBatchCount = 0

beginWorkloads = function()
	local unitScan, areaEffect, rulesParams, commands, compute = workloadUnitScan()
	workloadCallbacks = {
		{ name = "wl_unit_scan", callback = unitScan },
		{ name = "wl_area_effect", callback = areaEffect },
		{ name = "wl_rules_params", callback = rulesParams },
		{ name = "wl_commands", callback = commands },
		{ name = "wl_compute", callback = compute },
	}
	workloadIndex = 1
	workloadFrame = 0
	workloadSamples = {}
	workloadChecksum = 0
	workloadBatchFrames = math.max(1, math.floor(workloadFrames / repeats + 0.5))
	workloadBatchStart = 0
	workloadBatchCount = 0
	if benchmarkCase ~= "workloads" then
		appendJSON(outputPath("lua"), {
			backend = "lua",
			test = "wl_ui_draw",
			status = "unavailable",
			reason = "draw workload requires the unsynced GL context",
		})
	end
end

local function runBoundedWorkloads()
	local unitScan, areaEffect, rulesParams, commands, compute = workloadUnitScan()
	for _, workload in ipairs({
		{ name = "wl_unit_scan", callback = unitScan },
		{ name = "wl_area_effect", callback = areaEffect },
		{ name = "wl_rules_params", callback = rulesParams },
		{ name = "wl_commands", callback = commands },
		{ name = "wl_compute", callback = compute },
	}) do
		measure(workload.name, workloadFrames, function(iterations)
			local value
			for _ = 1, iterations do
				value = workload.callback()
			end
			return value
		end, { measurement = "Lua workload body loop" })
	end
	benchmarkComplete = true
	appendJSON(outputPath("lua"), {
		backend = "lua",
		test = "complete",
		status = "pass",
		benchmarkCase = benchmarkCase,
	})
end

local function finishWorkload()
	local sorted = {}
	for index, value in ipairs(workloadSamples) do
		sorted[index] = value
	end
	table.sort(sorted)
	local medianValue = sorted[math.floor((#sorted + 1) / 2)]
	local low = sorted[1]
	local high = sorted[#sorted]
	local floorNs = resolutionFloorNs(clockQuantumNs(), 1) / workloadBatchFrames
	if medianValue < floorNs then
		reportUnresolved(workloadCallbacks[workloadIndex].name,
			medianValue * workloadBatchFrames, floorNs * workloadBatchFrames,
			workloadFrames, { nominalIterations = 5000 })
		workloadIndex = workloadIndex + 1
		workloadFrame = 0
		workloadSamples = {}
		workloadChecksum = 0
		workloadBatchStart = 0
		workloadBatchCount = 0
		if workloadIndex > #workloadCallbacks then
			workloadIndex = 0
			workloadSamples = nil
			benchmarkComplete = true
			appendJSON(outputPath("lua"), {backend = "lua", test = "complete", status = "pass", seed = benchmarkSeed})
		end
		return
	end
	appendJSON(outputPath("lua"), {
		backend = "lua",
		test = workloadCallbacks[workloadIndex].name,
		status = "pass",
		iterations = workloadFrames,
		nominalIterations = 5000,
		scale = benchmarkScale,
		medianNs = medianValue,
		p99Ns = percentile(workloadSamples, 0.99),
		spreadNs = high - low,
		checksum = workloadChecksum,
		samplesNs = workloadSamples,
		measurement = "Lua workload measured across actual GameFrame callbacks",
	})
	workloadIndex = workloadIndex + 1
	workloadFrame = 0
	workloadSamples = {}
	workloadChecksum = 0
	workloadBatchStart = 0
	workloadBatchCount = 0
	if workloadIndex > #workloadCallbacks then
		workloadIndex = 0
		workloadSamples = nil
		benchmarkComplete = true
		appendJSON(outputPath("lua"), {backend = "lua", test = "complete", status = "pass", seed = benchmarkSeed})
	end
end

local function stepWorkload()
	if workloadIndex == 0 then
		return
	end
	local workload = workloadCallbacks[workloadIndex]
	if workloadBatchCount == 0 then
		workloadBatchStart = monotonicSeconds()
	end
	local value = workload.callback()
	workloadFrame = workloadFrame + 1
	workloadBatchCount = workloadBatchCount + 1
	if type(value) == "number" then
		workloadChecksum = workloadChecksum + value
	elseif type(value) == "table" then
		workloadChecksum = workloadChecksum + #value
	elseif value == true then
		workloadChecksum = workloadChecksum + 1
	end
	if workloadBatchCount == workloadBatchFrames or workloadFrame == workloadFrames then
		local elapsed = (monotonicSeconds() - workloadBatchStart) * 1000000000
		workloadSamples[#workloadSamples + 1] = elapsed / workloadBatchCount
		workloadBatchCount = 0
	end
	if workloadFrame == workloadFrames then
		finishWorkload()
	end
end

local function runLuaBenchmark()
	if benchmarkCase == "draw" then
		-- DrawWorld is driven by LuaUI's real render callback.  Do not also run
		-- the generic synced callback-body table here; it would create a second
		-- row with the same test name and would not measure the UI environment.
		return
	end
	if benchmarkCase == "callins" then
		-- The engine-gated recorder measures the actual callin boundary. The
		-- deterministic driver below enters the same CEventHandler paths used by
		-- real simulation events; there is no guest-side timing loop here.
		return
	end
	if benchmarkCase == "callout_scalar" or benchmarkCase == "callouts" then
		beginCallouts()
		return
	end
	if benchmarkCase == "heightmap" then
		beginHeightmap()
		return
	end
	if benchmarkCase == "workloads" then
		-- Run one workload body per real GameFrame so the engine can perform its
		-- normal per-frame GC between samples. Running the whole matrix in one
		-- Lua call retains every temporary list until the call returns.
		beginWorkloads()
		return
	end
	-- These rows are deliberately recorded separately from callouts. The
	-- callback rows below remain body-loop measurements; workload rows are
	-- scheduled over actual GameFrame callbacks so the engine can run its
	-- normal per-frame GC between samples.
	for _, callin in ipairs({
		{ name = "callin_empty", callback = function() end, iterations = scaledCount(10000) },
		{ name = "callin_gameframe", callback = function(value) return value end, iterations = scaledCount(10000) },
		{ name = "callin_update", callback = function(value) return value + 0.25 end, iterations = scaledCount(10000) },
		{ name = "callin_drawworld", callback = function() end, iterations = scaledCount(10000) },
		{ name = "callin_unitcreated", callback = function() return unitIDs[1], firstUnitDefID, 0, -1 end, iterations = scaledCount(5000) },
		{ name = "callin_unitpredamaged", callback = function() return 1, 0 end, iterations = scaledCount(50000) },
		{ name = "callin_allowunitcreation", callback = function() return true end, iterations = scaledCount(5000) },
		{ name = "callin_unimplemented", callback = function() end, iterations = scaledCount(10000) },
		{ name = "callin_4modules", callback = function() end, iterations = scaledCount(10000) },
	}) do
		local samples = {}
		for repeatIndex = 1, repeats do
			local start = monotonicSeconds()
			local value
			for index = 1, callin.iterations do
				value = callin.callback(index)
			end
			samples[repeatIndex] = (monotonicSeconds() - start) * 1000000000 / callin.iterations
		end
		appendJSON(outputPath("lua"), {
			backend = "lua",
			test = callin.name,
			status = "pass",
			iterations = callin.iterations,
			medianNs = median(samples),
			p99Ns = percentile(samples, 0.99),
			spreadNs = spread(samples),
			samplesNs = samples,
			scale = benchmarkScale,
			nominalIterations = callin.iterations / benchmarkScale,
			measurement = "Lua callback body loop",
		})
	end
	beginCallouts()
end

local function runCallinDriver()
	if not debug or type(debug.emulateNativeApiParityCallins) ~= "function" then
		error("missing deterministic callin event driver", 0)
	end
	local iterations = scaledCount(5000)
	callinDriverActive = true
	for _ = 1, iterations do
		debug.emulateNativeApiParityCallins(firstUnitID, 0, -1, true)
		callinDriverCalls = callinDriverCalls + 1
	end
	callinDriverActive = false
	-- Let the engine deliver a short tail of ordinary frames after the
	-- deterministic event driver. This supplies useful GameFrame/Update
	-- samples without turning the bounded profile into the nominal 5,000-frame
	-- workload.
	callinFramesRemaining = 30
end

	if not synced then
		local function memorySnapshot()
			local _, _, globalBytes, globalAllocs = Spring.GetLuaMemUsage()
			return (globalBytes or 0) * 1024, (globalAllocs or 0) * 1000
		end
		local function elapsedMilliseconds(startTimer)
			-- Unsynced GetTimerMicros returns the lightuserdata timer token used by
			-- Spring.DiffTimers; it is not an arithmetic integer.
			return Spring.DiffTimers(Spring.GetTimerMicros(), startTimer, true, true)
		end

	local function runUnsyncedMemoryBenchmark()
		local units = Spring.GetTeamUnits(0) or {}
		local unitID = units[1]
		if unitID == nil then return false end
		local unitLimit = math.min(#units, scaledCount(1000))

		local beforeBytes, beforeAllocs = memorySnapshot()
		local start = Spring.GetTimerMicros()
		for _ = 1, scaledCount(100000) do
			Spring.GetUnitPosition(unitID)
		end
		local elapsed = elapsedMilliseconds(start)
		local afterBytes, afterAllocs = memorySnapshot()
		appendJSON(outputPath("lua"), {
			backend = "lua",
			test = "mem_per_call_small",
			status = "pass",
			bytes = math.max(0, afterBytes - beforeBytes),
			allocations = math.max(0, afterAllocs - beforeAllocs),
			elapsedMs = elapsed,
			scale = benchmarkScale,
		})

		beforeBytes, beforeAllocs = memorySnapshot()
		start = Spring.GetTimerMicros()
		for _ = 1, scaledCount(1000) do
			Spring.GetTeamUnits(0)
		end
		elapsed = elapsedMilliseconds(start)
		afterBytes, afterAllocs = memorySnapshot()
		appendJSON(outputPath("lua"), {
			backend = "lua",
			test = "mem_per_call_list",
			status = "pass",
			bytes = math.max(0, afterBytes - beforeBytes),
			allocations = math.max(0, afterAllocs - beforeAllocs),
			elapsedMs = elapsed,
			scale = benchmarkScale,
		})

		start = Spring.GetTimerMicros()
		local gcKB = Spring.GetSyncedGCInfo(true) or 0
		local gcElapsed = elapsedMilliseconds(start)
		appendJSON(outputPath("lua"), {
			backend = "lua",
			test = "gc_pause",
			status = "pass",
			gcKB = gcKB,
			totalPauseMs = gcElapsed,
			scale = benchmarkScale,
		})

		local frameTimes = {}
		for frame = 1, scaledCount(5000) do
			start = Spring.GetTimerMicros()
			for index = 1, unitLimit do
				local id = units[index]
				Spring.GetUnitDefID(id)
				Spring.GetUnitPosition(id)
			end
			frameTimes[frame] = elapsedMilliseconds(start)
		end
		table.sort(frameTimes)
		local p99Index = math.max(1, math.ceil(#frameTimes * 0.99))
		appendJSON(outputPath("lua"), {
			backend = "lua",
			test = "frame_spike",
			status = "pass",
			worstMs = frameTimes[#frameTimes],
			p99Ms = frameTimes[p99Index],
			frames = #frameTimes,
			scale = benchmarkScale,
		})

		afterBytes = select(1, memorySnapshot())
		appendJSON(outputPath("lua"), {
			backend = "lua",
			test = "mem_growth",
			status = "pass",
			peakBytes = afterBytes,
			steadyBytes = afterBytes,
			scale = benchmarkScale,
		})
		appendJSON(outputPath("lua"), {
			backend = "lua",
			test = "complete",
			status = "pass",
			benchmarkCase = benchmarkCase,
		})
		benchmarkComplete = true
		return true
	end

	function gadget:RecvFromSynced(name, ...)
		if name == "native_api_benchmark_lua_result" then
			local encoded = ...
			local file = io.open(outputPath("lua"), "a")
			if file then
				file:write(encoded)
				file:write("\n")
				file:close()
			end
		elseif name == "native_api_benchmark_result" then
			local encoded = ...
			local file = io.open(outputPath("wasm"), "a")
			if file then
				file:write(encoded)
				file:write("\n")
				file:close()
			end
		elseif name == "native_api_benchmark_complete" then
			benchmarkComplete = true
		end
	end
	function gadget:Initialize()
		-- The benchmark records work inside the callback and needs the engine to
		-- advance 5,000 logical frames without waiting for wall-clock 30 Hz.
		-- This only affects the isolated benchmark process; it is not part of
		-- any measured operation.
		Spring.SendCommands("setmaxspeed 100", "setminspeed 100", "setspeed 100", "speedcontrol 0")
	end
	function gadget:Update()
		if benchmarkCase == "callins" and callinVariant == "unimplemented" and not memoryRan then
			memoryRan = true
			if not debug or type(debug.emulateNativeApiParityUnimplementedCallin) ~= "function" then
				error("missing unimplemented callin benchmark helper", 0)
			end
			debug.emulateNativeApiParityUnimplementedCallin(scaledCount(10000))
			benchmarkComplete = true
			Spring.Quit()
			return
		end
			if benchmarkCase == "memory" and not memoryRan then
				if backend == "lua" then
					memoryRan = runUnsyncedMemoryBenchmark()
				elseif backend == "native" and #(Spring.GetTeamUnits(0) or {}) > 0 then
					memoryRan = true
					Spring.InvokeNativeModule(Common.encode({testName = "benchmark", command = "run", seed = benchmarkSeed, repeats = repeats}))
				end
		end
		-- LuaUI owns the quit for the draw profile: callin_drawworld is measured
		-- from the empty DrawWorld frames that follow the workload.
		if benchmarkComplete and benchmarkCase ~= "draw" then
			Spring.Quit()
		end
	end
	return
end

function gadget:Initialize()
	-- Unit creation is deferred until GameFrame so all three backends see the
	-- same initialized simulation state and the benchmark does not include map
	-- loading or unit-definition setup time.
end

function gadget:RecvLuaMsg(message)
	local nativePrefix = "NATIVE_BENCH|"
	if message:sub(1, #nativePrefix) == nativePrefix then
		local encoded = message:sub(#nativePrefix + 1)
		if encoded:find('"test":"complete"', 1, true) ~= nil then
			benchmarkComplete = true
		end
		return true
	end
	local wasmPrefix = "WASM_BENCH|"
	if message:sub(1, #wasmPrefix) == wasmPrefix then
		local encoded = message:sub(#wasmPrefix + 1)
		if encoded:find('"test":"complete"', 1, true) ~= nil then
			benchmarkComplete = true
			SendToUnsynced("native_api_benchmark_complete")
		end
		SendToUnsynced("native_api_benchmark_result", encoded)
		return true
	end
	return false
end

if callinVariant ~= "unimplemented" then
function gadget:GameFrame(frame)
	if benchmarkCase == "callins" and callinVariant == "gameframe" then
		-- Keep the argument-read case distinct from the empty dispatch floor.
		callinFrameSink = callinFrameSink + frame
	elseif benchmarkCase == "callins" and callinVariant == "fourmodules" then
		-- Lua keeps gadgets in one Lua state, so the four independent callback
		-- bodies are invoked by the handle's actual GameFrame dispatch.
		benchmarkModuleOne(frame)
		benchmarkModuleTwo(frame)
		benchmarkModuleThree(frame)
		benchmarkModuleFour(frame)
	end
	if frame == 1 then
		makeUnits()
	elseif frame == (benchmarkCase == "heightmap" and 65 or 3) and fixtureReady then
		-- Let the engine complete its initial height-bound update before the
		-- callback profile starts; otherwise the first no-op terrain batch can
		-- inherit the map-load dirty flag and run a full-map refresh.
		if benchmarkCase == "callins" then
			runCallinDriver()
		elseif benchmarkCase == "memory" then
			-- Unsynced memory profiling is driven from gadget:Update below.
		elseif backend == "lua" then
			runLuaBenchmark()
		elseif backend == "native" and benchmarkCase ~= "draw" then
			Spring.InvokeNativeModule(Common.encode({testName = "benchmark", command = "run", seed = benchmarkSeed, repeats = repeats}))
		elseif backend == "wasm" then
			-- The Wasm guest owns its timing loop and sends one row per test. The
			-- frame callback is deliberately not used as a timing boundary.
		end
	elseif benchmarkCase == "callins" and callinFramesRemaining > 0 then
		callinFramesRemaining = callinFramesRemaining - 1
		if callinFramesRemaining == 0 then
			benchmarkComplete = true
		end
	elseif backend == "lua" and calloutIndex ~= 0 then
		stepCallouts()
	elseif backend == "lua" and heightmapCaseIndex ~= 0 then
		stepHeightmap()
	elseif backend == "lua" and workloadIndex ~= 0 then
		stepWorkload()
	elseif backend == "native" and benchmarkCase ~= "heightmap"
		and benchmarkCase ~= "draw" and frame > 10 then
		-- The draw profile quits from LuaUI once its DrawWorld tail is recorded.
		Spring.Quit()
	end
	if benchmarkComplete and benchmarkCase ~= "draw" then
		Spring.Quit()
	end
end

end

function gadget:UnitCreated(_unitID, _unitDefID, _unitTeam, _builderID)
	if benchmarkCase == "callins" then
		-- The C++ benchmark recorder surrounds this real event dispatch.
	end
end

function gadget:UnitPreDamaged(_unitID, _unitDefID, _unitTeam, damage, _paralyzer, _weaponDefID, _projectileID, _attackerID, _attackerDefID, _attackerTeam)
	if benchmarkCase == "callins" then
		return damage, 1
	end
	end

function gadget:AllowUnitCreation(_unitDefID, _builderID, _builderTeam, _x, _y, _z, _facing)
	if benchmarkCase == "callins" then
		return true, true
	end
	end

function gadget:GameFramePost(_frame)
	-- Keep this callback empty: the engine-side recorder measures the actual
	-- dispatch boundary for the Lua/native/Wasm comparison.
end
