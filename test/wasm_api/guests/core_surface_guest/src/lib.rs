#![allow(static_mut_refs)]

use core::ptr;
use spring as spring;

// Volatile observables keep the compiler from deleting imported calls while
// remaining independent of any allocator/WASI implementation.
#[unsafe(no_mangle)]
pub static mut CORE_SURFACE_CHECKSUM: u64 = 0;
#[unsafe(no_mangle)]
pub static mut CORE_DRAW_CALLS: u32 = 0;

#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "spring:unit-defs")]
unsafe extern "C" {
    #[link_name = "get-unit-def-name"]
    fn raw_get_unit_def_name(unit_def_id: i32, output: i32, capacity: i32) -> i64;
}

#[inline(never)]
fn mix(value: u64) {
    unsafe {
        let old = ptr::read_volatile(ptr::addr_of!(CORE_SURFACE_CHECKSUM));
        ptr::write_volatile(
            ptr::addr_of_mut!(CORE_SURFACE_CHECKSUM),
            old.rotate_left(13) ^ value.wrapping_mul(0x9e37_79b9_7f4a_7c15),
        );
    }
}

fn game_frame(frame: i32) {
    let unit_id = spring::UnitId::from(0);

    if let Ok(value) = spring::get_unit_def_id(unit_id) {
        mix(i32::from(value) as u32 as u64);
    }
    if let Ok(value) = spring::get_unit_team(unit_id) {
        mix(i32::from(value) as u32 as u64);
    }
    if let Ok(value) = spring::get_unit_is_dead(unit_id) {
        mix(value as u64);
    }
    if let Ok(value) = spring::get_unit_experience(unit_id) {
        mix(value.to_bits() as u64);
    }
    if let Ok(value) = spring::get_unit_position(unit_id, true, false) {
        mix(value[0].to_bits() as u64);
        mix(value[1].to_bits() as u64);
        mix(value[2].to_bits() as u64);
    }
    if let Ok(value) = spring::get_unit_velocity(unit_id) {
        mix(value[0].to_bits() as u64);
        mix(value[1].to_bits() as u64);
        mix(value[2].to_bits() as u64);
    }
    if let Ok(value) = spring::get_unit_health(unit_id) {
        mix(value.health.to_bits() as u64);
        mix(value.max_health.to_bits() as u64);
        mix(value.build_progress.to_bits() as u64);
    }

    let mut ids = [0i32; 256];
    if let Ok(fill) = spring::get_all_units_into(&mut ids) {
        match fill {
            spring::BufferFill::Complete(count) => {
                mix(count as u64);
                for &id in &ids[..count.min(ids.len())] {
                    mix(id as u32 as u64);
                }
            }
            spring::BufferFill::Insufficient { required } => mix(required as u64),
        }
    }
    if let Ok(fill) = spring::get_team_units_into(spring::TeamId::from(0), &mut ids) {
        match fill {
            spring::BufferFill::Complete(count) => mix((count as u64) << 1),
            spring::BufferFill::Insufficient { required } => mix((required as u64) << 1),
        }
    }
    if let Ok(fill) = spring::get_units_in_sphere_into(spring::Float3::ZERO, 1024.0, -1, &mut ids) {
        match fill {
            spring::BufferFill::Complete(count) => mix((count as u64) << 2),
            spring::BufferFill::Insufficient { required } => mix((required as u64) << 2),
        }
    }

    // Exercise the raw-byte string ABI directly as well as the safe SDK
    // surface. The SDK exposes the same convention; this fixture keeps its
    // dependency minimal and does not require alloc.
    #[cfg(target_arch = "wasm32")]
    {
        let mut bytes = [0u8; 128];
        let pointer = bytes.as_mut_ptr() as usize as u32 as i32;
        let packed = unsafe { raw_get_unit_def_name(0, pointer, bytes.len() as i32) } as u64;
        let count = packed as u32 as usize;
        let status = (packed >> 32) as u32 as i32;
        mix(status as u32 as u64);
        if status == 0 {
            for &byte in &bytes[..count.min(bytes.len())] {
                mix(byte as u64);
            }
        }
    }

    mix(frame as u32 as u64);
}

fn game_frame_post(frame: i32) {
    mix((frame as u32 as u64) ^ 0xfeed_face);
}

fn update(delta_seconds: f32) {
    mix(delta_seconds.to_bits() as u64);
}

fn unit_created(
    unit_id: spring::UnitId,
    unit_def_id: spring::DefId,
    unit_team: spring::TeamId,
    builder_id: spring::UnitId,
) {
    mix(unit_id.0 as u32 as u64);
    mix(unit_def_id.0 as u32 as u64);
    mix(unit_team.0 as u32 as u64);
    mix(builder_id.0 as u32 as u64);
}

#[allow(clippy::too_many_arguments)]
fn unit_pre_damaged(
    unit_id: spring::UnitId,
    unit_def_id: spring::DefId,
    unit_team: spring::TeamId,
    damage: f32,
    paralyzer: bool,
    weapon_def_id: spring::WeaponDefId,
    projectile_id: spring::ProjectileId,
    attacker_id: spring::UnitId,
    attacker_def_id: spring::DefId,
    attacker_team: spring::TeamId,
) -> spring::DamageResult {
    mix(unit_id.0 as u32 as u64);
    mix(unit_def_id.0 as u32 as u64);
    mix(unit_team.0 as u32 as u64);
    mix(damage.to_bits() as u64);
    mix(paralyzer as u64);
    mix(weapon_def_id.0 as u32 as u64);
    mix(projectile_id.0 as u32 as u64);
    mix(attacker_id.0 as u32 as u64);
    mix(attacker_def_id.0 as u32 as u64);
    mix(attacker_team.0 as u32 as u64);
    spring::DamageResult::unchanged(damage)
}

fn allow_unit_creation(
    unit_def_id: spring::DefId,
    builder_id: spring::UnitId,
    builder_team: spring::TeamId,
    has_build_info: bool,
    build_pos: spring::Float3,
    build_facing: i32,
) -> spring::AllowUnitCreationResult {
    mix(unit_def_id.0 as u32 as u64);
    mix(builder_id.0 as u32 as u64);
    mix(builder_team.0 as u32 as u64);
    mix(has_build_info as u64);
    mix(build_pos.x.to_bits() as u64);
    mix(build_pos.y.to_bits() as u64);
    mix(build_pos.z.to_bits() as u64);
    mix(build_facing as u32 as u64);
    spring::AllowUnitCreationResult::ALLOW
}

fn draw_world() {
    unsafe {
        let value = ptr::read_volatile(ptr::addr_of!(CORE_DRAW_CALLS));
        ptr::write_volatile(ptr::addr_of_mut!(CORE_DRAW_CALLS), value.wrapping_add(1));
    }
}

spring::export_game_frame!(game_frame);
spring::export_game_frame_post!(game_frame_post);
spring::export_update!(update);
spring::export_unit_created!(unit_created);
spring::export_unit_pre_damaged!(unit_pre_damaged);
spring::export_allow_unit_creation!(allow_unit_creation);
spring::export_draw_world!(draw_world);
