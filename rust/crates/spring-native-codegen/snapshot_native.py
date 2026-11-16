#!/usr/bin/env python3
# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
"""Capture the deterministic native Rust generator output for normal builds."""

from __future__ import annotations

import hashlib
import json
import argparse
import filecmp
import os
from pathlib import Path
import shutil
import subprocess
import sys


def regenerate(root: Path) -> Path | None:
    """Build spring-native with SPRING_NATIVE_REGENERATE=1 and return its OUT_DIR.

    Cargo reports the build script's OUT_DIR in its JSON messages. Searching
    target/debug/build instead broke when Cargo 1.100 moved build scripts to
    build/<package>/<hash>/out, and picking the newest of several directories
    could compare a stale earlier generation. Any cargo or rustc warning fails.
    """
    manifest = root / "rust" / "Cargo.toml"
    metadata = json.loads(
        subprocess.run(
            ["cargo", "metadata", "--manifest-path", str(manifest), "--no-deps",
             "--format-version", "1", "--locked"],
            check=True, stdout=subprocess.PIPE, text=True,
        ).stdout
    )
    package_id = next(p["id"] for p in metadata["packages"] if p["name"] == "spring-native")
    build = subprocess.run(
        ["cargo", "build", "--manifest-path", str(manifest), "-p", "spring-native", "--lib",
         "--locked", "--message-format=json", "--color", "never"],
        env=dict(os.environ, SPRING_NATIVE_REGENERATE="1"),
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
    )
    sys.stderr.write(build.stderr)
    warned = any(line.startswith("warning") for line in build.stderr.splitlines())
    out_dir = None
    for line in build.stdout.splitlines():
        message = json.loads(line)
        if message.get("reason") == "compiler-message":
            sys.stderr.write(message["message"].get("rendered") or "")
            warned |= message["message"].get("level") == "warning"
        elif message.get("reason") == "build-script-executed" and message["package_id"] == package_id:
            out_dir = Path(message["out_dir"])
    if build.returncode != 0:
        print("spring-native regeneration build failed", file=sys.stderr)
        return None
    if warned:
        print("spring-native regeneration build printed warnings (above); none are allowed",
              file=sys.stderr)
        return None
    if out_dir is None or not (out_dir / "units_query_generated.rs").exists():
        print(f"no native generator output from cargo (OUT_DIR {out_dir})", file=sys.stderr)
        return None
    return out_dir


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
    output = regenerate(root)
    if output is None:
        return 1
    snapshot = root / "rust" / "crates" / "spring-native" / "generated"
    # version.rs is read from Common.h on every build, so it is never snapshotted.
    source_files = [
        path
        for path in sorted(output.iterdir())
        if path.is_file() and path.name != "version.rs"
    ]

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
        # The OUT_DIR path is hashed per build; keep the checked-in manifest
        # deterministic and name where the files come from.
        "source_out": "OUT_DIR of spring-native's build script (SPRING_NATIVE_REGENERATE=1)",
        "files": files,
    }
    (snapshot / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    print(f"captured {len(files)} native generated files from {output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
