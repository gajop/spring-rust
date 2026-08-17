/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmUiVisibility.h"

#ifdef UNIT_TEST

namespace WasmUiVisibility {

namespace {
	thread_local ContextState currentContext;
}

ScopedContext::ScopedContext(bool enabled)
{
	previous = currentContext;
	currentContext = {};
	// The unit-test target does not link the simulation globals.  Keeping the
	// context inactive there lets adapter tests exercise ordinary callouts
	// without accidentally providing a second, incomplete game-world model.
	(void) enabled;
}

ScopedContext::~ScopedContext()
{
	currentContext = previous;
}

bool Active() { return currentContext.active; }
bool FullRead() { return currentContext.fullRead; }
int ReadPlayer() { return currentContext.readPlayer; }
int ReadTeam() { return currentContext.readTeam; }
int ReadAllyTeam() { return currentContext.readAllyTeam; }
bool IsUnitAlly(const CUnit*) { return !Active(); }
bool IsUnitVisible(const CUnit*) { return !Active(); }
bool IsUnitInLos(const CUnit*) { return !Active(); }
bool IsUnitTyped(const CUnit*) { return !Active(); }
bool UnitPasses(const CUnit*, UnitAccess) { return !Active(); }
const UnitDef* EffectiveUnitDef(const CUnit*) { return nullptr; }
const CUnit* FindUnit(int, UnitAccess) { return nullptr; }
bool IsFeatureVisible(const CFeature*) { return !Active(); }
const CFeature* FindFeature(int) { return nullptr; }
bool IsProjectileVisible(const CProjectile*) { return !Active(); }
const CProjectile* FindProjectile(int) { return nullptr; }
bool IsPositionVisible(const float3&) { return !Active(); }
bool IsTeamVisible(int) { return !Active(); }
bool IsAllyTeamVisible(int) { return !Active(); }
bool IsPlayerVisible(int) { return !Active(); }
bool IsLosPerspectiveAllowed(int) { return !Active(); }
int UnitRulesParamMask(const CUnit*) { return 63; }
int FeatureRulesParamMask(const CFeature*) { return 63; }
bool RulesParamVisible(int paramLos, int allowedMask) { return (paramLos & allowedMask) != 0; }

} // namespace WasmUiVisibility

#else

#include "Game/Game.h"
#include "Game/GlobalUnsynced.h"
#include "Game/Players/Player.h"
#include "Game/Players/PlayerHandler.h"
#include "Sim/Features/Feature.h"
#include "Sim/Features/FeatureHandler.h"
#include "Sim/Misc/GlobalSynced.h"
#include "Sim/Misc/LosHandler.h"
#include "Sim/Misc/TeamHandler.h"
#include "Sim/Projectiles/Projectile.h"
#include "Sim/Projectiles/ProjectileHandler.h"
#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Units/UnitDef.h"
#include "System/EventClient.h"
#include "System/float3.h"

namespace WasmUiVisibility {

namespace {
	thread_local ContextState currentContext;
}

ScopedContext::ScopedContext(bool enabled)
{
	previous = currentContext;
	currentContext = {};
	if (!enabled || gu == nullptr)
		return;

	currentContext.active = true;
	currentContext.fullRead = gu->spectatingFullView;
	currentContext.readPlayer = gu->myPlayerNum;
	currentContext.readTeam = currentContext.fullRead ? CEventClient::AllAccessTeam : gu->myTeam;
	currentContext.readAllyTeam = currentContext.fullRead ? CEventClient::AllAccessTeam : gu->myAllyTeam;
}

ScopedContext::~ScopedContext()
{
	currentContext = previous;
}

bool Active()
{
	return currentContext.active;
}

bool FullRead()
{
	return currentContext.fullRead;
}

int ReadPlayer()
{
	return currentContext.readPlayer;
}

int ReadTeam()
{
	return currentContext.readTeam;
}

int ReadAllyTeam()
{
	return currentContext.readAllyTeam;
}


bool IsUnitAlly(const CUnit* unit)
{
	if (unit == nullptr)
		return false;
	if (!Active())
		return true;
	return FullRead() || (ReadAllyTeam() >= 0 && unit->allyteam == ReadAllyTeam());
}

bool IsUnitVisible(const CUnit* unit)
{
	if (unit == nullptr)
		return false;
	if (!Active())
		return true;
	if (FullRead() || IsUnitAlly(unit))
		return true;
	return ReadAllyTeam() >= 0 &&
		(unit->losStatus[ReadAllyTeam()] & (LOS_INLOS | LOS_INRADAR)) != 0;
}

bool IsUnitInLos(const CUnit* unit)
{
	if (unit == nullptr)
		return false;
	if (!Active())
		return true;
	if (FullRead() || IsUnitAlly(unit))
		return true;
	return ReadAllyTeam() >= 0 && (unit->losStatus[ReadAllyTeam()] & LOS_INLOS) != 0;
}

bool IsUnitTyped(const CUnit* unit)
{
	if (unit == nullptr)
		return false;
	if (!Active())
		return true;
	if (FullRead() || IsUnitAlly(unit))
		return true;
	if (ReadAllyTeam() < 0)
		return false;
	const unsigned short status = unit->losStatus[ReadAllyTeam()];
	return (status & LOS_INLOS) != 0 ||
		(status & (LOS_PREVLOS | LOS_CONTRADAR)) == (LOS_PREVLOS | LOS_CONTRADAR);
}

bool UnitPasses(const CUnit* unit, UnitAccess access)
{
	if (!Active())
		return true;
	switch (access) {
		case UnitAccess::Visible: return IsUnitVisible(unit);
		case UnitAccess::Typed: return IsUnitTyped(unit);
		case UnitAccess::InLos: return IsUnitInLos(unit);
		case UnitAccess::Ally: return IsUnitAlly(unit);
	}
	return false;
}

const UnitDef* EffectiveUnitDef(const CUnit* unit)
{
	if (unit == nullptr || unit->unitDef == nullptr)
		return nullptr;
	if (IsUnitAlly(unit) || unit->unitDef->decoyDef == nullptr)
		return unit->unitDef;
	return unit->unitDef->decoyDef;
}

const CUnit* FindUnit(int unitID, UnitAccess access)
{
	const CUnit* unit = unitHandler.GetUnit(unitID);
	return UnitPasses(unit, access) ? unit : nullptr;
}

bool IsFeatureVisible(const CFeature* feature)
{
	if (feature == nullptr)
		return false;
	if (!Active())
		return true;
	return FullRead() || (ReadAllyTeam() >= 0 && feature->IsInLosForAllyTeam(ReadAllyTeam()));
}

const CFeature* FindFeature(int featureID)
{
	const CFeature* feature = featureHandler.GetFeature(featureID);
	return (!Active() || IsFeatureVisible(feature)) ? feature : nullptr;
}

bool IsProjectileVisible(const CProjectile* projectile)
{
	if (projectile == nullptr)
		return false;
	if (!Active())
		return true;
	if (FullRead())
		return true;
	if (ReadAllyTeam() < 0)
		return false;
	return projectile->GetAllyteamID() == ReadAllyTeam() ||
		(losHandler != nullptr && losHandler->InLos(projectile->pos, ReadAllyTeam()));
}

const CProjectile* FindProjectile(int projectileID)
{
	const CProjectile* projectile = projectileHandler.GetProjectileBySyncedID(projectileID);
	return (!Active() || IsProjectileVisible(projectile)) ? projectile : nullptr;
}

bool IsPositionVisible(const float3& position)
{
	if (!Active() || FullRead())
		return true;
	if (ReadAllyTeam() < 0 || losHandler == nullptr)
		return false;
	return losHandler->InLos(position, ReadAllyTeam()) ||
		losHandler->InAirLos(position, ReadAllyTeam());
}

bool IsTeamVisible(int teamID)
{
	if (!Active())
		return true;
	if (!teamHandler.IsValidTeam(teamID))
		return false;
	return FullRead() || (game != nullptr && game->IsGameOver()) ||
		(ReadAllyTeam() >= 0 && teamHandler.AllyTeam(teamID) == ReadAllyTeam());
}

bool IsAllyTeamVisible(int allyTeamID)
{
	if (!Active())
		return true;
	if (FullRead())
		return teamHandler.IsValidAllyTeam(allyTeamID);
	return allyTeamID == ReadAllyTeam() && teamHandler.IsValidAllyTeam(allyTeamID);
}

bool IsPlayerVisible(int playerID)
{
	if (!Active())
		return true;
	const CPlayer* player = playerHandler.IsValidPlayer(playerID) ? playerHandler.Player(playerID) : nullptr;
	if (player == nullptr)
		return false;
	if (FullRead() || (game != nullptr && game->IsGameOver()))
		return true;
	return player->team >= 0 && IsTeamVisible(player->team);
}

bool IsLosPerspectiveAllowed(int allyTeamID)
{
	if (!Active())
		return true;
	if (FullRead())
		return allyTeamID == CEventClient::AllAccessTeam || teamHandler.IsValidAllyTeam(allyTeamID);
	return allyTeamID == ReadAllyTeam();
}

int UnitRulesParamMask(const CUnit* unit)
{
	if (!Active() || unit == nullptr)
		return 63;
	if (FullRead() || (game != nullptr && game->IsGameOver()) || IsUnitAlly(unit))
		return 63;
	if (ReadTeam() >= 0 && teamHandler.AlliedTeams(unit->team, ReadTeam()))
		return 62;
	if (ReadAllyTeam() < 0)
		return 32;
	const unsigned short status = unit->losStatus[ReadAllyTeam()];
	if ((status & LOS_INLOS) != 0)
		return 60;
	if ((status & (LOS_PREVLOS | LOS_CONTRADAR)) != 0)
		return 56;
	if ((status & LOS_INRADAR) != 0)
		return 48;
	return 32;
}

int FeatureRulesParamMask(const CFeature* feature)
{
	if (!Active() || feature == nullptr)
		return 63;
	if (FullRead() || (game != nullptr && game->IsGameOver()) ||
		(ReadAllyTeam() >= 0 && feature->allyteam == ReadAllyTeam()))
		return 63;
	if (ReadTeam() >= 0 && teamHandler.AlliedTeams(feature->team, ReadTeam()))
		return 62;
	if (IsFeatureVisible(feature))
		return 60;
	return 32;
}

bool RulesParamVisible(int paramLos, int allowedMask)
{
	return (paramLos & allowedMask) != 0;
}

} // namespace WasmUiVisibility

#endif
