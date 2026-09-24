/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

//! PCG32 (XSH RR, <https://www.pcg-random.org>): the module-local generator
//! behind `random::unsynced`.

const MULTIPLIER: u64 = 6_364_136_223_846_793_005;
const INCREMENT: u64 = 1_442_695_040_888_963_407;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Pcg32(pub(crate) u64);

impl Pcg32 {
    /// The reference PCG32 seeding: advance once around adding the seed.
    pub(crate) fn seeded(seed: i32) -> Self {
        let mut rng = Self(0);
        rng.step();
        rng.0 = rng.0.wrapping_add(u64::from(seed as u32));
        rng.step();
        rng
    }

    fn step(&mut self) {
        self.0 = self.0.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT);
    }

    pub(crate) fn next_u32(&mut self) -> u32 {
        let old = self.0;
        self.step();
        let xorshifted = (((old >> 18) ^ old) >> 27) as u32;
        xorshifted.rotate_right((old >> 59) as u32)
    }

    /// A float in `[0, 1)`.
    pub(crate) fn float(&mut self) -> f32 {
        (self.next_u32() >> 8) as f32 * (1.0 / 16_777_216.0)
    }

    /// An integer in `[lower, upper]`; `lower <= upper`.
    pub(crate) fn int(&mut self, lower: i32, upper: i32) -> i32 {
        let span = (i64::from(upper) - i64::from(lower) + 1) as u64;
        let offset = (u64::from(self.next_u32()) * span) >> 32;
        (i64::from(lower) + offset as i64) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Pcg32;

    #[test]
    fn a_seed_repeats_its_sequence() {
        let mut a = Pcg32::seeded(42);
        let mut b = Pcg32::seeded(42);
        let mut c = Pcg32::seeded(43);
        let first: [u32; 4] = core::array::from_fn(|_| a.next_u32());
        assert_eq!(first, core::array::from_fn(|_| b.next_u32()));
        assert_ne!(first, core::array::from_fn(|_| c.next_u32()));
    }

    #[test]
    fn draws_stay_in_range() {
        let mut rng = Pcg32::seeded(7);
        for _ in 0..10_000 {
            assert!((0.0..1.0).contains(&rng.float()));
            assert!((-3..=3).contains(&rng.int(-3, 3)));
            assert!((1..=6).contains(&rng.int(1, 6)));
            let _ = rng.int(i32::MIN, i32::MAX);
        }
        let mut rng = Pcg32::seeded(0);
        let mut seen = [false; 6];
        for _ in 0..1000 {
            seen[(rng.int(1, 6) - 1) as usize] = true;
        }
        assert!(seen.iter().all(|seen| *seen));
    }
}
