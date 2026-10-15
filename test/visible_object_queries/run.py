#!/usr/bin/env python3
# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
"""Small rendered regression; compares IDs, without fleet timing loops."""
from __future__ import annotations

import argparse
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'test/native_api_parity'))
from run_harness import BASE_CONTENT, ENGINE_INSTALL, SOURCE_CONTENT, GAME_FIXTURE, blank_map_name, prepare_datadir, write_script


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--spring', type=Path, default=ENGINE_INSTALL / 'spring')
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--map-size', type=int, default=16)
    parser.add_argument('--terrain', choices=['blank', 'null'], default='null')
    parser.add_argument('--storage', choices=['auto', 'dense', 'sparse'], default='auto')
    options = parser.parse_args()
    output = options.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    args = argparse.Namespace(blank_map_x=options.map_size, blank_map_y=options.map_size, blank_map_height=0,
        blank_map_terrain=options.terrain, blank_map_seed=424242, cases=1, tests=None, test_prefix=None,
        enable_rendering_tests=True, process_test=None, timeout=30,
        spring=options.spring.resolve(), spring_headless=options.spring.resolve())
    with tempfile.TemporaryDirectory(prefix='visible-object-queries-') as directory:
        datadir, _ = prepare_datadir(Path(directory), None)
        game = datadir / 'games/native_api_parity.sdd'
        if game.is_symlink(): game.unlink()
        else: shutil.rmtree(game)
        shutil.copytree(GAME_FIXTURE, game)
        for path in (game / 'LuaRules/Gadgets').glob('*.lua'): path.unlink()
        shutil.copy2(Path(__file__).with_name('setup.lua'), game / 'LuaRules/Gadgets/visible_queries.lua')
        shutil.copy2(Path(__file__).with_name('checks.lua'), game / 'LuaUI/main.lua')
        # The baseline fixture intentionally omits explosion definitions. Supply
        # its default name here so unrelated content warnings do not mask errors.
        (game / 'weapons').mkdir(exist_ok=True)
        (game / 'weapons/no_weapon.lua').write_text("return {noweapon={name='No explosion',weaponType='Cannon',range=1,reloadtime=1,weaponVelocity=1,damage={default=0}}}\n")
        # Huge null maps need no fine sensor grid for this candidate-query check.
        (game / 'gamedata/modrules.lua').write_text('return {system={pathFinderSystem=0},sensors={los={losMipLevel=6,airMipLevel=6},radarMipLevel=6}}\n')
        script = output / 'script.txt'
        write_script(script, blank_map_name(args, 'lua'), 'native_api_parity', 'lua', True, args, 424242, 'ui', None)
        settings = output / 'write/springsettings.cfg'
        settings.parent.mkdir(parents=True, exist_ok=True)
        storage = {'auto':0, 'dense':1, 'sparse':2}[options.storage]
        settings.write_text('Fullscreen=0\nXResolutionWindowed=640\nYResolutionWindowed=480\nVSync=1\n'
            'Shadows=0\nSound=0\nWorkerThreadCount=1\nWindowedEdgeMove=0\nFullscreenEdgeMove=0\n'
            'UseFontConfigLib=0\nUseLuaMemPools=0\nTextureMemPoolSize=0\n'
            f'SparseMapQuadField={storage}\n')
        env = os.environ.copy()
        env['SPRING_DATADIR'] = os.pathsep.join(map(str, [datadir, BASE_CONTENT, ENGINE_INSTALL, SOURCE_CONTENT]))
        env['SPRING_ISOLATED'] = str(datadir)
        env['SPRING_NATIVE_MODULE'] = ''
        with (output / 'spring.log').open('wb') as log:
            process = subprocess.Popen([str(args.spring), '--window', '--nocolor', '--config', str(settings),
                '--write-dir', str(settings.parent), str(script)], cwd=ROOT, env=env, stdout=log, stderr=subprocess.STDOUT)
            try:
                code = process.wait(timeout=60)
            except (subprocess.TimeoutExpired, KeyboardInterrupt):
                process.terminate()
                try: process.wait(timeout=5)
                except subprocess.TimeoutExpired: process.kill();process.wait()
                raise
    log = (output / 'spring.log').read_text(errors='replace')
    if code or 'VISIBLE_OBJECT_QUERIES_PASS' not in log or 'VISIBLE_OBJECT_QUERIES_FAIL' in log:
        raise SystemExit(f'Visibility query regression failed (exit {code}): {output / "spring.log"}')
    if 'Could not finalize groundFX' in log or 'bad allocation for atlas "GroundFXAtlas"' in log:
        raise SystemExit(f'Empty groundFX atlas regression: {output / "spring.log"}')
    print(f'PASS {options.terrain} {options.map_size}, {options.storage}: unit/feature visibility and screen rectangles, all query modes')


if __name__ == '__main__': main()
