#!/usr/bin/env python3
"""Aggregate parity coverage from a normal run and isolated process runs."""

from __future__ import annotations

import argparse
from pathlib import Path

from run_harness import (
    coverage_summary,
    load_jsonl,
    pct,
    read_context_inventory,
    read_recorded_ids_by_context,
    read_surface_test_ids,
    report_link,
    result_names,
    write_coverage_details,
)


ROOT = Path(__file__).resolve().parents[2]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Merge coverage identities from a rendering run and isolated process-control runs."
    )
    parser.add_argument("--base-run", type=Path, required=True, help="normal rendering-enabled parity run")
    parser.add_argument(
        "--process-run",
        type=Path,
        action="append",
        required=True,
        help="isolated --process-test run; repeat once for each process API",
    )
    parser.add_argument(
        "--output",
        type=Path,
        help="aggregate report path (defaults to <base-run>/coverage_aggregate.md)",
    )
    return parser.parse_args()


def validate_run(run: Path) -> None:
    report = run / "report.md"
    if not report.is_file():
        raise SystemExit(f"missing parity report: {report}")
    if "- Result: PASS" not in report.read_text(encoding="utf-8"):
        raise SystemExit(f"parity run is not PASS: {report}")
    native = run / "native" / "native.jsonl"
    if not native.is_file():
        raise SystemExit(f"missing native result stream: {native}")


def main() -> int:
    args = parse_args()
    base_run = args.base_run.resolve()
    process_runs = [run.resolve() for run in args.process_run]
    validate_run(base_run)
    for run in process_runs:
        validate_run(run)

    all_checked_names: set[str] = set()
    all_surface_test_ids: set[str] = set()
    recorded_by_context: dict[str, set[str]] = {}
    for run in [base_run, *process_runs]:
        native_rows = load_jsonl(run / "native" / "native.jsonl")
        all_checked_names.update(result_names(native_rows))
        all_surface_test_ids.update(read_surface_test_ids(run))
        for context, test_ids in read_recorded_ids_by_context(run).items():
            recorded_by_context.setdefault(context, set()).update(test_ids)

    summary = coverage_summary(all_checked_names, all_surface_test_ids)
    inventory = read_context_inventory(base_run)
    details = write_coverage_details(base_run, summary, all_checked_names, inventory, recorded_by_context)
    aggregate_details = base_run / "coverage_aggregate_details.md"
    details.replace(aggregate_details)

    output = (args.output or base_run / "coverage_aggregate.md").resolve()
    lines = [
        "# Native API Parity Aggregate Coverage",
        "",
        "This report unions check identities from one normal rendering run and isolated process-control runs.",
        "Each source run was required to report `PASS`; result-stream equality is therefore preserved per run.",
        "",
        "## Source Runs",
        "",
        f"- Base rendering run: {report_link(base_run)}",
    ]
    lines.extend(f"- Isolated process run: {report_link(run)}" for run in process_runs)
    lines.extend(
        [
            "",
            "## Coverage",
            "",
            "| Surface | Total Functions | Tested Functions | Coverage | Unknown Tested Names |",
            "| --- | ---: | ---: | ---: | ---: |",
            (
                f"| Lua `Spring.*` | {len(summary['lua_total'])} | {len(summary['lua_tested_known'])} | "
                f"{pct(len(summary['lua_tested_known']), len(summary['lua_total']))} | "
                f"{len(summary['lua_tested_unknown'])} |"
            ),
            (
                f"| Native Rust labels | {len(summary['rust_total'])} | {len(summary['rust_tested_known'])} | "
                f"{pct(len(summary['rust_tested_known']), len(summary['rust_total']))} | "
                f"{len(summary['rust_tested_unknown'])} |"
            ),
            (
                f"| Native Rust counterparts | {len(summary['rust_counterpart'])} | "
                f"{len(summary['rust_counterpart_tested'])} | "
                f"{pct(len(summary['rust_counterpart_tested']), len(summary['rust_counterpart']))} | 0 |"
            ),
            (
                f"| Native-only Rust surfaces | {len(summary['rust_native_only'])} | "
                f"{len(summary['rust_native_only_tested'])} | "
                f"{pct(len(summary['rust_native_only_tested']), len(summary['rust_native_only']))} | 0 |"
            ),
            "",
            f"- Full details: {report_link(aggregate_details)}",
            "",
            "## Remaining Inventory",
            "",
            f"- Untested Lua callouts: {len(summary['lua_untested'])}",
            f"- Tested Lua names missing from the documentation inventory: {len(summary['lua_tested_unknown'])}",
            f"- Untested Rust functions (including native-only surfaces): {len(summary['rust_untested'])}",
            f"- Untested native counterparts: {len(summary['rust_counterpart_untested'])}",
            f"- Untested native-only surfaces: {len(summary['rust_native_only_untested'])}",
            f"- Tested Rust labels missing from the documentation inventory: {len(summary['rust_tested_unknown'])}",
        ]
    )
    output.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(output)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
