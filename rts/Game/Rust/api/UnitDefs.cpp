#include "UnitDefs.h"

#include "Sim/Units/UnitDefHandler.h"
#include <cstring>

namespace {

// Scratch buffer
static thread_local uint8_t scratchBuffer[8192];
static thread_local size_t bufferPos = 0;

// Static errors
static const Error INVALID_UNITDEF_ERROR = { .code = ERROR_INVALID_ID, .message = "Invalid unit def ID" };
static const Error NOT_FOUND_ERROR = { .code = ERROR_NOT_FOUND, .message = "UnitDef not found" };

static void NativeGetUnitDefIDs(const GetUnitDefIDsQuery* query, GetUnitDefIDsResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->ids = nullptr;
	result->count = 0;

	const auto& unitDefs = unitDefHandler->GetUnitDefsVec();
	const size_t maxIDs = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);

	int32_t* ids = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t count = 0;

	// Start at 1 since ID 0 is not valid
	for (size_t i = 1; i < unitDefs.size() && count < maxIDs; i++) {
		ids[count++] = static_cast<int32_t>(i);
	}

	result->ids = ids;
	result->count = count;
	bufferPos += count * sizeof(int32_t);
}

static void NativeGetUnitDefCount(const GetUnitDefCountQuery* query, GetUnitDefCountResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->count = static_cast<uint32_t>(unitDefHandler->NumUnitDefs());
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
	result->physics.height = ud->height;
	result->physics.radius = ud->radius;
	result->physics.speed = ud->speed;
	result->physics.turnRate = ud->turnRate;
	result->physics.acceleration = ud->acceleration;
	result->physics.brakeRate = ud->brakeRate;
	result->physics.canFly = ud->canfly;
	result->physics.canMove = ud->canmove;
	result->physics.canHover = ud->canHover;
	result->physics.floatOnWater = ud->floatOnWater;
	result->physics.moveDefID = (ud->pathType != -1U) ? static_cast<int32_t>(ud->pathType) : -1;

	// Weapons
	const size_t maxWeapons = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);
	int32_t* weaponIDs = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t weaponCount = 0;
	for (unsigned int i = 0; i < ud->weapons.size() && weaponCount < maxWeapons; i++) {
		if (ud->weapons[i].def != nullptr) {
			weaponIDs[weaponCount++] = ud->weapons[i].def->id;
		}
	}
	result->weapons.weaponDefIDs = weaponIDs;
	result->weapons.weaponCount = weaponCount;
	bufferPos += weaponCount * sizeof(int32_t);

	// Build options
	const size_t maxBuildable = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);
	int32_t* buildableIDs = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t buildableCount = 0;
	for (size_t i = 0; i < ud->buildOptions.size() && buildableCount < maxBuildable; i++) {
		buildableIDs[buildableCount++] = ud->buildOptions[i];
	}
	result->buildOptions.buildableUnitDefIDs = buildableIDs;
	result->buildOptions.buildableCount = buildableCount;
	bufferPos += buildableCount * sizeof(int32_t);

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

static void NativeGetUnitDefID(const GetUnitDefIDQuery* query, GetUnitDefIDResult* result) {
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

	// Allocate array of string pointers
	const size_t maxKeys = ud->customParams.size();
	if (bufferPos + maxKeys * sizeof(const char*) > sizeof(scratchBuffer)) {
		result->error = &INVALID_UNITDEF_ERROR;
		return;
	}

	const char** keys = reinterpret_cast<const char**>(scratchBuffer + bufferPos);
	bufferPos += maxKeys * sizeof(const char*);
	uint32_t count = 0;

	for (const auto& pair : ud->customParams) {
		const char* key = pair.first.c_str();
		const size_t keyLen = strlen(key);
		if (bufferPos + keyLen + 1 > sizeof(scratchBuffer)) {
			result->error = &INVALID_UNITDEF_ERROR;
			return;
		}
		char* keyBuf = reinterpret_cast<char*>(scratchBuffer + bufferPos);
		memcpy(keyBuf, key, keyLen + 1);
		keys[count++] = keyBuf;
		bufferPos += keyLen + 1;
	}

	result->keys = keys;
	result->count = count;
}

} // namespace

const UnitDefsApi UNIT_DEFS_API = {
	.GetUnitDefIDs = NativeGetUnitDefIDs,
	.GetUnitDefCount = NativeGetUnitDefCount,
	.GetUnitDefByID = NativeGetUnitDefByID,
	.GetUnitDefID = NativeGetUnitDefID,
	.ValidUnitDefID = NativeValidUnitDefID,
	.GetUnitDefName = NativeGetUnitDefName,
	.GetUnitDefHumanName = NativeGetUnitDefHumanName,
	.GetUnitDefCosts = NativeGetUnitDefCosts,
	.GetUnitDefSpeed = NativeGetUnitDefSpeed,
	.GetUnitDefHealth = NativeGetUnitDefHealth,
	.GetUnitDefCustomParam = NativeGetUnitDefCustomParam,
	.GetUnitDefCustomParamKeys = NativeGetUnitDefCustomParamKeys,
};
