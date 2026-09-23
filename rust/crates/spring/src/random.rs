//! Engine random numbers with Lua `math.random` / `math.randomseed` semantics.
//!
//! - [`synced`]: the synced RNG that synced Lua's `math.random` draws from.
//!   Every client draws the same sequence and `FixedRNGSeed` pins it. Only
//!   importable from `rules-synced` and `gaia-synced`.
//! - [`unsynced`]: the engine's unsynced RNG, for local effects such as
//!   particles, lights and sound variation. Draws differ between clients, so
//!   never feed them into game state. Only importable from `rules-unsynced`,
//!   `gaia-unsynced` and `ui`.
//!
//! A module that uses the wrong one fails to load with an unknown-import error.

macro_rules! random_api {
    ($generated:ident, $what:literal) => {
        use crate::generated::$generated as api;

        /// A float in `[0, 1)`, like `math.random()`.
        #[inline]
        pub fn float() -> f32 {
            api::next_float(0).expect(concat!($what, " random float"))
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
            api::next_int(lower, upper).expect(concat!($what, " random int"))
        }

        /// An integer in `[1, upper]`, like `math.random(upper)`.
        ///
        /// Panics when `upper < 1`, where Lua raises an error.
        #[inline]
        pub fn up_to(upper: i32) -> i32 {
            assert!(upper >= 1, "upper limit below 1 in random::up_to({upper})");
            api::next_int_up_to(upper).expect(concat!($what, " random int"))
        }

        /// A float in `[lower, upper)`.
        #[inline]
        pub fn range(lower: f32, upper: f32) -> f32 {
            lower + (upper - lower) * float()
        }

        /// Reseed the RNG, like `math.randomseed(seed)`. This changes the
        /// sequence for every other consumer of this RNG, the engine included.
        #[inline]
        pub fn seed(seed: i32) {
            api::set_seed(seed).expect(concat!($what, " random seed"));
        }
    };
}

/// The synced RNG (`gsRNG`); see the [module docs](self).
pub mod synced {
    random_api!(synced_random, "synced");
}

/// The unsynced RNG (`guRNG`); see the [module docs](self).
pub mod unsynced {
    random_api!(unsynced_random, "unsynced");
}
