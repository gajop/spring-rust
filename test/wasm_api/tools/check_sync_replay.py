#!/usr/bin/env python3
"""Run one replay command repeatedly and compare its per-frame checksums."""

# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

from __future__ import annotations

import argparse
import json
import shutil
import subprocess
import tempfile
from pathlib import Path


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--runs", type=int, default=3)
    parser.add_argument(
        "--checksum-file",
        default="sync_checksums.jsonl",
        help="JSONL file written inside each {run_dir}; rows need frame and checksum",
    )
    parser.add_argument(
        "--keep",
        type=Path,
        help="copy run directories here after a failure or successful check",
    )
    parser.add_argument(
        "--command",
        nargs=argparse.REMAINDER,
        required=True,
        help="command and arguments; use {run_dir} for the isolated output directory",
    )
    return parser.parse_args()


def load_checksums(path: Path) -> list[tuple[int, str]]:
    if not path.is_file():
        raise RuntimeError(f"missing checksum stream: {path}")
    rows = []
    for line_number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        try:
            row = json.loads(line)
            frame = int(row["frame"])
            checksum = str(row["checksum"])
        except (ValueError, KeyError, TypeError, json.JSONDecodeError) as error:
            raise RuntimeError(f"invalid checksum row {path}:{line_number}: {error}") from error
        rows.append((frame, checksum))
    if not rows:
        raise RuntimeError(f"empty checksum stream: {path}")
    return rows


def main() -> int:
    args = parse_args()
    if args.runs < 2:
        raise SystemExit("--runs must be at least 2")
    if not args.command:
        raise SystemExit("--command is required")

    with tempfile.TemporaryDirectory(prefix="wasm-sync-replay-") as temporary:
        root = Path(temporary)
        expected = None
        for index in range(args.runs):
            run_dir = root / f"run-{index + 1:02d}"
            run_dir.mkdir()
            command = [argument.replace("{run_dir}", str(run_dir)) for argument in args.command]
            completed = subprocess.run(command, check=False)
            if completed.returncode != 0:
                raise RuntimeError(f"replay command failed on run {index + 1}: {completed.returncode}")
            checksums = load_checksums(run_dir / args.checksum_file)
            if expected is None:
                expected = checksums
            elif checksums != expected:
                raise RuntimeError(
                    f"per-frame checksum mismatch on run {index + 1}: "
                    f"expected {len(expected)} rows, got {len(checksums)}"
                )
        if args.keep:
            if args.keep.exists():
                shutil.rmtree(args.keep)
            shutil.copytree(root, args.keep)
    print(f"same-binary replay checksum gate passed: {args.runs} runs")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
