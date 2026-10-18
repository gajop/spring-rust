/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */
//! Matched callback bodies, compiled separately through the two public SDKs.
use std::{
    hint::black_box,
    sync::atomic::{AtomicI32, AtomicU32, AtomicU64, Ordering},
};
static UNIT: AtomicI32 = AtomicI32::new(-1);
static QUERIES: AtomicU32 = AtomicU32::new(0);
static KIND: AtomicU32 = AtomicU32::new(0);
static CHECKSUM: AtomicU64 = AtomicU64::new(0);
#[unsafe(no_mangle)]
pub extern "C" fn ConfigureBenchmark(unit: i32, queries: u32) {
    KIND.store(0, Ordering::Relaxed);
    UNIT.store(unit, Ordering::Relaxed);
    QUERIES.store(queries, Ordering::Relaxed);
    CHECKSUM.store(0, Ordering::Relaxed);
}
#[unsafe(no_mangle)]
pub extern "C" fn ConfigureCallout(kind: u32, queries: u32) {
    KIND.store(kind, Ordering::Relaxed);
    QUERIES.store(queries, Ordering::Relaxed);
    CHECKSUM.store(0, Ordering::Relaxed);
}
#[unsafe(no_mangle)]
pub extern "C" fn BenchmarkChecksum() -> u64 {
    CHECKSUM.load(Ordering::Relaxed)
}
fn frame(frame: i32, mut query: impl FnMut(i32, u32) -> u64) {
    let mut sum = black_box(frame) as u64;
    let kind = KIND.load(Ordering::Relaxed);
    let unit = UNIT.load(Ordering::Relaxed);
    for _ in 0..QUERIES.load(Ordering::Relaxed) {
        sum += black_box(query(unit, kind));
    }
    CHECKSUM.store(sum, Ordering::Relaxed);
}

// The macro expands the same operations through each official public API.
macro_rules! query_body {
    ($spring:expr,$unit:expr,$kind:expr,$options:expr,$mutate:expr) => {{
        let spring = $spring;
        match $kind {
            0 => spring.units_info().get_unit_def_id($unit).unwrap() as u64,
            1 => {
                let p = spring
                    .units_info()
                    .get_unit_position($unit, $options)
                    .unwrap();
                black_box((p.x, p.y, p.z));
                u64::from(p.x.to_bits())
            }
            2 => {
                let d = spring.units_info().get_unit_def_id($unit).unwrap();
                let s = spring.unit_defs().get_unit_def_name(d).unwrap();
                #[cfg(feature = "legacy")]
                let s = s.expect("fixture unit definition");
                black_box(&s);
                s.len() as u64
            }
            3 => {
                let v = spring.units_commands().get_unit_commands($unit, 5).unwrap();
                black_box(&v);
                v.len() as u64
            }
            4 => {
                let v = spring.units_query().get_team_units(0).unwrap();
                black_box(&v);
                v.len() as u64
            }
            5 => {
                let v = spring
                    .units_query()
                    .get_units_in_cylinder(100.0, 100.0, 300.0, -1)
                    .unwrap();
                black_box(&v);
                v.len() as u64
            }
            6 => {
                $mutate;
                1
            }
            _ => panic!("unknown callout shape"),
        }
    }};
}

#[cfg(feature = "legacy")]
mod implementation {
    use super::*;
    use spring_native::{Error, NativeInterfaceRef, NativeModule};
    struct App(NativeInterfaceRef);
    impl NativeModule for App {
        fn new(spring: NativeInterfaceRef) -> Self {
            Self(spring)
        }
        fn game_frame(&mut self, n: i32) -> Result<(), Error> {
            frame(n, |unit, kind| {
                query_body!(
                    &self.0,
                    unit,
                    kind,
                    spring_native::GetUnitPositionOptions::default(),
                    self.0
                        .rules_params()
                        .set_unit_rules_param(
                            unit,
                            "bench",
                            spring_native::RulesParamValue::Float(1.0),
                            -1
                        )
                        .unwrap()
                )
            });
            Ok(())
        }
        fn update(&mut self, dt: f32) -> Result<(), Error> {
            black_box(dt);
            Ok(())
        }
        fn unit_created(&mut self, u: i32, d: i32, t: i32, b: i32) -> Result<(), Error> {
            black_box((u, d, t, b));
            Ok(())
        }
        fn unit_pre_damaged(
            &mut self,
            u: i32,
            d: i32,
            t: i32,
            damage: f32,
            p: bool,
            w: i32,
            proj: i32,
            a: Option<i32>,
            ad: Option<i32>,
            at: Option<i32>,
            _: f32,
            _: f32,
        ) -> Result<(f32, f32), Error> {
            black_box((u, d, t, p, w, proj, a, ad, at));
            Ok((damage, 1.0))
        }
        fn allow_unit_creation(
            &mut self,
            d: i32,
            b: i32,
            t: i32,
            pos: Option<spring_native::sys::Float3>,
            f: i32,
        ) -> Result<(bool, bool), Error> {
            black_box((d, b, t, pos, f));
            Ok((true, false))
        }
        fn command_notify(
            &mut self,
            command: spring_native::sys::NativeCallinCommand,
        ) -> Result<bool, Error> {
            let params = if command.numParams == 0 {
                &[]
            } else {
                unsafe { std::slice::from_raw_parts(command.params, command.numParams as usize) }
            };
            black_box((
                command.id,
                command.timeOut,
                command.pageIndex,
                command.tag,
                command.options,
                params,
            ));
            Ok(false)
        }
        fn add_console_line(
            &mut self,
            message: &str,
            section: &str,
            level: i32,
        ) -> Result<bool, Error> {
            black_box((message, section, level));
            Ok(false)
        }
    }
    spring_native::export_module!(App);
}
#[cfg(feature = "unified")]
mod implementation {
    use super::*;
    use spring_sdk::{All, Result, Spring, module::Module};
    struct App;
    impl Module<All> for App {
        fn initialize(_: &mut Spring<'_, All>) -> Result<Self> {
            Ok(Self)
        }
    }
    fn game_frame(_: &App, spring: &mut Spring<'_, All>, n: i32) -> Result<()> {
        frame(n, |unit, kind| {
            query_body!(
                &*spring,
                unit,
                kind,
                spring_sdk::units_info::GetUnitPositionOptions::default(),
                spring
                    .rules_params()
                    .set_unit_rules_param(
                        unit,
                        "bench",
                        &spring_sdk::rules_params::RulesParamValue {
                            type_: spring_sdk::rules_params::RulesParamType::RulesparamTypeFloat,
                            bool_value: false,
                            float_value: 1.0,
                            string_value: String::new()
                        },
                        -1
                    )
                    .unwrap()
            )
        });
        Ok(())
    }
    fn update(_: &App, _: &mut Spring<'_, All>, dt: f32) -> Result<()> {
        black_box(dt);
        Ok(())
    }
    fn created(_: &App, _: &mut Spring<'_, All>, u: i32, d: i32, t: i32, b: i32) -> Result<()> {
        black_box((u, d, t, b));
        Ok(())
    }
    #[allow(clippy::too_many_arguments)] // Public callin signature.
    fn damaged(
        _: &App,
        _: &mut Spring<'_, All>,
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
        black_box((
            u,
            d,
            t,
            p,
            w,
            proj,
            (a >= 0).then_some(a),
            (ad >= 0).then_some(ad),
            (at >= 0).then_some(at),
        ));
        Ok((damage, 1.0))
    }
    #[allow(clippy::too_many_arguments)] // Public callin signature.
    fn creation(
        _: &App,
        _: &mut Spring<'_, All>,
        d: i32,
        b: i32,
        t: i32,
        h: bool,
        pos: spring_sdk::Float3,
        f: i32,
    ) -> Result<(bool, bool)> {
        black_box((d, b, t, h.then_some(pos), f));
        Ok((true, false))
    }
    #[cfg(feature = "borrowed-command")]
    type Command<'a> = spring_sdk::CallinCommand<'a>;
    #[cfg(not(feature = "borrowed-command"))]
    type Command<'a> = spring_sdk::types::callins::NativeCallinCommand;
    fn command(_: &App, _: &mut Spring<'_, All>, command: &Command<'_>) -> Result<bool> {
        #[cfg(feature = "borrowed-command")]
        let params = command.params;
        #[cfg(not(feature = "borrowed-command"))]
        let params = command.params.as_slice();
        black_box((
            command.id,
            command.time_out,
            command.page_index,
            command.tag,
            command.options,
            params,
        ));
        Ok(false)
    }
    spring_sdk::export_command_notify!(APP, command);
    fn console(
        _: &App,
        _: &mut Spring<'_, All>,
        message: &str,
        section: &str,
        level: i32,
    ) -> Result<bool> {
        black_box((message, section, level));
        Ok(false)
    }
    spring_sdk::export_module!(APP, All, App);
    spring_sdk::export_game_frame!(APP, game_frame);
    spring_sdk::export_update!(APP, update);
    spring_sdk::export_unit_created!(APP, created);
    spring_sdk::export_unit_pre_damaged!(APP, damaged);
    spring_sdk::export_allow_unit_creation!(APP, creation);
    spring_sdk::export_add_console_line!(APP, console);
}
