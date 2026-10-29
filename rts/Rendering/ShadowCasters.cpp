/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "ShadowCasters.h"

#include <algorithm>
#include <array>
#include <cmath>
#include <cstdlib>
#include <vector>

#include "Game/Camera.h"
#include "Game/CameraHandler.h"
#include "Rendering/Env/ISky.h"
#include "Rendering/LuaObjectDrawer.h"
#include "Rendering/ShadowHandler.h"
#include "Sim/Features/Feature.h"
#include "Sim/Features/FeatureHandler.h"
#include "Sim/Objects/SolidObject.h"
#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "System/Config/ConfigHandler.h"
#include "System/Log/ILog.h"

CONFIG(bool, ShadowCasterLOD).defaultValue(false).description("Shadow-pass models take the level of detail that matches the length one shadow-map texel covers (times LODScaleShadow), or one screen pixel at the model (times LODScale) where coarser. Off: the shadow camera's projected distance.");
CONFIG(float, ShadowCasterMinTexels).defaultValue(0.0f).minimumValue(0.0f).description("Models whose diameter spans fewer shadow-map texels than this cast no shadow.");
CONFIG(bool, ShadowCasterReceivers).defaultValue(false).description("A model casts only if it is in the player's view or its shadow reaches a unit or feature in the player's view. For games whose models are the only shadow receivers: the map loses the shadows of models left out.");

namespace {

struct Receiver {
	float x, y, z, r;
};

struct State {
	bool levels = false;
	bool receivers = false;
	float minTexels = 0.0f;

	float texel = 0.0f;

	// light space: x, y across the light, z toward it
	float3 lx, ly, lz;

	// receivers bucketed by light-space x, y
	std::vector<Receiver> recv;
	std::vector<int> cellStart;
	std::vector<int> cellItems;
	float gx0 = 0.0f, gy0 = 0.0f, cell = 1.0f;
	int gw = 0, gh = 0;
	float maxR = 0.0f;
} s;

struct Stats {
	bool on = false;
	unsigned passes = 0;
	unsigned long considered = 0, small = 0, unreceived = 0;
	std::array<unsigned long, 8> levels{};
	double texel = 0.0;
} stats;

bool StatsWanted() {
	static const bool on = [] { const char* v = std::getenv("SPRING_SHADOW_CASTER_STATS"); return v != nullptr && v[0] == '1'; }();
	return on;
}

void LightBasis() {
	const float4& d = ISky::GetSky()->GetLight()->GetLightDir();
	s.lz = float3(d.x, d.y, d.z).SafeANormalize();
	const float3 ref = (std::fabs(s.lz.x) < 0.9f) ? RgtVector : FwdVector;
	s.lx = (ref - s.lz * ref.dot(s.lz)).SafeANormalize();
	s.ly = s.lx.cross(s.lz).SafeANormalize();
}

void AddReceiver(const CSolidObject* o) {
	if (!o->HasDrawFlag(DrawFlags::SO_OPAQUE_FLAG) && !o->HasDrawFlag(DrawFlags::SO_ALPHAF_FLAG))
		return;
	const float3& p = o->drawMidPos;
	const float r = o->GetDrawRadius();
	s.recv.push_back({p.dot(s.lx), p.dot(s.ly), p.dot(s.lz), r});
	s.maxR = std::max(s.maxR, r);
}

// A grid over the receivers' light-space extent, about one receiver a cell; a receiver is
// listed in every cell its disc overlaps.
void BuildGrid() {
	s.gw = s.gh = 0;
	if (s.recv.empty())
		return;

	float x0 = s.recv[0].x, x1 = x0, y0 = s.recv[0].y, y1 = y0;
	for (const Receiver& r: s.recv) {
		x0 = std::min(x0, r.x - r.r); x1 = std::max(x1, r.x + r.r);
		y0 = std::min(y0, r.y - r.r); y1 = std::max(y1, r.y + r.r);
	}
	const float area = std::max((x1 - x0) * (y1 - y0), 1.0f);
	s.cell = std::max(std::sqrt(area / s.recv.size()), 2.0f * s.maxR);
	s.gx0 = x0; s.gy0 = y0;
	s.gw = std::clamp(int((x1 - x0) / s.cell) + 1, 1, 256);
	s.gh = std::clamp(int((y1 - y0) / s.cell) + 1, 1, 256);
	s.cell = std::max(s.cell, std::max((x1 - x0) / s.gw, (y1 - y0) / s.gh) * 1.0001f);

	const auto cells = [](float a0, float a1, float o, int n) {
		return std::pair<int, int>(std::clamp(int((a0 - o) / s.cell), 0, n - 1), std::clamp(int((a1 - o) / s.cell), 0, n - 1));
	};

	s.cellStart.assign(s.gw * s.gh + 1, 0);
	for (const Receiver& r: s.recv) {
		const auto [cx0, cx1] = cells(r.x - r.r, r.x + r.r, s.gx0, s.gw);
		const auto [cy0, cy1] = cells(r.y - r.r, r.y + r.r, s.gy0, s.gh);
		for (int cy = cy0; cy <= cy1; ++cy)
			for (int cx = cx0; cx <= cx1; ++cx)
				s.cellStart[cy * s.gw + cx + 1]++;
	}
	for (int i = 0; i < s.gw * s.gh; ++i)
		s.cellStart[i + 1] += s.cellStart[i];

	s.cellItems.assign(s.cellStart.back(), 0);
	std::vector<int> fill(s.cellStart.begin(), s.cellStart.end() - 1);
	for (int i = 0; i < int(s.recv.size()); ++i) {
		const Receiver& r = s.recv[i];
		const auto [cx0, cx1] = cells(r.x - r.r, r.x + r.r, s.gx0, s.gw);
		const auto [cy0, cy1] = cells(r.y - r.r, r.y + r.r, s.gy0, s.gh);
		for (int cy = cy0; cy <= cy1; ++cy)
			for (int cx = cx0; cx <= cx1; ++cx)
				s.cellItems[fill[cy * s.gw + cx]++] = i;
	}
}

// Whether a caster at light-space (x, y, z) with radius r shadows some receiver: their discs
// across the light overlap, and the caster is not wholly further from the light.
bool Shadows(float x, float y, float z, float r) {
	if (s.gw == 0)
		return false;
	const float fx0 = (x - r - s.gx0) / s.cell, fx1 = (x + r - s.gx0) / s.cell;
	const float fy0 = (y - r - s.gy0) / s.cell, fy1 = (y + r - s.gy0) / s.cell;
	if (fx1 < 0.0f || fy1 < 0.0f || fx0 >= s.gw || fy0 >= s.gh)
		return false;
	const int cx0 = std::max(int(fx0), 0), cx1 = std::min(int(fx1), s.gw - 1);
	const int cy0 = std::max(int(fy0), 0), cy1 = std::min(int(fy1), s.gh - 1);
	for (int cy = cy0; cy <= cy1; ++cy) {
		for (int cx = cx0; cx <= cx1; ++cx) {
			const int c = cy * s.gw + cx;
			for (int k = s.cellStart[c]; k < s.cellStart[c + 1]; ++k) {
				const Receiver& q = s.recv[s.cellItems[k]];
				const float rr = r + q.r;
				if (z < q.z - rr)
					continue;
				const float dx = x - q.x, dy = y - q.y;
				if (dx * dx + dy * dy < rr * rr)
					return true;
			}
		}
	}
	return false;
}

} // namespace


void ShadowCasters::Begin()
{
	s.levels = configHandler->GetBool("ShadowCasterLOD");
	s.receivers = configHandler->GetBool("ShadowCasterReceivers");
	s.minTexels = configHandler->GetFloat("ShadowCasterMinTexels");
	s.texel = shadowHandler.GetTexelWorldSize();
	stats.on = StatsWanted();

	s.recv.clear();
	s.maxR = 0.0f;
	s.gw = s.gh = 0;
	if (!s.receivers)
		return;

	LightBasis();
	for (const CUnit* u: unitHandler.GetActiveUnits())
		AddReceiver(u);
	for (const int id: featureHandler.GetActiveFeatureIDs()) {
		if (const CFeature* f = featureHandler.GetFeature(id); f != nullptr)
			AddReceiver(f);
	}
	BuildGrid();
}

void ShadowCasters::End()
{
	if (!stats.on)
		return;
	stats.texel += s.texel;
	if (++stats.passes < 300)
		return;
	const double n = stats.passes;
	LOG("[shadow-casters] passes=%u texel=%.2f casters=%.1f small=%.1f unreceived=%.1f levels=%.1f,%.1f,%.1f,%.1f,%.1f,%.1f lod=%d receivers=%d minTexels=%.2f",
		stats.passes, stats.texel / n, stats.considered / n, stats.small / n, stats.unreceived / n,
		stats.levels[0] / n, stats.levels[1] / n, stats.levels[2] / n, stats.levels[3] / n, stats.levels[4] / n, stats.levels[5] / n,
		int(s.levels), int(s.receivers), s.minTexels);
	stats = Stats{};
	stats.on = true;
}

bool ShadowCasters::Wanted(const CSolidObject* obj)
{
	stats.considered += stats.on;
	const float r = obj->GetDrawRadius();

	if (s.minTexels > 0.0f && 2.0f * r < s.minTexels * s.texel) {
		stats.small += stats.on;
		return false;
	}
	if (!s.receivers)
		return true;
	// in view itself: it shadows at least itself
	if (obj->HasDrawFlag(DrawFlags::SO_OPAQUE_FLAG) || obj->HasDrawFlag(DrawFlags::SO_ALPHAF_FLAG))
		return true;

	const float3& p = obj->drawMidPos;
	if (Shadows(p.dot(s.lx), p.dot(s.ly), p.dot(s.lz), r))
		return true;

	stats.unreceived += stats.on;
	return false;
}

float ShadowCasters::LevelLength(const CSolidObject* obj, int objType)
{
	if (!s.levels)
		return -1.0f;
	const CCamera* cam = CCameraHandler::GetCamera(CCamera::CAMTYPE_PLAYER);
	const float pixel = std::max(cam->ProjectedDistance(obj->pos), 0.0f) * cam->GetLPPScale() * LuaObjectDrawer::GetLODScale(objType);
	return std::max(s.texel * LuaObjectDrawer::GetLODScaleShadow(objType), pixel);
}

void ShadowCasters::CountLevel(unsigned int level)
{
	if (stats.on)
		stats.levels[std::min(level, 7u)]++;
}
