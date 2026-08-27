use std::hint::black_box;
use std::sync::{Mutex, OnceLock};

use spring as spring;

use crate::common;

struct State {
    units: Vec<i32>,
    frame: usize,
    frames: usize,
    samples_ns: Vec<Vec<f64>>,
    checksums: Vec<f64>,
}

static STATE: OnceLock<Mutex<Option<State>>> = OnceLock::new();

const NAMES: [&str; 5] = [
    "wl_unit_scan",
    "wl_area_effect",
    "wl_rules_params",
    "wl_commands",
    "wl_compute",
];

fn owned_cylinder(x: f32, z: f32, radius: f32, allegiance: i32) -> spring::Result<Vec<i32>> {
    let mut empty: [i32; 0] = [];
    let first = spring::get_units_in_cylinder_into(x, z, radius, allegiance, &mut empty)?;
    let required = match first {
        spring::BufferFill::Complete(count)
        | spring::BufferFill::Insufficient { required: count } => count,
    };
    let mut units = vec![0i32; required];
    for _ in 0..3 {
        match spring::get_units_in_cylinder_into(x, z, radius, allegiance, &mut units)? {
            spring::BufferFill::Complete(count) => {
                units.truncate(count);
                return Ok(units);
            }
            spring::BufferFill::Insufficient { required } => units.resize(required, 0),
        }
    }
    Err(spring::ApiError::new(spring::ErrorCode::BufferOverflow as i32))
}

fn timed<F>(operation: F) -> spring::Result<(f64, f64)>
where
    F: FnOnce() -> spring::Result<f64>,
{
    let start = common::timer_micros()?;
    let checksum = operation()?;
    let end = common::timer_micros()?;
    Ok((end.saturating_sub(start) as f64 * 1_000.0, checksum))
}

fn ensure_started() -> spring::Result<()> {
    let cell = STATE.get_or_init(|| Mutex::new(None));
    let mut guard = cell
        .lock()
        .map_err(|_| spring::ApiError::new(spring::ErrorCode::InvalidState as i32))?;
    if guard.is_some() {
        return Ok(());
    }
    let units = spring::get_team_units(spring::TeamId::from(0))?;
    if units.is_empty() {
        return Err(spring::ApiError::new(spring::ErrorCode::NotFound as i32));
    }
    let frames = common::scaled_count(5_000, common::scale());
    *guard = Some(State {
        units,
        frame: 0,
        frames,
        samples_ns: (0..NAMES.len()).map(|_| Vec::with_capacity(frames)).collect(),
        checksums: vec![0.0; NAMES.len()],
    });
    Ok(())
}

pub fn step() -> spring::Result<bool> {
    ensure_started()?;
    let cell = STATE.get_or_init(|| Mutex::new(None));
    let completed = {
        let mut guard = cell
            .lock()
            .map_err(|_| spring::ApiError::new(spring::ErrorCode::InvalidState as i32))?;
        let state = guard
            .as_mut()
            .ok_or(spring::ApiError::new(spring::ErrorCode::InvalidState as i32))?;
        let unit_limit = state.units.len().min(1_000);
        let area_limit = state.units.len().min(100);
        let command_limit = state.units.len().min(200);
        let units = &state.units;

        let (elapsed, checksum) = timed(|| {
            let mut checksum = 0.0;
            for unit in units.iter().take(unit_limit) {
                let position = spring::get_unit_position(spring::UnitId::from(*unit), false, false)?;
                checksum += f64::from(position[0] + position[1] + position[2]);
                let _ = spring::get_unit_health(spring::UnitId::from(*unit))?;
                checksum += f64::from(i32::from(
                    spring::get_unit_def_id(spring::UnitId::from(*unit))?,
                ));
            }
            Ok(checksum)
        })?;
        state.samples_ns[0].push(elapsed);
        state.checksums[0] += checksum;

        let (elapsed, checksum) = timed(|| {
            let mut checksum = 0.0;
            for unit in units.iter().take(area_limit) {
                let position = spring::get_unit_position(spring::UnitId::from(*unit), false, false)?;
                checksum += owned_cylinder(position[0], position[2], 300.0, -1)?.len() as f64;
            }
            Ok(checksum)
        })?;
        state.samples_ns[1].push(elapsed);
        state.checksums[1] += checksum;

        let (elapsed, checksum) = timed(|| {
            let mut checksum = 0.0;
            for (index, unit) in units.iter().take(unit_limit).enumerate() {
                spring::set_unit_rules_param_f32(spring::UnitId::from(*unit), "bench", 1.0, -1)?;
                let _ = spring::get_unit_rules_param_f32(spring::UnitId::from(*unit), "bench")?;
                checksum += index as f64;
            }
            Ok(checksum)
        })?;
        state.samples_ns[2].push(elapsed);
        state.checksums[2] += checksum;

        let (elapsed, checksum) = timed(|| {
            let mut checksum = 0.0;
            for unit in units.iter().take(command_limit) {
                let position = spring::get_unit_position(spring::UnitId::from(*unit), false, false)?;
                spring::give_order_to_unit(
                    spring::UnitId::from(*unit),
                    10,
                    &[position[0] + 8.0, position[1], position[2] + 8.0],
                    0,
                    0,
                )?;
                checksum += 1.0;
            }
            Ok(checksum)
        })?;
        state.samples_ns[3].push(elapsed);
        state.checksums[3] += checksum;

        let (elapsed, checksum) = timed(|| {
            let mut value = 0.0f32;
            for index in 1..=100_000 {
                value = (value + index as f32 * 0.25) % 1_000_003.0;
            }
            black_box(value);
            Ok(f64::from(value))
        })?;
        state.samples_ns[4].push(elapsed);
        state.checksums[4] += checksum;

        state.frame += 1;
        if state.frame < state.frames {
            None
        } else {
            Some((
                state.frames,
                std::mem::take(&mut state.samples_ns),
                state.checksums.clone(),
            ))
        }
    };

    let Some((frames, samples, checksums)) = completed else {
        return Ok(false);
    };
    let sample_count = common::repeats().min(frames).max(1);
    for (index, name) in NAMES.iter().enumerate() {
        let frames_per_sample = frames.div_ceil(sample_count);
        let mut grouped = Vec::with_capacity(sample_count);
        for chunk in samples[index].chunks(frames_per_sample) {
            grouped.push(chunk.iter().sum::<f64>() / chunk.len() as f64);
        }
        let mut sorted = grouped;
        sorted.sort_by(|left, right| left.total_cmp(right));
        let median = sorted[(sorted.len() - 1) / 2];
        let spread = sorted[sorted.len() - 1] - sorted[0];
        let p99 = sorted[((sorted.len() - 1) * 99) / 100];
        let samples_json = sorted
            .iter()
            .map(|sample| format!("{sample:.3}"))
            .collect::<Vec<_>>()
            .join(",");
        common::send_row(&format!(
            "{{\"backend\":\"wasm_core\",\"test\":\"{name}\",\"status\":\"pass\",\"iterations\":{frames},\"medianNs\":{median:.3},\"p99Ns\":{p99:.3},\"spreadNs\":{spread:.3},\"samplesNs\":[{samples_json}],\"checksum\":{:.3},\"scale\":{},\"measurement\":\"Core Wasm workload measured per GameFrame callback\"}}",
            checksums[index],
            common::scale()
        ));
    }
    common::send_complete("workloads");
    Ok(true)
}
