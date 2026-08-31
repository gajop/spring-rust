/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <span>
#include <vector>

#include "UnitScript.h"
#include "NativeInterface/api/Cus.h"

class CUnit;
class CNativeUnitScript;

// Resolve the live script for a generation-safe unit/instance pair. Deferred
// backend work must use this lookup instead of retaining a script pointer.
CNativeUnitScript* FindNativeUnitScript(int32_t unitID, uint32_t instanceID);

extern const CusApi CUS_API;

enum class NativeUnitScriptCall : uint8_t {
	RawCall,
	Create,
	Killed,
	WindChanged,
	ExtractionRateChanged,
	WorldRockUnit,
	RockUnit,
	WorldHitByWeapon,
	HitByWeapon,
	SetSFXOccupy,
	QueryLandingPads,
	BeginTransport,
	QueryTransport,
	TransportPickup,
	TransportDrop,
	StartBuildingWithAim,
	QueryNanoPiece,
	QueryBuildInfo,
	Destroy,
	StartMoving,
	StopMoving,
	StartSkidding,
	StopSkidding,
	ChangeHeading,
	StartUnload,
	EndTransport,
	StartBuilding,
	StopBuilding,
	Falling,
	Landed,
	Activate,
	Deactivate,
	MoveRate,
	FireWeapon,
	EndBurst,
	QueryWeapon,
	AimWeapon,
	AimShieldWeapon,
	AimFromWeapon,
	Shot,
	BlockShot,
	TargetWeight,
	AnimFinished,
};

// These bits mirror NativeUnitScriptCall's discriminants and the portable
// spring::cus::ScriptCapabilities type. A missing entry point must retain the
// engine's neutral fallback instead of crossing the backend.
enum CusCapability : uint64_t {
	CUS_CAP_RAW_CALL = 1ull << 0,
	CUS_CAP_CREATE = 1ull << 1,
	CUS_CAP_KILLED = 1ull << 2,
	CUS_CAP_WIND_CHANGED = 1ull << 3,
	CUS_CAP_EXTRACTION_RATE_CHANGED = 1ull << 4,
	CUS_CAP_WORLD_ROCK_UNIT = 1ull << 5,
	CUS_CAP_ROCK_UNIT = 1ull << 6,
	CUS_CAP_WORLD_HIT_BY_WEAPON = 1ull << 7,
	CUS_CAP_HIT_BY_WEAPON = 1ull << 8,
	CUS_CAP_SET_SFX_OCCUPY = 1ull << 9,
	CUS_CAP_QUERY_LANDING_PADS = 1ull << 10,
	CUS_CAP_BEGIN_TRANSPORT = 1ull << 11,
	CUS_CAP_QUERY_TRANSPORT = 1ull << 12,
	CUS_CAP_TRANSPORT_PICKUP = 1ull << 13,
	CUS_CAP_TRANSPORT_DROP = 1ull << 14,
	CUS_CAP_START_BUILDING_WITH_AIM = 1ull << 15,
	CUS_CAP_QUERY_NANO_PIECE = 1ull << 16,
	CUS_CAP_QUERY_BUILD_INFO = 1ull << 17,
	CUS_CAP_DESTROY = 1ull << 18,
	CUS_CAP_START_MOVING = 1ull << 19,
	CUS_CAP_STOP_MOVING = 1ull << 20,
	CUS_CAP_START_SKIDDING = 1ull << 21,
	CUS_CAP_STOP_SKIDDING = 1ull << 22,
	CUS_CAP_CHANGE_HEADING = 1ull << 23,
	CUS_CAP_START_UNLOAD = 1ull << 24,
	CUS_CAP_END_TRANSPORT = 1ull << 25,
	CUS_CAP_START_BUILDING = 1ull << 26,
	CUS_CAP_STOP_BUILDING = 1ull << 27,
	CUS_CAP_FALLING = 1ull << 28,
	CUS_CAP_LANDED = 1ull << 29,
	CUS_CAP_ACTIVATE = 1ull << 30,
	CUS_CAP_DEACTIVATE = 1ull << 31,
	CUS_CAP_MOVE_RATE = 1ull << 32,
	CUS_CAP_FIRE_WEAPON = 1ull << 33,
	CUS_CAP_END_BURST = 1ull << 34,
	CUS_CAP_QUERY_WEAPON = 1ull << 35,
	CUS_CAP_AIM_WEAPON = 1ull << 36,
	CUS_CAP_AIM_SHIELD_WEAPON = 1ull << 37,
	CUS_CAP_AIM_FROM_WEAPON = 1ull << 38,
	CUS_CAP_SHOT = 1ull << 39,
	CUS_CAP_BLOCK_SHOT = 1ull << 40,
	CUS_CAP_TARGET_WEIGHT = 1ull << 41,
	CUS_CAP_ANIM_FINISHED = 1ull << 42,
};

struct NativeUnitScriptCallResult {
	int32_t intValue = -1;
	float floatValue = 1.0f;
	bool boolValue = false;
	bool complete = false;
	std::vector<int32_t> intValues;
};

// Implemented once by a native or Core-Wasm CUS module.  The adapter below
// owns all CUnitScript compatibility behavior; the backend only translates the
// normalized call payload into the module API and advances its module
// scheduler at the GameFrame boundary.
class NativeUnitScriptBackend
{
public:
	virtual ~NativeUnitScriptBackend() = default;

	// The host keeps this weak association so a deferred task can publish
	// engine-owned completion without retaining the inline script object.
	virtual void Attach(uint32_t instanceId, CNativeUnitScript* script)
	{
		(void) instanceId;
		(void) script;
	}

	// Core-Wasm cannot re-enter its Wasmtime store from the `attach` import.
	// That backend queues Create until the current guest call has returned;
	// native backends start it directly from their owning host path.
	virtual void StartCreate(CNativeUnitScript* script);

	virtual bool Invoke(uint32_t instanceId, NativeUnitScriptCall call,
		std::span<const float> floatArgs, std::span<const int32_t> intArgs,
		NativeUnitScriptCallResult& result) = 0;

	// Implementations must write no more than retValues.size() elements and
	// report the number written through retCount.  The adapter validates the
	// reported count before exposing values to CUnitScript callers.
	virtual bool CallNamed(uint32_t instanceId, const char* functionName,
		std::span<const float> args, std::span<float> retValues,
		uint32_t& retCount, bool& found)
	{
		(void) instanceId;
		(void) functionName;
		(void) args;
		(void) retValues;
		found = false;
		retCount = 0;
		return false;
	}

	virtual void Detach(uint32_t instanceId) { (void) instanceId; }
	virtual void Tick(uint32_t frame) = 0;
};

class CNativeUnitScript final : public CUnitScript
{
	CR_DECLARE_DERIVED(CNativeUnitScript)

public:
	CNativeUnitScript();
	CNativeUnitScript(CUnit* unit, NativeUnitScriptBackend* backend,
		uint32_t instanceId, uint64_t capabilities);
	~CNativeUnitScript() override;
	bool IsCusScript() const override { return true; }
	void PostLoad();

	uint32_t GetInstanceId() const { return instanceId; }
	uint64_t GetCapabilities() const { return capabilities; }
	bool HasCapability(uint64_t capability) const { return (capabilities & capability) != 0; }
	bool UsesBackend(const NativeUnitScriptBackend* candidate) const { return backend == candidate; }
	void DetachBackend(NativeUnitScriptBackend* expected);
	void CancelPendingKilled();
	bool HasBlockShot(int) const override { return HasCapability(CUS_CAP_BLOCK_SHOT); }
	bool HasTargetWeight(int) const override { return HasCapability(CUS_CAP_TARGET_WEIGHT); }

	// Completion hooks are intentionally engine-owned.  A suspended CUS task
	// calls these through its UnitEngine implementation when it settles.
	void FinishAim(int weaponNum, bool ready);
	void FinishShieldAim(int weaponNum, bool enabled);
	void FinishKilled(int wreckLevel);

protected:
	void ShowScriptError(const std::string& msg) override;

private:
	bool Invoke(NativeUnitScriptCall call, std::span<const float> floatArgs = {},
		std::span<const int32_t> intArgs = {}, NativeUnitScriptCallResult* result = nullptr) const;
	bool InvokeWithResult(NativeUnitScriptCall call, std::span<const float> floatArgs,
		std::span<const int32_t> intArgs, NativeUnitScriptCallResult& result) const;
	void WarnMissingBackend() const;

	NativeUnitScriptBackend* backend = nullptr;
	uint32_t instanceId = 0;
	uint64_t capabilities = 0;
	bool killedPending = false;
	mutable bool missingBackendWarningLogged = false;

public:
	void RawCall(int functionId) override;
	void Create() override;
	void Killed() override;
	void WindChanged(float heading, float speed) override;
	void ExtractionRateChanged(float speed) override;
	void WorldRockUnit(const float3& rockDir) override;
	void RockUnit(const float3& rockDir) override;
	void WorldHitByWeapon(const float3& hitDir, int weaponDefId, float& inoutDamage) override;
	void HitByWeapon(const float3& hitDir, int weaponDefId, float& inoutDamage) override;
	void SetSFXOccupy(int curTerrainType) override;
	void QueryLandingPads(std::vector<int>& out_pieces) override;
	void BeginTransport(const CUnit* unit) override;
	int QueryTransport(const CUnit* unit) override;
	void TransportPickup(const CUnit* unit) override;
	void TransportDrop(const CUnit* unit, const float3& pos) override;
	void StartBuilding(float heading, float pitch) override;
	int QueryNanoPiece() override;
	int QueryBuildInfo() override;

	void Destroy() override;
	void StartMoving(bool reversing) override;
	void StopMoving() override;
	void StartSkidding(const float3&) override;
	void StopSkidding() override;
	void ChangeHeading(short deltaHeading) override;
	void StartUnload() override;
	void EndTransport() override;
	void StartBuilding() override;
	void StopBuilding() override;
	void Falling() override;
	void Landed() override;
	void Activate() override;
	void Deactivate() override;
	void MoveRate(int curRate) override;
	void FireWeapon(int weaponNum) override;
	void EndBurst(int weaponNum) override;

	int QueryWeapon(int weaponNum) override;
	void AimWeapon(int weaponNum, float heading, float pitch) override;
	void AimShieldWeapon(CPlasmaRepulser* weapon) override;
	int AimFromWeapon(int weaponNum) override;
	void Shot(int weaponNum) override;
	bool BlockShot(int weaponNum, const CUnit* targetUnit, bool userTarget) override;
	float TargetWeight(int weaponNum, const CUnit* targetUnit) override;
	void AnimFinished(AnimType type, int piece, int axis) override;

	bool CallFunctionByName(const char* functionName, const float* args, uint32_t argCount,
		float* retValues, uint32_t retCapacity, uint32_t& retCount, bool& found) override;
};
