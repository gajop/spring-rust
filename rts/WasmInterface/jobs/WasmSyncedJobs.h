/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <exception>
#include <memory>
#include <span>
#include <string>
#include <vector>

#include "WasmJobProgram.h"
#include "WasmJobBatch.h"

// Owned and mutated exclusively by the sim thread. Background work reads an
// immutable sealed batch and writes one private result slot per submission.
class WasmSyncedJobs {
public:
	enum class Mode { Synced, Update };
	static constexpr std::size_t MAX_JOBS_PER_FRAME = WasmJobBatch::MAX_JOBS;
	static constexpr std::size_t MAX_INPUT_BYTES = WasmJobBatch::MAX_BYTES;
	static constexpr std::size_t MAX_FRAME_INPUT_BYTES = WasmJobBatch::MAX_TOTAL_BYTES;
	struct Delivery {
		void* owner;
		std::uint32_t id;
		int submittedFrame;
		WasmJobResult result;
	};

	explicit WasmSyncedJobs(Mode mode = Mode::Synced);
	~WasmSyncedJobs();
	// Returns zero for a bounded admission rejection; never depends on worker
	// completion, queue readiness, wall time or worker count.
	std::uint32_t Submit(void* owner, std::shared_ptr<const WasmJobProgram> program,
		std::string exportName, std::span<const std::uint8_t> input
#if defined(TRACY_ENABLE)
		// Immutable owner name, valid until this queue drains; not synced data.
		, std::string_view profileOwner = "native"
#endif
	);
	std::vector<Delivery> BeginFrame(int frame);
	void EndFrame();
	// Local update queues stay open between Update calls (input/draw/init may
	// submit). EndUpdate seals that window and opens the following window.
	std::vector<Delivery> BeginUpdate();
	void EndUpdate();
	bool Accepting() const;
	enum class BatchStatus { Ok, InvalidState, InvalidArgument, BufferOverflow };
	// Same-frame owned results, with no callback and no interaction with active
	// next-frame deliveries. Admission shares this window's job/input quotas.
	BatchStatus RunBatch(void* owner, std::shared_ptr<const WasmJobProgram> program,
		std::string exportName, std::vector<WasmJobBatchInput> inputs,
		std::vector<WasmJobResult>& results);
	// A C-ABI import must unwind by returning a trap, not a C++ exception.
	// Retain host failures so the native frame boundary still terminates the
	// simulation instead of accepting a locally faulted/missing rules module.
	void FailBatch(std::exception_ptr failure);
	void CancelOwner(void* owner);
	void Reset();
	std::uint64_t StateHash(void* owner) const;

private:
	struct State;
	std::unique_ptr<State> state;
};
