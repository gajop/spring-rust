#!/usr/bin/env python3
"""Gate high-value Core command parity against the existing Component oracle.

This intentionally reuses `parity_guest/src/probe_generated.rs`; it does not
invent alternate expected values. Covered scenarios must have every recovered
semantic callout executable through Core. Command endpoints without an existing
oracle case are reported separately so executable transport cannot be mistaken
for verified semantic parity.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from generate_core_parity_plan import (
    CORE_COVERAGE,
    PROBE_SOURCE,
    REVIEWED_TRANSPORTS,
    executable_callouts,
    extract_probe_dependencies,
    load_json,
)

ROOT = Path(__file__).resolve().parents[2]

REQUIRED_ORACLE_TESTS = (
    "unit_cmd_desc_lifecycle",
    "give_order_array_to_unit_synced",
    "give_order_array_to_unit_map_synced",
    "give_order_array_to_unit_array_synced_pairwise",
    "give_order_array_to_unit_array_synced_broadcast",
    "get_unit_commands",
    "get_factory_commands",
    "get_command_queue",
    "get_factory_counts",
)

# These are important nested-command endpoints whose transport must exist even
# before a matching Component oracle case is added.
REQUIRED_EXECUTABLE_ENDPOINTS = (
    ("units_commands", "get-command-params"),
    ("units_commands", "give-order-array-to-unit-map"),
    ("unit_control", "insert-unit-cmd-desc"),
    ("unit_control", "edit-unit-cmd-desc"),
    ("unit_control", "give-order-array-to-unit"),
    ("unit_control", "give-order-array-to-unit-array"),
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--probe-source", type=Path, default=PROBE_SOURCE)
    parser.add_argument("--coverage", type=Path, default=CORE_COVERAGE)
    parser.add_argument("--reviewed-transports", type=Path, default=REVIEWED_TRANSPORTS)
    parser.add_argument(
        "--require-endpoint-oracles",
        action="store_true",
        help="also fail when a required executable endpoint has no recovered oracle case",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    dependencies = extract_probe_dependencies(args.probe_source.read_text(encoding="utf-8"))
    executable, entries = executable_callouts(
        load_json(args.coverage), load_json(args.reviewed_transports)
    )

    failures: list[str] = []
    command_oracle_keys: set[tuple[str, str]] = set()
    oracle_report: list[dict] = []

    for test in REQUIRED_ORACLE_TESTS:
        calls = dependencies.get(test, ())
        missing = [call.key for call in calls if call.key not in executable]
        command_oracle_keys.update(call.key for call in calls)
        oracle_report.append(
            {
                "test": test,
                "callouts": [list(call.key) for call in calls],
                "missing": [list(key) for key in missing],
                "core_executable": bool(calls) and not missing,
            }
        )
        if not calls:
            failures.append(f"oracle test {test} has no recovered semantic callout")
        elif missing:
            failures.append(
                f"oracle test {test} is blocked by "
                + ", ".join(f"{module}.{name}" for module, name in missing)
            )

    endpoint_report: list[dict] = []
    for key in REQUIRED_EXECUTABLE_ENDPOINTS:
        executable_now = key in executable
        has_oracle = key in command_oracle_keys
        entry = entries.get(key)
        endpoint_report.append(
            {
                "module": key[0],
                "import_name": key[1],
                "class": entry.get("class") if entry else None,
                "core_executable": executable_now,
                "has_required_oracle": has_oracle,
            }
        )
        if not executable_now:
            failures.append(f"required command endpoint {key[0]}.{key[1]} is not Core executable")
        if args.require_endpoint_oracles and not has_oracle:
            failures.append(f"required command endpoint {key[0]}.{key[1]} has no command oracle case")

    report = {
        "oracle_tests": oracle_report,
        "required_endpoints": endpoint_report,
        "summary": {
            "oracle_tests": len(oracle_report),
            "oracle_tests_core_executable": sum(item["core_executable"] for item in oracle_report),
            "required_endpoints": len(endpoint_report),
            "required_endpoints_core_executable": sum(item["core_executable"] for item in endpoint_report),
            "required_endpoints_with_oracle": sum(item["has_required_oracle"] for item in endpoint_report),
        },
    }
    print(json.dumps(report, indent=2))
    if failures:
        raise RuntimeError("\n".join(failures))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
