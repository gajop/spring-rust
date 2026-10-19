#!/usr/bin/env python3
# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
"""Create a local, engine-only replay kit for a machine without Rust or Git.

Only the public parity fixture/base content, test scripts and selected engine
are copied. This performs no upload, publication or release operation.
"""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
import shutil

ROOT = Path(__file__).resolve().parents[2]


def digest(path):
    h = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda:stream.read(1024*1024),b''):
            h.update(block)
    return h.hexdigest()


def build(corpus, output, engine=None):
    metadata = json.loads((corpus/'corpus.json').read_text())
    for name,checksum in metadata['files'].items():
        if Path(name).name != name or digest(corpus/name) != checksum:
            raise RuntimeError(f'invalid corpus member: {name}')
    output.mkdir() # Never overwrite an earlier kit or result.
    shutil.copytree(corpus,output/'corpus')
    # Deliberate allowlist; never take downstream game/workspace content.
    directories = ['test/native_api_parity/fixtures/game.sdd',
        'test/native_api_parity/api_tests', 'cont/base/springcontent',
        'cont/base/maphelper','cont/base/bitmaps','cont/base/cursors']
    files = ['LICENSE.html','test/synctest/wasm_adversarial.py',
        'test/synctest/wasm_sync_replay.py','test/native_api_parity/run_harness.py',
        'test/native_api_parity/spec_loader.py','test/native_api_parity/api_tests.json',
        'test/native_api_parity/known_issues.json','test/native_api_parity/surface_tests.json']
    for name in directories:
        shutil.copytree(ROOT/name,output/name,symlinks=False)
    for name in files:
        target = output/name
        target.parent.mkdir(parents=True,exist_ok=True)
        shutil.copy2(ROOT/name,target)
    provenance = corpus.parent/'provenance.json'
    if provenance.is_file():
        shutil.copy2(provenance,output/'recording-provenance.json')
    if engine:
        target = output/'engine'
        target.mkdir()
        for source in [engine,*engine.parent.glob('*.dll')]:
            shutil.copy2(source,target/source.name)
    (output/'README.txt').write_text(
        'Local Wasm replay test kit: public engine fixture only.\n'
        'Requires Python 3.10+ and a matching platform engine. No Rust, WAT compiler or Git needed.\n'
        'From this folder, run (Windows: replace python3 with py):\n'
        'python3 test/synctest/wasm_adversarial.py --spring engine/' +
        (engine.name if engine else 'spring-headless') +
        ' --corpus corpus --workers 1 4 8 --check-unlogged-replay --output results\n'
        'Use a new output directory for another attempt. Keep failed results.\n'
        'Nothing is uploaded. Copy results through your chosen private transport.\n')
    manifest = dict(schema=1,files={p.relative_to(output).as_posix():digest(p)
        for p in sorted(output.rglob('*')) if p.is_file()})
    (output/'bundle.json').write_text(json.dumps(manifest,indent=2)+'\n')
    return manifest


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--corpus',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    parser.add_argument('--spring',type=Path,help='Optional matching target engine; sibling DLLs included')
    args = parser.parse_args()
    build(args.corpus.resolve(),args.output.resolve(),args.spring.resolve() if args.spring else None)


if __name__ == '__main__':
    main()
