# Authoring UX

The primary UX should be ordinary typed Rust with `async` only where the script
can suspend.

The exact attribute and trait spelling is intentionally not fixed here. A
representative shape is:

```rust
#[cus]
impl UnitScript for Atlas {
    fn query_weapon(
        &mut self,
        _ctx: &mut UnitCtx,
        weapon: WeaponId,
    ) -> Piece {
        self.flares[weapon.index()]
    }

    async fn aim_weapon(
        &mut self,
        ctx: &mut UnitCtx,
        weapon: WeaponId,
        heading: Angle,
        pitch: Angle,
    ) -> bool {
        ctx.turn(self.torso, Axis::Y, heading, self.torso_speed);
        ctx.turn(self.gun, Axis::X, -pitch, self.elevation_speed);

        ctx.wait_for_turn(self.torso, Axis::Y).await;
        ctx.wait_for_turn(self.gun, Axis::X).await;

        true
    }
}
```

## Immediate operations versus suspension

Animation commands remain immediate, as in COB/LUS:

```rust
ctx.turn(piece, Axis::X, target, speed);
ctx.move_piece(piece, Axis::Y, target, speed);
ctx.spin(piece, Axis::Z, speed, accel);
ctx.show(piece);
ctx.hide(piece);
```

Only operations that actually suspend use `.await`:

```rust
ctx.sleep(500.ms()).await;
ctx.wait_for_turn(piece, Axis::X).await;
ctx.wait_for_move(piece, Axis::Y).await;
ctx.next_frame().await;
```

This preserves an important unit-script pattern: start several animations, then
wait for selected ones.

## Standard versus custom entry points

Standard engine CUS methods should be validated by Rust rather than discovered
from magic string names where practical.

Games also need custom entry points. A provisional form is:

```rust
impl Atlas {
    #[cus(export)]
    fn change_ammo(
        &mut self,
        ctx: &mut UnitCtx,
        ammo: AmmoType,
        amount: i32,
    ) -> bool {
        // game code
    }
}
```

This is the typed replacement for LUS patterns that expose arbitrary functions
through a script environment.

## Internal helpers

Helpers are normal Rust:

```rust
impl Atlas {
    async fn restore_after_delay(&mut self, ctx: &mut UnitCtx) {
        ctx.sleep(self.restore_delay).await;
        ctx.turn(self.torso, Axis::Y, Angle::ZERO, self.torso_speed);
    }
}
```

The exact syntax for spawning these tasks is left to implementation. The API
should keep task ownership inside CUS rather than make arbitrary executor
behavior part of the public contract.

## What is deliberately not fixed yet

- exact `UnitScript` trait shape;
- whether each standard method may be sync, async, or both;
- exact task/spawn handle API;
- exact signal type and cancellation API;
- naming of context types.

Those details can be iterated during implementation without changing the core
model.
