#!/usr/bin/env python3
# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
"""Compare complete per-frame Wasm logs, or replay a pinned game on a local engine."""
from __future__ import annotations

import argparse
import gzip
import hashlib
import itertools
import json
import os
import re
import socket
import struct
from pathlib import Path
import shutil
import subprocess
import sys


class VerificationError(RuntimeError):
    pass


def load_rows(path: Path):
    previous = -1
    with path.open() as stream:
        for number, line in enumerate(stream, 1):
            try:
                row = json.loads(line)
                frame = row['frame']
                if type(frame) is not int or frame != previous + 1:
                    raise ValueError(f'expected contiguous frame {previous + 1}, got {frame}')
                if row['synccheck'] is not True:
                    raise ValueError('engine built without SYNCCHECK')
                if not row['modules']:
                    raise ValueError('no synced Wasm modules')
                names = [m['name'] for m in row['modules']]
                if names != sorted(set(names)):
                    raise ValueError('module names must be unique and sorted')
                for module in row['modules']:
                    if module['faulted']:
                        raise ValueError(f"module faulted: {module['name']}")
                    if module['environment'] not in ('rules-synced', 'gaia-synced'):
                        raise ValueError('unsynced module in verification stream')
                    if type(module['bytes']) is not int or module['bytes'] <= 0:
                        raise ValueError('invalid memory size')
                    if len(module['hash']) != 16 or len(module['identity']) != 128:
                        raise ValueError('invalid hash or module identity')
                if type(row['engine']) is not int or type(row['checksum']) is not int:
                    raise ValueError('invalid engine checksum')
            except (ValueError, TypeError, KeyError) as error:
                raise VerificationError(f'{path}:{number}: {error}') from error
            previous = frame
            yield row
    if previous == -1:
        raise VerificationError(f'empty log: {path}')


def first_difference(left: Path, right: Path):
    count = 0
    for a, b in itertools.zip_longest(load_rows(left), load_rows(right)):
        if a is None or b is None:
            row = a if a is not None else b
            return dict(frame=row['frame'], module='stream', reason='missing frame')
        count += 1
        ma = {m['name']: m for m in a['modules']}
        mb = {m['name']: m for m in b['modules']}
        for name in sorted(ma.keys() | mb.keys()):
            if ma.get(name) != mb.get(name):
                return dict(frame=a['frame'], module=name, reason='Wasm state or module identity',
                            left=ma.get(name), right=mb.get(name))
        for field in ('engine', 'checksum'):
            if a[field] != b[field]:
                return dict(frame=a['frame'], module='engine', reason=field,
                            left=a[field], right=b[field])
    return dict(equal=True, frames=count)


def memory_difference(left: Path, right: Path):
    a, b = left.read_bytes(), right.read_bytes()
    offsets = [i for i, (x, y) in enumerate(itertools.zip_longest(a, b)) if x != y]
    return dict(left_bytes=len(a), right_bytes=len(b), differing_bytes=len(offsets),
                pages=sorted({i // 65536 for i in offsets}), first_offsets=offsets[:64])


def sha256(path: Path):
    h = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            h.update(block)
    return h.hexdigest()


def mount_game(source: Path, target: Path, fixture: Path | None, worker_fixture: Path | None = None):
    # Directory archives do not traverse symlinked subdirectories. Use real
    # directories and hard-link immutable files; unlink before overlay writes.
    def link_file(source_file, destination):
        try:
            os.link(source_file, destination)
        except OSError:
            shutil.copy2(source_file, destination)
        return destination
    shutil.copytree(source, target, copy_function=link_file)
    if fixture or worker_fixture:
        wasm = target / 'LuaRules/wasm'
        manifest = wasm / 'manifest.txt'
        text = manifest.read_text()
        manifest.unlink()
        for name, path, order in [('sync-jobs-worker', worker_fixture, 998),
                                  ('sync-jobs-test', fixture, 999)]:
            if path:
                text += f'\nmodule({name}, LuaRules/wasm/{name}.wasm, rules-synced, {order}, 1.0.0)\n'
                destination = wasm / f'{name}.wasm'
                # An earlier fixture may already exist in the pinned input.
                # Detach its hard link before writing any overlay bytes.
                if destination.exists() or destination.is_symlink():
                    destination.unlink()
                shutil.copy2(path, destination)
        manifest.write_text(text)


def replay(args, threads: int, run: Path, dump_frame=-1, record=False, validate_recorded_warnings=True):
    run.mkdir()
    isolation, write = run / 'isolation', run / 'write'
    (isolation / 'games').mkdir(parents=True)
    (isolation / 'maps').mkdir()
    write.mkdir()
    mount_game(args.game, isolation / 'games/game.sdd', args.fixture, args.worker_fixture)
    for name in ('base', 'fonts', 'AI'):
        source = args.spring.parent / name
        if source.exists():
            (isolation / name).symlink_to(source)
    for path in args.map_dir:
        for source in path.iterdir():
            target = isolation / 'maps' / source.name
            if not target.exists():
                target.symlink_to(source)
    config = run / 'spring.conf'
    log = run / 'sync.jsonl'
    settings = dict(WorkerThreadCount=threads, PathingThreadCount=threads,
                    HardwareThreadCount=threads, DemoFromDemo=0, LuaUI=0,
                    UseLuaMemPools=0, TextureMemPoolSize=0, snd_volmaster=0,
                    WasmSyncLog=log, WasmSyncStopFrame=args.frames,
                    WasmSyncDumpFrame=dump_frame, LogSections='*', MaxTextureAtlasSizeX=16384,
                    MaxTextureAtlasSizeY=16384, ROAM=0, GrassDetail=0)
    config.write_text(''.join(f'{key} = {value}\n' for key, value in settings.items()))
    input_file = args.demo
    if record:
        data = gzip.decompress(args.demo.read_bytes())
        if data[:16] != b'spring demofile\0' or struct.unpack_from('<i', data, 16)[0] != 5:
            raise VerificationError('record-first requires a version-5 Spring demo')
        offset = struct.unpack_from('<i', data, 20)[0]
        size = struct.unpack_from('<i', data, 304)[0]
        script = data[offset:offset + size].decode().rstrip('\0')
        # A fresh host reuses the pinned scenario, not old checksum echoes.
        script = re.sub(r'(?im)^\s*demofile=.*;\s*$', '', script)
        with socket.socket() as probe:
            probe.bind(('127.0.0.1', 0))
            port = probe.getsockname()[1]
        script = re.sub(r'(?im)^\s*hostport=.*;', f'hostport={port};', script)
        if args.instruction_fuel is not None:
            script = re.sub(r'(?im)^\s*wasm_instruction_fuel=.*;\s*$', '', script)
            script = re.sub(r'(?i)(\[modoptions\]\s*\{)',
                            lambda match: match[1] + f'\nwasm_instruction_fuel={args.instruction_fuel};', script)
        input_file = run / 'record-script.txt'
        input_file.write_text(script)
    command = [str(args.spring), '--write-dir', str(write), '--isolation',
               '--isolation-dir', str(isolation), '--config', str(config), str(input_file)]
    env = dict(os.environ, RAYON_NUM_THREADS='2')
    if args.enable_synced_timers:
        env['SPRING_ENABLE_SYNCED_TIMERS'] = '1'
    with (run / 'console.log').open('w') as console:
        result = subprocess.run(command, env=env, stdout=console, stderr=subprocess.STDOUT,
                                timeout=args.timeout)
    if result.returncode:
        raise VerificationError(f'engine exited {result.returncode}; see {run}/console.log')
    rows = list(load_rows(log))
    if rows[-1]['frame'] != args.frames:
        raise VerificationError(f"expected final frame {args.frames}, got {rows[-1]['frame']}")
    for row in rows:
        required = {args.require_module} | ({"sync-jobs-test"} if args.fixture else set())
        if args.worker_fixture:
            required.add("sync-jobs-worker")
        names = {m["name"] for m in row["modules"]}
        if not required <= names:
            raise VerificationError(f"required modules {sorted(required - names)} missing at frame {row['frame']}")
        if "workers" in row and row["workers"] != threads:
            raise VerificationError(f"requested {threads} workers, engine used {row['workers']} at frame {row['frame']}")
    infolog = (write / 'infolog.txt').read_text(errors='replace')
    if validate_recorded_warnings and '[DESYNC WARNING]' in infolog and not args.allow_recorded_desync:
        raise VerificationError(f'recorded replay desync warning; see {write}/infolog.txt')
    if record:
        demos = list((write / 'demos').glob('*.sdfz'))
        if len(demos) != 1:
            raise VerificationError(f'expected one recorded demo, found {len(demos)}')
        return log, demos[0]
    return log


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest='mode', required=True)
    compare = sub.add_parser('compare')
    compare.add_argument('left', type=Path)
    compare.add_argument('right', type=Path)
    memory = sub.add_parser('memory-diff')
    memory.add_argument('left', type=Path)
    memory.add_argument('right', type=Path)
    run = sub.add_parser('run')
    for name in ('spring', 'game', 'demo', 'output'):
        run.add_argument('--' + name, type=Path, required=True)
    run.add_argument('--map-dir', type=Path, action='append', default=[])
    run.add_argument('--threads', type=int, nargs='+', default=[1, 4])
    run.add_argument('--frames', type=int, default=1200, help='inclusive final sim frame')
    run.add_argument('--timeout', type=float, default=900)
    run.add_argument('--fixture', type=Path)
    run.add_argument('--worker-fixture', type=Path, help='optional dedicated sync-jobs-worker module')
    run.add_argument('--dump-frame', type=int, default=-1)
    run.add_argument('--record-first', action='store_true', help='record the pinned scenario on this engine before comparing fresh replay runs')
    run.add_argument('--instruction-fuel', type=int, help='fuel for a freshly recorded scenario; requires --record-first')
    run.add_argument('--require-module', default='shipgame-rules')
    run.add_argument('--allow-recorded-desync', action='store_true', help='explicitly allow old-engine checksums in a replay; local per-frame comparison remains mandatory')
    run.add_argument('--enable-synced-timers', action='store_true', help='only for deliberate desync fixture')
    run.add_argument('--expect-desync-frame', type=int)
    run.add_argument('--expect-desync-module', default='sync-jobs-test')
    args = parser.parse_args()
    if args.mode == 'memory-diff':
        print(json.dumps(memory_difference(args.left, args.right), indent=2))
        return 0
    if args.mode == 'compare':
        result = first_difference(args.left, args.right)
        print(json.dumps(result, indent=2))
        return 0 if result.get('equal') else 1
    if len(args.threads) < 2 or any(t < 1 or t > 32 for t in args.threads):
        parser.error('at least two worker counts in 1..32 are required')
    if args.instruction_fuel is not None and (not args.record_first or args.instruction_fuel < 0):
        parser.error('--instruction-fuel requires --record-first and a nonnegative budget')
    if args.frames < 0:
        parser.error('--frames must be nonnegative')
    if args.output.exists():
        parser.error('output already exists; use a new directory to preserve prior evidence')
    for name in ('spring', 'game', 'demo', 'output', 'fixture', 'worker_fixture'):
        value = getattr(args, name)
        if value is not None:
            setattr(args, name, value.resolve())
    args.output.mkdir(parents=True)
    provenance = dict(command=sys.argv, engine_sha256=sha256(args.spring),
                      demo_sha256=sha256(args.demo), game=str(args.game),
                      fixture_sha256=sha256(args.fixture) if args.fixture else None,
                      worker_fixture_sha256=sha256(args.worker_fixture) if args.worker_fixture else None,
                      threads=args.threads, final_frame=args.frames)
    (args.output / 'provenance.json').write_text(json.dumps(provenance, indent=2) + '\n')
    recorded_log = None
    if args.record_first:
        print('recording a fresh demo from the pinned scenario on this engine', flush=True)
        recorded_log, args.demo = replay(args, args.threads[0], args.output / 'record', args.dump_frame, record=True)
        args.allow_recorded_desync = False
        provenance['recorded_demo'] = str(args.demo)
        provenance['recorded_demo_sha256'] = sha256(args.demo)
        (args.output / 'provenance.json').write_text(json.dumps(provenance, indent=2) + '\n')
    baseline = None
    results = []
    recording_comparisons = []
    recorded_warnings = []
    for index, threads in enumerate(args.threads):
        print(f'replaying with {threads} workers (including sim thread)', flush=True)
        run = args.output / f'run-{index}-{threads}'
        # Compare complete state first so coarse demo warnings cannot obscure
        # the earliest frame/module. Equal streams still must pass demo checks.
        log = replay(args, threads, run, args.dump_frame, validate_recorded_warnings=False)
        if '[DESYNC WARNING]' in (run / 'write/infolog.txt').read_text(errors='replace'):
            recorded_warnings.append(str(run / 'write/infolog.txt'))
        if recorded_log is not None:
            result = first_difference(recorded_log, log)
            recording_comparisons.append(result)
            if not result.get('equal'):
                result['comparison'] = 'recording versus playback'
                print(json.dumps(result, indent=2), flush=True)
                (args.output / 'result.json').write_text(json.dumps(result, indent=2) + '\n')
                return 1
        if baseline is None:
            baseline = log
            continue
        result = first_difference(baseline, log)
        results.append(result)
        print(json.dumps(result, indent=2), flush=True)
        expected = args.expect_desync_frame
        if expected is not None:
            if result.get('frame') != expected or result.get('module') != args.expect_desync_module:
                raise VerificationError(f'expected desync at frame {expected} in {args.expect_desync_module}, got {result}')
        elif not result.get('equal'):
            (args.output / 'result.json').write_text(json.dumps(result, indent=2) + '\n')
            return 1
    if recorded_warnings and not args.allow_recorded_desync:
        raise VerificationError(f'recorded replay desync warning; see {recorded_warnings[0]}')
    (args.output / 'result.json').write_text(json.dumps(dict(passed=True, comparisons=results,
        recording_comparisons=recording_comparisons), indent=2) + '\n')
    return 0


if __name__ == '__main__':
    try:
        raise SystemExit(main())
    except (VerificationError, OSError, subprocess.TimeoutExpired) as error:
        print(f'verification failed: {error}', file=sys.stderr)
        raise SystemExit(2)
