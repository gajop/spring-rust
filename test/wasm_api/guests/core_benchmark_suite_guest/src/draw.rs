use std::sync::atomic::{AtomicUsize, Ordering};

use spring_wasm_core as spring;

use crate::common;

static VERTICES: AtomicUsize = AtomicUsize::new(0);
const DRAW_BATCHES: usize = 8;

pub fn callback() {
    let count = VERTICES.load(Ordering::Relaxed);
    for index in 0..count {
        let x = (index % 100) as f32 * 0.01;
        let y = (index / 100) as f32 * 0.01;
        let _ = spring::gfx_vertex(x, y, 0.0, 1.0, 4);
    }
}

pub fn run() -> spring::Result<()> {
    let mut callout_samples = Vec::new();
    let callout_vertices = common::count(100_000);
    for _ in 0..5 {
        VERTICES.store(callout_vertices, Ordering::Relaxed);
        let start = common::timer_micros()?;
        for _ in 0..DRAW_BATCHES {
            spring::gfx_begin_end(1, 2, 0)?;
        }
        let elapsed = common::timer_micros()?.saturating_sub(start);
        callout_samples.push(elapsed as f64 * 1_000.0 / (callout_vertices * DRAW_BATCHES) as f64);
    }
    callout_samples.sort_by(|left, right| left.total_cmp(right));
    let callout_median = callout_samples[(callout_samples.len() - 1) / 2];
    common::send_draw_row(&format!(
        "{{\"backend\":\"wasm_core\",\"test\":\"callout_draw\",\"status\":\"pass\",\"iterations\":{callout_vertices},\"batches\":{DRAW_BATCHES},\"medianNs\":{callout_median:.3},\"scale\":{}}}",
        common::scale()
    ));

    let mut workload_samples = Vec::new();
    let workload_vertices = common::count(4_000);
    for _ in 0..5 {
        VERTICES.store(workload_vertices, Ordering::Relaxed);
        let start = common::timer_micros()?;
        for _ in 0..DRAW_BATCHES {
            spring::gfx_begin_end(1, 2, 0)?;
        }
        let elapsed = common::timer_micros()?.saturating_sub(start);
        workload_samples.push(elapsed as f64 / 1_000.0 / DRAW_BATCHES as f64);
    }
    workload_samples.sort_by(|left, right| left.total_cmp(right));
    let workload_median = workload_samples[(workload_samples.len() - 1) / 2];
    common::send_draw_row(&format!(
        "{{\"backend\":\"wasm_core\",\"test\":\"wl_ui_draw\",\"status\":\"pass\",\"lines\":{},\"batches\":{DRAW_BATCHES},\"medianMs\":{workload_median:.6},\"scale\":{}}}",
        workload_vertices / 2,
        common::scale()
    ));
    common::send_draw_row(
        "{\"backend\":\"wasm_core\",\"test\":\"complete\",\"status\":\"pass\"}",
    );
    Ok(())
}
