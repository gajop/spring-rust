/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */
#include "WasmCoreState.h"

#include <algorithm>
#include <bit>
#include <limits>
#include <span>
#include <stdexcept>

namespace recoil::wasm::core {
namespace {
struct Reader {
	std::span<const std::uint8_t> bytes;
	std::size_t pos = 0;
	std::uint8_t Byte() {
		if (pos == bytes.size()) throw std::runtime_error("truncated Wasm state metadata");
		return bytes[pos++];
	}
	std::uint32_t Leb() {
		std::uint32_t n = 0;
		for (unsigned shift = 0; shift < 35; shift += 7) {
			const auto b = Byte();
			if (shift == 28 && b > 15) break;
			n |= (b & 127u) << shift;
			if (!(b & 128u)) return n;
		}
		throw std::runtime_error("invalid Wasm state metadata integer");
	}
	std::span<const std::uint8_t> Take(std::size_t n) {
		if (n > bytes.size() - pos) throw std::runtime_error("truncated Wasm state metadata section");
		auto result = bytes.subspan(pos, n); pos += n; return result;
	}
	std::string String() {
		const auto n = Leb(); const auto s = Take(n);
		return {reinterpret_cast<const char*>(s.data()), s.size()};
	}
};
void Leb(std::vector<std::uint8_t>& out, std::uint32_t n) {
	do { auto b = static_cast<std::uint8_t>(n & 127); n >>= 7; out.push_back(b | (n ? 128 : 0)); } while (n);
}
void Number(std::vector<std::uint8_t>& out, std::uint64_t n, unsigned width) {
	for (unsigned i = 0; i < width; ++i) out.push_back(n >> (8 * i));
}
}

std::string StateExports::Name(char kind, std::uint32_t index) const
{
	return prefix + kind + std::to_string(index);
}

bool StateExports::Prepare(const std::vector<std::uint8_t>& original,
	std::vector<std::uint8_t>& compiled, StateExports& exports, std::string& error)
{
	try {
		if (original.size() < 8) throw std::runtime_error("missing Wasm state header");
		Reader module{original}; module.Take(8);
		std::vector<std::string> names;
		std::size_t exportStart = 0, exportEnd = 0;
		std::vector<std::uint8_t> oldEntries;
		std::uint32_t oldCount = 0;
		exports = {};
		while (module.pos < module.bytes.size()) {
			const auto start = module.pos;
			const auto id = module.Byte(); const auto size = module.Leb();
			Reader section{module.Take(size)};
			if (id == 2) {
				const auto count = section.Leb();
				for (std::uint32_t i = 0; i < count; ++i) {
					section.String(); section.String();
					if (section.Byte() != 0) throw std::runtime_error("state inspection requires Core function-only imports");
					section.Leb(); ++exports.functions;
				}
			} else if (id == 3) {
				const auto count = section.Leb();
				if (count > 65536) throw std::runtime_error("too many Wasm functions for state inspection");
				exports.functions += count;
			} else if (id == 4 || id == 6) {
				const auto count = section.Leb();
				if (count > 65536) throw std::runtime_error("too many Wasm state objects");
				if (id == 4) exports.tables = count; else exports.globals = count;
			} else if (id == 7) {
				if (exportStart) throw std::runtime_error("duplicate Wasm export section");
				exportStart = start; exportEnd = module.pos; oldCount = section.Leb();
				oldEntries.assign(section.bytes.begin() + section.pos, section.bytes.end());
				for (std::uint32_t i = 0; i < oldCount; ++i) {
					names.push_back(section.String()); section.Byte(); section.Leb();
				}
			}
		}
		if (!exportStart) throw std::runtime_error("state inspection needs the validated Core export section");
		for (unsigned nonce = 0;; ++nonce) {
			exports.prefix = "__spring_state_" + std::to_string(nonce) + "_";
			if (std::none_of(names.begin(), names.end(), [&](const auto& n) { return n.starts_with(exports.prefix); })) break;
		}
		std::vector<std::uint8_t> entries;
		Leb(entries, oldCount + exports.functions + exports.globals + exports.tables);
		entries.insert(entries.end(), oldEntries.begin(), oldEntries.end());
		const auto append = [&](char name, std::uint8_t kind, std::uint32_t count) {
			for (std::uint32_t i = 0; i < count; ++i) {
				const auto n = exports.Name(name, i); Leb(entries, n.size());
				entries.insert(entries.end(), n.begin(), n.end()); entries.push_back(kind); Leb(entries, i);
			}
		};
		append('f', 0, exports.functions); append('g', 3, exports.globals); append('t', 1, exports.tables);
		compiled.assign(original.begin(), original.begin() + exportStart);
		compiled.push_back(7); Leb(compiled, entries.size());
		compiled.insert(compiled.end(), entries.begin(), entries.end());
		compiled.insert(compiled.end(), original.begin() + exportEnd, original.end());
		return true;
	} catch (const std::runtime_error& failure) {
		error = failure.what(); return false;
	}
}

#if defined(RECOIL_WASMTIME_AVAILABLE)
bool StateInspector::Bind(wasmtime_context_t* store, const wasmtime_instance_t& instance,
	const StateExports& exports, std::string& error)
{
	context = store; globals.clear(); tables.clear(); functions.clear();
	const auto resolve = [&](char kind, std::uint32_t index, std::uint8_t type, wasmtime_extern_t& item) {
		const auto name = exports.Name(kind, index);
		if (wasmtime_instance_export_get(context, &instance, name.data(), name.size(), &item) && item.kind == type) return true;
		error = "missing private Wasm state export: " + name; return false;
	};
	wasmtime_extern_t item;
	for (std::uint32_t i = 0; i < exports.functions; ++i) {
		if (!resolve('f', i, WASMTIME_EXTERN_FUNC, item)) return false;
		// Only lookup uses native addresses; the snapshot records the original
		// function index. Imports can alias one native function: choose first.
		functions.emplace(wasmtime_func_to_raw(context, &item.of.func), i);
	}
	for (std::uint32_t i = 0; i < exports.globals; ++i) {
		if (!resolve('g', i, WASMTIME_EXTERN_GLOBAL, item)) return false;
		globals.push_back(item.of.global);
	}
	for (std::uint32_t i = 0; i < exports.tables; ++i) {
		if (!resolve('t', i, WASMTIME_EXTERN_TABLE, item)) return false;
		tables.push_back(item.of.table);
	}
	return true;
}

bool StateInspector::Value(const wasmtime_val_t& value, std::vector<std::uint8_t>& out,
	std::string& error) const
{
	out.push_back(value.kind);
	switch (value.kind) {
		case WASMTIME_I32: Number(out, static_cast<std::uint32_t>(value.of.i32), 4); break;
		case WASMTIME_I64: Number(out, static_cast<std::uint64_t>(value.of.i64), 8); break;
		case WASMTIME_F32: Number(out, std::bit_cast<std::uint32_t>(value.of.f32), 4); break;
		case WASMTIME_F64: Number(out, std::bit_cast<std::uint64_t>(value.of.f64), 8); break;
		case WASMTIME_V128: out.insert(out.end(), std::begin(value.of.v128), std::end(value.of.v128)); break;
		case WASMTIME_FUNCREF: {
			if (wasmtime_funcref_is_null(&value.of.funcref)) { Number(out, 0, 4); break; }
			const auto found = functions.find(wasmtime_func_to_raw(context, &value.of.funcref));
			if (found == functions.end()) { error = "unrecognized Wasm function reference in synced state"; return false; }
			Number(out, found->second + 1, 4); break;
		}
		case WASMTIME_EXTERNREF:
			if (wasmtime_externref_is_null(&value.of.externref)) { out.push_back(0); break; }
			[[fallthrough]];
		default: error = "nonportable reference type in synced Wasm state"; return false;
	}
	return true;
}

bool StateInspector::Snapshot(std::vector<std::uint8_t>& out, std::string& error) const
{
	out.clear(); Number(out, 1, 4); // versioned canonical serialization
	Number(out, globals.size(), 4);
	for (const auto& global: globals) {
		wasmtime_val_t value; wasmtime_global_get(context, &global, &value);
		const bool ok = Value(value, out, error); wasmtime_val_unroot(&value);
		if (!ok) return false;
	}
	Number(out, tables.size(), 4);
	for (const auto& table: tables) {
		const auto size = wasmtime_table_size(context, &table); Number(out, size, 8);
		for (std::uint64_t i = 0; i < size; ++i) {
			wasmtime_val_t value;
			if (!wasmtime_table_get(context, &table, i, &value)) { error = "cannot read synced Wasm table"; return false; }
			const bool ok = Value(value, out, error); wasmtime_val_unroot(&value);
			if (!ok) return false;
		}
	}
	return true;
}
#endif
} // namespace recoil::wasm::core
