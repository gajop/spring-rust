/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <atomic>
#include <chrono>
#include <future>
#include <utility>

// Keeps the loading screen drawing while one long step runs. In single-threaded loading
// (LoadingMT=0, the default) the screen otherwise redraws only when a step posts a new
// load message, so a step of several seconds freezes it and the window stops responding.
// The load screen installs the hooks for its lifetime; without them every call is a no-op,
// so code that also runs outside loading can call these freely.
namespace LoadingPump {
	// With draw unset, reports whether the calling thread could draw the loading screen;
	// with draw set, draws it if its last draw is older than the screen's interval.
	using PumpHook = bool (*)(bool draw);
	// Replaces the screen's last progress line without logging it, then pumps.
	using ProgressHook = void (*)(const char* text);

	inline std::atomic<PumpHook> pumpHook{nullptr};
	inline std::atomic<ProgressHook> progressHook{nullptr};

	inline void Install(PumpHook pump, ProgressHook progress)
	{
		pumpHook.store(pump, std::memory_order_release);
		progressHook.store(progress, std::memory_order_release);
	}

	inline bool Active()
	{
		const PumpHook hook = pumpHook.load(std::memory_order_acquire);
		return hook != nullptr && hook(false);
	}

	inline void Pump()
	{
		if (const PumpHook hook = pumpHook.load(std::memory_order_acquire))
			hook(true);
	}

	inline void Progress(const char* text)
	{
		if (const ProgressHook hook = progressHook.load(std::memory_order_acquire))
			hook(text);
	}

	// Runs work on a helper thread while this one keeps the screen drawing, when it can;
	// inline otherwise. The work must not touch GL or other main-thread-only state.
	template<typename Work>
	auto RunPumped(Work&& work) -> decltype(work())
	{
		if (!Active())
			return work();

		auto future = std::async(std::launch::async, std::forward<Work>(work));
		while (future.wait_for(std::chrono::milliseconds(10)) != std::future_status::ready)
			Pump();

		return future.get();
	}
}
