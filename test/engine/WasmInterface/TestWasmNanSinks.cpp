/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

// NaN canonicalization at sinks (WasmNanSinks) against Cranelift's global
// canonicalization. A module rewritten by the pass and compiled without global
// canonicalization must leave exactly the memory, globals, host-call arguments
// and results that the original module leaves with it.

#include <catch_amalgamated.hpp>

#include <cstdint>
#include <cstring>
#include <memory>
#include <string>
#include <vector>

#include <wasmtime.h>

#include "WasmInterface/runtime/WasmNanSinks.h"

namespace {

enum class Mode { Global, Sinks, None };

std::vector<std::uint8_t> Wat(const std::string& wat)
{
	wasm_byte_vec_t bytes;
	wasmtime_error_t* error = wasmtime_wat2wasm(wat.data(), wat.size(), &bytes);
	if (error != nullptr) {
		wasm_message_t message;
		wasmtime_error_message(error, &message);
		const std::string text(message.data, message.size);
		wasm_byte_vec_delete(&message);
		wasmtime_error_delete(error);
		FAIL("wat: " << text);
	}
	std::vector<std::uint8_t> result(bytes.data, bytes.data + bytes.size);
	wasm_byte_vec_delete(&bytes);
	return result;
}

struct Engine {
	wasm_engine_t* engine = nullptr;
	explicit Engine(bool canonicalize)
	{
		wasm_config_t* config = wasm_config_new();
		wasmtime_config_wasm_simd_set(config, true);
		wasmtime_config_wasm_multi_value_set(config, true);
		wasmtime_config_cranelift_nan_canonicalization_set(config, canonicalize);
		engine = wasm_engine_new_with_config(config);
	}
	~Engine() { wasm_engine_delete(engine); }
};

Engine& EngineFor(Mode mode)
{
	static Engine canonicalizing(true), plain(false);
	return mode == Mode::Global ? canonicalizing : plain;
}

struct Observation {
	std::vector<std::uint8_t> memory;
	std::vector<std::uint64_t> results, calls;
	std::uint32_t g32 = 0;
	std::uint64_t g64 = 0;
	bool operator==(const Observation&) const = default;
};

std::uint64_t Bits(const wasmtime_val_t& v)
{
	std::uint64_t bits = 0;
	switch (v.kind) {
		case WASMTIME_F32: { std::uint32_t b; std::memcpy(&b, &v.of.f32, 4); bits = b; break; }
		case WASMTIME_F64: std::memcpy(&bits, &v.of.f64, 8); break;
		case WASMTIME_I32: bits = std::uint32_t(v.of.i32); break;
		case WASMTIME_I64: bits = std::uint64_t(v.of.i64); break;
		default: break;
	}
	return bits;
}

wasm_trap_t* Record(void* env, wasmtime_caller_t*, const wasmtime_val_t* args, size_t nargs,
	wasmtime_val_t*, size_t)
{
	auto* calls = static_cast<std::vector<std::uint64_t>*>(env);
	for (size_t i = 0; i < nargs; ++i)
		calls->push_back(Bits(args[i]));
	return nullptr;
}

// Compiles `bytes` for `mode` (rewriting them for Mode::Sinks), runs `entry`
// with `arg`, and returns what is observable afterwards.
Observation Run(const std::vector<std::uint8_t>& original, Mode mode, const char* entry, int arg,
	std::size_t results, std::size_t observedBytes = 4096)
{
	std::vector<std::uint8_t> bytes = original;
	if (mode == Mode::Sinks) {
		std::string error;
		recoil::wasm::NanSinkStats stats;
		REQUIRE(recoil::wasm::CanonicalizeNanAtSinks(original, bytes, &stats, error));
		std::vector<std::uint8_t> again;
		REQUIRE(recoil::wasm::CanonicalizeNanAtSinks(original, again, nullptr, error));
		REQUIRE(again == bytes);
	}
	wasm_engine_t* engine = EngineFor(mode).engine;
	wasmtime_module_t* module = nullptr;
	wasmtime_error_t* error = wasmtime_module_new(engine, bytes.data(), bytes.size(), &module);
	if (error != nullptr) {
		wasm_message_t message;
		wasmtime_error_message(error, &message);
		const std::string text(message.data, message.size);
		wasm_byte_vec_delete(&message);
		wasmtime_error_delete(error);
		FAIL("compile: " << text);
	}
	Observation obs;
	wasmtime_store_t* store = wasmtime_store_new(engine, nullptr, nullptr);
	wasmtime_context_t* context = wasmtime_store_context(store);
	wasmtime_linker_t* linker = wasmtime_linker_new(engine);
	for (const auto& [name, kind] : {std::pair{"rec32", WASM_F32}, std::pair{"rec64", WASM_F64}}) {
		wasm_functype_t* type = wasm_functype_new_1_0(wasm_valtype_new(kind));
		REQUIRE(wasmtime_linker_define_func(linker, "env", 3, name, 5, type, Record, &obs.calls, nullptr) == nullptr);
		wasm_functype_delete(type);
	}
	wasmtime_instance_t instance;
	wasm_trap_t* trap = nullptr;
	REQUIRE(wasmtime_linker_instantiate(linker, context, module, &instance, &trap) == nullptr);
	REQUIRE(trap == nullptr);

	wasmtime_extern_t item;
	REQUIRE(wasmtime_instance_export_get(context, &instance, entry, std::strlen(entry), &item));
	REQUIRE(item.kind == WASMTIME_EXTERN_FUNC);
	wasmtime_val_t argument;
	argument.kind = WASMTIME_I32;
	argument.of.i32 = arg;
	std::vector<wasmtime_val_t> out(results);
	REQUIRE(wasmtime_func_call(context, &item.of.func, &argument, arg < 0 ? 0 : 1, out.data(), results, &trap) == nullptr);
	REQUIRE(trap == nullptr);
	for (const wasmtime_val_t& v : out)
		obs.results.push_back(Bits(v));

	REQUIRE(wasmtime_instance_export_get(context, &instance, "mem", 3, &item));
	const std::uint8_t* memory = wasmtime_memory_data(context, &item.of.memory);
	obs.memory.assign(memory, memory + observedBytes);
	for (const char* name : {"g32", "g64"}) {
		REQUIRE(wasmtime_instance_export_get(context, &instance, name, 3, &item));
		wasmtime_val_t value;
		wasmtime_global_get(context, &item.of.global, &value);
		if (value.kind == WASMTIME_F32) obs.g32 = std::uint32_t(Bits(value));
		else obs.g64 = Bits(value);
	}
	wasmtime_linker_delete(linker);
	wasmtime_store_delete(store);
	wasmtime_module_delete(module);
	return obs;
}

std::uint32_t U32(const Observation& o, std::size_t at)
{
	std::uint32_t v;
	std::memcpy(&v, o.memory.data() + at, 4);
	return v;
}
std::uint64_t U64(const Observation& o, std::size_t at)
{
	std::uint64_t v;
	std::memcpy(&v, o.memory.data() + at, 8);
	return v;
}

constexpr std::uint32_t CANON32 = 0x7fc00000u, PAYLOAD32 = 0x7fc12345u;
constexpr std::uint64_t CANON64 = 0x7ff8000000000000ull, PAYLOAD64 = 0x7ff8000000012345ull;

const char* const CASES = R"((module
  (import "env" "rec32" (func $rec32 (param f32)))
  (import "env" "rec64" (func $rec64 (param f64)))
  (memory (export "mem") 1)
  (global $g32 (export "g32") (mut f32) (f32.const 0))
  (global $g64 (export "g64") (mut f64) (f64.const 0))
  ;; 0: f32 payload NaN, negative signalling NaN, 1, -1; 16: f64 payload NaN, 1;
  ;; 32..47 zero; 48: f32 inf; 56: f64 inf
  (data (i32.const 0) "\45\23\c1\7f\01\00\a0\ff\00\00\80\3f\00\00\80\bf")
  (data (i32.const 16) "\45\23\01\00\00\00\f8\7f\00\00\00\00\00\00\f0\3f")
  (data (i32.const 48) "\00\00\80\7f\00\00\00\00\00\00\00\00\00\00\f0\7f")
  (func $nan32 (result f32) (f32.div (f32.load (i32.const 32)) (f32.load (i32.const 32))))
  (func (export "ret32") (param i32) (result f32) (call $nan32))
  (func (export "ret64") (param i32) (result f64) (f64.div (f64.load (i32.const 40)) (f64.load (i32.const 40))))
  (func (export "stores") (param i32)
    (f32.store (i32.const 1024) (f32.div (f32.load (i32.const 32)) (f32.load (i32.const 32))))
    (f32.store (i32.const 1028) (f32.sub (f32.load (i32.const 48)) (f32.load (i32.const 48))))
    (f32.store (i32.const 1032) (f32.sqrt (f32.load (i32.const 12))))
    (f32.store (i32.const 1036) (f32.add (f32.load (i32.const 0)) (f32.load (i32.const 8))))
    (f32.store (i32.const 1040) (f32.load (i32.const 0)))
    (f32.store (i32.const 1044) (f32.neg (f32.div (f32.load (i32.const 32)) (f32.load (i32.const 32)))))
    (f32.store (i32.const 1048) (f32.copysign (f32.load (i32.const 8)) (f32.div (f32.load (i32.const 32)) (f32.load (i32.const 32)))))
    (i32.store (i32.const 1052) (i32.reinterpret_f32 (f32.mul (f32.load (i32.const 0)) (f32.load (i32.const 8)))))
    (f32.store (i32.const 1056) (f32.abs (f32.mul (f32.load (i32.const 4)) (f32.load (i32.const 8)))))
    (f32.store (i32.const 1060) (f32.min (f32.load (i32.const 0)) (f32.load (i32.const 8))))
    (f32.store (i32.const 1064) (f32.demote_f64 (f64.load (i32.const 16))))
    (f64.store (i32.const 1072) (f64.div (f64.load (i32.const 40)) (f64.load (i32.const 40))))
    (f64.store (i32.const 1080) (f64.add (f64.load (i32.const 16)) (f64.load (i32.const 24))))
    (f64.store (i32.const 1088) (f64.load (i32.const 16)))
    (f64.store (i32.const 1096) (f64.promote_f32 (f32.load (i32.const 0))))
    (call $rec32 (f32.div (f32.load (i32.const 32)) (f32.load (i32.const 32))))
    (call $rec32 (f32.load (i32.const 0)))
    (call $rec64 (f64.div (f64.load (i32.const 40)) (f64.load (i32.const 40))))
    (global.set $g32 (f32.sub (f32.load (i32.const 48)) (f32.load (i32.const 48))))
    (global.set $g64 (f64.mul (f64.load (i32.const 16)) (f64.load (i32.const 24)))))
  (func (export "merges") (param $c i32) (local $x f32)
    (f32.store (i32.const 1104) (select (f32.load (i32.const 0)) (call $nan32) (local.get $c)))
    (if (local.get $c)
      (then (local.set $x (f32.load (i32.const 0))))
      (else (local.set $x (f32.add (f32.load (i32.const 0)) (f32.load (i32.const 8))))))
    (f32.store (i32.const 1108) (local.get $x))
    (f32.store (i32.const 1112)
      (block (result f32)
        (drop (br_if 0 (f32.load (i32.const 0)) (local.get $c)))
        (f32.div (f32.load (i32.const 32)) (f32.load (i32.const 32))))))
  (func (export "loop") (param i32) (local $acc f32) (local $i i32)
    (local.set $acc (f32.load (i32.const 0)))
    (local.set $i (i32.const 3))
    (loop $l
      (f32.store offset=1120 (i32.shl (local.get $i) (i32.const 2)) (local.get $acc))
      (local.set $acc (f32.add (local.get $acc) (f32.load (i32.const 8))))
      (br_if $l (local.tee $i (i32.sub (local.get $i) (i32.const 1)))))
    (f32.store (i32.const 1136) (local.get $acc)))
  (func (export "vectors") (param i32)
    (v128.store (i32.const 1152) (f32x4.div (v128.load (i32.const 32)) (v128.load (i32.const 32))))
    (v128.store (i32.const 1168) (f64x2.div (v128.load (i32.const 32)) (v128.load (i32.const 32))))
    (i32.store (i32.const 1184) (i32x4.bitmask (f32x4.neg (f32x4.div (v128.load (i32.const 32)) (v128.load (i32.const 32))))))
    (i32.store (i32.const 1188) (i32x4.bitmask (f32x4.div (v128.load (i32.const 32)) (v128.load (i32.const 32)))))
    (v128.store (i32.const 1200) (v128.load (i32.const 0)))
    (v128.store (i32.const 1216) (f32x4.add (v128.load (i32.const 0)) (v128.load (i32.const 0))))
    (f32.store (i32.const 1232) (f32x4.extract_lane 0 (f32x4.mul (v128.load (i32.const 0)) (v128.load (i32.const 0)))))
    (v128.store (i32.const 1248) (f64x2.promote_low_f32x4 (v128.load (i32.const 0))))
    (v128.store (i32.const 1264) (v128.and (f32x4.div (v128.load (i32.const 32)) (v128.load (i32.const 32)))
      (v128.const i32x4 -1 -1 -1 -1)))
    (v128.store (i32.const 1280) (i64x2.add (f64x2.sqrt (v128.load (i32.const 0))) (v128.const i64x2 0 0))))
))";

// ---------------------------------------------------------------------------
// Random programs

struct Rng {
	std::uint64_t s;
	std::uint64_t Next()
	{
		std::uint64_t z = (s += 0x9e3779b97f4a7c15ull);
		z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9ull;
		z = (z ^ (z >> 27)) * 0x94d049bb133111ebull;
		return z ^ (z >> 31);
	}
	unsigned Below(unsigned n) { return unsigned(Next() % n); }
	bool Chance(unsigned percent) { return Below(100) < percent; }
};

const std::uint32_t F32_BITS[] = {0x00000000u, 0x80000000u, 0x3f800000u, 0xbf800000u, 0x7f800000u,
	0xff800000u, 0x7fc00000u, 0xffc00000u, 0x7fc12345u, 0xffc54321u, 0x7f800001u, 0xff800001u,
	0x7fbfffffu, 0x00000001u, 0x7f7fffffu, 0x40490fdbu};
const std::uint64_t F64_BITS[] = {0, 0x8000000000000000ull, 0x3ff0000000000000ull, 0xbff0000000000000ull,
	0x7ff0000000000000ull, 0xfff0000000000000ull, 0x7ff8000000000000ull, 0xfff8000000000000ull,
	0x7ff8000000012345ull, 0xfff8000000054321ull, 0x7ff0000000000001ull, 0xfff4000000000000ull, 1,
	0x7fefffffffffffffull, 0x400921fb54442d18ull, 0x3fe0000000000000ull};

struct Generator {
	Rng rng;
	int slot = 0;
	int loops = 0;
	int labels = 0;

	std::string Hex(std::uint64_t v)
	{
		char buf[24];
		std::snprintf(buf, sizeof buf, "0x%llx", static_cast<unsigned long long>(v));
		return buf;
	}
	std::string Store()
	{
		return "offset=" + std::to_string(1024 + 16 * (slot++ % 180)) + " (i32.const 0)";
	}
	std::string Pick(std::initializer_list<const char*> options)
	{
		return *(options.begin() + rng.Below(unsigned(options.size())));
	}

	std::string Cond(int d)
	{
		switch (rng.Below(6)) {
			case 0: return "(local.get $c)";
			case 1: return "(" + Pick({"f32.lt", "f32.eq", "f32.ne", "f32.ge"}) + " " + F32(d - 1) + " " + F32(d - 1) + ")";
			case 2: return "(" + Pick({"f64.lt", "f64.eq", "f64.gt"}) + " " + F64(d - 1) + " " + F64(d - 1) + ")";
			case 3: return "(i32.trunc_sat_f32_s " + F32(d - 1) + ")";
			case 4: return "(i32.and (i32x4.bitmask " + V(d - 1) + ") (i32.const 1))";
			default: return "(i32.and (local.get $c) (i32.trunc_sat_f64_u " + F64(d - 1) + "))";
		}
	}

	std::string F32(int d)
	{
		if (d <= 0 || rng.Chance(20)) {
			switch (rng.Below(6)) {
				case 0: case 1: return "(f32.load offset=" + std::to_string(4 * rng.Below(64)) + " (i32.const 0))";
				case 2: return "(f32.reinterpret_i32 (i32.const " + Hex(F32_BITS[rng.Below(16)]) + "))";
				case 3: return "(local.get " + Pick({"$a", "$b"}) + ")";
				case 4: return "(f32.convert_i32_s (local.get $c))";
				default: return "(global.get $g32)";
			}
		}
		switch (rng.Below(14)) {
			case 0: case 1:
				return "(" + Pick({"f32.add", "f32.sub", "f32.mul", "f32.div", "f32.min", "f32.max"}) + " " + F32(d - 1) + " " + F32(d - 1) + ")";
			case 2: return "(f32.copysign " + F32(d - 1) + " " + F32(d - 1) + ")";
			case 3:
				return "(" + Pick({"f32.neg", "f32.abs", "f32.sqrt", "f32.ceil", "f32.floor", "f32.trunc", "f32.nearest"}) + " " + F32(d - 1) + ")";
			case 4: return "(f32.demote_f64 " + F64(d - 1) + ")";
			case 5: return "(select " + F32(d - 1) + " " + F32(d - 1) + " " + Cond(d - 1) + ")";
			case 6: return "(f32x4.extract_lane " + std::to_string(rng.Below(4)) + " " + V(d - 1) + ")";
			case 7: return "(if (result f32) " + Cond(d - 1) + " (then " + F32(d - 1) + ") (else " + F32(d - 1) + "))";
			case 8: return "(block (result f32) (drop (br_if 0 " + F32(d - 1) + " " + Cond(d - 1) + ")) " + F32(d - 1) + ")";
			case 9: return "(local.tee " + Pick({"$a", "$b"}) + " " + F32(d - 1) + ")";
			case 10: return "(call $h32 " + F32(d - 1) + " " + F32(d - 1) + ")";
			case 11: return "(f32.reinterpret_i32 (i32.reinterpret_f32 " + F32(d - 1) + "))";
			default: return "(f32.add " + F32(d - 1) + " (f32.load offset=" + std::to_string(4 * rng.Below(64)) + " (i32.const 0)))";
		}
	}

	std::string F64(int d)
	{
		if (d <= 0 || rng.Chance(20)) {
			switch (rng.Below(5)) {
				case 0: case 1: return "(f64.load offset=" + std::to_string(256 + 8 * rng.Below(32)) + " (i32.const 0))";
				case 2: return "(f64.reinterpret_i64 (i64.const " + Hex(F64_BITS[rng.Below(16)]) + "))";
				case 3: return "(local.get " + Pick({"$x", "$y"}) + ")";
				default: return "(global.get $g64)";
			}
		}
		switch (rng.Below(11)) {
			case 0: case 1:
				return "(" + Pick({"f64.add", "f64.sub", "f64.mul", "f64.div", "f64.min", "f64.max"}) + " " + F64(d - 1) + " " + F64(d - 1) + ")";
			case 2: return "(f64.copysign " + F64(d - 1) + " " + F64(d - 1) + ")";
			case 3:
				return "(" + Pick({"f64.neg", "f64.abs", "f64.sqrt", "f64.ceil", "f64.floor", "f64.trunc", "f64.nearest"}) + " " + F64(d - 1) + ")";
			case 4: return "(f64.promote_f32 " + F32(d - 1) + ")";
			case 5: return "(select " + F64(d - 1) + " " + F64(d - 1) + " " + Cond(d - 1) + ")";
			case 6: return "(f64x2.extract_lane " + std::to_string(rng.Below(2)) + " " + V(d - 1) + ")";
			case 7: return "(if (result f64) " + Cond(d - 1) + " (then " + F64(d - 1) + ") (else " + F64(d - 1) + "))";
			case 8: return "(local.tee " + Pick({"$x", "$y"}) + " " + F64(d - 1) + ")";
			case 9: return "(call $h64 " + F64(d - 1) + " " + F64(d - 1) + ")";
			default: return "(f64.mul " + F64(d - 1) + " (f64.load offset=" + std::to_string(256 + 8 * rng.Below(32)) + " (i32.const 0)))";
		}
	}

	std::string V(int d)
	{
		if (d <= 0 || rng.Chance(20)) {
			switch (rng.Below(4)) {
				case 0: case 1: return "(v128.load offset=" + std::to_string(16 * rng.Below(32)) + " (i32.const 0))";
				case 2: {
					std::string s = "(v128.const i32x4";
					for (int i = 0; i < 4; ++i) s += " " + Hex(F32_BITS[rng.Below(16)]);
					return s + ")";
				}
				default: return "(local.get " + Pick({"$v", "$w"}) + ")";
			}
		}
		switch (rng.Below(18)) {
			case 0: case 1:
				return "(" + Pick({"f32x4.add", "f32x4.sub", "f32x4.mul", "f32x4.div", "f32x4.min", "f32x4.max", "f32x4.pmin", "f32x4.pmax"}) + " " + V(d - 1) + " " + V(d - 1) + ")";
			case 2: case 3:
				return "(" + Pick({"f64x2.add", "f64x2.sub", "f64x2.mul", "f64x2.div", "f64x2.min", "f64x2.max", "f64x2.pmin", "f64x2.pmax"}) + " " + V(d - 1) + " " + V(d - 1) + ")";
			case 4:
				return "(" + Pick({"f32x4.sqrt", "f32x4.abs", "f32x4.neg", "f32x4.ceil", "f32x4.floor", "f32x4.trunc", "f32x4.nearest",
					"f64x2.sqrt", "f64x2.abs", "f64x2.neg", "f64x2.ceil", "f64x2.nearest"}) + " " + V(d - 1) + ")";
			case 5: return "(f32x4.splat " + F32(d - 1) + ")";
			case 6: return "(f64x2.splat " + F64(d - 1) + ")";
			case 7: return "(f32x4.replace_lane " + std::to_string(rng.Below(4)) + " " + V(d - 1) + " " + F32(d - 1) + ")";
			case 8: return "(f64x2.replace_lane " + std::to_string(rng.Below(2)) + " " + V(d - 1) + " " + F64(d - 1) + ")";
			case 9: return "(" + Pick({"f32x4.demote_f64x2_zero", "f64x2.promote_low_f32x4"}) + " " + V(d - 1) + ")";
			case 10: return "(v128.bitselect " + V(d - 1) + " " + V(d - 1) + " (f32x4.lt " + V(d - 1) + " " + V(d - 1) + "))";
			case 11: return "(" + Pick({"v128.and", "v128.or", "v128.xor", "i32x4.add", "i8x16.swizzle"}) + " " + V(d - 1) + " " + V(d - 1) + ")";
			case 12: {
				std::string s = "(i8x16.shuffle";
				const bool lanes = rng.Chance(50);
				for (int i = 0; i < 16; ++i) {
					if (lanes) s += " " + std::to_string(i % 4 + 4 * ((i / 4 + 1) % 8));
					else s += " " + std::to_string(rng.Below(32));
				}
				return s + " " + V(d - 1) + " " + V(d - 1) + ")";
			}
			case 13: return "(" + Pick({"f32x4.convert_i32x4_s", "i32x4.trunc_sat_f32x4_s", "f64x2.convert_low_i32x4_u", "i32x4.trunc_sat_f64x2_s_zero"}) + " " + V(d - 1) + ")";
			case 14: return "(select " + V(d - 1) + " " + V(d - 1) + " " + Cond(d - 1) + ")";
			case 15: return "(local.tee " + Pick({"$v", "$w"}) + " " + V(d - 1) + ")";
			case 16: return "(" + Pick({"f32x4.eq", "f64x2.lt", "f32x4.ne"}) + " " + V(d - 1) + " " + V(d - 1) + ")";
			default: return "(v128.load32_lane offset=" + std::to_string(4 * rng.Below(64)) + " " + std::to_string(rng.Below(4)) + " (i32.const 0) " + V(d - 1) + ")";
		}
	}

	std::string Stmt(int d)
	{
		const int expr = 3;
		switch (rng.Below(d > 0 ? 16 : 12)) {
			case 0: case 1: return "(f32.store " + Store() + " " + F32(expr) + ")";
			case 2: return "(f64.store " + Store() + " " + F64(expr) + ")";
			case 3: case 4: return "(v128.store " + Store() + " " + V(expr) + ")";
			case 5: return "(i32.store " + Store() + " (i32.reinterpret_f32 " + F32(expr) + "))";
			case 6: return "(i64.store " + Store() + " (i64.reinterpret_f64 " + F64(expr) + "))";
			case 7: {
				const std::string at = std::to_string(1024 + 16 * (slot++ % 180));
				return "(v128.store64_lane offset=" + at + " " + std::to_string(rng.Below(2)) + " (i32.const 0) " + V(expr) + ")";
			}
			case 8:
				switch (rng.Below(3)) {
					case 0: return "(local.set " + Pick({"$a", "$b"}) + " " + F32(expr) + ")";
					case 1: return "(local.set " + Pick({"$x", "$y"}) + " " + F64(expr) + ")";
					default: return "(local.set " + Pick({"$v", "$w"}) + " " + V(expr) + ")";
				}
			case 9: return rng.Chance(50) ? "(global.set $g32 " + F32(expr) + ")" : "(global.set $g64 " + F64(expr) + ")";
			case 10: return rng.Chance(50) ? "(call $rec32 " + F32(expr) + ")" : "(call $rec64 " + F64(expr) + ")";
			case 11: return "(i32.store " + Store() + " (i32x4.bitmask " + V(expr) + "))";
			case 12: case 13:
				return "(if " + Cond(2) + " (then " + Stmts(d - 1) + ") (else " + Stmts(d - 1) + "))";
			case 14: {
				if (loops >= 2)
					return Stmts(d - 1);
				const std::string i = "$i" + std::to_string(loops++);
				const std::string l = "$l" + std::to_string(labels++);
				std::string body = "(local.set " + i + " (i32.const 3)) (loop " + l + " " + Stmts(d - 1) +
					" (br_if " + l + " (local.tee " + i + " (i32.sub (local.get " + i + ") (i32.const 1)))))";
				--loops;
				return body;
			}
			default: {
				const std::string l = "$b" + std::to_string(labels++);
				return "(block " + l + " " + Stmts(d - 1) + " (br_if " + l + " " + Cond(2) + ") " + Stmts(d - 1) + ")";
			}
		}
	}

	std::string Stmts(int d)
	{
		std::string s;
		for (unsigned n = 1 + rng.Below(4); n > 0; --n)
			s += Stmt(d) + "\n";
		return s;
	}

	std::string Module()
	{
		std::string data;
		auto bytes = [&](std::uint64_t v, int n) {
			for (int i = 0; i < n; ++i) {
				char buf[4];
				std::snprintf(buf, sizeof buf, "\\%02x", unsigned((v >> (8 * i)) & 0xff));
				data += buf;
			}
		};
		for (int i = 0; i < 64; ++i) bytes(F32_BITS[rng.Below(16)], 4);
		for (int i = 0; i < 32; ++i) bytes(F64_BITS[rng.Below(16)], 8);
		std::string m = R"((module
  (import "env" "rec32" (func $rec32 (param f32)))
  (import "env" "rec64" (func $rec64 (param f64)))
  (memory (export "mem") 1)
  (global $g32 (export "g32") (mut f32) (f32.const 0))
  (global $g64 (export "g64") (mut f64) (f64.const 0))
  (data (i32.const 0) ")" + data + R"(")
  (func $h32 (param $p f32) (param $q f32) (result f32)
    (f32.store offset=3968 (i32.const 0) (local.get $p))
    (select (local.get $q) (f32.mul (local.get $p) (local.get $q)) (f32.lt (local.get $p) (local.get $q))))
  (func $h64 (param $p f64) (param $q f64) (result f64)
    (f64.store offset=3976 (i32.const 0) (f64.sub (local.get $p) (local.get $q)))
    (local.get $p))
  (func (export "run") (param $c i32) (result f32 f64 i64 i64)
    (local $a f32) (local $b f32) (local $x f64) (local $y f64) (local $v v128) (local $w v128)
    (local $i0 i32) (local $i1 i32)
)";
		m += Stmts(3);
		m += F32(3) + "\n" + F64(3) + "\n";
		m += "(i64x2.extract_lane 0 (local.get $v)) (i64x2.extract_lane 1 (local.get $w))))\n";
		return m;
	}
};

} // namespace

TEST_CASE("NaN sinks: arithmetic NaNs are canonical where they become visible")
{
	const std::vector<std::uint8_t> module = Wat(CASES);
	const auto both = [&](const char* entry, int arg, std::size_t results) {
		const Observation global = Run(module, Mode::Global, entry, arg, results);
		const Observation sinks = Run(module, Mode::Sinks, entry, arg, results);
		CHECK(global == sinks);
		return sinks;
	};

	CHECK(both("ret32", 0, 1).results[0] == CANON32);
	CHECK(both("ret64", 0, 1).results[0] == CANON64);

	const Observation s = both("stores", 0, 0);
	for (std::size_t at : {1024u, 1028u, 1032u, 1036u, 1052u, 1056u, 1060u, 1064u})
		CHECK(U32(s, at) == CANON32);
	CHECK(U32(s, 1040) == PAYLOAD32);      // a loaded NaN keeps its bits
	CHECK(U32(s, 1044) == 0xffc00000u);    // the sign of a canonical NaN, flipped
	CHECK(U32(s, 1048) == 0x3f800000u);    // copysign takes the canonical NaN's (positive) sign
	for (std::size_t at : {1072u, 1080u, 1096u})
		CHECK(U64(s, at) == CANON64);
	CHECK(U64(s, 1088) == PAYLOAD64);
	CHECK(s.calls == std::vector<std::uint64_t>{CANON32, PAYLOAD32, CANON64});
	CHECK(s.g32 == CANON32);
	CHECK(s.g64 == CANON64);

	const Observation raw = both("merges", 1, 0);
	const Observation arith = both("merges", 0, 0);
	for (std::size_t at : {1104u, 1108u, 1112u}) {
		CHECK(U32(raw, at) == PAYLOAD32);
		CHECK(U32(arith, at) == CANON32);
	}

	const Observation l = both("loop", 0, 0);
	CHECK(U32(l, 1132) == PAYLOAD32);
	for (std::size_t at : {1128u, 1124u, 1136u})
		CHECK(U32(l, at) == CANON32);

	const Observation v = both("vectors", 0, 0);
	for (std::size_t at = 1152; at < 1168; at += 4)
		CHECK(U32(v, at) == CANON32);
	CHECK(U64(v, 1168) == CANON64);
	CHECK(U64(v, 1176) == CANON64);
	CHECK(U32(v, 1184) == 0xfu);
	CHECK(U32(v, 1188) == 0u);
	CHECK(U32(v, 1200) == PAYLOAD32);
	CHECK(U32(v, 1204) == 0xffa00001u);
	CHECK(U32(v, 1216) == CANON32);
	CHECK(U32(v, 1220) == CANON32);
	CHECK(U32(v, 1224) == 0x40000000u);
	CHECK(U32(v, 1228) == 0xc0000000u);
	CHECK(U32(v, 1232) == CANON32);
	CHECK(U64(v, 1248) == CANON64);
	CHECK(U64(v, 1256) == CANON64);
	for (std::size_t at = 1264; at < 1280; at += 4)
		CHECK(U32(v, at) == CANON32);

#if defined(__x86_64__) || defined(_M_X64)
	// Without any canonicalization x86 produces its default NaN (negative): the
	// cases above do see architecture-dependent bits.
	CHECK(U32(Run(module, Mode::None, "stores", 0, 0), 1024) == 0xffc00000u);
#endif
}

TEST_CASE("NaN sinks: random float programs observe what global canonicalization gives")
{
	int programs = 0, differWithout = 0;
	recoil::wasm::NanSinkStats total;
	for (std::uint64_t seed = 1; seed <= 400; ++seed) {
		Generator gen{Rng{seed}};
		const std::string wat = gen.Module();
		const std::vector<std::uint8_t> module = Wat(wat);
		{
			std::vector<std::uint8_t> rewritten;
			std::string error;
			recoil::wasm::NanSinkStats stats;
			REQUIRE(recoil::wasm::CanonicalizeNanAtSinks(module, rewritten, &stats, error));
			total.arithmeticSites += stats.arithmeticSites;
			total.storeSinks += stats.Inserted();
		}
		for (int c : {0, 1}) {
			const Observation global = Run(module, Mode::Global, "run", c, 4);
			const Observation sinks = Run(module, Mode::Sinks, "run", c, 4);
			const Observation none = Run(module, Mode::None, "run", c, 4);
			INFO("seed " << seed << " c " << c << "\n" << wat);
			REQUIRE(global == sinks);
			differWithout += !(global == none);
		}
		++programs;
	}
	INFO("programs " << programs << ", arithmetic sites " << total.arithmeticSites << ", inserted " << total.storeSinks);
	CHECK(programs == 400);
#if defined(__x86_64__) || defined(_M_X64)
	// The programs do produce NaNs whose bits differ without canonicalization.
	CHECK(differWithout > 100);
#endif
	WARN("random programs: " << programs << ", runs differing without canonicalization: " << differWithout
		<< ", arithmetic sites " << total.arithmeticSites << ", canonicalizations inserted " << total.storeSinks);
}
