/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>

struct UnitDef;

struct lua_State;
class CCobFile;

class CUnit;
class CUnitScript;
class NativeUnitScriptBackend;

class CUnitScriptFactory
{
public:
	static void InitStatic();

	static CUnitScript* CreateScript(CUnit* unit, const UnitDef* udef);

	static CUnitScript* CreateCOBScript(CUnit* unit, CCobFile* F);
	static CUnitScript* CreateLuaScript(CUnit* unit, lua_State* L);
	static CUnitScript* AttachCusScript(CUnit* unit, NativeUnitScriptBackend* backend,
		uint32_t instanceId, uint64_t capabilities);
	static CUnitScript* CreateCusScript(CUnit* unit, NativeUnitScriptBackend* backend,
		uint32_t instanceId, uint64_t capabilities);
};
