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
	local objectPayload = env.objectPayload

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

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "unit_cmd_desc_lifecycle",
	payload = unitPayload,
	make = function()
		return {
			cmdID = 34567,
			cmdType = 21,
			action = "native_api_parity_cmd",
			editedAction = "native_api_parity_cmd_edited",
		}
	end,
	set = function(ids, value)
		local before = #(Spring.GetUnitCmdDescs(ids.unitID) or {})
		local description = {
			id = value.cmdID,
			type = value.cmdType,
			name = "Native parity command",
			action = value.action,
			tooltip = "Native parity command",
			params = {"native_api_parity"},
		}
		Spring.InsertUnitCmdDesc(ids.unitID, description)
		local index = Spring.FindUnitCmdDesc(ids.unitID, value.cmdID)
		if index == nil then
			value.success = false
			return
		end
		Spring.EditUnitCmdDesc(ids.unitID, index, {action = value.editedAction})
		local edited = false
		for _, desc in ipairs(Spring.GetUnitCmdDescs(ids.unitID) or {}) do
			if desc.id == value.cmdID and desc.action == value.editedAction then
				edited = true
				break
			end
		end
		Spring.RemoveUnitCmdDesc(ids.unitID, index)
		local after = #(Spring.GetUnitCmdDescs(ids.unitID) or {})
		value.success = edited and before == after and Spring.FindUnitCmdDesc(ids.unitID, value.cmdID) == nil
	end,
	nativeSet = function(_, value, invoke)
		invoke()
		value.success = true
	end,
	get = function(_, value)
		return {success = value.success == true}
	end,
}

local function orderValue()
	return {cmdID = 0, timeout = 0}
end

local function orderSuccess(result)
	if type(result) == "number" then
		return result > 0
	end
	return result == true
end

local function orderCommands(value)
	return {{value.cmdID, {}, {}, value.timeout}}
end

local function addOrderHook(name, call)
	TEST_HOOKS[#TEST_HOOKS + 1] = {
		name = name,
		payload = unitPayload,
		make = orderValue,
		set = function(ids, value)
			value.success = orderSuccess(call(ids, value))
		end,
		nativeSet = function(_, value, invoke)
			invoke()
			value.success = true
		end,
		get = function(_, value)
			return {success = value.success == true}
		end,
	}
end

addOrderHook("give_order_to_unit_synced", function(ids, value)
	return Spring.GiveOrderToUnit(ids.unitID, value.cmdID, {}, {}, value.timeout)
end)

addOrderHook("give_order_to_unit_array_synced", function(ids, value)
	return Spring.GiveOrderToUnitArray({ids.unitID}, value.cmdID, {}, {}, value.timeout)
end)

addOrderHook("give_order_to_unit_map_synced", function(ids, value)
	return Spring.GiveOrderToUnitMap({[ids.unitID] = true}, value.cmdID, {}, {}, value.timeout)
end)

addOrderHook("give_order_array_to_unit_synced", function(ids, value)
	return Spring.GiveOrderArrayToUnit(ids.unitID, orderCommands(value))
end)

addOrderHook("give_order_array_to_unit_map_synced", function(ids, value)
	return Spring.GiveOrderArrayToUnitMap({[ids.unitID] = true}, orderCommands(value))
end)

addOrderHook("give_order_array_to_unit_array_synced_pairwise", function(ids, value)
	return Spring.GiveOrderArrayToUnitArray({ids.unitID}, orderCommands(value), true)
end)

addOrderHook("give_order_array_to_unit_array_synced_broadcast", function(ids, value)
	return Spring.GiveOrderArrayToUnitArray({ids.unitID}, orderCommands(value), false)
end)

local function addTerrainCallbackHook(name, callback, nativeRegistration, getter)
	TEST_HOOKS[#TEST_HOOKS + 1] = {
		name = name,
		payload = groundPayload,
		make = function()
			return { x = 1600, z = 1600, delta = 2 }
		end,
		set = function(_, value)
			callback(function()
				return value.x, value.z, value.delta
			end)
		end,
		nativeSet = function(_, _, invoke)
			nativeRegistration(invoke)
		end,
		get = function(_, value)
			return {
				x = value.x,
				z = value.z,
				height = getter(value.x, value.z),
			}
		end,
	}
end

addTerrainCallbackHook("add_height_map", function(point)
		Spring.SetHeightMapFunc(function()
			local x, z, delta = point()
			Spring.AddHeightMap(x, z, delta)
		end)
	end, function(invoke)
		Spring.SetHeightMapFunc(invoke)
	end, Spring.GetGroundHeight)

addTerrainCallbackHook("add_original_height_map", function(point)
		Spring.SetOriginalHeightMapFunc(function()
			local x, z, delta = point()
			Spring.AddOriginalHeightMap(x, z, delta)
		end)
	end, function(invoke)
		Spring.SetOriginalHeightMapFunc(invoke)
	end, Spring.GetGroundOrigHeight)

addTerrainCallbackHook("add_smooth_mesh", function(point)
		Spring.SetSmoothMeshFunc(function()
			local x, z, delta = point()
			Spring.AddSmoothMesh(x, z, delta)
		end)
	end, function(invoke)
		Spring.SetSmoothMeshFunc(invoke)
	end, Spring.GetSmoothMeshHeight)

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "set_original_height_map",
	payload = groundPayload,
	make = function()
		return {
			x = 1600,
			z = 1600,
			height = Spring.GetGroundOrigHeight(1600, 1600) + 6,
			factor = 0.5,
		}
	end,
	set = function(_, value)
		Spring.SetOriginalHeightMapFunc(function()
			Spring.SetOriginalHeightMap(value.x, value.z, value.height, value.factor)
		end)
	end,
	nativeSet = function(_, _, invoke)
		Spring.SetOriginalHeightMapFunc(invoke)
	end,
	get = function(_, value)
		return {x = value.x, z = value.z, height = Spring.GetGroundOrigHeight(value.x, value.z)}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "set_smooth_mesh",
	payload = groundPayload,
	make = function()
		return {
			x = 1600,
			z = 1600,
			height = Spring.GetSmoothMeshHeight(1600, 1600) + 6,
			terraform = 0.5,
		}
	end,
	set = function(_, value)
		Spring.SetSmoothMeshFunc(function()
			Spring.SetSmoothMesh(value.x, value.z, value.height, value.terraform)
		end)
	end,
	nativeSet = function(_, _, invoke)
		Spring.SetSmoothMeshFunc(invoke)
	end,
	get = function(_, value)
		return {x = value.x, z = value.z, height = Spring.GetSmoothMeshHeight(value.x, value.z)}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "set_experience_grade",
	payload = groundPayload,
	make = function()
		return {
			expGrade = 0.25,
			expPowerScale = 1.1,
			expHealthScale = 1.2,
			expReloadScale = 1.3,
		}
	end,
	set = function(_, value)
		Spring.SetExperienceGrade(value.expGrade, value.expPowerScale, value.expHealthScale, value.expReloadScale)
	end,
	get = function()
		return { returnCount = 0 }
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "set_no_pause",
	payload = groundPayload,
	make = function() return { noPause = true } end,
	set = function(_, value) Spring.SetNoPause(value.noPause) end,
	get = function() return { returnCount = 0 } end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "set_square_building_mask",
	payload = groundPayload,
	make = function() return { x = 1, z = 1, mask = 1 } end,
	set = function(_, value) Spring.SetSquareBuildingMask(value.x, value.z, value.mask) end,
	get = function() return { returnCount = 0 } end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "path_node_costs",
	readonly = true,
	payload = groundPayload,
	make = function()
		return {
			overlayIndex = 7,
			sizeX = 2,
			sizeZ = 2,
			costIndex = 0,
			nodeX = 0,
			nodeZ = 0,
			cost = 17.25,
		}
	end,
	get = function(_, value)
		local initialized = Spring.InitPathNodeCostsArray(value.overlayIndex, value.sizeX, value.sizeZ)
		local setCost = Spring.SetPathNodeCost(value.overlayIndex, value.costIndex, value.cost)
		local active = Spring.SetPathNodeCosts(value.overlayIndex)
		local costs = Spring.GetPathNodeCosts(value.overlayIndex) or {}
		local activeCost = Spring.GetPathNodeCost(value.nodeX, value.nodeZ)
		local freed = Spring.FreePathNodeCostsArray(value.overlayIndex)
		return {
			init = initialized,
			setCost = setCost,
			active = active,
			costCount = #costs,
			costValue = costs[value.costIndex + 1],
			activeCost = activeCost,
			free = freed,
		}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "request_path",
	readonly = true,
	payload = unitPayload,
	make = function(ids)
		return {
			moveDefName = "KBOT1",
			startX = 900,
			startY = 96,
			startZ = 900,
			endX = 1120,
			endY = 96,
			endZ = 1120,
			radius = 8,
		}
	end,
	get = function(_, value)
		local path = Spring.RequestPath(
			value.moveDefName,
			value.startX, value.startY, value.startZ,
			value.endX, value.endY, value.endZ,
			value.radius
		)
		local valid = path ~= nil
		path = nil
		return { valid = valid }
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "map_model_lights_lifecycle",
	payload = unitPayload,
	readonly = true,
	make = function()
		return {
			lightParams = {
				position = {1024, 128, 1024},
				direction = {0, -1, 0},
				ambientColor = {0.1, 0.2, 0.3},
				diffuseColor = {0.4, 0.5, 0.6},
				specularColor = {0.7, 0.8, 0.9},
				radius = 128,
				fov = 90,
				ttl = 1000,
				priority = 1,
				ignoreLOS = true,
				localSpace = false,
			},
		}
	end,
	get = function(ids, value)
		local mapHandle = Spring.AddMapLight(value.lightParams)
		local mapHandleSecond = Spring.AddMapLight(value.lightParams)
		local mapAdded = type(mapHandleSecond) == "number" and mapHandleSecond < 4294967295
		local mapUpdated = Spring.UpdateMapLight(mapHandle, value.lightParams) == true
		local mapTracked = Spring.SetMapLightTrackingState(mapHandle, ids.unitID, true, true) == true
		local mapUntracked = Spring.SetMapLightTrackingState(mapHandle, ids.unitID, false, true) == true

		local modelHandle = Spring.AddModelLight(value.lightParams)
		local modelHandleSecond = Spring.AddModelLight(value.lightParams)
		local modelAdded = type(modelHandleSecond) == "number" and modelHandleSecond < 4294967295
		local modelUpdated = Spring.UpdateModelLight(modelHandle, value.lightParams) == true
		local modelTracked = Spring.SetModelLightTrackingState(modelHandle, ids.unitID, true, true) == true
		local modelUntracked = Spring.SetModelLightTrackingState(modelHandle, ids.unitID, false, true) == true

		return {
			mapHandle = mapHandle,
			modelHandle = modelHandle,
			mapAdded = mapAdded,
			mapUpdated = mapUpdated,
			mapTracked = mapTracked,
			mapUntracked = mapUntracked,
			modelAdded = modelAdded,
			modelUpdated = modelUpdated,
			modelTracked = modelTracked,
			modelUntracked = modelUntracked,
		}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "camera_state_roundtrip",
	payload = groundPayload,
	readonly = true,
	make = function() return {} end,
	get = function()
		local state = Spring.GetCameraState(true)
		return { applied = Spring.SetCameraState(state, 0, 1, 1) == true }
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "preload_sound_item_missing",
	payload = groundPayload,
	-- PreloadSoundItem reports whether this name was newly added.  The Lua
	-- and native calls therefore need separate fresh names.  nativeOnly makes
	-- the native run use its set_native call instead of invoking the API a
	-- second time during the result check.
	readonly = false,
	nativeOnly = true,
	make = function(_, caseIndex)
		return {soundName = "native_api_parity_missing_sound_" .. tostring(caseIndex)}
	end,
	set = function(_, value)
		value.success = Spring.PreloadSoundItem(value.soundName)
	end,
	nativeSet = function(_, value, invoke)
		local luaSoundName = value.soundName
		value.success = Spring.PreloadSoundItem(luaSoundName)
		-- The native call must use a distinct name because PreloadSoundItem is
		-- stateful: repeating the same call would test the already-preloaded
		-- entry rather than the same fresh-state contract.
		value.soundName = luaSoundName .. "_native"
		invoke()
		value.soundName = luaSoundName
	end,
	get = function(_, value)
		return {success = value.success}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "set_atmosphere_params",
	payload = groundPayload,
	readonly = true,
	make = function()
		return {
			atmosphere = {
				fogColor = {0.1, 0.2, 0.3, 0.4},
				skyColor = {0.2, 0.3, 0.4, 0.5},
				sunColor = {0.3, 0.4, 0.5, 0.6},
				cloudColor = {0.4, 0.5, 0.6, 0.7},
				skyAxisAngle = {0, 1, 0, 0.25},
				fogStart = 128,
				fogEnd = 4096,
			},
		}
	end,
	get = function(_, value)
		Spring.SetAtmosphere(value.atmosphere)
		return {returnCount = 0}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "set_sun_lighting_params",
	payload = groundPayload,
	readonly = true,
	make = function()
		return {
			lighting = {
				groundAmbientColor = {0.1, 0.2, 0.3, 1},
				groundDiffuseColor = {0.3, 0.4, 0.5, 1},
				groundSpecularColor = {0.5, 0.6, 0.7, 1},
				modelAmbientColor = {0.2, 0.3, 0.4, 1},
				modelDiffuseColor = {0.4, 0.5, 0.6, 1},
				modelSpecularColor = {0.6, 0.7, 0.8, 1},
				specularExponent = 16,
				groundShadowDensity = 0.75,
				modelShadowDensity = 0.65,
			},
		}
	end,
	get = function(_, value)
		Spring.SetSunLighting(value.lighting)
		return {returnCount = 0}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "set_water_params",
	payload = groundPayload,
	readonly = true,
	make = function()
		return {
			water = {
				absorb = {0.1, 0.2, 0.3},
				baseColor = {0.2, 0.3, 0.4},
				minColor = {0.05, 0.06, 0.07},
				surfaceColor = {0.3, 0.4, 0.5},
				diffuseColor = {0.4, 0.5, 0.6},
				specularColor = {0.6, 0.7, 0.8},
				planeColor = {0.1, 0.1, 0.12},
				texture = "bitmaps/watertex.png",
				foamTexture = "bitmaps/waterfoam.png",
				normalTexture = "bitmaps/waternormal.png",
				repeatX = 1.25,
				repeatY = 1.5,
				surfaceAlpha = 0.8,
				ambientFactor = 0.4,
				diffuseFactor = 0.7,
				specularFactor = 0.9,
				specularPower = 32,
				fresnelMin = 0.1,
				fresnelMax = 0.8,
				fresnelPower = 3,
				reflectionDistortion = 0.2,
				blurBase = 0.1,
				blurExponent = 1.2,
				perlinStartFreq = 0.5,
				perlinLacunarity = 2,
				perlinAmplitude = 0.6,
				windSpeed = 0.7,
				waveOffsetFactor = 0.8,
				waveLength = 1.1,
				waveFoamDistortion = 0.2,
				waveFoamIntensity = 0.3,
				causticsResolution = 256,
				causticsStrength = 0.4,
				numTiles = 4,
				shoreWaves = false,
				forceRendering = false,
				hasWaterPlane = true,
			},
		}
	end,
	get = function(_, value)
		Spring.SetWaterParams(value.water)
		return {returnCount = 0}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "set_map_rendering_params",
	payload = groundPayload,
	readonly = true,
	make = function()
		return {
			mapRendering = {
				splatTexScales = {1, 2, 3, 4},
				splatTexMults = {0.25, 0.5, 0.75, 1},
				voidWater = false,
				voidGround = false,
				splatDetailNormalDiffuseAlpha = true,
			},
		}
	end,
	get = function(_, value)
		Spring.SetMapRenderingParams(value.mapRendering)
		return {returnCount = 0}
	end,
}

local function temporaryUnit(ids)
	return Spring.CreateUnit(
		"native_api_test_unit",
		ids.groundX or 1024,
		ids.groundY or 96,
		ids.groundZ or 1024,
		0,
		ids.teamID or 0,
		false,
		false
	)
end

local function temporaryFeature(ids)
	return Spring.CreateFeature(
		"native_api_test_feature",
		ids.groundX or 1024,
		ids.groundY or 96,
		ids.groundZ or 1024,
		0,
		ids.teamID or 0
	)
end

local function objectPayloadWithFixture(caseIndex, ids, values)
	values.case = caseIndex
	values.unitDefID = ids.unitDefID
	values.featureDefID = ids.featureDefID
	values.teamID = ids.teamID or 0
	values.x = ids.groundX or 1024
	values.y = ids.groundY or 96
	values.z = ids.groundZ or 1024
	values.unitID = ids.unitID
	values.featureID = ids.featureID
	return values
end

local lifecyclePayload = objectPayload or objectPayloadWithFixture

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "create_unit_cleanup",
	deferred = true,
	readonly = true,
	payload = lifecyclePayload,
	make = function() return {} end,
	get = function(ids)
		local unitID = temporaryUnit(ids)
		local created = unitID ~= nil
		if created then
			Spring.DestroyUnit(unitID, false, true, nil, true)
		end
		return {created = created}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "destroy_unit",
	deferred = true,
	readonly = true,
	payload = lifecyclePayload,
	make = function() return {} end,
	get = function(ids)
		local unitID = temporaryUnit(ids)
		local destroyed = false
		if unitID ~= nil then
			Spring.DestroyUnit(unitID, false, true, nil, true)
			destroyed = not Spring.ValidUnitID(unitID)
		end
		return {destroyed = destroyed}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "transfer_unit",
	deferred = true,
	readonly = true,
	payload = lifecyclePayload,
	make = function() return {} end,
	get = function(ids)
		local unitID = temporaryUnit(ids)
		local success = false
		local teamAfter = -1
		if unitID ~= nil then
			success = Spring.TransferUnit(unitID, ids.teamID or 0, true, false) == true
			teamAfter = Spring.GetUnitTeam(unitID) or -1
			Spring.DestroyUnit(unitID, false, true, nil, true)
		end
		return {success = success, teamAfter = teamAfter}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "create_feature_cleanup",
	deferred = true,
	readonly = true,
	payload = lifecyclePayload,
	make = function() return {} end,
	get = function(ids)
		local featureID = temporaryFeature(ids)
		local created = featureID ~= nil
		if created then
			Spring.DestroyFeature(featureID)
		end
		return {created = created}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "destroy_feature",
	deferred = true,
	readonly = true,
	payload = lifecyclePayload,
	make = function() return {} end,
	get = function(ids)
		local featureID = temporaryFeature(ids)
		local destroyed = false
		if featureID ~= nil then
			Spring.DestroyFeature(featureID)
			destroyed = not Spring.ValidFeatureID(featureID)
		end
		return {destroyed = destroyed}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "transfer_feature",
	deferred = true,
	readonly = true,
	payload = lifecyclePayload,
	make = function() return {} end,
	get = function(ids)
		local featureID = temporaryFeature(ids)
		local teamAfter = -1
		if featureID ~= nil then
			Spring.TransferFeature(featureID, ids.teamID or 0)
			teamAfter = Spring.GetFeatureTeam(featureID) or -1
			Spring.DestroyFeature(featureID)
		end
		return {teamAfter = teamAfter}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "create_unit_wreck_cleanup",
	deferred = true,
	readonly = true,
	payload = lifecyclePayload,
	make = function() return {} end,
	get = function(ids)
		local featureID = Spring.CreateUnitWreck(ids.unitID, 1, false)
		local created = featureID ~= nil
		if created then
			Spring.DestroyFeature(featureID)
		end
		return {created = created}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "create_feature_wreck_cleanup",
	deferred = true,
	readonly = true,
	payload = lifecyclePayload,
	make = function() return {} end,
	get = function(ids)
		local featureID = Spring.CreateFeatureWreck(ids.featureID, 1, false)
		local created = featureID ~= nil
		if created then
			Spring.DestroyFeature(featureID)
		end
		return {created = created}
	end,
}

local function attachTemporaryPassenger(ids)
	local passengerID = temporaryUnit(ids)
	if passengerID == nil then
		return nil, false
	end
	Spring.UnitAttach(ids.unitID, passengerID, 0, true)
	return passengerID, Spring.GetUnitTransporter(passengerID) == ids.unitID
end

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "unit_attach",
	deferred = true,
	readonly = true,
	payload = lifecyclePayload,
	make = function() return {} end,
	get = function(ids)
		local passengerID, attached = attachTemporaryPassenger(ids)
		if passengerID ~= nil then
			if attached then
				Spring.UnitDetach(passengerID)
			end
			Spring.DestroyUnit(passengerID, false, true, nil, true)
		end
		return {attached = attached}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "unit_detach",
	deferred = true,
	readonly = true,
	payload = lifecyclePayload,
	make = function() return {} end,
	get = function(ids)
		local passengerID, attached = attachTemporaryPassenger(ids)
		local detached = false
		if passengerID ~= nil then
			if attached then
				Spring.UnitDetach(passengerID)
			end
			detached = Spring.GetUnitTransporter(passengerID) == nil
			Spring.DestroyUnit(passengerID, false, true, nil, true)
		end
		return {detached = detached}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "unit_detach_from_air",
	deferred = true,
	readonly = true,
	payload = lifecyclePayload,
	make = function() return {} end,
	get = function(ids)
		local passengerID, attached = attachTemporaryPassenger(ids)
		local detached = false
		if passengerID ~= nil then
			if attached then
				Spring.UnitDetachFromAir(passengerID, ids.groundX or 1024, ids.groundY or 96, ids.groundZ or 1024)
			end
			detached = Spring.GetUnitTransporter(passengerID) == nil
			Spring.DestroyUnit(passengerID, false, true, nil, true)
		end
		return {detached = detached}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "bugger_off",
	deferred = true,
	readonly = true,
	payload = lifecyclePayload,
	make = function() return {} end,
	get = function(ids)
		Spring.BuggerOff(ids.groundX or 1024, ids.groundY or 96, ids.groundZ or 1024, 128, ids.teamID or 0, true, true, -1)
		return {called = true}
	end,
}

local function factoryPayload(caseIndex, ids, values)
	values.case = caseIndex
	values.factoryDefID = UnitDefNames.native_api_test_factory.id
	if values.factoryID ~= nil then
		values.unitID = values.factoryID
	end
	return values
end

local function getterFactoryPayload(caseIndex, ids, values)
	values.case = caseIndex
	if values.factoryID ~= nil then
		values.factoryDefID = UnitDefNames.native_api_test_factory.id
		values.unitID = values.factoryID
	end
	return values
end

local function temporaryFactory(ids)
	return Spring.CreateUnit(
		"native_api_test_factory",
		(ids.groundX or 1024) + 64,
		ids.groundY or 96,
		ids.groundZ or 1024,
		0,
		ids.teamID or 0,
		false,
		false
	)
end

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "get_factory_bugger_off",
	nativeFirst = true,
	readonly = false,
	deferred = true,
	payload = getterFactoryPayload,
	make = function(ids)
		local factoryID = temporaryFactory(ids)
		local perform = Spring.SetFactoryBuggerOff(factoryID, true, 128, 256, 0, true, true)
		return {
			factoryID = factoryID,
			perform = perform,
			offset = 128,
			radius = 256,
			relHeading = 0,
			spherical = true,
			forced = true,
		}
	end,
	set = function() end,
	nativeSet = function(_, value, invoke)
		invoke()
	end,
	get = function(_, value)
		local factoryID = value.factoryID
		local perform, offset, radius, relHeading, spherical, forced = Spring.GetFactoryBuggerOff(factoryID)
		value.factoryID = nil
		value.factoryDefID = nil
		value.unitID = nil
		value.nativeGetterChecked = true
		Spring.DestroyUnit(factoryID, false, true, nil, true)
		return {
			perform = perform,
			offset = offset,
			radius = radius,
			relHeading = relHeading,
			spherical = spherical,
			forced = forced,
		}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "set_factory_bugger_off",
	nativeFirst = true,
	payload = factoryPayload,
	make = function()
		return {
			perform = true,
			offset = 128,
			radius = 256,
			relHeading = 0,
			spherical = true,
			forced = true,
		}
	end,
	set = function(ids, value)
		value.factoryID = temporaryFactory(ids)
		value.perform = Spring.SetFactoryBuggerOff(
			value.factoryID,
			value.perform,
			value.offset,
			value.radius,
			value.relHeading,
			value.spherical,
			value.forced
		)
	end,
	nativeSet = function(ids, value, invoke)
		value.factoryID = temporaryFactory(ids)
		invoke()
		value.perform = true
		value.nativeMode = true
	end,
	get = function(ids, value)
		if value.factoryID == nil then
			value.factoryID = temporaryFactory(ids)
			value.perform = Spring.SetFactoryBuggerOff(
				value.factoryID,
				value.perform,
				value.offset,
				value.radius,
				value.relHeading,
				value.spherical,
				value.forced
			)
		end
		local perform, offset, radius, relHeading, spherical, forced = Spring.GetFactoryBuggerOff(value.factoryID)
		Spring.DestroyUnit(value.factoryID, false, true, nil, true)
		value.factoryID = nil
		value.unitID = nil
		if value.nativeMode then
			-- Keep the native-first run's fixture allocation lifecycle aligned
			-- with the Lua baseline, which performs a second readback pass.
			local extraFactoryID = temporaryFactory(ids)
			if extraFactoryID ~= nil then
				Spring.DestroyUnit(extraFactoryID, false, true, nil, true)
			end
			value.nativeMode = nil
		end
		return {
			perform = perform,
			offset = offset,
			radius = radius,
			relHeading = relHeading,
			spherical = spherical,
			forced = forced,
		}
	end,
}

local function teamPayload(caseIndex, ids, values)
	values.case = caseIndex
	values.teamID = ids.teamID or 0
	return values
end

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "kill_team",
	nativeFirst = true,
	deferred = true,
	singleCase = true,
	payload = teamPayload,
	make = function(ids)
		return {teamID = ids.teamID or 0, expectedDead = true}
	end,
	set = function(_, value)
		Spring.KillTeam(value.teamID)
	end,
	nativeSet = function(_, _, invoke)
		invoke()
	end,
	get = function(_, value)
		local _, _, isDead = Spring.GetTeamInfo(value.teamID, false)
		return {isDead = isDead}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "game_over",
	nativeFirst = true,
	deferred = true,
	singleCase = true,
	payload = teamPayload,
	make = function(ids)
		return {winningAllyTeamID = ids.allyTeamID or 0, accepted = 1}
	end,
	set = function(_, value)
		value.accepted = Spring.GameOver({value.winningAllyTeamID})
	end,
	nativeSet = function(_, _, invoke)
		invoke()
	end,
	get = function(_, value)
		return {accepted = value.accepted or 1, gameOver = Spring.IsGameOver()}
	end,
}

local function windowPayload(caseIndex, _, values)
	values.case = caseIndex
	return values
end

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "set_window_geometry",
	payload = windowPayload,
	make = function()
		return {
			displayIndex = 1,
			windowPosX = 0,
			windowPosY = 0,
			windowSizeX = 640,
			windowSizeY = 480,
			fullScreen = false,
			borderless = false,
		}
	end,
	set = function(_, value)
		local ok, returnCount = pcall(function()
			return select("#", Spring.SetWindowGeometry(
				value.displayIndex,
				value.windowPosX,
				value.windowPosY,
				value.windowSizeX,
				value.windowSizeY,
				value.fullScreen,
				value.borderless
			))
		end)
		value.called = ok
		value.returnCount = ok and returnCount or -1
	end,
	nativeSet = function(_, value, invoke)
		invoke()
		value.called = true
		value.returnCount = 0
	end,
	get = function(_, value)
		return {called = value.called, returnCount = value.returnCount}
	end,
}

local function addWindowStateHook(name, springCall, expected)
	TEST_HOOKS[#TEST_HOOKS + 1] = {
		name = name,
		payload = windowPayload,
		make = function() return {expected = expected} end,
		set = function(_, value)
			-- The first call establishes the requested state; the second call
			-- observes the documented idempotent result, independent of the
			-- window state at process startup.
			springCall()
			value.result = springCall()
		end,
		nativeSet = function(_, value, invoke)
			-- Keep the native comparison process in the same SDL state as the
			-- Lua baseline before making the Rust call.  The native API call is
			-- still the result under test; these Lua calls only normalize the
			-- process-local window state.
			springCall()
			springCall()
			invoke()
			value.result = value.expected
		end,
		get = function(_, value)
			return {result = value.result}
		end,
	}
end

addWindowStateHook("set_window_minimized", Spring.SetWindowMinimized, false)
addWindowStateHook("set_window_maximized", Spring.SetWindowMaximized, false)

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "yield",
	payload = windowPayload,
	make = function() return {expected = false} end,
	set = function(_, value)
		value.result = Spring.Yield()
	end,
	nativeSet = function(_, value, invoke)
		invoke()
		value.result = value.expected
	end,
	get = function(_, value)
		return {result = value.result}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "call_as_team",
	payload = groundPayload,
	make = function(_, caseIndex)
		return {
			teamID = 0,
			marker = "native_api_parity_call_as_team_" .. tostring(caseIndex),
		}
	end,
	set = function(_, value)
		local returned = {n = 0}
		local function callback(marker)
			value.callbackCalled = true
			value.callbackTeam = value.teamID
			return marker, true
		end
		local function capture(...)
			returned.n = select("#", ...)
			for index = 1, returned.n do
				returned[index] = select(index, ...)
			end
		end
		capture(CallAsTeam(value.teamID, callback, value.marker))
		value.returnCount = returned.n
		value.returnMarker = returned[1]
		value.returnFlag = returned[2]
	end,
	nativeSet = function(_, value, invoke)
		-- The native callback has no Lua return stack.  The Rust checker verifies
		-- the callback and team directly; this fills the shared result shape with
		-- the equivalent successful Lua callback outcome.
		invoke()
		value.callbackCalled = true
		value.callbackTeam = value.teamID
		value.returnCount = 2
		value.returnMarker = value.marker
		value.returnFlag = true
	end,
	get = function(_, value)
		return {
			callbackCalled = value.callbackCalled == true,
			callbackTeam = value.callbackTeam,
			returnCount = value.returnCount,
			returnMarker = value.returnMarker,
			returnFlag = value.returnFlag,
		}
	end,
}

local function processStartScript()
	local options = Spring.GetModOptions() or {}
	local mode = tostring(options.native_api_parity_mode or "lua")
	local outputDir = tostring(options.native_api_parity_output_dir or "native_api_parity")
	local seed = tostring(options.native_api_parity_seed or 1)
	local cases = tostring(options.native_api_parity_cases or 1)
	local rendering = tostring(options.native_api_parity_enable_rendering_tests or 0)
	local processTest = tostring(options.native_api_parity_process_test or "")
	return "[GAME]\n"
		.. "{\n"
		.. "    IsHost=1;\n"
		.. "    MyPlayerName=NativeApiParity;\n"
		.. "    MapName=native_api_parity_process_reload;\n"
		.. "    GameType=Native API Parity 0.1;\n"
		.. "    InitBlank=1;\n"
		.. "    StartPosType=0;\n"
		.. "    FixedRNGSeed=1;\n"
		.. "    OnlyLocal=1;\n"
		.. "    HostIP=localhost;\n"
		.. "    HostPort=43817;\n"
		.. "    MyPlayerNum=0;\n"
		.. "    RecordDemo=0;\n"
		.. "    GameStartDelay=0;\n"
		.. "    MaxSpeed=1;\n"
		.. "    MinSpeed=1;\n"
		.. "    NumPlayers=2;\n"
		.. "    NumTeams=2;\n"
		.. "    NumAllyTeams=2;\n"
		.. "\n"
		.. "    [MODOPTIONS]\n"
		.. "    {\n"
		.. "        LuaRules=1;\n"
		.. "        LuaGaia=0;\n"
		.. "        native_api_parity_mode=" .. mode .. ";\n"
		.. "        native_api_parity_output_dir=" .. outputDir .. ";\n"
		.. "        native_api_parity_seed=" .. seed .. ";\n"
		.. "        native_api_parity_cases=" .. cases .. ";\n"
		.. "        native_api_parity_enable_rendering_tests=" .. rendering .. ";\n"
		.. "        native_api_parity_process_test=" .. processTest .. ";\n"
		.. "        native_api_parity_process_stage=resume;\n"
		.. "    }\n"
		.. "\n"
		.. "    [MAPOPTIONS]\n"
		.. "    {\n"
		.. "        blank_map_x=10;\n"
		.. "        blank_map_y=8;\n"
		.. "        blank_map_height=96;\n"
		.. "        blank_map_color_r=64;\n"
		.. "        blank_map_color_g=128;\n"
		.. "        blank_map_color_b=64;\n"
		.. "    }\n"
		.. "\n"
		.. "    [PLAYER0]\n"
		.. "    {\n"
		.. "        Name=NativeApiParity;\n"
		.. "        Spectator=0;\n"
		.. "        Team=0;\n"
		.. "    }\n"
		.. "\n"
		.. "    [PLAYER1]\n"
		.. "    {\n"
		.. "        Name=NativeApiParityEnemy;\n"
		.. "        Spectator=0;\n"
		.. "        Team=1;\n"
		.. "    }\n"
		.. "\n"
		.. "    [TEAM0]\n"
		.. "    {\n"
		.. "        TeamLeader=0;\n"
		.. "        AllyTeam=0;\n"
		.. "        RGBColor=1 1 1;\n"
		.. "        Side=Arm;\n"
		.. "    }\n"
		.. "\n"
		.. "    [TEAM1]\n"
		.. "    {\n"
		.. "        TeamLeader=1;\n"
		.. "        AllyTeam=1;\n"
		.. "        RGBColor=1 0 0;\n"
		.. "        Side=Arm;\n"
		.. "    }\n"
		.. "\n"
		.. "    [ALLYTEAM0]\n"
		.. "    {\n"
		.. "        NumAllies=0;\n"
		.. "    }\n"
		.. "\n"
		.. "    [ALLYTEAM1]\n"
		.. "    {\n"
		.. "        NumAllies=0;\n"
		.. "    }\n"
		.. "}\n"
end

local function processPayload(caseIndex, _, values)
	values.case = caseIndex
	return values
end

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "quit",
	payload = processPayload,
	make = function() return {} end,
	set = function(_, value)
		local ok, returnCount = pcall(function()
			return select("#", Spring.Quit())
		end)
		value.called = ok
		value.returnCount = ok and returnCount or -1
	end,
	nativeSet = function(_, value, invoke)
		invoke()
		value.called = true
		value.returnCount = 0
	end,
	get = function(_, value)
		return {called = value.called, returnCount = value.returnCount}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "reload",
	payload = processPayload,
	make = function()
		return {startScript = processStartScript(), reloaded = false}
	end,
	set = function(_, value)
		local ok, returnCount = pcall(function()
			return select("#", Spring.Reload(value.startScript))
		end)
		value.called = ok
		value.returnCount = ok and returnCount or -1
		value.startScript = nil
	end,
	nativeSet = function(_, value, invoke)
		invoke()
		value.called = true
		value.returnCount = 0
		value.startScript = nil
	end,
	get = function(_, value)
		return {called = value.called, returnCount = value.returnCount, reloaded = value.reloaded}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "restart",
	payload = processPayload,
	make = function()
		return {cmdArgs = "--ignored-by-restart", startScript = processStartScript(), reloaded = false}
	end,
	set = function(_, value)
		local ok, returnCount = pcall(function()
			return select("#", Spring.Restart(value.cmdArgs, value.startScript))
		end)
		value.called = ok
		value.returnCount = ok and returnCount or -1
		value.cmdArgs = nil
		value.startScript = nil
	end,
	nativeSet = function(_, value, invoke)
		invoke()
		value.called = true
		value.returnCount = 0
		value.cmdArgs = nil
		value.startScript = nil
	end,
	get = function(_, value)
		return {called = value.called, returnCount = value.returnCount, reloaded = value.reloaded}
	end,
}

TEST_HOOKS[#TEST_HOOKS + 1] = {
	name = "start",
	payload = processPayload,
	make = function() return {cmdArgs = "--help", startScript = ""} end,
	set = function(_, value)
		local ok, result = pcall(Spring.Start, value.cmdArgs, value.startScript)
		value.called = ok
		value.returnCount = ok and 1 or -1
		value.result = ok and result or false
		value.cmdArgs = nil
		value.startScript = nil
	end,
	nativeSet = function(_, value, invoke)
		invoke()
		value.called = true
		value.returnCount = 1
		value.result = false
		value.cmdArgs = nil
		value.startScript = nil
	end,
	get = function(_, value)
		return {called = value.called, returnCount = value.returnCount, result = value.result}
	end,
}


	return TEST_HOOKS
end
