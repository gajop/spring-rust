#!/usr/bin/env python3
# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
"""Check real synchronous/deferred job imports in a tiny isolated engine game.

Run under the caller's native-test lock. No Rust build, downloads, shared game
edits, retention cleanup or installed binary replacement is performed.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "test/native_api_parity"))
from run_harness import blank_map_name, prepare_datadir, write_script


def digest(path: Path) -> str:
    result = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            result.update(block)
    return result.hexdigest()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--spring", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--threads", default="1,4")
    options = parser.parse_args()
    widths = [int(value) for value in options.threads.split(",")]
    if not widths or len(set(widths)) != len(widths) or any(width < 1 or width > 32 for width in widths):
        raise SystemExit("--threads must contain distinct widths in 1..32")
    binary = options.spring.resolve(strict=True)
    output = options.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    source = ROOT / "test/wasm_api/data/synced_jobs/batch_in_game.wat"
    wasm = output / "batch.wasm"
    subprocess.run(["wat2wasm", str(source), "-o", str(wasm)], check=True)
    identity = {"engine": str(binary), "engine_sha256": digest(binary),
                "fixture": str(source), "fixture_sha256": digest(source),
                "wasm_sha256": digest(wasm), "threads": widths}
    (output / "identity.json").write_text(json.dumps(identity, indent=2) + "\n")
    results = []
    for width in widths:
        case = output / f"threads-{width}"
        case.mkdir()
        args = argparse.Namespace(blank_map_x=4, blank_map_y=4, blank_map_height=96,
            blank_map_seed=424242, cases=1, tests=None, test_prefix=None,
            enable_rendering_tests=False, process_test=None, timeout=60,
            spring=binary, spring_headless=binary)
        datadir, _ = prepare_datadir(case / "fixture", None, wasm)
        gadgets = datadir / "games/native_api_parity.sdd/LuaRules/Gadgets"
        if not gadgets.resolve().is_relative_to(datadir.resolve()):
            raise SystemExit(f"refusing to edit external fixture target: {gadgets}")
        for path in gadgets.glob("*.lua"):
            path.unlink()  # only the newly copied private fixture
        (gadgets / "batch_exit.lua").write_text('''function gadget:GetInfo()
    return {name = "Batch fixture exit", layer = 0, enabled = true}
end
if not gadgetHandler:IsSyncedCode() then
    function gadget:GameFrame(frame)
        if frame >= 3 then Spring.SendCommands("quitforce") end
    end
end
''')
        write = case / "write-dir"
        write.mkdir()
        config = write / "springsettings.cfg"
        config.write_text(f"WorkerThreadCount={width}\nWorkerThreadSpinTime=0\n")
        script = case / "script.txt"
        write_script(script, blank_map_name(args, "wasm"), "native_api_parity",
                     "wasm", True, args, 424242, "synced_gadget", None)
        script.write_text(script.read_text().replace("FixedRNGSeed=1;", "FixedRNGSeed=1;\n    FixedUnsyncedRNGSeed=1;"))
        env = os.environ.copy()
        env.update(SPRING_DATADIR=os.pathsep.join(map(str, (datadir, ROOT / "cont/base", ROOT / "cont"))),
                   SPRING_ISOLATED=str(datadir), SPRING_NATIVE_MODULE="", SPRING_WASM_CORE_HOST="1")
        command = [str(binary), "--nocolor", "--config", str(config), "--write-dir", str(write), str(script)]
        (case / "command.json").write_text(json.dumps(command, indent=2) + "\n")
        with (case / "spring.log").open("wb") as stream:
            code = subprocess.run(command, env=env, cwd=case, stdout=stream,
                                  stderr=subprocess.STDOUT, timeout=60).returncode
        log = (case / "spring.log").read_text(errors="replace")
        before_marker = log.split("WASM_BATCH_IMPORT_PASSED", 1)[0]
        width_proved = bool(re.search(rf"\[ThreadPool::SetThreadCount\]\[1\] wanted={width}\b", before_marker)
                            and re.search(rf"\[ThreadPool::SetThreadCount\]\[2\] workers={width - 1}\b", before_marker))
        passed = code == 0 and "WASM_BATCH_IMPORT_PASSED" in log and "Wasm trap" not in log and width_proved
        results.append({"threads": width, "exit": code, "width_proved": width_proved, "passed": passed})
        (output / "results.json").write_text(json.dumps(results, indent=2) + "\n")
        if not passed:
            raise SystemExit(f"batch import fixture failed at {width} threads; see {case / 'spring.log'}")
    print("PASS: ordered same-frame results, mixed guest errors, aliased buffers, preflight and F+1 coexistence")


if __name__ == "__main__":
    main()
