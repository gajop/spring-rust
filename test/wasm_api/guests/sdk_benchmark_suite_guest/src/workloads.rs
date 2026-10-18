/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use std::hint::black_box;
use std::sync::{Mutex, OnceLock};

use crate::Engine;
use spring_sdk as spring;

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

fn timed<F>(spring: &Engine<'_>, operation: F) -> spring::Result<(f64, f64)>
where
    F: FnOnce() -> spring::Result<f64>,
{
    let start = common::timer_micros(spring)?;
    let checksum = operation()?;
    let end = common::timer_micros(spring)?;
    Ok((end.saturating_sub(start) as f64 * 1_000.0, checksum))
}

fn ensure_started(spring: &Engine<'_>) -> spring::Result<()> {
    let cell = STATE.get_or_init(|| Mutex::new(None));
    let mut guard = cell
        .lock()
        .map_err(|_| spring::ApiError::new(spring::ErrorCode::InvalidState as i32))?;
    if guard.is_some() {
        return Ok(());
    }
    let units = spring.units().get_team_units(0)?;
    if units.is_empty() {
        return Err(spring::ApiError::new(spring::ErrorCode::NotFound as i32));
    }
    let frames = common::scaled_count(5_000, common::scale());
    *guard = Some(State {
        units,
        frame: 0,
        frames,
        samples_ns: (0..NAMES.len())
            .map(|_| Vec::with_capacity(frames))
            .collect(),
        checksums: vec![0.0; NAMES.len()],
    });
    Ok(())
}

pub fn step(spring: &Engine<'_>) -> spring::Result<bool> {
    ensure_started(spring)?;
    let cell = STATE.get_or_init(|| Mutex::new(None));
    let completed = {
        let mut guard = cell
            .lock()
            .map_err(|_| spring::ApiError::new(spring::ErrorCode::InvalidState as i32))?;
        let state = guard.as_mut().ok_or(spring::ApiError::new(
            spring::ErrorCode::InvalidState as i32,
        ))?;
        let unit_limit = state.units.len().min(1_000);
        let area_limit = state.units.len().min(100);
        let command_limit = state.units.len().min(200);
        let units = &state.units;

        let (elapsed, checksum) = timed(spring, || {
            let mut checksum = 0.0;
            for unit in units.iter().take(unit_limit) {
                let position = spring.units_info().get_unit_position(
                    *unit,
                    spring::units_info::GetUnitPositionOptions::default(),
                )?;
                checksum += f64::from(position[0] + position[1] + position[2]);
                let _ = spring.units_info().get_unit_health(*unit)?;
                checksum += f64::from(spring.units_info().get_unit_def_id(*unit)?);
            }
            Ok(checksum)
        })?;
        state.samples_ns[0].push(elapsed);
        state.checksums[0] += checksum;

        let (elapsed, checksum) = timed(spring, || {
            let mut checksum = 0.0;
            for unit in units.iter().take(area_limit) {
                let position = spring.units_info().get_unit_position(
                    *unit,
                    spring::units_info::GetUnitPositionOptions::default(),
                )?;
                checksum += spring
                    .units()
                    .get_units_in_cylinder(position[0], position[2], 300.0, -1)?
                    .len() as f64;
            }
            Ok(checksum)
        })?;
        state.samples_ns[1].push(elapsed);
        state.checksums[1] += checksum;

        let (elapsed, checksum) = timed(spring, || {
            let mut checksum = 0.0;
            for (index, unit) in units.iter().take(unit_limit).enumerate() {
                spring
                    .rules_params()
                    .set_unit_rules_param_f32(*unit, "bench", 1.0, -1)?;
                let _ = spring
                    .rules_params()
                    .get_unit_rules_param_f32(*unit, "bench")?;
                checksum += index as f64;
            }
            Ok(checksum)
        })?;
        state.samples_ns[2].push(elapsed);
        state.checksums[2] += checksum;

        let (elapsed, checksum) = timed(spring, || {
            let mut checksum = 0.0;
            for unit in units.iter().take(command_limit) {
                let position = spring.units_info().get_unit_position(
                    *unit,
                    spring::units_info::GetUnitPositionOptions::default(),
                )?;
                spring.unit_control().give_order_to_unit(
                    *unit,
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

        let (elapsed, checksum) = timed(spring, || {
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
        common::send_row(
            spring,
            &format!(
                "{{\"backend\":\"wasm_core\",\"test\":\"{name}\",\"status\":\"pass\",\"iterations\":{frames},\"medianNs\":{median:.3},\"p99Ns\":{p99:.3},\"spreadNs\":{spread:.3},\"samplesNs\":[{samples_json}],\"checksum\":{:.3},\"scale\":{},\"measurement\":\"Core Wasm workload measured per GameFrame callback\"}}",
                checksums[index],
                common::scale()
            ),
        );
    }
    common::send_complete(spring, "workloads");
    Ok(true)
}
