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
	if (!state->accepting || state->frame < 0 || owner == nullptr || !program || exportName.empty() || exportName.size() > 256 ||
		state->nextId == 0 || state->pending.size() >= MAX_JOBS_PER_FRAME ||
		input.size() > MAX_INPUT_BYTES || input.size() > MAX_FRAME_INPUT_BYTES - state->inputBytes)
		return 0;
	const auto id = state->nextId++;
	ZoneValue(id);
	if (ZoneIsActive)
		ZoneTextF("frame=%d job=%u export=%s input=%zu", state->frame, id, exportName.c_str(), input.size());
	state->pending.push_back({owner, id, state->frame, std::move(program),
		std::move(exportName), {input.begin(), input.end()}
#if defined(TRACY_ENABLE)
		, profileOwner
#endif
	});
	state->inputBytes += input.size();
	return id;
}

std::vector<WasmSyncedJobs::Delivery> WasmSyncedJobs::BeginFrame(int frame)
{
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
	return deliveries;
}

bool WasmSyncedJobs::Accepting() const { return state->accepting; }

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
	state->accepting = true;
}

void WasmSyncedJobs::EndFrame()
{
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
	const auto addJob = [&](const auto& job) {
		if (job.owner != owner) return;
		addWord(job.id);
		addWord(job.frame);
		hash = XXH3_64bits_withSeed(job.program->Identity().data(), job.program->Identity().size(), hash);
		addWord(job.exportName.size());
		hash = XXH3_64bits_withSeed(job.exportName.data(), job.exportName.size(), hash);
		addWord(job.input.size());
		hash = XXH3_64bits_withSeed(job.input.data(), job.input.size(), hash);
	};
	if (state->active)
		for (const auto& job: state->active->jobs) addJob(job);
	for (const auto& job: state->pending) addJob(job);
	return hash;
}
