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

`CUnitScript::AimWeapon` itself returns `void`. The Rust `bool` above is an
authoring/runtime convention analogous to LUS: completion of the async method
publishes the weapon's aim-ready state. It is not synchronously returned through
the C++ virtual.

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

CUS does not need to ban other ordinary Rust futures. The CUS-provided waits are
special because they have defined engine scheduling semantics and are the
suspension points a future durable CUS backend is expected to understand.

## Standard versus custom entry points

Standard engine CUS methods should be validated by Rust rather than discovered
from magic string names where practical.

Some standard methods require an immediate result from the engine and therefore
cannot suspend. The exact set follows from `CUnitScript`'s synchronous return or
in/out contract and is documented in [Engine integration](engine-integration.md).

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

## Internal helpers and spawned tasks

Helpers are normal Rust:

```rust
impl Atlas {
    async fn restore_after_delay(&mut self, ctx: &mut UnitCtx, delay: Duration) {
        ctx.sleep(delay).await;
        ctx.turn(self.torso, Axis::Y, Angle::ZERO, self.torso_speed);
    }
}
```

The public spawn model should refer to a **known/named CUS task**, for example
conceptually:

```rust
ctx.spawn(Self::restore_after_delay, 5.seconds());
```

The exact syntax and task-handle type remain open, but callers should not pass an
arbitrary opaque `Future` as the spawned task. This keeps task identity,
signals/debugging, and a later generated durable representation under CUS
control.

This does **not** require V1 to store tasks inline. The implementation may box a
rustc-generated future initially and later replace that storage with generated
or inline task state without changing the game-facing spawn API.

## What is deliberately not fixed yet

- exact `UnitScript` trait shape;
- exact named-task/spawn syntax and handle type;
- exact signal type and cancellation API;
- naming of context types.

Those details can be iterated during implementation without changing the core
model.
