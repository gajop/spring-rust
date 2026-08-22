#!/usr/bin/env python3
"""Measure representative variable engine->guest callins in Lua and Core Wasm.

Inner transport timings remain diagnostics because their boundaries differ.
The outer event timings bracket the same CEventHandler boundary for Lua and
Core and are the decision comparison for these variable-shape callins.
"""

from __future__ import annotations

import argparse
from datetime import datetime
import json
import os
from pathlib import Path
import sys

import run_benchmarks as base
import run_benchmarks_core as core


ROOT = base.ROOT
INNER_TESTS = ("callin_string", "callin_command")
OUTER_TESTS = ("callin_string_event", "callin_command_event")
TESTS = (*INNER_TESTS, *OUTER_TESTS)
CORE_VARIABLE = core.CORE_RAW.with_name(
    "recoil_wasm_core_benchmark_suite_guest.variable-callins.wasm"
)
VARIABLE_COMPONENT_KEY = base.BENCHMARK_COMPONENT.with_name(
    "recoil_wasm_core_benchmark_suite_guest.variable-callins.wasm"
)
VARIABLE_CONTEXT = "unsynced_gadget"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Benchmark variable callin transport diagnostics in Lua and Core Wasm",
    )
    parser.add_argument("--spring-headless", type=Path, required=True)
    parser.add_argument("--output-root", type=Path, default=ROOT / "benchmark-output")
    parser.add_argument(
        "--results",
        type=Path,
        default=ROOT / "rts" / "wasm" / "docs" / "impl" / "benchmarking_variable_callins.md",
    )
    parser.add_argument("--summary-json", type=Path)
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--scale", type=float, default=1.0)
    parser.add_argument("--timeout", type=int, default=30)
    parser.add_argument("--iterations", type=int, default=1)
    parser.add_argument("--repeats", type=int, default=5)
    parser.add_argument("--skip-build", action="store_true")
    return parser.parse_args()


def build_core(scale: float, iterations: int, repeats: int) -> None:
    os.environ["SPRING_BENCHMARK_CASE"] = "callins"
    os.environ["SPRING_BENCHMARK_SCALE"] = str(scale)
    os.environ["SPRING_BENCHMARK_ITERATIONS"] = str(iterations)
    os.environ["SPRING_BENCHMARK_REPEATS"] = str(repeats)
    os.environ["SPRING_BENCHMARK_CALLIN_VARIANT"] = "variable"
    core.build_core_wasm(CORE_VARIABLE, VARIABLE_CONTEXT)


def run(args: argparse.Namespace) -> dict:
    if not 0.0 < args.scale <= 1.0:
        raise RuntimeError(f"--scale must be in (0, 1]: {args.scale}")
    if not args.spring_headless.is_file():
        raise RuntimeError(f"spring-headless not found: {args.spring_headless}")
    if not args.skip_build:
        build_core(args.scale, args.iterations, args.repeats)
    elif not CORE_VARIABLE.is_file():
        raise RuntimeError(f"--skip-build requires {CORE_VARIABLE}")

    run_root = args.output_root / datetime.now().strftime("variable-callins-%Y%m%d-%H%M%S")

    lua_rows = base.run_backend(
        "lua",
        run_root,
        args.seed,
        args.timeout,
        args.spring_headless,
        True,
        args.scale,
        "callins",
        args.iterations,
        args.repeats,
        TESTS,
        callin_variant="variable",
        wasm_component=base.BENCHMARK_COMPONENT_EMPTY,
        output_name="lua-variable-callins",
        wasm_context=VARIABLE_CONTEXT,
        load_native_module=False,
    )

    original_mapping = core.core_artifact_for_backend
    try:
        core.core_artifact_for_backend = lambda component: (
            CORE_VARIABLE
            if Path(component) == VARIABLE_COMPONENT_KEY
            else original_mapping(component)
        )
        core_rows = core._run_core_backend(
            run_root,
            args.seed,
            args.timeout,
            args.spring_headless,
            args.scale,
            "callins",
            args.iterations,
            args.repeats,
            TESTS,
            callin_variant="variable",
            wasm_component=VARIABLE_COMPONENT_KEY,
            output_name="wasm-core-variable-callins",
            wasm_context=VARIABLE_CONTEXT,
            wasm_module_count=1,
        )
    finally:
        core.core_artifact_for_backend = original_mapping

    return {
        "profile": "variable-callins",
        "scale": args.scale,
        "seed": args.seed,
        "context": VARIABLE_CONTEXT,
        "comparisonReady": True,
        "timingBoundaries": {
            "outer": "CEventHandler entry through complete event-client dispatch",
            "lua_inner": "guest call after Lua arguments have been pushed",
            "wasm_core_inner": "scratch serialization plus unchecked Core guest call",
        },
        "tests": list(TESTS),
        "rows": {"lua": lua_rows, "wasm_core": core_rows},
        "output": str(run_root),
    }


def render(summary: dict) -> str:
    indexed = {
        backend: {str(row["test"]): row for row in rows if row.get("test") != "complete"}
        for backend, rows in summary["rows"].items()
    }
    labels = {
        "callin_string": "engine->guest strings (`AddConsoleLine`)",
        "callin_command": "engine->guest fixed command + `f32[]` (`CommandNotify`)",
        "callin_string_event": "`AddConsoleLine`",
        "callin_command_event": "`CommandNotify`",
    }
    lines = [
        "<!-- Generated by test/native_api_parity/run_variable_callins_core.py. -->",
        "",
        "# Variable-callin transport benchmarks",
        "",
        "## Decision comparison: identical outer event boundary",
        "",
        "| Callin | Lua | Wasm Core | Lua / Core |",
        "| --- | ---: | ---: | ---: |",
    ]
    for test in OUTER_TESTS:
        lua = indexed["lua"][test]
        wasm = indexed["wasm_core"][test]
        lines.append(
            f"| {labels[test]} | {base.format_timed_metric(lua, test)} | "
            f"{base.format_timed_metric(wasm, test)} | "
            f"{base.ratio_for_test(test, lua, wasm)} |"
        )

    lines.extend([
        "",
        "Both columns above start immediately before `CEventHandler` dispatches the event "
        "client list and stop immediately after that dispatch completes. This includes "
        "backend-specific argument lowering and guest invocation on both paths.",
        "",
        "## Inner transport diagnostics",
        "",
        "| ABI shape | Lua inner timing | Wasm Core lowering + call |",
        "| --- | ---: | ---: |",
    ])
    for test in INNER_TESTS:
        lua = indexed["lua"][test]
        wasm = indexed["wasm_core"][test]
        lines.append(
            f"| {labels[test]} | {base.format_timed_metric(lua, test)} | "
            f"{base.format_timed_metric(wasm, test)} |"
        )
    lines.extend([
        "",
        "Do not compute a Lua/Core ratio from the inner table: Lua starts after argument "
        "pushing, while Core includes scratch serialization. The outer table is the "
        "like-for-like comparison.",
        "",
        "Core uses one guest-owned scratch buffer negotiated at module bind time; the "
        "steady-state path performs bounded serialization plus exactly one unchecked "
        "host->guest call, with no host heap allocation.",
        "",
    ])
    return "\n".join(lines)


def main() -> int:
    args = parse_args()
    summary = run(args)
    report = render(summary)
    args.results.parent.mkdir(parents=True, exist_ok=True)
    args.results.write_text(report, encoding="utf-8")
    if args.summary_json is not None:
        args.summary_json.parent.mkdir(parents=True, exist_ok=True)
        args.summary_json.write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")
    print(f"wrote {args.results}")
    print(f"raw output: {summary['output']}")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as error:
        print(f"variable-callin benchmark failed: {error}", file=sys.stderr)
        raise
