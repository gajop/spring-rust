/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <algorithm>
#include <cstddef>

#include "Game/Camera.h"
#include "Game/CameraHandler.h"
#include "Map/ReadMap.h"
#include "Map/VisibleGridQuery.h"
#include "Sim/Misc/QuadField.h"
#include "System/Config/ConfigHandler.h"

namespace visibleobjects {

// A completed grid collects the same candidate lists as GridVisibility. On
// budget exhaustion the caller must discard the partial lists and scan active
// objects instead. Both paths retain the query's existing per-object filters.
template<typename Drawer>
bool CollectSpatial(Drawer& drawer, size_t population)
{
	constexpr int QUAD_SIZE = CQuadField::BASE_QUAD_SIZE / SQUARE_SIZE;
	if (!readMap->IsNullMap() || configHandler->GetInt("SparseMapVisibilityQuery") == 1) {
		readMap->GridVisibility(nullptr, &drawer, 1e9, QUAD_SIZE);
		return true;
	}
	if (configHandler->GetInt("SparseMapVisibilityQuery") == 2)
		return false;

	// Small populations do not amortize even the row/frustum calculations.
	// Storage selection is independent of which query algorithm wins.
	const int width = mapDims.mapx / QUAD_SIZE;
	const int height = mapDims.mapy / QUAD_SIZE;
	if (population <= 256 || population <= size_t(height) / 2)
		return false;

	CCamera* cam = CCameraHandler::GetCamera(CCamera::CAMTYPE_VISCUL);
	cam->CalcFrustumLines(readMap->GetCurrMinHeight() - 100.0f, readMap->GetCurrMaxHeight() + 100.0f, SQUARE_SIZE);

	// Mirror GridVisibility's maxDist=1e9, extraSize=0 row bounds. Ordinary
	// terrain maps keep that original implementation and incur no budgets.
	const int cx = cam->GetPos().x / (SQUARE_SIZE * QUAD_SIZE);
	const int cy = cam->GetPos().z / (SQUARE_SIZE * QUAD_SIZE);
	const int distance = int(1e9 / (SQUARE_SIZE * QUAD_SIZE)) + 1;
	const int sy = std::clamp(cy - distance, 0, height - 1);
	const int ey = std::clamp(cy + distance, 0, height - 1);
	const int left = std::clamp(cx - distance, 0, width - 1);
	const int right = std::clamp(cx + distance, 0, width - 1);
	return visiblegrid::CollectBounded({width, height, left, right, sy, ey}, QUAD_SIZE,
		cam->GetNegFrustumLines(), cam->GetPosFrustumLines(), drawer, population);
}

} // namespace visibleobjects
