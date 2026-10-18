/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use crate::Engine;
use spring_sdk as spring;

pub fn scale() -> f64 {
    option_env!("SPRING_BENCHMARK_SCALE")
        .and_then(|value| value.parse::<f64>().ok())
        .map(|value| value.clamp(0.0001, 1.0))
        .unwrap_or(1.0)
}

pub fn benchmark_case() -> Option<&'static str> {
    option_env!("SPRING_BENCHMARK_CASE").filter(|value| !value.is_empty())
}

#[cfg(any(
    not(target_arch = "wasm32"),
    not(any(benchmark_context_unsynced, benchmark_context_ui))
))]
pub fn callin_variant() -> Option<&'static str> {
    #[cfg(target_arch = "wasm32")]
    {
        option_env!("SPRING_BENCHMARK_CALLIN_VARIANT").filter(|value| !value.is_empty())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        static VALUE: std::sync::OnceLock<Option<String>> = std::sync::OnceLock::new();
        VALUE
            .get_or_init(|| std::env::var("SPRING_NATIVE_BENCHMARK_CALLIN_VARIANT").ok())
            .as_deref()
    }
}

#[cfg(any(not(target_arch = "wasm32"), not(benchmark_context_unsynced)))]
pub fn repeats() -> usize {
    option_env!("SPRING_BENCHMARK_REPEATS")
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(5)
}

#[cfg(any(
    not(target_arch = "wasm32"),
    not(any(benchmark_context_unsynced, benchmark_context_ui))
))]
pub fn iterations(default: usize) -> usize {
    option_env!("SPRING_BENCHMARK_ITERATIONS")
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

#[cfg(any(
    not(target_arch = "wasm32"),
    benchmark_context_unsynced,
    benchmark_context_ui
))]
pub fn count(value: usize) -> usize {
    ((value as f64 * scale()).round() as usize).max(1)
}

#[cfg(any(
    not(target_arch = "wasm32"),
    not(any(benchmark_context_unsynced, benchmark_context_ui))
))]
pub fn scaled_count(value: usize, factor: f64) -> usize {
    ((value as f64 * factor).round() as usize).max(1)
}

pub fn timer_micros(spring: &Engine<'_>) -> spring::Result<u64> {
    #[cfg(target_arch = "wasm32")]
    {
        spring.profiling().get_timer_micros()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = spring;
        static START: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
        Ok(START
            .get_or_init(std::time::Instant::now)
            .elapsed()
            .as_micros() as u64)
    }
}

#[cfg(any(
    not(target_arch = "wasm32"),
    not(any(benchmark_context_unsynced, benchmark_context_ui))
))]
pub fn clock_quantum_ns(spring: &Engine<'_>) -> spring::Result<f64> {
    let mut best = u64::MAX;
    for _ in 0..5 {
        let start = timer_micros(spring)?;
        let mut now = start;
        while now == start {
            now = timer_micros(spring)?;
        }
        best = best.min(now - start);
    }
    Ok(best.max(1) as f64 * 1_000.0)
}

#[cfg(any(
    not(target_arch = "wasm32"),
    not(any(benchmark_context_unsynced, benchmark_context_ui))
))]
pub fn resolution_floor_ns(quantum_ns: f64, regions: usize) -> f64 {
    50.0 * quantum_ns * (regions.max(1) as f64).sqrt()
}

#[cfg(any(not(target_arch = "wasm32"), not(benchmark_context_ui)))]
pub fn send_row(spring: &Engine<'_>, row: &str) {
    #[cfg(target_arch = "wasm32")]
    spring
        .messages()
        .send_lua_rules_msg(&format!("WASM_BENCH|{row}"))
        .expect("benchmark result delivery");
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::io::Write;
        let row = row.replace("wasm_core", "native");
        let dir = std::path::PathBuf::from(
            std::env::var_os("SPRING_NATIVE_PARITY_OUTPUT_DIR")
                .expect("benchmark output directory"),
        );
        std::fs::create_dir_all(&dir).unwrap();
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(dir.join("benchmark_native.jsonl"))
            .unwrap();
        writeln!(file, "{row}").unwrap();
        spring
            .messages()
            .send_lua_rules_msg(&format!("NATIVE_BENCH|{row}"))
            .expect("benchmark result delivery");
    }
}
#[cfg(any(not(target_arch = "wasm32"), benchmark_context_ui))]
pub fn send_draw_row(spring: &Engine<'_>, row: &str) {
    let row = if cfg!(target_arch = "wasm32") {
        row.to_owned()
    } else {
        row.replace("wasm_core", "native")
    };
    spring
        .messages()
        .send_lua_ui_msg(&format!("WASM_DRAW|{row}"), "")
        .expect("draw result delivery");
}

#[cfg(any(not(target_arch = "wasm32"), not(benchmark_context_ui)))]
pub fn send_complete(spring: &Engine<'_>, case: &str) {
    send_row(
        spring,
        &format!(
            "{{\"backend\":\"wasm_core\",\"test\":\"complete\",\"status\":\"pass\",\"scale\":{},\"benchmarkCase\":\"{case}\"}}",
            scale()
        ),
    );
}

#[cfg(any(
    not(target_arch = "wasm32"),
    not(any(benchmark_context_unsynced, benchmark_context_ui))
))]
pub fn measure<F>(
    spring: &Engine<'_>,
    name: &str,
    requested: usize,
    mut operation: F,
) -> spring::Result<()>
where
    F: FnMut() -> spring::Result<()>,
{
    let quantum_ns = clock_quantum_ns(spring)?;
    let floor_ns = resolution_floor_ns(quantum_ns, 1);
    let mut calls = requested.max(1);
    let mut elapsed_ns;
    loop {
        let start = timer_micros(spring)?;
        for _ in 0..calls {
            operation()?;
        }
        elapsed_ns = timer_micros(spring)?.saturating_sub(start) as f64 * 1_000.0;
        if elapsed_ns >= floor_ns || calls >= requested.max(1).saturating_mul(1_024) {
            break;
        }
        let growth = if elapsed_ns <= 0.0 {
            16.0
        } else {
            (floor_ns / elapsed_ns).ceil().max(2.0)
        };
        calls = ((calls as f64 * growth).ceil() as usize).max(calls + 1);
    }
    if elapsed_ns < floor_ns {
        send_row(
            spring,
            &format!(
                "{{\"backend\":\"wasm_core\",\"test\":\"{name}\",\"status\":\"unavailable\",\"iterations\":{calls},\"scale\":{},\"reason\":\"sample of {elapsed_ns:.0} ns is below the {floor_ns:.0} ns timer-resolution floor\"}}",
                scale()
            ),
        );
        return Ok(());
    }

    let mut samples = Vec::with_capacity(repeats());
    for _ in 0..repeats() {
        let start = timer_micros(spring)?;
        for _ in 0..calls {
            operation()?;
        }
        let end = timer_micros(spring)?;
        samples.push((end.saturating_sub(start) as f64 * 1_000.0) / calls as f64);
    }
    samples.sort_by(|left, right| left.total_cmp(right));
    let median = samples[(samples.len() - 1) / 2];
    let spread = samples[samples.len() - 1] - samples[0];
    let p99 = samples[((samples.len() - 1) * 99) / 100];
    let samples_json = samples
        .iter()
        .map(|sample| format!("{sample:.3}"))
        .collect::<Vec<_>>()
        .join(",");
    send_row(
        spring,
        &format!(
            "{{\"backend\":\"wasm_core\",\"test\":\"{name}\",\"status\":\"pass\",\"iterations\":{calls},\"medianNs\":{median:.3},\"spreadNs\":{spread:.3},\"p99Ns\":{p99:.3},\"samplesNs\":[{samples_json}],\"totalMedianNs\":{:.3},\"totalSpreadNs\":{:.3},\"quantumNs\":{quantum_ns:.0},\"scale\":{},\"measurement\":\"SDK callout loop\"}}",
            median * calls as f64,
            spread * calls as f64,
            scale()
        ),
    );
    Ok(())
}
