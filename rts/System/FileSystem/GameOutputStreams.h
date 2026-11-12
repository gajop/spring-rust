/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstddef>

/// Closes every game output stream modules left open (GameDataFiles.h, `GameData::Outputs`); returns
/// how many there were. A header of its own: `GameData` is also a class in Game/GameData.h.
std::size_t CloseGameOutputStreams();
