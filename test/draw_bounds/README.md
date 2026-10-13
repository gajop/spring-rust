# World draw bounds

An optional rectangle hides engine-rendered objects from the other regions of a map.
Coordinates are world elmos; send these through `Spring.SendCommands` or the Rust SDK's
`spring::messages::send_commands`:

```text
drawbounds 1000 1000 3000 2000
drawbounds clear
```

The setting is unsynced, off by default and reset between games. Rectangles must be finite,
ordered, nonempty and inside the map. Invalid commands leave the previous setting intact.
It is independent of `camerabounds` and `minimap view`; send all three on a system switch.

Units, features, projectiles, engine particle emitters, flying pieces and ground flashes
are filtered by their interpolated draw position (their origin, not their mesh extent).
Simple particle systems also filter individual particles by their interpolated position.
An accepted model or effect can extend over the boundary; this command does not cut geometry.
World icons, ghost buildings and temporary build models follow the rectangle too. Filtering
covers world, reflection/refraction and shadow passes, for both GLSL and GL4 model drawers.
Terrain and game-authored Lua drawing retain their own behavior. The minimap keeps its
independent view rectangle. Simulation, LOS and object lifetimes continue normally.

After rebuilding the normal engine, run the isolated graphical regression on Linux:

```bash
xvfb-run -a python3 test/draw_bounds/run.py --drawer 1
xvfb-run -a python3 test/draw_bounds/run.py --drawer 0 --output test/native_api_parity/out/draw-bounds-glsl
```

Use `--spring /path/to/spring` to select another executable and `--output /path/to/logs` to
retain its logs. The fixture places two groups in a tilted camera's view, with a unit,
feature, model projectile, particle system and ground flash in each. Pixel checks verify
that each group disappears and returns when switching or clearing the rectangle, while
objects in the active group remain visible. Draw-flag checks include unit/feature shadows.
It also checks invalid input preserves the setting, object counts stay unchanged and
projectiles continue updating while hidden.

For a manual Ship Game check, use branch `wormholes` and
`just play wormhole-sandbox --modoption sb_wormhole_at=3`. Send `drawbounds` with the current
system's rectangle, press N to switch, then tilt the camera toward the other system from
the near edge. Its ships, rocks, shots, engine effects and shadows should disappear. Switch
rectangles and check that the newly selected system is visible. Clear the command to see
both systems again. Repeat with cloaked units, radar icons, destroyed buildings left in fog
and explosions; verify that hiding them does not interrupt movement or combat. Start a new
game and verify that rendering defaults to the whole map.
