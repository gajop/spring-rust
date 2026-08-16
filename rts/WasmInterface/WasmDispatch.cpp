/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmDispatch.h"

bool WasmDispatch::Dispatch(WasmModule& module, const WasmCallinEvent& event, std::string& error)
{
	return module.Callin(event.name, event.arguments, error);
}
