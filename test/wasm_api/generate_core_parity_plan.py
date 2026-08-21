#!/usr/bin/env python3
"""Derive the Core-Wasm parity checklist from the existing Component oracle.

This script deliberately does not invent a second test inventory. The current
`parity_guest` generator already encodes the known-good test cases and their
fixture inputs. We scan its generated probe functions, recover the semantic API
calls each test makes, and intersect those dependencies with executable Core
transport coverage.

Generated transports and reviewed handwritten transports remain distinct in the
report. They are merged only for the question "can Core execute this semantic
callout?"; neither source is allowed to impersonate the other.

No result is marked verified here. Verification requires actually running the
Core guest and comparing its normalized `WASM_API|...` records with the existing
Component oracle.
"""

from __future__ import annotations

import argparse
import json
import re
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable

ROOT = Path(__file__).resolve().parents[2]
PARITY_ROOT = ROOT / "test" / "wasm_api" / "parity_guest"
PROBE_SOURCE = PARITY_ROOT / "src" / "probe_generated.rs"
CORE_COVERAGE = ROOT / "rts" / "wasm" / "generated" / "core-executable-coverage.json"
REVIEWED_TRANSPORTS = ROOT / "test" / "wasm_api" / "core_reviewed_handwritten_transports.json"
DEFAULT_OUTPUT = ROOT / "test" / "wasm_api" / "core_parity_plan.json"

PROBE_START = re.compile(r"(?m)^fn probe_([a-zA-Z0-9_]+)\(")
CORE_CALL = re.compile(
    r"crate::bindings::recoil::spring_api::"
    r"(?P<module>r#[a-zA-Z0-9_]+|[a-zA-Z0-9_]+)::"
    r"(?P<function>r#[a-zA-Z0-9_]+|[a-zA-Z0-9_]+)\s*\("
)


@dataclass(frozen=True, order=True)
class Callout:
    module: str
    function: str

    @property
    def import_name(self) -> str:
        return self.function.replace("_", "-")

    @property
    def key(self) -> tuple[str, str]:
        return self.module, self.import_name

    def to_json(self) -> dict[str, str]:
        return {
            "module": self.module,
            "function": self.function,
            "import_name": self.import_name,
        }


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--probe-source", type=Path, default=PROBE_SOURCE)
    parser.add_argument("--coverage", type=Path, default=CORE_COVERAGE)
    parser.add_argument("--reviewed-transports", type=Path, default=REVIEWED_TRANSPORTS)
    parser.add_argument("--parity-root", type=Path, default=PARITY_ROOT)
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument(
        "--strict",
        action="store_true",
        help="fail when a manifest test has no recoverable semantic API call",
    )
    parser.add_argument(
        "--require-no-blocked-tests",
        action="store_true",
        help="fail when any existing oracle case depends on a non-executable Core callout",
    )
    parser.add_argument(
        "--require-all-executable-oracles",
        action="store_true",
        help="fail when an executable Core callout has no existing oracle case",
    )
    return parser.parse_args()


def load_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def normalize_ident(value: str) -> str:
    return value.removeprefix("r#")


def extract_probe_dependencies(source: str) -> dict[str, tuple[Callout, ...]]:
    starts = list(PROBE_START.finditer(source))
    dependencies: dict[str, tuple[Callout, ...]] = {}
    for index, match in enumerate(starts):
        end = starts[index + 1].start() if index + 1 < len(starts) else len(source)
        body = source[match.start():end]
        calls = {
            Callout(
                normalize_ident(call.group("module")),
                normalize_ident(call.group("function")),
            )
            for call in CORE_CALL.finditer(body)
        }
        dependencies[match.group(1)] = tuple(sorted(calls))
    return dependencies


def executable_callouts(
    coverage: dict,
    reviewed: dict,
) -> tuple[set[tuple[str, str]], dict[tuple[str, str], dict]]:
    keys: set[tuple[str, str]] = set()
    entries: dict[tuple[str, str], dict] = {}

    for entry in coverage.get("executable", []):
        normalized = dict(entry)
        # The generator reports reviewed handwritten bindings in the same
        # coverage list, tagged by class. Carry that distinction through rather
        # than letting a handwritten transport enter as "generated": the two
        # sources must never impersonate each other.
        if normalized.get("class") == "handwritten-reviewed":
            normalized.setdefault("source", "reviewed-handwritten")
        else:
            normalized.setdefault("source", "generated")
        key = (str(normalized["module"]), str(normalized["import_name"]))
        keys.add(key)
        entries[key] = normalized

    for entry in reviewed.get("transports", []):
        normalized = dict(entry)
        normalized["source"] = "reviewed-handwritten"
        key = (str(normalized["module"]), str(normalized["import_name"]))
        existing = entries.get(key)
        if existing is not None and existing.get("source") != "reviewed-handwritten":
            raise RuntimeError(
                f"Core transport {key[0]}.{key[1]} is declared by both generated "
                "and reviewed-handwritten inventories"
            )
        keys.add(key)
        entries[key] = normalized

    return keys, entries


def manifest_paths(root: Path) -> list[Path]:
    return sorted(root.glob("probe_manifest*.json"))


def iter_manifest_tests(root: Path) -> Iterable[tuple[str, str, str]]:
    """Yield (context, manifest filename, test name), deduplicated by context/test."""
    seen: set[tuple[str, str]] = set()
    for path in manifest_paths(root):
        manifest = load_json(path)
        context = str(manifest.get("context", path.stem.removeprefix("probe_manifest_")))
        for test in manifest.get("tests", []):
            key = (context, str(test))
            if key in seen:
                continue
            seen.add(key)
            yield context, path.name, str(test)


def missing_callout_report(
    blocked: dict[tuple[str, str], list[tuple[str, str]]],
    dependencies: dict[str, tuple[Callout, ...]],
) -> list[dict]:
    report: list[dict] = []
    for key, blocked_tests in blocked.items():
        function = None
        for _, test_name in blocked_tests:
            for call in dependencies.get(test_name, ()):
                if call.key == key:
                    function = call.function
                    break
            if function is not None:
                break
        contexts = sorted({context for context, _ in blocked_tests})
        tests = sorted({test for _, test in blocked_tests})
        report.append({
            "module": key[0],
            "function": function,
            "import_name": key[1],
            "blocked_test_count": len(blocked_tests),
            "contexts": contexts,
            "tests": tests,
        })
    report.sort(
        key=lambda item: (
            -int(item["blocked_test_count"]),
            str(item["module"]),
            str(item["import_name"]),
        )
    )
    return report


def main() -> int:
    args = parse_args()
    source = args.probe_source.read_text(encoding="utf-8")
    dependencies = extract_probe_dependencies(source)
    coverage = load_json(args.coverage)
    reviewed = load_json(args.reviewed_transports)
    executable, executable_entries = executable_callouts(coverage, reviewed)

    tests: list[dict] = []
    unmapped: list[dict] = []
    by_context: dict[str, dict[str, int]] = {}
    callouts_with_oracle: set[tuple[str, str]] = set()
    blocked_callouts: set[tuple[str, str]] = set()
    blocked_by_callout: dict[tuple[str, str], list[tuple[str, str]]] = {}

    for context, manifest_name, test_name in iter_manifest_tests(args.parity_root):
        calls = dependencies.get(test_name, ())
        missing = [call for call in calls if call.key not in executable]
        core_executable = bool(calls) and not missing
        if core_executable:
            callouts_with_oracle.update(call.key for call in calls)
        else:
            for call in missing:
                blocked_callouts.add(call.key)
                blocked_by_callout.setdefault(call.key, []).append((context, test_name))
        if not calls:
            unmapped.append({
                "context": context,
                "manifest": manifest_name,
                "test": test_name,
            })

        context_stats = by_context.setdefault(
            context,
            {"total": 0, "core_executable": 0, "blocked": 0, "unmapped": 0},
        )
        context_stats["total"] += 1
        if core_executable:
            context_stats["core_executable"] += 1
        else:
            context_stats["blocked"] += 1
        if not calls:
            context_stats["unmapped"] += 1

        tests.append({
            "context": context,
            "manifest": manifest_name,
            "test": test_name,
            "callouts": [call.to_json() for call in calls],
            "core_executable": core_executable,
            "missing_core_callouts": [call.to_json() for call in missing],
            "verified": False,
        })

    executable_without_oracle = []
    for key in sorted(executable - callouts_with_oracle):
        entry = executable_entries[key]
        executable_without_oracle.append({
            "module": key[0],
            "import_name": key[1],
            "function": entry.get("function"),
            "class": entry.get("class"),
            "source": entry.get("source"),
        })

    generated_count = sum(
        entry.get("source") == "generated" for entry in executable_entries.values()
    )
    reviewed_count = sum(
        entry.get("source") == "reviewed-handwritten" for entry in executable_entries.values()
    )
    missing_ranked = missing_callout_report(blocked_by_callout, dependencies)
    blocked_test_count = sum(not bool(test["core_executable"]) for test in tests)

    report = {
        "version": 3,
        "source": {
            "probe_source": str(args.probe_source.relative_to(ROOT)),
            "core_coverage": str(args.coverage.relative_to(ROOT)),
            "reviewed_handwritten_transports": str(args.reviewed_transports.relative_to(ROOT)),
            "manifest_glob": "test/wasm_api/parity_guest/probe_manifest*.json",
        },
        "summary": {
            "tests_total": len(tests),
            "tests_core_executable": sum(bool(test["core_executable"]) for test in tests),
            "tests_blocked": blocked_test_count,
            "tests_unmapped": len(unmapped),
            "core_executable_callouts": len(executable),
            "core_generated_callouts": generated_count,
            "core_reviewed_handwritten_callouts": reviewed_count,
            "core_executable_callouts_with_oracle": len(callouts_with_oracle),
            "core_executable_callouts_without_oracle": len(executable_without_oracle),
            "missing_callouts_blocking_oracle_tests": len(blocked_callouts),
        },
        "by_context": by_context,
        "missing_core_callouts_ranked": missing_ranked,
        "tests": tests,
        "unmapped_tests": unmapped,
        "core_executable_without_oracle_case": executable_without_oracle,
        "notes": [
            "core_executable means every semantic API call recovered from the existing oracle case has either generated or explicitly reviewed handwritten Core transport",
            "generated and reviewed-handwritten transport sources remain distinct in coverage metadata",
            "missing_core_callouts_ranked is ordered by the number of existing oracle cases each endpoint blocks and is the implementation-priority list",
            "verified remains false until a Core guest is run and its normalized WASM_API record is compared with the existing Component oracle",
            "an executable Core callout without an oracle case is not reviewed parity coverage and must receive a generated or explicit test",
        ],
    }

    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")

    failures: list[str] = []
    if args.strict and unmapped:
        names = ", ".join(item["test"] for item in unmapped[:20])
        failures.append(
            f"{len(unmapped)} parity tests have no recoverable semantic callout: {names}"
        )
    if args.require_no_blocked_tests and blocked_test_count:
        top = ", ".join(
            f"{item['module']}.{item['import_name']}({item['blocked_test_count']})"
            for item in missing_ranked[:20]
        )
        failures.append(
            f"{blocked_test_count} parity tests are blocked by "
            f"{len(blocked_callouts)} missing Core callouts: {top}"
        )
    if args.require_all_executable_oracles and executable_without_oracle:
        names = ", ".join(
            f"{item['module']}.{item['import_name']}"
            for item in executable_without_oracle[:20]
        )
        failures.append(
            f"{len(executable_without_oracle)} executable Core callouts have no oracle case: {names}"
        )

    print(json.dumps(report["summary"], indent=2))
    print(f"wrote {args.output}")
    if failures:
        raise RuntimeError("\n".join(failures))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
