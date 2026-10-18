/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#![cfg_attr(benchmark_callin_unimplemented, allow(dead_code))]

use spring::{Result, Spring, module::Module};

use spring_sdk as spring;
#[cfg(any(not(target_arch = "wasm32"), not(benchmark_context_ui)))]
use std::hint::black_box;
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(not(target_arch = "wasm32"))]
type Environment = spring::All;
#[cfg(all(target_arch = "wasm32", benchmark_context_unsynced))]
type Environment = spring::RulesUnsynced;
#[cfg(all(target_arch = "wasm32", benchmark_context_ui))]
type Environment = spring::Ui;
#[cfg(all(
    target_arch = "wasm32",
    not(any(benchmark_context_unsynced, benchmark_context_ui))
))]
type Environment = spring::RulesSynced;
type Engine<'a> = Spring<'a, Environment>;
struct App;
impl Module<Environment> for App {
    fn initialize(_: &mut Engine<'_>) -> Result<Self> {
        Ok(Self)
    }
}
spring::export_module!(APP, Environment, App);
#[cfg(target_arch = "wasm32")]
spring::export_callin_scratch!(4096);
#[cfg(any(
    not(target_arch = "wasm32"),
    not(any(benchmark_context_unsynced, benchmark_context_ui))
))]
mod callouts;
mod common;
#[cfg(any(not(target_arch = "wasm32"), benchmark_context_ui))]
mod draw;
#[cfg(any(
    not(target_arch = "wasm32"),
    not(any(benchmark_context_unsynced, benchmark_context_ui))
))]
mod heightmap;
#[cfg(any(not(target_arch = "wasm32"), benchmark_context_unsynced))]
mod memory;
#[cfg(any(
    not(target_arch = "wasm32"),
    not(any(benchmark_context_unsynced, benchmark_context_ui))
))]
mod workloads;
static RAN: AtomicBool = AtomicBool::new(false);

#[cfg(any(
    not(target_arch = "wasm32"),
    not(any(benchmark_context_unsynced, benchmark_context_ui))
))]
fn game_frame(_: &App, spring: &mut Engine<'_>, frame: i32) -> Result<()> {
    if common::benchmark_case() == Some("callins") {
        if common::callin_variant() == Some("gameframe") {
            black_box(frame);
        }
        return Ok(());
    }
    if frame < 3 || RAN.load(Ordering::Relaxed) {
        return Ok(());
    }
    if common::benchmark_case() == Some("workloads") {
        if workloads::step(spring)? {
            RAN.store(true, Ordering::Relaxed);
        }
        return Ok(());
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Ok(())
    } // Lua starts the native benchmark at the established boundary.
    #[cfg(target_arch = "wasm32")]
    run(spring)
}
#[cfg(all(
    not(benchmark_callin_unimplemented),
    any(
        not(target_arch = "wasm32"),
        not(any(benchmark_context_unsynced, benchmark_context_ui))
    )
))]
spring::export_game_frame!(APP, game_frame);

#[cfg(any(
    not(target_arch = "wasm32"),
    not(any(benchmark_context_unsynced, benchmark_context_ui))
))]
fn run(spring: &mut Engine<'_>) -> Result<()> {
    if RAN.swap(true, Ordering::Relaxed) {
        return Ok(());
    }
    match common::benchmark_case() {
        Some("heightmap") => {
            heightmap::run(spring, common::scale())?;
            common::send_complete(spring, "heightmap");
        }
        Some("callout_scalar") => callouts::run(spring, true)?,
        Some("workloads" | "memory" | "draw" | "callins") => {
            RAN.store(false, Ordering::Relaxed);
        }
        _ => callouts::run(spring, false)?,
    }
    Ok(())
}
#[cfg(not(target_arch = "wasm32"))]
fn lua_call(_: &App, spring: &mut Engine<'_>, _: &[u8]) -> Result<()> {
    if common::benchmark_case() == Some("memory") {
        if !RAN.swap(true, Ordering::Relaxed) {
            memory::run(spring)?;
        }
        Ok(())
    } else {
        run(spring)
    }
}
#[cfg(not(target_arch = "wasm32"))]
spring::export_handle_lua_call!(APP, lua_call);

#[cfg(any(not(target_arch = "wasm32"), benchmark_context_unsynced))]
fn update(_: &App, spring: &mut Engine<'_>, dt: f32) -> Result<()> {
    if common::benchmark_case() == Some("callins") {
        black_box(dt);
        return Ok(());
    }
    #[cfg(target_arch = "wasm32")]
    if !RAN.load(Ordering::Relaxed) && !spring.units().get_team_units(0)?.is_empty() {
        RAN.store(true, Ordering::Relaxed);
        memory::run(spring)?;
    }
    let _ = spring;
    Ok(())
}
#[cfg(any(not(target_arch = "wasm32"), benchmark_context_unsynced))]
spring::export_update!(APP, update);

#[cfg(any(not(target_arch = "wasm32"), benchmark_context_ui))]
fn draw_world(_: &App, spring: &mut Engine<'_>) -> Result<()> {
    if common::benchmark_case() == Some("draw") && !RAN.swap(true, Ordering::Relaxed) {
        draw::run(spring)?;
    }
    Ok(())
}
#[cfg(any(not(target_arch = "wasm32"), benchmark_context_ui))]
spring::export_draw_world!(APP, draw_world);

#[cfg(any(
    not(target_arch = "wasm32"),
    not(any(benchmark_context_unsynced, benchmark_context_ui))
))]
fn unit_created(_: &App, _: &mut Engine<'_>, u: i32, d: i32, t: i32, b: i32) -> Result<()> {
    black_box((u, d, t, b));
    Ok(())
}
#[cfg(any(
    not(target_arch = "wasm32"),
    not(any(benchmark_context_unsynced, benchmark_context_ui))
))]
spring::export_unit_created!(APP, unit_created);
#[cfg(any(
    not(target_arch = "wasm32"),
    not(any(benchmark_context_unsynced, benchmark_context_ui))
))]
#[allow(clippy::too_many_arguments)] // Engine callback signature.
fn pre_damaged(
    _: &App,
    _: &mut Engine<'_>,
    u: i32,
    d: i32,
    t: i32,
    damage: f32,
    p: bool,
    w: i32,
    proj: i32,
    a: i32,
    ad: i32,
    at: i32,
) -> Result<(f32, f32)> {
    black_box((u, d, t, p, w, proj, a, ad, at));
    Ok((damage, 1.0))
}
#[cfg(any(
    not(target_arch = "wasm32"),
    not(any(benchmark_context_unsynced, benchmark_context_ui))
))]
spring::export_unit_pre_damaged!(APP, pre_damaged);
#[cfg(any(
    not(target_arch = "wasm32"),
    not(any(benchmark_context_unsynced, benchmark_context_ui))
))]
#[allow(clippy::too_many_arguments)] // Engine callback signature.
fn allow_creation(
    _: &App,
    _: &mut Engine<'_>,
    d: i32,
    b: i32,
    t: i32,
    h: bool,
    pos: spring::Float3,
    f: i32,
) -> Result<(bool, bool)> {
    black_box((d, b, t, h, pos, f));
    Ok((true, false))
}
#[cfg(any(
    not(target_arch = "wasm32"),
    not(any(benchmark_context_unsynced, benchmark_context_ui))
))]
spring::export_allow_unit_creation!(APP, allow_creation);
