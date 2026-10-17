/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */
mod game {
    #[cfg(feature = "all")]
    use spring_sdk::All as GameEnvironment;
    #[cfg(not(feature = "all"))]
    use spring_sdk::RulesSynced as GameEnvironment;
    #[cfg(feature = "all")]
    use spring_sdk::cus::{host::CusHost, prelude::*};
    use spring_sdk::{Float3, Result, Spring, module::Module};
    use std::cell::Cell;
    #[cfg(feature = "all")]
    #[derive(Default)]
    struct ProbeScript {
        done: Shared<SharedCell<bool>>,
    }
    #[cfg(feature = "all")]
    async fn animate(done: Shared<SharedCell<bool>>, ctx: UnitCtx) {
        ctx.sleep_frames(2).await;
        ctx.move_now(Piece(0), Axis::Y, 8.0);
        ctx.with_spring(|spring| {
            assert!(spring.game().current_frame()? > 0);
            Ok(())
        })
        .unwrap();
        *done.borrow_mut() = true;
    }
    #[cfg(feature = "all")]
    impl UnitScript for ProbeScript {
        fn create(&mut self, ctx: &UnitCtx) {
            ctx.move_now(Piece(0), Axis::Y, 4.0);
            ctx.spawn(TaskDefinition::with_state(
                "probe",
                self.done.clone(),
                animate,
            ));
        }
        fn call_named(
            &mut self,
            _: &UnitCtx,
            name: &str,
            args: &[f32],
            returns: &mut [f32],
        ) -> Option<usize> {
            if name != "double" {
                return None;
            }
            returns[0] = args[0] * 2.0;
            Some(1)
        }
    }
    struct Game {
        seen: Cell<bool>,
        last_frame: Cell<i32>,
        #[cfg(feature = "all")]
        drawn: Cell<bool>,
        #[cfg(not(target_arch = "wasm32"))]
        initialized_on: std::thread::ThreadId,
        #[cfg(feature = "all")]
        scripts: CusHost<ProbeScript, (), GameEnvironment>,
        #[cfg(feature = "all")]
        unit: Cell<i32>,
        #[cfg(feature = "all")]
        cus_checked: Cell<bool>,
    }
    impl Module<GameEnvironment> for Game {
        fn initialize(spring: &mut Spring<'_, GameEnvironment>) -> Result<Self> {
            if cfg!(feature = "fail-init") {
                return Err(spring_sdk::ApiError::new(
                    spring_sdk::ErrorCode::InvalidArgument as i32,
                ));
            }
            #[cfg(feature = "all")]
            {
                // This optional clock must link through the production import,
                // including when synced timing is disabled. Do not use it in
                // simulation decisions.
                let clock = spring.profiling().get_timer_micros();
                assert!(
                    clock.is_ok()
                        || clock.unwrap_err().code == spring_sdk::ErrorCode::NotAvailable as i32
                );
                assert_eq!(
                    spring.profiling().diff_timers(
                        2000,
                        1000,
                        spring_sdk::profiling::DiffTimersOptions {
                            return_ms: true,
                            from_micro_secs: true
                        }
                    )?,
                    1.0
                );
            }
            let result = spring.math_extra().normalize(Float3 {
                x: 3.0,
                y: 4.0,
                z: 0.0,
            })?;
            assert_eq!(result.length, 5.0);
            assert_eq!(
                result.vec,
                Float3 {
                    x: 0.6,
                    y: 0.8,
                    z: 0.0
                }
            );
            assert!(spring.messages().echo("[ScopedSdkGame] initialized", "")?);
            Ok(Self {
                seen: Cell::new(false),
                last_frame: Cell::new(-1),
                #[cfg(feature = "all")]
                drawn: Cell::new(false),
                #[cfg(not(target_arch = "wasm32"))]
                initialized_on: std::thread::current().id(),
                #[cfg(feature = "all")]
                scripts: CusHost::default(),
                #[cfg(feature = "all")]
                unit: Cell::new(-1),
                #[cfg(feature = "all")]
                cus_checked: Cell::new(false),
            })
        }
        fn shutdown(&self, spring: &mut Spring<'_, GameEnvironment>) -> Result<()> {
            #[cfg(feature = "all")]
            assert!(self.cus_checked.get(), "CUS live checks did not finish");
            assert!(spring.messages().echo("[ScopedSdkGame] shutdown", "")?);
            Ok(())
        }
    }
    fn frame(game: &Game, spring: &mut Spring<'_, GameEnvironment>, game_frame: i32) -> Result<()> {
        assert!(
            game_frame > game.last_frame.replace(game_frame),
            "duplicate or out-of-order GameFrame"
        );
        if !game.seen.replace(true) {
            let frame = spring.game().get_game_frame()?;
            assert_eq!(
                ((frame.high16 << 16) | (frame.low16 & 0xffff)) as i32,
                game_frame
            );
            assert!(spring.units().get_all_units()?.is_empty());
            assert!(spring.messages().echo("[ScopedSdkGame] callin", "")?);
            #[cfg(not(target_arch = "wasm32"))]
            {
                assert_ne!(game.initialized_on, std::thread::current().id());
                assert!(
                    spring
                        .messages()
                        .echo("[ScopedSdkGame] loading-thread handoff", "")?
                );
            }
            #[cfg(feature = "all")]
            {
                let mut edited = false;
                assert!(spring.terrain_control().set_height_map_func(|spring| {
                    assert!(
                        spring
                            .terrain_control()
                            .set_height_map(8.0, 8.0, 23.0, 1.0)
                            .unwrap()
                    );
                    edited = true;
                })?);
                assert!(edited);
                assert!((spring.terrain().get_ground_height(8.0, 8.0)? - 23.0).abs() < 0.001);
                assert!(
                    spring
                        .messages()
                        .echo("[ScopedSdkGame] terrain edited", "")?
                );
                let unit = spring.unit_control().create_unit(
                    &spring_sdk::unit_control::DefRef {
                        name: "sdk_probe".into(),
                        id: -1,
                    },
                    Float3::new(320.0, 0.0, 256.0),
                    0,
                    0,
                    Default::default(),
                )?;
                game.unit.set(unit);
                game.scripts.attach(
                    spring,
                    UnitId(unit),
                    ScriptCapabilities::new(ScriptCapabilities::CREATE),
                    |_| ProbeScript::default(),
                )?;
            }
        }
        #[cfg(feature = "all")]
        if game_frame >= 10 && !game.cus_checked.get() {
            let unit = game.unit.get();
            assert_eq!(
                game.scripts
                    .with_state(UnitId(unit), |script| *script.done.borrow()),
                Some(true)
            );
            let position = spring.units_pieces().get_unit_piece_position(unit, 1)?;
            assert!(
                (position.y - 8.0).abs() < 0.001,
                "CUS piece did not move: {position:?}"
            );
            let value = spring
                .unit_script()
                .call_unit_script(unit, "double", &[3.5], 1)?;
            assert!(value.function_found && value.success);
            assert_eq!(value.ret_values, [7.0]);
            assert!(spring.unit_control().destroy_unit(
                unit,
                spring_sdk::unit_control::DestroyUnitOptions {
                    reclaimed: true,
                    ..Default::default()
                }
            )?);
            game.cus_checked.set(true);
            assert!(
                spring
                    .messages()
                    .echo("[ScopedSdkGame] CUS live passed", "")?
            );
        }
        Ok(())
    }
    #[cfg(feature = "all")]
    fn draw(
        game: &Game,
        spring: &mut Spring<'_, GameEnvironment>,
        _width: i32,
        _height: i32,
    ) -> Result<()> {
        spring.gfx().color(1.0, 0.0, 0.0, 1.0)?;
        spring.gfx().rect(0.0, 0.0, 8.0, 8.0)?;
        if !game.drawn.replace(true) {
            assert!(spring.messages().echo("[ScopedSdkGame] draw", "")?);
        }
        Ok(())
    }
    #[cfg(feature = "all")]
    spring_sdk::export_draw_screen!(GAME, draw);
    spring_sdk::export_module!(GAME, GameEnvironment, Game);
    spring_sdk::export_game_frame!(GAME, frame);
    #[cfg(feature = "all")]
    fn scripts(game: &Game) -> &CusHost<ProbeScript, (), GameEnvironment> {
        &game.scripts
    }
    #[cfg(feature = "all")]
    spring_sdk::export_core_cus!(GAME, CusHost<ProbeScript, (), GameEnvironment>, scripts, GameEnvironment);
}
