# Zero-K `amphraid` Sketch

Source reference:
<https://github.com/ZeroK-RTS/Zero-K/blob/master/scripts/amphraid.lua>

This is intentionally partial. It shows how a real LUS pattern maps to the
proposed UX.

## Walking

LUS starts several animation commands, waits for torso movement, then explicitly
yields with `Sleep(0)`.

Rust CUS can preserve that shape:

```rust
async fn walk(&mut self, ctx: &mut UnitCtx) {
    ctx.signal(SIG_WALK);
    ctx.set_signal_mask(SIG_WALK);

    loop {
        ctx.turn(self.lthigh, Axis::X, 20.deg(), 252.deg_per_sec());
        ctx.turn(self.lshin, Axis::X, -60.deg(), 294.deg_per_sec());
        ctx.turn(self.rthigh, Axis::X, -20.deg(), 441.deg_per_sec());
        ctx.move_piece(self.torso, Axis::Y, 4.0, 18.9);

        ctx.wait_for_move(self.torso, Axis::Y).await;
        ctx.next_frame().await;

        // next key pose ...
    }
}
```

`ctx.next_frame()` above is the direct Rust expression of LUS `Sleep(0)`, whose
framework semantics are a minimum one-frame sleep.

## Aiming

```rust
async fn aim_weapon(
    &mut self,
    ctx: &mut UnitCtx,
    weapon: WeaponId,
    heading: Angle,
    pitch: Angle,
) -> bool {
    let signal = self.aim_signal(weapon);
    ctx.signal(signal);
    ctx.set_signal_mask(signal);

    ctx.turn(self.head, Axis::Y, heading, 380.deg_per_sec());
    ctx.turn(self.lturret, Axis::X, -pitch, 160.deg_per_sec());
    ctx.turn(self.rturret, Axis::X, -pitch, 160.deg_per_sec());

    ctx.wait_for_turn(self.head, Axis::Y).await;
    ctx.wait_for_turn(self.lturret, Axis::X).await;
    ctx.wait_for_turn(self.rturret, Axis::X).await;

    ctx.spawn(Self::restore_after_delay);
    true
}
```

CUS `spawn` should preserve LUS `StartThread` ordering: the child starts
immediately and runs until its first suspension or completion before this task
continues. It also inherits this task's current signal mask.

## Game-specific API

The original `BlockShot` calls Zero-K's overkill-prevention logic. Rust CUS
must not force that through a generic Spring CUS API:

```rust
fn block_shot(
    &mut self,
    ctx: &mut UnitCtx,
    weapon: WeaponId,
    target: UnitId,
    user_target: bool,
) -> bool {
    crate::overkill::check_block(
        ctx.unit(),
        target,
        weapon,
        user_target,
        130.0,
    )
}
```

Because the script and Zero-K synced game logic are in the same Rust module,
this can be an ordinary typed Rust call.
