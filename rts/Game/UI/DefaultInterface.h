/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>

/**
 * Hides and restores parts of the engine's built-in interface for games that
 * draw their own. Changes are runtime-only (nothing is written to the user's
 * config) and idempotent; showing a part restores the state it had when it
 * was hidden. One instance lives per game.
 */
class CDefaultInterface
{
public:
	// Bit values are part of the NativeInterface ABI (DefaultInterfacePart).
	enum Part : std::uint32_t {
		CONSOLE      = 1u <<  0,
		RESOURCE_BAR = 1u <<  1,
		TOOLTIP      = 1u <<  2,
		CLOCK        = 1u <<  3,
		FPS          = 1u <<  4,
		SPEED        = 1u <<  5,
		PLAYER_INFO  = 1u <<  6,
		MINIMAP      = 1u <<  7,
		COMMAND_MENU = 1u <<  8,
		END_GRAPH    = 1u <<  9,
		UNIT_ICONS   = 1u << 10,
		MAP_BORDER   = 1u << 11,
		ALL          = (1u << 12) - 1,
	};

	/// Show or hide `parts`; returns the mask of parts now hidden.
	std::uint32_t SetVisible(std::uint32_t parts, bool visible);
	std::uint32_t GetHidden() const { return hidden; }

private:
	void Hide(Part part);
	void Show(Part part);

	std::uint32_t hidden = 0;

	// State saved when a part was hidden, restored when it is shown again.
	bool consoleEnabled = true;
	bool resourceBarEnabled = true;
	bool tooltipEnabled = true;
	bool clockShown = true;
	bool fpsShown = true;
	bool speedShown = true;
	int playerInfoSort = 0;
	int endGraphMode = 0;
	float unitIconDist = 0.0f;
	bool mapBorderShown = true;
};
