return function(env)
	local randFloat = env.randFloat
	local randInt = env.randInt
	local rounded = env.rounded
	local randomPosition = env.randomPosition
	local randomGroundPoint = env.randomGroundPoint
	local randomVelocity = env.randomVelocity
	local randomFlatVelocity = env.randomFlatVelocity
	local randomDirection = env.randomDirection
	local randomResource = env.randomResource
	local unitPayload = env.unitPayload
	local featurePayload = env.featurePayload
	local groundPayload = env.groundPayload

local TEST_HOOKS = {
	{
		name = "game_frame",
		readonly = true,
		payload = groundPayload,
		make = function() return {} end,
		get = function() return { frame = Spring.GetGameFrame() } end,
	},
	{
		name = "game_seconds",
		readonly = true,
		payload = groundPayload,
		make = function() return {} end,
		get = function() return { seconds = Spring.GetGameSeconds() } end,
	},
	{
		name = "gaia_team_id",
		readonly = true,
		payload = groundPayload,
		make = function() return {} end,
		get = function() return { teamID = Spring.GetGaiaTeamID() } end,
	},
	{
		name = "heading_from_facing",
		readonly = true,
		payload = groundPayload,
		make = function() return { facing = randInt(0, 3) } end,
		get = function(_, value) return { facing = value.facing, heading = Spring.GetHeadingFromFacing(value.facing) } end,
	},
	{
		name = "facing_from_heading",
		readonly = true,
		payload = groundPayload,
		make = function() return { heading = randInt(-32768, 32767) } end,
		get = function(_, value) return { heading = value.heading, facing = Spring.GetFacingFromHeading(value.heading) } end,
	},
	{
		name = "heading_from_vector",
		readonly = true,
		payload = groundPayload,
		make = function()
			local angle = randFloat(0, math.pi * 2)
			return { x = rounded(math.sin(angle)), z = rounded(math.cos(angle)) }
		end,
		get = function(_, value) return { x = value.x, z = value.z, heading = Spring.GetHeadingFromVector(value.x, value.z) } end,
	},
	{
		name = "unit_health",
		payload = unitPayload,
		make = function()
			return {
				health = rounded(randFloat(100, 900)),
				paralyzeDamage = rounded(randFloat(0, 30)),
				captureProgress = rounded(randFloat(0, 0.3)),
				buildProgress = rounded(randFloat(0.4, 1)),
			}
		end,
		set = function(ids, value)
			Spring.SetUnitHealth(ids.unitID, {
				health = value.health,
				paralyze = value.paralyzeDamage,
				capture = value.captureProgress,
				build = value.buildProgress,
			})
		end,
		get = function(ids)
			local health, maxHealth, paralyzeDamage, captureProgress, buildProgress = Spring.GetUnitHealth(ids.unitID)
			return { health = health, maxHealth = maxHealth, paralyzeDamage = paralyzeDamage, captureProgress = captureProgress, buildProgress = buildProgress }
		end,
	},
	{
		name = "unit_max_health",
		payload = unitPayload,
		make = function() return { maxHealth = rounded(randFloat(700, 1400)) } end,
		set = function(ids, value) Spring.SetUnitMaxHealth(ids.unitID, value.maxHealth) end,
		get = function(ids)
			local _, maxHealth = Spring.GetUnitHealth(ids.unitID)
			return { maxHealth = maxHealth }
		end,
	},
	{
		name = "unit_experience",
		payload = unitPayload,
		make = function() return { experience = rounded(randFloat(0, 3)) } end,
		set = function(ids, value) Spring.SetUnitExperience(ids.unitID, value.experience) end,
		get = function(ids)
			local experience = Spring.GetUnitExperience(ids.unitID)
			return { experience = experience }
		end,
	},
	{
		name = "unit_neutral",
		payload = unitPayload,
		make = function() return { neutral = randInt(0, 1) == 1 } end,
		set = function(ids, value) Spring.SetUnitNeutral(ids.unitID, value.neutral) end,
		get = function(ids) return { neutral = Spring.GetUnitNeutral(ids.unitID) } end,
	},
	{
		name = "unit_seismic_signature",
		payload = unitPayload,
		make = function() return { seismicSignature = rounded(randFloat(0, 32)) } end,
		set = function(ids, value) Spring.SetUnitSeismicSignature(ids.unitID, value.seismicSignature) end,
		get = function(ids) return { seismicSignature = Spring.GetUnitSeismicSignature(ids.unitID) } end,
	},
	{
		name = "unit_mass",
		payload = unitPayload,
		make = function() return { mass = rounded(randFloat(20, 300)) } end,
		set = function(ids, value) Spring.SetUnitMass(ids.unitID, value.mass) end,
		get = function(ids) return { mass = Spring.GetUnitMass(ids.unitID) } end,
	},
	{
		name = "unit_armored",
		payload = unitPayload,
		make = function() return { armored = randInt(0, 1) == 1, armorMultiple = rounded(randFloat(0.2, 3)) } end,
		set = function(ids, value) Spring.SetUnitArmored(ids.unitID, value.armored, value.armorMultiple) end,
		get = function(ids)
			local armored, armorMultiple = Spring.GetUnitArmored(ids.unitID)
			return { armored = armored, armorMultiple = armorMultiple }
		end,
	},
	{
		name = "unit_costs",
		payload = unitPayload,
		make = function()
			return {
				buildTime = rounded(randFloat(10, 500)),
				metalCost = rounded(randFloat(10, 500)),
				energyCost = rounded(randFloat(10, 500)),
			}
		end,
		set = function(ids, value)
			Spring.SetUnitCosts(ids.unitID, {
				buildTime = value.buildTime,
				metalCost = value.metalCost,
				energyCost = value.energyCost,
			})
		end,
		get = function(ids)
			local buildTime, metalCost, energyCost = Spring.GetUnitCosts(ids.unitID)
			return { buildTime = buildTime, metalCost = metalCost, energyCost = energyCost }
		end,
	},
	{
		name = "unit_storage",
		payload = unitPayload,
		make = function()
			return { resource = randomResource(), amount = rounded(randFloat(50, 1000)) }
		end,
		set = function(ids, value) Spring.SetUnitStorage(ids.unitID, value.resource, value.amount) end,
		get = function(ids, value)
			local metalStorage, energyStorage = Spring.GetUnitStorage(ids.unitID)
			return { resource = value.resource, amount = value.resource == "metal" and metalStorage or energyStorage, metalStorage = metalStorage, energyStorage = energyStorage }
		end,
	},
	{
		name = "unit_harvest_storage",
		payload = unitPayload,
		make = function()
			local maxStoredMetal = rounded(randFloat(50, 200))
			local maxStoredEnergy = rounded(randFloat(75, 250))
			return {
				storedMetal = rounded(randFloat(1, maxStoredMetal - 1)),
				maxStoredMetal = maxStoredMetal,
				storedEnergy = rounded(randFloat(1, maxStoredEnergy - 1)),
				maxStoredEnergy = maxStoredEnergy,
			}
		end,
		set = function(ids, value)
			Spring.SetUnitHarvestStorage(ids.unitID, value.storedMetal, value.maxStoredMetal, value.storedEnergy, value.maxStoredEnergy)
		end,
		get = function(ids)
			local storedMetal, maxStoredMetal, storedEnergy, maxStoredEnergy = Spring.GetUnitHarvestStorage(ids.unitID)
			return { storedMetal = storedMetal, maxStoredMetal = maxStoredMetal, storedEnergy = storedEnergy, maxStoredEnergy = maxStoredEnergy }
		end,
	},
	{
		name = "unit_max_range",
		payload = unitPayload,
		make = function() return { maxRange = rounded(randFloat(100, 1200)) } end,
		set = function(ids, value) Spring.SetUnitMaxRange(ids.unitID, value.maxRange) end,
		get = function(ids) return { maxRange = Spring.GetUnitMaxRange(ids.unitID) } end,
	},
	{
		name = "unit_position",
		payload = unitPayload,
		make = randomPosition,
		set = function(ids, value) Spring.SetUnitPosition(ids.unitID, value.x, value.y, value.z) end,
		get = function(ids)
			local x, y, z = Spring.GetUnitPosition(ids.unitID)
			return { x = x, y = y, z = z }
		end,
	},
	{
		name = "unit_def_id",
		readonly = true,
		payload = unitPayload,
		make = function() return {} end,
		get = function(ids) return { defID = Spring.GetUnitDefID(ids.unitID) } end,
	},
	{
		name = "unit_team",
		readonly = true,
		payload = unitPayload,
		make = function() return {} end,
		get = function(ids) return { teamID = Spring.GetUnitTeam(ids.unitID) } end,
	},
	{
		name = "unit_ally_team",
		readonly = true,
		payload = unitPayload,
		make = function() return {} end,
		get = function(ids) return { allyTeamID = Spring.GetUnitAllyTeam(ids.unitID) } end,
	},
	{
		name = "unit_is_dead",
		readonly = true,
		payload = unitPayload,
		make = function() return {} end,
		get = function(ids) return { isDead = Spring.GetUnitIsDead(ids.unitID) } end,
	},
	{
		name = "unit_is_stunned",
		readonly = true,
		payload = unitPayload,
		make = function() return {} end,
		get = function(ids)
			local stunnedOrInBuild, stunned, beingBuilt = Spring.GetUnitIsStunned(ids.unitID)
			return { stunnedOrInBuild = stunnedOrInBuild, stunned = stunned, beingBuilt = beingBuilt }
		end,
	},
	{
		name = "unit_height",
		readonly = true,
		payload = unitPayload,
		make = function() return {} end,
		get = function(ids) return { height = Spring.GetUnitHeight(ids.unitID) } end,
	},
	{
		name = "unit_radius",
		readonly = true,
		payload = unitPayload,
		make = function() return {} end,
		get = function(ids) return { radius = Spring.GetUnitRadius(ids.unitID) } end,
	},
	{
		name = "unit_base_position",
		readonly = true,
		payload = unitPayload,
		make = function() return {} end,
		get = function(ids)
			local x, y, z = Spring.GetUnitBasePosition(ids.unitID)
			return { x = x, baseY = y, z = z }
		end,
	},
	{
		name = "unit_direction",
		readonly = true,
		payload = unitPayload,
		make = function() return {} end,
		get = function(ids)
			local frontX, frontY, frontZ = Spring.GetUnitDirection(ids.unitID)
			return { x = frontX, y = frontY, z = frontZ }
		end,
	},
	{
		name = "unit_heading",
		readonly = true,
		payload = unitPayload,
		make = function() return {} end,
		get = function(ids) return { heading = Spring.GetUnitHeading(ids.unitID) } end,
	},
	{
		name = "unit_velocity",
		payload = unitPayload,
		make = randomVelocity,
		set = function(ids, value) Spring.SetUnitVelocity(ids.unitID, value.x, value.y, value.z) end,
		get = function(ids)
			local x, y, z = Spring.GetUnitVelocity(ids.unitID)
			return { x = x, y = y, z = z }
		end,
	},
	{
		name = "feature_health",
		payload = featurePayload,
		make = function() return { health = rounded(randFloat(100, 950)), checkDestruction = false } end,
		set = function(ids, value) Spring.SetFeatureHealth(ids.featureID, value.health, value.checkDestruction) end,
		get = function(ids)
			local health = Spring.GetFeatureHealth(ids.featureID)
			return { health = health }
		end,
	},
	{
		name = "feature_max_health",
		payload = featurePayload,
		make = function() return { maxHealth = rounded(randFloat(150, 900)) } end,
		set = function(ids, value)
			Spring.SetFeatureMaxHealth(ids.featureID, value.maxHealth)
			Spring.SetFeatureHealth(ids.featureID, value.maxHealth + 500, false)
		end,
		get = function(ids)
			local health = Spring.GetFeatureHealth(ids.featureID)
			return { maxHealth = health, health = health }
		end,
	},
	{
		name = "feature_mass",
		payload = featurePayload,
		make = function() return { mass = rounded(randFloat(20, 300)) } end,
		set = function(ids, value) Spring.SetFeatureMass(ids.featureID, value.mass) end,
		get = function(ids) return { mass = Spring.GetFeatureMass(ids.featureID) } end,
	},
	{
		name = "feature_resources",
		payload = featurePayload,
		make = function()
			local defMetal = rounded(randFloat(300, 900))
			local defEnergy = rounded(randFloat(300, 900))
			return {
				metal = rounded(randFloat(0, defMetal)),
				energy = rounded(randFloat(0, defEnergy)),
				reclaimTime = rounded(randFloat(1, 500)),
				reclaimLeft = rounded(randFloat(0, 1)),
				featureDefMetal = defMetal,
				featureDefEnergy = defEnergy,
			}
		end,
		set = function(ids, value)
			Spring.SetFeatureResources(ids.featureID, value.metal, value.energy, value.reclaimTime, value.reclaimLeft, value.featureDefMetal, value.featureDefEnergy)
		end,
		get = function(ids)
			local metal, defMetal, energy, defEnergy, reclaimLeft, reclaimTime = Spring.GetFeatureResources(ids.featureID)
			return { metal = metal, featureDefMetal = defMetal, energy = energy, featureDefEnergy = defEnergy, reclaimLeft = reclaimLeft, reclaimTime = reclaimTime }
		end,
	},
	{
		name = "feature_reclaim",
		payload = featurePayload,
		make = function() return { reclaimLeft = rounded(randFloat(0, 1)) } end,
		set = function(ids, value) Spring.SetFeatureReclaim(ids.featureID, value.reclaimLeft) end,
		get = function(ids)
			local _, _, _, _, reclaimLeft = Spring.GetFeatureResources(ids.featureID)
			return { reclaimLeft = reclaimLeft }
		end,
	},
	{
		name = "feature_resurrect",
		payload = featurePayload,
		make = function() return { unitDef = "native_api_test_unit", facing = randInt(0, 3), progress = rounded(randFloat(0, 1)) } end,
		set = function(ids, value) Spring.SetFeatureResurrect(ids.featureID, value.unitDef, value.facing, value.progress) end,
		get = function(ids)
			local unitDef, facing = Spring.GetFeatureResurrect(ids.featureID)
			return { unitDef = unitDef, facing = facing }
		end,
	},
	{
		name = "feature_position",
		payload = featurePayload,
		make = function()
			local value = randomPosition()
			value.snapToGround = false
			return value
		end,
		set = function(ids, value) Spring.SetFeaturePosition(ids.featureID, value.x, value.y, value.z, value.snapToGround) end,
		get = function(ids)
			local x, y, z = Spring.GetFeaturePosition(ids.featureID)
			return { x = x, y = y, z = z }
		end,
	},
	{
		name = "feature_height",
		readonly = true,
		payload = featurePayload,
		make = function() return {} end,
		get = function(ids) return { height = Spring.GetFeatureHeight(ids.featureID) } end,
	},
	{
		name = "feature_radius",
		readonly = true,
		payload = featurePayload,
		make = function() return {} end,
		get = function(ids) return { radius = Spring.GetFeatureRadius(ids.featureID) } end,
	},
	{
		name = "feature_heading",
		readonly = true,
		payload = featurePayload,
		make = function() return {} end,
		get = function(ids) return { heading = Spring.GetFeatureHeading(ids.featureID) } end,
	},
	{
		name = "feature_velocity",
		payload = featurePayload,
		make = randomFlatVelocity,
		set = function(ids, value) Spring.SetFeatureVelocity(ids.featureID, value.x, value.y, value.z) end,
		get = function(ids)
			local x, y, z = Spring.GetFeatureVelocity(ids.featureID)
			return { x = x, y = y, z = z }
		end,
	},
	{
		name = "feature_direction",
		payload = featurePayload,
		make = randomDirection,
		set = function(ids, value) Spring.SetFeatureDirection(ids.featureID, value.frontX, value.frontY, value.frontZ, value.rightX, value.rightY, value.rightZ) end,
		get = function(ids)
			local frontX, frontY, frontZ, rightX, rightY, rightZ = Spring.GetFeatureDirection(ids.featureID)
			return { frontX = frontX, frontY = frontY, frontZ = frontZ, rightX = rightX, rightY = rightY, rightZ = rightZ }
		end,
	},
	{
		name = "feature_no_select",
		payload = featurePayload,
		make = function() return { noSelect = randInt(0, 1) == 1 } end,
		set = function(ids, value) Spring.SetFeatureNoSelect(ids.featureID, value.noSelect) end,
		get = function(ids) return { noSelect = Spring.GetFeatureNoSelect(ids.featureID) } end,
	},
	{
		name = "ground_height",
		payload = groundPayload,
		make = function()
			local function editCoord(mapSize)
				local high = math.max(0, (mapSize or 1024) - 64)
				local low = math.min(880, high)
				return rounded(randFloat(low, high))
			end
			local x = editCoord(Game.mapSizeX)
			local z = editCoord(Game.mapSizeZ)
			return {
				x = x,
				z = z,
				height = rounded(Spring.GetGroundHeight(x, z) + randFloat(2, 20)),
				terraform = rounded(randFloat(0, 1)),
			}
		end,
		set = function(_, value)
			Spring.SetHeightMapFunc(function()
				Spring.SetHeightMap(value.x, value.z, value.height, value.terraform)
			end)
		end,
		nativeSet = function(_, _, invoke)
			Spring.SetHeightMapFunc(invoke)
		end,
		get = function(_, value) return { x = value.x, z = value.z, height = Spring.GetGroundHeight(value.x, value.z) } end,
	},
	{
		name = "ground_orig_height",
		readonly = true,
		payload = groundPayload,
		make = randomGroundPoint,
		get = function(_, value) return { x = value.x, z = value.z, height = Spring.GetGroundOrigHeight(value.x, value.z) } end,
	},
	{
		name = "ground_normal",
		readonly = true,
		payload = groundPayload,
		make = function()
			local value = randomGroundPoint()
			value.smoothed = randInt(0, 1) == 1
			return value
		end,
		get = function(_, value)
			local x, y, z, slope = Spring.GetGroundNormal(value.x, value.z, value.smoothed)
			return { x = value.x, z = value.z, smoothed = value.smoothed, normalX = x, normalY = y, normalZ = z, slope = slope }
		end,
	},
}

local function rulesParamPayload(caseIndex, ids, values)
	values.case = caseIndex
	values.teamID = ids.teamID
	values.playerID = 0
	values.unitID = ids.unitID
	values.featureID = ids.featureID
	return values
end

local function hasRuleParam(params, paramName)
	return type(params) == "table" and params[paramName] ~= nil
end

local function addRulesParamHook(name, scope)
	local paramName = "native_api_parity_" .. name
	local function makeValue()
		return {
			scope = scope,
			paramName = paramName,
			value = rounded(randFloat(10, 900)),
			los = { public = true },
		}
	end

	local function setValue(ids, value)
		if scope == "game" then
			Spring.SetGameRulesParam(value.paramName, value.value, value.los)
		elseif scope == "team" then
			Spring.SetTeamRulesParam(ids.teamID, value.paramName, value.value, value.los)
		elseif scope == "player" then
			Spring.SetPlayerRulesParam(0, value.paramName, value.value, value.los)
		elseif scope == "unit" then
			Spring.SetUnitRulesParam(ids.unitID, value.paramName, value.value, value.los)
		elseif scope == "feature" then
			Spring.SetFeatureRulesParam(ids.featureID, value.paramName, value.value, value.los)
		end
	end

	local function getValue(ids, value)
		local readValue
		local allParams
		if scope == "game" then
			readValue = Spring.GetGameRulesParam(value.paramName)
			allParams = Spring.GetGameRulesParams()
		elseif scope == "team" then
			readValue = Spring.GetTeamRulesParam(ids.teamID, value.paramName)
			allParams = Spring.GetTeamRulesParams(ids.teamID)
		elseif scope == "player" then
			readValue = Spring.GetPlayerRulesParam(0, value.paramName)
			allParams = Spring.GetPlayerRulesParams(0)
		elseif scope == "unit" then
			readValue = Spring.GetUnitRulesParam(ids.unitID, value.paramName)
			allParams = Spring.GetUnitRulesParams(ids.unitID)
		elseif scope == "feature" then
			readValue = Spring.GetFeatureRulesParam(ids.featureID, value.paramName)
			allParams = Spring.GetFeatureRulesParams(ids.featureID)
		end
		return { value = readValue, listed = hasRuleParam(allParams, value.paramName) }
	end

	TEST_HOOKS[#TEST_HOOKS + 1] = {
		name = name,
		payload = rulesParamPayload,
		make = makeValue,
		set = setValue,
		get = getValue,
	}
end

addRulesParamHook("game_rules_param", "game")
addRulesParamHook("team_rules_param", "team")
addRulesParamHook("player_rules_param", "player")
addRulesParamHook("unit_rules_param", "unit")
addRulesParamHook("feature_rules_param", "feature")

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "get_mod_option",
	readonly = true,
	payload = groundPayload,
	make = function() return { key = "native_api_parity_cases" } end,
	get = function(_, value)
		local readValue = Spring.GetModOption(value.key)
		return { key = value.key, value = tostring(readValue), exists = readValue ~= nil }
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "get_map_option",
	readonly = true,
	payload = groundPayload,
	make = function() return { key = "blank_map_height" } end,
	get = function(_, value)
		local readValue = Spring.GetMapOption(value.key)
		return { key = value.key, value = tostring(readValue), exists = readValue ~= nil }
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "get_wind",
	readonly = true,
	payload = groundPayload,
	make = function() return {} end,
	get = function()
		local _, _, _, windStrength = Spring.GetWind()
		return { windStrength = windStrength }
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "team_resource",
	payload = groundPayload,
	make = function()
		return {
			teamID = 0,
			resource = randomResource(),
			amount = rounded(randFloat(100, 900)),
		}
	end,
	set = function(_, value)
		Spring.SetTeamResource(value.teamID, value.resource, value.amount)
	end,
	get = function(_, value)
		local currentLevel, storage = Spring.GetTeamResources(value.teamID, value.resource)
		return {
			teamID = value.teamID,
			resource = value.resource,
			currentLevel = currentLevel,
			storage = storage,
		}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "get_unit_array_centroid",
	readonly = true,
	payload = unitPayload,
	make = function() return {} end,
	get = function(ids)
		local x, y, z = Spring.GetUnitArrayCentroid({ ids.unitID })
		return { x = x, y = y, z = z }
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "get_unit_map_centroid",
	readonly = true,
	payload = unitPayload,
	make = function() return {} end,
	get = function(ids)
		local x, y, z = Spring.GetUnitMapCentroid({ [ids.unitID] = true })
		return { x = x, y = y, z = z }
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "get_unit_tooltip",
	readonly = true,
	payload = unitPayload,
	make = function() return {} end,
	get = function(ids)
		return { tooltip = Spring.GetUnitTooltip(ids.unitID) }
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "unit_physical_state_bit",
	payload = unitPayload,
	make = function() return { stateBit = 11 } end,
	set = function(ids, value) Spring.SetUnitPhysicalStateBit(ids.unitID, value.stateBit) end,
	get = function(ids, value)
		return { stateBit = value.stateBit, physicalState = Spring.GetUnitPhysicalState(ids.unitID) }
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "feature_fire_time",
	payload = featurePayload,
	make = function() return { fireTime = rounded(randFloat(1, 12)) } end,
	set = function(ids, value) Spring.SetFeatureFireTime(ids.featureID, value.fireTime) end,
	get = function(ids, value)
		return { fireTime = Spring.GetFeatureFireTime(ids.featureID), requestedFireTime = value.fireTime }
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "feature_smoke_time",
	payload = featurePayload,
	make = function() return { smokeTime = rounded(randFloat(1, 12)) } end,
	set = function(ids, value) Spring.SetFeatureSmokeTime(ids.featureID, value.smokeTime) end,
	get = function(ids, value)
		return { smokeTime = Spring.GetFeatureSmokeTime(ids.featureID), requestedSmokeTime = value.smokeTime }
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "get_feature_blocking",
	readonly = true,
	payload = featurePayload,
	make = function() return {} end,
	get = function(ids)
		local isBlocking, isSolidObjectCollidable, isProjectileCollidable, isRaySegmentCollidable, crushable, blockEnemyPushing, blockHeightChanges = Spring.GetFeatureBlocking(ids.featureID)
		return {
			isBlocking = isBlocking,
			isSolidObjectCollidable = isSolidObjectCollidable,
			isProjectileCollidable = isProjectileCollidable,
			isRaySegmentCollidable = isRaySegmentCollidable,
			crushable = crushable,
			blockEnemyPushing = blockEnemyPushing,
			blockHeightChanges = blockHeightChanges,
		}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "get_team_lua_ai",
	readonly = true,
	payload = groundPayload,
	make = function() return {} end,
	get = function(ids)
		local teamID = ids.teamID
		local luaAI = Spring.GetTeamLuaAI(teamID)
		return { teamID = teamID, hasLuaAI = luaAI ~= nil, luaAI = luaAI or "" }
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "get_player_controlled_unit",
	readonly = true,
	payload = groundPayload,
	make = function() return { playerID = 0 } end,
	get = function(_, value)
		local unitID = Spring.GetPlayerControlledUnit(value.playerID)
		return { playerID = value.playerID, hasUnit = unitID ~= nil, unitIDResult = unitID or -1 }
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "get_position_los_state",
	readonly = true,
	payload = groundPayload,
	make = function()
		local value = randomGroundPoint()
		value.y = Spring.GetGroundHeight(value.x, value.z)
		value.allyTeamID = 0
		return value
	end,
	get = function(_, value)
		local inLosOrRadar, inLos, inRadar, inJammer = Spring.GetPositionLosState(value.x, value.y, value.z, value.allyTeamID)
		return { x = value.x, y = value.y, z = value.z, allyTeamID = value.allyTeamID, inLosOrRadar = inLosOrRadar, inLos = inLos, inRadar = inRadar, inJammer = inJammer }
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "radar_error_params",
	payload = groundPayload,
	make = function()
		return {
			allyTeamID = 0,
			radarErrorSize = rounded(randFloat(16, 96)),
			baseRadarErrorSize = rounded(randFloat(4, 48)),
			baseRadarErrorMult = rounded(randFloat(0.25, 2.5)),
		}
	end,
	set = function(_, value)
		Spring.SetRadarErrorParams(value.allyTeamID, value.radarErrorSize, value.baseRadarErrorSize, value.baseRadarErrorMult)
	end,
	get = function(_, value)
		local radarErrorSize, baseRadarErrorSize, baseRadarErrorMult = Spring.GetRadarErrorParams(value.allyTeamID)
		return {
			allyTeamID = value.allyTeamID,
			radarErrorSize = radarErrorSize,
			baseRadarErrorSize = baseRadarErrorSize,
			baseRadarErrorMult = baseRadarErrorMult,
		}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "unit_los_state",
	payload = unitPayload,
	make = function()
		return { allyTeamID = 0, rawMask = randInt(0, 15) }
	end,
	set = function(ids, value)
		Spring.SetUnitLosMask(ids.unitID, value.allyTeamID, 15)
		Spring.SetUnitLosState(ids.unitID, value.allyTeamID, value.rawMask)
	end,
	get = function(ids, value)
		local rawMask = Spring.GetUnitLosState(ids.unitID, value.allyTeamID, true) or 0
		local state = Spring.GetUnitLosState(ids.unitID, value.allyTeamID, false) or {}
		return {
			unitID = ids.unitID,
			allyTeamID = value.allyTeamID,
			rawMask = rawMask % 16,
			los = state.los == true,
			radar = state.radar == true,
			typed = state.typed == true,
		}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "unit_build_distance",
	payload = unitPayload,
	make = function()
		return { paramName = "buildDistance", value = rounded(randFloat(64, 512)) }
	end,
	set = function(ids, value)
		Spring.SetUnitBuildParams(ids.unitID, value.paramName, value.value)
	end,
	get = function(ids, value)
		local result = Spring.GetUnitBuildParams(ids.unitID, value.paramName)
		return { unitID = ids.unitID, paramName = value.paramName, hasValue = result ~= nil, value = result or 0 }
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "unit_build_range3d",
	payload = unitPayload,
	make = function()
		return { paramName = "buildRange3D", enabled = randInt(0, 1) == 1 }
	end,
	set = function(ids, value)
		Spring.SetUnitBuildParams(ids.unitID, value.paramName, value.enabled)
	end,
	get = function(ids, value)
		local result = Spring.GetUnitBuildParams(ids.unitID, value.paramName)
		return { unitID = ids.unitID, paramName = value.paramName, hasValue = result ~= nil, enabled = result == true }
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "get_unit_worker_task",
	readonly = true,
	payload = unitPayload,
	make = function() return {} end,
	get = function(ids)
		local cmdID, targetID = Spring.GetUnitWorkerTask(ids.unitID)
		return {
			unitID = ids.unitID,
			hasTask = cmdID ~= nil,
			cmdID = cmdID or 0,
			hasTarget = targetID ~= nil,
			targetID = targetID or 0,
		}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "get_ally_team_info",
	readonly = true,
	payload = groundPayload,
	make = function() return { allyTeamID = 0 } end,
	get = function(_, value)
		local info = Spring.GetAllyTeamInfo(value.allyTeamID) or {}
		local keys = {}
		for key in pairs(info) do
			keys[#keys + 1] = key
		end
		return { allyTeamID = value.allyTeamID, count = #keys, keys = keys }
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "get_unit_def_dimensions",
	readonly = true,
	payload = unitPayload,
	make = function() return {} end,
	get = function(ids)
		local dimensions = Spring.GetUnitDefDimensions(ids.unitDefID) or {}
		dimensions.unitDefID = ids.unitDefID
		return dimensions
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "map_square_terrain_type",
	payload = groundPayload,
	make = function()
		local value = randomGroundPoint()
		value.terrainTypeIndex = 1
		return value
	end,
	set = function(_, value)
		Spring.SetMapSquareTerrainType(value.x, value.z, value.terrainTypeIndex)
	end,
	get = function(_, value)
		local terrainTypeIndex = Spring.GetGroundInfo(value.x, value.z)
		return { x = value.x, z = value.z, terrainTypeIndex = terrainTypeIndex }
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "side_data_by_name",
	readonly = true,
	payload = groundPayload,
	make = function()
		local sides = Spring.GetSideData() or {}
		local first = sides[1] or {}
		return { sideName = first.sideName or "arm" }
	end,
	get = function(_, value)
		local startUnit, caseName = Spring.GetSideData(value.sideName)
		return {
			sideName = value.sideName,
			normalizedSideName = string.lower(caseName or value.sideName),
			caseName = caseName,
			startUnit = startUnit,
		}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "side_data_by_index",
	readonly = true,
	payload = groundPayload,
	make = function()
		return { sideIndex = 1 }
	end,
	get = function(_, value)
		local sideName, startUnit, caseName = Spring.GetSideData(value.sideIndex)
		return {
			sideIndex = value.sideIndex,
			normalizedSideName = sideName,
			caseName = caseName,
			startUnit = startUnit,
		}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "side_data_count",
	readonly = true,
	payload = groundPayload,
	make = function()
		return {}
	end,
	get = function()
		local sides = Spring.GetSideData() or {}
		return { sideCount = #sides }
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "closest_valid_position",
	readonly = true,
	payload = groundPayload,
	make = function()
		local value = randomGroundPoint()
		value.radius = rounded(randFloat(16, 96))
		value.unitDefID = UnitDefNames.native_api_test_unit.id
		value.teamID = Spring.GetGaiaTeamID()
		return value
	end,
	get = function(_, value)
		local posX, posY, posZ = Spring.GetClosestValidPosition(value.unitDefID, value.x, value.z, value.radius)
		return {
			x = value.x,
			y = value.y,
			z = value.z,
			radius = value.radius,
			unitDefID = value.unitDefID,
			teamID = value.teamID,
			hasPosition = posX ~= nil and posY ~= nil and posZ ~= nil,
		}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "closest_build_pos",
	readonly = true,
	payload = groundPayload,
	make = function()
		return {
			x = 1024,
			y = 96,
			z = 1024,
			teamID = 0,
			unitDefID = UnitDefNames.native_api_test_unit.id,
			searchRadius = 0,
			minDistance = 0,
			facing = 0,
		}
	end,
	get = function(_, value)
		local x, y, z = Spring.ClosestBuildPos(
			value.teamID,
			value.unitDefID,
			value.x,
			value.y,
			value.z,
			value.searchRadius,
			value.minDistance,
			value.facing
		)
		return {
			teamID = value.teamID,
			unitDefID = value.unitDefID,
			searchRadius = value.searchRadius,
			minDistance = value.minDistance,
			facing = value.facing,
			inputX = value.x,
			inputY = value.y,
			inputZ = value.z,
			x = x,
			y = y,
			z = z,
		}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "test_build_order",
	readonly = true,
	payload = groundPayload,
	make = function()
		return {
			x = 512,
			y = 96,
			z = 512,
			unitDefID = UnitDefNames.native_api_test_unit.id,
			facing = randInt(0, 3),
		}
	end,
	get = function(_, value)
		local status, featureID = Spring.TestBuildOrder(value.unitDefID, value.x, value.y, value.z, value.facing)
		return {
			unitDefID = value.unitDefID,
			x = value.x,
			y = value.y,
			z = value.z,
			facing = value.facing,
			status = status,
			canBuild = status ~= nil and status >= 2,
			featureID = featureID or -1,
		}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "unit_rotation",
	payload = unitPayload,
	make = function()
		return {
			pitch = rounded(randFloat(-0.6, 0.6)),
			yaw = rounded(randFloat(-3.0, 3.0)),
			roll = rounded(randFloat(-0.6, 0.6)),
		}
	end,
	set = function(ids, value) Spring.SetUnitRotation(ids.unitID, value.pitch, value.yaw, value.roll) end,
	get = function(ids)
		local pitch, yaw, roll = Spring.GetUnitRotation(ids.unitID)
		local frontX, frontY, frontZ = Spring.GetUnitDirection(ids.unitID)
		return { pitch = pitch, yaw = yaw, roll = roll, frontX = frontX, frontY = frontY, frontZ = frontZ }
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "feature_rotation",
	payload = featurePayload,
	make = function()
		return {
			pitch = rounded(randFloat(-0.6, 0.6)),
			yaw = rounded(randFloat(-3.0, 3.0)),
			roll = rounded(randFloat(-0.6, 0.6)),
		}
	end,
	set = function(ids, value) Spring.SetFeatureRotation(ids.featureID, value.pitch, value.yaw, value.roll) end,
	get = function(ids)
		local pitch, yaw, roll = Spring.GetFeatureRotation(ids.featureID)
		return { pitch = pitch, yaw = yaw, roll = roll }
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "unit_buildee_radius",
	payload = unitPayload,
	make = function() return { buildeeRadius = rounded(randFloat(4, 96)) } end,
	set = function(ids, value) Spring.SetUnitBuildeeRadius(ids.unitID, value.buildeeRadius) end,
	get = function(ids) return { buildeeRadius = Spring.GetUnitBuildeeRadius(ids.unitID) } end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "unit_blocking",
	payload = unitPayload,
	make = function()
		return {
			isBlocking = randInt(0, 1) == 1,
			isSolidObjectCollidable = randInt(0, 1) == 1,
			isProjectileCollidable = randInt(0, 1) == 1,
			isRaySegmentCollidable = randInt(0, 1) == 1,
			crushable = randInt(0, 1) == 1,
			blockEnemyPushing = randInt(0, 1) == 1,
			blockHeightChanges = randInt(0, 1) == 1,
		}
	end,
	set = function(ids, value)
		Spring.SetUnitBlocking(
			ids.unitID,
			value.isBlocking,
			value.isSolidObjectCollidable,
			value.isProjectileCollidable,
			value.isRaySegmentCollidable,
			value.crushable,
			value.blockEnemyPushing,
			value.blockHeightChanges
		)
	end,
	get = function(ids)
		local isBlocking, isSolidObjectCollidable, isProjectileCollidable, isRaySegmentCollidable, crushable, blockEnemyPushing, blockHeightChanges = Spring.GetUnitBlocking(ids.unitID)
		return {
			isBlocking = isBlocking,
			isSolidObjectCollidable = isSolidObjectCollidable,
			isProjectileCollidable = isProjectileCollidable,
			isRaySegmentCollidable = isRaySegmentCollidable,
			crushable = crushable,
			blockEnemyPushing = blockEnemyPushing,
			blockHeightChanges = blockHeightChanges,
		}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "get_team_units_sorted",
	readonly = true,
	payload = groundPayload,
	make = function() return {} end,
	get = function(ids)
		local unitsByDef = Spring.GetTeamUnitsSorted(ids.teamID) or {}
		local groups = {}
		for unitDefID, unitIDs in pairs(unitsByDef) do
			if type(unitDefID) == "number" and type(unitIDs) == "table" then
				groups[#groups + 1] = { unitDefID = unitDefID, unitIDs = unitIDs }
			end
		end
		table.sort(groups, function(a, b) return a.unitDefID < b.unitDefID end)
		return { teamID = ids.teamID, groups = groups }
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "get_unit_cmd_descs",
	readonly = true,
	payload = unitPayload,
	make = function() return {} end,
	get = function(ids)
		local descs = Spring.GetUnitCmdDescs(ids.unitID) or {}
		return { count = #descs }
	end,
}


	return TEST_HOOKS
end
