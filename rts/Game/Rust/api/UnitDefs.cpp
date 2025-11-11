#include "UnitDefs.h"

#include "Sim/Units/UnitDefHandler.h"

namespace {

static const Error NOT_IMPLEMENTED_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "UnitDefs API not yet fully implemented - stubs only"
};

// UnitDef queries - all stubbed for now
// Would return unit definition data from unitDefHandler

static UInt32Result NativeGetUnitDefCount()
{
	UInt32Result result = {};
	result.value = static_cast<uint32_t>(unitDefHandler->NumUnitDefs());
	return result;
}

// Many more functions stubbed...

} // namespace

const UnitDefsApi UNIT_DEFS_API = {
	.GetUnitDefCount = NativeGetUnitDefCount,
	// ... many more function pointers
};
