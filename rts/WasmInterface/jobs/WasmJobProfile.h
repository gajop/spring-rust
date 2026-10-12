/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include "System/Misc/TracyDefs.h"

#if defined(TRACY_ENABLE) && defined(RECOIL_WASM_JOB_DETAIL_ZONES)
#include <optional>

// Keep one stage open at a time, including on early returns. Declare stages
// before the store/linker, and the teardown marker after them: destruction then
// closes the output/error stage, profiles BOTH resource destructors, and finally
// closes teardown. Uses Tracy's per-thread events, with no separate logger or
// shared aggregation lock in workers. Ordinary builds compile this helper out.
class WasmJobProfileStages {
public:
	WasmJobProfileStages(): active(TracyIsConnected) {}
	void Next(const tracy::SourceLocationData* location) {
		if (!active) return;
		zone.reset();
		zone.emplace(location, true);
	}
private:
	bool active;
	std::optional<tracy::ScopedZone> zone;
};

class WasmJobProfileTeardown {
public:
	WasmJobProfileTeardown(WasmJobProfileStages& stages, const tracy::SourceLocationData* location):
		stages(stages), location(location) {}
	~WasmJobProfileTeardown() { stages.Next(location); }
private:
	WasmJobProfileStages& stages;
	const tracy::SourceLocationData* location;
};

#define WASM_JOB_PROFILE_START(state) WasmJobProfileStages state
#define WASM_JOB_PROFILE_STAGE(state, label) \
	static constexpr tracy::SourceLocationData TracyConcat(jobProfileLocation, __LINE__) \
		{label, TracyFunction, TracyFile, __LINE__, 0}; \
	state.Next(&TracyConcat(jobProfileLocation, __LINE__))
#define WASM_JOB_PROFILE_TEARDOWN(state) \
	static constexpr tracy::SourceLocationData jobTeardownLocation \
		{"Wasm::Jobs::Teardown", TracyFunction, TracyFile, __LINE__, 0}; \
	WasmJobProfileTeardown jobTeardown(state, &jobTeardownLocation)
#else
#define WASM_JOB_PROFILE_START(state)
#define WASM_JOB_PROFILE_STAGE(state, label)
#define WASM_JOB_PROFILE_TEARDOWN(state)
#endif
