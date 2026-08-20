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
	std::uint32_t maxMemoryPages = 1024;
	std::uint32_t maxTableElements = 1u << 20;
	std::uint32_t maxResources = 1u << 16;
	std::uint32_t maxImports = 256;
	std::uint32_t maxExports = 256;
	std::uint32_t maxSections = 128;
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
	friend class WasmModule;
	friend class WasmCoreHost;

	struct BackendState;
	void* BackendEngine() const;

	WasmRuntimeConfig config;
	WasmRuntimeBackend backend = WasmRuntimeBackend::Unavailable;
	std::unique_ptr<BackendState> backendState;
};
