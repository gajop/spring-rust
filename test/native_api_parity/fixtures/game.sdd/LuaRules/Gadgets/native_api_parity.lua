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
	local wasmContext = tostring((Spring.GetModOptions() or {}).native_api_parity_wasm_context or "synced_gadget")
	local wasmSpecPath = wasmContext == "synced_gadget"
		and "LuaRules/Utilities/wasm_api_probe_tests.lua"
		or "LuaRules/Utilities/wasm_api_probe_tests_" .. wasmContext .. ".lua"
	local WasmProbeSpec = VFS.Include(wasmSpecPath)
	local sentInventory = false
	local dynamicTimeReferenceSent = false
	local ranGeneratedTests = false
	local fixtureIDs = {}
	local groundDecalID
	local parityOptions = Spring.GetModOptions() or {}
	local processTest = tostring(parityOptions.native_api_parity_process_test or "")
	local processStage = tostring(parityOptions.native_api_parity_process_stage or "initial")
	local wasmRole = tostring(parityOptions.native_api_parity_wasm_role or "combined")
	local wasmReferenceFrame = (Common.mode() == "wasm"
		and (wasmContext == "unsynced_gadget" or wasmContext == "gaia_unsynced"))
		and 10 or 4
	local selectedTestsOption = tostring(parityOptions.native_api_parity_tests or "")
	local selectedTestPrefix = tonumber(parityOptions.native_api_parity_test_prefix or "")
	local selectedTests = {}
	for testName in string.gmatch(selectedTestsOption, "[^,]+") do
		selectedTests[testName] = true
	end
	local function selectedSurfaceTest(testName)
		return (selectedTestsOption == "" and selectedTestPrefix == nil) or selectedTests[testName] == true
	end
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

	local function sortedStrings(values)
		local result = {}
		for _, value in ipairs(values or {}) do
			result[#result + 1] = value
		end
		table.sort(result)
		return result
	end

	local function optionalSurfaceValue(value)
		return {
			present = value ~= nil,
			value = value or "",
		}
	end

	local function optionalSurfacePath(value)
		local result = {
			present = value ~= nil,
			basename = "",
		}
		if value ~= nil then
			result.basename = value:match("([^/]+)$") or value
		end
		return result
	end

	local function archiveInfoSurface(info)
		local result = {}
		for key, value in pairs(info or {}) do
			result[#result + 1] = {
				key = tostring(key),
				valueType = type(value),
				value = value,
			}
		end
		table.sort(result, function(left, right)
			return left.key < right.key
		end)
		return result
	end

	local function availableAIsSurface(ais)
		local result = {}
		for _, ai in ipairs(ais or {}) do
			result[#result + 1] = {
				shortName = ai.shortName or "",
				version = ai.version or "",
				isLuaAI = ai.isLuaAI == true,
			}
		end
		table.sort(result, function(left, right)
			if left.shortName ~= right.shortName then
				return left.shortName < right.shortName
			end
			if left.version ~= right.version then
				return left.version < right.version
			end
			return tostring(left.isLuaAI) < tostring(right.isLuaAI)
		end)
		return result
	end

	local function runVfsArchiveSurface()
		if not selectedSurfaceTest("vfs.archive_surface") then
			return
		end

		local testName = "vfs.archive_surface"
		-- VFS archive lookups require the documented lowercase path form.  The
		-- maphelper archive contains this file, so this is stable in both the
		-- Lua baseline and the native replay.
		local fileName = "mapoptions.lua"
		-- VFS.ZIP_ONLY is the documented archive-only mode.  The individual
		-- mode letters are `M`, `m`, `e`, and `b`; `z` is not a VFS mode.
		local fileMode = "Mmeb"
		local rawFileName = "LuaRules/Gadgets/native_api_parity.lua"
		-- nil selects Lua's raw-first default; passing an empty string explicitly
		-- would instead override that default.  The native wrapper represents the
		-- same default with an empty mode string in its query.
		local rawFileMode = nil
		local archiveName = VFS.GetArchiveContainingFile(fileName, fileMode)
		if archiveName == nil then
			error("VFS archive surface could not find mapoptions.lua in a zip archive", 0)
		end

		local archiveInfo = VFS.GetArchiveInfo(archiveName)
		if archiveInfo == nil then
			error("VFS archive surface could not read archive info", 0)
		end
		local dependencies = VFS.GetArchiveDependencies(archiveName)
		local replaces = VFS.GetArchiveReplaces(archiveName)
		if dependencies == nil or replaces == nil then
			error("VFS archive surface could not read archive relationships", 0)
		end

		local singleChecksum, completeChecksum = VFS.GetArchiveChecksum(archiveName)
		local absolutePath = VFS.GetFileAbsolutePath(fileName, fileMode)
		local loadedArchives = sortedStrings(VFS.GetLoadedArchives())
		local allArchives = sortedStrings(VFS.GetAllArchives())
		local availableAIs = availableAIsSurface(VFS.GetAvailableAIs("", ""))
		local rapidTagReturns = { VFS.GetNameFromRapidTag("native-api-parity-not-a-rapid-tag") }
		local rawData = VFS.LoadFile(rawFileName, rawFileMode)
		if rawData == nil then
			error("VFS archive surface could not load its raw fixture script", 0)
		end
		local rawDataBytes = {}
		for index = 1, #rawData do
			rawDataBytes[#rawDataBytes + 1] = string.format("%02x", string.byte(rawData, index))
		end
		local rawFileHex = table.concat(rawDataBytes)
		local rawDirList = sortedStrings(VFS.DirList("LuaRules/Gadgets", "*.lua", rawFileMode, false))
		local rawSubDirs = sortedStrings(VFS.SubDirs("LuaRules", "*", rawFileMode, false))

		-- Build a fresh archive in the fixture's isolated data directory to test
		-- compression.  LuaRules cannot call VFS.ScanAllDirs (that entry is
		-- installed only in LuaUI), so UseArchive uses the deterministic base
		-- cursor archive, which is scanned but not mapped during this fixture.
		local luaArchiveName = "native_api_parity_vfs_surface_lua.sdz"
		local nativeArchiveName = "native_api_parity_vfs_surface_native.sdz"
		local useArchiveName
		for _, candidate in ipairs(allArchives) do
			if string.find(string.lower(candidate), "cursors", 1, true) then
				useArchiveName = candidate
				break
			end
		end
		if useArchiveName == nil then
			error("VFS archive surface could not find the scanned cursor archive", 0)
		end
		local compressReturns = { pcall(
			VFS.CompressFolder,
			"LuaRules/Gadgets",
			"zip",
			luaArchiveName,
			false,
			"r"
		) }
		if not compressReturns[1] then
			error("VFS archive surface compression failed: " .. tostring(compressReturns[2]), 0)
		end
		local compressedExists = VFS.FileExists(luaArchiveName)

		local callbackVisible = false
		local callbackFileExists = false
		local useReturns = { pcall(VFS.UseArchive, useArchiveName, function()
			for _, loadedArchive in ipairs(VFS.GetLoadedArchives()) do
				if loadedArchive == useArchiveName then
					callbackVisible = true
					break
				end
			end
			callbackFileExists = VFS.FileExists("anims/cursorattack_0.bmp")
			return "callback-return"
		end) }
		if not useReturns[1] then
			error("VFS archive surface UseArchive failed: " .. tostring(useReturns[2]), 0)
		end
		local postLoadedArchives = sortedStrings(VFS.GetLoadedArchives())

		local payload = {
			status = "pass",
			context = "unsynced_gadget",
			result = {
				fileName = fileName,
				fileMode = fileMode,
				archiveName = archiveName,
				fileAbsolutePath = optionalSurfacePath(absolutePath),
				archiveContainingFile = optionalSurfaceValue(archiveName),
				hasArchive = VFS.HasArchive(archiveName),
				loadedArchives = loadedArchives,
				allArchives = allArchives,
				archivePath = optionalSurfacePath(VFS.GetArchivePath(archiveName)),
				archiveInfo = archiveInfoSurface(archiveInfo),
				archiveDependencies = sortedStrings(dependencies),
				archiveReplaces = sortedStrings(replaces),
				archiveChecksum = {
					single = singleChecksum or "",
					complete = completeChecksum or "",
				},
				rapidTag = optionalSurfaceValue(rapidTagReturns[1]),
				availableAIs = availableAIs,
				rawFile = {
					name = rawFileName,
					mode = rawFileMode or "",
					hex = rawFileHex,
					dirList = rawDirList,
					subDirs = rawSubDirs,
				},
				compress = {
					archiveName = luaArchiveName,
					exists = compressedExists,
					-- Lua.CompressFolder returns no values on success; the native
					-- ABI reports the same operation through Result<bool>.
					luaReturnCount = #compressReturns - 1,
				},
				useArchive = {
					archiveName = useArchiveName,
					ok = true,
					callbackVisible = callbackVisible,
					callbackFileExists = callbackFileExists,
					postLoadedArchives = postLoadedArchives,
					-- Lua preserves arbitrary callback returns; the native callback
					-- ABI intentionally exposes only the operation success flag.
					luaReturnCount = #useReturns - 1,
					luaReturnValue = useReturns[2] or "",
				},
				nativeArchiveName = nativeArchiveName,
			},
		}
		record(testName, payload)
		if Common.mode() == "native" then
			Spring.InvokeNativeModule(Common.encode(payload))
		end
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

	local function runWasmApiReference()
		if wasmRole == "guest" then
			return
		end
		if (wasmContext ~= "unsynced_gadget" and wasmContext ~= "gaia_unsynced")
			or fixtureIDs.unitID == nil
		then
			return
		end
		-- Match the Wasm probe's explicit precondition for renderer bookkeeping.
		-- GetRender*DrawFlagChanged reports previous/current state, so allowing a
		-- render pass between the two probes would compare scheduling rather than
		-- the API contract itself.  Register the fixture log section as well so
		-- GetLogSections observes the same test-created section on both paths.
		Spring.SetLogSectionFilterLevel("NativeApiParity", 3)
		Spring.ClearUnitsPreviousDrawFlag()
		Spring.ClearFeaturesPreviousDrawFlag()
		Common.runWasmApiReference(
			wasmContext,
			WasmProbeSpec,
			GeneratedTests,
			function(payload)
				forward("wasm", Common.encode(payload))
			end,
			fixtureIDs,
			function(testName)
				return Common.mode() == "wasm"
					and Common.enableRenderingTests()
					and testName == "get_game_seconds_interpolated"
			end
		)
	end

	function gadget:Initialize()
		-- Keep camera-control comparisons independent of the wall-clock time
		-- between the synced fixture notification and the first unsynced update.
		-- Mode 1 applies a zero-duration SetCameraTarget immediately; the
		-- default exponential transition would leave WorldToScreenCoords
		-- dependent on each process's render scheduling.
		Spring.SetConfigInt("CamTransitionMode", 1)
		Spring.SetConfigInt("WindowedEdgeMove", 0, true)
		Spring.SetConfigInt("FullscreenEdgeMove", 0, true)

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
			local unitID, featureID, unitDefID, featureDefID, weaponDefID, projectileID, pieceProjectileID, teamID, allyTeamID, enemyLosUnitID, enemyRadarUnitID, enemyHiddenUnitID = ...
			fixtureIDs = {
				unitID = unitID,
				featureID = featureID,
				unitDefID = unitDefID,
				featureDefID = featureDefID,
				weaponDefID = weaponDefID,
				weaponDefName = weaponDefID and WeaponDefs[weaponDefID] and WeaponDefs[weaponDefID].name,
				projectileID = projectileID,
				pieceProjectileID = pieceProjectileID,
				teamID = teamID,
				allyTeamID = allyTeamID,
				enemyLosUnitID = enemyLosUnitID,
				enemyRadarUnitID = enemyRadarUnitID,
				enemyHiddenUnitID = enemyHiddenUnitID,
				groundDecalID = groundDecalID,
			}
			-- Exercise the renderer's opt-in Lua object callbacks with the same
			-- objects that the native module sees.  These flags are intentionally
			-- set from unsynced Lua: the corresponding controls do not exist in
			-- synced Lua or LuaUI.
			if Spring.UnitRendering then
				Spring.UnitRendering.SetUnitLuaDraw(unitID, true)
				Spring.UnitRendering.SetProjectileLuaDraw(projectileID, true)
				if pieceProjectileID then
					Spring.UnitRendering.SetProjectileLuaDraw(pieceProjectileID, true)
				end
			end
			if Spring.FeatureRendering then
				Spring.FeatureRendering.SetFeatureLuaDraw(featureID, true)
			end
			-- GetFeatureTransformMatrix is the unsynced cached render matrix. Keep
			-- the fixture's matrix refreshed on every render pass so the parity
			-- comparison does not depend on whether the two processes happened to
			-- visit the feature drawer before frame four.
			if Spring.SetFeatureAlwaysUpdateMatrix then
				Spring.SetFeatureAlwaysUpdateMatrix(featureID, true)
			end
			local unitX, unitY, unitZ = Spring.GetUnitPosition(unitID)
			if unitX then
				-- Keep the opt-in objects in the active frustum so DrawUnit and
				-- DrawFeature are exercised by the rendering-enabled run.
				Spring.SetCameraTarget(unitX, unitY, unitZ, 0)
			end

			-- LuaDebugExtra feeds these through the ordinary input pipeline while
			-- keeping the test independent of the physical mouse and keyboard.
			local viewSizeX, viewSizeY = Spring.GetViewGeometry()
			local debugFunctions = {
				"emulateMousePress",
				"emulateMouseMove",
				"emulateMouseRelease",
				"emulateMouseWheel",
				"emulateKeyPress",
				"emulateKeyRelease",
				"clearEmulatedInput",
			}
			for _, name in ipairs(debugFunctions) do
				if not debug or type(debug[name]) ~= "function" then
					error("missing debug Lua-only helper " .. name, 0)
				end
			end
			local mouseX = math.floor(viewSizeX * 0.5)
			local mouseY = math.floor(viewSizeY * 0.5)
			debug.emulateMousePress(1)
			-- MouseMove is dispatched to the event handler only while a
			-- consumer owns the button.  Exercise a real press -> move ->
			-- release sequence.
			debug.emulateMouseMove(mouseX, mouseY)
			debug.emulateMouseRelease(1)
			debug.emulateMouseWheel(1)
			record("debug.mouse_wheel_injection", { called = true })
			debug.emulateKeyPress(string.byte("a"))
			debug.emulateKeyRelease(string.byte("a"))
			debug.clearEmulatedInput()
			record("debug.input_injection", {
				called = true,
				mouseX = mouseX,
				mouseY = mouseY,
			})

			-- Enter the engine's normal CEventHandler path with valid fixture
			-- objects.  This is test infrastructure exposed only by the debug
			-- library; it lets both Lua and native consumers observe the same
			-- engine-constructed callin payloads deterministically.  The driver
			-- includes renderer-only callbacks (for example DrawWorldShadow),
			-- which are unavailable in a headless engine.
			if Common.enableRenderingTests() then
				if not debug or type(debug.emulateNativeApiParityCallins) ~= "function" then
					error("missing debug Lua-only helper emulateNativeApiParityCallins", 0)
				end
				debug.emulateNativeApiParityCallins(unitID, featureID, projectileID)
				record("debug.callin_driver", { called = true })
			end

			if unitX then
				-- The deterministic input driver includes a wheel event.  Re-anchor
				-- the camera after it so the camera conversion checks do not depend
				-- on how much render time elapsed in either process.
				Spring.SetCameraTarget(unitX, unitY, unitZ, 0)
			end

			-- These controls synchronously emit the corresponding GUI callins.
			Spring.SetActiveCommand("move")
			Spring.SetActiveCommand(nil)
			Spring.SetMiniMapRotation(math.pi * 0.5)
			Spring.SendCommands("minimap minimize 1", "minimap minimize 0")
			local hasLuaUIFixture = Script.LuaUI("NativeApiParityFixture")
			if hasLuaUIFixture then
				Script.LuaUI.NativeApiParityFixture(unitID, featureID, unitDefID, featureDefID, weaponDefID, projectileID, pieceProjectileID, teamID, allyTeamID, groundDecalID, enemyLosUnitID, enemyRadarUnitID, enemyHiddenUnitID)
			end
			return
		elseif name == "native_api_parity_render_fixture" then
			local unitID, featureID, unitDefID, featureDefID, projectileID, teamID = ...
			if Script.LuaUI("NativeApiParityRenderFixture") then
				Script.LuaUI.NativeApiParityRenderFixture(unitID, featureID, unitDefID, featureDefID, projectileID, teamID)
			end
			return
		elseif name == "native_api_parity_result" then
			local stream, encodedPayload = ...
			forward(stream, encodedPayload)
		elseif name == "native_api_wasm_result" then
			forward("wasm", ...)
		elseif name == "native_api_wasm_reference_time" then
			if not dynamicTimeReferenceSent then
				dynamicTimeReferenceSent = true
				local ok, seconds = pcall(Spring.GetGameSecondsInterpolated)
				local payload = {
					source = "lua-api",
					context = wasmContext,
					frame = Spring.GetGameFrame(),
					testName = "get_game_seconds_interpolated",
					status = ok and seconds ~= nil and "pass" or "fail",
					fields = ok and { seconds = seconds } or {},
				}
				if not ok or seconds == nil then
					payload.error = tostring(seconds)
				end
				forward("wasm", Common.encode(payload))
			end
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
			if Common.mode() ~= "wasm" or Common.enableRenderingTests() then
				ensureGroundDecal()
			end
			if fixtureIDs.unitID and Spring.UnitRendering then
				Spring.UnitRendering.SetUnitLuaDraw(fixtureIDs.unitID, true)
				Spring.UnitRendering.SetProjectileLuaDraw(fixtureIDs.projectileID, true)
			end
			if fixtureIDs.featureID and Spring.FeatureRendering then
				Spring.FeatureRendering.SetFeatureLuaDraw(fixtureIDs.featureID, true)
			end
			sendInventory()
			if Common.mode() ~= "wasm" or Common.enableRenderingTests() then
				runGeneratedTests()
			end
			if Common.mode() ~= "wasm" or Common.enableRenderingTests() then
				runVfsArchiveSurface()
			end
		end
		if frame == wasmReferenceFrame then
			runWasmApiReference()
		end
		local wasmFixtureFrame = (Common.mode() == "wasm"
			and (wasmContext == "unsynced_gadget" or wasmContext == "gaia_unsynced"))
			and 60 or 20
		if frame == wasmFixtureFrame then
			sendInventory()
			if Common.mode() ~= "wasm" then
				runGeneratedTests()
				runProcessTests()
			end
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
local LuaScriptSurfaceTests = VFS.Include("LuaRules/Utilities/lua_script_surface_tests.lua")
local wasmContext = tostring((Spring.GetModOptions() or {}).native_api_parity_wasm_context or "synced_gadget")
local wasmSpecPath = wasmContext == "synced_gadget"
	and "LuaRules/Utilities/wasm_api_probe_tests.lua"
	or "LuaRules/Utilities/wasm_api_probe_tests_" .. wasmContext .. ".lua"
local WasmProbeSpec = VFS.Include(wasmSpecPath)
local WasmProbeTests = WasmProbeSpec.tests or WasmProbeSpec
local WasmProbeValues = WasmProbeSpec.values or {}
	local syncedParityOptions = Spring.GetModOptions() or {}
	local syncedProcessStage = tostring(syncedParityOptions.native_api_parity_process_stage or "initial")
	local syncedWasmRole = tostring(syncedParityOptions.native_api_parity_wasm_role or "combined")

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
	if name == "complete" then
		SendToUnsynced("native_api_callin_phase", "complete")
	end
	if Common.mode() == "native" then
		Spring.InvokeNativeModule(encoded)
	end
end

local function sendWasmParity(payload)
	SendToUnsynced("native_api_wasm_result", Common.encode(payload))
end

-- Keep these signatures deliberately representation-based.  The Wasm fixture
-- computes the same values with f32 operations and a wrapping integer LCG.
-- The Lua side derives the integer signature independently from the observed
-- unit count, so rendering-only fixture objects do not invalidate the check.
local function wasmDeterminismFpSignature()
	return "80000000:00000001:1:40000000:1:-1:2147483647:0"
end

local function wasmU32Bytes(value)
	local bytes = {}
	for index = 1, 4 do
		bytes[index] = value % 256
		value = math.floor(value / 256)
	end
	return bytes
end

local function wasmLcgStep(state)
	-- LuaRules numbers are float32 on this engine.  Keep the LCG in base 256
	-- so every product and carry is exactly representable instead of losing
	-- the low bits of a 32-bit multiplication.
	local multiplier = {13, 102, 25, 0} -- 1664525, little endian
	local increment = {95, 243, 110, 60} -- 1013904223, little endian
	local result = {}
	local carry = 0
	for index = 1, 4 do
		local total = increment[index] + carry
		for factor = 1, index do
			total = total + state[factor] * multiplier[index - factor + 1]
		end
		result[index] = total % 256
		carry = math.floor(total / 256)
	end
	return result
end

local function wasmU32Hex(bytes)
	return string.format("%02x%02x%02x%02x", bytes[4], bytes[3], bytes[2], bytes[1])
end

local function wasmDeterminismRngSignature(frame, count)
	local state = wasmU32Bytes(frame * 31 + count)
	local values = {}
	for index = 1, 3 do
		state = wasmLcgStep(state)
		values[index] = wasmU32Hex(state)
	end
	return table.concat(values, ":")
end

-- Script.* is intentionally Lua-only: it operates on the embedded Lua
-- handle, not on the native module ABI.  Keep its results in the shared Lua
-- result stream so Lua-vs-native process runs still prove deterministic Lua
-- behavior, but do not send these rows to the native checker as if they were
-- native API requests.
local function sendLuaOnly(name, payload)
	Common.setTestName(payload, name)
	payload.context = "synced_gadget"
	SendToUnsynced("native_api_parity_result", "synced_gadget", Common.encode(payload))
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
	if param.fixture_list then
		return { ids[param.fixture_list] }
	end
	if param.fixture_map then
		return { [ids[param.fixture_map]] = true }
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

local function generatedArg(spec, ids, value, locals)
	if type(spec) == "table" then
		local resolved = {}
		for key, item in pairs(spec) do
			resolved[key] = generatedArg(item, ids, value, locals)
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
	local localKey = spec:match("^local%.(.+)$")
	if localKey then
		return locals and locals[localKey]
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
			value = nil
			break
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
	elseif returnSpec.transform == "string_hex" then
		if type(value) == "string" then
			local bytes = {}
			for index = 1, #value do
				bytes[#bytes + 1] = string.format("%02x", string.byte(value, index))
			end
			value = table.concat(bytes)
		else
			value = ""
		end
	elseif returnSpec.transform == "table_values" then
		local values = {}
		if type(value) == "table" then
			for _, item in ipairs(value) do
				values[#values + 1] = item
			end
		end
		value = values
	elseif returnSpec.transform == "nil_to_minus_one" then
		if value == nil then
			value = -1
		end
	elseif returnSpec.transform == "nil_to_empty" then
		if value == nil then
			value = ""
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
				for key, unitID in pairs(units) do
					if type(key) == "number" and type(unitID) == "number" then
						unitIDs[#unitIDs + 1] = unitID
					end
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
				for key, unitID in pairs(units) do
					if type(key) == "number" and type(unitID) == "number" then
						unitIDs[#unitIDs + 1] = unitID
					end
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
	local locals = {}
	for name, localSpec in pairs(runtime.locals or {}) do
		local func = resolveLuaFunction(localSpec.call)
		local args = {}
		local argCount = 0
		for _, argSpec in ipairs(localSpec.args or {}) do
			argCount = argCount + 1
			args[argCount] = generatedArg(argSpec, ids, value, locals)
		end
		locals[name] = func(unpack(args, 1, argCount))
	end

	if runtime.table then
		local tableValue = _G[runtime.table]
		if type(tableValue) ~= "table" then
			error("Lua API value is not a table: " .. tostring(runtime.table))
		end
		local key = generatedArg(runtime.key, ids, value, locals)
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
		args[argCount] = generatedArg(argSpec, ids, value, locals)
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
	local enemyTeamID = 1
	local baseX = randFloat(880, 1180)
	local baseZ = randFloat(880, 1180)
	local unitID = Spring.CreateUnit("native_api_test_unit", baseX, 96, baseZ, randInt(0, 3), teamID, false, false)
	local extractorUnitID = Spring.CreateUnit("native_api_test_extractor", baseX + 64, 96, baseZ - 64, 0, teamID, false, false)
	-- Lua unit scripts are created lazily.  The piece/script parity checks need
	-- the same CLuaUnitScript piece map that a real script-backed unit has.
	if unitID and Spring.UnitScript and Spring.UnitScript.CreateScript then
		Spring.UnitScript.CreateScript(unitID, {})
	end
	-- Keep a three-state visibility fixture alive alongside the ordinary
	-- team-owned unit.  The observer is ally-team 0; the units belong to the
	-- second ally-team and are explicitly placed in LOS, radar-only, and hidden
	-- states.  This is intentionally created in synced Lua so native and Lua
	-- parity runs see the same engine-owned state.
	local enemyLosUnitID = Spring.CreateUnit("native_api_test_unit", baseX + 192, 96, baseZ, 0, enemyTeamID, false, false)
	local enemyRadarUnitID = Spring.CreateUnit("native_api_test_unit", baseX + 256, 96, baseZ, 0, enemyTeamID, false, false)
	local enemyHiddenUnitID = Spring.CreateUnit("native_api_test_unit", baseX + 320, 96, baseZ, 0, enemyTeamID, false, false)
	for _, entry in ipairs({
		{ id = enemyLosUnitID, mask = 1 },
		{ id = enemyRadarUnitID, mask = 2 },
		{ id = enemyHiddenUnitID, mask = 0 },
	}) do
		if entry.id then
			Spring.SetUnitLosMask(entry.id, 0, 15)
			Spring.SetUnitLosState(entry.id, 0, entry.mask)
		end
	end
	local featureInputX = baseX + randFloat(24, 80)
	local featureInputZ = baseZ + randFloat(24, 80)
	local featureFacing = randInt(0, 3)
	local featureID = Spring.CreateFeature("native_api_test_feature", featureInputX, 96, featureInputZ, featureFacing, teamID)
	local unitDefID = Spring.GetUnitDefID(unitID)
	local featureDefID = Spring.GetFeatureDefID(featureID)
	local unitDef = UnitDefs[unitDefID]
	-- These callbacks are deliberately gated by the Lua watch masks.  Register
	-- the fixture definitions before the deterministic engine callin driver
	-- runs, so Lua and native observe the same collision/movement events.
	Script.SetWatchUnit(unitDefID, true)
	Script.SetWatchFeature(featureDefID, true)
	local weaponEntry = unitDef and unitDef.weapons and (unitDef.weapons[1] or unitDef.weapons[0])
	local weaponDefID = weaponEntry and weaponEntry.weaponDef
	if weaponDefID then
		-- These callins are opt-in on the Lua side.  Register the same fixture
		-- weapon before spawning it so Lua and native receive the same watched
		-- projectile/explosion/target events.
		Script.SetWatchWeapon(weaponDefID, true)
		Script.SetWatchExplosion(weaponDefID, true)
	end
	-- Piece projectiles use the special -1 watch slot.  Register it so the
	-- callin fixture exercises the complete ProjectileCreated/Destroyed
	-- argument contract, including non-weapon projectiles.
	Script.SetWatchProjectile(-1, true)
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
		enemyTeamID = enemyTeamID,
		enemyAllyTeamID = 1,
		unitID = unitID,
		extractorUnitID = extractorUnitID,
		allyUnitID = unitID,
		enemyLosUnitID = enemyLosUnitID,
		enemyRadarUnitID = enemyRadarUnitID,
		enemyHiddenUnitID = enemyHiddenUnitID,
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
	for _, unitID in ipairs({
		ids.unitID,
		ids.extractorUnitID,
		ids.enemyLosUnitID,
		ids.enemyRadarUnitID,
		ids.enemyHiddenUnitID,
	}) do
		if unitID and (not Spring.ValidUnitID or Spring.ValidUnitID(unitID)) then
			-- Test fixtures are owned by this case.  Use Lua's immediate-cleanup
			-- option so the next randomized case cannot observe a dead fixture.
			Spring.DestroyUnit(unitID, false, true, nil, true)
		end
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
local TEST_BY_NAME = {}
for _, test in ipairs(TESTS) do
	TEST_BY_NAME[test.name] = test
end
local persistentFixture
local renderFixture
local ranDeferredSyncedChecks = false
local ranLuaScriptSurfaceTests = false

local function runWasmApiReference(frame)
	if syncedWasmRole == "guest" then
		return
	end
	if wasmContext ~= "synced_gadget" and wasmContext ~= "gaia_synced" then
		return
	end
	if persistentFixture == nil then
		return
	end

	local probeTests = WasmProbeTests
	if selectedTestsOption ~= "" then
		probeTests = {}
		for _, testName in ipairs(WasmProbeTests) do
			if selectedTest(testName) then
				probeTests[#probeTests + 1] = testName
			end
		end
	end

	for _, testName in ipairs(probeTests) do
		local test = TEST_BY_NAME[testName]
		if test == nil then
			error("Wasm parity probe test is not present in the synced API fixture: " .. testName, 0)
		end
		local value = test.make(persistentFixture)
		for key, probeValue in pairs(WasmProbeValues[testName] or {}) do
			value[key] = probeValue
		end
		local ok, readback = pcall(function()
			if test.kind == "setter_getter" then
				test.set(persistentFixture, value)
			end
			return test.get(persistentFixture, value)
		end)
		local payload = {
			source = "lua-api",
			frame = frame,
			testName = testName,
			status = ok and "pass" or (test.expect_error and "error" or "fail"),
			fields = {},
		}
		if ok then
			for _, field in ipairs((test.compare or {}).fields or {}) do
				local fieldValue = readback[field]
				if fieldValue == nil and test.compare ~= nil and test.compare.missing ~= nil then
					fieldValue = test.compare.missing[field]
				end
				payload.fields[field] = fieldValue
			end
		elseif test.expect_error then
			payload.fields.error = true
		else
			payload.error = tostring(readback)
		end
		sendWasmParity(payload)
	end
end

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

local function visibilitySnapshot(unitID, allyTeamID)
	local rawMask = Spring.GetUnitLosState(unitID, allyTeamID, true) or 0
	local state = Spring.GetUnitLosState(unitID, allyTeamID, false) or {}
	return {
		rawMask = rawMask % 16,
		los = state.los == true,
		radar = state.radar == true,
		typed = state.typed == true,
	}
end

function gadget:GameFrame(frame)
	if syncedProcessStage == "resume" then
		return
	end
	if frame == 1 then
		persistentFixture = Fixture.create()
		if Common.mode() ~= "wasm" then
			if not debug or type(debug.emulateUnitMoveFailed) ~= "function" then
				error("missing debug Lua-only helper emulateUnitMoveFailed", 0)
			end
			debug.emulateUnitMoveFailed(persistentFixture.unitID)
			sendLuaOnly("debug.unit_move_failed_driver", { called = true })
		end
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
			persistentFixture.allyTeamID,
			persistentFixture.enemyLosUnitID,
			persistentFixture.enemyRadarUnitID,
			persistentFixture.enemyHiddenUnitID
		)
	end
	local wasmDeterminismFrame = (Common.mode() == "wasm"
		and Common.enableRenderingTests()
		and (wasmContext == "unsynced_gadget" or wasmContext == "gaia_unsynced"))
		and 10 or 3
	if frame == wasmDeterminismFrame then
		local teamUnitCount = Spring.GetTeamUnitCount(0) or 0
		sendWasmParity({
			source = "lua",
			-- The unsynced Wasm guest reports its deterministic sample after
			-- renderer setup; keep the logical sample identity at frame three
			-- while observing the same post-setup unit inventory.
			frame = 3,
			teamUnitCount = teamUnitCount,
			fpEdgeSignature = wasmDeterminismFpSignature(),
			rngSignature = wasmDeterminismRngSignature(3, teamUnitCount),
		})
	end
	if frame == 2 then
		if Common.mode() ~= "wasm" and not ranLuaScriptSurfaceTests and persistentFixture ~= nil then
			ranLuaScriptSurfaceTests = true
			LuaScriptSurfaceTests.run(sendLuaOnly, persistentFixture)
		end
		if Common.mode() ~= "wasm" then
			runSyncedChecks()
		end
		local ids = persistentFixture
		if ids then
			send("multi_ally_visibility", {
				observerAllyTeamID = 0,
				allyUnitID = ids.allyUnitID,
				enemyLosUnitID = ids.enemyLosUnitID,
				enemyRadarUnitID = ids.enemyRadarUnitID,
				enemyHiddenUnitID = ids.enemyHiddenUnitID,
				ally = visibilitySnapshot(ids.allyUnitID, 0),
				enemyLos = visibilitySnapshot(ids.enemyLosUnitID, 0),
				enemyRadar = visibilitySnapshot(ids.enemyRadarUnitID, 0),
				enemyHidden = visibilitySnapshot(ids.enemyHiddenUnitID, 0),
			})
		end
	end
	if frame == 4 and (Common.mode() ~= "wasm" or Common.enableRenderingTests()) then
		-- The renderer's model-drawer event clients are fully initialized by
		-- this point. Keep a separate fixture for the VBO/VAO instance-data
		-- surface; objects created during frame one can legitimately predate
		-- those render-only allocations.
		renderFixture = Fixture.create()
		SendToUnsynced(
			"native_api_parity_render_fixture",
			renderFixture.unitID,
			renderFixture.featureID,
			renderFixture.unitDefID,
			renderFixture.featureDefID,
			renderFixture.projectileID,
			renderFixture.teamID
		)
	end
	if frame == 18 then
		if Common.mode() ~= "wasm" then
			runDeferredSyncedChecks()
		end
	end
	local wasmFixtureFrame = (Common.mode() == "wasm"
		and (wasmContext == "unsynced_gadget" or wasmContext == "gaia_unsynced"))
		and 120 or 20
	if frame == wasmFixtureFrame and persistentFixture ~= nil then
		Fixture.destroy(persistentFixture)
		persistentFixture = nil
	end
	if frame == wasmFixtureFrame and renderFixture ~= nil then
		Fixture.destroy(renderFixture)
		renderFixture = nil
	end
end

function gadget:GameFramePost(frame)
	if frame == 2 then
		runWasmApiReference(frame)
	end
end

function gadget:RecvLuaMsg(message, playerID)
	local probeParts = {}
	for part in string.gmatch(message .. "|", "(.-)|") do
		probeParts[#probeParts + 1] = part
	end
	if probeParts[1] == "WASM_API" then
		local fields = {}
		local status = "pass"
		local errorCode
		local index = 3
		while index <= #probeParts - 2 do
			local field = probeParts[index]
			local kind = probeParts[index + 1]
			local encoded = probeParts[index + 2]
			if field == "__error" then
				status = "error"
				errorCode = tonumber(encoded)
			else
				local value
				if kind == "b" then
					value = encoded == "1"
				elseif kind == "i" or kind == "f" then
					value = tonumber(encoded)
				elseif kind == "s" then
					local bytes = {}
					for byteIndex = 1, #encoded, 2 do
						bytes[#bytes + 1] = string.char(tonumber(encoded:sub(byteIndex, byteIndex + 1), 16))
					end
					value = table.concat(bytes)
				elseif kind == "o" then
					local optionKind, present, optionEncoded = encoded:match("^([^:]+):([01]):(.*)$")
					if optionKind == nil then
						status = "error"
					elseif present == "0" then
						value = nil
					elseif optionKind == "b" then
						value = optionEncoded == "1"
					elseif optionKind == "i" or optionKind == "f" then
						value = tonumber(optionEncoded)
					elseif optionKind == "s" then
						local bytes = {}
						for byteIndex = 1, #optionEncoded, 2 do
							bytes[#bytes + 1] = string.char(tonumber(optionEncoded:sub(byteIndex, byteIndex + 1), 16))
						end
						value = table.concat(bytes)
					else
						status = "error"
					end
				elseif kind == "ln" then
					value = {}
					if encoded ~= "" then
						for item in string.gmatch(encoded, "[^,]+") do
							value[#value + 1] = tonumber(item)
						end
					end
				elseif kind == "lb" then
					value = {}
					if encoded ~= "" then
						for item in string.gmatch(encoded, "[^,]+") do
							value[#value + 1] = item == "1"
						end
					end
					elseif kind == "ls" then
						value = {}
					if encoded ~= "" then
						for item in string.gmatch(encoded, "[^,]+") do
							if item ~= "" then
								local bytes = {}
								for byteIndex = 1, #item, 2 do
									bytes[#bytes + 1] = string.char(tonumber(item:sub(byteIndex, byteIndex + 1), 16))
								end
								value[#value + 1] = table.concat(bytes)
							end
							end
						end
					elseif kind == "lr" then
						-- Structured list results use a deliberately small, escaped-free
						-- wire format: records are separated by ';' and scalar fields by
						-- commas, with each field encoded as name:kind:value.  Generated
						-- record fields use only identifier characters, numeric values, or
						-- hex strings, so the delimiters cannot collide with payload data.
						value = {}
						if encoded ~= "" then
							for itemEncoded in string.gmatch(encoded, "[^;]+") do
								local item = {}
								for fieldName, fieldKind, fieldEncoded in string.gmatch(itemEncoded, "([^,:]+):([^,:]+):([^,]*)") do
									if fieldKind == "b" then
										item[fieldName] = fieldEncoded == "1"
									elseif fieldKind == "i" or fieldKind == "f" then
										item[fieldName] = tonumber(fieldEncoded)
									elseif fieldKind == "s" then
										local bytes = {}
										for byteIndex = 1, #fieldEncoded, 2 do
											bytes[#bytes + 1] = string.char(tonumber(fieldEncoded:sub(byteIndex, byteIndex + 1), 16))
										end
										item[fieldName] = table.concat(bytes)
									elseif fieldKind == "li" then
										item[fieldName] = {}
										if fieldEncoded ~= "" then
											for unitID in string.gmatch(fieldEncoded, "[^,]+") do
												item[fieldName][#item[fieldName] + 1] = tonumber(unitID)
											end
										end
									end
								end
								value[#value + 1] = item
							end
						end
					else
					status = "error"
				end
				fields[field] = value
			end
			index = index + 3
		end
		if Common.mode() == "wasm"
			and Common.enableRenderingTests()
			and probeParts[2] == "get_game_seconds_interpolated"
		then
			-- The interpolated clock advances while the renderer is drawing. Ask
			-- the unsynced Lua handle to sample it when this exact Wasm result
			-- arrives, rather than comparing values captured in different frames.
			SendToUnsynced("native_api_wasm_reference_time")
		end
		sendWasmParity({
			source = "wasm-api",
			frame = Spring.GetGameFrame and Spring.GetGameFrame() or 0,
			testName = probeParts[2],
			status = status,
			errorCode = errorCode,
			fields = fields,
			playerID = playerID,
		})
		return false
	end
	if probeParts[1] == "WASM_API_STATUS" then
		local status = probeParts[3] == "ready" and "pass" or "error"
		sendWasmParity({
			source = "wasm-status",
			frame = Spring.GetGameFrame and Spring.GetGameFrame() or 0,
			testName = probeParts[2] or "fixture",
			status = status,
			reason = probeParts[4] or "",
			playerID = playerID,
		})
		return false
	end

	local frame, teamUnitCount, fpEdgeSignature, rngSignature = message:match(
		"^WASM_PARITY|([%-0-9]+)|([%-0-9]+)|([^|]+)|([^|]+)$"
	)
	if frame == nil or teamUnitCount == nil or fpEdgeSignature == nil or rngSignature == nil then
		return false
	end
	sendWasmParity({
		source = "wasm",
		frame = tonumber(frame),
		teamUnitCount = tonumber(teamUnitCount),
		fpEdgeSignature = fpEdgeSignature,
		rngSignature = rngSignature,
		playerID = playerID,
	})
	return false
end
