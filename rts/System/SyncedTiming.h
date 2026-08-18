/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdlib>
#include <string_view>

namespace spring::synced_timing {

// Wall-clock timing is deliberately opt-in for synced script environments.
// Reading it during lockstep simulation can make a script's decisions differ
// between peers, so callers must keep this check immediately next to the
// clock access rather than treating the timer as an ordinary synced API.
inline bool IsEnabledSetting(std::string_view setting)
{
	return setting == "1" || setting == "true" || setting == "TRUE" ||
		setting == "yes" || setting == "YES" || setting == "on" || setting == "ON";
}

// Read once: this is a process-level diagnostic opt-in, and the check sits on
// the per-callout path where a getenv scan is a measurable cost.
inline bool IsEnabled()
{
	static const bool enabled = [] {
		const char* value = std::getenv("SPRING_ENABLE_SYNCED_TIMERS");
		return value != nullptr && IsEnabledSetting(value);
	}();
	return enabled;
}

inline bool IsAllowed(bool syncedEnvironment, bool timerCall, bool enabled)
{
	return !syncedEnvironment || !timerCall || enabled;
}

} // namespace spring::synced_timing
