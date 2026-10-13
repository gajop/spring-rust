#!/usr/bin/env python3
# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
"""Run world draw bounds checks in an isolated graphics game."""
from __future__ import annotations

import argparse
from pathlib import Path
import os
import shutil
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "test/native_api_parity"))
from run_harness import BASE_CONTENT, ENGINE_INSTALL, SOURCE_CONTENT, GAME_FIXTURE, blank_map_name, prepare_datadir, write_script


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--spring", type=Path, default=ROOT / "build-amd64-linux/install/spring")
    parser.add_argument("--output", type=Path, default=ROOT / "test/native_api_parity/out/draw-bounds")
    parser.add_argument("--drawer", type=int, choices=(0, 1), default=1, help="0: legacy GLSL; 1: instanced GL4")
    options = parser.parse_args()
    output = options.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    args = argparse.Namespace(blank_map_x=10, blank_map_y=8, blank_map_height=96,
        blank_map_seed=424242, cases=1, tests=None, test_prefix=None,
        enable_rendering_tests=True, process_test=None, timeout=60,
        spring=options.spring.resolve(), spring_headless=options.spring.resolve())
    with tempfile.TemporaryDirectory(prefix="draw-bounds-") as directory:
        datadir, _ = prepare_datadir(Path(directory), None)
        game = datadir / "games/native_api_parity.sdd"
        if game.is_symlink():
            game.unlink()
        else:
            shutil.rmtree(game)
        shutil.copytree(GAME_FIXTURE, game)
        gadgets = game / "LuaRules/Gadgets"
        for path in gadgets.glob("*.lua"):
            path.unlink()
        shutil.copy2(Path(__file__).with_name("setup.lua"), gadgets / "draw_bounds.lua")
        shutil.copy2(Path(__file__).with_name("checks.lua"), game / "LuaUI/main.lua")
        for name in ("explosions.lua", "resources.lua"):
            shutil.copy2(Path(__file__).with_name(name), game / "gamedata" / name)
        unit_defs = game / "units/native_api_test_unit.lua"
        unit_defs.write_text(unit_defs.read_text().replace("sightDistance = 256,", "sightDistance = 5000,\n\t\tairSightDistance = 5000,"))
        (game / "weapons").mkdir(exist_ok=True)
        shutil.copy2(Path(__file__).with_name("projectile.lua"), game / "weapons/draw_bounds.lua")
        with (game / "LuaUI/main.lua").open("a") as file:
            file.write(f"\nlocal DrawBoundsDrawer = {options.drawer}\nSpring.SendCommands('unitdrawer ' .. DrawBoundsDrawer, 'featuredrawer ' .. DrawBoundsDrawer)\n")
        script = output / "script.txt"
        write_script(script, blank_map_name(args, "lua"), "native_api_parity",
                     "lua", True, args, 424242, "ui", None)
        settings = output / "write-dir/springsettings.cfg"
        settings.parent.mkdir(parents=True, exist_ok=True)
        settings.write_text("Fullscreen=0\nWindowBorderless=0\nXResolutionWindowed=1280\n"
                            "YResolutionWindowed=720\nShadows=1\nShadowMapSize=1024\n"
                            "UnitIconsAsUI=0\nUnitIconDist=100000\nFeatureDrawDistance=10000\n"
                            "FeatureFadeDistance=10000\nSoftParticles=0\n")
        env = os.environ.copy()
        env["SPRING_DATADIR"] = os.pathsep.join(map(str, [datadir, BASE_CONTENT, ENGINE_INSTALL, SOURCE_CONTENT]))
        env["SPRING_ISOLATED"] = str(datadir)
        env["SPRING_NATIVE_MODULE"] = ""
        with (output / "spring.log").open("wb") as log:
            code = subprocess.run([str(args.spring), "--window", "--nocolor", "--config", str(settings),
                                   "--write-dir", str(settings.parent), str(script)],
                                  cwd=ROOT, env=env, stdout=log, stderr=subprocess.STDOUT, timeout=90).returncode
    log = (output / "spring.log").read_text(encoding="utf-8", errors="replace")
    if code != 0 or "DRAW_BOUNDS_PASS" not in log or "DRAW_BOUNDS_FAIL" in log:
        raise SystemExit(f"draw bounds regression failed (exit {code}); see {output / 'spring.log'}")
    print(f"PASS: draw bounds, drawer {options.drawer}; world objects/effects, shadow flags, region switches and clear")


if __name__ == "__main__":
    main()
