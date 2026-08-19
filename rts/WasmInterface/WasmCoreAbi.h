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

// Low-level ABI helpers for the generated core-Wasm transport.
//
// This layer intentionally has no semantic WasmValue representation. Hot
// callouts/callins cross as core Wasm scalars plus explicitly validated linear
// memory. All signature validation is done once when functions are bound; the
// steady-state path uses Wasmtime's unchecked entry points.
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

// Fallible scalar results use one i64 slot. The low 32 bits contain the value
// and the high 32 bits contain the signed engine error code. This avoids an
// out-pointer and memory access for the most common scalar query shape.
constexpr std::uint64_t PackI32(std::int32_t value, std::int32_t status)
{
	return static_cast<std::uint64_t>(static_cast<std::uint32_t>(value)) |
		(static_cast<std::uint64_t>(static_cast<std::uint32_t>(status)) << 32);
}

constexpr std::int32_t UnpackI32Value(std::uint64_t packed)
{
	return static_cast<std::int32_t>(static_cast<std::uint32_t>(packed));
}

constexpr std::int32_t UnpackStatus(std::uint64_t packed)
{
	return static_cast<std::int32_t>(static_cast<std::uint32_t>(packed >> 32));
}

// Two f32 values fit in a single Core-Wasm i64 result. This is useful for hot
// callins such as UnitPreDamaged where returning through linear memory would be
// strictly more expensive and would add another bounds check/cache touch.
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
	}

	bool IsBound() const { return bound; }

	// Bind an exported memory lazily. This is mainly for imports executed from a
	// module start function before the host has had a chance to cache the memory
	// after instantiation. Normal steady-state calls never perform this lookup.
	bool BindFromCaller(wasmtime_caller_t* caller, std::string& error);
	bool BindFromInstance(wasmtime_context_t* context, const wasmtime_instance_t& instance,
		std::string& error);

	std::size_t Size() const;
	bool Read(std::uint32_t offset, void* destination, std::size_t bytes) const;
	bool Write(std::uint32_t offset, const void* source, std::size_t bytes) const;

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

	wasmtime_context_t* storeContext = nullptr;
	wasmtime_memory_t linearMemory{};
	bool bound = false;
};

std::string ErrorMessage(wasmtime_error_t* error);
std::string TrapMessage(wasm_trap_t* trap);
wasm_functype_t* MakeFuncType(const wasm_valkind_t* params, std::size_t paramCount,
	const wasm_valkind_t* results, std::size_t resultCount);
bool FunctionHasSignature(wasmtime_context_t* context, const wasmtime_func_t& function,
	const wasm_valkind_t* params, std::size_t paramCount,
	const wasm_valkind_t* results, std::size_t resultCount);

// Generic cached export used by generated callin bindings. Resolve performs
// the export lookup and exact type check once. Call performs no reflection and
// no allocation; the caller provides the fixed raw slot array on its stack.
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

// Convenience wrapper retained for the overwhelmingly common frame/event
// shape. Generated code can use RawExport directly for richer signatures.
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
