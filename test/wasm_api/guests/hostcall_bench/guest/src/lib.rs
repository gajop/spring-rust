#![allow(clippy::all)]

mod bindings {
    wit_bindgen::generate!({
        path: "../wit",
        world: "bench",
    });
}

use bindings::recoil::hostcall_bench::host;
use bindings::recoil::hostcall_bench::types::{PositionOptions, Sample};
use bindings::Guest;

struct BenchGuest;

impl Guest for BenchGuest {
    fn run_callout(iters: i32) -> i32 {
        let mut acc = 0i32;
        for _ in 0..iters {
            acc = host::add_i32(acc);
        }
        acc
    }

    fn run_callout_result(iters: i32) -> i32 {
        let mut acc = 0i32;
        for _ in 0..iters {
            acc = match host::add_i32_result(acc) {
                Ok(value) => value,
                Err(error) => return error.code,
            };
        }
        acc
    }

    fn run_callout_vec3(iters: i32) -> i32 {
        let mut acc = 0i32;
        for _ in 0..iters {
            acc = match host::get_vec3(acc) {
                Ok(value) => value.x as i32,
                Err(error) => return error.code,
            };
        }
        acc
    }

    fn run_callout_vec3_opts(iters: i32) -> i32 {
        let mut acc = 0i32;
        for _ in 0..iters {
            let options = PositionOptions { mid_pos: false, aim_pos: false };
            acc = match host::get_vec3_opts(acc, options) {
                Ok(value) => value.x as i32,
                Err(error) => return error.code,
            };
        }
        acc
    }

    fn run_spin(iters: i32) -> i32 {
        let mut acc = 0i32;
        for _ in 0..iters {
            acc = std::hint::black_box(acc.wrapping_add(1));
        }
        acc
    }

    fn noop(x: i32) -> i32 {
        x
    }

    fn noop_record(value: Sample) -> i32 {
        value.a
    }
}

bindings::export!(BenchGuest with_types_in bindings);
