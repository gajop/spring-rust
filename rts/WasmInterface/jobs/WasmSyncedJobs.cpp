/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmSyncedJobs.h"

#include <algorithm>
#include <exception>
#include <functional>
#include <limits>
#include <stdexcept>

#include "System/SpringHash.h"
#include "System/Misc/TracyDefs.h"
#include "System/Threading/ThreadPool.h"

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
	std::shared_ptr<Batch> active;
	std::uint32_t nextId = 1;
	std::size_t inputBytes = 0;
	std::size_t batchJobs = 0;
	std::size_t batchInputBytes = 0;
	std::size_t batchOutputBytes = 0;
	bool runningBatch = false;
	std::exception_ptr batchFailure;
	int frame = -1;
	bool accepting = false;
#if defined(THREADPOOL)
	using Work = std::function<void(int)>;
	std::shared_ptr<ForBackgroundTaskGroup<Work>> background;
#endif

	void Wait()
	{
#if defined(THREADPOOL)
		if (background) {
			wait_for_mt_background(background);
			background.reset();
		}
#endif
	}
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
		state->nextId == 0 || state->pending.size() + state->batchJobs >= MAX_JOBS_PER_FRAME ||
		input.size() > MAX_INPUT_BYTES || input.size() > MAX_FRAME_INPUT_BYTES - state->inputBytes - state->batchInputBytes)
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
		ZoneNamedN(barrier, "Wasm::Jobs::Barrier", state->active != nullptr);
		ZoneValueV(barrier, frame);
		state->Wait();
	}
	ZoneNamedN(collect, "Wasm::Jobs::Collect", state->active != nullptr);
	ZoneValueV(collect, frame);
	if (!state->pending.empty() && (state->mode != Mode::Update ||
		std::any_of(state->pending.begin(), state->pending.end(), [frame](const auto& job) { return job.frame != frame; })))
		throw std::logic_error("Wasm job submissions were not sealed at the prior frame boundary");
	std::vector<Delivery> deliveries;
	if (state->active) {
		const auto& batch = *state->active;
		// Complete the whole barrier before publishing any result. Host resource
		// failure terminates simulation instead of silently becoming game state.
		for (std::size_t i = 0; i < batch.jobs.size(); ++i) {
			if (batch.jobs[i].frame + 1 != frame)
				throw std::logic_error("Wasm job missed its fixed next-frame delivery boundary");
			if (batch.exceptions[i])
				std::rethrow_exception(batch.exceptions[i]);
			if (batch.results[i].status == WasmJobStatus::RuntimeFailure)
				throw std::runtime_error("Wasm job host failure: " + batch.results[i].diagnostic);
		}
		for (std::size_t i = 0; i < batch.jobs.size(); ++i) {
			auto& job = state->active->jobs[i];
			deliveries.push_back({job.owner, job.id, job.frame, std::move(state->active->results[i])});
		}
		state->active.reset();
	}
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
	if (inputs.size() > MAX_JOBS_PER_FRAME - state->pending.size() - state->batchJobs)
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
	if (inputBytes > MAX_FRAME_INPUT_BYTES - state->inputBytes - state->batchInputBytes ||
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
	for_mt(0, inputs.size(), 1, [&](int index) {
		ZoneScopedN("Wasm::Jobs::BatchWorker");
		ZoneValue(index);
		try {
			completed[index] = program->Run(exportName, inputs[index].bytes, nullptr, inputs[index].maxOutputBytes);
		} catch (...) {
			exceptions[index] = std::current_exception();
		}
	});
	// The whole barrier completes before publishing even the first result.
	for (std::size_t i = 0; i < completed.size(); ++i) {
		if (exceptions[i]) std::rethrow_exception(exceptions[i]);
		if (completed[i].status == WasmJobStatus::RuntimeFailure)
			throw std::runtime_error("Wasm batch host failure: " + completed[i].diagnostic);
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
	if (state->active)
		throw std::logic_error("Prior Wasm job batch was not consumed");
	if (state->pending.empty())
		return;
	ZoneScopedN("Wasm::Jobs::Seal");
	ZoneValue(state->frame);
	if (ZoneIsActive)
		ZoneTextF("frame=%d jobs=%zu input=%zu threads=%d", state->frame,
		state->pending.size(), state->inputBytes, ThreadPool::GetNumThreads());
	auto batch = std::make_shared<State::Batch>();
	batch->jobs = std::move(state->pending);
	state->pending.clear();
	state->inputBytes = 0;
	batch->results.resize(batch->jobs.size());
	batch->exceptions.resize(batch->jobs.size());
	state->active = batch;
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
			batch->results[index] = job.program->Run(job.exportName, job.input);
		} catch (...) {
			batch->exceptions[index] = std::current_exception();
		}
		if (ZoneIsActive)
			ZoneTextF("status=%u output=%zu exception=%d", static_cast<unsigned>(batch->results[index].status),
				batch->results[index].bytes.size(), int(batch->exceptions[index] != nullptr));
	};
#if defined(THREADPOOL)
	state->background = for_mt_background(0, batch->jobs.size(), 1, std::move(work));
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
	if (state->active) {
		for (std::size_t i = state->active->jobs.size(); i-- > 0;) {
			if (state->active->jobs[i].owner != owner) continue;
			state->active->jobs.erase(state->active->jobs.begin() + i);
			state->active->results.erase(state->active->results.begin() + i);
			state->active->exceptions.erase(state->active->exceptions.begin() + i);
		}
	}
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
	if (state->active)
		for (const auto& job: state->active->jobs) addJob(job);
	for (const auto& job: state->pending) addJob(job);
	return hash;
}
