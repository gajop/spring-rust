/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstddef>

namespace quadstorage {

// Called only for null maps. The ordinary terrain-map representation is fixed.
// Automatic mode caps empty-grid metadata; object vector payloads grow in both
// representations. Divide before comparing to avoid overflowing on huge maps.
inline bool UseSparse(int mode, size_t cells, size_t bytesPerCell)
{
	if (mode == 1) return false;
	if (mode == 2) return true;
	constexpr size_t DENSE_BUDGET = 16u * 1024u * 1024u;
	return cells > DENSE_BUDGET / bytesPerCell;
}

} // namespace quadstorage
