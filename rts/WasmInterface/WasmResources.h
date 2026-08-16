/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <functional>
#include <map>
#include <string>
#include <vector>

using WasmInstanceID = std::uint64_t;
using WasmHandle = std::uint64_t;
using WasmCallbackID = std::uint32_t;

class WasmResourceTable {
public:
	explicit WasmResourceTable(std::size_t maxEntries = 1u << 16)
		: maxEntries(maxEntries)
	{
	}

	WasmHandle Insert(WasmInstanceID owner, std::string family);
	bool SetLimit(std::size_t newMaxEntries);
	bool Validate(WasmHandle handle, WasmInstanceID owner, const std::string& family) const;
	bool Drop(WasmHandle handle, WasmInstanceID owner, const std::string& family);
	void Clear();
	std::size_t Size() const;

private:
	struct Entry {
		WasmInstanceID owner = 0;
		std::string family;
		std::uint32_t generation = 0;
	};

	std::vector<Entry> entries;
	std::vector<std::uint32_t> generations;
	std::vector<bool> occupied;
	std::size_t maxEntries;
};

struct WasmCallbackPolicy {
	bool reentrant = false;
};

class WasmCallbackRegistry {
public:
	using Callback = std::function<bool(const std::vector<std::uint64_t>&)>;

	WasmCallbackID Register(WasmCallbackPolicy policy, Callback callback);
	bool IsReentrant(WasmCallbackID id) const;
	bool Invoke(WasmCallbackID id, const std::vector<std::uint64_t>& arguments,
		bool currentlyInHostCall, bool& reentryAllowed);
	bool Drop(WasmCallbackID id);
	void Clear();
	std::size_t Size() const;

private:
	struct Entry {
		WasmCallbackPolicy policy;
		Callback callback;
	};

	WasmCallbackID nextID = 0;
	std::map<WasmCallbackID, Entry> callbacks;
};

class WasmExecutionBudget {
public:
	WasmExecutionBudget(std::uint64_t instructionFuel, std::uint64_t hostWorkLimit,
		std::size_t resultBytesLimit);
	void Reset(std::uint64_t instructionFuel, std::uint64_t hostWorkLimit,
		std::size_t resultBytesLimit);

	bool ChargeGuest(std::uint64_t fuel);
	bool ChargeHost(std::uint64_t work);
	bool CheckResultSize(std::size_t bytes) const;
	bool EnterImport(bool allowReentry);
	void LeaveImport();
	bool EnterCallback(bool reentrant);
	void LeaveCallback();
	bool CallbackReentryAllowed() const;

	std::uint64_t InstructionFuel() const { return instructionFuel; }
	std::uint64_t HostWork() const { return hostWork; }
	std::uint32_t CallbackDepth() const { return callbackDepth; }

private:
	std::uint64_t instructionFuel;
	std::uint64_t hostWork = 0;
	std::uint64_t hostWorkLimit;
	std::size_t resultBytesLimit;
	std::uint32_t hostCallDepth = 0;
	std::uint32_t callbackDepth = 0;
	std::uint32_t nonReentrantCallbackDepth = 0;
	std::vector<bool> callbackReentry;
	static constexpr std::uint32_t MaxCallbackDepth = 32;
};
