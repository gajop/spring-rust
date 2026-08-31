/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "NativeUnitScript.h"

#include <cassert>

#include "NativeInterface/NativeInterfaceSystem.h"
#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Weapons/PlasmaRepulser.h"
#include "Sim/Weapons/Weapon.h"
#include "System/Log/ILog.h"

CR_BIND_DERIVED(CNativeUnitScript, CUnitScript, )

CR_REG_METADATA(CNativeUnitScript, (
	CR_MEMBER(instanceId),
	CR_MEMBER(capabilities),
	CR_IGNORED(killedPending),
	CR_IGNORED(missingBackendWarningLogged),
	CR_IGNORED(backend),
	CR_POSTLOAD(PostLoad)
))

namespace {

int UnitID(const CUnit* unit)
{
	return (unit != nullptr)? unit->id: -1;
}

constexpr Error kCusInvalidUnit = {
	.code = 1,
	.message = "CUS unit or script is unavailable",
};
constexpr Error kCusInvalidArgument = {
	.code = 2,
	.message = "CUS operation argument is invalid",
};

void AttachCusScript(const CusAttachQuery* query, CusAttachResult* result)
{
	if (result == nullptr)
		return;
	result->error = nullptr;
	result->attached = 0;
	if (query == nullptr || NativeInterfaceSystem::s_instance == nullptr) {
		result->error = &kCusInvalidArgument;
		return;
	}
	result->error = nullptr;
	result->attached = NativeInterfaceSystem::s_instance->AttachCusScript(
		query->unitID, query->instanceID, query->capabilities) ? 1 : 0;
	if (result->attached == 0)
		result->error = &kCusInvalidUnit;
}

void CusOperationCall(const CusOperationQuery* query, CusOperationResult* result)
{
	if (result == nullptr)
		return;
	*result = {.error = nullptr, .value = -1, .completed = 0};
	if (query == nullptr) {
		result->error = &kCusInvalidArgument;
		return;
	}
	CNativeUnitScript* script = FindNativeUnitScript(query->unitID, query->instanceID);
	if (script == nullptr) {
		result->error = &kCusInvalidUnit;
		return;
	}

	result->completed = 1;
	switch (static_cast<CusOperation>(query->operation)) {
		case CUS_OP_TURN:
			script->Turn(query->piece, query->axis, query->second, query->first);
			break;
		case CUS_OP_MOVE:
			script->Move(query->piece, query->axis, query->second, query->first);
			break;
		case CUS_OP_SPIN:
			script->Spin(query->piece, query->axis, query->first, query->second);
			break;
		case CUS_OP_STOP_SPIN:
			script->StopSpin(query->piece, query->axis, query->first);
			break;
		case CUS_OP_SCALE:
			script->Scale(query->piece, query->second, query->first);
			break;
		case CUS_OP_MOVE_NOW:
			script->MoveNow(query->piece, query->axis, query->first);
			break;
		case CUS_OP_TURN_NOW:
			script->TurnNow(query->piece, query->axis, query->first);
			break;
		case CUS_OP_SCALE_NOW:
			script->ScaleNow(query->piece, query->first);
			break;
		case CUS_OP_SHOW:
			script->SetVisibility(query->piece, true);
			break;
		case CUS_OP_HIDE:
			script->SetVisibility(query->piece, false);
			break;
		case CUS_OP_EXPLODE:
			script->Explode(query->piece, query->value);
			break;
		case CUS_OP_EMIT_SFX:
			result->completed = script->EmitSfx(query->value, query->piece) ? 1 : 0;
			break;
		case CUS_OP_ATTACH_UNIT:
			script->AttachUnit(query->piece, query->target);
			break;
		case CUS_OP_DROP_UNIT:
			script->DropUnit(query->target);
			break;
		case CUS_OP_SET_UNIT_VALUE:
			script->SetUnitVal(query->value, query->target);
			break;
		case CUS_OP_AIM_SCRIPT_FINISHED:
			script->FinishAim(query->target, query->value != 0);
			break;
		case CUS_OP_AIM_SHIELD_FINISHED:
			script->FinishShieldAim(query->target, query->value != 0);
			break;
		case CUS_OP_KILLED_SCRIPT_FINISHED:
			script->FinishKilled(query->value);
			break;
		default:
			result->error = &kCusInvalidArgument;
			result->completed = 0;
			break;
	}
}

void CusAnimationActive(const CusAnimationQuery* query, CusAnimationResult* result)
{
	if (result == nullptr)
		return;
	*result = {.error = nullptr, .active = 0};
	if (query == nullptr) {
		result->error = &kCusInvalidArgument;
		return;
	}
	CNativeUnitScript* script = FindNativeUnitScript(query->unitID, query->instanceID);
	if (script == nullptr) {
		result->error = &kCusInvalidUnit;
		return;
	}
	CUnitScript::AnimType type;
	switch (query->animation) {
		case 0: type = CUnitScript::ATurn; break;
		case 1: type = CUnitScript::AMove; break;
		case 2: type = CUnitScript::ASpin; break;
		case 3: type = CUnitScript::AScale; break;
		default:
			result->error = &kCusInvalidArgument;
			return;
	}
	result->active = script->IsInAnimation(type, query->piece, query->axis) ? 1 : 0;
}

}

CNativeUnitScript* FindNativeUnitScript(const int32_t unitID, const uint32_t instanceID)
{
	if (unitID < 0)
		return nullptr;
	CUnit* unit = unitHandler.GetUnit(static_cast<unsigned int>(unitID));
	if (unit == nullptr || unit->script == nullptr)
		return nullptr;
	if (!unit->script->IsCusScript())
		return nullptr;
	auto* script = static_cast<CNativeUnitScript*>(unit->script);
	return script != nullptr && script->GetInstanceId() == instanceID ? script : nullptr;
}

void NativeUnitScriptBackend::StartCreate(CNativeUnitScript* script)
{
	if (script != nullptr)
		script->Create();
}

const CusApi CUS_API = {
	.Attach = &AttachCusScript,
	.Operation = &CusOperationCall,
	.AnimationActive = &CusAnimationActive,
};

CNativeUnitScript::CNativeUnitScript()
	: CUnitScript(nullptr)
{
}

CNativeUnitScript::CNativeUnitScript(CUnit* unit, NativeUnitScriptBackend* backend,
	uint32_t instanceId, uint64_t capabilities)
	: CUnitScript(unit)
	, backend(backend)
	, instanceId(instanceId)
	, capabilities(capabilities)
{
	hasSetSFXOccupy = HasCapability(CUS_CAP_SET_SFX_OCCUPY);
	hasRockUnit = HasCapability(CUS_CAP_ROCK_UNIT);
	hasStartBuilding = HasCapability(CUS_CAP_START_BUILDING_WITH_AIM);

	if (unit == nullptr)
		return;

	pieces.reserve(unit->localModel.pieces.size());
	for (auto& piece: unit->localModel.pieces) {
		pieces.push_back(&piece);
		if (!piece.parent)
			rootPiece = &piece;
	}

	assert(rootPiece != nullptr);
}

CNativeUnitScript::~CNativeUnitScript()
{
	if (backend != nullptr) {
		auto* detachedBackend = backend;
		backend = nullptr;
		detachedBackend->Detach(instanceId);
	}
	CancelPendingKilled();
}

void CNativeUnitScript::PostLoad()
{
	// V1 does not persist suspended CUS futures.  Keep the restored adapter
	// inert and make that loss of the live module state visible immediately.
	killedPending = false;
	missingBackendWarningLogged = false;
	if (unit == nullptr)
		return;

	pieces.clear();
	pieces.reserve(unit->localModel.pieces.size());
	rootPiece = nullptr;
	for (auto& piece: unit->localModel.pieces) {
		pieces.push_back(&piece);
		if (!piece.parent)
			rootPiece = &piece;
	}
	WarnMissingBackend();

	if (rootPiece == nullptr)
		return;

	hasSetSFXOccupy = HasCapability(CUS_CAP_SET_SFX_OCCUPY);
	hasRockUnit = HasCapability(CUS_CAP_ROCK_UNIT);
	hasStartBuilding = HasCapability(CUS_CAP_START_BUILDING_WITH_AIM);
}

void CNativeUnitScript::DetachBackend(NativeUnitScriptBackend* expected)
{
	if (backend != expected)
		return;
	auto* detachedBackend = backend;
	backend = nullptr;
	if (detachedBackend != nullptr)
		detachedBackend->Detach(instanceId);
	CancelPendingKilled();
}

void CNativeUnitScript::CancelPendingKilled()
{
	if (!killedPending)
		return;
	FinishKilled(unit != nullptr ? unit->delayedWreckLevel : -1);
}

void CNativeUnitScript::ShowScriptError(const std::string& msg)
{
	LOG_L(L_ERROR, "[NativeUnitScript] unit %i: %s", UnitID(unit), msg.c_str());
}

void CNativeUnitScript::WarnMissingBackend() const
{
	if (backend != nullptr || missingBackendWarningLogged)
		return;
	missingBackendWarningLogged = true;
	LOG_L(L_WARNING, "[NativeUnitScript] unit %i instance %u has no backend; "
		"CUS state is unavailable after module unload or save/load", UnitID(unit), instanceId);
}

bool CNativeUnitScript::Invoke(NativeUnitScriptCall call, std::span<const float> floatArgs,
	std::span<const int32_t> intArgs, NativeUnitScriptCallResult* result) const
{
	NativeUnitScriptCallResult localResult;
	if (result == nullptr)
		result = &localResult;

	const uint64_t capability = 1ull << static_cast<uint8_t>(call);
	if (backend == nullptr) {
		WarnMissingBackend();
		return false;
	}
	if (!HasCapability(capability))
		return false;

	return backend->Invoke(instanceId, call, floatArgs, intArgs, *result);
}

bool CNativeUnitScript::InvokeWithResult(NativeUnitScriptCall call, std::span<const float> floatArgs,
	std::span<const int32_t> intArgs, NativeUnitScriptCallResult& result) const
{
	return Invoke(call, floatArgs, intArgs, &result);
}

void CNativeUnitScript::FinishAim(int weaponNum, bool ready)
{
	if (unit == nullptr || weaponNum < 0 || static_cast<size_t>(weaponNum) >= unit->weapons.size())
		return;

	if (CWeapon* weapon = unit->weapons[weaponNum]; weapon != nullptr)
		weapon->AimScriptFinished(ready);
}

void CNativeUnitScript::FinishShieldAim(int weaponNum, bool enabled)
{
	if (unit == nullptr || weaponNum < 0 || static_cast<size_t>(weaponNum) >= unit->weapons.size())
		return;

	if (auto* shield = dynamic_cast<CPlasmaRepulser*>(unit->weapons[weaponNum]); shield != nullptr)
		shield->SetEnabled(enabled);
}

void CNativeUnitScript::FinishKilled(int wreckLevel)
{
	killedPending = false;
	if (unit != nullptr)
		unit->KilledScriptFinished(wreckLevel);
}

void CNativeUnitScript::RawCall(int functionId)
{
	const int32_t args[] = {functionId};
	Invoke(NativeUnitScriptCall::RawCall, {}, args);
}

void CNativeUnitScript::Create()
{
	Invoke(NativeUnitScriptCall::Create);
}

void CNativeUnitScript::Killed()
{
	NativeUnitScriptCallResult result;
	const float args[] = {unit != nullptr? unit->recentDamage: 0.0f, unit != nullptr? unit->maxHealth: 0.0f};
	const bool invoked = HasCapability(CUS_CAP_KILLED) && InvokeWithResult(NativeUnitScriptCall::Killed, args, {}, result);

	// Killed is the one callin which must always settle the engine-side death
	// state.  A pending CUS task explicitly settles it through FinishKilled.
	if (!invoked)
		FinishKilled(unit != nullptr? unit->delayedWreckLevel: -1);
	else if (result.complete)
		FinishKilled(result.intValue);
	else
		killedPending = true;
}

void CNativeUnitScript::WindChanged(float heading, float speed)
{
	const float args[] = {heading, speed};
	Invoke(NativeUnitScriptCall::WindChanged, args);
}

void CNativeUnitScript::ExtractionRateChanged(float speed)
{
	const float args[] = {speed};
	Invoke(NativeUnitScriptCall::ExtractionRateChanged, args);
}

void CNativeUnitScript::WorldRockUnit(const float3& rockDir)
{
	const float args[] = {rockDir.x, rockDir.y, rockDir.z};
	Invoke(NativeUnitScriptCall::WorldRockUnit, args);
}

void CNativeUnitScript::RockUnit(const float3& rockDir)
{
	const float args[] = {rockDir.x, rockDir.y, rockDir.z};
	Invoke(NativeUnitScriptCall::RockUnit, args);
}

void CNativeUnitScript::WorldHitByWeapon(const float3& hitDir, int weaponDefId, float& inoutDamage)
{
	const float args[] = {hitDir.x, hitDir.y, hitDir.z, inoutDamage};
	const int32_t ids[] = {weaponDefId};
	NativeUnitScriptCallResult result;
	result.floatValue = inoutDamage;
	if (InvokeWithResult(NativeUnitScriptCall::WorldHitByWeapon, args, ids, result))
		inoutDamage = result.floatValue;
}

void CNativeUnitScript::HitByWeapon(const float3& hitDir, int weaponDefId, float& inoutDamage)
{
	const float args[] = {hitDir.x, hitDir.y, hitDir.z, inoutDamage};
	const int32_t ids[] = {weaponDefId};
	NativeUnitScriptCallResult result;
	result.floatValue = inoutDamage;
	if (InvokeWithResult(NativeUnitScriptCall::HitByWeapon, args, ids, result))
		inoutDamage = result.floatValue;
}

void CNativeUnitScript::SetSFXOccupy(int curTerrainType)
{
	const int32_t args[] = {curTerrainType};
	Invoke(NativeUnitScriptCall::SetSFXOccupy, {}, args);
}

void CNativeUnitScript::QueryLandingPads(std::vector<int>& out_pieces)
{
	NativeUnitScriptCallResult result;
	if (!InvokeWithResult(NativeUnitScriptCall::QueryLandingPads, {}, {}, result))
		return;

	out_pieces.insert(out_pieces.end(), result.intValues.begin(), result.intValues.end());
}

void CNativeUnitScript::BeginTransport(const CUnit* transportee)
{
	const int32_t args[] = {UnitID(transportee)};
	Invoke(NativeUnitScriptCall::BeginTransport, {}, args);
}

int CNativeUnitScript::QueryTransport(const CUnit* transportee)
{
	const int32_t args[] = {UnitID(transportee)};
	NativeUnitScriptCallResult result;
	return InvokeWithResult(NativeUnitScriptCall::QueryTransport, {}, args, result)? result.intValue: -1;
}

void CNativeUnitScript::TransportPickup(const CUnit* transportee)
{
	const int32_t args[] = {UnitID(transportee)};
	Invoke(NativeUnitScriptCall::TransportPickup, {}, args);
}

void CNativeUnitScript::TransportDrop(const CUnit* transportee, const float3& pos)
{
	const float args[] = {pos.x, pos.y, pos.z};
	const int32_t ids[] = {UnitID(transportee)};
	Invoke(NativeUnitScriptCall::TransportDrop, args, ids);
}

void CNativeUnitScript::StartBuilding(float heading, float pitch)
{
	const float args[] = {heading, pitch};
	Invoke(NativeUnitScriptCall::StartBuildingWithAim, args);
}

int CNativeUnitScript::QueryNanoPiece()
{
	NativeUnitScriptCallResult result;
	return InvokeWithResult(NativeUnitScriptCall::QueryNanoPiece, {}, {}, result)? result.intValue: -1;
}

int CNativeUnitScript::QueryBuildInfo()
{
	NativeUnitScriptCallResult result;
	return InvokeWithResult(NativeUnitScriptCall::QueryBuildInfo, {}, {}, result)? result.intValue: -1;
}

void CNativeUnitScript::Destroy()
{
	Invoke(NativeUnitScriptCall::Destroy);
}

void CNativeUnitScript::StartMoving(bool reversing)
{
	const int32_t args[] = {reversing? 1: 0};
	Invoke(NativeUnitScriptCall::StartMoving, {}, args);
}

void CNativeUnitScript::StopMoving()
{
	Invoke(NativeUnitScriptCall::StopMoving);
}

void CNativeUnitScript::StartSkidding(const float3& velocity)
{
	const float args[] = {velocity.x, velocity.y, velocity.z};
	Invoke(NativeUnitScriptCall::StartSkidding, args);
}

void CNativeUnitScript::StopSkidding()
{
	Invoke(NativeUnitScriptCall::StopSkidding);
}

void CNativeUnitScript::ChangeHeading(short deltaHeading)
{
	const int32_t args[] = {deltaHeading};
	Invoke(NativeUnitScriptCall::ChangeHeading, {}, args);
}

void CNativeUnitScript::StartUnload()
{
	Invoke(NativeUnitScriptCall::StartUnload);
}

void CNativeUnitScript::EndTransport()
{
	Invoke(NativeUnitScriptCall::EndTransport);
}

void CNativeUnitScript::StartBuilding()
{
	Invoke(NativeUnitScriptCall::StartBuilding);
}

void CNativeUnitScript::StopBuilding()
{
	Invoke(NativeUnitScriptCall::StopBuilding);
}

void CNativeUnitScript::Falling()
{
	Invoke(NativeUnitScriptCall::Falling);
}

void CNativeUnitScript::Landed()
{
	Invoke(NativeUnitScriptCall::Landed);
}

void CNativeUnitScript::Activate()
{
	Invoke(NativeUnitScriptCall::Activate);
}

void CNativeUnitScript::Deactivate()
{
	Invoke(NativeUnitScriptCall::Deactivate);
}

void CNativeUnitScript::MoveRate(int curRate)
{
	const int32_t args[] = {curRate};
	Invoke(NativeUnitScriptCall::MoveRate, {}, args);
}

void CNativeUnitScript::FireWeapon(int weaponNum)
{
	const int32_t args[] = {weaponNum};
	Invoke(NativeUnitScriptCall::FireWeapon, {}, args);
}

void CNativeUnitScript::EndBurst(int weaponNum)
{
	const int32_t args[] = {weaponNum};
	Invoke(NativeUnitScriptCall::EndBurst, {}, args);
}

int CNativeUnitScript::QueryWeapon(int weaponNum)
{
	if (!HasCapability(CUS_CAP_QUERY_WEAPON))
		return -1;

	const int32_t args[] = {weaponNum};
	NativeUnitScriptCallResult result;
	return InvokeWithResult(NativeUnitScriptCall::QueryWeapon, {}, args, result)? result.intValue: -1;
}

void CNativeUnitScript::AimWeapon(int weaponNum, float heading, float pitch)
{
	if (!HasCapability(CUS_CAP_AIM_WEAPON))
		return;

	const float args[] = {heading, pitch};
	const int32_t ids[] = {weaponNum};
	NativeUnitScriptCallResult result;
	if (!InvokeWithResult(NativeUnitScriptCall::AimWeapon, args, ids, result))
		return;

	if (result.complete)
		FinishAim(weaponNum, result.boolValue);
}

void CNativeUnitScript::AimShieldWeapon(CPlasmaRepulser* weapon)
{
	if (!HasCapability(CUS_CAP_AIM_SHIELD_WEAPON) || weapon == nullptr)
		return;

	const int32_t args[] = {weapon->weaponNum};
	NativeUnitScriptCallResult result;
	if (!InvokeWithResult(NativeUnitScriptCall::AimShieldWeapon, {}, args, result))
		return;

	if (result.complete)
		FinishShieldAim(weapon->weaponNum, result.boolValue);
}

int CNativeUnitScript::AimFromWeapon(int weaponNum)
{
	if (!HasCapability(CUS_CAP_AIM_FROM_WEAPON))
		return -1;

	const int32_t args[] = {weaponNum};
	NativeUnitScriptCallResult result;
	return InvokeWithResult(NativeUnitScriptCall::AimFromWeapon, {}, args, result)? result.intValue: -1;
}

void CNativeUnitScript::Shot(int weaponNum)
{
	const int32_t args[] = {weaponNum};
	Invoke(NativeUnitScriptCall::Shot, {}, args);
}

bool CNativeUnitScript::BlockShot(int weaponNum, const CUnit* targetUnit, bool userTarget)
{
	if (!HasCapability(CUS_CAP_BLOCK_SHOT))
		return false;

	const int32_t args[] = {weaponNum, UnitID(targetUnit), userTarget? 1: 0};
	NativeUnitScriptCallResult result;
	return InvokeWithResult(NativeUnitScriptCall::BlockShot, {}, args, result)? result.boolValue: false;
}

float CNativeUnitScript::TargetWeight(int weaponNum, const CUnit* targetUnit)
{
	if (!HasCapability(CUS_CAP_TARGET_WEIGHT))
		return 1.0f;

	const int32_t args[] = {weaponNum, UnitID(targetUnit)};
	NativeUnitScriptCallResult result;
	return InvokeWithResult(NativeUnitScriptCall::TargetWeight, {}, args, result)? result.floatValue: 1.0f;
}

void CNativeUnitScript::AnimFinished(AnimType type, int piece, int axis)
{
	const int32_t args[] = {static_cast<int32_t>(type), piece, axis};
	// AnimFinished is also the scheduler wake signal. It must reach the CUS
	// backend even when the script did not opt into an application callback;
	// the portable dispatcher can wake waiters while the typed callback remains
	// the default no-op.
	if (backend != nullptr) {
		NativeUnitScriptCallResult result;
		backend->Invoke(instanceId, NativeUnitScriptCall::AnimFinished, {}, args, result);
	}
}

bool CNativeUnitScript::CallFunctionByName(const char* functionName, const float* args, uint32_t argCount,
	float* retValues, uint32_t retCapacity, uint32_t& retCount, bool& found)
{
	found = false;
	retCount = 0;
	if (backend == nullptr) {
		WarnMissingBackend();
		return false;
	}
	if (functionName == nullptr || (argCount != 0 && args == nullptr) ||
		(retCapacity != 0 && retValues == nullptr))
		return false;

	const bool success = backend->CallNamed(
		instanceId,
		functionName,
		{args, argCount},
		{retValues, retCapacity},
		retCount,
		found
	);
	if (!success)
		return false;

	if (retCount > retCapacity) {
		ShowScriptError("named call returned more values than its capacity");
		found = false;
		retCount = 0;
		return false;
	}

	return true;
}
