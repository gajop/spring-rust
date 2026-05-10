/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "CustomColorPalette.h"

#include <algorithm>

CCustomColorPalette customColorPalette;

CCustomColorPalette::CCustomColorPalette()
{
	colors.fill(float4{1.0f, 1.0f, 1.0f, 1.0f});
}

void CCustomColorPalette::SetColor(uint16_t index, float r, float g, float b)
{
	if (!IsValidIndex(index))
		return;

	colors[index] = float4{
		std::clamp(r, 0.0f, 1.0f),
		std::clamp(g, 0.0f, 1.0f),
		std::clamp(b, 0.0f, 1.0f),
		1.0f
	};
}

float4 CCustomColorPalette::GetColor(uint16_t index) const
{
	if (!IsValidIndex(index))
		return {1.0f, 1.0f, 1.0f, 1.0f};

	return colors[index];
}
