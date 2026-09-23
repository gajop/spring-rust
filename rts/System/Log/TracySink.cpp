/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#ifdef TRACY_ENABLE

#include "Backend.h"
#include "LogUtil.h"
#include "Level.h"
#include <fmt/format.h>

#include <algorithm>
#include <chrono>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include "System/Misc/TracyDefs.h"

static void log_sink_record_tracy(int level, const char* section, const char* record)
{
	auto buf = fmt::format("[{}:{}][{}] {}", log_util_levelToString(log_util_getNearestLevel(level)),
	                       level, section, record);
	// Profile-window messages bridge perf's monotonic clock and Tracy's clock.
	// Opt-in and local to the launched process; no synchronized state is changed.
	static const char* const markersPath = std::getenv("SPRING_PROFILE_MARKERS");
	if (const char* path = markersPath) {
		const char* tag = std::strstr(record, "[profile-window] ");
		if (tag != nullptr) {
			const auto ns = std::chrono::duration_cast<std::chrono::nanoseconds>(
				std::chrono::steady_clock::now().time_since_epoch()).count();
			if (FILE* file = std::fopen(path, "a")) {
				std::fprintf(file, "%lld\t%s\n", static_cast<long long>(ns), tag);
				std::fclose(file);
			}
		}
		// Routine diagnostic logs must not generate thousands of symbol requests.
		// Preserve stack capture for warnings/errors and outside profile mode.
		if (level < LOG_LEVEL_WARNING) {
			TracyMessage(buf.c_str(), buf.size());
			return;
		}
	}
	TracyMessageS(buf.c_str(), buf.size(), 30);
}

namespace {

/// Auto-registers the sink defined in this file before main() is called
struct TracySinkRegistrator {
	TracySinkRegistrator() {
		log_backend_registerSink(&log_sink_record_tracy);
	}
	~TracySinkRegistrator() {
		log_backend_unregisterSink(&log_sink_record_tracy);
	}
} tracySinkRegistrator;

}

#endif
