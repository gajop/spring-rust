#!/usr/bin/env python3
"""Run the established Lua/native/Component benchmark suite with Core Wasm.

This deliberately layers on run_benchmarks.py instead of duplicating its game
fixture, process orchestration, validation, or measurement semantics.  The only
new axis is the raw Core-Wasm guest/host transport and its report column.
"""

from __future__ import annotations

from datetime import datetime
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

import run_benchmarks as base


ROOT = base.ROOT
CORE_CRATE = ROOT / "test" / "wasm_api" / "core_benchmark_suite_guest" / "Cargo.toml"
CORE_RAW = (
    ROOT
    / "test"
    / "wasm_api"
    / "core_benchmark_suite_guest"
    / "target"
    / "wasm32-unknown-unknown"
    / "release"
    / "recoil_wasm_core_benchmark_suite_guest.wasm"
)
CORE_DEFAULT = CORE_RAW
CORE_EMPTY = CORE_RAW.with_name("recoil_wasm_core_benchmark_suite_guest.empty.wasm")
CORE_GAMEFRAME = CORE_RAW.with_name("recoil_wasm_core_benchmark_suite_guest.gameframe.wasm")
CORE_UNIMPLEMENTED = CORE_RAW.with_name("recoil_wasm_core_benchmark_suite_guest.unimplemented.wasm")
CORE_UPDATE = CORE_RAW.with_name("recoil_wasm_core_benchmark_suite_guest.update.wasm")
CORE_MEMORY = CORE_RAW.with_name("recoil_wasm_core_benchmark_suite_guest.memory.wasm")
CORE_DRAW = CORE_RAW.with_name("recoil_wasm_core_benchmark_suite_guest.draw.wasm")

# Preserve the existing backend ordering and append Core so historical columns
# do not move when a regenerated table is diffed.
base.BACKENDS = (*base.BACKENDS, "wasm_core")

_ORIGINAL_BUILD_WASM = base.build_wasm
_ORIGINAL_RUN_BACKEND = base.run_backend


def core_artifact_for_component(component: Path) -> Path:
    mapping = {
        base.BENCHMARK_COMPONENT: CORE_DEFAULT,
        base.BENCHMARK_COMPONENT_EMPTY: CORE_EMPTY,
        base.BENCHMARK_COMPONENT_GAMEFRAME: CORE_GAMEFRAME,
        base.BENCHMARK_COMPONENT_UNIMPLEMENTED: CORE_UNIMPLEMENTED,
        base.BENCHMARK_COMPONENT_UPDATE: CORE_UPDATE,
        base.BENCHMARK_COMPONENT_MEMORY: CORE_MEMORY,
        base.BENCHMARK_COMPONENT_DRAW: CORE_DRAW,
    }
    try:
        return mapping[Path(component)]
    except KeyError as error:
        raise RuntimeError(f"no Core benchmark artifact corresponds to {component}") from error


def build_core_wasm(destination: Path, context: str = "synced_gadget") -> None:
    build_env = os.environ.copy()
    build_env["SPRING_BENCHMARK_CONTEXT"] = context
    base.run_checked(
        [
            "cargo",
            "build",
            "--manifest-path",
            str(CORE_CRATE),
            "--target",
            "wasm32-unknown-unknown",
            "--release",
        ],
        cwd=ROOT,
        env=build_env,
    )
    if not CORE_RAW.is_file():
        raise RuntimeError(f"Core Wasm benchmark module was not produced: {CORE_RAW}")
    destination.parent.mkdir(parents=True, exist_ok=True)
    if destination != CORE_RAW:
        shutil.copy2(CORE_RAW, destination)
    if not destination.is_file():
        raise RuntimeError(f"Core Wasm benchmark variant was not produced: {destination}")


def build_wasm_and_core(
    component: Path = base.BENCHMARK_COMPONENT,
    context: str = "synced_gadget",
) -> None:
    # Build the Component artifact exactly as the established runner does, then
    # build a raw Core module under the same compile-time benchmark profile.
    _ORIGINAL_BUILD_WASM(component, context)
    build_core_wasm(core_artifact_for_component(component), context)


def run_backend_with_core(
    backend: str,
    root_output: Path,
    seed: int,
    timeout: int,
    spring: Path,
    skip_build: bool,
    scale: float,
    benchmark_case: str,
    benchmark_iterations: int,
    benchmark_repeats: int,
    expected_tests: tuple[str, ...],
    callin_variant: str = "empty",
    wasm_component: Path = base.BENCHMARK_COMPONENT,
    output_name: str | None = None,
    wasm_context: str = "synced_gadget",
    wasm_module_count: int = 1,
    load_native_module: bool = True,
) -> list[dict]:
    selected_module = wasm_component
    if backend == "wasm_core":
        selected_module = core_artifact_for_component(wasm_component)
        if not selected_module.is_file():
            raise RuntimeError(
                f"Core Wasm artifact is missing: {selected_module}; rerun without --skip-build"
            )

    previous_core_host = os.environ.get("SPRING_WASM_CORE_HOST")
    try:
        if backend == "wasm_core":
            os.environ["SPRING_WASM_CORE_HOST"] = "1"
        else:
            os.environ.pop("SPRING_WASM_CORE_HOST", None)
        return _ORIGINAL_RUN_BACKEND(
            backend,
            root_output,
            seed,
            timeout,
            spring,
            skip_build,
            scale,
            benchmark_case,
            benchmark_iterations,
            benchmark_repeats,
            expected_tests,
            callin_variant=callin_variant,
            wasm_component=selected_module,
            output_name=output_name,
            wasm_context=wasm_context,
            wasm_module_count=wasm_module_count,
            load_native_module=load_native_module,
        )
    finally:
        if previous_core_host is None:
            os.environ.pop("SPRING_WASM_CORE_HOST", None)
        else:
            os.environ["SPRING_WASM_CORE_HOST"] = previous_core_host


def render_report(summaries: list[dict]) -> str:
    lines = [
        "<!-- This file is generated by test/native_api_parity/run_benchmarks_core.py. -->",
        "",
        "| Profile | Scale | Test | Lua | Native | Wasm (C API, dynamic, CM) | "
        "Wasm (Rust, typed, CM) | Wasm (C API, unchecked, Core) | "
        "Lua vs native | Lua vs typed | Typed vs native | Dynamic vs typed | "
        "Core vs native | Typed vs Core |",
        "| --- | ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |",
    ]
    for summary in summaries:
        profile = str(summary["profile"])
        indexed = {
            backend: {
                str(row["test"]): row
                for row in rows
                if row.get("test") != "complete"
            }
            for backend, rows in summary["rows"].items()
        }
        for test in (str(test) for test in summary["tests"]):
            rows = {backend: indexed[backend][test] for backend in base.BACKENDS}
            if profile == "memory":
                values = [base.format_memory_metric(rows[backend], test) for backend in base.BACKENDS]
            else:
                values = [base.format_timed_metric(rows[backend], test) for backend in base.BACKENDS]
            lines.append(
                f"| `{profile}` | {float(summary['scale']):g} | `{test}` | {values[0]} | "
                f"{values[1]} | {values[2]} | {values[3]} | {values[4]} | "
                f"{base.ratio_for_test(test, rows['lua'], rows['native'])} | "
                f"{base.ratio_for_test(test, rows['lua'], rows['wasm_rust_typed'])} | "
                f"{base.ratio_for_test(test, rows['wasm_rust_typed'], rows['native'])} | "
                f"{base.ratio_for_test(test, rows['wasm'], rows['wasm_rust_typed'])} | "
                f"{base.ratio_for_test(test, rows['wasm_core'], rows['native'])} | "
                f"{base.ratio_for_test(test, rows['wasm_rust_typed'], rows['wasm_core'])} |"
            )
    lines.append("")
    return "\n".join(lines)


def validate_report_shape(report: str) -> None:
    lines = report.splitlines()
    if len(lines) < 4:
        raise RuntimeError("benchmark report is missing its table")
    if not lines[0].startswith("<!-- This file is generated by "):
        raise RuntimeError("benchmark report is missing its generated marker")
    if lines[1] != "":
        raise RuntimeError("benchmark report must separate its marker from the table")
    if not lines[2].startswith("| Profile | Scale | Test |"):
        raise RuntimeError("benchmark report has an unexpected table header")
    expected_separator = (
        "| --- | ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | "
        "---: | ---: | ---: | ---: |"
    )
    if not lines[3].startswith(expected_separator):
        raise RuntimeError("benchmark report has an unexpected Core-aware table separator")
    if any(line and not line.startswith("|") for line in lines[4:]):
        raise RuntimeError("benchmark report may contain only generated table rows")


def run_suite(args) -> int:
    if args.skip_build:
        raise RuntimeError("--suite requires builds; use an individual profile with --skip-build")
    if any(
        getattr(args, name)
        for name in ("quick", "callouts", "heightmap", "workloads", "callins", "memory", "draw")
    ):
        raise RuntimeError("--suite cannot be combined with an individual benchmark profile")
    if args.scale is not None and not 0.0 < args.scale <= 1.0:
        raise RuntimeError(f"--scale must be greater than 0 and no greater than 1: {args.scale}")

    suite_root = args.output_root / f"suite-{datetime.now().strftime('%Y%m%d-%H%M%S-%f')}"
    summaries = []
    profiles = base.BOUNDED_SUITE_PROFILES if args.bounded_suite else base.SUITE_PROFILES
    for profile, flag, default_scale, default_timeout in profiles:
        scale = args.scale if args.scale is not None else default_scale
        timeout = args.timeout if args.timeout is not None else default_timeout
        summary_path = suite_root / f"{profile}.json"
        profile_output = suite_root / profile
        command = [
            sys.executable,
            str(Path(__file__).resolve()),
            flag,
            "--seed",
            str(args.seed),
            "--scale",
            str(scale),
            "--timeout",
            str(timeout),
            "--spring-headless",
            str(args.spring_headless),
            "--spring",
            str(args.spring),
            "--output-root",
            str(profile_output),
            "--summary-json",
            str(summary_path),
            "--no-report",
        ]
        print(f"[suite+core] {profile}: scale={scale:g}, timeout={timeout}s", flush=True)
        subprocess.run(command, cwd=ROOT, check=True)
        if not summary_path.is_file():
            raise RuntimeError(f"benchmark profile did not write its summary: {summary_path}")
        summaries.append(json.loads(summary_path.read_text(encoding="utf-8")))

    base.write_results(args.results, summaries)
    print(f"wrote {args.results}", flush=True)
    print(f"raw suite output: {suite_root}", flush=True)
    return 0


# Patch only the extension seams; base.main continues to own CLI parsing,
# profile selection, fixture generation, process execution and row validation.
base.build_wasm = build_wasm_and_core
base.run_backend = run_backend_with_core
base.render_report = render_report
base.validate_report_shape = validate_report_shape
base.run_suite = run_suite


if __name__ == "__main__":
    try:
        raise SystemExit(base.main())
    except Exception as error:
        print(f"benchmark+core failed: {error}", file=sys.stderr)
        raise
