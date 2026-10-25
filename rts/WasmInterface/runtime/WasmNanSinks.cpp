/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmNanSinks.h"

#include <algorithm>
#include <array>
#include <cstring>
#include <stdexcept>
#include <string_view>

// How the pass works (see WasmNanSinks.h for the rule).
//
// Every float, f64 and v128 value is tracked through a function by abstract
// interpretation over Wasm's structured control flow. A scalar value is
//   K  its bits are those of global canonicalization and are not a non-canonical NaN
//      (constants, int->float conversions, zero-initialized locals, canonicalized values);
//   T  canonicalizing it gives the bits global canonicalization would have
//      (the result of an operation that may produce a NaN, passed on unchanged);
//   R  its bits must stay exactly as they are (loads, parameters, call results,
//      reinterpretations, sign-bit results).
// K joins with either; T and R must not merge into one value, because a later
// sink could not canonicalize the one without altering the other. A v128 value
// is tracked per float shape: K/T for its f32x4 or f64x2 lanes, or R.
//
// A T value that reaches a sink is canonicalized right after the instruction
// that produced it (each operand-stack value has exactly one producer and one
// consumer, so this is equivalent to canonicalizing it at the sink). Merges that
// would join T with R are resolved by canonicalizing the T side: a select's
// operand at its producer; a local, or a block or loop result slot, by flagging
// it so that every T value entering it is canonicalized, and running the
// analysis again until no new flag appears. A local that is dead at a merge
// (backward liveness) takes no part in it.
namespace recoil::wasm {
namespace {

struct Unsupported : std::runtime_error {
	using std::runtime_error::runtime_error;
};
struct Malformed : std::runtime_error {
	using std::runtime_error::runtime_error;
};

struct Reader {
	const std::uint8_t* data = nullptr;
	std::size_t size = 0;
	std::size_t pos = 0;

	bool AtEnd() const { return pos >= size; }
	std::uint8_t Byte()
	{
		if (pos >= size)
			throw Malformed("truncated module");
		return data[pos++];
	}
	std::uint8_t Peek() const
	{
		if (pos >= size)
			throw Malformed("truncated module");
		return data[pos];
	}
	std::uint64_t Uleb(unsigned bits)
	{
		std::uint64_t result = 0;
		for (unsigned shift = 0;; shift += 7) {
			if (shift >= bits + 7)
				throw Malformed("integer too long");
			const std::uint8_t b = Byte();
			if (shift < 64)
				result |= std::uint64_t(b & 0x7f) << shift;
			if (!(b & 0x80))
				return result;
		}
	}
	std::uint32_t U32() { return static_cast<std::uint32_t>(Uleb(32)); }
	std::int64_t Sleb(unsigned bits)
	{
		std::int64_t result = 0;
		unsigned shift = 0;
		std::uint8_t b = 0;
		do {
			if (shift >= bits + 7)
				throw Malformed("integer too long");
			b = Byte();
			if (shift < 64)
				result |= std::int64_t(b & 0x7f) << shift;
			shift += 7;
		} while (b & 0x80);
		if (shift < 64 && (b & 0x40))
			result |= -(std::int64_t(1) << shift);
		return result;
	}
	void Skip(std::size_t n)
	{
		if (n > size - pos)
			throw Malformed("truncated module");
		pos += n;
	}
	std::uint32_t Fixed32()
	{
		std::uint32_t v = 0;
		for (unsigned i = 0; i < 4; ++i)
			v |= std::uint32_t(Byte()) << (8 * i);
		return v;
	}
};

void PutUleb(std::vector<std::uint8_t>& out, std::uint64_t v)
{
	do {
		std::uint8_t b = v & 0x7f;
		v >>= 7;
		out.push_back(b | (v ? 0x80 : 0));
	} while (v);
}

// Value types, coarsened: the analysis only distinguishes the float carriers.
enum Ty : std::uint8_t { INT = 0, F32 = 1, F64 = 2, V128 = 3 };

Ty ValType(std::uint8_t b)
{
	switch (b) {
		case 0x7f: case 0x7e: case 0x70: case 0x6f: return INT;
		case 0x7d: return F32;
		case 0x7c: return F64;
		case 0x7b: return V128;
		default: throw Unsupported("value type 0x" + std::to_string(b) + " is not modelled");
	}
}
bool IsValType(std::uint8_t b)
{
	return b == 0x7f || b == 0x7e || b == 0x7d || b == 0x7c || b == 0x7b || b == 0x70 || b == 0x6f;
}

struct FuncType {
	std::vector<Ty> params, results;
};

struct Module {
	std::vector<FuncType> types;
	std::vector<std::uint32_t> funcTypes; // imports first
	std::uint32_t importedFuncs = 0;
	std::vector<Ty> globals;
};

// ---------------------------------------------------------------------------
// States

constexpr std::uint8_t SK = 0, ST = 1, SR = 2;
constexpr std::uint8_t VK = 0, VK32 = 1, VK64 = 2, VR = 3, VT32 = 4, VT64 = 5;

bool Tainted(Ty ty, std::uint8_t st)
{
	if (ty == F32 || ty == F64)
		return st == ST;
	if (ty == V128)
		return st == VT32 || st == VT64;
	return false;
}
std::uint8_t Canon(Ty ty, std::uint8_t st)
{
	if (ty == F32 || ty == F64)
		return st == ST ? SK : st;
	if (ty == V128)
		return st == VT32 ? VK32 : st == VT64 ? VK64 : st;
	return st;
}
std::uint8_t Top(Ty ty)
{
	return ty == V128 ? VR : ty == INT ? 0 : SR;
}
// False when a tainted value would merge with one whose bits must stay exact.
bool Join(Ty ty, std::uint8_t a, std::uint8_t b, std::uint8_t& out)
{
	if (a == b || ty == INT) {
		out = a;
		return true;
	}
	if (ty != V128) {
		if (a == SK) { out = b; return true; }
		if (b == SK) { out = a; return true; }
		return false;
	}
	if (a > b)
		std::swap(a, b);
	if (a == VK) { out = b; return true; }
	if (Tainted(V128, a) || Tainted(V128, b)) {
		if (a == VK32 && b == VT32) { out = VT32; return true; }
		if (a == VK64 && b == VT64) { out = VT64; return true; }
		return false;
	}
	out = VR;
	return true;
}
// The state of a v128's lanes read as f32 (or f64) scalars, and back.
std::uint8_t View32(std::uint8_t v) { return v == VK || v == VK32 ? SK : v == VT32 ? ST : SR; }
std::uint8_t View64(std::uint8_t v) { return v == VK || v == VK64 ? SK : v == VT64 ? ST : SR; }
std::uint8_t Vec32(std::uint8_t s) { return s == SK ? VK32 : s == ST ? VT32 : VR; }
std::uint8_t Vec64(std::uint8_t s) { return s == SK ? VK64 : s == ST ? VT64 : VR; }

bool CanonicalF32(std::uint32_t bits)
{
	return (bits & 0x7f800000u) != 0x7f800000u || (bits & 0x007fffffu) == 0 || bits == 0x7fc00000u;
}
bool CanonicalF64(std::uint64_t bits)
{
	return (bits & 0x7ff0000000000000ull) != 0x7ff0000000000000ull ||
		(bits & 0x000fffffffffffffull) == 0 || bits == 0x7ff8000000000000ull;
}

// ---------------------------------------------------------------------------
// Instructions

constexpr std::uint32_t NONE = 0xffffffffu;
constexpr std::uint32_t PFX_FC = 0xfc0000u, PFX_FD = 0xfd0000u;

struct Insn {
	std::uint32_t start = 0, end = 0; // byte range in the function body
	std::uint32_t op = 0;             // one-byte opcode, or prefix | sub-opcode
	std::uint32_t a = 0;              // index immediate, f32 bits, br_table count
	std::uint32_t b = NONE;           // structured partner (end of a block; opener of an end)
	std::uint32_t c = NONE;           // else of an if; f64 high bits; br_table offset
	std::int32_t blockType = -1;      // -1 empty, -2 - Ty single result, >= 0 type index
};

// Operand signature kinds for numeric and vector instructions.
enum Kind : std::uint8_t {
	K_PLAIN,      // no float carrier in or out that needs a rule
	K_FARITH,     // may produce a NaN
	K_FPURE,      // reads floats, depends only on NaN-ness (compare, truncate)
	K_ITOF,       // integer to float: never a NaN
	K_F2I_BITS,   // reinterpret: a sink
	K_I2F_BITS,   // reinterpret: exact bits
	K_ABS, K_NEG, K_COPYSIGN,
	V_SINK,       // reads vectors as bits
	V_ARITH32, V_ARITH64, V_CMP32, V_CMP64, V_TOI32, V_TOI64, V_ITOF32, V_ITOF64,
	V_DEMOTE, V_PROMOTE, V_ABS32, V_ABS64, V_NEG32, V_NEG64, V_PMM32, V_PMM64,
	V_SPLAT32, V_SPLAT64, V_EXTRACT32, V_EXTRACT64, V_REPLACE32, V_REPLACE64,
	V_LOAD, V_LOADLANE, V_STORE, V_CONST,
};

struct Sig {
	std::array<Ty, 3> in{};
	std::uint8_t nin = 0;
	Ty outTy = INT;
	std::uint8_t nout = 0;
	Kind kind = K_PLAIN;
};

Ty TyChar(char c)
{
	switch (c) {
		case 'f': return F32;
		case 'd': return F64;
		case 'v': return V128;
		default: return INT;
	}
}
Sig MakeSig(const char* s, Kind kind)
{
	Sig sig;
	sig.kind = kind;
	bool outputs = false;
	for (; *s; ++s) {
		if (*s == '>') { outputs = true; continue; }
		if (outputs) { sig.outTy = TyChar(*s); sig.nout = 1; }
		else sig.in[sig.nin++] = TyChar(*s);
	}
	return sig;
}

bool In(std::uint32_t v, std::uint32_t lo, std::uint32_t hi) { return v >= lo && v <= hi; }

// Signatures of numeric one-byte and 0xfc 0..7 instructions.
bool NumericSig(std::uint32_t op, Sig& sig)
{
	if (op == 0x45 || op == 0x50 || In(op, 0x67, 0x69) || In(op, 0x79, 0x7b) || op == 0xa7 ||
		In(op, 0xac, 0xad) || In(op, 0xc0, 0xc4)) { sig = MakeSig("i>i", K_PLAIN); return true; }
	if (In(op, 0x46, 0x4f) || In(op, 0x51, 0x5a) || In(op, 0x6a, 0x78) || In(op, 0x7c, 0x8a)) {
		sig = MakeSig("ii>i", K_PLAIN); return true;
	}
	if (In(op, 0x5b, 0x60)) { sig = MakeSig("ff>i", K_FPURE); return true; }
	if (In(op, 0x61, 0x66)) { sig = MakeSig("dd>i", K_FPURE); return true; }
	switch (op) {
		case 0x8b: sig = MakeSig("f>f", K_ABS); return true;
		case 0x8c: sig = MakeSig("f>f", K_NEG); return true;
		case 0x98: sig = MakeSig("ff>f", K_COPYSIGN); return true;
		case 0x99: sig = MakeSig("d>d", K_ABS); return true;
		case 0x9a: sig = MakeSig("d>d", K_NEG); return true;
		case 0xa6: sig = MakeSig("dd>d", K_COPYSIGN); return true;
		case 0xb6: sig = MakeSig("d>f", K_FARITH); return true; // demote
		case 0xbb: sig = MakeSig("f>d", K_FARITH); return true; // promote
		case 0xbc: sig = MakeSig("f>i", K_F2I_BITS); return true;
		case 0xbd: sig = MakeSig("d>i", K_F2I_BITS); return true;
		case 0xbe: sig = MakeSig("i>f", K_I2F_BITS); return true;
		case 0xbf: sig = MakeSig("i>d", K_I2F_BITS); return true;
		default: break;
	}
	if (In(op, 0x8d, 0x91)) { sig = MakeSig("f>f", K_FARITH); return true; }
	if (In(op, 0x92, 0x97)) { sig = MakeSig("ff>f", K_FARITH); return true; }
	if (In(op, 0x9b, 0x9f)) { sig = MakeSig("d>d", K_FARITH); return true; }
	if (In(op, 0xa0, 0xa5)) { sig = MakeSig("dd>d", K_FARITH); return true; }
	if (In(op, 0xa8, 0xa9) || In(op, 0xae, 0xaf)) { sig = MakeSig("f>i", K_FPURE); return true; }
	if (In(op, 0xaa, 0xab) || In(op, 0xb0, 0xb1)) { sig = MakeSig("d>i", K_FPURE); return true; }
	if (In(op, 0xb2, 0xb5)) { sig = MakeSig("i>f", K_ITOF); return true; }
	if (In(op, 0xb7, 0xba)) { sig = MakeSig("i>d", K_ITOF); return true; }
	if (op == (PFX_FC | 0) || op == (PFX_FC | 1) || op == (PFX_FC | 4) || op == (PFX_FC | 5)) {
		sig = MakeSig("f>i", K_FPURE); return true;
	}
	if (op == (PFX_FC | 2) || op == (PFX_FC | 3) || op == (PFX_FC | 6) || op == (PFX_FC | 7)) {
		sig = MakeSig("d>i", K_FPURE); return true;
	}
	return false;
}

bool SimdSig(std::uint32_t sub, Sig& sig)
{
	auto set = [&](const char* s, Kind k) { sig = MakeSig(s, k); return true; };
	if (In(sub, 0x00, 0x0a) || sub == 0x5c || sub == 0x5d) return set("i>v", V_LOAD);
	if (sub == 0x0b || In(sub, 0x58, 0x5b)) return set("iv>", V_STORE);
	if (In(sub, 0x54, 0x57)) return set("iv>v", V_LOADLANE);
	switch (sub) {
		case 0x0c: return set(">v", V_CONST);
		case 0x0d: case 0x0e: return set("vv>v", V_SINK);
		case 0x13: return set("f>v", V_SPLAT32);
		case 0x14: return set("d>v", V_SPLAT64);
		case 0x1f: return set("v>f", V_EXTRACT32);
		case 0x20: return set("vf>v", V_REPLACE32);
		case 0x21: return set("v>d", V_EXTRACT64);
		case 0x22: return set("vd>v", V_REPLACE64);
		case 0x4d: return set("v>v", V_SINK);
		case 0x52: return set("vvv>v", V_SINK);
		case 0x53: return set("v>i", V_SINK);
		case 0x5e: return set("v>v", V_DEMOTE);
		case 0x5f: return set("v>v", V_PROMOTE);
		case 0x74: case 0x75: case 0x7a: case 0x94: case 0xef: return set("v>v", V_ARITH64);
		case 0xe0: return set("v>v", V_ABS32);
		case 0xe1: return set("v>v", V_NEG32);
		case 0xe3: return set("v>v", V_ARITH32);
		case 0xea: case 0xeb: return set("vv>v", V_PMM32);
		case 0xec: return set("v>v", V_ABS64);
		case 0xed: return set("v>v", V_NEG64);
		case 0xf6: case 0xf7: return set("vv>v", V_PMM64);
		case 0xf8: case 0xf9: case 0x101: case 0x102: return set("v>v", V_TOI32);
		case 0xfa: case 0xfb: return set("v>v", V_ITOF32);
		case 0xfc: case 0xfd: case 0x103: case 0x104: return set("v>v", V_TOI64);
		case 0xfe: case 0xff: return set("v>v", V_ITOF64);
		case 0x100: return set("vv>v", V_SINK);
		case 0x105: case 0x106: return set("vvv>v", V_ARITH32);
		case 0x107: case 0x108: return set("vvv>v", V_ARITH64);
		case 0x109: case 0x10a: case 0x10b: case 0x10c: case 0x113: return set("vvv>v", V_SINK);
		case 0x10d: case 0x10e: return set("vv>v", V_ARITH32);
		case 0x10f: case 0x110: return set("vv>v", V_ARITH64);
		case 0x111: case 0x112: return set("vv>v", V_SINK);
		default: break;
	}
	if (In(sub, 0x0f, 0x11)) return set("i>v", V_SINK);
	if (sub == 0x12) return set("i>v", V_SINK);
	if (sub == 0x15 || sub == 0x16 || sub == 0x18 || sub == 0x19 || sub == 0x1b || sub == 0x1d)
		return set("v>i", V_SINK);
	if (sub == 0x17 || sub == 0x1a || sub == 0x1c || sub == 0x1e) return set("vi>v", V_SINK);
	if (In(sub, 0x23, 0x40)) return set("vv>v", V_SINK);
	if (In(sub, 0x41, 0x46)) return set("vv>v", V_CMP32);
	if (In(sub, 0x47, 0x4c)) return set("vv>v", V_CMP64);
	if (In(sub, 0x4e, 0x51)) return set("vv>v", V_SINK);
	if (In(sub, 0x60, 0x62)) return set("v>v", V_SINK);
	if (sub == 0x63 || sub == 0x64) return set("v>i", V_SINK);
	if (sub == 0x65 || sub == 0x66) return set("vv>v", V_SINK);
	if (In(sub, 0x67, 0x6a)) return set("v>v", V_ARITH32);
	if (In(sub, 0x6b, 0x6d)) return set("vi>v", V_SINK);
	if (In(sub, 0x6e, 0x73)) return set("vv>v", V_SINK);
	if (In(sub, 0x76, 0x79) || sub == 0x7b) return set("vv>v", V_SINK);
	if (In(sub, 0x7c, 0x7f)) return set("v>v", V_SINK);
	// i16x8
	if (sub == 0x80 || sub == 0x81 || In(sub, 0x87, 0x8a)) return set("v>v", V_SINK);
	if (sub == 0x83 || sub == 0x84) return set("v>i", V_SINK);
	if (In(sub, 0x8b, 0x8d)) return set("vi>v", V_SINK);
	if (sub == 0x82 || sub == 0x85 || sub == 0x86 || In(sub, 0x8e, 0x93) || In(sub, 0x95, 0x99) ||
		In(sub, 0x9b, 0x9f)) return set("vv>v", V_SINK);
	// i32x4
	if (sub == 0xa0 || sub == 0xa1 || In(sub, 0xa7, 0xaa)) return set("v>v", V_SINK);
	if (sub == 0xa3 || sub == 0xa4) return set("v>i", V_SINK);
	if (In(sub, 0xab, 0xad)) return set("vi>v", V_SINK);
	if (sub == 0xae || sub == 0xb1 || In(sub, 0xb5, 0xba) || In(sub, 0xbc, 0xbf)) return set("vv>v", V_SINK);
	// i64x2
	if (sub == 0xc0 || sub == 0xc1 || In(sub, 0xc7, 0xca)) return set("v>v", V_SINK);
	if (sub == 0xc3 || sub == 0xc4) return set("v>i", V_SINK);
	if (In(sub, 0xcb, 0xcd)) return set("vi>v", V_SINK);
	if (sub == 0xce || sub == 0xd1 || In(sub, 0xd5, 0xdf)) return set("vv>v", V_SINK);
	// f32x4 / f64x2 binary arithmetic
	if (In(sub, 0xe4, 0xe9)) return set("vv>v", V_ARITH32);
	if (In(sub, 0xf0, 0xf5)) return set("vv>v", V_ARITH64);
	return false;
}

bool IsArithmetic(const Sig& s)
{
	return s.kind == K_FARITH || s.kind == V_ARITH32 || s.kind == V_ARITH64 || s.kind == V_DEMOTE ||
		s.kind == V_PROMOTE;
}

void SkipMemarg(Reader& r)
{
	const std::uint32_t align = r.U32();
	if (align & 0x40)
		r.U32(); // memory index (multi-memory)
	r.Uleb(64);  // offset
}

std::int32_t ReadBlockType(Reader& r, const Module& m)
{
	const std::uint8_t b = r.Peek();
	if (b == 0x40) { r.Byte(); return -1; }
	if (IsValType(b)) { r.Byte(); return -2 - std::int32_t(ValType(b)); }
	const std::int64_t index = r.Sleb(33);
	if (index < 0 || std::uint64_t(index) >= m.types.size())
		throw Unsupported("block type is not modelled");
	return std::int32_t(index);
}

// Decodes one instruction. Throws Unsupported for anything the analysis does
// not model (exceptions, GC, atomics, typed function references, ...).
void DecodeInsn(Reader& r, const Module& m, Insn& in, std::vector<std::uint32_t>& targets)
{
	in.start = std::uint32_t(r.pos);
	const std::uint8_t op = r.Byte();
	in.op = op;
	switch (op) {
		case 0x00: case 0x01: case 0x05: case 0x0b: case 0x0f: case 0x1a: case 0x1b: case 0xd1:
			break;
		case 0x02: case 0x03: case 0x04:
			in.blockType = ReadBlockType(r, m);
			break;
		case 0x0c: case 0x0d: case 0x10: case 0x12: case 0x20: case 0x21: case 0x22: case 0x23:
		case 0x24: case 0x25: case 0x26: case 0x3f: case 0x40: case 0xd2:
			in.a = r.U32();
			break;
		case 0x0e: {
			const std::uint32_t n = r.U32();
			if (n > (1u << 20))
				throw Malformed("br_table too large");
			in.c = std::uint32_t(targets.size());
			for (std::uint32_t i = 0; i <= n; ++i)
				targets.push_back(r.U32());
			in.a = n + 1;
			break;
		}
		case 0x11: case 0x13:
			in.a = r.U32();
			in.b = r.U32();
			break;
		case 0x1c: {
			const std::uint32_t n = r.U32();
			if (n != 1)
				throw Unsupported("typed select arity");
			in.a = ValType(r.Byte());
			break;
		}
		case 0x41: r.Sleb(32); break;
		case 0x42: r.Sleb(64); break;
		case 0x43: in.a = r.Fixed32(); break;
		case 0x44: in.a = r.Fixed32(); in.c = r.Fixed32(); break;
		case 0xd0: {
			const std::uint8_t ht = r.Byte();
			if (ht != 0x70 && ht != 0x6f)
				throw Unsupported("heap type");
			break;
		}
		case 0xfc: {
			const std::uint32_t sub = r.U32();
			in.op = PFX_FC | sub;
			switch (sub) {
				case 0: case 1: case 2: case 3: case 4: case 5: case 6: case 7: break;
				case 8: r.U32(); r.U32(); break;
				case 9: case 13: r.U32(); break;
				case 10: case 12: case 14: r.U32(); r.U32(); break;
				case 11: case 15: case 16: case 17: r.U32(); break;
				default: throw Unsupported("0xfc instruction " + std::to_string(sub));
			}
			break;
		}
		case 0xfd: {
			const std::uint32_t sub = r.U32();
			in.op = PFX_FD | sub;
			Sig sig;
			if (sub > 0x113 || !SimdSig(sub, sig))
				throw Unsupported("SIMD instruction " + std::to_string(sub));
			if (In(sub, 0x00, 0x0b) || sub == 0x5c || sub == 0x5d)
				SkipMemarg(r);
			else if (In(sub, 0x54, 0x5b)) {
				SkipMemarg(r);
				r.Byte();
			} else if (sub == 0x0c || sub == 0x0d)
				r.Skip(16);
			else if (In(sub, 0x15, 0x22))
				in.a = r.Byte();
			break;
		}
		default:
			if (In(op, 0x28, 0x3e)) {
				SkipMemarg(r);
				break;
			}
			if (In(op, 0x45, 0xc4))
				break;
			throw Unsupported("instruction 0x" + std::to_string(op));
	}
	in.end = std::uint32_t(r.pos);
}

void SkipConstExpr(Reader& r, const Module& m)
{
	std::vector<std::uint32_t> targets;
	for (;;) {
		Insn in;
		DecodeInsn(r, m, in, targets);
		if (in.op == 0x0b)
			return;
	}
}

void SkipLimits(Reader& r)
{
	const std::uint8_t flags = r.Byte();
	r.Uleb(64);
	if (flags & 1)
		r.Uleb(64);
	if (flags & 8)
		r.U32();
}

// ---------------------------------------------------------------------------
// One function

enum Reason : std::uint8_t {
	R_STORE, R_REINTERPRET, R_SIGN, R_CALL, R_RETURN, R_GLOBAL, R_VECTOR, R_MERGE, R_COUNT
};

struct Val {
	Ty ty = INT;
	std::uint8_t st = 0;
	std::uint16_t out = 0;
	std::uint32_t prod = NONE;
};

class FunctionRewrite {
public:
	FunctionRewrite(const Module& m, const FuncType& sig, std::vector<Ty> localTypes,
		std::vector<Insn> insns, std::vector<std::uint32_t> targets)
		: m(m), sig(sig), localTypes(std::move(localTypes)), insns(std::move(insns)),
		  targets(std::move(targets))
	{
		floatSlot.assign(this->localTypes.size(), NONE);
		for (std::size_t i = 0; i < this->localTypes.size(); ++i) {
			if (this->localTypes[i] != INT) {
				floatSlot[i] = std::uint32_t(floatLocals.size());
				floatLocals.push_back(this->localTypes[i]);
			}
		}
		localFlag.assign(floatLocals.size(), 0);
		slotFlag.assign(this->insns.size(), 0);
	}

	std::uint32_t passes = 0;

	void Analyze()
	{
		ComputeLiveness();
		for (passes = 1;; ++passes) {
			if (passes > 64)
				throw Unsupported("NaN sink analysis did not converge");
			changed = false;
			RunPass();
			if (!changed)
				return;
		}
	}

	bool Changed() const
	{
		return std::any_of(canonOut.begin(), canonOut.end(), [](std::uint64_t v) { return v != 0; });
	}

	void Count(NanSinkStats& stats) const
	{
		// The last request for an output decides its reason.
		std::vector<std::pair<std::uint64_t, Reason>> last(reasons);
		std::stable_sort(last.begin(), last.end(),
			[](const auto& x, const auto& y) { return x.first < y.first; });
		for (std::size_t k = 0; k < last.size(); ++k) {
			if (k + 1 < last.size() && last[k + 1].first == last[k].first)
				continue;
			const auto [key, reason] = last[k];
			const std::uint32_t insn = std::uint32_t(key >> 8);
			if (!(canonOut[insn] & (std::uint64_t(1) << (key & 0xff))))
				continue;
			switch (reason) {
				case R_STORE: ++stats.storeSinks; break;
				case R_REINTERPRET: ++stats.reinterpretSinks; break;
				case R_SIGN: ++stats.signSinks; break;
				case R_CALL: ++stats.callSinks; break;
				case R_RETURN: ++stats.returnSinks; break;
				case R_GLOBAL: ++stats.globalSinks; break;
				case R_VECTOR: ++stats.vectorSinks; break;
				default: ++stats.mergeSinks; break;
			}
		}
	}

	// Appends the rewritten body (without its size prefix).
	void Emit(std::span<const std::uint8_t> body, std::size_t localsStart, std::size_t codeStart,
		std::uint32_t groupCount, std::vector<std::uint8_t>& out)
	{
		std::vector<std::uint8_t> code;
		code.reserve(body.size() - codeStart + 64);
		std::uint32_t nextLocal = std::uint32_t(localTypes.size());
		std::vector<Ty> added;
		std::array<std::uint32_t, 4> canonScratch{NONE, NONE, NONE, NONE};
		std::array<std::vector<std::uint32_t>, 4> spillScratch;
		auto newLocal = [&](Ty ty) {
			added.push_back(ty);
			return nextLocal++;
		};
		auto scratch = [&](Ty ty) {
			if (canonScratch[ty] == NONE)
				canonScratch[ty] = newLocal(ty);
			return canonScratch[ty];
		};
		auto emitCanon = [&](Ty ty, bool shape64) {
			const std::uint32_t s = scratch(ty);
			code.push_back(0x22); PutUleb(code, s);
			if (ty == F32) {
				code.push_back(0x43);
				code.insert(code.end(), {0x00, 0x00, 0xc0, 0x7f});
			} else if (ty == F64) {
				code.push_back(0x44);
				code.insert(code.end(), {0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf8, 0x7f});
			} else {
				code.push_back(0xfd); code.push_back(0x0c);
				for (int i = 0; i < (shape64 ? 2 : 4); ++i) {
					if (shape64) code.insert(code.end(), {0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf8, 0x7f});
					else code.insert(code.end(), {0x00, 0x00, 0xc0, 0x7f});
				}
			}
			code.push_back(0x20); PutUleb(code, s);
			code.push_back(0x20); PutUleb(code, s);
			if (ty == F32) code.push_back(0x5b);
			else if (ty == F64) code.push_back(0x61);
			else { code.push_back(0xfd); code.push_back(shape64 ? 0x47 : 0x41); }
			if (ty == V128) { code.push_back(0xfd); code.push_back(0x52); }
			else code.push_back(0x1b);
		};

		for (std::size_t i = 0; i < insns.size(); ++i) {
			const Insn& in = insns[i];
			code.insert(code.end(), body.begin() + codeStart + in.start, body.begin() + codeStart + in.end);
			const std::uint64_t mask = canonOut[i];
			if (mask == 0)
				continue;
			const std::vector<Ty> outs = OutputTypes(std::uint32_t(i));
			const std::size_t n = outs.size();
			unsigned lo = 0;
			while (!(mask & (std::uint64_t(1) << lo)))
				++lo;
			if (lo >= n)
				throw std::logic_error("NaN sink output index out of range");
			// Spill the outputs above the lowest one to canonicalize, then restore them.
			std::array<std::uint32_t, 4> depth{0, 0, 0, 0};
			std::vector<std::uint32_t> spilled(n, NONE);
			for (std::size_t k = n; k-- > lo + 1;) {
				const Ty ty = outs[k];
				auto& pool = spillScratch[ty];
				if (depth[ty] == pool.size())
					pool.push_back(newLocal(ty));
				spilled[k] = pool[depth[ty]++];
				code.push_back(0x21); PutUleb(code, spilled[k]);
			}
			for (std::size_t k = lo; k < n; ++k) {
				if (k > lo) { code.push_back(0x20); PutUleb(code, spilled[k]); }
				if (mask & (std::uint64_t(1) << k))
					emitCanon(outs[k], (canonShape64[i] >> k) & 1);
			}
		}

		if (localTypes.size() + added.size() > 50000)
			throw Unsupported("too many locals after rewriting");
		// Locals: the original groups, then one group per added local.
		PutUleb(out, groupCount + added.size());
		out.insert(out.end(), body.begin() + localsStart, body.begin() + codeStart);
		for (const Ty ty : added) {
			out.push_back(0x01);
			out.push_back(ty == F32 ? 0x7d : ty == F64 ? 0x7c : 0x7b);
		}
		out.insert(out.end(), code.begin(), code.end());
	}

private:
	enum FrameKind : std::uint8_t { BLOCK, LOOP, IF, FUNC };
	struct Frame {
		FrameKind kind = BLOCK;
		std::uint32_t insn = NONE;
		std::size_t height = 0;
		std::vector<Ty> params, results;
		bool unreachable = false, dead = false, elseSeen = false;
		bool hasExit = false, hasBack = false;
		std::vector<std::uint8_t> exitLocals, exitVals, backLocals, backVals;
		std::vector<std::uint8_t> entryLocals;
		std::vector<Val> entryParams;
	};
	struct Header {
		bool valid = false;
		std::vector<std::uint8_t> locals, vals;
	};

	const Module& m;
	const FuncType& sig;
	std::vector<Ty> localTypes;
	std::vector<Insn> insns;
	std::vector<std::uint32_t> targets;
	std::vector<std::uint32_t> floatSlot; // local index -> float local slot
	std::vector<Ty> floatLocals;
	std::vector<std::uint8_t> localFlag;
	std::vector<std::uint64_t> slotFlag;
	// Float locals live after a block's or if's end, or at a loop's header,
	// indexed by the opening instruction: a merge of a dead local is no conflict.
	std::size_t liveWords = 0;
	std::vector<std::uint32_t> liveIndex;
	std::vector<std::uint64_t> livePool;

	std::vector<std::uint64_t> canonOut, canonShape64;
	std::vector<std::uint8_t> canonTy;
	std::vector<std::pair<std::uint64_t, Reason>> reasons;
	std::vector<Header> headers;
	std::vector<Val> stack;
	std::vector<Frame> frames;
	std::vector<std::uint8_t> locals;
	bool changed = false;

	void BlockSig(std::int32_t bt, std::vector<Ty>& params, std::vector<Ty>& results) const
	{
		params.clear();
		results.clear();
		if (bt == -1)
			return;
		if (bt < -1) {
			results.push_back(Ty(-2 - bt));
			return;
		}
		params = m.types[bt].params;
		results = m.types[bt].results;
	}

	std::vector<Ty> OutputTypes(std::uint32_t i) const
	{
		const Insn& in = insns[i];
		std::vector<Ty> p, r;
		switch (in.op) {
			case 0x0b:
				BlockSig(insns[in.b].blockType, p, r);
				return r;
			case 0x03:
				BlockSig(in.blockType, p, r);
				return p;
			case 0x10:
				return m.types[m.funcTypes[in.a]].results;
			case 0x11:
				return m.types[in.a].results;
			default:
				return {Ty(canonTy[i])};
		}
	}

	void Need(Val& v, Reason reason)
	{
		if (!Tainted(v.ty, v.st))
			return;
		if (v.prod == NONE || v.out >= 64)
			throw Unsupported("NaN sink without a producer");
		const std::uint64_t bit = std::uint64_t(1) << v.out;
		canonOut[v.prod] |= bit;
		if (v.ty == V128 && v.st == VT64)
			canonShape64[v.prod] |= bit;
		canonTy[v.prod] = v.ty;
		reasons.emplace_back((std::uint64_t(v.prod) << 8) | v.out, reason);
		v.st = Canon(v.ty, v.st);
	}

	Val Pop()
	{
		if (stack.size() <= frames.back().height)
			throw Malformed("operand stack underflow");
		Val v = stack.back();
		stack.pop_back();
		return v;
	}
	void Push(Ty ty, std::uint8_t st, std::uint32_t prod, std::uint16_t out = 0)
	{
		stack.push_back(Val{ty, st, out, prod});
	}

	void InitialLocals()
	{
		locals.assign(floatLocals.size(), 0);
		for (std::size_t i = 0; i < sig.params.size(); ++i) {
			if (floatSlot[i] != NONE)
				locals[floatSlot[i]] = Top(sig.params[i]);
		}
	}

	void FlagLocal(std::size_t j)
	{
		if (!localFlag[j]) {
			localFlag[j] = 1;
			changed = true;
		} else if (!changed) {
			throw std::logic_error("NaN sink analysis: a flagged local still merges a NaN");
		}
	}
	void FlagSlot(std::uint32_t insn, std::size_t k)
	{
		if (k >= 64)
			throw Unsupported("too many block results");
		const std::uint64_t bit = std::uint64_t(1) << k;
		if (!(slotFlag[insn] & bit)) {
			slotFlag[insn] |= bit;
			changed = true;
		} else if (!changed) {
			throw std::logic_error("NaN sink analysis: a flagged block result still merges a NaN");
		}
	}

	// Values about to enter a flagged result slot are canonicalized first.
	void ApplySlotFlags(std::uint32_t insn, Val* vals, std::size_t n)
	{
		for (std::size_t k = 0; k < n && k < 64; ++k) {
			if (slotFlag[insn] & (std::uint64_t(1) << k))
				Need(vals[k], R_MERGE);
		}
	}

	const std::uint64_t* Live(std::uint32_t opener) const
	{
		return livePool.data() + std::size_t(liveIndex[opener]) * liveWords;
	}
	static bool Bit(const std::uint64_t* set, std::size_t j) { return (set[j / 64] >> (j % 64)) & 1; }

	// Backward liveness of float locals over the structured body, to a fixpoint
	// over loop headers.
	void ComputeLiveness()
	{
		liveWords = (floatLocals.size() + 63) / 64;
		liveIndex.assign(insns.size(), NONE);
		std::uint32_t openers = 0;
		for (std::size_t i = 0; i < insns.size(); ++i)
			if (In(insns[i].op, 0x02, 0x04))
				liveIndex[i] = openers++;
		livePool.assign(std::size_t(openers) * liveWords, 0);
		if (liveWords == 0)
			return;
		struct RFrame {
			std::uint32_t opener;
			std::vector<std::uint64_t> endLive, elseStart;
			bool sawElse = false;
		};
		std::vector<RFrame> rframes;
		std::vector<std::uint64_t> live(liveWords);
		auto target = [&](std::uint32_t depth, std::vector<std::uint64_t>& into, bool add) {
			if (depth >= rframes.size())
				throw Malformed("branch depth");
			const RFrame& f = rframes[rframes.size() - 1 - depth];
			const std::uint64_t* src = nullptr;
			if (f.opener == NONE) {
				if (!add) std::fill(into.begin(), into.end(), 0);
				return;
			}
			src = insns[f.opener].op == 0x03 ? Live(f.opener) : f.endLive.data();
			for (std::size_t w = 0; w < liveWords; ++w)
				into[w] = add ? into[w] | src[w] : src[w];
		};
		for (bool changed = true; changed;) {
			changed = false;
			rframes.clear();
			std::fill(live.begin(), live.end(), 0);
			for (std::size_t i = insns.size(); i-- > 0;) {
				const Insn& in = insns[i];
				switch (in.op) {
					case 0x0b:
						rframes.push_back(RFrame{in.b, live, {}, false});
						if (in.b != NONE && insns[in.b].op != 0x03)
							std::copy(live.begin(), live.end(), livePool.begin() + std::size_t(liveIndex[in.b]) * liveWords);
						break;
					case 0x05: {
						RFrame& f = rframes.back();
						f.elseStart = live;
						f.sawElse = true;
						live = f.endLive;
						break;
					}
					case 0x02: rframes.pop_back(); break;
					case 0x04: {
						const RFrame& f = rframes.back();
						const auto& other = f.sawElse ? f.elseStart : f.endLive;
						for (std::size_t w = 0; w < liveWords; ++w) live[w] |= other[w];
						rframes.pop_back();
						break;
					}
					case 0x03: {
						auto* header = livePool.data() + std::size_t(liveIndex[i]) * liveWords;
						for (std::size_t w = 0; w < liveWords; ++w) {
							if ((header[w] | live[w]) != header[w]) {
								header[w] |= live[w];
								changed = true;
							}
						}
						rframes.pop_back();
						break;
					}
					case 0x0c: target(in.a, live, false); break;
					case 0x0d: target(in.a, live, true); break;
					case 0x0e:
						target(targets[in.c], live, false);
						for (std::uint32_t k = 1; k < in.a; ++k)
							target(targets[in.c + k], live, true);
						break;
					case 0x00: case 0x0f: case 0x12: case 0x13:
						std::fill(live.begin(), live.end(), 0);
						break;
					case 0x20: case 0x21: case 0x22: {
						const std::uint32_t j = in.a < floatSlot.size() ? floatSlot[in.a] : NONE;
						if (j == NONE)
							break;
						if (in.op == 0x20) live[j / 64] |= std::uint64_t(1) << (j % 64);
						else live[j / 64] &= ~(std::uint64_t(1) << (j % 64));
						break;
					}
					default: break;
				}
			}
		}
	}

	void JoinInto(std::uint32_t insn, bool& has, std::vector<std::uint8_t>& accLocals,
		std::vector<std::uint8_t>& accVals, const std::vector<std::uint8_t>& curLocals, const Val* vals,
		std::size_t n)
	{
		const std::uint64_t* live = liveWords ? Live(insn) : nullptr;
		if (!has) {
			has = true;
			accLocals = curLocals;
			// A dead local's state is never read: the most precise one will do.
			for (std::size_t j = 0; j < accLocals.size(); ++j)
				if (!Bit(live, j)) accLocals[j] = 0;
			accVals.resize(n);
			for (std::size_t k = 0; k < n; ++k)
				accVals[k] = vals[k].st;
			return;
		}
		for (std::size_t j = 0; j < accLocals.size(); ++j) {
			if (!Bit(live, j)) {
				accLocals[j] = 0;
				continue;
			}
			std::uint8_t o;
			if (!Join(floatLocals[j], accLocals[j], curLocals[j], o)) {
				FlagLocal(j);
				o = Top(floatLocals[j]);
			}
			accLocals[j] = o;
		}
		for (std::size_t k = 0; k < n; ++k) {
			std::uint8_t o;
			if (!Join(vals[k].ty, accVals[k], vals[k].st, o)) {
				FlagSlot(insn, k);
				o = Top(vals[k].ty);
			}
			accVals[k] = o;
		}
	}

	void Branch(std::uint32_t depth)
	{
		if (depth >= frames.size())
			throw Malformed("branch depth");
		Frame& f = frames[frames.size() - 1 - depth];
		if (f.kind == FUNC) {
			Return();
			return;
		}
		const std::size_t n = (f.kind == LOOP ? f.params : f.results).size();
		if (stack.size() < n)
			throw Malformed("branch arity");
		Val* vals = stack.data() + stack.size() - n;
		ApplySlotFlags(f.insn, vals, n);
		if (f.kind == LOOP)
			JoinInto(f.insn, f.hasBack, f.backLocals, f.backVals, locals, vals, n);
		else
			JoinInto(f.insn, f.hasExit, f.exitLocals, f.exitVals, locals, vals, n);
	}

	void Return()
	{
		const std::size_t n = sig.results.size();
		if (stack.size() < n)
			throw Malformed("return arity");
		for (std::size_t k = stack.size() - n; k < stack.size(); ++k)
			Need(stack[k], R_RETURN);
	}

	void PushFrame(FrameKind kind, std::uint32_t insn)
	{
		Frame f;
		f.kind = kind;
		f.insn = insn;
		BlockSig(insns[insn].blockType, f.params, f.results);
		const Frame& parent = frames.back();
		if (parent.unreachable || parent.dead) {
			f.dead = f.unreachable = true;
			f.height = stack.size();
			frames.push_back(std::move(f));
			return;
		}
		if (stack.size() < parent.height + f.params.size())
			throw Malformed("block parameters");
		f.height = stack.size() - f.params.size();
		frames.push_back(std::move(f));
	}

	void EnterLoop(std::uint32_t i)
	{
		Frame& f = frames.back();
		const std::size_t n = f.params.size();
		Val* vals = stack.data() + f.height;
		ApplySlotFlags(i, vals, n);
		Header& h = headers[i];
		if (!h.valid) {
			h.valid = true;
			h.locals = locals;
			h.vals.resize(n);
			for (std::size_t k = 0; k < n; ++k)
				h.vals[k] = vals[k].st;
		} else {
			bool has = true;
			JoinInto(i, has, h.locals, h.vals, locals, vals, n);
		}
		StartLoopBody(i);
	}

	void StartLoopBody(std::uint32_t i)
	{
		Frame& f = frames.back();
		const Header& h = headers[i];
		stack.resize(f.height);
		for (std::size_t k = 0; k < f.params.size(); ++k)
			Push(f.params[k], h.vals[k], i, std::uint16_t(k));
		locals = h.locals;
		canonOut[i] = canonShape64[i] = 0;
		f.hasBack = false;
		f.unreachable = false;
	}

	// Returns the next instruction index.
	std::size_t End(std::size_t i)
	{
		if (frames.size() == 1) {
			if (!frames.back().unreachable)
				Return();
			frames.pop_back();
			return i + 1;
		}
		Frame& f = frames.back();
		if (f.kind == LOOP && !f.dead && f.hasBack) {
			Header& h = headers[f.insn];
			const std::vector<std::uint8_t> oldLocals = h.locals, oldVals = h.vals;
			bool has = true;
			// The back edges' values were canonicalized for flagged slots in Branch.
			std::vector<Val> backVals(f.params.size());
			for (std::size_t k = 0; k < backVals.size(); ++k)
				backVals[k] = Val{f.params[k], f.backVals[k], 0, NONE};
			JoinInto(f.insn, has, h.locals, h.vals, f.backLocals, backVals.data(), backVals.size());
			if (h.locals != oldLocals || h.vals != oldVals) {
				StartLoopBody(f.insn);
				return f.insn + 1;
			}
		}
		const bool fallthrough = !f.unreachable && !f.dead;
		const std::size_t n = f.results.size();
		if (fallthrough && stack.size() < f.height + n)
			throw Malformed("block results");
		Frame done = std::move(frames.back());
		frames.pop_back();
		if (done.dead)
			return i + 1;
		if (done.kind == LOOP) {
			if (!fallthrough) {
				stack.resize(done.height);
				frames.back().unreachable = true;
				return i + 1;
			}
			std::vector<std::uint8_t> states(n);
			for (std::size_t k = 0; k < n; ++k)
				states[k] = stack[stack.size() - n + k].st;
			stack.resize(done.height);
			for (std::size_t k = 0; k < n; ++k)
				Push(done.results[k], states[k], std::uint32_t(i), std::uint16_t(k));
			return i + 1;
		}
		if (fallthrough) {
			Val* vals = stack.data() + stack.size() - n;
			ApplySlotFlags(done.insn, vals, n);
			JoinInto(done.insn, done.hasExit, done.exitLocals, done.exitVals, locals, vals, n);
		}
		if (done.kind == IF && !done.elseSeen) {
			// The condition was false: the parameters flow to the results.
			ApplySlotFlags(done.insn, done.entryParams.data(), done.entryParams.size());
			JoinInto(done.insn, done.hasExit, done.exitLocals, done.exitVals, done.entryLocals,
				done.entryParams.data(), done.entryParams.size());
		}
		stack.resize(done.height);
		if (!done.hasExit) {
			frames.back().unreachable = true;
			return i + 1;
		}
		locals = done.exitLocals;
		for (std::size_t k = 0; k < n; ++k)
			Push(done.results[k], done.exitVals[k], std::uint32_t(i), std::uint16_t(k));
		return i + 1;
	}

	void Else()
	{
		Frame& f = frames.back();
		if (f.kind != IF)
			throw Malformed("else without if");
		if (f.dead)
			return;
		const std::size_t n = f.results.size();
		if (!f.unreachable) {
			if (stack.size() < f.height + n)
				throw Malformed("if results");
			Val* vals = stack.data() + stack.size() - n;
			ApplySlotFlags(f.insn, vals, n);
			JoinInto(f.insn, f.hasExit, f.exitLocals, f.exitVals, locals, vals, n);
		}
		f.elseSeen = true;
		f.unreachable = false;
		stack.resize(f.height);
		stack.insert(stack.end(), f.entryParams.begin(), f.entryParams.end());
		locals = f.entryLocals;
	}

	void Numeric(std::uint32_t i, const Insn& in, const Sig& s)
	{
		std::array<Val, 3> a;
		for (std::size_t k = s.nin; k-- > 0;)
			a[k] = Pop();
		auto view32 = [&](Val& v) { if (v.st == VT64) Need(v, R_VECTOR); };
		auto view64 = [&](Val& v) { if (v.st == VT32) Need(v, R_VECTOR); };
		std::uint8_t st = 0;
		switch (s.kind) {
			case K_PLAIN: case K_FPURE: st = 0; break;
			case K_FARITH: st = ST; break;
			case K_ITOF: st = SK; break;
			case K_F2I_BITS: Need(a[0], R_REINTERPRET); break;
			case K_I2F_BITS: st = SR; break;
			case K_ABS: st = a[0].st; break;
			case K_NEG: Need(a[0], R_SIGN); st = SR; break;
			case K_COPYSIGN: Need(a[0], R_SIGN); Need(a[1], R_SIGN); st = SR; break;
			case V_SINK:
				for (std::size_t k = 0; k < s.nin; ++k)
					if (a[k].ty == V128) Need(a[k], R_VECTOR);
				st = s.outTy == V128 ? VR : 0;
				break;
			case V_ARITH32: for (std::size_t k = 0; k < s.nin; ++k) view32(a[k]); st = VT32; break;
			case V_ARITH64: for (std::size_t k = 0; k < s.nin; ++k) view64(a[k]); st = VT64; break;
			case V_CMP32: view32(a[0]); view32(a[1]); st = VR; break;
			case V_CMP64: view64(a[0]); view64(a[1]); st = VR; break;
			case V_TOI32: view32(a[0]); st = VR; break;
			case V_TOI64: view64(a[0]); st = VR; break;
			case V_ITOF32: Need(a[0], R_VECTOR); st = VK32; break;
			case V_ITOF64: Need(a[0], R_VECTOR); st = VK64; break;
			case V_DEMOTE: view64(a[0]); st = VT32; break;
			case V_PROMOTE: view32(a[0]); st = VT64; break;
			case V_ABS32: view32(a[0]); st = Vec32(View32(a[0].st)); break;
			case V_ABS64: view64(a[0]); st = Vec64(View64(a[0].st)); break;
			case V_NEG32: view32(a[0]); Need(a[0], R_SIGN); st = VR; break;
			case V_NEG64: view64(a[0]); Need(a[0], R_SIGN); st = VR; break;
			case V_PMM32: case V_PMM64: {
				const bool is32 = s.kind == V_PMM32;
				if (is32) { view32(a[0]); view32(a[1]); } else { view64(a[0]); view64(a[1]); }
				auto view = [&](const Val& v) { return is32 ? View32(v.st) : View64(v.st); };
				std::uint8_t o;
				if (!Join(F32, view(a[0]), view(a[1]), o)) {
					Need(a[0], R_MERGE);
					Need(a[1], R_MERGE);
					Join(F32, view(a[0]), view(a[1]), o);
				}
				st = is32 ? Vec32(o) : Vec64(o);
				break;
			}
			case V_SPLAT32: st = Vec32(a[0].st); break;
			case V_SPLAT64: st = Vec64(a[0].st); break;
			case V_EXTRACT32: view32(a[0]); st = View32(a[0].st); break;
			case V_EXTRACT64: view64(a[0]); st = View64(a[0].st); break;
			case V_REPLACE32: case V_REPLACE64: {
				const bool is32 = s.kind == V_REPLACE32;
				if (is32) view32(a[0]); else view64(a[0]);
				auto view = [&]() { return is32 ? View32(a[0].st) : View64(a[0].st); };
				std::uint8_t o;
				if (!Join(F32, view(), a[1].st, o)) {
					Need(a[0], R_MERGE);
					Need(a[1], R_MERGE);
					Join(F32, view(), a[1].st, o);
				}
				st = is32 ? Vec32(o) : Vec64(o);
				break;
			}
			case V_LOAD: st = VR; break;
			case V_LOADLANE: Need(a[1], R_VECTOR); st = VR; break;
			case V_STORE: Need(a[1], R_STORE); break;
			case V_CONST: {
				const std::uint8_t* bytes = body.data() + codeStart + in.end - 16;
				bool k32 = true, k64 = true;
				for (int l = 0; l < 4; ++l) {
					std::uint32_t v;
					std::memcpy(&v, bytes + 4 * l, 4);
					k32 = k32 && CanonicalF32(v);
				}
				for (int l = 0; l < 2; ++l) {
					std::uint64_t v;
					std::memcpy(&v, bytes + 8 * l, 8);
					k64 = k64 && CanonicalF64(v);
				}
				st = k32 && k64 ? VK : k32 ? VK32 : k64 ? VK64 : VR;
				break;
			}
		}
		if (s.nout)
			Push(s.outTy, st, i);
	}

public:
	std::span<const std::uint8_t> body;
	std::size_t codeStart = 0;

private:
	void RunPass()
	{
		const std::size_t n = insns.size();
		canonOut.assign(n, 0);
		canonShape64.assign(n, 0);
		canonTy.assign(n, 0);
		reasons.clear();
		headers.assign(n, Header{});
		stack.clear();
		frames.clear();
		InitialLocals();
		Frame fn;
		fn.kind = FUNC;
		fn.results = sig.results;
		frames.push_back(std::move(fn));

		std::size_t i = 0;
		while (i < n) {
			const Insn& in = insns[i];
			canonOut[i] = canonShape64[i] = 0;
			if (frames.empty())
				throw Malformed("instructions after the function end");
			Frame& top = frames.back();
			const bool live = !top.unreachable && !top.dead;
			const std::uint32_t ii = std::uint32_t(i);
			switch (in.op) {
				case 0x02: PushFrame(BLOCK, ii); ++i; continue;
				case 0x03:
					PushFrame(LOOP, ii);
					if (!frames.back().dead)
						EnterLoop(ii);
					++i;
					continue;
				case 0x04: {
					if (live)
						Pop();
					PushFrame(IF, ii);
					Frame& f = frames.back();
					if (!f.dead) {
						f.entryLocals = locals;
						f.entryParams.assign(stack.begin() + f.height, stack.end());
					}
					++i;
					continue;
				}
				case 0x05: Else(); ++i; continue;
				case 0x0b: i = End(i); continue;
				default: break;
			}
			if (!live) {
				++i;
				continue;
			}
			switch (in.op) {
				case 0x00: top.unreachable = true; break;
				case 0x01: break;
				case 0x0c: Branch(in.a); frames.back().unreachable = true; break;
				case 0x0d: Pop(); Branch(in.a); break;
				case 0x0e: {
					Pop();
					std::vector<std::uint32_t> seen;
					for (std::uint32_t k = 0; k < in.a; ++k) {
						const std::uint32_t depth = targets[in.c + k];
						if (std::find(seen.begin(), seen.end(), depth) != seen.end())
							continue;
						seen.push_back(depth);
						Branch(depth);
					}
					frames.back().unreachable = true;
					break;
				}
				case 0x0f: Return(); top.unreachable = true; break;
				case 0x10: case 0x11: case 0x12: case 0x13: {
					const bool indirect = in.op == 0x11 || in.op == 0x13;
					if (indirect)
						Pop();
					const FuncType& ft = indirect ? m.types.at(in.a) : m.types.at(m.funcTypes.at(in.a));
					std::vector<Val> args(ft.params.size());
					for (std::size_t k = args.size(); k-- > 0;)
						args[k] = Pop();
					for (Val& v : args)
						Need(v, R_CALL);
					if (in.op == 0x12 || in.op == 0x13) {
						frames.back().unreachable = true;
						break;
					}
					for (std::size_t k = 0; k < ft.results.size(); ++k)
						Push(ft.results[k], Top(ft.results[k]), ii, std::uint16_t(k));
					break;
				}
				case 0x1a: Pop(); break;
				case 0x1b: case 0x1c: {
					Pop();
					Val b = Pop(), a = Pop();
					std::uint8_t o;
					if (!Join(a.ty, a.st, b.st, o)) {
						Need(a, R_MERGE);
						Need(b, R_MERGE);
						Join(a.ty, a.st, b.st, o);
					}
					Push(a.ty, o, ii);
					break;
				}
				case 0x20: {
					const Ty ty = localTypes.at(in.a);
					Push(ty, ty == INT ? 0 : locals[floatSlot[in.a]], ii);
					break;
				}
				case 0x21: case 0x22: {
					Val v = Pop();
					const Ty ty = localTypes.at(in.a);
					if (ty != INT) {
						const std::uint32_t j = floatSlot[in.a];
						if (localFlag[j])
							Need(v, R_MERGE);
						locals[j] = v.st;
					}
					if (in.op == 0x22)
						Push(ty, v.st, ii);
					break;
				}
				case 0x23: {
					const Ty ty = m.globals.at(in.a);
					Push(ty, Top(ty), ii);
					break;
				}
				case 0x24: {
					Val v = Pop();
					Need(v, R_GLOBAL);
					break;
				}
				case 0x25: Pop(); Push(INT, 0, ii); break;
				case 0x26: Pop(); Pop(); break;
				case 0x3f: Push(INT, 0, ii); break;
				case 0x40: Pop(); Push(INT, 0, ii); break;
				case 0x41: case 0x42: Push(INT, 0, ii); break;
				case 0x43: Push(F32, CanonicalF32(in.a) ? SK : SR, ii); break;
				case 0x44: Push(F64, CanonicalF64(std::uint64_t(in.c) << 32 | in.a) ? SK : SR, ii); break;
				case 0xd0: case 0xd2: Push(INT, 0, ii); break;
				case 0xd1: Pop(); Push(INT, 0, ii); break;
				default: {
					if (In(in.op, 0x28, 0x35)) {
						Pop();
						const Ty ty = in.op == 0x2a ? F32 : in.op == 0x2b ? F64 : INT;
						Push(ty, Top(ty), ii);
						break;
					}
					if (In(in.op, 0x36, 0x3e)) {
						Val v = Pop();
						Need(v, R_STORE);
						Pop();
						break;
					}
					if ((in.op & 0xff0000u) == PFX_FC && (in.op & 0xffffu) >= 8) {
						const std::uint32_t sub = in.op & 0xffffu;
						const std::size_t pops = sub == 9 || sub == 13 || sub == 16 ? 0 : sub == 15 ? 2 : 3;
						for (std::size_t k = 0; k < pops; ++k)
							Pop();
						if (sub == 15 || sub == 16)
							Push(INT, 0, ii);
						break;
					}
					Sig s;
					const bool simd = (in.op & 0xff0000u) == PFX_FD;
					if (simd ? !SimdSig(in.op & 0xffffu, s) : !NumericSig(in.op, s))
						throw Unsupported("instruction is not modelled");
					Numeric(ii, in, s);
					break;
				}
			}
			++i;
		}
		if (!frames.empty())
			throw Malformed("function body does not end");
	}
};

struct Section {
	std::uint8_t id;
	std::size_t start, payload, end; // section header start, payload start, end
	std::string_view name;           // custom sections
};

} // namespace

bool CanonicalizeNanAtSinks(std::span<const std::uint8_t> module, std::vector<std::uint8_t>& out,
	NanSinkStats* stats, std::string& error)
{
	NanSinkStats local;
	NanSinkStats& st = stats != nullptr ? *stats : local;
	st = {};
	try {
		static constexpr std::uint8_t header[8] = {0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00};
		if (module.size() < 8 || std::memcmp(module.data(), header, 8) != 0)
			throw Unsupported("not a Core Wasm module");
		Reader r{module.data(), module.size(), 8};
		std::vector<Section> sections;
		while (!r.AtEnd()) {
			Section s;
			s.start = r.pos;
			s.id = r.Byte();
			const std::uint32_t size = r.U32();
			s.payload = r.pos;
			r.Skip(size);
			s.end = r.pos;
			if (s.id == 0) {
				Reader n{module.data(), s.end, s.payload};
				const std::uint32_t len = n.U32();
				n.Skip(len);
				s.name = std::string_view(reinterpret_cast<const char*>(module.data()) + n.pos - len, len);
			}
			if (s.id == 13)
				throw Unsupported("exception tags");
			if (s.id > 13)
				throw Unsupported("section " + std::to_string(s.id));
			sections.push_back(s);
		}

		Module m;
		std::vector<std::uint32_t> definedTypes;
		const Section* code = nullptr;
		for (const Section& s : sections) {
			Reader p{module.data(), s.end, s.payload};
			switch (s.id) {
				case 1: {
					const std::uint32_t n = p.U32();
					for (std::uint32_t i = 0; i < n; ++i) {
						if (p.Byte() != 0x60)
							throw Unsupported("non-function type");
						FuncType ft;
						for (std::uint32_t k = p.U32(); k > 0; --k) ft.params.push_back(ValType(p.Byte()));
						for (std::uint32_t k = p.U32(); k > 0; --k) ft.results.push_back(ValType(p.Byte()));
						m.types.push_back(std::move(ft));
					}
					break;
				}
				case 2: {
					const std::uint32_t n = p.U32();
					for (std::uint32_t i = 0; i < n; ++i) {
						p.Skip(p.U32());
						p.Skip(p.U32());
						switch (p.Byte()) {
							case 0:
								m.funcTypes.push_back(p.U32());
								++m.importedFuncs;
								break;
							case 1: ValType(p.Byte()); SkipLimits(p); break;
							case 2: SkipLimits(p); break;
							case 3: m.globals.push_back(ValType(p.Byte())); p.Byte(); break;
							default: throw Unsupported("import kind");
						}
					}
					break;
				}
				case 3: {
					const std::uint32_t n = p.U32();
					for (std::uint32_t i = 0; i < n; ++i)
						definedTypes.push_back(p.U32());
					break;
				}
				case 6: {
					const std::uint32_t n = p.U32();
					for (std::uint32_t i = 0; i < n; ++i) {
						m.globals.push_back(ValType(p.Byte()));
						p.Byte();
						SkipConstExpr(p, m);
					}
					break;
				}
				case 10: code = &s; break;
				default: break;
			}
		}
		m.funcTypes.insert(m.funcTypes.end(), definedTypes.begin(), definedTypes.end());
		for (const std::uint32_t t : m.funcTypes)
			if (t >= m.types.size())
				throw Malformed("function type index");

		std::vector<std::uint8_t> newCode;
		bool anyChange = false;
		if (code != nullptr) {
			Reader p{module.data(), code->end, code->payload};
			const std::uint32_t count = p.U32();
			if (count != definedTypes.size())
				throw Malformed("function and code section counts differ");
			PutUleb(newCode, count);
			for (std::uint32_t f = 0; f < count; ++f) {
				const std::uint32_t size = p.U32();
				const std::size_t bodyStart = p.pos;
				p.Skip(size);
				const std::span<const std::uint8_t> body = module.subspan(bodyStart, size);
				Reader b{body.data(), body.size(), 0};
				const FuncType& ft = m.types[definedTypes[f]];
				std::vector<Ty> localTypes = ft.params;
				const std::uint32_t groups = b.U32();
				const std::size_t localsStart = b.pos;
				for (std::uint32_t g = 0; g < groups; ++g) {
					const std::uint32_t n = b.U32();
					const Ty ty = ValType(b.Byte());
					if (localTypes.size() + n > 50000)
						throw Unsupported("too many locals");
					localTypes.insert(localTypes.end(), n, ty);
				}
				const std::size_t codeStart = b.pos;
				Reader c{body.data() + codeStart, body.size() - codeStart, 0};
				std::vector<Insn> insns;
				std::vector<std::uint32_t> targets, open;
				bool ended = false;
				while (!c.AtEnd()) {
					if (ended)
						throw Malformed("bytes after the function end");
					Insn in;
					DecodeInsn(c, m, in, targets);
					const std::uint32_t idx = std::uint32_t(insns.size());
					if (in.op == 0x02 || in.op == 0x03 || in.op == 0x04)
						open.push_back(idx);
					else if (in.op == 0x05) {
						if (open.empty() || insns[open.back()].op != 0x04)
							throw Malformed("else without if");
						insns[open.back()].c = idx;
					} else if (in.op == 0x0b) {
						if (open.empty())
							ended = true;
						else {
							in.b = open.back();
							insns[open.back()].b = idx;
							open.pop_back();
						}
					}
					if (In(in.op, 0x02, 0x04) || in.op == 0x0b || in.op == 0x05 || in.op == 0x0c ||
						in.op == 0x0d || in.op == 0x0e || in.op == 0x0f || in.op == 0x12 || in.op == 0x13) {
						// structural
					} else {
						Sig s;
						if (((in.op & 0xff0000u) == PFX_FD && SimdSig(in.op & 0xffffu, s) && IsArithmetic(s)) ||
							(in.op < 0x100 && NumericSig(in.op, s) && IsArithmetic(s)))
							++st.arithmeticSites;
					}
					insns.push_back(in);
				}
				if (!ended)
					throw Malformed("function body does not end");

				FunctionRewrite rw(m, ft, std::move(localTypes), std::move(insns), std::move(targets));
				rw.body = body;
				rw.codeStart = codeStart;
				rw.Analyze();
				++st.functions;
				st.analysisPasses += rw.passes;
				std::vector<std::uint8_t> newBody;
				if (rw.Changed()) {
					anyChange = true;
					rw.Count(st);
					rw.Emit(body, localsStart, codeStart, groups, newBody);
				} else {
					newBody.assign(body.begin(), body.end());
				}
				PutUleb(newCode, newBody.size());
				newCode.insert(newCode.end(), newBody.begin(), newBody.end());
			}
			if (!p.AtEnd())
				throw Malformed("trailing bytes in the code section");
		}

		out.clear();
		if (!anyChange) {
			out.assign(module.begin(), module.end());
			return true;
		}
		out.reserve(module.size() + module.size() / 8);
		out.insert(out.end(), module.begin(), module.begin() + 8);
		for (const Section& s : sections) {
			if (&s == code) {
				out.push_back(10);
				PutUleb(out, newCode.size());
				out.insert(out.end(), newCode.begin(), newCode.end());
				continue;
			}
			// Custom sections that address code by byte offset no longer match it.
			if (s.id == 0 && (s.name.starts_with(".debug") || s.name.starts_with("metadata.code.")))
				continue;
			out.insert(out.end(), module.begin() + s.start, module.begin() + s.end);
		}
		return true;
	} catch (const std::exception& e) {
		error = e.what();
		out.clear();
		return false;
	}
}

} // namespace recoil::wasm
