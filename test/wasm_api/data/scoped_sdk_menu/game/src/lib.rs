/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */
#[cfg(target_arch = "wasm32")]
mod game {
    use spring_sdk::{Float3, Result, RulesSynced, Spring, module::Module, types::callins::*};
    use std::cell::Cell;
    struct Game {
        seen: Cell<bool>,
    }
    impl Module<RulesSynced> for Game {
        fn initialize(spring: &mut Spring<'_, RulesSynced>) -> Result<Self> {
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
            })
        }
        fn shutdown(&self, spring: &mut Spring<'_, RulesSynced>) -> Result<()> {
            assert!(spring.messages().echo("[ScopedSdkGame] shutdown", "")?);
            Ok(())
        }
    }
    fn frame(
        game: &Game,
        spring: &mut Spring<'_, RulesSynced>,
        query: &GameFrameQuery,
    ) -> Result<GameFrameResult> {
        if !game.seen.replace(true) {
            let frame = spring.game().get_game_frame()?;
            assert_eq!(
                ((frame.high16 << 16) | (frame.low16 & 0xffff)) as i32,
                query.game_frame
            );
            assert!(spring.units_query().get_all_units()?.is_empty());
            assert!(spring.messages().echo("[ScopedSdkGame] callin", "")?);
        }
        Ok(GameFrameResult::default())
    }
    spring_sdk::export_module!(GAME, RulesSynced, Game);
    spring_sdk::export_game_frame!(GAME, frame);
}
