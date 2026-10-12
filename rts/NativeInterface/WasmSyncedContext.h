/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

namespace WasmSyncedContext {

// Native callouts need the guest's world even in builds without SYNCCHECK.
// Nested unsynced imports must restore, rather than inherit, the outer world.
inline thread_local bool current = false;
inline bool Active() { return current; }

class ScopedContext {
public:
	explicit ScopedContext(bool synced): previous(current) { current = synced; }
	~ScopedContext() { current = previous; }
	ScopedContext(const ScopedContext&) = delete;
	ScopedContext& operator=(const ScopedContext&) = delete;
private:
	bool previous;
};

// During demo playback only recorded players belong to the synced world.
// The replay viewer may create additional local spectators.
inline bool PlayerVisible(bool hostDemo, bool fromDemo)
{
	return !Active() || !hostDemo || fromDemo;
}

} // namespace WasmSyncedContext
