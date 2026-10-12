#!/usr/bin/env python3
# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
"""Measure 1000 sim frames without diagnostic hashing, using an immutable game.

Run one engine at a time. --interval opts into a game-owned full-memory cadence;
omitting it measures the engine default. Each run also records a demo that can
be replayed WITH WasmSyncLog to check that logging leaves network checks intact.
"""
import argparse
import importlib.util
import json
from pathlib import Path
import re
import socket
import subprocess

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location('sync_runner', ROOT / 'test/synctest/wasm_sync_replay.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)


def measure(args):
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    isolation = output / 'isolation'
    write = output / 'write'
    (isolation / 'games').mkdir(parents=True)
    write.mkdir()
    fixture = output / 'markers.wasm'
    subprocess.run(['wat2wasm', str(ROOT / 'test/wasm_api/data/synced_jobs/state_hash_cost.wat'),
                    '-o', str(fixture)], check=True)
    game = isolation / 'games/game.sdd'
    runner.mount_game(args.game.resolve(), game, fixture, None)
    if args.interval is not None:
        path = game / 'gamedata/modrules.lua'
        text = path.read_text()
        # Evaluate the unchanged modrules chunk, then override one test setting.
        path.unlink()
        path.write_text('local rules = (function()\n' + text + '\nend)()\n' +
                        'rules.system = rules.system or {}\n' +
                        f'rules.system.WasmStateHashInterval = {args.interval}\nreturn rules\n')
    spring = args.spring.resolve()
    for name in ('base', 'fonts', 'AI'):
        source = spring.parent / name
        if source.exists():
            (isolation / name).symlink_to(source, target_is_directory=True)
    script = args.script.read_text()
    with socket.socket() as probe:
        probe.bind(('127.0.0.1', 0))
        port = probe.getsockname()[1]
    script = re.sub(r'(?im)^\s*hostport=.*;', f'HostPort={port};', script)
    start = output / 'start.txt'
    start.write_text(script)
    config = output / 'spring.conf'
    config.write_text('LogSections = *\nWasmSyncLog = \nWasmSyncDumpFrame = -1\n' +
                      'WasmSyncStopFrame = 1000\nUseLuaMemPools = 0\nTextureMemPoolSize = 0\n' +
                      'ROAM = 0\nGrassDetail = 0\nsnd_volmaster = 0\n' +
                      'MaxTextureAtlasSizeX = 16384\nMaxTextureAtlasSizeY = 16384\n' +
                      f'WorkerThreadCount = {args.threads}\nPathingThreadCount = {args.threads}\n' +
                      f'HardwareThreadCount = {args.threads}\n')
    command = [str(spring), '--write-dir', str(write), '--isolation',
               '--isolation-dir', str(isolation), '--config', str(config), str(start)]
    with (output / 'console.log').open('w') as console:
        subprocess.run(command, stdout=console, stderr=subprocess.STDOUT,
                       timeout=args.timeout, check=True)
    text = (write / 'infolog.txt').read_text()
    times = {}
    for match in re.finditer(r'\[t=(\d+):(\d+):(\d+\.\d+)\]\[f=(\d+)\] WASM_HASH_COST_(START|END)', text):
        hours, minutes, seconds, frame, label = match.groups()
        times[label] = (int(hours) * 3600 + int(minutes) * 60 + float(seconds), int(frame))
    if set(times) != {'START', 'END'} or times['START'][1] != 0 or times['END'][1] != 1000:
        raise RuntimeError('Missing timestamp markers at frames 0 and 1000')
    demos = list((write / 'demos').glob('*.sdfz'))
    if len(demos) != 1:
        raise RuntimeError('Expected exactly one recording')
    result = dict(spring=str(spring), engine_sha256=runner.sha256(spring),
                  game=str(args.game.resolve()), game_overlay=str(game),
                  interval=args.interval, threads=args.threads, frames=1000,
                  diagnostic_logging=False, duration_seconds=times['END'][0] - times['START'][0],
                  demo=str(demos[0]), command=command)
    result['ms_per_frame'] = result['duration_seconds'] * 1000 / result['frames']
    (output / 'result.json').write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps(result, indent=2), flush=True)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--spring', required=True, type=Path)
    parser.add_argument('--game', required=True, type=Path)
    parser.add_argument('--script', required=True, type=Path)
    parser.add_argument('--output', required=True, type=Path)
    parser.add_argument('--interval', type=int)
    parser.add_argument('--threads', type=int, default=4)
    parser.add_argument('--timeout', type=float, default=300)
    measure(parser.parse_args())
