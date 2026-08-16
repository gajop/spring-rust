#!/usr/bin/env python3
# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
"""Capture the deterministic native Rust generator output for normal builds."""

from __future__ import annotations

import hashlib
import json
import argparse
import filecmp
from pathlib import Path
import shutil
import sys


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[3])
    parser.add_argument(
        "--check",
        action="store_true",
        help="compare the freshly generated output without changing the snapshot",
    )
    args = parser.parse_args()
    root = args.root.resolve()
    target = root / "rust" / "target" / "debug" / "build"
    candidates = [
        path / "out"
        for path in target.glob("spring-native-*")
        if path.is_dir() and path.name != "spring-native-sys"
        and (path / "out" / "units_query_generated.rs").exists()
    ]
    if not candidates:
        print("no native generator output found; run cargo build first", file=sys.stderr)
        return 1
    # The build-script output directory's mtime is not a reliable generation
    # timestamp: Cargo may copy an older snapshot into a newer directory, and
    # multiple debug builds can leave several valid candidates behind.  Use
    # the newest generated file instead, which tracks the actual regeneration
    # event and keeps --check deterministic in a reused target directory.
    def generated_mtime(path: Path) -> int:
        return max(
            (entry.stat().st_mtime_ns for entry in path.iterdir() if entry.is_file()),
            default=path.stat().st_mtime_ns,
        )

    output = max(candidates, key=generated_mtime)
    snapshot = root / "rust" / "crates" / "spring-native" / "generated"
    source_files = [path for path in sorted(output.iterdir()) if path.is_file()]

    if args.check:
        expected = {
            path.name
            for path in snapshot.iterdir()
            if path.is_file() and path.name not in {"manifest.json", "README.md"}
        }
        actual = {path.name for path in source_files}
        differences = []
        for name in sorted(expected | actual):
            checked = snapshot / name
            fresh = output / name
            if not checked.exists():
                differences.append(f"missing snapshot file: {name}")
            elif not fresh.exists():
                differences.append(f"stale snapshot file: {name}")
            elif not filecmp.cmp(checked, fresh, shallow=False):
                differences.append(f"content drift: {name}")
        if differences:
            print("native generated snapshot is out of date:", file=sys.stderr)
            print("\n".join(differences), file=sys.stderr)
            return 1
        print(f"native generated snapshot is up to date ({len(actual)} files)")
        return 0

    snapshot.mkdir(parents=True, exist_ok=True)
    files = []
    for source in source_files:
        destination = snapshot / source.name
        shutil.copyfile(source, destination)
        digest = hashlib.sha256(destination.read_bytes()).hexdigest()
        files.append({"name": source.name, "sha256": digest, "bytes": destination.stat().st_size})

    manifest = {
        "format": "recoil.spring-native.snapshot.v1",
        "generator": "spring-api-codegen",
        # Cargo hashes the build directory; keep the checked-in manifest
        # deterministic while retaining the location pattern for diagnostics.
        "source_out": "rust/target/debug/build/spring-native-*/out",
        "files": files,
    }
    (snapshot / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    print(f"captured {len(files)} native generated files from {output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
