/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use std::hint::black_box;

use crate::Engine;
use spring_sdk as spring;

use crate::common;

pub fn run(spring: &Engine<'_>, scalar_only: bool) -> spring::Result<()> {
    let scale = common::scale();
    let units = spring.units().get_team_units(0)?;
    let unit_id = *units
        .first()
        .ok_or(spring::ApiError::new(spring::ErrorCode::NotFound as i32))?;
    let unit_def_id = spring.units_info().get_unit_def_id(unit_id)?;
    let position = spring.units_info().get_unit_position(
        unit_id,
        spring::units_info::GetUnitPositionOptions::default(),
    )?;

    #[cfg(benchmark_module_ref)]
    let module = spring.module_ref();
    common::measure(
        spring,
        "callout_scalar",
        if scalar_only {
            common::iterations(100_000)
        } else {
            common::scaled_count(100_000, scale)
        },
        || {
            #[cfg(benchmark_module_ref)]
            {
                module
                    .with_spring(|spring| spring.units_info().get_unit_def_id(unit_id))
                    .map(black_box)
                    .map(|_| ())
            }
            #[cfg(not(benchmark_module_ref))]
            spring
                .units_info()
                .get_unit_def_id(unit_id)
                .map(black_box)
                .map(|_| ())
        },
    )?;
    if scalar_only {
        common::send_complete(spring, "callout_scalar");
        return Ok(());
    }

    common::measure(
        spring,
        "callout_vec3",
        common::scaled_count(100_000, scale),
        || {
            spring
                .units_info()
                .get_unit_position(
                    unit_id,
                    spring::units_info::GetUnitPositionOptions::default(),
                )
                .map(black_box)
                .map(|_| ())
        },
    )?;
    common::measure(
        spring,
        "callout_string",
        common::scaled_count(50_000, scale),
        || {
            let name = spring.unit_defs().get_unit_def_name(unit_def_id)?;
            black_box(name);
            Ok(())
        },
    )?;
    common::measure(
        spring,
        "callout_smalllist",
        common::scaled_count(20_000, scale),
        || {
            spring
                .units_commands()
                .get_unit_commands(unit_id, 5)
                .map(black_box)
                .map(|_| ())
        },
    )?;
    common::measure(
        spring,
        "callout_biglist",
        common::scaled_count(1_000, scale),
        || spring.units().get_team_units(0).map(black_box).map(|_| ()),
    )?;
    common::measure(
        spring,
        "callout_spatial",
        common::scaled_count(10_000, scale),
        || {
            spring
                .units()
                .get_units_in_cylinder(position.x, position.z, 300.0, -1)
                .map(black_box)
                .map(|_| ())
        },
    )?;
    common::measure(
        spring,
        "callout_mutate",
        common::scaled_count(100_000, scale),
        || {
            spring
                .rules_params()
                .set_unit_rules_param_f32(unit_id, "bench", 1.0, -1)
                .map(black_box)
                .map(|_| ())
        },
    )?;

    // Paired wide-argument callout: Lua exposes these twelve scalar physics
    // fields directly, while Core carries the same payload as four Float3
    // records. Keep the values stable so the row measures transport and host
    // adaptation rather than a changing simulation outcome.
    let physics_position = spring::Float3::new(position.x, position.y, position.z);
    let velocity = spring::Float3::new(1.0, 0.0, 0.0);
    let rotation = spring::Float3::new(0.0, 1.0, 0.0);
    let drag = spring::Float3::new(1.0, 0.0, 0.0);
    common::measure(
        spring,
        "callout_wide_unit_physics",
        common::scaled_count(20_000, scale),
        || {
            spring
                .unit_control()
                .set_unit_physics(unit_id, physics_position, velocity, rotation, drag)
                .map(black_box)
                .map(|_| ())
        },
    )?;

    // The terrain name is a real variable input. These paired rows make the
    // payload-size slope visible instead of leaving string marshalling as a
    // single arbitrary point.
    for (name, payload) in [
        ("callout_payload_8", "terrain"),
        (
            "callout_payload_64",
            "terrain-payload-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
        ),
        (
            "callout_payload_256",
            "terrain-payload-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
        ),
    ] {
        common::measure(spring, name, common::scaled_count(20_000, scale), || {
            spring
                .terrain_control()
                .set_terrain_type_data(0, 1.0, 1.0, 1.0, 1.0, 1.0, true, payload)
                .map(black_box)
                .map(|_| ())
        })?;
    }

    common::send_complete(spring, "callouts");
    Ok(())
}
