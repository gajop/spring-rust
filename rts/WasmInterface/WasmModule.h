/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <functional>
#include <memory>
#include <string>
#include <string_view>
#include <vector>

#include "WasmEnvironment.h"
#include "WasmHost.h"
#include "WasmResources.h"
#include "WasmRuntime.h"

struct WasmModuleDescriptor {
	std::string name;
	std::string source;
	WasmEnvironment environment = WasmEnvironment::RulesSynced;
	std::uint32_t order = 0;
	// Component interface version declared by the content package.  A missing
	// manifest field is normalized to the current host version by the parser.
	std::string interfaceVersion = std::string(RECOIL_WASM_INTERFACE_VERSION_NUMBER);
	std::vector<std::uint8_t> bytes;
	// Content archive that declared the module. Empty means the declaration
	// came from the legacy single-source API.
	std::string archive;
};

enum class WasmModuleState : std::uint8_t {
	Created,
	Validated,
	Running,
	Faulted,
	Stopped,
};

// Callback trampolines may outlive a module's native subscription.  The
// lifetime token lets a late native callback become a no-op without
// dereferencing a destroyed WasmModule.
struct WasmCallbackLifetime {
	WasmModule* module = nullptr;
	bool active = false;
};

class WasmModule {
public:
	WasmModule(WasmInstanceID instanceID, WasmModuleDescriptor descriptor,
		const WasmRuntime& runtime, WasmHostAdapter* hostAdapter = nullptr);
	~WasmModule();

	bool Initialize(std::string& error);
	void Shutdown();

	WasmInstanceID InstanceID() const { return instanceID; }
	const WasmModuleDescriptor& Descriptor() const { return descriptor; }
	const WasmModuleIdentity& Identity() const { return identity; }
	WasmModuleState State() const { return state; }
	const std::string& FaultReason() const { return faultReason; }

	// Wasmtime execution stays behind this seam so the engine-facing adapter can
	// enforce the policy and resource rules before crossing into guest code.
	bool Callout(std::string_view interfaceName, std::string_view functionName,
		std::string& error);
	bool Callin(std::string_view name, const std::vector<std::uint64_t>& arguments,
		std::string& error);
	bool Callin(std::string_view name, const std::vector<std::uint64_t>& arguments,
		std::vector<std::uint64_t>& results, std::string& error);
	// Semantic Component Model callin path used by generated callin worlds.
	// `name` may be a slash-separated export path such as
	// `callins-rules-synced/game-frame`.
	bool Callin(std::string_view name, const std::vector<WasmValue>& arguments,
		WasmValue& result, std::string& error);

	// `resolved`: adapter cookie from ResolveCallout, or nullptr to resolve by name.
	bool InvokeCallout(std::string_view module, std::string_view function,
		const std::vector<WasmValue>& arguments, WasmValue& result, std::string& error,
		bool keepImportEntered = false, const void* resolved = nullptr);
	const void* ResolveCallout(std::string_view module, std::string_view function) const;
	// Component host callbacks may return a value whose canonical lowering runs
	// guest code after the C++ callback returns. Keep the import guard alive
	// until the enclosing component call has returned from Wasmtime.
	std::size_t ImportGuardCheckpoint() const { return deferredImportLeaves; }
	void ReleaseDeferredImportGuards(std::size_t checkpoint);

	WasmResourceTable& Resources() { return resources; }
	WasmCallbackRegistry& Callbacks() { return callbacks; }
	WasmExecutionBudget& Budget() { return budget; }
	const WasmRuntime& Runtime() const { return runtime; }
	std::shared_ptr<WasmCallbackLifetime> CallbackLifetime() const { return callbackLifetime; }
	WasmCallbackID RegisterGuestCallback(std::uint32_t guestCallbackID,
		WasmCallbackPolicy policy, std::string& error);
	bool InvokeGuestCallback(WasmCallbackID callbackID,
		const std::vector<std::uint64_t>& arguments, std::string& error);
	void DropGuestCallback(WasmCallbackID callbackID);
	// Component resource values are represented outside the public semantic
	// value as instance-owned handles. The raw pointers are accepted only at
	// the Wasmtime boundary and never escape this module.
	bool ImportComponentResource(void* context, void* resource, bool owned,
		WasmValue& output, std::string& error);
	bool ExportComponentResource(void* context, const WasmValueResource& value,
		bool transferOwnership, void*& resource, std::string& error,
		std::vector<WasmHandle>* pendingTransfers = nullptr);
	bool CommitComponentResourceTransfers(const std::vector<WasmHandle>& transfers,
		std::string& error);
	bool DropComponentResource(void* context, const WasmValueResource& value,
		std::string& error);
	// Register host-owned cleanup that must run after guest callbacks are
	// disabled and before the instance backend is released.  This is used for
	// long-lived engine subscriptions such as RmlUi contexts.
	bool RegisterCleanup(std::function<void()> cleanup);

	void Fault(std::string reason);

private:
	struct BackendState;
	void ClearComponentResources();

	WasmInstanceID instanceID;
	WasmModuleDescriptor descriptor;
	const WasmRuntime& runtime;
	WasmHostAdapter* hostAdapter = nullptr;
	WasmModuleIdentity identity;
	WasmModuleState state = WasmModuleState::Created;
	std::string faultReason;
	WasmResourceTable resources;
	WasmCallbackRegistry callbacks;
	WasmExecutionBudget budget;
	std::shared_ptr<WasmCallbackLifetime> callbackLifetime;
	std::vector<std::function<void()>> cleanupCallbacks;
	std::size_t deferredImportLeaves = 0;
	std::unique_ptr<BackendState> backendState;
};
