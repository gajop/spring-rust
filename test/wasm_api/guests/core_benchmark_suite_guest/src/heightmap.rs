use std::sync::atomic::{AtomicUsize, Ordering};

use spring_wasm_core as spring;

static BRUSH_SIZE: AtomicUsize = AtomicUsize::new(0);

fn scaled_terrain_count(value: usize, scale: f64) -> usize {
    ((value as f64 * scale * scale).round() as usize).max(1)
}

fn scaled_brush_size(value: usize, scale: f64) -> usize {
    if value == 0 {
        0
    } else {
        ((value as f64 * scale.powf(0.75)).round() as usize).max(1)
    }
}

fn timer_micros() -> spring::Result<u64> {
    spring::desync::get_timer_micros()
}

fn clock_quantum_ns() -> spring::Result<f64> {
    let mut best = u64::MAX;
    for _ in 0..5 {
        let start = timer_micros()?;
        let mut now = start;
        while now == start {
            now = timer_micros()?;
        }
        best = best.min(now - start);
    }
    Ok(best.max(1) as f64 * 1_000.0)
}

fn resolution_floor_ns(quantum_ns: f64) -> f64 {
    50.0 * quantum_ns
}

fn send_row(row: &str) {
    let _ = spring::send_lua_rules_msg(&format!("WASM_BENCH|{row}"));
}

fn send_unresolved(name: &str, elapsed_ns: f64, floor_ns: f64, count: usize, scale: f64) {
    send_row(&format!(
        "{{\"backend\":\"wasm_core\",\"test\":\"{name}\",\"status\":\"unavailable\",\"iterations\":{count},\"scale\":{scale},\"reason\":\"sample of {elapsed_ns:.0} ns is below the {floor_ns:.0} ns timer-resolution floor\"}}"
    ));
}

pub fn callback() {
    let size = BRUSH_SIZE.load(Ordering::Relaxed);
    for offset_x in 0..size {
        for offset_z in 0..size {
            let _ = spring::set_height_map(
                8.0 + (offset_x % 30) as f32 * 8.0,
                8.0 + (offset_z % 30) as f32 * 8.0,
                0.0,
                0.0,
            );
        }
    }
}

fn measure<F>(
    name: &str,
    invocations: usize,
    scale: f64,
    mut operation: F,
) -> spring::Result<Option<(usize, Vec<f64>)>>
where
    F: FnMut() -> spring::Result<()>,
{
    let quantum_ns = clock_quantum_ns()?;
    let floor_ns = resolution_floor_ns(quantum_ns);
    let mut count = invocations;
    let mut elapsed_ns;
    loop {
        let start = timer_micros()?;
        for _ in 0..count {
            operation()?;
        }
        elapsed_ns = timer_micros()?.saturating_sub(start) as f64 * 1_000.0;
        if elapsed_ns >= floor_ns || count >= invocations.saturating_mul(1_024) {
            break;
        }
        let growth = if elapsed_ns <= 0.0 {
            16.0
        } else {
            (floor_ns / elapsed_ns).ceil().max(2.0)
        };
        count = ((count as f64 * growth).ceil() as usize).max(count + 1);
    }
    if elapsed_ns < floor_ns {
        send_unresolved(name, elapsed_ns, floor_ns, count, scale);
        return Ok(None);
    }

    let mut samples = Vec::with_capacity(crate::common::repeats());
    for _ in 0..crate::common::repeats() {
        let start = timer_micros()?;
        for _ in 0..count {
            operation()?;
        }
        let end = timer_micros()?;
        samples.push(end.saturating_sub(start) as f64 / 1_000.0 / count as f64);
    }
    samples.sort_by(|left, right| left.total_cmp(right));
    Ok(Some((count, samples)))
}

pub fn run(scale: f64) -> spring::Result<()> {
    let terrain_height = spring::get_ground_orig_height(8.0, 8.0)?;
    for (name, nominal_size, nominal_invocations) in [
        ("hm_callback_empty", 0usize, 10_000usize),
        ("hm_brush_small", 32usize, 1_000usize),
        ("hm_brush_medium", 128usize, 100usize),
        ("hm_brush_large", 512usize, 10usize),
    ] {
        let size = scaled_brush_size(nominal_size, scale);
        BRUSH_SIZE.store(size, Ordering::Relaxed);
        let measured = measure(
            name,
            scaled_terrain_count(nominal_invocations, scale),
            scale,
            || {
                spring::set_height_map_func(spring::SyncCallback::new(1, 0)).map(|_| ())
            },
        )?;
        let Some((invocations, sorted)) = measured else {
            continue;
        };
        let median_ms = sorted[(sorted.len() - 1) / 2];
        let spread_ms = sorted[sorted.len() - 1] - sorted[0];
        let p99_ms = sorted[((sorted.len() - 1) * 99) / 100];
        let samples_json = sorted
            .iter()
            .map(|sample| format!("{sample:.6}"))
            .collect::<Vec<_>>()
            .join(",");
        let inner_calls = size * size;
        let inner_ns = if inner_calls == 0 {
            0.0
        } else {
            median_ms * 1_000_000.0 / inner_calls as f64
        };
        send_row(&format!(
            "{{\"backend\":\"wasm_core\",\"test\":\"{name}\",\"status\":\"pass\",\"invocations\":{invocations},\"innerCalls\":{inner_calls},\"medianMs\":{median_ms:.6},\"p99Ms\":{p99_ms:.6},\"spreadMs\":{spread_ms:.6},\"samplesMs\":[{samples_json}],\"innerNs\":{inner_ns:.3},\"scale\":{scale},\"nominalSize\":{nominal_size},\"nominalInvocations\":{nominal_invocations},\"measurement\":\"Core Wasm callback boundary with zero terraform; terrain rebuild excluded\"}}"
        ));
    }

    if let Some((invocations, sorted)) = measure(
        "hm_region_op",
        scaled_terrain_count(1_000, scale),
        scale,
        || spring::level_height_map(8.0, 8.0, 248.0, 248.0, terrain_height).map(|_| ()),
    )? {
        let p99_ms = sorted[((sorted.len() - 1) * 99) / 100];
        let samples_json = sorted
            .iter()
            .map(|sample| format!("{sample:.6}"))
            .collect::<Vec<_>>()
            .join(",");
        send_row(&format!(
            "{{\"backend\":\"wasm_core\",\"test\":\"hm_region_op\",\"status\":\"pass\",\"invocations\":{invocations},\"medianMs\":{:.6},\"p99Ms\":{p99_ms:.6},\"spreadMs\":{:.6},\"samplesMs\":[{samples_json}],\"innerNs\":0,\"scale\":{scale},\"nominalInvocations\":1000,\"measurement\":\"Core Wasm region boundary with unchanged height; terrain rebuild excluded\"}}",
            sorted[(sorted.len() - 1) / 2],
            sorted[sorted.len() - 1] - sorted[0]
        ));
    }
    Ok(())
}
