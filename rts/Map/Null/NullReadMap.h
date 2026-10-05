/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#ifndef NULL_READ_MAP_H
#define NULL_READ_MAP_H

#include "Map/ReadMap.h"
#include "Map/BaseGroundDrawer.h"

/**
 * A map with no ground: no map file, no terrain stored, drawn or deformed (see CReadMap::IsNullMap).
 * The map info's `nullmap` section gives its size. Memory and load time do not grow with its area,
 * so it suits large, mostly empty worlds such as space.
 */
class CNullReadMap : public CReadMap
{
public:
	CNullReadMap();
	~CNullReadMap() override;

	void ReloadTextures() override {}

	void InitGroundDrawer() override;
	void KillGroundDrawer() override;
	CBaseGroundDrawer* GetGroundDrawer() override { return groundDrawer; }

	uint32_t GetShadingTexture() const override { return 0; }
	uint32_t GetHeightMapTexture() const override { return 0; }
	const MapTexture& GetHeightMapTextureObj() const override { return heightMapTexture; }

	void BindMiniMapTextures() const override {}

	int GetNumFeatures() override { return 0; }
	int GetNumFeatureTypes() override { return 0; }
	void GetFeatureInfo(MapFeatureInfo* f) override {}
	const char* GetFeatureTypeName(int typeID) override { return ""; }

	unsigned char* GetInfoMap(const char* name, MapBitmapInfo* bm) override;
	void FreeInfoMap(const char* name, unsigned char* data) override {}

	int2 GetPatch(int hmx, int hmz) const override;
	const float3& GetUnsyncedHeightInfo(int patchX, int patchZ) const override { return unsyncedHeightInfo[0]; }

protected:
	void UpdateHeightMapUnsynced(const SRectangle&) override {}
	void UpdateHeightMapUnsyncedPost() override {}

private:
	CBaseGroundDrawer* groundDrawer = nullptr;
	MapTexture heightMapTexture;
};


/// Draws nothing: a null map has no ground to show.
class CNullGroundDrawer : public CBaseGroundDrawer
{
public:
	void Draw(const DrawPass::e& drawPass) override {}
	void Update() override {}
	void UpdateRenderState() override {}

	void IncreaseDetail() override {}
	void DecreaseDetail() override {}
	void SetDetail(int newGroundDetail) override {}
	int GetGroundDetail(const DrawPass::e& drawPass = DrawPass::Normal) const override { return 0; }
};

#endif
