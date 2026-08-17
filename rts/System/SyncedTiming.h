/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdlib>
#include <string_view>

namespace spring::synced_timing {

// Wall-clock timing is deliberately opt-in for synced script environments.
// Reading it during lockstep simulation can make a script's decisions differ
// between peers, so callers must keep this check immediately next to the
// clock access rather than treating the timer as an ordinary synced API.
inline bool IsEnabled()
{
	const char* value = std::getenv("SPRING_ENABLE_SYNCED_TIMERS");
	if (value == nullptr)
		return false;

	const std::string_view setting(value);
	return setting == "1" || setting == "true" || setting == "TRUE" ||
		setting == "yes" || setting == "YES" || setting == "on" || setting == "ON";
}

} // namespace spring::synced_timing
