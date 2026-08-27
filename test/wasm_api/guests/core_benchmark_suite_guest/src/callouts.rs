use std::hint::black_box;

use spring as spring;

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

#[cfg(feature = "transport_ceiling")]
fn run_transport_ceiling(
    unit_id: spring::UnitId,
    unit_def_id: spring::DefId,
    position: spring::Float3,
    scale: f64,
) -> spring::Result<()> {
    // These rows deliberately avoid per-call allocation. They measure the
    // steady-state Core transport path, not the ergonomic owned API's probing,
    // allocation or decoding costs.
    common::measure(
        "core_ceiling_fixed_struct",
        common::scaled_count(100_000, scale),
        || {
            spring::get_unit_health(unit_id)
                .map(|value| black_box(value))
                .map(|_| ())
        },
    )?;

    const INPUT_STRING: &str = "core-transport-benchmark";
    common::measure(
        "core_ceiling_string_in_borrowed",
        common::scaled_count(100_000, scale),
        || {
            spring::benchmark_consume_string(INPUT_STRING)
                .map(|value| black_box(value))
                .map(|_| ())
        },
    )?;

    const INPUT_FLOATS: [f32; 8] = [0.25, 1.0, -2.0, 4.0, 8.0, 16.0, 32.0, 64.0];
    common::measure(
        "core_ceiling_f32_list_in_borrowed",
        common::scaled_count(100_000, scale),
        || {
            spring::benchmark_consume_f32_list(&INPUT_FLOATS)
                .map(|value| black_box(value))
                .map(|_| ())
        },
    )?;

    let mut empty_bytes: [u8; 0] = [];
    let name_required = match spring::get_unit_def_name_into(unit_def_id, &mut empty_bytes)? {
        spring::ByteBufferFill::Complete(count)
        | spring::ByteBufferFill::Insufficient { required: count } => count,
    };
    let mut name_buffer = vec![0u8; name_required.max(1)];
    common::measure(
        "core_ceiling_string_out_reuse",
        common::scaled_count(50_000, scale),
        || match spring::get_unit_def_name_into(unit_def_id, &mut name_buffer)? {
            spring::ByteBufferFill::Complete(count) => {
                black_box(&name_buffer[..count]);
                Ok(())
            }
            spring::ByteBufferFill::Insufficient { .. } => {
                Err(spring::ApiError::new(spring::ErrorCode::BufferOverflow as i32))
            }
        },
    )?;

    let team_capacity = spring::get_team_unit_count(spring::TeamId::from(0))? as usize;
    let mut team_buffer = vec![0i32; team_capacity.max(1)];
    common::measure(
        "core_ceiling_list_out_reuse",
        common::scaled_count(20_000, scale),
        || match spring::get_team_units_into(spring::TeamId::from(0), &mut team_buffer)? {
            spring::BufferFill::Complete(count) => {
                black_box(&team_buffer[..count]);
                Ok(())
            }
            spring::BufferFill::Insufficient { .. } => {
                Err(spring::ApiError::new(spring::ErrorCode::BufferOverflow as i32))
            }
        },
    )?;

    // The fixture seeds this unit with one MOVE command before timing. Probe
    // the nested command wire size once, then reuse the byte buffer so the
    // timed row measures only host serialization + one Core crossing.
    let command_required = match spring::get_unit_commands_into(unit_id, 5, &mut [])? {
        spring::CommandBufferFill::Complete(bytes)
        | spring::CommandBufferFill::Insufficient { required: bytes } => bytes,
    };
    let mut command_buffer = vec![0u8; command_required.max(1)];
    common::measure(
        "core_ceiling_nested_list_out_reuse",
        common::scaled_count(20_000, scale),
        || match spring::get_unit_commands_into(unit_id, 5, &mut command_buffer)? {
            spring::CommandBufferFill::Complete(bytes) => {
                black_box(&command_buffer[..bytes]);
                Ok(())
            }
            spring::CommandBufferFill::Insufficient { .. } => {
                Err(spring::ApiError::new(spring::ErrorCode::BufferOverflow as i32))
            }
        },
    )?;

    let mut empty_units: [i32; 0] = [];
    let spatial_required = match spring::get_units_in_cylinder_into(
        position[0],
        position[2],
        300.0,
        -1,
        &mut empty_units,
    )? {
        spring::BufferFill::Complete(count)
        | spring::BufferFill::Insufficient { required: count } => count,
    };
    let mut spatial_buffer = vec![0i32; spatial_required.max(1)];
    common::measure(
        "core_ceiling_spatial_list_reuse",
        common::scaled_count(10_000, scale),
        || match spring::get_units_in_cylinder_into(
            position[0],
            position[2],
            300.0,
            -1,
            &mut spatial_buffer,
        )? {
            spring::BufferFill::Complete(count) => {
                black_box(&spatial_buffer[..count]);
                Ok(())
            }
            spring::BufferFill::Insufficient { .. } => {
                Err(spring::ApiError::new(spring::ErrorCode::BufferOverflow as i32))
            }
        },
    )?;

    // Real list<string> transport. Probe once, allocate guest storage once,
    // then reuse both descriptor and byte buffers. The timed path performs no
    // Vec<String> construction and no UTF-8 validation.
    let mut empty_ranges: [spring::StringRange; 0] = [];
    let mut empty_string_bytes: [u8; 0] = [];
    let string_list_required = match spring::get_unit_script_names_into(
        unit_id,
        &mut empty_ranges,
        &mut empty_string_bytes,
    )? {
        spring::StringListFill::Complete(view) => spring::StringListRequirements {
            strings: view.len(),
            bytes: view.packed_bytes().len(),
        },
        spring::StringListFill::Insufficient(required) => required,
    };
    let mut string_ranges = vec![spring::StringRange::default(); string_list_required.strings.max(1)];
    let mut string_bytes = vec![0u8; string_list_required.bytes.max(1)];
    common::measure(
        "core_ceiling_string_list_out_reuse",
        common::scaled_count(10_000, scale),
        || match spring::get_unit_script_names_into(unit_id, &mut string_ranges, &mut string_bytes)? {
            spring::StringListFill::Complete(view) => {
                black_box(view.ranges());
                black_box(view.packed_bytes());
                Ok(())
            }
            spring::StringListFill::Insufficient(_) => {
                Err(spring::ApiError::new(spring::ErrorCode::BufferOverflow as i32))
            }
        },
    )?;

    Ok(())
}

pub fn run(scalar_only: bool) -> spring::Result<()> {
    let scale = common::scale();
    let units = spring::get_team_units(spring::TeamId::from(0))?;
    let unit_id = *units
        .first()
        .ok_or(spring::ApiError::new(spring::ErrorCode::NotFound as i32))?;
    let unit_def_id = spring::get_unit_def_id(spring::UnitId::from(unit_id))?;
    let position = spring::get_unit_position(spring::UnitId::from(unit_id), false, false)?;

    common::measure(
        "callout_scalar",
        if scalar_only {
            common::iterations(100_000)
        } else {
            common::scaled_count(100_000, scale)
        },
        || {
            spring::get_unit_def_id(spring::UnitId::from(unit_id))
                .map(|value| black_box(value))
                .map(|_| ())
        },
    )?;
    if scalar_only {
        common::send_complete("callout_scalar");
        return Ok(());
    }

    common::measure("callout_vec3", common::scaled_count(100_000, scale), || {
        spring::get_unit_position(spring::UnitId::from(unit_id), false, false)
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
        spring::get_unit_commands(spring::UnitId::from(unit_id), 5)
            .map(|value| black_box(value))
            .map(|_| ())
    })?;
    common::measure("callout_biglist", common::scaled_count(1_000, scale), || {
        spring::get_team_units(spring::TeamId::from(0))
            .map(|value| black_box(value))
            .map(|_| ())
    })?;
    common::measure("callout_spatial", common::scaled_count(10_000, scale), || {
        owned_cylinder(position.x, position.z, 300.0, -1)
            .map(|value| black_box(value))
            .map(|_| ())
    })?;
    common::measure("callout_mutate", common::scaled_count(100_000, scale), || {
        spring::set_unit_rules_param_f32(spring::UnitId::from(unit_id), "bench", 1.0, -1)
            .map(|value| black_box(value))
            .map(|_| ())
    })?;

    // Paired wide-argument callout: Lua exposes these twelve scalar physics
    // fields directly, while Core carries the same payload as four Float3
    // records. Keep the values stable so the row measures transport and host
    // adaptation rather than a changing simulation outcome.
    let physics_position = spring::Float3::new(position.x, position.y, position.z);
    let velocity = spring::Float3::new(1.0, 0.0, 0.0);
    let rotation = spring::Float3::new(0.0, 1.0, 0.0);
    let drag = spring::Float3::new(1.0, 0.0, 0.0);
    common::measure(
        "callout_wide_unit_physics",
        common::scaled_count(20_000, scale),
        || {
            spring::rules_synced::unit_control::set_unit_physics(
                unit_id,
                physics_position.clone(),
                velocity.clone(),
                rotation.clone(),
                drag.clone(),
            )
            .map(|value| black_box(value))
            .map(|_| ())
        },
    )?;

    // The terrain name is a real variable input. These paired rows make the
    // payload-size slope visible instead of leaving string marshalling as a
    // single arbitrary point.
    for (name, payload) in [
        ("callout_payload_8", "terrain"),
        ("callout_payload_64", "terrain-payload-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"),
        ("callout_payload_256", "terrain-payload-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"),
    ] {
        common::measure(name, common::scaled_count(20_000, scale), || {
            spring::rules_synced::terrain_control::set_terrain_type_data(
                0, 1.0, 1.0, 1.0, 1.0, 1.0, true, payload,
            )
            .map(|value| black_box(value))
            .map(|_| ())
        })?;
    }

    #[cfg(feature = "transport_ceiling")]
    run_transport_ceiling(spring::UnitId::from(unit_id), unit_def_id, position, scale)?;
    #[cfg(not(feature = "transport_ceiling"))]
    let _ = (unit_def_id, position);

    common::send_complete("callouts");
    Ok(())
}
