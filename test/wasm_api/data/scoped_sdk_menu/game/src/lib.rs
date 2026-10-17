/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */
mod game {
    #[cfg(feature = "all")]
    use spring_sdk::All as GameEnvironment;
    #[cfg(not(feature = "all"))]
    use spring_sdk::RulesSynced as GameEnvironment;
    use spring_sdk::{Float3, Result, Spring, module::Module};
    use std::cell::Cell;
    struct Game {
        seen: Cell<bool>,
        last_frame: Cell<i32>,
        #[cfg(feature = "all")]
        drawn: Cell<bool>,
        #[cfg(not(target_arch = "wasm32"))]
        initialized_on: std::thread::ThreadId,
    }
    impl Module<GameEnvironment> for Game {
        fn initialize(spring: &mut Spring<'_, GameEnvironment>) -> Result<Self> {
            if cfg!(feature = "fail-init") {
                return Err(spring_sdk::ApiError::new(
                    spring_sdk::ErrorCode::InvalidArgument as i32,
                ));
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
            })
        }
        fn shutdown(&self, spring: &mut Spring<'_, GameEnvironment>) -> Result<()> {
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
            }
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
}
