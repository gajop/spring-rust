/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <bit>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <span>
#include <string>
#include <type_traits>

#if defined(RECOIL_WASMTIME_AVAILABLE)
#include <wasmtime.h>
#endif

namespace recoil::wasm::core {

inline constexpr std::uint32_t ABI_VERSION = 1;
inline constexpr std::uint32_t POSITION_MID = 1u << 0;
inline constexpr std::uint32_t POSITION_AIM = 1u << 1;

enum class Status : std::int32_t {
	Ok = 0,
	InvalidArgument = 1,
	OutOfBounds = 2,
	NotFound = 3,
	NotAvailable = 4,
	InvalidState = 5,
	PermissionDenied = 6,
	AlreadyExists = 7,
	OperationFailed = 8,
	BufferOverflow = 9,
	InvalidId = 10,
	Internal = 999,
};

constexpr std::uint64_t PackU32(std::uint32_t value, std::int32_t status)
{
	return static_cast<std::uint64_t>(value) |
		(static_cast<std::uint64_t>(static_cast<std::uint32_t>(status)) << 32);
}

constexpr std::uint64_t PackI32(std::int32_t value, std::int32_t status)
{
	return PackU32(static_cast<std::uint32_t>(value), status);
}

constexpr std::int32_t UnpackI32Value(std::uint64_t packed)
{
	return static_cast<std::int32_t>(static_cast<std::uint32_t>(packed));
}

constexpr std::int32_t UnpackStatus(std::uint64_t packed)
{
	return static_cast<std::int32_t>(static_cast<std::uint32_t>(packed >> 32));
}

inline std::uint64_t PackF32Pair(float first, float second)
{
	const std::uint32_t low = std::bit_cast<std::uint32_t>(first);
	const std::uint32_t high = std::bit_cast<std::uint32_t>(second);
	return static_cast<std::uint64_t>(low) |
		(static_cast<std::uint64_t>(high) << 32);
}

inline void UnpackF32Pair(std::uint64_t packed, float& first, float& second)
{
	first = std::bit_cast<float>(static_cast<std::uint32_t>(packed));
	second = std::bit_cast<float>(static_cast<std::uint32_t>(packed >> 32));
}

#if defined(RECOIL_WASMTIME_AVAILABLE)

class Memory {
public:
	Memory() = default;

	void Bind(wasmtime_context_t* context, const wasmtime_memory_t& memory)
	{
		storeContext = context;
		linearMemory = memory;
		bound = true;
		cachedBase = wasmtime_memory_data(storeContext, &linearMemory);
		cachedSize = wasmtime_memory_data_size(storeContext, &linearMemory);
	}

	bool IsBound() const { return bound; }

	// Synced validation requires max == min, so the memory can never grow. Once
	// instantiated, the Wasmtime base pointer and byte length can therefore be
	// reused directly on every host call without querying the runtime again.
	// Do not enable this for a growable unsynced memory.
	void MarkStable() { stable = true; }
	bool IsStable() const { return stable; }

	bool BindFromCaller(wasmtime_caller_t* caller, std::string& error);
	bool BindFromInstance(wasmtime_context_t* context, const wasmtime_instance_t& instance,
		std::string& error);

	std::size_t Size() const;
	bool Contains(std::uint32_t offset, std::size_t bytes) const;
	bool Read(std::uint32_t offset, void* destination, std::size_t bytes) const;
	bool Write(std::uint32_t offset, const void* source, std::size_t bytes) const;
	bool ReadI32SliceLE(std::uint32_t offset, std::span<std::int32_t> values) const;
	bool WriteI32SliceLE(std::uint32_t offset, std::span<const std::int32_t> values) const;

	// Synchronous imports sometimes need to encode a variable record directly
	// into guest memory. Validate the range once, then let a wire codec operate
	// on the returned span without repeating a Wasmtime/bounds query per field.
	// The view must not escape the current host callback.
	bool MutableView(std::uint32_t offset, std::size_t bytes,
		std::span<std::uint8_t>& view) const
	{
		std::uint8_t* base = nullptr;
		if (!Range(offset, bytes, base))
			return false;
		view = std::span<std::uint8_t>(base, bytes);
		return true;
	}

	template<typename T>
	bool ReadPod(std::uint32_t offset, T& value) const
	{
		static_assert(std::is_trivially_copyable_v<T>);
		return Read(offset, &value, sizeof(T));
	}

	template<typename T>
	bool WritePod(std::uint32_t offset, const T& value) const
	{
		static_assert(std::is_trivially_copyable_v<T>);
		return Write(offset, &value, sizeof(T));
	}

private:
	bool Range(std::uint32_t offset, std::size_t bytes, std::uint8_t*& base) const;
	std::uint8_t* CurrentBase() const;
	std::size_t CurrentSize() const;

	wasmtime_context_t* storeContext = nullptr;
	wasmtime_memory_t linearMemory{};
	std::uint8_t* cachedBase = nullptr;
	std::size_t cachedSize = 0;
	bool bound = false;
	bool stable = false;
};

std::string ErrorMessage(wasmtime_error_t* error);
std::string TrapMessage(wasm_trap_t* trap);
wasm_functype_t* MakeFuncType(const wasm_valkind_t* params, std::size_t paramCount,
	const wasm_valkind_t* results, std::size_t resultCount);
bool FunctionHasSignature(wasmtime_context_t* context, const wasmtime_func_t& function,
	const wasm_valkind_t* params, std::size_t paramCount,
	const wasm_valkind_t* results, std::size_t resultCount);

class RawExport {
public:
	bool Resolve(wasmtime_context_t* context, const wasmtime_instance_t& instance,
		const char* name, std::size_t nameLength,
		std::span<const wasm_valkind_t> params,
		std::span<const wasm_valkind_t> results,
		bool optional, std::string& error);
	bool Call(wasmtime_context_t* context, wasmtime_val_raw_t* slots,
		std::size_t slotCount, std::string& error) const;

	bool Present() const { return present; }
	std::size_t SlotCount() const { return slotCount; }

private:
	wasmtime_func_t function{};
	std::size_t slotCount = 0;
	bool present = false;
};

class I32ToVoidExport {
public:
	bool Resolve(wasmtime_context_t* context, const wasmtime_instance_t& instance,
		const char* name, std::size_t nameLength, bool optional, std::string& error);
	bool Call(wasmtime_context_t* context, std::int32_t value, std::string& error) const;
	bool Present() const { return raw.Present(); }

private:
	RawExport raw;
};

#endif

} // namespace recoil::wasm::core
