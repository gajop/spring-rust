/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "NullReadMap.h"
#include "Map/MapInfo.h"
#include "Game/LoadScreen.h"
#include "System/SafeUtil.h"

#include "System/Misc/TracyDefs.h"

CNullReadMap::CNullReadMap()
{
	RECOIL_DETAILED_TRACY_ZONE;
	loadscreen->SetLoadMessage("Creating Null Map");

	mapDims.mapx = mapInfo->nullMap.sizeX;
	mapDims.mapy = mapInfo->nullMap.sizeZ;

	CReadMap::InitializeNull();
}

CNullReadMap::~CNullReadMap()
{
	KillGroundDrawer();
}

void CNullReadMap::InitGroundDrawer() { groundDrawer = new CNullGroundDrawer(); }
void CNullReadMap::KillGroundDrawer() { spring::SafeDelete(groundDrawer); }

unsigned char* CNullReadMap::GetInfoMap(const char* name, MapBitmapInfo* bm)
{
	// no metal, grass or type maps: their absence means none, as on maps without them
	*bm = MapBitmapInfo(0, 0);
	return nullptr;
}

int2 CNullReadMap::GetPatch(int hmx, int hmz) const
{
	return {0, 0};
}
