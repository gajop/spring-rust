/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <map>
#include <memory>
#include <string>
#include <string_view>
#include <utility>
#include <variant>
#include <vector>

// Owned values used at the C++ side of the generated Wasm host adapter. No
// pointer into a Wasm store or the NativeInterface scratch area is retained.
struct WasmValue;
using WasmValueList = std::vector<WasmValue>;
// Transparent comparator so a field lookup can take a string_view without
// building a std::string first.
using WasmValueRecord = std::map<std::string, WasmValue, std::less<>>;

// Component variants/resources cannot be represented by a flat C++ union.
// Keep the semantic boundary owned and recursive while avoiding a raw
// Wasmtime pointer in values that may be retained by an adapter or callback.
struct WasmValueVariant {
	std::string discriminant;
	std::shared_ptr<WasmValue> value;

	bool HasValue() const { return value != nullptr; }
};

struct WasmValueResource {
	std::uint64_t handle = 0;
	std::string family;
	bool owned = false;
};

class WasmModule;

struct WasmValue {
	using Storage = std::variant<std::monostate, bool, std::int64_t, std::uint64_t,
		double, std::string, std::vector<std::uint8_t>, WasmValueList, WasmValueRecord,
		WasmValueVariant, WasmValueResource>;

	Storage storage;

	WasmValue() = default;

	static WasmValue Unit() { return WasmValue{}; }
	static WasmValue Bool(bool value) { return WasmValue{value}; }
	static WasmValue I64(std::int64_t value) { return WasmValue{value}; }
	static WasmValue U64(std::uint64_t value) { return WasmValue{value}; }
	static WasmValue F64(double value) { return WasmValue{value}; }
	static WasmValue String(std::string value) { return WasmValue{std::move(value)}; }
	static WasmValue Bytes(std::vector<std::uint8_t> value) { return WasmValue{std::move(value)}; }
	static WasmValue List(WasmValueList value) { return WasmValue{std::move(value)}; }
	static WasmValue Record(WasmValueRecord value) { return WasmValue{std::move(value)}; }
	static WasmValue Variant(std::string discriminant, WasmValue value);
	static WasmValue EmptyVariant(std::string discriminant);
	static WasmValue Resource(std::uint64_t handle, std::string family, bool owned = false)
	{
		return WasmValue{WasmValueResource{handle, std::move(family), owned}};
	}

	bool IsUnit() const { return std::holds_alternative<std::monostate>(storage); }

private:
	template<typename T>
	explicit WasmValue(T value)
		: storage(std::move(value))
	{
	}
};

inline WasmValue WasmValue::Variant(std::string discriminant, WasmValue value)
{
	return WasmValue{WasmValueVariant{
		std::move(discriminant), std::make_shared<WasmValue>(std::move(value))}};
}

inline WasmValue WasmValue::EmptyVariant(std::string discriminant)
{
	return WasmValue{WasmValueVariant{std::move(discriminant), nullptr}};
}

// A host adapter is deliberately supplied by the engine-facing owner. This
// keeps Wasmtime independent from NativeInterface headers and makes the same
// WasmModule usable in focused runtime tests.
class WasmHostAdapter {
public:
	virtual ~WasmHostAdapter() = default;

	virtual bool Callout(std::string_view module, std::string_view function,
		const std::vector<WasmValue>& arguments, WasmValue& result,
		std::string& error) = 0;

	// The owner-aware overload is used by callback-capable adapters.  Keeping
	// the default implementation preserves the small fake adapters used by
	// focused runtime tests while allowing an engine adapter to register
	// instance-owned callback resources.
	virtual bool Callout(WasmModule& owner, std::string_view module,
		std::string_view function, const std::vector<WasmValue>& arguments,
		WasmValue& result, std::string& error)
	{
		return Callout(module, function, arguments, result, error);
	}

	// Resolve an import's target once at bind time; the cookie is opaque so this
	// header stays free of NativeInterface types.  nullptr means resolve by name.
	virtual const void* ResolveCallout(std::string_view module, std::string_view function)
	{
		(void)module;
		(void)function;
		return nullptr;
	}

	virtual bool Callout(WasmModule& owner, const void* resolved, std::string_view module,
		std::string_view function, const std::vector<WasmValue>& arguments,
		WasmValue& result, std::string& error)
	{
		(void)resolved;
		return Callout(owner, module, function, arguments, result, error);
	}

	// Opaque NativeInterface* for alternative hosts that reach the native API
	// without going through the WasmValue transport.  void* keeps this header
	// free of NativeInterface types; nullptr means the adapter has none, which
	// is the case for the small fakes used by focused runtime tests.
	virtual void* NativeInterfaceHandle() { return nullptr; }
};
