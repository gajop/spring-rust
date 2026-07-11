/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#ifndef LUA_UNITDEFS_H
#define LUA_UNITDEFS_H

#include <string>

#include "LuaDefs.h"

struct lua_State;

class LuaUnitDefs {
public:
	static bool PushEntries(lua_State* L);

	// The reflection table describing every UnitDef property: name -> type plus
	// the byte offset of the field within a UnitDef. Lua indexes UnitDefs
	// through it; the native interface reads the same table so that both expose
	// the same properties and cannot drift apart.
	//
	// Offsets are relative to a UnitDef instance, so a field is read as
	// ((const char*)unitDef) + element.offset.
	static const ParamMap& GetParamMap();
};

#endif /* LUA_UNITDEFS_H */
