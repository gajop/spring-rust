/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstddef>
#include <cstdint>
#include <string>
#include <string_view>
#include <memory>
#include <vector>

#include "WasmEnvironment.h"

class WasmCoreHost;

// The runtime version is part of synced module identity/configuration.
inline constexpr std::string_view RECOIL_WASMTIME_VERSION = "42.0.1";
inline constexpr std::string_view RECOIL_WASM_INTERFACE_PREFIX =
	"recoil:spring-api/";
inline constexpr std::string_view RECOIL_WASM_INTERFACE_VERSION = "@1.0.0";
inline constexpr std::string_view RECOIL_WASM_INTERFACE_VERSION_NUMBER = "1.0.0";

enum class WasmRuntimeBackend : std::uint8_t {
	Unavailable,
	WasmtimeComponentModel,
};

struct WasmRuntimeConfig {
	std::size_t maxModuleBytes = 64u * 1024u * 1024u;
	// Linear memory ceiling per module in 64 KiB pages (8192 = 512 MiB). The
	// engine sets it from gamedata/modrules.lua's system.WasmMemoryLimit.
	std::uint32_t maxMemoryPages = 8192;
	std::uint32_t maxTableElements = 1u << 20;
	std::uint32_t maxResources = 1u << 16;
	// A complete generated Core API context can legitimately import several
	// hundred semantic callouts.  Keep headroom for the full inventory while
	// retaining a fixed validation bound for untrusted modules.
	std::uint32_t maxImports = 2048;
	std::uint32_t maxExports = 256;
	std::uint32_t maxSections = 128;
	// Fixed validation guard, not a game/runtime setting: changing it requires
	// an engine build rather than silently changing synced peer configuration.
	static constexpr std::size_t maxFunctionBodyBytes = 16u * 1024u * 1024u;
	std::uint32_t maxComponentNesting = 16;
	std::uint32_t maxValueNodes = 1u << 20;
	// Zero disables Wasmtime's instruction counter. A positive value is a
	// deterministic per-module fuel budget selected by the game configuration.
	std::uint64_t instructionFuel = 0;
	// Zero disables the host-call work counter. A positive value remains
	// available to games that want an explicit deterministic host-work cap.
	std::uint64_t hostWorkLimit = 10'000'000;
	std::size_t resultBytesLimit = 16u * 1024u * 1024u;
	bool allowThreads = false;
	// Fixed-width SIMD (simd128). Its lane arithmetic is IEEE-exact and NaN
	// payloads are canonicalized, so it is deterministic; a game opts in with
	// modrules system.WasmSimd. Off, a module using SIMD fails validation.
	bool allowSimd = false;
	// Relaxed SIMD has implementation-defined results; synced games keep it off.
	bool allowRelaxedSimd = false;
	bool allowWasi = false;
	bool allowAotDeserialization = false;
#if defined(UNIT_TEST)
	// Synthetic Component Model fixtures exercise value lowering with a small
	// interface that is intentionally not part of the production callout
	// inventory. This escape hatch exists only in unit-test builds; production
	// validation always requires every imported function to be generated.
	bool allowUnregisteredComponentFunctionsForTesting = false;
#endif
};

// How synced guest float code gets architecture-independent NaN bits.
// SPRING_WASM_NAN_CANONICALIZATION selects it: unset or "sinks", "all" (or "1"),
// "none" (or "0"). Peers must agree; it is part of ConfigurationIdentity().
enum class WasmNanCanonicalization : std::uint8_t {
	// Synced Core modules are rewritten to canonicalize a NaN only where its
	// bits become observable (WasmNanSinks.h). A module the rewrite does not
	// model compiles with All instead.
	Sinks,
	// Cranelift canonicalizes the result of every float operation.
	All,
	// Diagnostic only, never for a match: NaN bits may differ between x86 and ARM.
	None,
};

struct WasmModuleIdentity {
	std::string sha512;
	std::size_t byteSize = 0;
};

struct WasmValidationResult {
	bool valid = false;
	WasmModuleIdentity identity;
	std::string error;
	std::vector<std::string> imports;
};

class WasmRuntime {
public:
	explicit WasmRuntime(WasmRuntimeConfig config = {});
	~WasmRuntime();

	WasmRuntime(const WasmRuntime&) = delete;
	WasmRuntime& operator=(const WasmRuntime&) = delete;

	const WasmRuntimeConfig& Config() const { return config; }
	WasmNanCanonicalization NanCanonicalization() const { return nanCanonicalization; }
	WasmRuntimeBackend Backend() const { return backend; }
	bool IsAvailable() const { return backend != WasmRuntimeBackend::Unavailable; }

	WasmValidationResult ValidateModule(const std::vector<std::uint8_t>& bytes,
		WasmEnvironment environment, std::string_view requestedWorld,
		std::string_view requestedInterfaceVersion = RECOIL_WASM_INTERFACE_VERSION_NUMBER) const;
	// Stable text identity for the runtime feature/limit profile.  Synced
	// module configuration includes this alongside the module hash.
	std::string ConfigurationIdentity() const;

	// AOT artifacts are native executable code. They are deliberately rejected
	// unless a future host-owned, authenticated cache implementation enables the
	// flag and validates its exact runtime/compiler identity.
	bool CanDeserializeAot(std::string_view moduleHash, std::string_view runtimeConfigHash) const;

private:
	friend class WasmCoreHost;
	// Local C++ microbenchmark uses this same engine/config for its inline baseline.
	friend class WasmJobBenchmark;
	friend class WasmJobProgram;

	struct BackendState;
	void* BackendEngine() const;
	// The bytes and engine a synced Core module compiles with. Leaves `rewritten`
	// empty when the original bytes are compiled. `sha512` is the module's
	// identity, which keys a cache of rewrites. `note` describes the rewrite;
	// `fallback` is set when the module compiles with canonicalization everywhere.
	void* PrepareSyncedModule(const std::vector<std::uint8_t>& original, const std::string& sha512,
		std::shared_ptr<const std::vector<std::uint8_t>>& rewritten, std::string& note,
		bool& fallback) const;

	WasmRuntimeConfig config;
	WasmRuntimeBackend backend = WasmRuntimeBackend::Unavailable;
	WasmNanCanonicalization nanCanonicalization = WasmNanCanonicalization::Sinks;
	std::unique_ptr<BackendState> backendState;
};
