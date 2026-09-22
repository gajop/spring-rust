# Ship Game SDK compatibility blocker

Recorded: 2026-09-22T10:35:14+09:00 (Asia/Tokyo)

The Ship Game workspace currently uses the local `spring-bar` Rust SDK through `SHIPGAME_SDK`.
Its committed fallback is `gajop/spring-rust` branch `rust-wip` at
`73080761b39cd520038bba3f74e9ddae9f1aaa98`.

That public revision is older than the engine APIs used by the game. With `SHIPGAME_SDK` empty,
`cargo clippy --workspace --release` in `ship-game.sdd` fails before it can check the game because
the pinned SDK is missing or has incompatible versions of:

- `spring::object_rendering` and its material descriptors;
- the `UnsyncedAddon` draw, unit-event, projectile-event, and synced-message callins;
- `move_ctrl::set_velocity`;
- the `spring::gfx` matrix constants;
- `gfx::create_query` (the pinned API takes `u8`, while the game uses `Option<u32>`);
- `gfx::uniform_array_float` (the pinned API takes two arguments, while the game uses three); and
- the matrix-data return shape (the pinned API returns `Vec<f32>`, while the game expects a
  sixteen-element array).

The local `rust-wip` checkout currently contains the required later work through
`d269a7826742d25e77df4b3b324959a704829485` (`Fix Lua and typed CEG name dispatch`), including
`c323d01bce` (`Complete WASM and native API parity work`) and `872b0df821` (`Enable RmlUi in
native and Core-WASM menus`).

Full addon CI can be enabled once those SDK commits are published to the public `rust-wip`
revision used by the game, or after the game is deliberately migrated to the older ABI.
