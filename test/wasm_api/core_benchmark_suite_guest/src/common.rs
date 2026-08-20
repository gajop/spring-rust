use spring_wasm_core as spring;

pub fn scale() -> f64 {
    option_env!("SPRING_BENCHMARK_SCALE")
        .and_then(|value| value.parse::<f64>().ok())
        .map(|value| value.clamp(0.0001, 1.0))
        .unwrap_or(1.0)
}

pub fn benchmark_case() -> Option<&'static str> {
    option_env!("SPRING_BENCHMARK_CASE").filter(|value| !value.is_empty())
}

pub fn callin_variant() -> Option<&'static str> {
    option_env!("SPRING_BENCHMARK_CALLIN_VARIANT").filter(|value| !value.is_empty())
}

pub fn repeats() -> usize {
    option_env!("SPRING_BENCHMARK_REPEATS")
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(5)
}

pub fn iterations(default: usize) -> usize {
    option_env!("SPRING_BENCHMARK_ITERATIONS")
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

pub fn count(value: usize) -> usize {
    ((value as f64 * scale()).round() as usize).max(1)
}

pub fn scaled_count(value: usize, factor: f64) -> usize {
    ((value as f64 * factor).round() as usize).max(1)
}

pub fn timer_micros() -> spring::Result<u64> {
    spring::desync::get_timer_micros()
}

pub fn clock_quantum_ns() -> spring::Result<f64> {
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

pub fn resolution_floor_ns(quantum_ns: f64, regions: usize) -> f64 {
    50.0 * quantum_ns * (regions.max(1) as f64).sqrt()
}

pub fn send_row(row: &str) {
    let _ = spring::send_lua_rules_msg(&format!("WASM_BENCH|{row}"));
}

pub fn send_draw_row(row: &str) {
    let _ = spring::send_lua_ui_msg(&format!("WASM_DRAW|{row}"), "");
}

pub fn send_complete(case: &str) {
    send_row(&format!(
        "{{\"backend\":\"wasm_core\",\"test\":\"complete\",\"status\":\"pass\",\"scale\":{},\"benchmarkCase\":\"{case}\"}}",
        scale()
    ));
}

pub fn send_error(code: i32) {
    send_row(&format!(
        "{{\"backend\":\"wasm_core\",\"test\":\"complete\",\"status\":\"error\",\"code\":{code}}}"
    ));
}

pub fn measure<F>(name: &str, requested: usize, mut operation: F) -> spring::Result<()>
where
    F: FnMut() -> spring::Result<()>,
{
    let quantum_ns = clock_quantum_ns()?;
    let floor_ns = resolution_floor_ns(quantum_ns, 1);
    let mut calls = requested.max(1);
    let mut elapsed_ns;
    loop {
        let start = timer_micros()?;
        for _ in 0..calls {
            operation()?;
        }
        elapsed_ns = timer_micros()?.saturating_sub(start) as f64 * 1_000.0;
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
        send_row(&format!(
            "{{\"backend\":\"wasm_core\",\"test\":\"{name}\",\"status\":\"unavailable\",\"iterations\":{calls},\"scale\":{},\"reason\":\"sample of {elapsed_ns:.0} ns is below the {floor_ns:.0} ns timer-resolution floor\"}}",
            scale()
        ));
        return Ok(());
    }

    let mut samples = Vec::with_capacity(repeats());
    for _ in 0..repeats() {
        let start = timer_micros()?;
        for _ in 0..calls {
            operation()?;
        }
        let end = timer_micros()?;
        samples.push((end.saturating_sub(start) as f64 * 1_000.0) / calls as f64);
    }
    samples.sort_by(|left, right| left.total_cmp(right));
    let median = samples[(samples.len() - 1) / 2];
    let spread = samples[samples.len() - 1] - samples[0];
    send_row(&format!(
        "{{\"backend\":\"wasm_core\",\"test\":\"{name}\",\"status\":\"pass\",\"iterations\":{calls},\"medianNs\":{median:.3},\"spreadNs\":{spread:.3},\"totalMedianNs\":{:.3},\"totalSpreadNs\":{:.3},\"quantumNs\":{quantum_ns:.0},\"scale\":{},\"measurement\":\"Core Wasm callout loop\"}}",
        median * calls as f64,
        spread * calls as f64,
        scale()
    ));
    Ok(())
}
