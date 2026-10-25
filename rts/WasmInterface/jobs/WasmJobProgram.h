/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <memory>
#include <span>
#include <string>
#include <string_view>
#include <vector>

class WasmRuntime;

// Status values are part of the guest ABI. Error text and timings are local
// diagnostics and never become synced results.
enum class WasmJobStatus : std::uint32_t {
	Ok = 0,
	InvalidExport = 1,
	Trapped = 2,
	InputTooLarge = 3,
	InvalidOutput = 4,
	RuntimeFailure = 5,
};

struct WasmJobResult {
	WasmJobStatus status = WasmJobStatus::Ok;
	std::vector<std::uint8_t> bytes;
	std::string diagnostic;
};

struct WasmJobTiming {
	std::uint64_t instantiateNs = 0;
	std::uint64_t inputCopyNs = 0;
	std::uint64_t dispatchNs = 0;
	std::uint64_t outputCopyNs = 0;
};

class WasmJobProgram {
public:
	// Compile once on the sim thread. Workers share immutable compiled code,
	// and each Run creates and destroys its own store/instance/private memory.
	static std::shared_ptr<WasmJobProgram> Compile(const WasmRuntime& runtime,
		const std::vector<std::uint8_t>& bytes, std::string& error);
	// `engine` is the one the module was compiled with (the runtime's by default).
	static std::shared_ptr<WasmJobProgram> FromCompiled(const WasmRuntime& runtime,
		void* compiledModule, std::string identity, void* engine = nullptr);
	~WasmJobProgram();
	const std::string& Identity() const;
	WasmJobResult Run(std::string_view exportName, std::span<const std::uint8_t> input,
		WasmJobTiming* timing = nullptr, std::size_t maxOutputBytes = 1024 * 1024) const;

private:
	struct Backend;
	explicit WasmJobProgram(std::unique_ptr<Backend> backend);
	std::unique_ptr<Backend> backend;
};
