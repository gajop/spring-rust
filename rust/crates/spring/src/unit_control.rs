#[cfg(feature = "alloc")]
pub use crate::owned::unit_control::{
    add_object_decal, add_unit_damage, add_unit_experience, add_unit_impulse, add_unit_resource,
    add_unit_seismic_ping, bugger_off, clear_unit_goal, destroy_unit, edit_unit_cmd_desc,
    force_unit_collision_update, get_unit_feature_separation, get_unit_leaves_ghost,
    get_unit_physical_state, give_order_array_to_unit, give_order_array_to_unit_array,
    give_order_to_unit_array, insert_unit_cmd_desc, remove_object_decal, remove_unit_cmd_desc,
    set_factory_bugger_off, set_unit_always_visible, set_unit_armored, set_unit_blocking,
    set_unit_build_params, set_unit_build_speed, set_unit_buildee_radius, set_unit_cloak,
    set_unit_collision_volume_data, set_unit_costs, set_unit_crashing, set_unit_direction,
    set_unit_experience, set_unit_flanking, set_unit_harvest_storage, set_unit_heading,
    set_unit_heading_and_up_dir, set_unit_health, set_unit_land_goal, set_unit_leaves_ghost,
    set_unit_loading_transport, set_unit_los_mask, set_unit_los_state, set_unit_mass,
    set_unit_max_health, set_unit_max_range, set_unit_metal_extraction, set_unit_mid_and_aim_pos,
    set_unit_move_goal, set_unit_nano_pieces, set_unit_neutral, set_unit_physical_state_bit,
    set_unit_physics, set_unit_piece_collision_volume_data, set_unit_piece_matrix,
    set_unit_piece_parent, set_unit_piece_visible, set_unit_pos_error_params, set_unit_position,
    set_unit_radius_and_height, set_unit_resourcing, set_unit_rotation, set_unit_seismic_signature,
    set_unit_selection_volume_data, set_unit_sensor_radius, set_unit_shield_recharge_delay,
    set_unit_shield_state, set_unit_sonar_stealth, set_unit_stealth, set_unit_stockpile,
    set_unit_storage, set_unit_target, set_unit_tooltip, set_unit_use_air_los,
    set_unit_use_weapons, set_unit_velocity, set_unit_weapon_damages, set_unit_weapon_state,
    transfer_unit, unit_attach, unit_detach, unit_detach_from_air, unit_finish_command,
    unit_weapon_fire, unit_weapon_hold_fire, use_unit_resource,
};

// UnitControl portion of the Spring Core-Wasm guest SDK.

use super::{ApiError, ErrorCode, Result, UnitId};

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:unit-control")]
    unsafe extern "C" {
        #[link_name = "give-order-to-unit"]
        pub safe fn give_order_to_unit(
            unit_id: i32,
            cmd_id: i32,
            params_pointer: i32,
            param_count: i32,
            options: i32,
            timeout: i32,
        ) -> i64;
    }
}

#[inline]
pub fn give_order_to_unit(
    unit_id: impl Into<UnitId>,
    cmd_id: i32,
    params: &[f32],
    options: u32,
    timeout: i32,
) -> Result<bool> {
    let unit_id = unit_id.into();
    #[cfg(target_arch = "wasm32")]
    {
        let (pointer, count) = super::wasm_slice_parts(params)?;
        let packed =
            raw::give_order_to_unit(unit_id.0, cmd_id, pointer, count, options as i32, timeout)
                as u64;
        let status = (packed >> 32) as u32 as i32;
        if status != 0 {
            return Err(ApiError::new(status));
        }
        match packed as u32 {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(ApiError::new(ErrorCode::Internal as i32)),
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (unit_id, cmd_id, params, options, timeout);
        Err(unreachable!())
    }
}
