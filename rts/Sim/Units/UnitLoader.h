/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#ifndef UNIT_LOADER_H
#define UNIT_LOADER_H

#include "System/SpringMath.h"
#include "System/float3.h"

#include <string>
#include <vector>

class CCommandAI;
class CUnit;
class CWeapon;

struct UnitDef;
struct UnitDefWeapon;

struct UnitLoadParams {
	const UnitDef* unitDef = nullptr; /// must be non-NULL
	const CUnit* builder = nullptr; /// may be NULL

	float3 pos = ZeroVector;
	float3 speed = ZeroVector;

	int unitID = -1;
	int teamID = -1;
	int facing = FACING_SOUTH;

	bool beingBuilt = false;
	bool flattenGround = false;
};

class CUnitLoader
{
public:
	static CUnitLoader* GetInstance();
	static CCommandAI* NewCommandAI(CUnit* u, const UnitDef* ud);

	CUnit* LoadUnit(const std::string& name, const UnitLoadParams& params);
	CUnit* LoadUnit(const UnitLoadParams& params);

	CWeapon* LoadWeapon(CUnit* owner, const UnitDefWeapon* udw);

	void ParseAndExecuteGiveUnitsCommand(const std::vector<std::string>& args, int team);

	void FlattenGround(const CUnit* unit);
	void RestoreGround(const CUnit* unit);
private:
	void GiveUnits(const std::string& objectName, float3 pos, int amount, int team, int allyTeamFeatures);
};

#define unitLoader (CUnitLoader::GetInstance())

#endif /* UNIT_LOADER_H */
