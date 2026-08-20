/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmResources.h"

#include <algorithm>
#include <limits>

WasmHandle WasmResourceTable::Insert(WasmInstanceID owner, std::string family)
{
	std::size_t slot = entries.size();
	for (std::size_t index = 0; index < occupied.size(); ++index) {
		if (!occupied[index] && generations[index] != std::numeric_limits<std::uint32_t>::max()) {
			slot = index;
			break;
		}
	}
	if (slot == occupied.size()) {
		if (entries.size() >= maxEntries)
			return 0;
		occupied.push_back(false);
		entries.emplace_back();
		generations.push_back(0);
	}

	std::uint32_t& generation = generations[slot];
	if (generation == std::numeric_limits<std::uint32_t>::max())
		return 0;
	generation = std::max<std::uint32_t>(1, generation + 1);
	entries[slot] = Entry{owner, std::move(family), generation};
	occupied[slot] = true;
	return (static_cast<WasmHandle>(generation) << 32) |
		static_cast<WasmHandle>(slot);
}

bool WasmResourceTable::SetLimit(std::size_t newMaxEntries)
{
	if (newMaxEntries < Size())
		return false;
	maxEntries = newMaxEntries;
	return true;
}

bool WasmResourceTable::Validate(WasmHandle handle, WasmInstanceID owner,
	const std::string& family) const
{
	const std::size_t slot = static_cast<std::size_t>(handle & 0xffff'ffffu);
	const auto generation = static_cast<std::uint32_t>(handle >> 32);
	if (slot >= entries.size() || !occupied[slot])
		return false;
	const Entry& entry = entries[slot];
	return entry.owner == owner && entry.family == family && entry.generation == generation;
}

bool WasmResourceTable::Drop(WasmHandle handle, WasmInstanceID owner, const std::string& family)
{
	if (!Validate(handle, owner, family))
		return false;
	occupied[static_cast<std::size_t>(handle & 0xffff'ffffu)] = false;
	return true;
}

void WasmResourceTable::Clear()
{
	std::fill(occupied.begin(), occupied.end(), false);
}

std::size_t WasmResourceTable::Size() const
{
	return static_cast<std::size_t>(std::count(occupied.begin(), occupied.end(), true));
}

WasmCallbackID WasmCallbackRegistry::Register(WasmCallbackPolicy policy, Callback callback)
{
	if (nextID == std::numeric_limits<WasmCallbackID>::max())
		return 0;
	++nextID;
	if (nextID == 0)
		return 0;
	callbacks.emplace(nextID, Entry{policy, std::move(callback)});
	return nextID;
}

bool WasmCallbackRegistry::IsReentrant(WasmCallbackID id) const
{
	const auto iter = callbacks.find(id);
	return iter != callbacks.end() && iter->second.policy.reentrant;
}

bool WasmCallbackRegistry::Invoke(WasmCallbackID id, const std::vector<std::uint64_t>& arguments,
	bool currentlyInHostCall, bool& reentryAllowed)
{
	const auto iter = callbacks.find(id);
	if (iter == callbacks.end())
		return false;
	if (currentlyInHostCall && !iter->second.policy.reentrant)
		return false;
	reentryAllowed = iter->second.policy.reentrant;
	return iter->second.callback(arguments);
}

bool WasmCallbackRegistry::Drop(WasmCallbackID id)
{
	return callbacks.erase(id) != 0;
}

void WasmCallbackRegistry::Clear()
{
	callbacks.clear();
}

std::size_t WasmCallbackRegistry::Size() const
{
	return callbacks.size();
}

WasmExecutionBudget::WasmExecutionBudget(std::uint64_t instructionFuel,
	std::uint64_t hostWorkLimit, std::size_t resultBytesLimit)
{
	Reset(instructionFuel, hostWorkLimit, resultBytesLimit);
}

void WasmExecutionBudget::Reset(std::uint64_t newInstructionFuel,
	std::uint64_t newHostWorkLimit, std::size_t newResultBytesLimit)
{
	instructionFuel = newInstructionFuel;
	hostWork = 0;
	hostWorkLimit = newHostWorkLimit;
	resultBytesLimit = newResultBytesLimit;
	hostCallDepth = 0;
	callbackDepth = 0;
	nonReentrantCallbackDepth = 0;
	callbackReentry.clear();
	// Callback depth is hard-capped at MaxCallbackDepth, so reserve the whole
	// stack once off the callback hot path. EnterCallback must never need to
	// grow this vector while an engine -> guest -> host callback is in flight.
	callbackReentry.reserve(MaxCallbackDepth);
}

bool WasmExecutionBudget::ChargeGuest(std::uint64_t fuel)
{
	if (instructionFuel == 0)
		return true;
	if (fuel > instructionFuel)
		return false;
	instructionFuel -= fuel;
	return true;
}

bool WasmExecutionBudget::ChargeHost(std::uint64_t work)
{
	if (hostWorkLimit == 0)
		return true;
	if (work > std::numeric_limits<std::uint64_t>::max() - hostWork)
		return false;
	hostWork += work;
	return hostWork <= hostWorkLimit;
}

bool WasmExecutionBudget::CheckResultSize(std::size_t bytes) const
{
	return bytes <= resultBytesLimit;
}

bool WasmExecutionBudget::EnterImport(bool allowReentry)
{
	if ((hostCallDepth != 0 || callbackDepth != 0) && !allowReentry)
		return false;
	++hostCallDepth;
	return true;
}

void WasmExecutionBudget::LeaveImport()
{
	if (hostCallDepth != 0)
		--hostCallDepth;
}

bool WasmExecutionBudget::EnterCallback(bool reentrant)
{
	if (hostCallDepth != 0 && !reentrant)
		return false;
	if (callbackDepth >= MaxCallbackDepth)
		return false;
	++callbackDepth;
	callbackReentry.push_back(reentrant);
	if (!reentrant)
		++nonReentrantCallbackDepth;
	return true;
}

void WasmExecutionBudget::LeaveCallback()
{
	if (callbackDepth != 0) {
		--callbackDepth;
		if (!callbackReentry.empty()) {
			if (!callbackReentry.back() && nonReentrantCallbackDepth != 0)
				--nonReentrantCallbackDepth;
			callbackReentry.pop_back();
		}
	}
}

bool WasmExecutionBudget::CallbackReentryAllowed() const
{
	return callbackDepth == 0 || nonReentrantCallbackDepth == 0;
}
