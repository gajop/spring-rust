local M = {}
local Common = VFS.Include("LuaRules/Utilities/native_api_parity_common.lua")

local function returnShape(call)
	local result = { n = 0 }
	local function capture(...)
		result.n = select("#", ...)
		for index = 1, result.n do
			result[index] = select(index, ...)
		end
	end
	capture(call())
	return result
end

local function tableCount(value)
	local count = 0
	if type(value) == "table" then
		for _ in pairs(value) do
			count = count + 1
		end
	end
	return count
end

local function callinShape()
	local callins = Script.GetCallInList()
	local gameFrame = type(callins) == "table" and callins.GameFrame or nil
	local gameFrameFields = {}
	if type(gameFrame) == "table" then
		for key in pairs(gameFrame) do
			gameFrameFields[#gameFrameFields + 1] = tostring(key)
		end
		table.sort(gameFrameFields)
	end
	return {
		count = tableCount(callins),
		valueType = type(callins),
		gameFrameType = type(gameFrame),
		gameFrameFields = gameFrameFields,
		gameFrameUnsynced = type(gameFrame) == "table" and gameFrame.unsynced or nil,
		gameFrameController = type(gameFrame) == "table" and gameFrame.controller or nil,
	}
end

local function watchRoundTrip(record, label, setter, getter, defID)
	local before = getter(defID)
	local offReturnCount = select("#", setter(defID, false))
	local afterOff = getter(defID)
	local onReturnCount = select("#", setter(defID, true))
	local afterOn = getter(defID)
	record("script." .. label, {
		defIDType = type(defID),
		before = before,
		afterOff = afterOff,
		afterOn = afterOn,
		offReturnCount = offReturnCount,
		onReturnCount = onReturnCount,
	})
end

function M.run(record, ids)
	local script = Script

	-- These two helpers are intentionally exercised here even though they are
	-- not native-module callouts: VFS.Include executes Lua and table.new is a
	-- Lua allocator hint.  The surface manifest records them as Lua-only so
	-- coverage cannot silently confuse “no counterpart” with “not tested”.
	local includedTests = VFS.Include("LuaRules/Utilities/generated_api_tests.lua")
	record("vfs.include", {
		valueType = type(includedTests),
		count = tableCount(includedTests),
	})
	local allocated = table.new(4, 2)
	allocated[1] = "native_api_parity"
	allocated.key = true
	record("table.new", {
		valueType = type(allocated),
		arrayValue = allocated[1],
		recordValue = allocated.key,
	})

	record("script.constants", {
		noAccessTeam = script.NO_ACCESS_TEAM,
		allAccessTeam = script.ALL_ACCESS_TEAM,
	})
	record("script.get_name", {
		valueType = type(script.GetName()),
		value = script.GetName(),
	})
	record("script.get_synced", { value = script.GetSynced() })
	record("script.permissions", {
		fullCtrl = script.GetFullCtrl(),
		fullRead = script.GetFullRead(),
		ctrlTeam = script.GetCtrlTeam(),
		readTeam = script.GetReadTeam(),
		readAllyTeam = script.GetReadAllyTeam(),
		selectTeam = script.GetSelectTeam(),
	})

	local helperAIsEnabled = Spring.AreHelperAIsEnabled()
	local permitHelperAIsReturnCount = select("#", script.PermitHelperAIs(helperAIsEnabled))
	record("script.permit_helper_ais", {
		returnCount = permitHelperAIsReturnCount,
	})

	local global = returnShape(script.GetGlobal)
	record("script.get_global", {
		returnCount = global.n,
		valueType = type(global[1]),
	})
	local registry = returnShape(script.GetRegistry)
	record("script.get_registry", {
		returnCount = registry.n,
		valueType = type(registry[1]),
	})

	record("script.get_callin_list", callinShape())
	record("script.version_checks", {
		minimum = script.IsEngineMinVersion(0, 0, 0),
		future = script.IsEngineMinVersion(999999, 0, 0),
	})

	local actionFallback = "native_api_parity_action arg"
	local addedFallback = script.AddActionFallback(actionFallback, "native_api_parity help")
	local removedFallback = script.RemoveActionFallback(actionFallback)
	local removedMissingFallback = script.RemoveActionFallback(actionFallback)
	record("script.action_fallback", {
		added = addedFallback,
		removed = removedFallback,
		removedMissing = removedMissingFallback,
	})

	local watchWeaponID = ids.weaponDefID or -1
	watchRoundTrip(record, "watch_unit", script.SetWatchUnit, script.GetWatchUnit, ids.unitDefID)
	watchRoundTrip(record, "watch_feature", script.SetWatchFeature, script.GetWatchFeature, ids.featureDefID)
	watchRoundTrip(record, "watch_weapon", script.SetWatchWeapon, script.GetWatchWeapon, watchWeaponID)
	watchRoundTrip(record, "watch_explosion", script.SetWatchExplosion, script.GetWatchExplosion, watchWeaponID)
	watchRoundTrip(record, "watch_projectile", script.SetWatchProjectile, script.GetWatchProjectile, -1)
	watchRoundTrip(record, "watch_allow_target", script.SetWatchAllowTarget, script.GetWatchAllowTarget, watchWeaponID)

	local updateReturnCount = select("#", script.UpdateCallIn("GameFrame"))
	record("script.update_callin", { returnCount = updateReturnCount })

	local delayReturnCount = select("#", script.DelayByFrames(1, function(marker)
		record("script.delay_callback", { marker = marker, callback = true })
	end, "native_api_parity"))
	record("script.delay_by_frames", { returnCount = delayReturnCount })

	-- InvokeNativeModule is itself a Lua-only bridge surface. Exercise the
	-- bridge explicitly with a harness-only probe; the native module accepts
	-- this probe without treating it as an API parity getter/setter.
	local invokeReturnCount = select("#", Spring.InvokeNativeModule(Common.encode({
		testName = "spring.invoke_native_module",
	})))
	record("spring.invoke_native_module", { returnCount = invokeReturnCount })
end

return M
