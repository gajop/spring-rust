// Nondeterministic imports, available in every environment including synced.
//
// Reading a wall clock from synced code makes the simulation diverge between
// clients. That is the whole point of the group: debugging and benchmarking
// synced code needs a clock, and a guest may choose to trade determinism for
// one. The deterministic API surface has no equivalent, and `spring:profiling`
// keeps the same timers unsynced-only.
//
// This module is deliberately NOT re-exported from the crate root, so reaching
// it always means writing `spring::desync::` at the call site.

use super::{ApiError, Result};

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:desync")]
    unsafe extern "C" {
        #[link_name = "try-get-timer"]
        pub safe fn get_timer(output: i32) -> i32;
        #[link_name = "try-get-timer-micros"]
        pub safe fn get_timer_micros(output: i32) -> i32;
        #[link_name = "diff-timers"]
        pub safe fn diff_timers(
            end_timer: i64,
            start_timer: i64,
            return_ms: i32,
            from_micro_secs: i32,
        ) -> i64;
    }
}

/// Host timer ticks. Desyncs synced guests when explicitly enabled.
/// Returns `NotAvailable` when synced timing is disabled; no clock is read.
#[inline]
pub fn get_timer() -> Result<u64> {
    #[cfg(target_arch = "wasm32")]
    {
        let mut timer = 0u64;
        let status = raw::get_timer((&raw mut timer) as usize as i32);
        if status != 0 {
            return Err(ApiError::new(status));
        }
        Ok(timer)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Err(unreachable!())
    }
}

/// Host timer in microseconds. Desyncs synced guests when explicitly enabled.
/// Returns `NotAvailable` when synced timing is disabled; no clock is read.
#[inline]
pub fn get_timer_micros() -> Result<u64> {
    #[cfg(target_arch = "wasm32")]
    {
        let mut timer = 0u64;
        let status = raw::get_timer_micros((&raw mut timer) as usize as i32);
        if status != 0 {
            return Err(ApiError::new(status));
        }
        Ok(timer)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Err(unreachable!())
    }
}

/// Difference between two timers. Desyncs synced guests.
#[inline]
pub fn diff_timers(
    end_timer: u64,
    start_timer: u64,
    return_ms: bool,
    from_micro_secs: bool,
) -> Result<f32> {
    #[cfg(target_arch = "wasm32")]
    {
        let packed = raw::diff_timers(
            end_timer as i64,
            start_timer as i64,
            return_ms as i32,
            from_micro_secs as i32,
        );
        let status = (packed >> 32) as i32;
        if status != 0 {
            return Err(ApiError::new(status));
        }
        Ok(f32::from_bits(packed as u32))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (end_timer, start_timer, return_ms, from_micro_secs);
        Err(unreachable!())
    }
}
