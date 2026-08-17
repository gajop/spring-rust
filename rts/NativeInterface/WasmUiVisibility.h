/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>

class CFeature;
class CProjectile;
class CUnit;
class float3;
struct UnitDef;

namespace WasmUiVisibility {

struct ContextState {
	bool active = false;
	bool fullRead = false;
	int readPlayer = -1;
	int readTeam = -1;
	int readAllyTeam = -1;
};

enum class UnitAccess : std::uint8_t {
	Visible,
	Typed,
	InLos,
	Ally,
};

// The context is deliberately thread-local. Wasm imports execute on the
// engine thread, and nested imports must restore the caller's perspective
// rather than inheriting a stale UI read context.
class ScopedContext {
public:
	explicit ScopedContext(bool enabled);
	~ScopedContext();

	ScopedContext(const ScopedContext&) = delete;
	ScopedContext& operator=(const ScopedContext&) = delete;

private:
	ContextState previous;
};

bool Active();
bool FullRead();
int ReadPlayer();
int ReadTeam();
int ReadAllyTeam();

bool UnitPasses(const CUnit* unit, UnitAccess access);
bool IsUnitVisible(const CUnit* unit);
bool IsUnitInLos(const CUnit* unit);
bool IsUnitTyped(const CUnit* unit);
bool IsUnitAlly(const CUnit* unit);
const UnitDef* EffectiveUnitDef(const CUnit* unit);
// Lua's general object parsers use visible (LOS or radar) access. Callers
// whose counterpart requires a stronger condition must request it explicitly.
const CUnit* FindUnit(int unitID, UnitAccess access = UnitAccess::Visible);

bool IsFeatureVisible(const CFeature* feature);
const CFeature* FindFeature(int featureID);

bool IsProjectileVisible(const CProjectile* projectile);
const CProjectile* FindProjectile(int projectileID);
bool IsPositionVisible(const float3& position);

bool IsTeamVisible(int teamID);
bool IsAllyTeamVisible(int allyTeamID);
bool IsPlayerVisible(int playerID);

// LuaUI may only ask for its own LOS perspective unless it has full view.
bool IsLosPerspectiveAllowed(int allyTeamID);

// The current LuaUI perspective's rules-param mask for an object. The
// returned mask is the same mask used by LuaRulesParams::PushRulesParams.
int UnitRulesParamMask(const CUnit* unit);
int FeatureRulesParamMask(const CFeature* feature);
bool RulesParamVisible(int paramLos, int allowedMask);

} // namespace WasmUiVisibility
