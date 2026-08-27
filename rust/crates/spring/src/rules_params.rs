#[cfg(feature = "alloc")]
pub use crate::owned::rules_params::{
    get_feature_rules_param, get_feature_rules_params, get_game_rules_param, get_game_rules_params,
    get_player_rules_param, get_player_rules_params, get_team_rules_param, get_team_rules_params,
    get_unit_rules_param, get_unit_rules_params, set_feature_rules_param, set_game_rules_param,
    set_player_rules_param, set_team_rules_param, set_unit_rules_param,
};

// RulesParams portion of the Spring Core-Wasm guest SDK.

use super::{Result, UnitId, decode_packed_f32, unpack_bool};

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:rules-params")]
    unsafe extern "C" {
        #[link_name = "get-unit-rules-param-f32"]
        pub safe fn get_unit_rules_param_f32(unit_id: i32, name: i32, name_len: i32) -> i64;
        #[link_name = "set-unit-rules-param-f32"]
        pub safe fn set_unit_rules_param_f32(
            unit_id: i32,
            name: i32,
            name_len: i32,
            value: f32,
            los: i32,
        ) -> i64;
    }
}

#[cfg(target_arch = "wasm32")]
#[inline]
fn string_parts(value: &str) -> Result<(i32, i32)> {
    super::wasm_slice_parts(value.as_bytes())
}

#[inline]
pub fn get_unit_rules_param_f32(unit_id: impl Into<UnitId>, name: &str) -> Result<f32> {
    let unit_id = unit_id.into();
    #[cfg(target_arch = "wasm32")]
    {
        let (pointer, length) = string_parts(name)?;
        decode_packed_f32(raw::get_unit_rules_param_f32(unit_id.0, pointer, length))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (unit_id, name);
        Err(unreachable!())
    }
}

#[inline]
pub fn set_unit_rules_param_f32(
    unit_id: impl Into<UnitId>,
    name: &str,
    value: f32,
    los: i32,
) -> Result<bool> {
    let unit_id = unit_id.into();
    #[cfg(target_arch = "wasm32")]
    {
        let (pointer, length) = string_parts(name)?;
        unpack_bool(raw::set_unit_rules_param_f32(
            unit_id.0, pointer, length, value, los,
        ))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (unit_id, name, value, los);
        Err(unreachable!())
    }
}
