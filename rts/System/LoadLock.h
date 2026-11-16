#pragma once

#include <chrono>
#include <mutex>

#include "System/Threading/SpringThreading.h"
#include "System/Threading/WrappedSync.h"
#include "System/ScopedResource.h"
#include "Rendering/GlobalRendering.h"

class CLoadLockMtx {
public:
	using native_handle_type = uint32_t;

	void lock() {
		mtx.lock();
		++heldDepth;

		// hack to protect from multiple context acquisions in case of recursive lock
		auto& thisThreadLocksCount = locksCount[Threading::IsGameLoadThread()];
		++thisThreadLocksCount;
		if (thisThreadLocksCount == 1) {
			globalRendering->MakeCurrentContext(false); //set
			globalRendering->ToggleMultisampling();
		}
	}
	void unlock() {

		// hack to protect from multiple context clear in case of recursive lock
		auto& thisThreadLocksCount = locksCount[Threading::IsGameLoadThread()];
		--thisThreadLocksCount;

		if (thisThreadLocksCount == 0)
			globalRendering->MakeCurrentContext(true ); //clear

		--heldDepth;
		mtx.unlock();
	}
	// How many times the calling thread holds the lock.
	static uint32_t HeldDepth() { return heldDepth; }
	bool try_lock() { assert(false); return true; } // placeholder
	native_handle_type native_handle() { return native_handle_type{}; } // placeholder
private:
	std::array<uint32_t, 2> locksCount = { 0 };
	std::recursive_mutex mtx = {};
	inline static thread_local uint32_t heldDepth = 0;
};

class CLoadLockImpl : public spring::WrappedSync<CLoadLockMtx> {
public:
	CLoadLockImpl() : spring::WrappedSync<CLoadLockMtx>() {
		needThreadSafety = false;
	}
	auto& GetMutex() {
		return *sync[needThreadSafety];
	}
};

class CLoadLock {
public:
	static auto& GetMutex() {
		return loadLockImpl.GetMutex();
	}
	static auto GetUniqueLock() {
		return loadLockImpl.GetUniqueLock();
	}
	// With LoadingMT=1, lets the loading screen draw once in the middle of a long step that
	// holds the lock (and so the GL context), as single-threaded loading draws inside long
	// steps (LoadingPump). A no-op when the calling thread does not hold the lock, or did
	// this less than a frame (at the loading screen's 50 Hz) ago. Returns whether it let go.
	static bool YieldToDraw() {
		if (!GetThreadSafety())
			return false;
		const uint32_t depth = CLoadLockMtx::HeldDepth();
		if (depth == 0)
			return false;
		using Clock = std::chrono::steady_clock;
		thread_local Clock::time_point lastYield = {};
		if (Clock::now() - lastYield < std::chrono::milliseconds(20))
			return false;
		auto& mtx = GetMutex();
		for (uint32_t i = 0; i < depth; ++i)
			mtx.unlock();
		// The drawing thread waits on the lock; give it the moment to take it.
		spring::this_thread::sleep_for(std::chrono::milliseconds(1));
		for (uint32_t i = 0; i < depth; ++i)
			mtx.lock();
		lastYield = Clock::now();
		return true;
	}
	static void SetThreadSafety(bool b) { loadLockImpl.SetThreadSafety(b); }
	static bool GetThreadSafety() { return loadLockImpl.GetThreadSafety(); }
private:
	inline static CLoadLockImpl loadLockImpl;
};