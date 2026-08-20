use std::hint::black_box;

use spring_wasm_core as spring;

use crate::common;

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

pub fn run(scalar_only: bool) -> spring::Result<()> {
    let scale = common::scale();
    let units = spring::get_team_units(0)?;
    let unit_id = *units
        .first()
        .ok_or(spring::ApiError::new(spring::ErrorCode::NotFound as i32))?;
    let unit_def_id = spring::get_unit_def_id(unit_id)?;
    let position = spring::get_unit_position(unit_id, false, false)?;

    common::measure(
        "callout_scalar",
        if scalar_only {
            common::iterations(100_000)
        } else {
            common::scaled_count(100_000, scale)
        },
        || {
            spring::get_unit_def_id(unit_id)
                .map(|value| black_box(value))
                .map(|_| ())
        },
    )?;
    if scalar_only {
        common::send_complete("callout_scalar");
        return Ok(());
    }

    common::measure("callout_vec3", common::scaled_count(100_000, scale), || {
        spring::get_unit_position(unit_id, false, false)
            .map(|value| black_box(value))
            .map(|_| ())
    })?;
    common::measure("callout_string", common::scaled_count(50_000, scale), || {
        let bytes = spring::get_unit_def_name_bytes(unit_def_id)?;
        let name = String::from_utf8(bytes)
            .map_err(|_| spring::ApiError::new(spring::ErrorCode::Internal as i32))?;
        black_box(name);
        Ok(())
    })?;
    common::measure("callout_smalllist", common::scaled_count(20_000, scale), || {
        spring::get_unit_commands(unit_id, 5)
            .map(|value| black_box(value))
            .map(|_| ())
    })?;
    common::measure("callout_biglist", common::scaled_count(1_000, scale), || {
        spring::get_team_units(0)
            .map(|value| black_box(value))
            .map(|_| ())
    })?;
    common::measure("callout_spatial", common::scaled_count(10_000, scale), || {
        owned_cylinder(position[0], position[2], 300.0, -1)
            .map(|value| black_box(value))
            .map(|_| ())
    })?;
    common::measure("callout_mutate", common::scaled_count(100_000, scale), || {
        spring::set_unit_rules_param_f32(unit_id, "bench", 1.0, -1)
            .map(|value| black_box(value))
            .map(|_| ())
    })?;
    common::send_complete("callouts");
    Ok(())
}
