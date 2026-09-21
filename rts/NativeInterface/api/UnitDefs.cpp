#include "UnitDefs.h"

#include "Sim/Units/UnitDefHandler.h"
#include "Sim/Units/UnitDef.h"
#include "Sim/Weapons/WeaponDef.h"
#include "Lua/LuaUnitDefs.h"
#include "Lua/LuaDefs.h"
#include <cstring>
#include <string>
#include <vector>

namespace {

// Scratch buffer
// Results that are lists are written here. 1KB was not enough for any of them:
// it truncated GetUnitDefIDs at 256 defs (a real game has more), and the
// property table at 64 of its 249 entries.
static thread_local uint8_t scratchBuffer[64 * 1024];
static thread_local size_t bufferPos = 0;
static thread_local std::vector<int32_t> unitDefIDs;
static thread_local std::vector<UnitDefParamKey> unitDefParamKeys;
static thread_local std::vector<int32_t> unitWeaponIDs;
static thread_local std::vector<int32_t> unitBuildableIDs;
static thread_local std::vector<const char*> unitCustomParamKeys;

// Static errors
static const Error INVALID_UNITDEF_ERROR = { .code = ERROR_INVALID_ID, .message = "Invalid unit def ID" };
static const Error NOT_FOUND_ERROR = { .code = ERROR_NOT_FOUND, .message = "UnitDef not found" };
static const Error INVALID_UNITDEF_PARAM_ERROR = { .code = ERROR_NOT_FOUND, .message = "No such UnitDef property, or it is not of the requested type" };

static void NativeGetUnitDefIDs(const GetUnitDefIDsQuery* query, GetUnitDefIDsResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->ids = nullptr;
	result->count = 0;

	const auto& unitDefs = unitDefHandler->GetUnitDefsVec();
	unitDefIDs.clear();
	unitDefIDs.reserve(unitDefs.size() > 0 ? unitDefs.size() - 1 : 0);

	// Start at 1 since ID 0 is not valid
	for (size_t i = 1; i < unitDefs.size(); i++)
		unitDefIDs.push_back(static_cast<int32_t>(i));

	result->ids = unitDefIDs.empty() ? nullptr : unitDefIDs.data();
	result->count = unitDefIDs.size();
}

static void NativeGetUnitDefCount(const GetUnitDefCountQuery* query, GetUnitDefCountResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->count = static_cast<uint32_t>(unitDefHandler->NumUnitDefs());
}

// UnitDef computes these rather than storing them, which is why Lua exposes them
// as functions (ADD_FUNCTION("isBuilding", ...)) instead of fields. They have no
// offset in the reflection table, so they are read through here.
static void FillClassify(const UnitDef* ud, UnitDefClassify* out) {
	out->isTransport = ud->IsTransportUnit();
	out->isImmobile = ud->IsImmobileUnit();
	out->isBuilding = ud->IsBuildingUnit();
	out->isBuilder = ud->IsBuilderUnit();
	out->isMobileBuilder = ud->IsMobileBuilderUnit();
	out->isStaticBuilder = ud->IsStaticBuilderUnit();
	out->isFactory = ud->IsFactoryUnit();
	out->isExtractor = ud->IsExtractorUnit();
	out->isGroundUnit = ud->IsGroundUnit();
	out->isAirUnit = ud->IsAirUnit();
	out->isStrafingAirUnit = ud->IsStrafingAirUnit();
	out->isHoveringAirUnit = ud->IsHoveringAirUnit();
	out->isFighterAirUnit = ud->IsFighterAirUnit();
	out->isBomberAirUnit = ud->IsBomberAirUnit();
}

// The computed booleans, by the same names Lua uses, so GetUnitDefParamBool can
// serve them alongside the offset-based fields.
static bool ClassifyByName(const UnitDef* ud, const std::string& key, bool* found) {
	*found = true;
	if (key == "isTransport")       return ud->IsTransportUnit();
	if (key == "isImmobile")        return ud->IsImmobileUnit();
	if (key == "isBuilding")        return ud->IsBuildingUnit();
	if (key == "isBuilder")         return ud->IsBuilderUnit();
	if (key == "isMobileBuilder")   return ud->IsMobileBuilderUnit();
	if (key == "isStaticBuilder")   return ud->IsStaticBuilderUnit();
	if (key == "isFactory")         return ud->IsFactoryUnit();
	if (key == "isExtractor")       return ud->IsExtractorUnit();
	if (key == "isGroundUnit")      return ud->IsGroundUnit();
	if (key == "isAirUnit")         return ud->IsAirUnit();
	if (key == "isStrafingAirUnit") return ud->IsStrafingAirUnit();
	if (key == "isHoveringAirUnit") return ud->IsHoveringAirUnit();
	if (key == "isFighterAirUnit")  return ud->IsFighterAirUnit();
	if (key == "isBomberAirUnit")   return ud->IsBomberAirUnit();
	*found = false;
	return false;
}

// Resolve a property name against the engine's reflection table.
static const DataElement* FindParam(const char* key) {
	if (key == nullptr)
		return nullptr;

	const ParamMap& params = LuaUnitDefs::GetParamMap();
	const auto it = params.find(key);

	if (it == params.end())
		return nullptr;

	return &it->second;
}

// The field's address inside this UnitDef. Offsets in the reflection table are
// relative to a UnitDef instance.
static const void* ParamAddress(const UnitDef* ud, const DataElement* elem) {
	return reinterpret_cast<const char*>(ud) + elem->offset;
}

static int32_t ParamTypeOf(const DataElement* elem) {
	switch (elem->type) {
		case INT_TYPE:      return UNIT_DEF_PARAM_INT;
		case BOOL_TYPE:     return UNIT_DEF_PARAM_BOOL;
		case FLOAT_TYPE:    return UNIT_DEF_PARAM_FLOAT;
		case STRING_TYPE:   return UNIT_DEF_PARAM_STRING;
		// Lua serves these by calling an accessor. The boolean classifiers are
		// answered natively (ClassifyByName); the rest are tables.
		case FUNCTION_TYPE: return UNIT_DEF_PARAM_TABLE;
		default:            return UNIT_DEF_PARAM_MISSING;
	}
}

// Is this one of the computed booleans rather than a stored field?
static bool IsClassifyKey(const char* key) {
	if (key == nullptr)
		return false;

	bool found = false;
	const UnitDef* ud = &unitDefHandler->GetUnitDefsVec()[0];
	ClassifyByName(ud, key, &found);
	return found;
}

static void NativeGetUnitDefParamKeys(const GetUnitDefParamKeysQuery* query, GetUnitDefParamKeysResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	const ParamMap& params = LuaUnitDefs::GetParamMap();
	unitDefParamKeys.clear();
	unitDefParamKeys.reserve(params.size());

	for (const auto& pair : params) {
		// Lua's own iteration helpers, not properties.
		if (pair.first == "next" || pair.first == "pairs")
			continue;

		unitDefParamKeys.push_back({
			.name = pair.first.c_str(),
			.type = IsClassifyKey(pair.first.c_str()) ? UNIT_DEF_PARAM_BOOL : ParamTypeOf(&pair.second),
		});
	}

	result->keys = unitDefParamKeys.empty() ? nullptr : unitDefParamKeys.data();
	result->count = unitDefParamKeys.size();
}

static void NativeGetUnitDefParamType(const GetUnitDefParamTypeQuery* query, GetUnitDefParamTypeResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (IsClassifyKey(query->key)) {
		result->type = UNIT_DEF_PARAM_BOOL;
		return;
	}

	const DataElement* elem = FindParam(query->key);
	result->type = (elem == nullptr) ? UNIT_DEF_PARAM_MISSING : ParamTypeOf(elem);
}

static void NativeGetUnitDefParamBool(const GetUnitDefParamBoolQuery* query, GetUnitDefParamBoolResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->value = false;

	const UnitDef* ud = unitDefHandler->GetUnitDefByID(query->unitDefID);
	if (ud == nullptr) {
		result->error = &INVALID_UNITDEF_ERROR;
		return;
	}

	bool found = false;
	const bool value = ClassifyByName(ud, query->key != nullptr ? query->key : "", &found);
	if (found) {
		result->value = value;
		return;
	}

	const DataElement* elem = FindParam(query->key);
	if (elem == nullptr || elem->type != BOOL_TYPE) {
		result->error = &INVALID_UNITDEF_PARAM_ERROR;
		return;
	}
	result->value = *reinterpret_cast<const bool*>(ParamAddress(ud, elem));
}

static void NativeGetUnitDefParamInt(const GetUnitDefParamIntQuery* query, GetUnitDefParamIntResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->value = 0;

	const UnitDef* ud = unitDefHandler->GetUnitDefByID(query->unitDefID);
	if (ud == nullptr) {
		result->error = &INVALID_UNITDEF_ERROR;
		return;
	}

	const DataElement* elem = FindParam(query->key);
	if (elem == nullptr || elem->type != INT_TYPE) {
		result->error = &INVALID_UNITDEF_PARAM_ERROR;
		return;
	}
	result->value = *reinterpret_cast<const int*>(ParamAddress(ud, elem));
}

static void NativeGetUnitDefParamFloat(const GetUnitDefParamFloatQuery* query, GetUnitDefParamFloatResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->value = 0.0f;

	const UnitDef* ud = unitDefHandler->GetUnitDefByID(query->unitDefID);
	if (ud == nullptr) {
		result->error = &INVALID_UNITDEF_ERROR;
		return;
	}

	const DataElement* elem = FindParam(query->key);
	if (elem == nullptr || elem->type != FLOAT_TYPE) {
		result->error = &INVALID_UNITDEF_PARAM_ERROR;
		return;
	}
	result->value = *reinterpret_cast<const float*>(ParamAddress(ud, elem));
}

static void NativeGetUnitDefParamString(const GetUnitDefParamStringQuery* query, GetUnitDefParamStringResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->value = nullptr;

	const UnitDef* ud = unitDefHandler->GetUnitDefByID(query->unitDefID);
	if (ud == nullptr) {
		result->error = &INVALID_UNITDEF_ERROR;
		return;
	}

	const DataElement* elem = FindParam(query->key);
	if (elem == nullptr || elem->type != STRING_TYPE) {
		result->error = &INVALID_UNITDEF_PARAM_ERROR;
		return;
	}
	result->value = reinterpret_cast<const std::string*>(ParamAddress(ud, elem))->c_str();
}

static void NativeGetUnitDefClassify(const GetUnitDefClassifyQuery* query, GetUnitDefClassifyResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	const UnitDef* ud = unitDefHandler->GetUnitDefByID(query->unitDefID);
	if (ud == nullptr) {
		result->error = &INVALID_UNITDEF_ERROR;
		return;
	}
	FillClassify(ud, &result->classify);
}

static void NativeGetUnitDefByID(const GetUnitDefByIDQuery* query, GetUnitDefByIDResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->exists = false;

	const UnitDef* ud = unitDefHandler->GetUnitDefByID(query->unitDefID);
	if (ud == nullptr) {
		result->error = &INVALID_UNITDEF_ERROR;
		return;
	}

	result->exists = true;

	// Basic info
	result->basic.id = ud->id;
	result->basic.unitDefID = ud->id;

	// Copy name to scratch buffer
	const char* name = ud->name.c_str();
	const size_t nameLen = strlen(name);
	if (bufferPos + nameLen + 1 > sizeof(scratchBuffer)) {
		result->error = &INVALID_UNITDEF_ERROR;
		return;
	}
	char* nameBuf = reinterpret_cast<char*>(scratchBuffer + bufferPos);
	memcpy(nameBuf, name, nameLen + 1);
	result->basic.name = nameBuf;
	bufferPos += nameLen + 1;

	// Copy human name to scratch buffer
	const char* humanName = ud->humanName.c_str();
	const size_t humanNameLen = strlen(humanName);
	if (bufferPos + humanNameLen + 1 > sizeof(scratchBuffer)) {
		result->error = &INVALID_UNITDEF_ERROR;
		return;
	}
	char* humanNameBuf = reinterpret_cast<char*>(scratchBuffer + bufferPos);
	memcpy(humanNameBuf, humanName, humanNameLen + 1);
	result->basic.humanName = humanNameBuf;
	bufferPos += humanNameLen + 1;

	// Copy tooltip to scratch buffer
	const char* tooltip = ud->tooltip.c_str();
	const size_t tooltipLen = strlen(tooltip);
	if (bufferPos + tooltipLen + 1 > sizeof(scratchBuffer)) {
		result->error = &INVALID_UNITDEF_ERROR;
		return;
	}
	char* tooltipBuf = reinterpret_cast<char*>(scratchBuffer + bufferPos);
	memcpy(tooltipBuf, tooltip, tooltipLen + 1);
	result->basic.tooltip = tooltipBuf;
	bufferPos += tooltipLen + 1;

	// Costs
	result->costs.metalCost = ud->cost.metal;
	result->costs.energyCost = ud->cost.energy;
	result->costs.buildTime = ud->buildTime;

	// Physics
	result->physics.mass = ud->mass;
	result->physics.height = ud->collisionVolume.GetScale(1);  // Y axis
	result->physics.radius = ud->collisionVolume.GetBoundingRadius();
	result->physics.speed = ud->speed;
	result->physics.turnRate = ud->turnRate;
	result->physics.acceleration = ud->maxAcc;
	result->physics.brakeRate = ud->maxDec;
	result->physics.canFly = ud->canfly;
	result->physics.canMove = ud->canmove;
	result->physics.canHover = ud->hoverAttack;  // Closest equivalent
	result->physics.floatOnWater = ud->floatOnWater;
	result->physics.moveDefID = (ud->pathType != -1U) ? static_cast<int32_t>(ud->pathType) : -1;
	result->physics.canSubmerge = ud->canSubmerge;
	result->physics.waterline = ud->waterline;
	result->physics.minWaterDepth = ud->minWaterDepth;
	result->physics.maxWaterDepth = ud->maxWaterDepth;

	// Classification
	FillClassify(ud, &result->classify);

	// Weapons
	unitWeaponIDs.clear();
	unitWeaponIDs.reserve(ud->weapons.size());
	for (unsigned int i = 0; i < ud->weapons.size(); i++) {
		if (ud->weapons[i].def != nullptr) {
			unitWeaponIDs.push_back(ud->weapons[i].def->id);
		}
	}
	result->weapons.weaponDefIDs = unitWeaponIDs.empty() ? nullptr : unitWeaponIDs.data();
	result->weapons.weaponCount = unitWeaponIDs.size();

	// Build options (now a map<int, string>)
	unitBuildableIDs.clear();
	unitBuildableIDs.reserve(ud->buildOptions.size());
	for (const auto& buildOption : ud->buildOptions)
		unitBuildableIDs.push_back(buildOption.first);  // first is the unit def ID
	result->buildOptions.buildableUnitDefIDs = unitBuildableIDs.empty() ? nullptr : unitBuildableIDs.data();
	result->buildOptions.buildableCount = unitBuildableIDs.size();

	// Sensors
	result->sensors.losRadius = ud->losRadius;
	result->sensors.airLosRadius = ud->airLosRadius;
	result->sensors.radarRadius = static_cast<float>(ud->radarRadius);
	result->sensors.sonarRadius = static_cast<float>(ud->sonarRadius);
	result->sensors.seismicRadius = static_cast<float>(ud->seismicRadius);
	result->sensors.radarJammerRadius = static_cast<float>(ud->jammerRadius);
	result->sensors.sonarJammerRadius = static_cast<float>(ud->sonarJamRadius);

	// Health
	result->health.health = ud->health;
	result->health.autoHeal = ud->autoHeal;
	result->health.idleAutoHeal = ud->idleAutoHeal;
	result->health.idleTime = ud->idleTime;
}

static void NativeGetUnitDefIDByName(const GetUnitDefIDByNameQuery* query, GetUnitDefIDByNameResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->id = -1;

	if (query->unitDefName == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	const UnitDef* ud = unitDefHandler->GetUnitDefByName(query->unitDefName);
	if (ud == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	result->id = ud->id;
}

static void NativeValidUnitDefID(const ValidUnitDefIDQuery* query, ValidUnitDefIDResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->valid = unitDefHandler->IsValidUnitDefID(query->unitDefID);
}

static void NativeGetUnitDefName(const GetUnitDefNameQuery* query, GetUnitDefNameResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->name = "";

	const UnitDef* ud = unitDefHandler->GetUnitDefByID(query->unitDefID);
	if (ud == nullptr) {
		result->error = &INVALID_UNITDEF_ERROR;
		return;
	}

	const char* name = ud->name.c_str();
	const size_t nameLen = strlen(name);
	if (bufferPos + nameLen + 1 > sizeof(scratchBuffer)) {
		result->error = &INVALID_UNITDEF_ERROR;
		return;
	}
	char* nameBuf = reinterpret_cast<char*>(scratchBuffer + bufferPos);
	memcpy(nameBuf, name, nameLen + 1);
	result->name = nameBuf;
	bufferPos += nameLen + 1;
}

static void NativeGetUnitDefHumanName(const GetUnitDefHumanNameQuery* query, GetUnitDefHumanNameResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->humanName = "";

	const UnitDef* ud = unitDefHandler->GetUnitDefByID(query->unitDefID);
	if (ud == nullptr) {
		result->error = &INVALID_UNITDEF_ERROR;
		return;
	}

	const char* humanName = ud->humanName.c_str();
	const size_t humanNameLen = strlen(humanName);
	if (bufferPos + humanNameLen + 1 > sizeof(scratchBuffer)) {
		result->error = &INVALID_UNITDEF_ERROR;
		return;
	}
	char* humanNameBuf = reinterpret_cast<char*>(scratchBuffer + bufferPos);
	memcpy(humanNameBuf, humanName, humanNameLen + 1);
	result->humanName = humanNameBuf;
	bufferPos += humanNameLen + 1;
}

static void NativeGetUnitDefCosts(const GetUnitDefCostsQuery* query, GetUnitDefCostsResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	const UnitDef* ud = unitDefHandler->GetUnitDefByID(query->unitDefID);
	if (ud == nullptr) {
		result->error = &INVALID_UNITDEF_ERROR;
		return;
	}

	result->costs.metalCost = ud->cost.metal;
	result->costs.energyCost = ud->cost.energy;
	result->costs.buildTime = ud->buildTime;
}

static void NativeGetUnitDefSpeed(const GetUnitDefSpeedQuery* query, GetUnitDefSpeedResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->speed = 0.0f;

	const UnitDef* ud = unitDefHandler->GetUnitDefByID(query->unitDefID);
	if (ud == nullptr) {
		result->error = &INVALID_UNITDEF_ERROR;
		return;
	}

	result->speed = ud->speed;
}

static void NativeGetUnitDefHealth(const GetUnitDefHealthQuery* query, GetUnitDefHealthResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->health = 0.0f;

	const UnitDef* ud = unitDefHandler->GetUnitDefByID(query->unitDefID);
	if (ud == nullptr) {
		result->error = &INVALID_UNITDEF_ERROR;
		return;
	}

	result->health = ud->health;
}

static void NativeGetUnitDefCustomParam(const GetUnitDefCustomParamQuery* query, GetUnitDefCustomParamResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->value = "";

	const UnitDef* ud = unitDefHandler->GetUnitDefByID(query->unitDefID);
	if (ud == nullptr) {
		result->error = &INVALID_UNITDEF_ERROR;
		return;
	}

	if (query->key == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	auto it = ud->customParams.find(query->key);
	if (it == ud->customParams.end()) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	const char* value = it->second.c_str();
	const size_t valueLen = strlen(value);
	if (bufferPos + valueLen + 1 > sizeof(scratchBuffer)) {
		result->error = &INVALID_UNITDEF_ERROR;
		return;
	}
	char* valueBuf = reinterpret_cast<char*>(scratchBuffer + bufferPos);
	memcpy(valueBuf, value, valueLen + 1);
	result->value = valueBuf;
	bufferPos += valueLen + 1;
}

static void NativeGetUnitDefCustomParamKeys(const GetUnitDefCustomParamKeysQuery* query, GetUnitDefCustomParamKeysResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->keys = nullptr;
	result->count = 0;

	const UnitDef* ud = unitDefHandler->GetUnitDefByID(query->unitDefID);
	if (ud == nullptr) {
		result->error = &INVALID_UNITDEF_ERROR;
		return;
	}

	unitCustomParamKeys.clear();
	unitCustomParamKeys.reserve(ud->customParams.size());
	for (const auto& pair : ud->customParams)
		unitCustomParamKeys.push_back(pair.first.c_str());

	result->keys = unitCustomParamKeys.empty() ? nullptr : unitCustomParamKeys.data();
	result->count = unitCustomParamKeys.size();
}

} // namespace

const UnitDefsApi UNIT_DEFS_API = {
	.GetUnitDefIDs = NativeGetUnitDefIDs,
	.GetUnitDefCount = NativeGetUnitDefCount,
	.GetUnitDefByID = NativeGetUnitDefByID,
	.GetUnitDefIDByName = NativeGetUnitDefIDByName,
	.ValidUnitDefID = NativeValidUnitDefID,
	.GetUnitDefName = NativeGetUnitDefName,
	.GetUnitDefHumanName = NativeGetUnitDefHumanName,
	.GetUnitDefCosts = NativeGetUnitDefCosts,
	.GetUnitDefSpeed = NativeGetUnitDefSpeed,
	.GetUnitDefHealth = NativeGetUnitDefHealth,
	.GetUnitDefCustomParam = NativeGetUnitDefCustomParam,
	.GetUnitDefCustomParamKeys = NativeGetUnitDefCustomParamKeys,
	.GetUnitDefClassify = NativeGetUnitDefClassify,
	.GetUnitDefParamKeys = NativeGetUnitDefParamKeys,
	.GetUnitDefParamType = NativeGetUnitDefParamType,
	.GetUnitDefParamBool = NativeGetUnitDefParamBool,
	.GetUnitDefParamInt = NativeGetUnitDefParamInt,
	.GetUnitDefParamFloat = NativeGetUnitDefParamFloat,
	.GetUnitDefParamString = NativeGetUnitDefParamString,
};
