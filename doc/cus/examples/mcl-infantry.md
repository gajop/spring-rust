# MechCommander: Legacy `Infantry` Sketch

Source reference:
<https://github.com/SpringMCLegacy/SpringMCLegacy/blob/master/scripts/Infantry.lua>

MCL is useful because the script is not merely animation code. It owns gameplay
state such as ammo and weapon configuration and exposes custom functions to
other game systems.

## Typed state

A Rust version can replace Lua tables with normal Rust structures:

```rust
#[cus]
pub struct Infantry {
    torso: Piece,
    left_arm: Option<Piece>,
    right_arm: Option<Piece>,

    launch_points: Vec<Vec<Piece>>,
    current_launch_point: Vec<usize>,
    ammo: AmmoStore,

    moving: bool,
    torso_speed: AngularSpeed,
    elevation_speed: AngularSpeed,
    restore_delay: Duration,
}
```

## Custom game entry point

The original script exposes `ChangeAmmo` for other game code:

```rust
#[cus(export)]
fn change_ammo(
    &mut self,
    ctx: &mut UnitCtx,
    ammo_type: AmmoType,
    amount: i32,
) -> bool {
    let Some(current) = self.ammo.get_mut(ammo_type) else {
        return false;
    };

    let max = crate::mcl::max_ammo(ctx.unit_def(), ammo_type);
    let next = *current + amount;

    if next > max {
        return false;
    }

    *current = next;
    ctx.set_unit_rule(
        crate::mcl::ammo_rule(ammo_type),
        100.0 * next as f32 / max as f32,
    );
    true
}
```

Other Rust game systems in the same synced module should normally invoke this
through a typed CUS handle rather than a string-based Lua environment lookup.

## Delayed restore

```rust
async fn restore_after_delay(&mut self, ctx: &mut UnitCtx) {
    ctx.sleep(self.restore_delay).await;

    ctx.turn(self.torso, Axis::Y, Angle::ZERO, self.torso_speed);

    if let Some(left) = self.left_arm {
        ctx.turn(left, Axis::X, Angle::ZERO, self.elevation_speed);
    }
    if let Some(right) = self.right_arm {
        ctx.turn(right, Axis::X, Angle::ZERO, self.elevation_speed);
    }
}
```

MCL demonstrates why Rust CUS needs arbitrary game APIs and typed persistent
unit state, not only COB-compatible animation operations.
