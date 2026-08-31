/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "UnitScriptFactory.h"

#include "CobFileHandler.h"
#include "CobInstance.h"
#include "LuaUnitScript.h"
#include "NullUnitScript.h"
#include "NativeUnitScript.h"

#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitDef.h"
#include "System/FileSystem/FileSystem.h"
#include "System/Log/ILog.h"
#include "System/StringUtil.h"

#include "System/Misc/TracyDefs.h"

void CUnitScriptFactory::InitStatic()
{
	static_assert(sizeof(CLuaUnitScript) >= sizeof(CCobInstance   ), "");
	static_assert(sizeof(CLuaUnitScript) >= sizeof(CNullUnitScript), "");
	static_assert(sizeof(CLuaUnitScript) >= sizeof(CNativeUnitScript), "");

	CCobUnitScriptNames::InitScriptNames();
	CLuaUnitScriptNames::InitScriptNames();
}


CUnitScript* CUnitScriptFactory::CreateScript(CUnit* unit, const UnitDef* udef)
{
	RECOIL_DETAILED_TRACY_ZONE;
	CUnitScript* script = &CNullUnitScript::value;

	// NOTE:
	//   Lua scripts are not loaded here but deferred to LuaUnitScript::CreateScript
	//   do not check extension in GetCobFile, prevents warning spam for .lua files
	if (FileSystem::GetExtensionLowerCase(udef->scriptName) != "cob")
		return script;

	CCobFile* file = cobFileHandler->GetCobFile(udef->scriptName);

	if (file != nullptr)
		return (CreateCOBScript(unit, file));

	LOG_L(L_WARNING, "[UnitScriptFactory::%s] could not load COB script \"%s\" for unit \"%s\"", __func__, udef->scriptName.c_str(), udef->name.c_str());
	return script;
}


CUnitScript* CUnitScriptFactory::CreateCOBScript(CUnit* unit, CCobFile* F)
{
	RECOIL_DETAILED_TRACY_ZONE;
	static_assert(sizeof(CCobInstance) <= sizeof(unit->usMemBuffer), "");
	return (new (unit->usMemBuffer) CCobInstance(F, unit));
}

CUnitScript* CUnitScriptFactory::CreateLuaScript(CUnit* unit, lua_State* L)
{
	RECOIL_DETAILED_TRACY_ZONE;
	static_assert(sizeof(CLuaUnitScript) <= sizeof(unit->usMemBuffer), "");
	return (new (unit->usMemBuffer) CLuaUnitScript(L, unit));
}

CUnitScript* CUnitScriptFactory::CreateCusScript(CUnit* unit, NativeUnitScriptBackend* backend,
	uint32_t instanceId, uint64_t capabilities)
{
	RECOIL_DETAILED_TRACY_ZONE;
	static_assert(sizeof(CNativeUnitScript) <= sizeof(unit->usMemBuffer), "");
	auto* script = new (unit->usMemBuffer) CNativeUnitScript(unit, backend, instanceId, capabilities);
	if (backend != nullptr)
		backend->Attach(instanceId, script);
	return script;
}

CUnitScript* CUnitScriptFactory::AttachCusScript(CUnit* unit, NativeUnitScriptBackend* backend,
	uint32_t instanceId, uint64_t capabilities)
{
	RECOIL_DETAILED_TRACY_ZONE;
	if (unit == nullptr)
		return nullptr;

	// Attachment is deliberately explicit: the caller registers the instance
	// with its module before invoking Create(), which is the first startup task.
	unit->DeleteScript();
	unit->script = CreateCusScript(unit, backend, instanceId, capabilities);
	return unit->script;
}
