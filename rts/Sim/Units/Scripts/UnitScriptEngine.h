/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

/* heavily based on CobEngine.h */

#pragma once

#include <vector>

#include "System/creg/creg_cond.h"

struct UnitDef;
class CUnit;
class CUnitScript;
class NativeUnitScriptBackend;


class CUnitScriptEngine
{
	CR_DECLARE_STRUCT(CUnitScriptEngine)

public:
	void AddInstance(CUnitScript* instance);
	void RemoveInstance(CUnitScript* instance);
	void ReloadScripts(const UnitDef* udef);

	void Tick(int deltaTime);
	void SetCusBackend(NativeUnitScriptBackend* backend);
	void AddCusBackend(NativeUnitScriptBackend* backend);
	void RemoveCusBackend(NativeUnitScriptBackend* backend);
	void CancelCusBackend(NativeUnitScriptBackend* backend);

	void Init() { animating.reserve(256); }
	void Kill();

	const auto& GetAnimating() const { return animating; }

	static void InitStatic();
	static void KillStatic();
private:
	CUnitScript* currentScript = nullptr;
	std::vector<NativeUnitScriptBackend*> cusBackends;

	std::vector<CUnitScript*> animating;
};

extern CUnitScriptEngine* unitScriptEngine;
