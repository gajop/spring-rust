/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "DefaultInterface.h"

#include "Game/Game.h"
#include "Game/UI/EndGameBox.h"
#include "Game/UI/GuiHandler.h"
#include "Game/UI/InfoConsole.h"
#include "Game/UI/MiniMap.h"
#include "Game/UI/PlayerRoster.h"
#include "Game/UI/ResourceBar.h"
#include "Game/UI/TooltipConsole.h"
#include "Map/BaseGroundDrawer.h"
#include "Map/ReadMap.h"
#include "Rendering/Units/UnitDrawer.h"

// Far enough that units never turn into distance icons.
static constexpr float NO_UNIT_ICONS_DIST = 1e9f;

std::uint32_t CDefaultInterface::SetVisible(std::uint32_t parts, bool visible)
{
	for (std::uint32_t bit = 1; (bit & ALL) != 0; bit <<= 1) {
		if ((parts & bit) == 0)
			continue;

		const bool isHidden = (hidden & bit) != 0;

		if (visible && isHidden) {
			Show(static_cast<Part>(bit));
			hidden &= ~bit;
		} else if (!visible && !isHidden) {
			Hide(static_cast<Part>(bit));
			hidden |= bit;
		}
	}

	return hidden;
}

void CDefaultInterface::Hide(Part part)
{
	switch (part) {
		case CONSOLE: {
			if (infoConsole != nullptr) {
				consoleEnabled = infoConsole->enabled;
				infoConsole->enabled = false;
			}
		} break;
		case RESOURCE_BAR: {
			if (resourceBar != nullptr) {
				resourceBarEnabled = resourceBar->enabled;
				resourceBar->enabled = false;
			}
		} break;
		case TOOLTIP: {
			if (tooltip != nullptr) {
				tooltipEnabled = tooltip->enabled;
				tooltip->enabled = false;
			}
		} break;
		case CLOCK: {
			clockShown = game->showClock;
			game->showClock = false;
		} break;
		case FPS: {
			fpsShown = game->showFPS;
			game->showFPS = false;
		} break;
		case SPEED: {
			speedShown = game->showSpeed;
			game->showSpeed = false;
		} break;
		case PLAYER_INFO: {
			playerInfoSort = playerRoster.GetSortType();
			playerRoster.SetSortTypeByCode(PlayerRoster::Disabled);
		} break;
		case MINIMAP: {
			if (minimap != nullptr)
				minimap->SetHidden(true);
		} break;
		case COMMAND_MENU: {
			if (guihandler != nullptr)
				guihandler->SetDefaultMenuVisible(false);
		} break;
		case END_GRAPH: {
			endGraphMode = CEndGameBox::enabledMode;
			CEndGameBox::enabledMode = 0;
		} break;
		case UNIT_ICONS: {
			unitIconDist = CUnitDrawer::GetUnitIconDist(0.0f);
			CUnitDrawer::SetUnitIconDist(NO_UNIT_ICONS_DIST);
		} break;
		case MAP_BORDER: {
			if (readMap != nullptr && readMap->GetGroundDrawer() != nullptr) {
				CBaseGroundDrawer* groundDrawer = readMap->GetGroundDrawer();
				mapBorderShown = groundDrawer->GetDrawMapEdges();
				if (mapBorderShown)
					groundDrawer->ToggleMapBorder();
			}
		} break;
		default: {
		} break;
	}
}

void CDefaultInterface::Show(Part part)
{
	switch (part) {
		case CONSOLE: {
			if (infoConsole != nullptr)
				infoConsole->enabled = consoleEnabled;
		} break;
		case RESOURCE_BAR: {
			if (resourceBar != nullptr)
				resourceBar->enabled = resourceBarEnabled;
		} break;
		case TOOLTIP: {
			if (tooltip != nullptr)
				tooltip->enabled = tooltipEnabled;
		} break;
		case CLOCK: {
			game->showClock = clockShown;
		} break;
		case FPS: {
			game->showFPS = fpsShown;
		} break;
		case SPEED: {
			game->showSpeed = speedShown;
		} break;
		case PLAYER_INFO: {
			playerRoster.SetSortTypeByCode(static_cast<PlayerRoster::SortType>(playerInfoSort));
		} break;
		case MINIMAP: {
			if (minimap != nullptr)
				minimap->SetHidden(false);
		} break;
		case COMMAND_MENU: {
			if (guihandler != nullptr)
				guihandler->SetDefaultMenuVisible(true);
		} break;
		case END_GRAPH: {
			CEndGameBox::enabledMode = endGraphMode;
		} break;
		case UNIT_ICONS: {
			CUnitDrawer::SetUnitIconDist(unitIconDist);
		} break;
		case MAP_BORDER: {
			if (readMap != nullptr && readMap->GetGroundDrawer() != nullptr) {
				CBaseGroundDrawer* groundDrawer = readMap->GetGroundDrawer();
				if (groundDrawer->GetDrawMapEdges() != mapBorderShown)
					groundDrawer->ToggleMapBorder();
			}
		} break;
		default: {
		} break;
	}
}
