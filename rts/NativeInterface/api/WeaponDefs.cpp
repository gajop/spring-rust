#include "WeaponDefs.h"

#include "Sim/Weapons/WeaponDefHandler.h"
#include <cstring>
#include <vector>

namespace {

// Scratch buffer
static thread_local uint8_t scratchBuffer[1024];
static thread_local size_t bufferPos = 0;
static thread_local std::vector<int32_t> weaponDefIDs;
static thread_local std::vector<const char*> customParamKeys;

// Static errors
static const Error INVALID_WEAPONDEF_ERROR = { .code = ERROR_INVALID_ID, .message = "Invalid weapon def ID" };
static const Error NOT_FOUND_ERROR = { .code = ERROR_NOT_FOUND, .message = "WeaponDef not found" };

static void NativeGetWeaponDefIDs(const GetWeaponDefIDsQuery* query, GetWeaponDefIDsResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->ids = nullptr;
	result->count = 0;

	const auto& weaponDefs = weaponDefHandler->GetWeaponDefsVec();
	weaponDefIDs.clear();
	weaponDefIDs.reserve(weaponDefs.size());

	// Weapon ID 0 *is* valid, start at 0
	for (size_t i = 0; i < weaponDefs.size(); i++)
		weaponDefIDs.push_back(static_cast<int32_t>(i));

	result->ids = weaponDefIDs.empty() ? nullptr : weaponDefIDs.data();
	result->count = weaponDefIDs.size();
}

static void NativeGetWeaponDefCount(const GetWeaponDefCountQuery* query, GetWeaponDefCountResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->count = static_cast<uint32_t>(weaponDefHandler->NumWeaponDefs());
}

static void NativeGetWeaponDefByID(const GetWeaponDefByIDQuery* query, GetWeaponDefByIDResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->exists = false;

	const WeaponDef* wd = weaponDefHandler->GetWeaponDefByID(query->weaponDefID);
	if (wd == nullptr) {
		result->error = &INVALID_WEAPONDEF_ERROR;
		return;
	}

	result->exists = true;
	result->info.id = wd->id;

	// Copy name to scratch buffer
	const char* name = wd->name.c_str();
	const size_t nameLen = strlen(name);
	if (bufferPos + nameLen + 1 > sizeof(scratchBuffer)) {
		result->error = &INVALID_WEAPONDEF_ERROR;
		return;
	}
	char* nameBuf = reinterpret_cast<char*>(scratchBuffer + bufferPos);
	memcpy(nameBuf, name, nameLen + 1);
	result->info.name = nameBuf;
	bufferPos += nameLen + 1;

	// Copy type to scratch buffer
	const char* type = wd->type.c_str();
	const size_t typeLen = strlen(type);
	if (bufferPos + typeLen + 1 > sizeof(scratchBuffer)) {
		result->error = &INVALID_WEAPONDEF_ERROR;
		return;
	}
	char* typeBuf = reinterpret_cast<char*>(scratchBuffer + bufferPos);
	memcpy(typeBuf, type, typeLen + 1);
	result->info.type = typeBuf;
	bufferPos += typeLen + 1;

	// Copy description to scratch buffer
	const char* desc = wd->description.c_str();
	const size_t descLen = strlen(desc);
	if (bufferPos + descLen + 1 > sizeof(scratchBuffer)) {
		result->error = &INVALID_WEAPONDEF_ERROR;
		return;
	}
	char* descBuf = reinterpret_cast<char*>(scratchBuffer + bufferPos);
	memcpy(descBuf, desc, descLen + 1);
	result->info.description = descBuf;
	bufferPos += descLen + 1;

	result->info.range = wd->range;
	result->info.reloadTime = wd->reload;
	result->info.damage = wd->damages.GetDefault();
	result->info.areaOfEffect = wd->damages.damageAreaOfEffect;
	result->info.projectileSpeed = wd->projectilespeed;
	result->info.paralyzer = wd->paralyzer;
	result->info.impactOnly = wd->impactOnly;
	result->info.turret = wd->turret;
}

static void NativeGetWeaponDefID(const GetWeaponDefIDQuery* query, GetWeaponDefIDResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->id = -1;

	if (query->weaponDefName == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	const WeaponDef* wd = weaponDefHandler->GetWeaponDef(query->weaponDefName);
	if (wd == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	result->id = wd->id;
}

static void NativeValidWeaponDefID(const ValidWeaponDefIDQuery* query, ValidWeaponDefIDResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->valid = weaponDefHandler->IsValidWeaponDefID(query->weaponDefID);
}

static void NativeGetWeaponDefName(const GetWeaponDefNameQuery* query, GetWeaponDefNameResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->name = "";

	const WeaponDef* wd = weaponDefHandler->GetWeaponDefByID(query->weaponDefID);
	if (wd == nullptr) {
		result->error = &INVALID_WEAPONDEF_ERROR;
		return;
	}

	const char* name = wd->name.c_str();
	const size_t nameLen = strlen(name);
	if (bufferPos + nameLen + 1 > sizeof(scratchBuffer)) {
		result->error = &INVALID_WEAPONDEF_ERROR;
		return;
	}
	char* nameBuf = reinterpret_cast<char*>(scratchBuffer + bufferPos);
	memcpy(nameBuf, name, nameLen + 1);
	result->name = nameBuf;
	bufferPos += nameLen + 1;
}

static void NativeGetWeaponDefRange(const GetWeaponDefRangeQuery* query, GetWeaponDefRangeResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->range = 0.0f;

	const WeaponDef* wd = weaponDefHandler->GetWeaponDefByID(query->weaponDefID);
	if (wd == nullptr) {
		result->error = &INVALID_WEAPONDEF_ERROR;
		return;
	}

	result->range = wd->range;
}

static void NativeGetWeaponDefDamage(const GetWeaponDefDamageQuery* query, GetWeaponDefDamageResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->damage = 0.0f;

	const WeaponDef* wd = weaponDefHandler->GetWeaponDefByID(query->weaponDefID);
	if (wd == nullptr) {
		result->error = &INVALID_WEAPONDEF_ERROR;
		return;
	}

	result->damage = wd->damages.GetDefault();
}

static void NativeGetWeaponDefCustomParam(const GetWeaponDefCustomParamQuery* query, GetWeaponDefCustomParamResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->value = "";

	const WeaponDef* wd = weaponDefHandler->GetWeaponDefByID(query->weaponDefID);
	if (wd == nullptr) {
		result->error = &INVALID_WEAPONDEF_ERROR;
		return;
	}

	if (query->key == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	auto it = wd->customParams.find(query->key);
	if (it == wd->customParams.end()) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	const char* value = it->second.c_str();
	const size_t valueLen = strlen(value);
	if (bufferPos + valueLen + 1 > sizeof(scratchBuffer)) {
		result->error = &INVALID_WEAPONDEF_ERROR;
		return;
	}
	char* valueBuf = reinterpret_cast<char*>(scratchBuffer + bufferPos);
	memcpy(valueBuf, value, valueLen + 1);
	result->value = valueBuf;
	bufferPos += valueLen + 1;
}

static void NativeGetWeaponDefCustomParamKeys(const GetWeaponDefCustomParamKeysQuery* query, GetWeaponDefCustomParamKeysResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->keys = nullptr;
	result->count = 0;

	const WeaponDef* wd = weaponDefHandler->GetWeaponDefByID(query->weaponDefID);
	if (wd == nullptr) {
		result->error = &INVALID_WEAPONDEF_ERROR;
		return;
	}

	customParamKeys.clear();
	customParamKeys.reserve(wd->customParams.size());
	for (const auto& pair : wd->customParams)
		customParamKeys.push_back(pair.first.c_str());

	result->keys = customParamKeys.empty() ? nullptr : customParamKeys.data();
	result->count = customParamKeys.size();
}

} // namespace

const WeaponDefsApi WEAPON_DEFS_API = {
	.GetWeaponDefIDs = NativeGetWeaponDefIDs,
	.GetWeaponDefCount = NativeGetWeaponDefCount,
	.GetWeaponDefByID = NativeGetWeaponDefByID,
	.GetWeaponDefID = NativeGetWeaponDefID,
	.ValidWeaponDefID = NativeValidWeaponDefID,
	.GetWeaponDefName = NativeGetWeaponDefName,
	.GetWeaponDefRange = NativeGetWeaponDefRange,
	.GetWeaponDefDamage = NativeGetWeaponDefDamage,
	.GetWeaponDefCustomParam = NativeGetWeaponDefCustomParam,
	.GetWeaponDefCustomParamKeys = NativeGetWeaponDefCustomParamKeys,
};
