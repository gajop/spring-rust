//! Typed guest-facing helpers for structured Core inputs.
//!
//! The generated ABI still uses validated byte slices for nested values. That
//! representation is kept private here so guests use ordinary Rust structs.

use alloc::vec::Vec;

use crate::generated;

#[derive(Clone, Copy, Debug, Default)]
pub struct Float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

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

pub fn create_unit(
    unit_def: UnitDefRef<'_>,
    position: Float3,
    team_id: i32,
    options: CreateUnitOptions,
) -> crate::Result<i32> {
    let def = def_wire(unit_def);
    let position = float3_wire(position);
    let options = create_unit_options_wire(options);
    generated::dynamic_input::unit_control::create_unit(0, team_id, &def, &position, &options)
}

pub fn set_camera_state(
    state: CameraState<'_>,
    transition_time: f32,
    transition_time_factor: f32,
    transition_time_exponent: f32,
) -> crate::Result<bool> {
    let wire = camera_state_wire(state);
    generated::dynamic_input::camera::set_camera_state(
        transition_time,
        transition_time_factor,
        transition_time_exponent,
        &wire,
    )
}

pub fn spawn_projectile(weapon_def_id: i32, params: ProjectileParams<'_>) -> crate::Result<i32> {
    let wire = projectile_params_wire(params);
    generated::dynamic_input::projectile_control::spawn_projectile(weapon_def_id, &wire)
}

pub fn unit_piece_position_by_name(unit_id: i32, name: &str) -> Option<Float3> {
    let piece_number = unit_piece_number(unit_id, name).ok()??;
    let piece =
        generated::owned::units_pieces::get_unit_piece_pos_dir(unit_id, piece_number).ok()?;
    Some(Float3 {
        x: piece.position.x,
        y: piece.position.y,
        z: piece.position.z,
    })
}

fn def_wire(unit_def: UnitDefRef<'_>) -> Vec<u8> {
    let mut bytes = Vec::new();
    put_u32(&mut bytes, unit_def.name.len() as u32);
    bytes.extend_from_slice(unit_def.name.as_bytes());
    put_i32(&mut bytes, unit_def.id);
    bytes
}

fn float3_wire(value: Float3) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(12);
    put_f32(&mut bytes, value.x);
    put_f32(&mut bytes, value.y);
    put_f32(&mut bytes, value.z);
    bytes
}

fn create_unit_options_wire(options: CreateUnitOptions) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(16);
    put_u32(&mut bytes, u32::from(options.build));
    put_u32(&mut bytes, u32::from(options.flatten_ground));
    put_i32(&mut bytes, options.unit_id);
    put_i32(&mut bytes, options.builder_id);
    bytes
}

fn camera_state_wire(state: CameraState<'_>) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(100);
    put_u32(&mut bytes, state.name.len() as u32);
    bytes.extend_from_slice(state.name.as_bytes());
    put_f32(&mut bytes, state.position.x);
    put_f32(&mut bytes, state.position.y);
    put_f32(&mut bytes, state.position.z);
    put_f32(&mut bytes, state.direction.x);
    put_f32(&mut bytes, state.direction.y);
    put_f32(&mut bytes, state.direction.z);
    put_f32(&mut bytes, state.up.x);
    put_f32(&mut bytes, state.up.y);
    put_f32(&mut bytes, state.up.z);
    put_f32(&mut bytes, state.right.x);
    put_f32(&mut bytes, state.right.y);
    put_f32(&mut bytes, state.right.z);
    put_f32(&mut bytes, state.fov);
    put_f32(&mut bytes, state.rx);
    put_f32(&mut bytes, state.ry);
    put_f32(&mut bytes, state.rz);
    put_f32(&mut bytes, state.dist);
    put_f32(&mut bytes, state.height);
    put_f32(&mut bytes, state.angle);
    put_i32(&mut bytes, state.mode);
    bytes
}

fn projectile_params_wire(params: ProjectileParams<'_>) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(128);
    put_float3(&mut bytes, params.position);
    put_float3(&mut bytes, params.speed);
    put_float3(&mut bytes, params.spread);
    put_float3(&mut bytes, params.end);
    put_i32(&mut bytes, params.owner);
    put_i32(&mut bytes, params.team);
    put_i32(&mut bytes, params.weapon_number);
    put_f32(&mut bytes, params.ttl);
    put_f32(&mut bytes, params.gravity);
    put_f32(&mut bytes, params.tracking);
    put_f32(&mut bytes, params.max_range);
    put_f32(&mut bytes, params.up_time);
    put_f32(&mut bytes, params.start_alpha);
    put_f32(&mut bytes, params.end_alpha);
    put_string(&mut bytes, params.model);
    put_string(&mut bytes, params.ceg_tag);
    bytes
}

fn put_float3(bytes: &mut Vec<u8>, value: Float3) {
    put_f32(bytes, value.x);
    put_f32(bytes, value.y);
    put_f32(bytes, value.z);
}

fn put_string(bytes: &mut Vec<u8>, value: &str) {
    put_u32(bytes, value.len() as u32);
    bytes.extend_from_slice(value.as_bytes());
}

fn unit_piece_number(unit_id: i32, name: &str) -> crate::Result<Option<i32>> {
    let mut bytes = Vec::new();
    let required =
        match generated::dynamic_output::units_pieces::get_unit_piece_map(unit_id, &mut bytes) {
            Ok(length) => length,
            Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                error.required
            }
            Err(error) => return Err(error.error),
        };
    bytes.resize(required, 0);
    let length = generated::dynamic_output::units_pieces::get_unit_piece_map(unit_id, &mut bytes)
        .map_err(|error| error.error)?;
    bytes.truncate(length);

    let mut offset = 0;
    let count = read_u32(&bytes, &mut offset)?;
    for _ in 0..count {
        let name_length = read_u32(&bytes, &mut offset)? as usize;
        let name_end = offset.checked_add(name_length).ok_or_else(invalid_wire)?;
        let piece_name = bytes.get(offset..name_end).ok_or_else(invalid_wire)?;
        offset = name_end;
        let piece_number = read_i32(&bytes, &mut offset)?;
        if piece_name == name.as_bytes() {
            return Ok(Some(piece_number));
        }
    }
    Ok(None)
}

fn read_u32(bytes: &[u8], offset: &mut usize) -> crate::Result<u32> {
    *offset = align_offset(*offset).ok_or_else(invalid_wire)?;
    let end = offset.checked_add(4).ok_or_else(invalid_wire)?;
    let value = bytes.get(*offset..end).ok_or_else(invalid_wire)?;
    *offset = end;
    Ok(u32::from_le_bytes(
        value.try_into().map_err(|_| invalid_wire())?,
    ))
}

fn read_i32(bytes: &[u8], offset: &mut usize) -> crate::Result<i32> {
    Ok(read_u32(bytes, offset)? as i32)
}

fn align_offset(offset: usize) -> Option<usize> {
    offset.checked_add(3).map(|value| value & !3)
}

fn invalid_wire() -> crate::ApiError {
    crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)
}

fn align4(bytes: &mut Vec<u8>) {
    while !bytes.len().is_multiple_of(4) {
        bytes.push(0);
    }
}

fn put_u32(bytes: &mut Vec<u8>, value: u32) {
    align4(bytes);
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn put_i32(bytes: &mut Vec<u8>, value: i32) {
    put_u32(bytes, value as u32);
}

fn put_f32(bytes: &mut Vec<u8>, value: f32) {
    put_u32(bytes, value.to_bits());
}
