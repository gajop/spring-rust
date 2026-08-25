//! Small semantic conveniences built on the generated owned Core façade.
//!
//! These functions exist only where a game wants one stable type across
//! several generated modules. They deliberately contain no ABI encoding or
//! pointer handling; that belongs to the generated engine SDK.

use alloc::string::String;

use crate::generated;

pub use generated::owned::types::Float3;

#[derive(Clone, Copy, Debug)]
pub struct UnitDefRef<'a> {
    pub name: &'a str,
    pub id: i32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CreateUnitOptions {
    pub build: bool,
    pub flatten_ground: bool,
    pub unit_id: i32,
    pub builder_id: i32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CameraState<'a> {
    pub name: &'a str,
    pub position: Float3,
    pub direction: Float3,
    pub up: Float3,
    pub right: Float3,
    pub fov: f32,
    pub rx: f32,
    pub ry: f32,
    pub rz: f32,
    pub dist: f32,
    pub height: f32,
    pub angle: f32,
    pub mode: i32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ProjectileParams<'a> {
    pub position: Float3,
    pub speed: Float3,
    pub spread: Float3,
    pub error: Float3,
    pub end: Float3,
    pub owner: i32,
    pub team: i32,
    pub weapon_number: i32,
    pub ttl: f32,
    pub gravity: f32,
    pub tracking: f32,
    pub max_range: f32,
    pub up_time: f32,
    pub start_alpha: f32,
    pub end_alpha: f32,
    pub model: &'a str,
    pub ceg_tag: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct UnitScriptCallResult {
    pub function_found: bool,
    pub success: bool,
    pub ret_values: alloc::vec::Vec<f32>,
}

pub fn call_unit_script(
    unit_id: i32,
    function_name: &str,
    args: &[f32],
    ret_capacity: usize,
) -> crate::Result<UnitScriptCallResult> {
    let result = generated::owned::unit_script::call_unit_script(
        unit_id,
        function_name,
        args,
        ret_capacity,
    )?;
    Ok(UnitScriptCallResult {
        function_found: result.function_found,
        success: result.success,
        ret_values: result.ret_values,
    })
}

pub fn create_unit(
    unit_def: UnitDefRef<'_>,
    position: Float3,
    team_id: i32,
    options: CreateUnitOptions,
) -> crate::Result<i32> {
    let unit_def = generated::owned::unit_control::DefRef {
        name: String::from(unit_def.name),
        id: unit_def.id,
    };
    let options = generated::owned::unit_control::CreateUnitOptions {
        build: options.build,
        flatten_ground: options.flatten_ground,
        unit_id: options.unit_id,
        builder_id: options.builder_id,
    };
    generated::owned::unit_control::create_unit(&unit_def, position, 0, team_id, options)
}

pub fn set_camera_state(
    state: CameraState<'_>,
    transition_time: f32,
    transition_time_factor: f32,
    transition_time_exponent: f32,
) -> crate::Result<bool> {
    let state = generated::owned::camera::CameraState {
        name: String::from(state.name),
        pos: state.position,
        dir: state.direction,
        up: state.up,
        right: state.right,
        fov: state.fov,
        rx: state.rx,
        ry: state.ry,
        rz: state.rz,
        dist: state.dist,
        height: state.height,
        angle: state.angle,
        mode: state.mode,
    };
    generated::owned::camera::set_camera_state(
        &state,
        transition_time,
        transition_time_factor,
        transition_time_exponent,
    )
}

pub fn spawn_projectile(weapon_def_id: i32, params: ProjectileParams<'_>) -> crate::Result<i32> {
    let params = generated::owned::projectile_control::NativeProjectileParams {
        pos: params.position,
        speed: params.speed,
        spread: params.spread,
        error: params.error,
        end: params.end,
        owner: params.owner,
        team: params.team,
        weapon_num: params.weapon_number,
        ttl: params.ttl,
        gravity: params.gravity,
        tracking: params.tracking,
        max_range: params.max_range,
        up_time: params.up_time,
        start_alpha: params.start_alpha,
        end_alpha: params.end_alpha,
        model: String::from(params.model),
        ceg_tag: String::from(params.ceg_tag),
    };
    generated::owned::projectile_control::spawn_projectile(weapon_def_id, &params)
}

pub fn unit_piece_position_by_name(unit_id: i32, name: &str) -> Option<Float3> {
    let piece_number = generated::owned::units_pieces::get_unit_piece_map(unit_id)
        .ok()?
        .into_iter()
        .find(|piece| piece.name == name)
        .map(|piece| piece.piece_num)?;
    let piece =
        generated::owned::units_pieces::get_unit_piece_pos_dir(unit_id, piece_number).ok()?;
    Some(piece.position)
}

pub struct SpawnCEGResult {
    pub success: bool,
    pub ceg_id: i32,
}

pub fn spawn_ceg(
    ceg_name: &str,
    pos: Float3,
    dir: Float3,
    radius: f32,
    damage: f32,
    dmg_mod: f32,
) -> crate::Result<SpawnCEGResult> {
    let ceg = generated::owned::effects_control::DefRef {
        name: String::from(ceg_name),
        id: 0,
    };
    let result =
        generated::owned::effects_control::spawn_ceg(&ceg, pos, dir, radius, damage, dmg_mod)?;
    Ok(SpawnCEGResult {
        success: result.success,
        ceg_id: result.ceg_id,
    })
}
