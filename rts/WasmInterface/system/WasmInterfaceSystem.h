/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <array>
#include <cstdint>
#include <functional>
#include <memory>
#include <span>
#include <string>
#include <string_view>
#include <vector>

#include "WasmEnvironment.h"
#include "WasmInterface/core/host/WasmCoreCallinPolicy.h"
#include "WasmModuleManifest.h"
#include "WasmRuntime.h"

struct NativeInterface;
class WasmCoreHost;
enum class WasmCoreCallin : std::uint16_t;
namespace recoil::wasm::core { struct WasmCoreDispatchPlan; }

class WasmInterfaceSystem {
public:
	explicit WasmInterfaceSystem(NativeInterface* nativeInterface,
		WasmRuntimeConfig runtimeConfig = {});
	~WasmInterfaceSystem();

	WasmInterfaceSystem(const WasmInterfaceSystem&) = delete;
	WasmInterfaceSystem& operator=(const WasmInterfaceSystem&) = delete;

	bool LoadModule(WasmModuleDescriptor descriptor, std::string& error);
	using ModuleBytesProvider = std::function<bool(std::string_view path,
		std::vector<std::uint8_t>& bytes, std::string& error)>;
	bool LoadManifest(std::string_view manifest, const ModuleBytesProvider& bytesProvider,
		std::string& error);
	using ArchiveModuleBytesProvider = std::function<bool(std::string_view archive,
		std::string_view path, std::vector<std::uint8_t>& bytes, std::string& error)>;
	bool LoadManifests(const std::vector<WasmManifestSource>& sources,
		const ArchiveModuleBytesProvider& bytesProvider, std::string& error);
	bool UnloadModule(std::string_view moduleName);
	void UnloadAll();
	void Update();

	// Dispatch is keyed by the numeric callin id, which every engine call site
	// knows at compile time. No layer below this point looks at an event name
	// except to build a diagnostic message.
	static bool DispatchActiveCoreCallin(WasmCoreCallin callin, const void* query,
		bool synced, void* nativeResult, bool& handled, std::string& error);
	struct CoreCallinInvocation {
		WasmEnvironment environment;
		const void* query = nullptr;
		bool contributesResult = true;
	};
	bool DispatchCoreCallin(WasmCoreCallin callin,
		std::span<const CoreCallinInvocation> invocations, void* nativeResult,
		bool& handled, std::string& error);
	bool DispatchSyncedMessage(std::string_view message, std::string& error);

	std::size_t ModuleCount() const;
	bool HasModules(WasmEnvironment environment) const;
	bool HasCoreModules(WasmEnvironment environment) const;
	const WasmRuntime& Runtime() const { return *runtime; }
	std::vector<std::string> SyncedConfiguration() const;

private:
	struct CoreModuleRecord {
		WasmModuleDescriptor descriptor;
		WasmModuleIdentity identity;
		WasmCoreHost* host = nullptr;
	};

	// Which guests receive a given callin changes only when a module is loaded,
	// unloaded or dropped after a fault. Resolving it per dispatch meant walking
	// every module for every event; it is precomputed here instead and rebuilt
	// on those three lifecycle points.
	class CoreSubscriberIndex {
	public:
		static constexpr std::size_t kEnvironmentCount =
			WasmEnvironmentMatrix::All().size();
		struct Range {
			std::uint32_t begin = 0;
			std::uint32_t count = 0;
		};

		// Every routing decision for one callin, in one cache line: which worlds
		// it can reach, how its results aggregate, whether the UI copy needs
		// visibility filtering, and where its dispatch plans live. A dispatch
		// reads this and then the plans; it consults no other table.
		struct alignas(64) CallinRoute {
			std::uint32_t environmentMask = 0;
			recoil::wasm::core::CoreAggregation aggregation =
				recoil::wasm::core::CoreAggregation::Unsupported;
			recoil::wasm::core::CoreResultKind resultKind =
				recoil::wasm::core::CoreResultKind::Unsupported;
			bool valid = false;
			bool uiNeedsFilter = false;
			bool uiContributesResult = true;
			Range perEnvironment[kEnvironmentCount];
		};

		void Rebuild(const std::vector<CoreModuleRecord>& modules);
		const CallinRoute& Route(WasmCoreCallin callin) const;
		// Entries are resolved dispatch plans, not module handles: a dispatch
		// reads the plan and calls the guest, with no lookup in between.
		std::span<const recoil::wasm::core::WasmCoreDispatchPlan* const> Plans(
			const CallinRoute& route, WasmEnvironment environment) const;

	private:
		std::vector<const recoil::wasm::core::WasmCoreDispatchPlan*> plans;
		// Held inline rather than behind a vector so reaching a route is one
		// load, not a load of the vector header followed by a load of the data.
		std::array<CallinRoute, recoil::wasm::core::CORE_CALLIN_COUNT> routes{};
	};

	const CoreSubscriberIndex& Subscribers();
	void InvalidateSubscribers() { coreSubscribersDirty = true; }
	bool DispatchIgnoredCallin(WasmCoreCallin callin,
		const CoreSubscriberIndex::CallinRoute& route, const void* query,
		std::uint32_t reachable, std::span<const WasmEnvironment> primary,
		bool& handled, std::string& error);
	bool ResetBudgetWindow(bool synced, std::string& error);
	void RemoveFaultedUnsyncedModules();

	class CoreDispatchRegistration {
	public:
		explicit CoreDispatchRegistration(WasmInterfaceSystem* owner);
		~CoreDispatchRegistration();
		CoreDispatchRegistration(const CoreDispatchRegistration&) = delete;
		CoreDispatchRegistration& operator=(const CoreDispatchRegistration&) = delete;
	private:
		WasmInterfaceSystem* owner = nullptr;
		WasmInterfaceSystem* previous = nullptr;
	};

	std::unique_ptr<WasmRuntime> runtime;
	NativeInterface* nativeInterface = nullptr;
	std::vector<CoreModuleRecord> coreModules;
	CoreSubscriberIndex coreSubscribers;
	bool coreSubscribersDirty = true;
	CoreDispatchRegistration coreDispatchRegistration{this};
};
