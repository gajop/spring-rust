/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */
#pragma once

#include <algorithm>
#include <cmath>
#include <sstream>
#include <string>

// Unsynced view state only. An unset rectangle retains the map's normal view.
struct ViewRectangle {
	float x0 = 0.0f;
	float z0 = 0.0f;
	float x1 = 0.0f;
	float z1 = 0.0f;

	bool IsSet() const { return x1 > x0 && z1 > z0; }
	float Width() const { return x1 - x0; }
	float Height() const { return z1 - z0; }
	float CenterX() const { return x0 + Width() * 0.5f; }
	float CenterZ() const { return z0 + Height() * 0.5f; }
	bool Contains(float x, float z) const { return x >= x0 && x <= x1 && z >= z0 && z <= z1; }
	float ClampX(float x) const {
		const float margin = std::min(0.01f, Width() * 0.25f);
		return std::clamp(x, x0 + margin, x1 - margin);
	}
	float ClampZ(float z) const {
		const float margin = std::min(0.01f, Height() * 0.25f);
		return std::clamp(z, z0 + margin, z1 - margin);
	}

	// Invalid input leaves the previous rectangle intact.
	static bool Parse(const std::string& command, float mapWidth, float mapHeight, ViewRectangle& result) {
		std::istringstream stream(command);
		std::string trailing;
		std::string first;
		stream >> first;
		if (first == "clear" && !(stream >> trailing)) {
			result = {};
			return true;
		}
		stream.clear();
		stream.str(command);
		ViewRectangle rectangle;
		if (!(stream >> rectangle.x0 >> rectangle.z0 >> rectangle.x1 >> rectangle.z1) || (stream >> trailing))
			return false;
		if (!std::isfinite(rectangle.x0) || !std::isfinite(rectangle.z0) ||
			!std::isfinite(rectangle.x1) || !std::isfinite(rectangle.z1) || !rectangle.IsSet() ||
			rectangle.x0 < 0.0f || rectangle.z0 < 0.0f || rectangle.x1 > mapWidth || rectangle.z1 > mapHeight)
			return false;
		result = rectangle;
		return true;
	}
};
