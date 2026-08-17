#![allow(clippy::all)]

pub(crate) mod bindings {
    wit_bindgen::generate!({
        path: "wit",
        world: "ui",
    });
}

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use bindings::exports::recoil::spring_api::callins_ui::{Guest, SimpleCallinQuery, SpringError};
use bindings::recoil::spring_api::{gfx, messages, profiling};

static RAN: AtomicBool = AtomicBool::new(false);
static VERTICES: AtomicUsize = AtomicUsize::new(0);

const DRAW_BATCHES: usize = 8;

fn scale() -> f64 {
    option_env!("SPRING_BENCHMARK_SCALE")
        .and_then(|value| value.parse::<f64>().ok())
        .map(|value| value.clamp(0.0001, 1.0))
        .unwrap_or(1.0)
}

fn count(value: usize) -> usize {
    ((value as f64 * scale()).round() as usize).max(1)
}

fn timer_micros() -> Result<u64, SpringError> {
    profiling::get_timer_micros(0u8).map_err(|error| SpringError { code: error.code })
}

fn emit_vertices(_user_data: u32) {
    let count = VERTICES.load(Ordering::Relaxed);
    for index in 0..count {
        let x = (index % 100) as f32 * 0.01;
        let y = (index / 100) as f32 * 0.01;
        let _ = gfx::vertex(x, y, 0.0, 1.0, 4);
    }
}

fn send_row(row: &str) -> Result<(), SpringError> {
    messages::send_lua_ui_msg(&format!("WASM_DRAW|{row}"), "")
        .map(|_| ())
        .map_err(|error| SpringError { code: error.code })
}

fn run_draw() -> Result<(), SpringError> {
    let mut callout_samples = Vec::new();
    let callout_vertices = count(100_000);
    for _ in 0..5 {
        VERTICES.store(callout_vertices, Ordering::Relaxed);
        let start = timer_micros()?;
        for _ in 0..DRAW_BATCHES {
            gfx::begin_end(1, 1, 0).map_err(|error| SpringError { code: error.code })?;
        }
        let elapsed = timer_micros()?.saturating_sub(start);
        callout_samples.push(
            elapsed as f64 * 1000.0 / (callout_vertices * DRAW_BATCHES) as f64,
        );
    }
    callout_samples.sort_by(|left, right| left.total_cmp(right));
    let callout_median = callout_samples[(callout_samples.len() - 1) / 2];
    send_row(&format!(
        "{{\"backend\":\"wasm\",\"test\":\"callout_draw\",\"status\":\"pass\",\"iterations\":{callout_vertices},\"batches\":{DRAW_BATCHES},\"medianNs\":{callout_median:.3},\"scale\":{}}}",
        scale()
    ))?;

    let mut workload_samples = Vec::new();
    let workload_vertices = count(4_000);
    for _ in 0..5 {
        VERTICES.store(workload_vertices, Ordering::Relaxed);
        let start = timer_micros()?;
        for _ in 0..DRAW_BATCHES {
            gfx::begin_end(1, 1, 0).map_err(|error| SpringError { code: error.code })?;
        }
        let elapsed = timer_micros()?.saturating_sub(start);
        workload_samples.push(elapsed as f64 / 1000.0 / DRAW_BATCHES as f64);
    }
    workload_samples.sort_by(|left, right| left.total_cmp(right));
    let workload_median = workload_samples[(workload_samples.len() - 1) / 2];
    send_row(&format!(
        "{{\"backend\":\"wasm\",\"test\":\"wl_ui_draw\",\"status\":\"pass\",\"lines\":{},\"batches\":{DRAW_BATCHES},\"medianMs\":{workload_median:.6},\"scale\":{}}}",
        workload_vertices / 2,
        scale()
    ))?;
    send_row("{\"backend\":\"wasm\",\"test\":\"complete\",\"status\":\"pass\"}")
}

struct BenchmarkUiGuest;

impl Guest for BenchmarkUiGuest {
    fn draw_world(_query: SimpleCallinQuery) -> Result<(), SpringError> {
        if !RAN.swap(true, Ordering::AcqRel) {
            run_draw()?;
        }
        Ok(())
    }
}

impl bindings::Guest for BenchmarkUiGuest {
    fn callback_1(_user_data: u32) {
        emit_vertices(0);
    }
}

bindings::export!(BenchmarkUiGuest with_types_in bindings);
