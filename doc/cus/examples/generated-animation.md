# Generated Animation Workflow

Large Spring games often generate or mechanically author substantial animation
content. Rust CUS should make that a first-class workflow rather than require an
exporter to emit enormous hand-written async functions.

## Suggested representation

An exporter can generate typed data:

```rust
pub static WALK: Animation = Animation {
    frames: &[
        Frame {
            time: 0,
            commands: &[
                turn(HumanPiece::LeftThigh, Axis::X, 20.deg(), SPEED),
                move_piece(HumanPiece::Torso, Axis::Y, 4.0, 9.0),
            ],
        },
        // ...
    ],
};
```

The reusable CUS player stays small:

```rust
async fn play_animation(
    &mut self,
    ctx: &mut UnitCtx,
    animation: AnimationId,
) {
    let mut frame_index: usize = 0;

    while frame_index < animation.len() {
        animation.apply_frame(frame_index, self, ctx);

        if let Some(delay) = animation.delay_after(frame_index) {
            ctx.sleep_frames(delay).await;
        }

        frame_index += 1;
    }
}
```

## Why IDs and indices matter

This representation also leaves a cleaner future save/load path.

Durable task state can contain:

```text
animation: AnimationId
frame_index: usize
```

rather than serializing an iterator, a borrow into static animation data, or an
opaque compiler-generated future layout.

## Existing-game motivation

- BAR has a very large BOS/COB animation corpus and active compiler/exporter
  tooling.
- SpringCabal Area-17's `Scripts/human.lua` loads generated keyframe data and
  implements a generic `PlayAnimation` loop.
- MCL combines generic unit scripts with per-unit animation modules.

Rust CUS should support these workflows directly even if the exact animation
data format is game-owned.
