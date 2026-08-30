# Minimal Rust CUS Example

The exact API names are provisional. This example shows the intended authoring
shape rather than a frozen SDK.

```rust
use spring::cus::prelude::*;

#[cus]
pub struct Tank {
    body: Piece,
    turret: Piece,
    flare: Piece,
}

#[cus]
impl UnitScript for Tank {
    fn new(ctx: &mut InitCtx) -> Self {
        Self {
            body: ctx.piece("body"),
            turret: ctx.piece("turret"),
            flare: ctx.piece("flare"),
        }
    }

    fn query_weapon(
        &mut self,
        _ctx: &mut UnitCtx,
        _weapon: WeaponId,
    ) -> Piece {
        self.flare
    }

    async fn aim_weapon(
        &mut self,
        ctx: &mut UnitCtx,
        _weapon: WeaponId,
        heading: Angle,
        _pitch: Angle,
    ) -> bool {
        ctx.turn(self.turret, Axis::Y, heading, 180.deg_per_sec());
        ctx.wait_for_turn(self.turret, Axis::Y).await;
        true
    }

    fn killed(
        &mut self,
        ctx: &mut UnitCtx,
        recent_damage: f32,
        max_health: f32,
    ) -> WreckLevel {
        if recent_damage / max_health > 0.5 {
            ctx.explode(self.body, SfxFlags::SHATTER);
            WreckLevel::Two
        } else {
            WreckLevel::One
        }
    }
}
```

Important properties:

- state is a normal Rust struct;
- synchronous engine queries stay synchronous;
- only waits/sleeps require async;
- animation interpolation remains engine-side;
- the same source is intended to compile for WasmCUS and NativeCUS.
