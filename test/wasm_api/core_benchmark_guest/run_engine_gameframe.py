#!/usr/bin/env python3
"""Build and run the focused Core-Wasm GameFrame engine benchmark.

This intentionally reports only callin_gameframe. The normal callin fixture also
emits a few shared rows; until their Core ABI exports exist they are not part of
this transport comparison.
"""

from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "test" / "native_api_parity"))

import run_benchmarks as bench  # noqa: E402

CRATE = ROOT / "test" / "wasm_api" / "core_benchmark_guest" / "Cargo.toml"
WASM = (
    ROOT
    / "test"
    / "wasm_api"
    / "core_benchmark_guest"
    / "target"
    / "wasm32-unknown-unknown"
    / "release"
    / "recoil_wasm_core_benchmark_guest.wasm"
)


def run(command: list[str]) -> None:
    print("+", " ".join(command), flush=True)
    subprocess.run(command, cwd=ROOT, check=True)


def build() -> None:
    installed = subprocess.run(
        ["rustup", "target", "list", "--installed"],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    ).stdout.splitlines()
    if "wasm32-unknown-unknown" not in installed:
        run(["rustup", "target", "add", "wasm32-unknown-unknown"])
    run([
        "cargo",
        "build",
        "--manifest-path",
        str(CRATE),
        "--target",
        "wasm32-unknown-unknown",
        "--release",
    ])
    if not WASM.is_file():
        raise RuntimeError(f"Core benchmark guest was not produced: {WASM}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--spring", type=Path, default=bench.ENGINE_INSTALL / "spring-headless")
    parser.add_argument(
        "--output-root",
        type=Path,
        default=ROOT / "test" / "native_api_parity" / "out" / "benchmark-core",
    )
    parser.add_argument("--seed", type=int, default=7)
    parser.add_argument("--timeout", type=int, default=180)
    parser.add_argument("--repeats", type=int, default=1)
    parser.add_argument("--scale", type=float, default=1.0)
    parser.add_argument("--skip-build", action="store_true")
    args = parser.parse_args()

    if not args.skip_build:
        build()
    if not WASM.is_file():
        raise RuntimeError(f"missing Core benchmark guest: {WASM}")

    bench.prepare_benchmark_context("synced_gadget")
    old_core = os.environ.get("SPRING_WASM_CORE_HOST")
    old_typed = os.environ.get("SPRING_WASM_TYPED_HOST")
    os.environ["SPRING_WASM_CORE_HOST"] = "1"
    os.environ.pop("SPRING_WASM_TYPED_HOST", None)
    try:
        # The fixture records the three common callins as well. They currently
        # fall through the legacy Core instance and are not used here; accepting
        # the rows lets us reuse the exact production benchmark launcher while
        # reporting only the implemented direct GameFrame path.
        expected = ("callin_gameframe",) + bench.CALLIN_COMMON_TESTS
        rows = bench.run_backend(
            "wasm",
            args.output_root,
            args.seed,
            args.timeout,
            args.spring,
            args.skip_build,
            args.scale,
            "callins",
            1_000_000,
            args.repeats,
            expected,
            callin_variant="gameframe",
            wasm_component=WASM,
            output_name="wasm_core-gameframe",
            wasm_context="synced_gadget",
            load_native_module=False,
        )
    finally:
        if old_core is None:
            os.environ.pop("SPRING_WASM_CORE_HOST", None)
        else:
            os.environ["SPRING_WASM_CORE_HOST"] = old_core
        if old_typed is not None:
            os.environ["SPRING_WASM_TYPED_HOST"] = old_typed

    row = next((row for row in rows if row.get("test") == "callin_gameframe"), None)
    if row is None:
        raise RuntimeError("Core benchmark produced no callin_gameframe row")
    print(json.dumps(row, indent=2, sort_keys=True))
    if "medianNs" in row:
        print(f"Core Wasmtime GameFrame: {float(row['medianNs']):.1f} ns")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
