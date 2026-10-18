#!/usr/bin/env python3
# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
"""Run the existing Lua/native/Core scenarios through the unified Rust SDK.

The legacy runner remains available for before/after comparison. This runner
changes the Rust consumers, not the Lua baseline, workloads or timing checks.
"""
import os
import hashlib
from pathlib import Path
import shutil
import sys
from contextvars import ContextVar

import run_benchmarks_core as core
import sdk_benchmark_fixture

base = core.base
if "--module-ref" in sys.argv:
    if "--quick" not in sys.argv:
        raise SystemExit("--module-ref requires --quick; it measures identifier resolution per scalar call")
    sys.argv.remove("--module-ref")
    os.environ["SPRING_BENCHMARK_MODULE_REF"] = "1"
core.RUNNER_SCRIPT = Path(__file__).resolve()
CRATE = core.ROOT / "test/wasm_api/guests/sdk_benchmark_suite_guest"
core.CORE_CRATE = CRATE / "Cargo.toml"
core.CORE_RAW = CRATE / "target/wasm32-unknown-unknown/release/recoil_sdk_benchmark_suite_guest.wasm"
for name, variant in [("DEFAULT", "default"), ("EMPTY", "empty"), ("GAMEFRAME", "gameframe"),
                      ("UNIMPLEMENTED", "unimplemented"), ("UPDATE", "update"),
                      ("MEMORY", "memory"), ("DRAW", "draw")]:
    setattr(core, f"CORE_{name}", core.CORE_RAW.with_name(f"sdk.{variant}.wasm"))
base.NATIVE_CRATE = CRATE / "Cargo.toml"
base.NATIVE_SO = CRATE / "target/release/librecoil_sdk_benchmark_suite_guest.so"
if artifact := os.environ.get("RECOIL_SDK_BENCHMARK_NATIVE_ARTIFACT"):
    base.NATIVE_SO = Path(artifact).resolve(strict=True)
    # An explicitly selected historical SDK artifact must not be overwritten
    # by building today's SDK before its measurement.
    base.build_native = lambda: None
base.BENCHMARK_STAMP = CRATE / "target/benchmark-profile.json"
# Frozen legacy measurements must never fill gaps in an SDK report.
core.FROZEN_ROOT = Path(os.environ.get("RECOIL_BENCHMARK_FROZEN_ROOT", str(CRATE / "target/frozen")))

def build_core(destination: Path, context: str = "synced_gadget") -> None:
    env = os.environ.copy()
    env["SPRING_BENCHMARK_CONTEXT"] = context
    env["CARGO_PROFILE_RELEASE_PANIC"] = "abort"
    base.run_checked(["cargo", "build", "--manifest-path", str(core.CORE_CRATE),
                      "--target", "wasm32-unknown-unknown", "--release"], cwd=core.ROOT, env=env)
    destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(core.CORE_RAW, destination)

core.build_core_wasm = build_core
_prepare = base.prepare_datadir
_native_workload = ContextVar("native_sdk_workload", default=False)
_run_reference = core._ORIGINAL_RUN_BACKEND

def run_reference(backend, *args, **kwargs):
    token = _native_workload.set(backend == "native" and os.environ.get("SPRING_BENCHMARK_CASE") == "workloads")
    try:
        return _run_reference(backend, *args, **kwargs)
    finally:
        _native_workload.reset(token)

core._ORIGINAL_RUN_BACKEND = run_reference

def prepare(*args, **kwargs):
    result = _prepare(*args, **kwargs)
    if _native_workload.get():
        sdk_benchmark_fixture.configure(result[0] / "games/native_api_parity.sdd")
    return result

base.prepare_datadir = prepare

_make_summary = base.make_summary

def make_summary(*args, **kwargs):
    summary = _make_summary(*args, **kwargs)
    summary["rustSdk"] = "spring-sdk"
    summary["moduleRefPerCall"] = os.environ.get("SPRING_BENCHMARK_MODULE_REF") == "1"
    summary["nativeArtifactOverride"] = os.environ.get("RECOIL_SDK_BENCHMARK_NATIVE_ARTIFACT")
    summary["nativeArtifactSha256"] = hashlib.sha256(base.NATIVE_SO.read_bytes()).hexdigest()
    return summary

base.make_summary = make_summary

if __name__ == "__main__":
    raise SystemExit(base.main())
