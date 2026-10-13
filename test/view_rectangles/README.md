# View rectangles

Both settings are optional, unsynced state for the current game. Coordinates are world elmos:

```text
camerabounds 1000 1000 3000 2000
minimap view 1000 1000 3000 2000
camerabounds clear
minimap view clear
```

Send these with `Spring.SendCommands` or the Rust SDK's `spring::messages::send_commands`.
Set camera bounds before setting the new camera target on a region switch. Bounds changes
cancel the old transition so the view does not interpolate through a different region.
The rectangles are independent; set both when the camera and minimap should show one system.
Invalid, non-finite, reversed, empty or out-of-map rectangles leave the previous setting intact.

After rebuilding the normal engine, run the isolated graphical regression on Linux:

```bash
xvfb-run -a python3 test/view_rectangles/run.py
xvfb-run -a python3 test/view_rectangles/run.py --direct
```

Use `--spring /path/to/spring` to select another executable and `--output /path/to/logs` to
retain its logs. The fixture checks all seven camera modes, explicit targets/state, zoom
limits and keyboard zoom resets, disabled controller clamp settings, invalid input, region
switches and clearing. Pixel checks cover background cropping, icons at the region boundary
and oversized `DrawInMiniMap` drawing. Ray checks cover all four minimap rotations.

For a manual game check, choose a rectangle inside the current map, send both commands, then
pan with keys, screen edges and dragging; click the minimap; zoom all the way out and reset;
switch camera modes, including overview. The focus should stay inside the rectangle. The
minimap should show only that region, and clicks should land where its icons are drawn. Switch
to a second rectangle while a camera transition is running, then clear both settings and check
whole-map behavior. Neither setting should carry into a new game.

World-space minimap uniform matrices include the region offset and dimensions. Lua/SDK
`DrawInMiniMap` continues to use minimap pixels, so the game can draw its own system chart
using the reported minimap size. The engine clips that drawing at the minimap's edges.
