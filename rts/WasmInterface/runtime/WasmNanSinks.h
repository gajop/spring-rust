/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <span>
#include <string>
#include <vector>

// NaN canonicalization at the points where a NaN's bits become observable.
//
// Wasm leaves the payload and sign of an arithmetic NaN result nondeterministic,
// and x86 and ARM do pick different bits. Cranelift's global canonicalization
// (Config::cranelift_nan_canonicalization) replaces the result of every float
// operation that may produce a NaN with the canonical NaN. The payload is only
// observable when the value leaves the operations that treat every NaN alike.
// This pass rewrites a Core module so that a value that may carry such a NaN is
// canonicalized only where it reaches one of these sinks:
//   - a store to linear memory (scalar, v128 or lane store);
//   - a reinterpretation as an integer (i32/i64.reinterpret_f32/f64);
//   - a sign-bit operation that would carry the NaN's sign or payload into a
//     result (fneg, copysign's operands);
//   - a call argument, a function result, a global.set;
//   - any v128 operation that does not treat the vector as lanes of the same
//     float shape (integer, bitwise, shuffle, bitmask, ... and the other shape);
//   - a merge (select, local, block result) with a value whose bits must stay
//     exact (a load, a parameter, a reinterpretation), which a later sink could
//     not tell apart.
// Arithmetic, min/max, comparisons and float-to-integer conversions depend only
// on whether a value is a NaN, so a NaN passes through them unchanged.
//
// The observable behaviour (memory, globals, call arguments and results) equals
// that of Cranelift's global canonicalization, with the module compiled without
// it. The pass is deterministic: the same input bytes give the same output.
namespace recoil::wasm {

struct NanSinkStats {
	std::uint32_t functions = 0;
	// Operations that global canonicalization checks (float arithmetic results).
	std::uint32_t arithmeticSites = 0;
	// Canonicalization sequences this pass inserted, by the reason.
	std::uint32_t storeSinks = 0;
	std::uint32_t reinterpretSinks = 0;
	std::uint32_t signSinks = 0;
	std::uint32_t callSinks = 0;
	std::uint32_t returnSinks = 0;
	std::uint32_t globalSinks = 0;
	std::uint32_t vectorSinks = 0;
	std::uint32_t mergeSinks = 0;
	std::uint32_t analysisPasses = 0;

	std::uint32_t Inserted() const
	{
		return storeSinks + reinterpretSinks + signSinks + callSinks + returnSinks + globalSinks +
			vectorSinks + mergeSinks;
	}
};

// Rewrites `module`. On failure (a malformed module, or an instruction or
// feature the pass does not model) returns false with `error` set; the caller
// must then compile the original module with global canonicalization.
bool CanonicalizeNanAtSinks(std::span<const std::uint8_t> module, std::vector<std::uint8_t>& out,
	NanSinkStats* stats, std::string& error);

// Version of the rule set; part of the runtime's configuration identity.
inline constexpr const char* NAN_SINKS_VERSION = "sinks-v1";

}
