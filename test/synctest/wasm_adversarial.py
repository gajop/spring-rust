#!/usr/bin/env python3
# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
"""Exercise the official Wasm SDK in fresh hosts and recorded-demo playback.

Uses the engine's small public parity game, never a private game's content.
Complete memory/module/job hashes and engine checksums are compared every frame.
"""
from __future__ import annotations

import argparse
import gzip
import hashlib
import json
import os
import re
import shutil
import socket
import struct
import platform
import traceback
import time
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'test/native_api_parity'))
from run_harness import blank_map_name, prepare_datadir, write_script
from wasm_sync_replay import first_difference, load_rows

GUEST = ROOT / 'test/wasm_api/guests/sync_adversarial'


def file_hash(path):
    h = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            h.update(block)
    return h.hexdigest()


def public_content_hash():
    h = hashlib.sha256()
    for folder in ['test/native_api_parity/fixtures/game.sdd', 'cont/base/springcontent',
                   'cont/base/maphelper', 'cont/base/bitmaps', 'cont/base/cursors']:
        for path in sorted((ROOT/folder).rglob('*'), key=lambda p:p.relative_to(ROOT).as_posix().split('/')):
            if path.is_file():
                h.update(path.relative_to(ROOT).as_posix().encode())
                h.update(bytes.fromhex(file_hash(path)))
    return h.hexdigest()


def prepare_run(args, name, workers, guest, demo=None):
    out = args.output / name
    out.mkdir()  # Never overwrite earlier evidence.
    data, _ = prepare_datadir(out, None, guest)
    game = data / 'games/native_api_parity.sdd'
    gadgets = game / 'LuaRules/Gadgets'
    for source in gadgets.glob('*.lua'):
        source.unlink()
    (game / 'LuaUI/main.lua').write_text('')
    if args.hash_interval:
        rules = game/'gamedata/modrules.lua'
        original = rules.read_text()
        if 'pathFinderSystem = 0,' not in original:
            raise RuntimeError('public fixture modrules changed')
        rules.write_text(original.replace('pathFinderSystem = 0,',
            f'pathFinderSystem = 0,\n        WasmStateHashInterval = {args.hash_interval},'))
    manifest = game/'LuaRules/wasm/manifest.txt'
    manifest.write_text(''.join(f'module(rules-{i}, LuaRules/wasm/parity.wasm, rules-synced, {i}, 1.0.0)\n'
        for i in range(args.rules_modules)))
    if args.hidden_state_control:
        shutil.copy2(args.hidden_fixture,manifest.parent/'hidden.wasm')
        with manifest.open('a') as stream:
            stream.write('module(hidden-state, LuaRules/wasm/hidden.wasm, rules-synced, 99, 1.0.0)\n')
    if args.state_workload:
        shutil.copy2(args.state_fixture,manifest.parent/'state-workload.wasm')
        with manifest.open('a') as stream:
            stream.write('module(state-workload, LuaRules/wasm/state-workload.wasm, rules-synced, 100, 1.0.0)\n')
    gaia_manifest = out/'gaia-manifest.txt'
    gaia_manifest.write_text(''.join(f'module(gaia-{i}, LuaGaia/wasm/parity.wasm, gaia-synced, {i}, 1.0.0)\n'
        for i in range(args.gaia_modules)))
    (gadgets / 'sync_adversarial.lua').write_text('''function gadget:GetInfo()
    return {name="Wasm sync adversarial setup", layer=-100000, enabled=true}
end
if not gadgetHandler:IsSyncedCode() then return false end
local units = {}
function gadget:Initialize()
    Spring.SetGameRulesParam("sync_adversarial_clock", CLOCK_MODE)
    Spring.SetGameRulesParam("sync_adversarial_seed", TEST_SEED)
    Spring.SetGameRulesParam("sync_adversarial_cross", CROSS_WORLD)
    for i=1,8 do
        units[i] = assert(Spring.CreateUnit("native_api_test_unit", 256+i*64,96,256,0,0))
    end
end
function gadget:GameFrame(frame)
    if frame % 23 == 0 then
        for i,unit in ipairs(units) do
            Spring.GiveOrderToUnit(unit, CMD.MOVE, {256+i*64,96,256+(frame%200)*2}, {})
        end
    end
end
'''.replace('CLOCK_MODE', '1' if args.negative_control else '0')
        .replace('TEST_SEED', str(args.seed)).replace('CROSS_WORLD', '1' if args.rules_modules and args.gaia_modules else '0'))
    settings = dict(WorkerThreadCount=workers, PathingThreadCount=workers,
        HardwareThreadCount=workers, DemoFromDemo=0, LuaUI=0, UseLuaMemPools=0,
        TextureMemPoolSize=0, snd_volmaster=0, WasmSyncLog=out/'sync.jsonl',
        WasmSyncStopFrame=args.frames, WasmSyncDumpFrame=args.dump_frame, LogSections='*', ROAM=0, GrassDetail=0)
    config = out / 'spring.conf'
    config.write_text(''.join(f'{key} = {value}\n' for key, value in settings.items()))
    script = out / 'script.txt'
    parity = argparse.Namespace(blank_map_x=10, blank_map_y=8, blank_map_height=96,
        blank_map_seed=424242, cases=1, tests=None, test_prefix=None,
        enable_rendering_tests=False, process_test=None)
    write_script(script, blank_map_name(parity, 'wasm'), 'sync-adversarial',
        'wasm', True, parity, 424242, 'synced_gadget', None)
    text = script.read_text().replace('RecordDemo=0;', 'RecordDemo=1;')
    text = text.replace('MaxSpeed=1;', 'MaxSpeed=20;').replace('MinSpeed=1;', 'MinSpeed=20;')
    engine = args.guest_spring if name == 'live-guest' and args.guest_spring else args.spring
    under_wine = engine.suffix.lower() == '.exe' and os.name != 'nt'
    def engine_path(path):
        return 'Z:' + str(path).replace('/', '\\') if under_wine else str(path)
    replacements = {}
    if args.gaia_modules:
        replacements = dict(blank_map_gaia_manifest=engine_path(gaia_manifest),
                            blank_map_gaia_module=engine_path(args.gaia_fixture))
        text = text.replace('[MAPOPTIONS]\n    {', '[MAPOPTIONS]\n    {\n' +
            ''.join(f'        {key}={value};\n' for key,value in replacements.items()))
    script.write_text(text)
    if demo and replacements:
        # Only relocate map-generator mount paths in the startscript. Network
        # packets, frame inputs and checksums remain byte-for-byte unchanged.
        raw = gzip.decompress(demo.read_bytes())
        if raw[:16] != b'spring demofile\0' or struct.unpack_from('<i',raw,16)[0] != 5:
            raise RuntimeError('expected version-5 demo')
        start = struct.unpack_from('<i',raw,20)[0]
        size = struct.unpack_from('<i',raw,304)[0]
        setup = raw[start:start+size].decode()
        for key,value in replacements.items():
            setup, count = re.subn(r'(?im)(\b' + key + r'=)[^;]*;', lambda m: m[1]+value+';', setup)
            if count != 1:
                raise RuntimeError(f'expected one demo mount path: {key}')
        encoded = setup.encode()
        header = bytearray(raw[:start])
        struct.pack_into('<i',header,304,len(encoded))
        relocated = out/'relocated.sdfz'
        relocated.write_bytes(gzip.compress(header+encoded+raw[start+size:],mtime=0))
        (out/'demo-input.json').write_text(json.dumps(dict(original_sha256=file_hash(demo),
            network_stream_sha256=hashlib.sha256(raw[start+size:]).hexdigest()),indent=2)+'\n')
        demo = relocated
    write = out/'write'
    write.mkdir()
    engine = args.guest_spring if name == 'live-guest' and args.guest_spring else args.spring
    command = [str(engine), '--nocolor', '--write-dir', str(write),
        '--isolation', '--isolation-dir', str(data), '--config', str(config), str(demo or script)]
    env = dict(os.environ, RAYON_NUM_THREADS='2', SPRING_WASM_CORE_HOST='1')
    env.pop('SPRING_NATIVE_MODULE', None)
    env.pop('SPRING_ENABLE_SYNCED_TIMERS', None)
    if args.negative_control or args.hidden_state_control:
        env['SPRING_ENABLE_SYNCED_TIMERS'] = '1'
    if engine.suffix.lower() == '.exe' and os.name != 'nt':
        if not args.wine_prefix:
            raise ValueError('Wine requires --wine-prefix pointing to a dedicated test prefix')
        def windows_path(value):
            return 'Z:' + str(value).replace('/', '\\')
        text = config.read_text()
        config.write_text('\n'.join(
            key + '=' + windows_path(value.strip()) if value.strip().startswith('/') else line
            for line in text.splitlines() for key, _, value in [line.partition('=')])+'\n')
        env.update(WINEPREFIX=str(args.wine_prefix.resolve()), WINEDEBUG='-all')
        command = [str(args.wine), str(engine),
            *(windows_path(arg) if arg.startswith('/') else arg for arg in command[1:])]
    (out/'command.json').write_text(json.dumps(command, indent=2)+'\n')
    (out/'request.json').write_text(json.dumps(dict(workers=workers,
        engine_sha256=file_hash(engine), guest_sha256=file_hash(guest)),indent=2)+'\n')
    return out, command, env


def validate_run(args, out):
    name = out.name
    log = (out/'console.log').read_text(errors='replace')
    count = args.rules_modules + args.gaia_modules
    milestone = args.frames // 60 * 60
    if log.count(f'[SyncAdversarial] frame={milestone} ') != count:
        raise RuntimeError(f'{name}: expected {count} module markers at frame {milestone}')
    if log.count('[SyncAdversarial] initialized') != count or 'Wasm trap' in log:
        raise RuntimeError(f'{name}: module failed; inspect {out}/console.log')
    rows = list(load_rows(out/'sync.jsonl', expected_modules=args.expected_modules, require_state=True))
    requested = json.loads((out/'request.json').read_text())['workers']
    if any(row['workers'] != requested for row in rows):
        raise RuntimeError(f'{name}: engine did not use the requested {requested} workers')
    if rows[-1]['frame'] != args.frames:
        raise RuntimeError(f'{name}: expected frame {args.frames}, got {rows[-1]["frame"]}')
    if '[DESYNC WARNING]' in log and not args.negative_control:
        raise RuntimeError(f'{name}: engine reported desync; inspect {out}/console.log')
    return out


def run(args, name, workers, guest, demo=None):
    prepared = prepare_run(args, name, workers, guest, demo)
    out, command, env = prepared
    with (out/'console.log').open('w') as console:
        subprocess.run(command, env=env, stdout=console, stderr=subprocess.STDOUT,
            timeout=args.timeout, check=True)
    return validate_run(args, out)


def live(args, guest):
    host, host_cmd, host_env = prepare_run(args, 'live-host', 1, guest)
    peer, peer_cmd, peer_env = prepare_run(args, 'live-guest', 4, guest)
    with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as probe:
        probe.bind(('127.0.0.1', 0))
        port = probe.getsockname()[1]
    script = host/'script.txt'
    text = script.read_text().replace('OnlyLocal=1;', 'OnlyLocal=0;').replace('NumPlayers=1;', 'NumPlayers=2;')
    text = re.sub(r'HostPort=\d+;', f'HostPort={port};', text)
    text = text.replace('[PLAYER0]', '[PLAYER1] {Name=Guest; Spectator=1; Team=0;}\n    [PLAYER0]')
    script.write_text(text)
    (peer/'script.txt').write_text(f'[GAME]{{IsHost=0;HostIP=127.0.0.1;HostPort={port};MyPlayerName=Guest;}}')
    processes = []
    began = time.monotonic()
    try:
        for out, command, env in [(host,host_cmd,host_env),(peer,peer_cmd,peer_env)]:
            log = (out/'console.log').open('w')
            process = subprocess.Popen(command, env=env, stdout=log, stderr=subprocess.STDOUT)
            processes.append((process,log))
        for process, _ in processes:
            process.wait(timeout=max(1,args.timeout-(time.monotonic()-began)))
            if process.returncode:
                raise RuntimeError(f'live peer exited {process.returncode}')
    finally:
        for process, log in processes:
            if process.poll() is None:
                process.terminate()
                try:
                    process.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    process.kill()
                    process.wait()
            log.close()
    return validate_run(args,host), validate_run(args,peer)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--spring', type=Path, default=ROOT/'build-amd64-linux/install/spring-headless')
    parser.add_argument('--guest-spring', type=Path, help='With --live: a second platform engine')
    parser.add_argument('--wine', type=Path, default=Path('wine'))
    parser.add_argument('--wine-prefix', type=Path)
    parser.add_argument('--hash-interval', type=int, default=0, help='Game opt-in interval for full-state network checks')
    parser.add_argument('--check-unlogged-replay', action='store_true', help='Also verify recorded network checksums with diagnostics disabled')
    parser.add_argument('--state-workload', action='store_true', help='Also exercise hidden globals, indirect calls and bulk-memory/element operations')
    parser.add_argument('--fixture', type=Path, help='Prebuilt Rules guest; skip Rust compilation')
    parser.add_argument('--gaia-fixture', type=Path, help='Prebuilt Gaia guest')
    parser.add_argument('--rules-modules', type=int, default=1)
    parser.add_argument('--gaia-modules', type=int, default=0)
    parser.add_argument('--seed', type=int, default=424242)
    parser.add_argument('--corpus', type=Path, help='Replay exact saved fixtures/demo/scenario; no Rust toolchain needed')
    parser.add_argument('--artifact-dir', type=Path, help='Shared immutable engine copy pool for large matrices')
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--frames', type=int, default=240)
    parser.add_argument('--live', action='store_true', help='Compare real TCP/UDP-connected host and guest instead of replay')
    parser.add_argument('--fresh-hosts', action='store_true', help='Diagnostic: compare fresh matches too; their network-stat input streams may differ')
    parser.add_argument('--dump-frame', type=int, default=-1)
    parser.add_argument('--timeout', type=int, default=120)
    parser.add_argument('--workers', type=int, nargs='+', default=[1,4,8])
    parser.add_argument('--hidden-state-control', action='store_true', help='Require frame-7 hidden global/table divergence with equal memory')
    parser.add_argument('--negative-control', action='store_true',
        help='Explicitly enable a clock read at frame 7; comparison MUST detect it')
    args = parser.parse_args()
    if args.corpus:
        args.corpus = args.corpus.resolve()
        corpus = json.loads((args.corpus/'corpus.json').read_text())
        for key in ('frames', 'seed', 'rules_modules', 'gaia_modules', 'state_workload', 'hash_interval'):
            setattr(args, key, corpus.get(key, 0) if key in ('state_workload','hash_interval') else corpus[key])
        args.fixture = args.corpus/'fixture.wasm'
        args.gaia_fixture = args.corpus/'gaia.wasm' if args.gaia_modules else None
        if corpus.get('public_content_sha256') != public_content_hash():
            parser.error('public parity/base content differs from the saved corpus')
        for name,digest in corpus['files'].items():
            if file_hash(args.corpus/name) != digest:
                parser.error(f'corpus hash mismatch: {name}')
        if args.live or args.negative_control or args.hidden_state_control or args.fresh_hosts:
            parser.error('--corpus is exact replay only')
    if not 0 <= args.rules_modules <= 8 or not 0 <= args.gaia_modules <= 8 or not args.rules_modules + args.gaia_modules:
        parser.error('one to sixteen modules, at most eight per world')
    if not 0 <= args.seed <= 16777215 or any(not 1 <= w <= 32 for w in args.workers):
        parser.error('seed must be exactly representable as f32 (0..16777215), workers 1..32')
    if args.guest_spring and not args.live:
        parser.error('--guest-spring requires --live')
    if not args.workers or len(set(args.workers)) != len(args.workers):
        parser.error('provide distinct worker counts')
    if args.hash_interval < 0:
        parser.error('hash interval must be nonnegative')
    if args.check_unlogged_replay and (args.live or args.negative_control or args.hidden_state_control):
        parser.error('unlogged policy check requires positive recorded playback')
    if args.frames < 8:
        parser.error('--frames must be at least 8')
    if args.negative_control and args.hidden_state_control:
        parser.error('choose one negative control')
    args.spring = args.spring.resolve()
    args.output = args.output.resolve()
    args.output.mkdir()  # A new directory keeps failed runs and provenance intact.
    built_guest = GUEST/'target/wasm32-unknown-unknown/release/wasm_sync_adversarial.wasm'
    def build_guest(gaia=False):
        subprocess.run(['cargo','build','--manifest-path',str(GUEST/'Cargo.toml'),
            '--locked','--release','--target','wasm32-unknown-unknown',
            *(['--features','gaia'] if gaia else [])],cwd=GUEST,check=True)
        return built_guest
    guest = args.output/'fixture.wasm'
    shutil.copy2(args.fixture or build_guest(),guest)
    if args.gaia_modules:
        gaia = args.output/'gaia.wasm'
        shutil.copy2(args.gaia_fixture or build_guest(True),gaia)
        args.gaia_fixture = gaia
    args.expected_modules = {
        **{f'rules-{i}': ('rules-synced', hashlib.sha512(guest.read_bytes()).hexdigest()) for i in range(args.rules_modules)},
        **{f'gaia-{i}': ('gaia-synced', hashlib.sha512(args.gaia_fixture.read_bytes()).hexdigest()) for i in range(args.gaia_modules)}}
    if args.state_workload:
        args.state_fixture = args.output/'state-workload.wasm'
        if args.corpus:
            shutil.copy2(args.corpus/'state-workload.wasm',args.state_fixture)
        else:
            subprocess.run(['wat2wasm',str(ROOT/'test/wasm_api/data/sync_adversarial/state_workload.wat'),
                '-o',str(args.state_fixture)],check=True)
        args.expected_modules['state-workload'] = ('rules-synced', hashlib.sha512(args.state_fixture.read_bytes()).hexdigest())
    if args.hidden_state_control:
        args.hidden_fixture = args.output/'hidden.wasm'
        subprocess.run(['wat2wasm',str(ROOT/'test/wasm_api/data/sync_adversarial/hidden_state.wat'),
            '-o',str(args.hidden_fixture)],check=True)
        args.expected_modules['hidden-state'] = ('rules-synced', hashlib.sha512(args.hidden_fixture.read_bytes()).hexdigest())
    provenance = dict(engine_sha256=file_hash(args.spring), guest_sha256=file_hash(guest),
        platform=platform.platform(), machine=platform.machine(), seed=args.seed,
        public_content_sha256=public_content_hash(),
        expected_modules=args.expected_modules,
        workers=args.workers, frames=args.frames, negative_control=args.negative_control, hidden_state_control=args.hidden_state_control,
        live=args.live, fresh_hosts=args.fresh_hosts,
        hash_interval=args.hash_interval)
    try:
        top = subprocess.check_output(['git','rev-parse','--show-toplevel'],cwd=ROOT,text=True,stderr=subprocess.DEVNULL).strip()
        if Path(top).resolve() != ROOT.resolve():
            raise ValueError('standalone kit is not the engine Git root')
        provenance['head'] = subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True,stderr=subprocess.DEVNULL).strip()
        patch = subprocess.check_output(['git','diff','--binary','HEAD'],cwd=ROOT,stderr=subprocess.DEVNULL)
    except (FileNotFoundError,subprocess.CalledProcessError,ValueError):
        provenance['head'] = None
        provenance['source_note'] = 'Portable test bundle; no Git checkout/tool required'
        patch = b''
    if (ROOT/'bundle.json').is_file():
        provenance['bundle_manifest_sha256'] = file_hash(ROOT/'bundle.json')
    (args.output/'engine.patch').write_bytes(patch)
    provenance['tracked_patch_sha256'] = hashlib.sha256(patch).hexdigest()
    # Include untracked fixture/guard source; a commit hash alone is insufficient.
    sources = [Path(__file__), GUEST/'Cargo.toml', GUEST/'Cargo.lock', GUEST/'.cargo/config.toml', GUEST/'src/lib.rs',
        ROOT/'rts/WasmInterface/runtime/WasmFloatEnvironment.h',
        ROOT/'rts/WasmInterface/core/host/WasmCoreState.cpp', ROOT/'rts/WasmInterface/core/host/WasmCoreState.h',
        ROOT/'test/synctest/wasm_sync_replay.py', ROOT/'test/wasm_api/data/sync_adversarial/hidden_state.wat', ROOT/'test/wasm_api/data/sync_adversarial/state_workload.wat']
    provenance['sources'] = {str(p.relative_to(ROOT)):file_hash(p) for p in sources if p.exists()}
    for path in sources:
        if path.exists():
            saved = args.output/'source'/path.relative_to(ROOT)
            saved.parent.mkdir(parents=True,exist_ok=True)
            shutil.copy2(path,saved)
    def pin_engine(original):
        original = original.resolve()
        folder = (args.artifact_dir or args.output/'artifacts')/file_hash(original)
        folder.mkdir(parents=True,exist_ok=True)
        for source in [original, *original.parent.glob('*.dll')]:
            target = folder/source.name
            if not target.exists():
                temporary = folder/(source.name+'.tmp')
                shutil.copy2(source,temporary)
                if file_hash(temporary) != file_hash(source):
                    raise RuntimeError('engine changed while pinning')
                temporary.replace(target)
            elif file_hash(target) != file_hash(source):
                raise RuntimeError(f'immutable artifact pool collision: {target}')
        return folder/original.name
    args.spring = pin_engine(args.spring)
    if args.guest_spring:
        args.guest_spring = pin_engine(args.guest_spring)
        provenance['guest_engine_sha256'] = file_hash(args.guest_spring)
        provenance['guest_dlls'] = {p.name:file_hash(p) for p in args.guest_spring.parent.glob('*.dll')}
    (args.output/'provenance.json').write_text(json.dumps(provenance,indent=2)+'\n')
    host_workers = args.workers if args.fresh_hosts or args.negative_control or args.hidden_state_control else args.workers[:1]
    if (args.negative_control or args.hidden_state_control) and len(host_workers) < 2:
        parser.error('negative control needs at least two worker entries')
    runs = ([run(args, f'replay-workers-{workers}', workers, guest, args.corpus/'scenario.sdfz')
             for workers in args.workers] if args.corpus else
            list(live(args,guest)) if args.live else [run(args, f'host-{index}-workers-{workers}', workers, guest)
                for index, workers in enumerate(host_workers)])
    baseline = runs[0]
    if not args.negative_control and not args.hidden_state_control and not args.live and not args.corpus:
        demos = list((baseline/'write/demos').glob('*.sdfz'))
        if len(demos) != 1:
            raise RuntimeError(f'expected one recorded demo, found {len(demos)}')
        corpus = args.output/'corpus'
        corpus.mkdir()
        for source,name in [(guest,'fixture.wasm'),(demos[0],'scenario.sdfz'),(baseline/'sync.jsonl','baseline.jsonl')]:
            shutil.copy2(source,corpus/name)
        if args.gaia_modules:
            shutil.copy2(args.gaia_fixture,corpus/'gaia.wasm')
        if args.state_workload:
            shutil.copy2(args.state_fixture,corpus/'state-workload.wasm')
        metadata = {key:getattr(args,key) for key in ('frames','seed','rules_modules','gaia_modules','state_workload','hash_interval')}
        metadata['public_content_sha256'] = public_content_hash()
        metadata['files'] = {p.name:file_hash(p) for p in corpus.iterdir()}
        (corpus/'corpus.json').write_text(json.dumps(metadata,indent=2)+'\n')
        for workers in args.workers:
            runs.append(run(args, f'replay-workers-{workers}', workers, guest, demos[0]))
    comparisons = []
    if args.corpus:
        comparisons.append(dict(run=baseline.name, **first_difference(args.corpus/'baseline.jsonl',baseline/'sync.jsonl')))
    for other in runs[1:]:
        comparison = first_difference(baseline/'sync.jsonl', other/'sync.jsonl')
        comparisons.append(dict(run=other.name, **comparison))
    (args.output/'comparisons.json').write_text(json.dumps(comparisons,indent=2)+'\n')
    for result in comparisons:
        print(json.dumps(result))
    if args.negative_control or args.hidden_state_control:
        expected_module = 'hidden-state' if args.hidden_state_control else sorted(args.expected_modules)[0]
        if not comparisons or any(r.get('frame') != 7 or r.get('module') != expected_module for r in comparisons):
            raise RuntimeError('negative control did not identify the injected frame-7 Wasm divergence')
        if args.hidden_state_control and any(r['left']['hash'] != r['right']['hash'] or r['left']['state_hash'] == r['right']['state_hash'] for r in comparisons):
            raise RuntimeError('hidden-state control must differ in globals/tables with identical memory')
    elif not comparisons:
        raise RuntimeError('no independent comparison was performed')
    elif any(not result.get('equal') for result in comparisons):
        raise RuntimeError('determinism comparison failed; see comparisons.json')

    policy = None
    if args.check_unlogged_replay:
        demo = args.corpus/'scenario.sdfz' if args.corpus else demos[0]
        out,command,env = prepare_run(args,'unlogged-replay',args.workers[-1],guest,demo)
        config = out/'spring.conf'
        text,count = re.subn(r'(?m)^WasmSyncLog\s*=.*$', 'WasmSyncLog =',config.read_text())
        if count != 1:
            raise RuntimeError('expected one diagnostic log setting')
        config.write_text(text)
        with (out/'console.log').open('w') as console:
            subprocess.run(command,env=env,stdout=console,stderr=subprocess.STDOUT,timeout=args.timeout,check=True)
        log = (out/'console.log').read_text(errors='replace')
        count = args.rules_modules+args.gaia_modules
        observed_frames = [int(f) for f in re.findall(r'\[f=(-?\d+)\]',log)]
        if not observed_frames or max(observed_frames) != args.frames:
            raise RuntimeError('unlogged replay did not reach the exact requested frame')
        if log.count(f'[SyncAdversarial] frame={args.frames//60*60} ') != count or '[DESYNC WARNING]' in log or 'Wasm trap' in log or (out/'sync.jsonl').exists():
            raise RuntimeError('unlogged replay failed the network-checksum/fixture policy check')
        policy = dict(passed=True,kind='recorded network checksums with diagnostic logging disabled',
            full_state_compared=False,hash_interval=args.hash_interval,reached_frame=args.frames)
    return dict(passed=True, comparisons=comparisons,logging_policy=policy)


if __name__ == '__main__':
    # Preserve machine-readable failures as well as raw evidence. Never turn a
    # crashed, missing, timed-out or partially completed case into a pass.
    output = None
    if '--output' in sys.argv:
        output = Path(sys.argv[sys.argv.index('--output')+1]).resolve()
    existed = output is not None and output.exists()
    began = time.monotonic()
    try:
        result = main()
    except Exception as error:
        result = dict(passed=False,error=str(error),exception=type(error).__name__)
        traceback.print_exc()
    if output and output.is_dir() and not existed:
        result['elapsed_seconds'] = time.monotonic()-began
        (output/'result.json').write_text(json.dumps(result,indent=2)+'\n')
    raise SystemExit(0 if result['passed'] else 1)
