use std::hint::black_box;
use std::sync::atomic::{AtomicBool, Ordering};

use spring_wasm_core as spring;

#[cfg(benchmark_context_unsynced)]
spring::export_environment_mask!(spring::rules_unsynced::ENVIRONMENT_MASK);
#[cfg(benchmark_context_ui)]
spring::export_environment_mask!(spring::ui::ENVIRONMENT_MASK);
#[cfg(not(any(benchmark_context_unsynced, benchmark_context_ui)))]
spring::export_environment_mask!(spring::rules_synced::ENVIRONMENT_MASK);

mod common;

// The representative variable-callin exports are transport fixtures, not tied
// to a particular world. AddConsoleLine and CommandNotify are dispatched by
// NativeInterface through the unsynced worlds, so they must remain available
// when the benchmark guest is projected as an unsynced gadget.
#[cfg(any(benchmark_callin_consoleline, benchmark_callin_commandnotify))]
mod variable_callins;

#[cfg(not(any(benchmark_context_unsynced, benchmark_context_ui)))]
mod callouts;
#[cfg(not(any(benchmark_context_unsynced, benchmark_context_ui)))]
mod heightmap;
#[cfg(not(any(benchmark_context_unsynced, benchmark_context_ui)))]
mod workloads;

#[cfg(benchmark_context_unsynced)]
mod memory;

#[cfg(benchmark_context_ui)]
mod draw;

static RAN: AtomicBool = AtomicBool::new(false);

#[cfg(not(any(benchmark_context_unsynced, benchmark_context_ui)))]
fn on_game_frame(frame: i32) {
    if common::benchmark_case() == Some("callins") {
        if common::callin_variant() == Some("gameframe") {
            black_box(frame);
        }
        return;
    }
    if frame < 3 {
        return;
    }

    if common::benchmark_case() == Some("workloads") {
        if RAN.load(Ordering::Acquire) {
            return;
        }
        match workloads::step() {
            Ok(true) => RAN.store(true, Ordering::Release),
            Ok(false) => {}
            Err(error) => {
                RAN.store(true, Ordering::Release);
                common::send_error(error.code);
            }
        }
        return;
    }

    if RAN.swap(true, Ordering::AcqRel) {
        return;
    }
    let result = match common::benchmark_case() {
        Some("callout_scalar") => callouts::run(true),
        Some("callouts") => callouts::run(false),
        Some("heightmap") => heightmap::run(common::scale()).map(|_| {
            common::send_complete("heightmap");
        }),
        Some("callins") => Ok(()),
        Some(_) => callouts::run(false),
        None => callouts::run(false),
    };
    if let Err(error) = result {
        common::send_error(error.code);
    }
}

#[cfg(all(
    not(any(benchmark_context_unsynced, benchmark_context_ui)),
    not(benchmark_callin_unimplemented)
))]
spring::export_game_frame!(on_game_frame);

#[cfg(not(any(benchmark_context_unsynced, benchmark_context_ui)))]
fn on_game_frame_post(_frame: i32) {}
#[cfg(not(any(benchmark_context_unsynced, benchmark_context_ui)))]
spring::export_game_frame_post!(on_game_frame_post);

#[cfg(not(any(benchmark_context_unsynced, benchmark_context_ui)))]
fn on_unit_created(unit_id: i32, unit_def_id: i32, unit_team: i32, builder_id: i32) {
    black_box((unit_id, unit_def_id, unit_team, builder_id));
}
#[cfg(not(any(benchmark_context_unsynced, benchmark_context_ui)))]
spring::export_unit_created!(on_unit_created);

#[cfg(not(any(benchmark_context_unsynced, benchmark_context_ui)))]
fn on_unit_pre_damaged(
    unit_id: i32,
    unit_def_id: i32,
    unit_team: i32,
    damage: f32,
    paralyzer: bool,
    weapon_def_id: i32,
    projectile_id: i32,
    attacker_id: i32,
    attacker_def_id: i32,
    attacker_team: i32,
) -> spring::DamageResult {
    black_box((
        unit_id,
        unit_def_id,
        unit_team,
        paralyzer,
        weapon_def_id,
        projectile_id,
        attacker_id,
        attacker_def_id,
        attacker_team,
    ));
    spring::DamageResult::unchanged(damage)
}
#[cfg(not(any(benchmark_context_unsynced, benchmark_context_ui)))]
spring::export_unit_pre_damaged!(on_unit_pre_damaged);

#[cfg(not(any(benchmark_context_unsynced, benchmark_context_ui)))]
fn on_allow_unit_creation(
    unit_def_id: i32,
    builder_id: i32,
    builder_team: i32,
    has_build_info: bool,
    build_pos: [f32; 3],
    build_facing: i32,
) -> spring::AllowUnitCreationResult {
    black_box((
        unit_def_id,
        builder_id,
        builder_team,
        has_build_info,
        build_pos,
        build_facing,
    ));
    spring::AllowUnitCreationResult::ALLOW
}
#[cfg(not(any(benchmark_context_unsynced, benchmark_context_ui)))]
spring::export_allow_unit_creation!(on_allow_unit_creation);

#[cfg(not(any(benchmark_context_unsynced, benchmark_context_ui)))]
#[export_name = "spring:callback/dispatch"]
pub extern "C" fn spring_callback_dispatch(callback_id: i32, _user_data: i32) {
    match callback_id as u32 {
        1 => heightmap::callback(),
        _ => panic!("unknown Core benchmark callback id {callback_id}"),
    }
}

#[cfg(benchmark_context_unsynced)]
fn on_update(delta_seconds: f32) {
    if common::benchmark_case() == Some("callins") {
        black_box(delta_seconds);
        return;
    }
    if RAN.load(Ordering::Acquire) {
        return;
    }
    let has_units = spring::get_team_units(0)
        .map(|units| !units.is_empty())
        .unwrap_or(false);
    if !has_units {
        return;
    }
    RAN.store(true, Ordering::Release);
    if let Err(error) = memory::run() {
        common::send_error(error.code);
    }
}
#[cfg(benchmark_context_unsynced)]
spring::export_update!(on_update);

#[cfg(benchmark_context_unsynced)]
fn on_unsynced_game_frame(_frame: i32) {}
#[cfg(benchmark_context_unsynced)]
spring::export_game_frame!(on_unsynced_game_frame);

#[cfg(benchmark_context_unsynced)]
fn on_unsynced_game_frame_post(_frame: i32) {}
#[cfg(benchmark_context_unsynced)]
spring::export_game_frame_post!(on_unsynced_game_frame_post);

#[cfg(benchmark_context_ui)]
fn on_draw_world() {
    if common::benchmark_case() != Some("draw") || RAN.swap(true, Ordering::AcqRel) {
        return;
    }
    if let Err(error) = draw::run() {
        common::send_draw_row(&format!(
            "{{\"backend\":\"wasm_core\",\"test\":\"complete\",\"status\":\"error\",\"code\":{}}}",
            error.code
        ));
    }
}
#[cfg(benchmark_context_ui)]
spring::export_draw_world!(on_draw_world);

#[cfg(benchmark_context_ui)]
#[export_name = "spring:callback/dispatch"]
pub extern "C" fn spring_callback_dispatch(callback_id: i32, _user_data: i32) {
    match callback_id as u32 {
        2 => draw::callback(),
        _ => panic!("unknown Core benchmark callback id {callback_id}"),
    }
}
