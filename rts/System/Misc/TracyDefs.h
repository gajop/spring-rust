#pragma once

#include <tracy/Tracy.hpp>

#ifdef RECOIL_DETAILED_TRACY_ZONING
	#define RECOIL_DETAILED_TRACY_ZONE ZoneNamed( ___recoil_detailed_tracy_zone, true )
#else
	#define RECOIL_DETAILED_TRACY_ZONE do {} while(0)
#endif