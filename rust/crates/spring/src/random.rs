//! Synced randomness: the engine RNG that synced Lua's `math.random` draws
//! from.
//!
//! Every client draws the same sequence, so synced rules code can use it
//! freely, and the `FixedRNGSeed` start script key pins it for reproducible
//! runs. It is only importable from the synced environments (`rules-synced`,
//! `gaia-synced`); unsynced code has no synced RNG, as in Lua.

use crate::generated::synced_random;

/// A float in `[0, 1)`, like `math.random()`.
#[inline]
pub fn float() -> f32 {
    synced_random::next_float(0).expect("synced random float")
}

/// An integer in `[lower, upper]`, like `math.random(lower, upper)`.
///
/// Panics when `lower > upper`, where Lua raises an error.
#[inline]
pub fn int(lower: i32, upper: i32) -> i32 {
    assert!(
        lower <= upper,
        "empty interval in random::int({lower}, {upper})"
    );
    synced_random::next_int(lower, upper).expect("synced random int")
}

/// An integer in `[1, upper]`, like `math.random(upper)`.
///
/// Panics when `upper < 1`, where Lua raises an error.
#[inline]
pub fn up_to(upper: i32) -> i32 {
    assert!(upper >= 1, "upper limit below 1 in random::up_to({upper})");
    synced_random::next_int_up_to(upper).expect("synced random int")
}

/// A float in `[lower, upper)`.
#[inline]
pub fn range(lower: f32, upper: f32) -> f32 {
    lower + (upper - lower) * float()
}

/// Reseed the synced RNG, like `math.randomseed(seed)`. This changes the
/// sequence for every synced consumer, the engine included.
#[inline]
pub fn seed(seed: i32) {
    synced_random::set_seed(seed).expect("synced random seed");
}
