/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include <algorithm>
#include <array>
#include <cassert>
#include <cstdint>
#include <cstdio>
#include <deque>
#include <limits>
#include <string>
#include <vector>

// Compile the same production branches as ThreadPool.cpp, including task stats
// and both synced/unsynced queue-precedence paths. UNIT_TEST is intentionally off
// so SCOPED_MT_TIMER remains present in the extracted real method bodies.
#define SYNCCHECK
#define USE_TASK_STATS_TRACKING

static std::vector<std::string> events;
static int timerConstructs;
static int timerClockReads;
static int taskClockReads;
static int activeTimers;
static int executed;
static bool synced;

struct ScopedMtTimer {
	explicit ScopedMtTimer(const char*) { ++timerConstructs; ++timerClockReads; ++activeTimers; }
	~ScopedMtTimer() { timerClockReads += 2; --activeTimers; }
};
#define SCOPED_MT_TIMER(name) ScopedMtTimer timer(name)
static uint64_t spring_now() { ++taskClockReads; return 1; }
struct CSyncChecker { static bool InSyncedCode() { return synced; } };

struct ITaskGroup {
	int id;
	bool slice = false;
	bool selfDelete = false;
	bool highPriority = true;
	bool async = false;
	int reschedules = 0;
	bool IsSliceTask() const { return slice; }
	bool SelfDelete() const { return selfDelete; }
	bool IsHighPriority() const { return highPriority; }
	bool IsAsyncTask() const { return async; }
	bool ShouldReschedule() { return reschedules-- > 0; }
	uint64_t GetDeltaTime(uint64_t) const { return 3; }
	uint64_t ExecuteLoop(int tid, bool) {
		assert(activeTimers == 1);
		taskClockReads += 2; // Existing ExecuteLoop start/end timestamps.
		++executed;
		events.push_back("execute:" + std::to_string(id) + ":" + std::to_string(tid));
		return 7;
	}
	void FinishQueueEntry() { events.push_back("finish:" + std::to_string(id)); }
};

struct Queue {
	std::deque<ITaskGroup*> values;
	bool try_dequeue(ITaskGroup*& group) {
		if (values.empty()) return false;
		group = values.front(); values.pop_front();
		events.push_back("dequeue:" + std::to_string(group->id));
		return true;
	}
	bool enqueue(ITaskGroup* group) {
		events.push_back("enqueue:" + std::to_string(group->id));
		values.push_back(group);
		return true;
	}
};
static std::array<Queue, 4> taskQueues[2];
static std::array<Queue, 4> taskQueuesSyncBackground[2];
struct ThreadStats {
	uint64_t numTasksRun = 0;
	uint64_t sumExecTime = 0;
	uint64_t minExecTime = std::numeric_limits<uint64_t>::max();
	uint64_t maxExecTime = 0;
	uint64_t sumWaitTime = 0;
	uint64_t minWaitTime = std::numeric_limits<uint64_t>::max();
	uint64_t maxWaitTime = 0;
	bool operator==(const ThreadStats&) const = default;
};
static std::array<ThreadStats, 4> threadStats[2];
static void NotifyWorkerThreads(bool, bool async) { events.push_back(async ? "notify:async" : "notify:sync"); }

namespace Baseline {
// INSERT_BASELINE
}
namespace Candidate {
// INSERT_CANDIDATE
}

struct Result {
	std::vector<std::string> events;
	ThreadStats stats;
	int timers;
	int timerClocks;
	int taskClocks;
	int runs;
	bool found;
};

static Result Run(bool candidate, int tid, bool async, bool inSynced, bool populated)
{
	for (int a = 0; a < 2; ++a) {
		for (int i = 0; i < 4; ++i) {
			taskQueues[a][i].values.clear(); taskQueuesSyncBackground[a][i].values.clear();
			threadStats[a][i] = {};
		}
	}
	events.clear(); timerConstructs = timerClockReads = taskClockReads = activeTimers = executed = 0;
	synced = inSynced;
	std::array<ITaskGroup, 7> groups;
	for (size_t i = 0; i < groups.size(); ++i) { groups[i].id = i; groups[i].async = async; }
	if (populated) {
		groups[1].slice = true;
		groups[2].highPriority = false; groups[2].reschedules = 1;
		groups[3].selfDelete = true;
		groups[4].slice = true; groups[4].selfDelete = true;
		// Global primary/background queue order, another async lane (untouched),
		// and per-thread primary/background queues all remain observable.
		taskQueues[async][0].values = {&groups[0], &groups[1], &groups[2]};
		taskQueuesSyncBackground[async][0].values = {&groups[3], &groups[4]};
		taskQueues[!async][0].values = {&groups[0]};
		if (tid != 0) {
			taskQueues[async][tid].values = {&groups[5]};
			taskQueuesSyncBackground[async][tid].values = {&groups[6]};
		}
	}
	bool found = candidate ? Candidate::DoTask(tid, async) : Baseline::DoTask(tid, async);
	assert(activeTimers == 0);
	assert(timerClockReads == timerConstructs * 3);
	assert(taskClockReads == executed * 3);
	assert(threadStats[async][tid].numTasksRun == static_cast<unsigned>(executed));
	if (candidate) assert(timerConstructs == executed);
	else assert(timerConstructs == 1);
	return {events, threadStats[async][tid], timerConstructs, timerClockReads,
		taskClockReads, executed, found};
}

int main()
{
	int cases = 0;
	int oldEmptyClocks = 0, newEmptyClocks = 0;
	int oldTimers = 0, newTimers = 0;
	for (int tid: {0, 1, 2, 3}) {
		for (bool async: {false, true}) {
			for (bool inSynced: {false, true}) {
				for (bool populated: {false, true}) {
					auto baseline = Run(false, tid, async, inSynced, populated);
					auto candidate = Run(true, tid, async, inSynced, populated);
					assert(baseline.events == candidate.events);
					assert(baseline.stats == candidate.stats);
					assert(baseline.taskClocks == candidate.taskClocks);
					assert(baseline.runs == candidate.runs);
					assert(baseline.found == candidate.found);
					assert(candidate.found == populated);
					oldTimers += baseline.timers; newTimers += candidate.timers;
					if (!populated) { oldEmptyClocks += baseline.timerClocks; newEmptyClocks += candidate.timerClocks; }
					++cases;
				}
			}
		}
	}
	std::printf("PASS %d queue/async/sync/thread cases; task ordering, notifications, rescheduling, queue release and task statistics identical\n", cases);
	std::printf("empty-poll profiler clocks baseline=%d candidate=%d\n", oldEmptyClocks, newEmptyClocks);
	std::printf("RunTask scope counts baseline=%d candidate=%d; candidate scopes exactly one per dequeued group\n", oldTimers, newTimers);
}
