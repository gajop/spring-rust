#!/usr/bin/env python3
# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
"""Prepare a read-only game snapshot with all seven job integration guests."""
import argparse
import importlib.util
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location('sync_runner', ROOT / 'test/synctest/wasm_sync_replay.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)


def prepare(game: Path, output: Path, pause: bool, loading_intro: bool = False):
    runner.mount_game(game, output, None)
    fixtures = ROOT / 'test/wasm_api/data/synced_jobs'
    template = (fixtures / 'all_environments.wat.in').read_text()
    manifest = output / 'LuaRules/wasm/manifest.txt'
    text = manifest.read_text()
    manifest.unlink()
    for number, environment in enumerate(('rules-synced', 'rules-unsynced', 'gaia-synced',
                                           'gaia-unsynced', 'ui', 'menu', 'intro')):
        name = 'jobs-env-' + environment
        other = 'jobs-env-ui' if environment in ('rules-synced', 'gaia-synced') else 'jobs-env-rules-synced'
        if environment in ('rules-synced', 'gaia-synced'):
            clock = '(func (export "spring:callin/game-frame") (param i32) call $bootstrap)'
        elif environment == 'intro':
            clock = '(func (export "spring:callin/draw-genesis") (param i32) call $bootstrap)\n' + \
                    '(func (export "spring:callin/draw-load-screen") (param i32) call $bootstrap)'
        else:
            clock = '(func (export "spring:callin/update") (param f32) call $bootstrap)'
        substitutions = {'ENV_MASK': str(1 << number), 'NAME': name, 'OTHER': other,
                         'NAME_LEN': str(len(name)), 'OTHER_LEN': str(len(other)),
                         'BOOT_LEN': str(len('JOBS_ENV_' + name + '_BOOT')),
                         'R100_LEN': str(len('JOBS_ENV_' + name + '_RESULTS_100')),
                         'R400_LEN': str(len('JOBS_ENV_' + name + '_RESULTS_400')),
                         'R10_LEN': str(len('JOBS_ENV_' + name + '_RESULTS_10')),
                         'CLOCK_EXPORTS': clock}
        wat = template
        for key, value in substitutions.items():
            wat = wat.replace('@' + key + '@', value)
        path = output / 'LuaRules/wasm' / (name + '.wat')
        path.write_text(wat)
        binary = path.with_suffix('.wasm')
        if binary.exists():
            binary.unlink()
        subprocess.run(['wat2wasm', str(path), '-o', str(binary)], check=True)
        text += f'\nmodule({name}, LuaRules/wasm/{name}.wasm, {environment}, {900 + number}, 1.0.0)\n'
    manifest.write_text(text)
    if loading_intro:
        # Separate NativeInterface system, pumped by the real loading screen.
        # Keep the name the same byte length as the template substitutions.
        directory = output / 'WasmIntro'
        directory.mkdir(exist_ok=True)
        wat = (output / 'LuaRules/wasm/jobs-env-intro.wat').read_text()
        wat = wat.replace('jobs-env-intro', 'jobs-pre-intro')
        path = directory / 'jobs.wat'
        path.write_text(wat)
        subprocess.run(['wat2wasm', str(path), '-o', str(path.with_suffix('.wasm'))], check=True)
        target = directory / 'manifest.json'
        if target.exists():
            target.unlink()
        target.write_text('module(jobs-pre-intro, WasmIntro/jobs.wasm, intro, 900, 1.0.0)\n')
    if pause:
        target = output / 'LuaRules/wasm/pause_observer.lua'
        target.parent.mkdir(parents=True, exist_ok=True)
        if target.exists():
            target.unlink()
        target.write_bytes((fixtures / 'pause_observer.lua').read_bytes())
        draw = output / 'LuaRules/draw.lua'
        original = draw.read_text() if draw.exists() else ''
        if draw.exists():
            draw.unlink()
        draw.write_text(original + '\nVFS.Include(\"LuaRules/wasm/pause_observer.lua\")\n')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--game', required=True, type=Path)
    parser.add_argument('--output', required=True, type=Path)
    parser.add_argument('--pause', action='store_true')
    parser.add_argument('--loading-intro', action='store_true')
    args = parser.parse_args()
    prepare(args.game.resolve(), args.output.resolve(), args.pause, args.loading_intro)
