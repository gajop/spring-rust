#!/usr/bin/env python3
# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
"""Serial, resumable Wasm determinism matrix. No uploads or CI dispatch.

Each case retains its raw evidence and structured result, including failures.
Use --profile deep for a longer seeded workload; --resume never erases failures.
"""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
RUNNER = Path(__file__).with_name('wasm_adversarial.py')


def digest(path):
    h = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1024*1024), b''):
            h.update(block)
    return h.hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--spring', type=Path, required=True)
    parser.add_argument('--guest-spring', type=Path, help='Also run live mixed-platform networking')
    parser.add_argument('--wine', type=Path, default=Path('wine'))
    parser.add_argument('--wine-prefix', type=Path)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--profile', choices=['smoke','deep'], default='smoke')
    parser.add_argument('--seeds', type=int, nargs='+')
    parser.add_argument('--repeats', type=int, default=1)
    parser.add_argument('--frames', type=int)
    parser.add_argument('--workers', type=int, nargs='+', default=[1,4,8])
    parser.add_argument('--timeout', type=int, default=900)
    parser.add_argument('--resume', action='store_true')
    args = parser.parse_args()
    if args.repeats < 1 or not args.workers or len(set(args.workers)) != len(args.workers):
        parser.error('positive repeats and distinct workers required')
    if args.output.exists() and not args.resume:
        parser.error('output exists; --resume retains prior attempts')
    args.output = args.output.resolve()
    args.output.mkdir(parents=True,exist_ok=True)
    frames = args.frames or (90 if args.profile == 'smoke' else 1800)
    seeds = args.seeds or ([424242] if args.profile == 'smoke' else [1,424242,16777215])
    # Pin the complete relevant source fingerprint for resume. An engine binary
    # hash alone cannot detect changed guest/workload/verifier implementations.
    sources = []
    for folder in ['test/synctest', 'test/wasm_api/guests/sync_adversarial',
                   'test/native_api_parity/fixtures/game.sdd', 'cont/base',
                   'rust/crates/spring-sdk', 'rust/crates/spring-sdk-internal',
                   'rts/wasm/generated/sdk']:
        sources.extend(p for p in (ROOT/folder).rglob('*') if p.is_file()
            and not set(p.relative_to(ROOT).parts).intersection({'target','__pycache__','.git'}))
    sources.extend((ROOT/'test/wasm_api/data/sync_adversarial').rglob('*.wat'))
    sources.extend(ROOT/p for p in ['test/native_api_parity/run_harness.py','test/native_api_parity/spec_loader.py'])
    plan = dict(profile=args.profile, frames=frames, seeds=seeds, repeats=args.repeats,
        workers=args.workers, timeout=args.timeout, spring=digest(args.spring),
        guest=digest(args.guest_spring) if args.guest_spring else None,
        sources={str(p.relative_to(ROOT)):digest(p) for p in sorted(set(sources))})
    plan_file = args.output/'plan.json'
    if plan_file.exists() and json.loads(plan_file.read_text()) != plan:
        parser.error('resume inputs changed; use a new output directory')
    plan_file.write_text(json.dumps(plan,indent=2)+'\n')
    cases = []
    for seed in seeds:
        for repeat in range(args.repeats):
            for name,rules,gaia in [('rules',2,0),('mixed',2,2)]:
                cases.append((f'{name}-seed-{seed}-repeat-{repeat}',
                    ['--state-workload','--check-unlogged-replay','--hash-interval','1' if name=='mixed' else '0','--seed',str(seed),'--frames',str(frames),'--rules-modules',str(rules),'--gaia-modules',str(gaia)]))
            if args.guest_spring:
                cases.append((f'live-seed-{seed}-repeat-{repeat}',
                    ['--state-workload','--seed',str(seed),'--frames',str(frames),'--rules-modules','2','--gaia-modules','2','--live',
                     '--guest-spring',str(args.guest_spring.resolve()),'--wine',str(args.wine),
                     *(['--wine-prefix',str(args.wine_prefix.resolve())] if args.wine_prefix else [])]))
    cases.append(('hidden-state-negative',['--hidden-state-control','--frames','30','--workers','1','4']))
    cases.append(('clock-negative',['--negative-control','--frames','30','--workers','1','4']))
    results = []
    (args.output/'result.json').write_text(json.dumps(dict(passed=False,complete=False,cases=[]),indent=2)+'\n')
    for name,options in cases:
        if any(not (ROOT/p).is_file() or digest(ROOT/p) != h for p,h in plan['sources'].items()):
            raise RuntimeError('suite source changed during execution; prior results retained, use a new output')
        case = args.output/name
        case.mkdir(exist_ok=True)
        attempts = sorted(p for p in case.glob('attempt-*') if p.is_dir())
        previous = attempts[-1] if attempts else None
        if previous and (previous/'result.json').exists():
            result = json.loads((previous/'result.json').read_text())
            evidence = previous/'evidence.json'
            if result.get('passed') and evidence.exists() and all((previous/p).is_file()
                and digest(previous/p) == h for p,h in json.loads(evidence.read_text()).items()):
                results.append(dict(case=name,attempt=previous.name,passed=True,resumed=True))
                (args.output/'result.json').write_text(json.dumps(dict(passed=False,complete=False,cases=results),indent=2)+'\n')
                continue
        attempt = case/f'attempt-{len(attempts):04d}'
        command = [sys.executable,str(RUNNER),'--spring',str(args.spring.resolve()),
            '--output',str(attempt),'--artifact-dir',str(args.output/'artifacts'),
            '--workers',*[str(w) for w in args.workers],'--timeout',str(args.timeout),*options]
        print(f'Running {name}; evidence: {attempt}',flush=True)
        with (case/f'{attempt.name}.log').open('w') as log:
            status = subprocess.run(command,stdout=log,stderr=subprocess.STDOUT).returncode
        result_file = attempt/'result.json'
        result = json.loads(result_file.read_text()) if result_file.exists() else dict(passed=False,error='missing result')
        passed = status == 0 and result.get('passed') is True
        if passed:
            evidence = {str(p.relative_to(attempt)):digest(p) for p in attempt.rglob('*')
                if p.is_file() and p.name != 'evidence.json'}
            (attempt/'evidence.json').write_text(json.dumps(evidence,indent=2)+'\n')
        results.append(dict(case=name,attempt=attempt.name,passed=passed,error=result.get('error')))
        (args.output/'result.json').write_text(json.dumps(dict(
            passed=len(results)==len(cases) and all(r['passed'] for r in results),
            complete=len(results)==len(cases),cases=results),indent=2)+'\n')
    final = dict(passed=all(r['passed'] for r in results),complete=True,cases=results)
    (args.output/'result.json').write_text(json.dumps(final,indent=2)+'\n')
    return 0 if final['passed'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
