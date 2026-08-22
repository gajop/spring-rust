#!/usr/bin/env python3
"""Focused end-to-end Wasmtime/Core engine benchmark.

Runs two otherwise identical synced guests:
  1. GameFrame -> one volatile guest-memory store.
  2. GameFrame -> one GetUnitDefID Core import -> one volatile store.

The difference is a useful engine-level scalar callout/round-trip estimate while
the absolute first row measures the Core host->guest callin path.
"""

from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
sys.path.insert(0, str(ROOT / "test" / "native_api_parity"))

import run_benchmarks as bench  # noqa: E402

BASE_CRATE_DIR = ROOT / "test" / "wasm_api" / "guests" / "core_benchmark_guest"
ROUNDTRIP_CRATE_DIR = ROOT / "test" / "wasm_api" / "guests" / "core_roundtrip_guest"
BASE_WASM = (
    BASE_CRATE_DIR
    / "target"
    / "wasm32-unknown-unknown"
    / "release"
    / "recoil_wasm_core_benchmark_guest.wasm"
)
ROUNDTRIP_WASM = (
    ROUNDTRIP_CRATE_DIR
    / "target"
    / "wasm32-unknown-unknown"
    / "release"
    / "recoil_wasm_core_roundtrip_guest.wasm"
)


def run(command: list[str], cwd: Path = ROOT) -> None:
    print("+", " ".join(command), flush=True)
    subprocess.run(command, cwd=cwd, check=True)


def build_crate(directory: Path, output: Path) -> None:
    # Running Cargo from the crate directory is intentional: it makes Cargo
    # read this guest's .cargo/config.toml, which passes wasm-ld
    # --no-growable-memory. The synced validator requires max == min.
    run(
        ["cargo", "build", "--target", "wasm32-unknown-unknown", "--release"],
        cwd=directory,
    )
    if not output.is_file():
        raise RuntimeError(f"Core benchmark guest was not produced: {output}")


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
    build_crate(BASE_CRATE_DIR, BASE_WASM)
    build_crate(ROUNDTRIP_CRATE_DIR, ROUNDTRIP_WASM)


def run_guest(args: argparse.Namespace, wasm: Path, output_name: str) -> dict:
    expected = ("callin_gameframe",) + bench.CALLIN_COMMON_TESTS
    rows = bench.run_backend(
        "wasm",
        args.output_root,
        args.seed,
        args.timeout,
        args.spring,
        True,
        args.scale,
        "callins",
        1_000_000,
        args.repeats,
        expected,
        callin_variant="gameframe",
        wasm_component=wasm,
        output_name=output_name,
        wasm_context="synced_gadget",
        load_native_module=False,
    )
    row = next((row for row in rows if row.get("test") == "callin_gameframe"), None)
    if row is None:
        raise RuntimeError(f"{output_name} produced no callin_gameframe row")
    return row


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
    for wasm in (BASE_WASM, ROUNDTRIP_WASM):
        if not wasm.is_file():
            raise RuntimeError(f"missing Core benchmark guest: {wasm}")

    bench.prepare_benchmark_context("synced_gadget")
    old_core = os.environ.get("SPRING_WASM_CORE_HOST")
    old_typed = os.environ.get("SPRING_WASM_TYPED_HOST")
    os.environ["SPRING_WASM_CORE_HOST"] = "1"
    os.environ.pop("SPRING_WASM_TYPED_HOST", None)
    try:
        baseline = run_guest(args, BASE_WASM, "wasm_core-gameframe")
        roundtrip = run_guest(args, ROUNDTRIP_WASM, "wasm_core-roundtrip")
    finally:
        if old_core is None:
            os.environ.pop("SPRING_WASM_CORE_HOST", None)
        else:
            os.environ["SPRING_WASM_CORE_HOST"] = old_core
        if old_typed is not None:
            os.environ["SPRING_WASM_TYPED_HOST"] = old_typed

    print("baseline:")
    print(json.dumps(baseline, indent=2, sort_keys=True))
    print("roundtrip:")
    print(json.dumps(roundtrip, indent=2, sort_keys=True))

    base_ns = float(baseline.get("medianNs", "nan"))
    roundtrip_ns = float(roundtrip.get("medianNs", "nan"))
    if base_ns == base_ns and roundtrip_ns == roundtrip_ns:
        print(f"Core Wasmtime GameFrame: {base_ns:.1f} ns")
        print(f"Core GameFrame + GetUnitDefID: {roundtrip_ns:.1f} ns")
        print(f"Incremental scalar callout: {roundtrip_ns - base_ns:.1f} ns")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
