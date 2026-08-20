// Nondeterministic imports, available in every environment including synced.
//
// Reading a wall clock from synced code makes the simulation diverge between
// clients. That is the whole point of the group: debugging and benchmarking
// synced code needs a clock, and a guest may choose to trade determinism for
// one. The deterministic API surface has no equivalent, and `spring:profiling`
// keeps the same timers unsynced-only.
//
// This module is deliberately NOT re-exported from the crate root, so reaching
// it always means writing `spring_wasm_core::desync::` at the call site.

use super::{ApiError, ErrorCode, Result};

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:desync")]
    extern "C" {
        #[link_name = "get-timer"]
        pub fn get_timer() -> i64;
        #[link_name = "get-timer-micros"]
        pub fn get_timer_micros() -> i64;
        #[link_name = "diff-timers"]
        pub fn diff_timers(
            end_timer: i64,
            start_timer: i64,
            return_ms: i32,
            from_micro_secs: i32,
        ) -> i64;
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[inline]
fn unsupported<T>() -> Result<T> {
    Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
}

/// Host timer ticks. Desyncs synced guests.
#[inline]
pub fn get_timer() -> Result<u64> {
    #[cfg(target_arch = "wasm32")]
    {
        return Ok(unsafe { raw::get_timer() } as u64);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        unsupported()
    }
}

/// Host timer in microseconds. Desyncs synced guests.
#[inline]
pub fn get_timer_micros() -> Result<u64> {
    #[cfg(target_arch = "wasm32")]
    {
        return Ok(unsafe { raw::get_timer_micros() } as u64);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        unsupported()
    }
}

/// Difference between two timers. Desyncs synced guests.
#[inline]
pub fn diff_timers(end_timer: u64, start_timer: u64, return_ms: bool, from_micro_secs: bool) -> Result<f32> {
    #[cfg(target_arch = "wasm32")]
    {
        let packed = unsafe {
            raw::diff_timers(
                end_timer as i64,
                start_timer as i64,
                return_ms as i32,
                from_micro_secs as i32,
            )
        };
        let status = (packed >> 32) as i32;
        if status != 0 {
            return Err(ApiError::new(status));
        }
        return Ok(f32::from_bits(packed as u32));
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (end_timer, start_timer, return_ms, from_micro_secs);
        unsupported()
    }
}
