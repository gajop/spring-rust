/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmSyncedJobs.h"

#include <algorithm>
#include <atomic>
#include <chrono>
#include <cstdlib>
#include <string_view>
#include <exception>
#include <functional>
#include <limits>
#include <stdexcept>

#include "System/Log/ILog.h"
#include "System/SpringHash.h"
#include "System/Misc/TracyDefs.h"
#include "System/Threading/ThreadPool.h"

namespace {
// `SPRING_WASM_JOB_TIMING=1`: where a job's time goes (instantiation, copies, the
// export, teardown; summed over workers) and how long the simulation thread spends
// in same-frame batches and at the next-frame barrier, logged every 900 frames.
// A local diagnostic: the clocks never reach synced state.
struct JobClock {
	using Clock = std::chrono::steady_clock;
	const bool on = [] {
		const char* value = std::getenv("SPRING_WASM_JOB_TIMING");
		return value != nullptr && std::string_view(value) == "1";
	}();
	std::atomic<std::uint64_t> jobs{0}, instantiateNs{0}, inputNs{0}, executeNs{0}, outputNs{0}, totalNs{0};
	std::uint64_t batchNs = 0, barrierNs = 0, batches = 0;
	int since = -1;

	static std::uint64_t Ns(Clock::time_point a, Clock::time_point b) {
		return std::uint64_t(std::chrono::duration_cast<std::chrono::nanoseconds>(b - a).count());
	}
	WasmJobResult Run(const WasmJobProgram& program, std::string_view exportName,
		std::span<const std::uint8_t> input, std::size_t maxOutputBytes) {
		if (!on)
			return program.Run(exportName, input, nullptr, maxOutputBytes);
		WasmJobTiming timing;
		const auto start = Clock::now();
		auto result = program.Run(exportName, input, &timing, maxOutputBytes);
		const auto total = Ns(start, Clock::now());
		jobs.fetch_add(1, std::memory_order_relaxed);
		instantiateNs.fetch_add(timing.instantiateNs, std::memory_order_relaxed);
		inputNs.fetch_add(timing.inputCopyNs, std::memory_order_relaxed);
		executeNs.fetch_add(timing.dispatchNs, std::memory_order_relaxed);
		outputNs.fetch_add(timing.outputCopyNs, std::memory_order_relaxed);
		totalNs.fetch_add(total, std::memory_order_relaxed);
		return result;
	}
	void Report(int frame) {
		if (!on)
			return;
		if (since < 0) { since = frame; return; }
		if (frame - since < 900)
			return;
		const double frames = frame - since;
		const auto n = jobs.exchange(0);
		const double per = n ? 1e-3 / n : 0; // microseconds a job
		const double inst = instantiateNs.exchange(0), in = inputNs.exchange(0), exe = executeNs.exchange(0),
			out = outputNs.exchange(0), total = totalNs.exchange(0);
		LOG("[Wasm jobs] timing frames %d-%d: %.1f jobs a frame; a job, us: instantiate %.1f, input %.1f, "
			"execute %.1f, output %.1f, teardown %.1f, whole %.1f; workers' sum %.3f ms a frame (setup and teardown %.3f); "
			"simulation thread: same-frame batches %.3f ms a frame (%.1f a frame), next-frame barrier %.3f ms a frame",
			since, frame, n / frames, inst * per, in * per, exe * per, out * per,
			(total - inst - in - exe - out) * per, total * per, total * 1e-6 / frames,
			(total - in - exe - out) * 1e-6 / frames, batchNs * 1e-6 / frames, batches / frames, barrierNs * 1e-6 / frames);
		batchNs = barrierNs = batches = 0;
		since = frame;
	}
};
JobClock& Timing() { static JobClock clock; return clock; }

// A failed job's status reaches the game as synced bytes; its diagnostic (the
// trap message) only this machine's log, so a game can tell a trap from a limit.
void LogFailure(std::string_view kind, std::uint32_t id, int frame, const std::string& exportName,
	std::size_t inputBytes, const WasmJobResult& result)
{
	if (result.status == WasmJobStatus::Ok)
		return;
	LOG_L(L_WARNING, "[Wasm jobs] %.*s job %u (frame %d, %s, %zu input bytes): status %u: %s",
		static_cast<int>(kind.size()), kind.data(), id, frame, exportName.c_str(), inputBytes,
		static_cast<unsigned>(result.status), result.diagnostic.c_str());
}
}

struct WasmSyncedJobs::State {
	explicit State(Mode mode): mode(mode) {}
	Mode mode;
	struct Job {
		void* owner;
		std::uint32_t id;
		int frame;
		std::shared_ptr<const WasmJobProgram> program;
		std::string exportName;
		std::vector<std::uint8_t> input;
		std::uint64_t inputHash;
#if defined(TRACY_ENABLE)
		std::string_view profileOwner;
#endif
	};
	struct Batch {
		std::vector<Job> jobs;
		std::vector<WasmJobResult> results;
		std::vector<std::exception_ptr> exceptions;
	};
	std::vector<Job> pending;
	// Sealed batches of this window in submission order; Flush seals early ones.
	std::vector<std::shared_ptr<Batch>> active;
	std::uint32_t nextId = 1;
	std::size_t inputBytes = 0;
	// Admission already sealed by Flush this window, so quotas match one batch.
	std::size_t sealedJobs = 0;
	std::size_t sealedInputBytes = 0;
	std::size_t batchJobs = 0;
	std::size_t batchInputBytes = 0;
	std::size_t batchOutputBytes = 0;
	bool runningBatch = false;
	std::exception_ptr batchFailure;
	int frame = -1;
	bool accepting = false;
#if defined(THREADPOOL)
	using Work = std::function<void(int)>;
	std::vector<std::shared_ptr<ForBackgroundTaskGroup<Work>>> background;
#endif

	void Wait()
	{
#if defined(THREADPOOL)
		for (auto& group: background)
			wait_for_mt_background(group);
		background.clear();
#endif
	}
	void Seal();
};

WasmSyncedJobs::WasmSyncedJobs(Mode mode): state(std::make_unique<State>(mode))
{
	if (mode == Mode::Update) BeginFrame(0);
}
WasmSyncedJobs::~WasmSyncedJobs() { state->Wait(); }

std::uint32_t WasmSyncedJobs::Submit(void* owner,
	std::shared_ptr<const WasmJobProgram> program, std::string exportName,
	std::span<const std::uint8_t> input
#if defined(TRACY_ENABLE)
	, std::string_view profileOwner
#endif
)
{
	ZoneScopedN("Wasm::Jobs::Admission");
	if (!Accepting() || state->frame < 0 || owner == nullptr || !program || exportName.empty() || exportName.size() > 256 ||
		state->nextId == 0 || state->pending.size() + state->sealedJobs + state->batchJobs >= MAX_JOBS_PER_FRAME ||
		input.size() > MAX_INPUT_BYTES ||
		input.size() > MAX_FRAME_INPUT_BYTES - state->inputBytes - state->sealedInputBytes - state->batchInputBytes)
		return 0;
	const auto id = state->nextId++;
	ZoneValue(id);
	if (ZoneIsActive)
		ZoneTextF("frame=%d job=%u export=%s input=%zu", state->frame, id, exportName.c_str(), input.size());
	state->pending.push_back({owner, id, state->frame, std::move(program),
		std::move(exportName), {input.begin(), input.end()}, 0
#if defined(TRACY_ENABLE)
		, profileOwner
#endif
	});
	// Hash the owned snapshot once at admission, rather than scanning payloads
	// from the ordinary end-of-frame checksum (up to 16 MiB per batch).
	auto& job = state->pending.back();
	job.inputHash = XXH3_64bits(job.input.data(), job.input.size());
	state->inputBytes += input.size();
	return id;
}

std::vector<WasmSyncedJobs::Delivery> WasmSyncedJobs::BeginFrame(int frame)
{
	if (state->runningBatch)
		throw std::logic_error("Cannot advance Wasm jobs during a synchronous batch");
	if (state->batchFailure) std::rethrow_exception(state->batchFailure);
	{
		ZoneNamedN(barrier, "Wasm::Jobs::Barrier", !state->active.empty());
		ZoneValueV(barrier, frame);
		const auto start = JobClock::Clock::now();
		state->Wait();
		if (Timing().on && state->mode == Mode::Synced) {
			Timing().barrierNs += JobClock::Ns(start, JobClock::Clock::now());
			Timing().Report(frame);
		}
	}
	ZoneNamedN(collect, "Wasm::Jobs::Collect", !state->active.empty());
	ZoneValueV(collect, frame);
	if (!state->pending.empty() && (state->mode != Mode::Update ||
		std::any_of(state->pending.begin(), state->pending.end(), [frame](const auto& job) { return job.frame != frame; })))
		throw std::logic_error("Wasm job submissions were not sealed at the prior frame boundary");
	std::vector<Delivery> deliveries;
	// Complete the whole barrier before publishing any result. Host resource
	// failure terminates simulation instead of silently becoming game state.
	for (const auto& batch: state->active) {
		for (std::size_t i = 0; i < batch->jobs.size(); ++i) {
			if (batch->jobs[i].frame + 1 != frame)
				throw std::logic_error("Wasm job missed its fixed next-frame delivery boundary");
			if (batch->exceptions[i])
				std::rethrow_exception(batch->exceptions[i]);
			if (batch->results[i].status == WasmJobStatus::RuntimeFailure)
				throw std::runtime_error("Wasm job host failure: " + batch->results[i].diagnostic);
		}
	}
	for (const auto& batch: state->active) {
		for (std::size_t i = 0; i < batch->jobs.size(); ++i) {
			auto& job = batch->jobs[i];
			LogFailure("next-frame", job.id, job.frame, job.exportName, job.input.size(), batch->results[i]);
			deliveries.push_back({job.owner, job.id, job.frame, std::move(batch->results[i])});
		}
	}
	state->active.clear();
	state->sealedJobs = 0;
	state->sealedInputBytes = 0;
	state->frame = frame;
	state->accepting = true;
	if (state->mode == Mode::Synced) {
		state->batchJobs = 0;
		state->batchInputBytes = 0;
		state->batchOutputBytes = 0;
	}
	return deliveries;
}

bool WasmSyncedJobs::Accepting() const { return state->accepting && !state->runningBatch && !state->batchFailure; }

void WasmSyncedJobs::FailBatch(std::exception_ptr failure)
{
	if (!state->batchFailure) state->batchFailure = std::move(failure);
}

WasmSyncedJobs::BatchStatus WasmSyncedJobs::RunBatch(void* owner,
	std::shared_ptr<const WasmJobProgram> program, std::string exportName,
	std::vector<WasmJobBatchInput> inputs, std::vector<WasmJobResult>& results) try
{
	if (!Accepting() || state->frame < 0)
		return BatchStatus::InvalidState;
	if (!owner || !program || exportName.empty() || exportName.size() > 256 || inputs.empty())
		return BatchStatus::InvalidArgument;
	if (inputs.size() > MAX_JOBS_PER_FRAME - state->pending.size() - state->sealedJobs - state->batchJobs)
		return BatchStatus::BufferOverflow;
	std::size_t inputBytes = 0, outputBytes = 0;
	for (const auto& input: inputs) {
		if (input.bytes.size() > MAX_INPUT_BYTES || input.maxOutputBytes > WasmJobBatch::MAX_BYTES)
			return BatchStatus::InvalidArgument;
		if (input.bytes.size() > MAX_FRAME_INPUT_BYTES - inputBytes ||
			input.maxOutputBytes > WasmJobBatch::MAX_TOTAL_BYTES - outputBytes)
			return BatchStatus::BufferOverflow;
		inputBytes += input.bytes.size();
		outputBytes += input.maxOutputBytes;
	}
	if (inputBytes > MAX_FRAME_INPUT_BYTES - state->inputBytes - state->sealedInputBytes - state->batchInputBytes ||
		outputBytes > WasmJobBatch::MAX_TOTAL_BYTES - state->batchOutputBytes)
		return BatchStatus::BufferOverflow;

	// Allocate every result/exception slot before committing admission. Nothing
	// below refers to caller memory or mutable host/queue state in workers.
	std::vector<WasmJobResult> completed(inputs.size());
	std::vector<std::exception_ptr> exceptions(inputs.size());
	state->batchJobs += inputs.size();
	state->batchInputBytes += inputBytes;
	state->batchOutputBytes += outputBytes;
	state->runningBatch = true;
	struct Scope {
		bool& running;
		bool priorMtSection;
		~Scope() {
			ThreadPool::SetInMultiThreadedSection(priorMtSection);
			running = false;
		}
	} scope{state->runningBatch, bool(ThreadPool::IsInMultiThreadedSection())};
	ZoneScopedN("Wasm::Jobs::RunBatch");
	ZoneValue(inputs.size());
	if (ZoneIsActive)
		ZoneTextF("frame=%d jobs=%zu input=%zu output-reserved=%zu threads=%d export=%s",
			state->frame, inputs.size(), inputBytes, outputBytes, ThreadPool::GetNumThreads(), exportName.c_str());
	const auto batchStart = JobClock::Clock::now();
	for_mt(0, inputs.size(), 1, [&](int index) {
		ZoneScopedN("Wasm::Jobs::BatchWorker");
		ZoneValue(index);
		try {
			completed[index] = Timing().Run(*program, exportName, inputs[index].bytes, inputs[index].maxOutputBytes);
		} catch (...) {
			exceptions[index] = std::current_exception();
		}
	});
	if (Timing().on) {
		Timing().batchNs += JobClock::Ns(batchStart, JobClock::Clock::now());
		Timing().batches++;
	}
	// The whole barrier completes before publishing even the first result.
	for (std::size_t i = 0; i < completed.size(); ++i) {
		if (exceptions[i]) std::rethrow_exception(exceptions[i]);
		if (completed[i].status == WasmJobStatus::RuntimeFailure)
			throw std::runtime_error("Wasm batch host failure: " + completed[i].diagnostic);
		LogFailure("same-frame", static_cast<std::uint32_t>(i), state->frame, exportName, inputs[i].bytes.size(), completed[i]);
	}
	results = std::move(completed);
	return BatchStatus::Ok;
} catch (...) {
	FailBatch(std::current_exception());
	throw;
}

std::vector<WasmSyncedJobs::Delivery> WasmSyncedJobs::BeginUpdate()
{
	if (state->mode != Mode::Update)
		throw std::logic_error("A synced job queue cannot be advanced by an unsynced Update");
	return BeginFrame(state->frame);
}

void WasmSyncedJobs::EndUpdate()
{
	if (state->mode != Mode::Update)
		throw std::logic_error("A synced job queue cannot be sealed by an unsynced Update");
	EndFrame();
	if (state->frame == std::numeric_limits<int>::max())
		throw std::overflow_error("Wasm job update sequence exhausted");
	++state->frame;
	state->batchJobs = 0;
	state->batchInputBytes = 0;
	state->batchOutputBytes = 0;
	state->accepting = true;
}

void WasmSyncedJobs::EndFrame()
{
	if (state->runningBatch)
		throw std::logic_error("Cannot seal Wasm jobs during a synchronous batch");
	if (state->batchFailure) std::rethrow_exception(state->batchFailure);
	state->accepting = false;
	if (!state->active.empty() && state->active.front()->jobs.front().frame != state->frame)
		throw std::logic_error("Prior Wasm job batch was not consumed");
	state->Seal();
}

void WasmSyncedJobs::Flush()
{
	if (state->mode != Mode::Synced || !Accepting())
		return;
	state->Seal();
}

// Dispatch what is pending as one background batch. Results still wait for the
// next BeginFrame and arrive in submission order, so when a batch is sealed only
// moves its work earlier in wall time.
void WasmSyncedJobs::State::Seal()
{
	if (pending.empty())
		return;
	ZoneScopedN("Wasm::Jobs::Seal");
	ZoneValue(frame);
	if (ZoneIsActive)
		ZoneTextF("frame=%d jobs=%zu input=%zu threads=%d", frame,
		pending.size(), inputBytes, ThreadPool::GetNumThreads());
	auto batch = std::make_shared<Batch>();
	batch->jobs = std::move(pending);
	pending.clear();
	sealedJobs += batch->jobs.size();
	sealedInputBytes += inputBytes;
	inputBytes = 0;
	batch->results.resize(batch->jobs.size());
	batch->exceptions.resize(batch->jobs.size());
	active.push_back(batch);
	// The queue owns this batch until Wait completes. Do not capture a shared
	// pointer: the engine pools 256 task groups and retains their last callable.
	std::function<void(int)> work = [batch = batch.get()](int index) {
		const auto& job = batch->jobs[index];
		ZoneScopedN("Wasm::Jobs::Worker");
		ZoneValue(job.id);
		if (ZoneIsActive)
			ZoneTextF("frame=%d job=%u worker=%d owner=%.*s export=%s module=%.12s input=%zu", job.frame,
				job.id, ThreadPool::GetThreadNum(), static_cast<int>(job.profileOwner.size()), job.profileOwner.data(),
				job.exportName.c_str(), job.program->Identity().c_str(), job.input.size());
		try {
			batch->results[index] = Timing().Run(*job.program, job.exportName, job.input, 1024 * 1024);
		} catch (...) {
			batch->exceptions[index] = std::current_exception();
		}
		if (ZoneIsActive)
			ZoneTextF("status=%u output=%zu exception=%d", static_cast<unsigned>(batch->results[index].status),
				batch->results[index].bytes.size(), int(batch->exceptions[index] != nullptr));
	};
#if defined(THREADPOOL)
	background.push_back(for_mt_background(0, batch->jobs.size(), 1, std::move(work)));
#else
	for (int i = 0; i < batch->jobs.size(); ++i) work(i);
#endif
}

void WasmSyncedJobs::CancelOwner(void* owner)
{
	if (state->runningBatch)
		throw std::logic_error("Cannot remove a Wasm batch caller during its active callback");
	// Drain before removing this owner's result slots. Workers never read the
	// owner pointer; their programs independently retain the runtime engine.
	state->Wait();
	std::erase_if(state->pending, [owner](const auto& job) { return job.owner == owner; });
	state->inputBytes = 0;
	for (const auto& job: state->pending) state->inputBytes += job.input.size();
	for (const auto& batch: state->active) {
		for (std::size_t i = batch->jobs.size(); i-- > 0;) {
			if (batch->jobs[i].owner != owner) continue;
			batch->jobs.erase(batch->jobs.begin() + i);
			batch->results.erase(batch->results.begin() + i);
			batch->exceptions.erase(batch->exceptions.begin() + i);
		}
	}
	std::erase_if(state->active, [](const auto& batch) { return batch->jobs.empty(); });
}

void WasmSyncedJobs::Reset()
{
	if (state->runningBatch)
		throw std::logic_error("Cannot reset Wasm jobs during a synchronous batch");
	state->Wait();
	state = std::make_unique<State>(state->mode);
	if (state->mode == Mode::Update) BeginFrame(0);
}

std::uint64_t WasmSyncedJobs::StateHash(void* owner) const
{
	// Hash only immutable submission metadata, NEVER readiness or worker output.
	// IDs/frame values are encoded explicitly little endian for portability.
	std::uint64_t hash = 0;
	const auto addWord = [&hash](std::uint32_t value) {
		const std::uint8_t bytes[] = {std::uint8_t(value), std::uint8_t(value >> 8),
			std::uint8_t(value >> 16), std::uint8_t(value >> 24)};
		hash = XXH3_64bits_withSeed(bytes, sizeof(bytes), hash);
	};
	addWord(state->nextId);
	// Completed synchronous work leaves only deterministic admission counters.
	// Keep the original checksum unchanged for users of the deferred API alone.
	if (state->batchJobs != 0) {
		addWord(0x42415443);
		addWord(state->batchJobs);
		addWord(state->batchInputBytes);
		addWord(state->batchOutputBytes);
	}
	const auto addJob = [&](const auto& job) {
		if (job.owner != owner) return;
		addWord(job.id);
		addWord(job.frame);
		hash = XXH3_64bits_withSeed(job.program->Identity().data(), job.program->Identity().size(), hash);
		addWord(job.exportName.size());
		hash = XXH3_64bits_withSeed(job.exportName.data(), job.exportName.size(), hash);
		addWord(job.input.size());
		addWord(static_cast<std::uint32_t>(job.inputHash));
		addWord(static_cast<std::uint32_t>(job.inputHash >> 32));
	};
	for (const auto& batch: state->active)
		for (const auto& job: batch->jobs) addJob(job);
	for (const auto& job: state->pending) addJob(job);
	return hash;
}
