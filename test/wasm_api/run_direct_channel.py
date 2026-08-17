#!/usr/bin/env python3
"""Exercise the synced-Wasm to unsynced-Wasm message channel in Spring."""

from __future__ import annotations

import argparse
import json
import shutil
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
HARNESS = ROOT / "test" / "native_api_parity"
sys.path.insert(0, str(HARNESS))

from run_harness import (  # noqa: E402
    GAME_FIXTURE,
    ROOT as HARNESS_ROOT,
    WASM_CONTEXT_MODULES,
    blank_map_name,
    prepare_datadir,
    run_spring,
    write_script,
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--spring",
        type=Path,
        default=HARNESS_ROOT / "build-amd64-linux" / "install" / "spring",
    )
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=HARNESS / "out" / "direct-channel",
    )
    parser.add_argument("--timeout", type=int, default=180)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    synced = WASM_CONTEXT_MODULES["synced_gadget"]
    unsynced = WASM_CONTEXT_MODULES["unsynced_gadget"]
    for path in (args.spring, synced, unsynced):
        if not path.is_file():
            raise SystemExit(f"required artifact is missing: {path}")

    output_dir = args.output_dir.resolve()
    output_dir.mkdir(parents=True, exist_ok=True)
    run_dir = output_dir / "latest"
    if run_dir.exists():
        shutil.rmtree(run_dir)
    run_dir.mkdir()

    harness_args = argparse.Namespace(
        blank_map_x=10,
        blank_map_y=8,
        blank_map_height=96,
        blank_map_seed=424242,
        cases=1,
        tests=None,
        test_prefix=None,
        enable_rendering_tests=False,
        process_test=None,
        timeout=args.timeout,
        spring=args.spring.resolve(),
        spring_headless=args.spring.resolve(),
    )

    with tempfile.TemporaryDirectory(prefix="wasm-direct-channel-") as temporary:
        workdir = Path(temporary)
        datadir, _ = prepare_datadir(workdir, None, unsynced, "unsynced_gadget")
        game = datadir / "games" / "native_api_parity.sdd"
        wasm_dir = game / "LuaRules" / "wasm"
        shutil.copy2(synced, wasm_dir / "synced.wasm")
        shutil.copy2(unsynced, wasm_dir / "unsynced.wasm")
        (wasm_dir / "manifest.txt").write_text(
            "module(synced, LuaRules/wasm/synced.wasm, rules-synced, 0, 1.0.0)\n"
            "module(unsynced, LuaRules/wasm/unsynced.wasm, rules-unsynced, 1, 1.0.0)\n",
            encoding="utf-8",
        )

        script = run_dir / "script.txt"
        write_script(
            script,
            blank_map_name(harness_args, "direct-channel"),
            "native_api_parity",
            "wasm",
            True,
            harness_args,
            424242,
            "unsynced_gadget",
            None,
            wasm_role="direct-channel",
        )
        exit_code = run_spring(
            harness_args,
            datadir,
            script,
            run_dir,
            "wasm",
            "unsynced_gadget",
        )
        if exit_code != 0:
            raise SystemExit(f"Spring exited with {exit_code}; see {run_dir / 'spring.log'}")

    stream = run_dir / "write-dir" / "native_api_parity" / "wasm.jsonl"
    rows = [
        json.loads(line)
        for line in stream.read_text(encoding="utf-8").splitlines()
        if line.strip()
    ] if stream.is_file() else []
    direct_rows = [row for row in rows if row.get("source") == "wasm-direct"]
    if len(direct_rows) != 1 or direct_rows[0].get("status") != "pass":
        raise SystemExit(
            f"direct channel check failed: expected one passing row, got {direct_rows!r}; "
            f"see {stream}"
        )
    print(json.dumps(direct_rows[0], sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
