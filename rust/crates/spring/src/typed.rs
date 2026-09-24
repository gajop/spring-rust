//! Small semantic conveniences built on the generated owned Core façade.
//!
//! These wrappers accept borrowed data (`&str`) and construct the generated
//! owned values. ABI encoding remains an implementation detail of the engine's
//! generated layer.

extern crate alloc;
use alloc::string::String;

use crate::Float3;
use crate::generated;

pub use generated::owned::move_ctrl::{MoveTypeBooleanField, MoveTypeNumericField};

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
    /// A non-positive value leaves the engine's current field of view unchanged.
    pub fov: f32,
    pub rx: f32,
    pub ry: f32,
    pub rz: f32,
    /// A non-positive value leaves the engine's current camera distance unchanged.
    pub dist: f32,
    pub height: f32,
    pub angle: f32,
    pub mode: i32,
}

impl<'a> CameraState<'a> {
    /// A camera at `position` looking along `direction`, with `up` and `right`
    /// derived from it (no roll). The other fields keep their defaults: a zero
    /// `fov` or `dist` leaves the engine's value unchanged. Set `mode`, `height`,
    /// `angle` and so on with struct update syntax:
    ///
    /// ```rust,ignore
    /// let camera = spring::CameraState {
    ///     fov: 45.0,
    ///     mode: 1,
    ///     ..spring::CameraState::new("spring", position, direction)
    /// };
    /// ```
    pub fn new(name: &'a str, position: Float3, direction: Float3) -> Self {
        let direction = direction.normalized();
        let mut right = direction.cross(Float3::new(0.0, 1.0, 0.0)).normalized();
        if right == Float3::ZERO {
            // Looking straight up or down.
            right = Float3::new(1.0, 0.0, 0.0);
        }
        Self {
            name,
            position,
            direction,
            up: right.cross(direction),
            right,
            ..Default::default()
        }
    }
}

/// Background music position, from Lua's `Spring.GetSoundStreamTime`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SoundStreamTime {
    /// Seconds played so far.
    pub played: f32,
    /// Length of the stream in seconds.
    pub length: f32,
}

impl SoundStreamTime {
    /// Whether the stream has played to its end (or nothing is playing).
    pub fn finished(self) -> bool {
        self.played >= self.length
    }
}

pub fn sound_stream_time() -> crate::Result<SoundStreamTime> {
    Ok(SoundStreamTime {
        played: generated::owned::sound::get_sound_stream_play_time(0)?,
        length: generated::owned::sound::get_sound_stream_time(0)?,
    })
}

/// Reload the game's synced and unsynced rules (`/luarules reload`), turning
/// cheats on only for the reload if they were off, and leaving them as they
/// were.
pub fn reload_rules() -> crate::Result<bool> {
    if generated::owned::game::is_cheating_enabled(0)? {
        crate::send_command("luarules reload")
    } else {
        crate::send_command_lines(&["cheat 1", "luarules reload", "cheat 0"])
    }
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

/// Set one of the numeric properties accepted by Lua's
/// `MoveCtrl.Set*MoveTypeData` family.
pub fn set_move_type_numeric(
    unit_id: i32,
    field: MoveTypeNumericField,
    value: f32,
) -> crate::Result<bool> {
    generated::owned::move_ctrl::set_move_type_numeric(unit_id, field, value)
}

/// Set one of the boolean properties accepted by Lua's
/// `MoveCtrl.Set*MoveTypeData` family.
pub fn set_move_type_boolean(
    unit_id: i32,
    field: MoveTypeBooleanField,
    value: bool,
) -> crate::Result<bool> {
    generated::owned::move_ctrl::set_move_type_boolean(unit_id, field, value)
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
    generated::owned::units_pieces::get_unit_piece_position(unit_id, piece_number).ok()
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
        // DefRef uses a nonnegative ID for numeric lookup; -1 selects the name.
        id: -1,
    };
    let result =
        generated::owned::effects_control::spawn_ceg(&ceg, pos, dir, radius, damage, dmg_mod)?;
    Ok(SpawnCEGResult {
        success: result.success,
        ceg_id: result.ceg_id,
    })
}
