#!/usr/bin/env python3
"""Generate instantiate-only Core-Wasm ABI surface modules.

This is deliberately separate from semantic parity. The generated modules
import every executable Core callout and export every executable Core callin
spelling (canonical names and aliases) in each environment, so module
validation/link/binding exercises the complete advertised ABI surface without
invoking mutating engine APIs.

Inputs are generated metadata; outputs are disposable test artifacts. Nothing
under rts/wasm/generated is modified by this script.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any, Iterable

ROOT = Path(__file__).resolve().parents[3]
GENERATED = ROOT / "rts" / "wasm" / "generated"
DEFAULT_OUTPUT = ROOT / "test" / "wasm_api" / "data" / "core_surface_generated"

ENVIRONMENTS = {
    "rules-synced": 1,
    "rules-unsynced": 2,
    "gaia-synced": 4,
    "gaia-unsynced": 8,
    "ui": 16,
}

SPECIALIZED_CALLINS = {
    "AddConsoleLine": ("i32", "i64"),
    "AllowUnitCreation": (
        "i32,i32,i32,i32,f32,f32,f32,i32",
        "i32",
    ),
    "CommandNotify": ("i32", "i64"),
    "DrawWorld": ("", ""),
    "GameFrame": ("i32", ""),
    "GameFramePost": ("i32", ""),
    "UnitCreated": ("i32,i32,i32,i32", ""),
    "UnitPreDamaged": (
        "i32,i32,i32,f32,i32,i32,i32,i32,i32,i32",
        "i64",
    ),
    "Update": ("f32", ""),
}

SCRATCH_CALLIN_CLASSES = {
    "generated-shared-scratch",
    "generated-shared-scratch-packed-result",
    "generated-shared-scratch-ignored-result",
}


def load_json(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as stream:
        return json.load(stream)


def rust_kebab(value: str) -> str:
    # Mirrors heck::ToKebabCase for the NativeInterface spellings used here.
    out: list[str] = []
    for index, char in enumerate(value):
        if char == "_":
            out.append("-")
            continue
        if char.isupper() and index and value[index - 1] != "_":
            previous = value[index - 1]
            next_char = value[index + 1] if index + 1 < len(value) else ""
            if previous.islower() or previous.isdigit() or next_char.islower():
                out.append("-")
        out.append(char.lower())
    return "".join(out)


def core_type(semantic: dict[str, Any]) -> str | None:
    kind = semantic.get("kind")
    if kind == "scalar":
        name = semantic["name"]
        if name == "f32":
            return "f32"
        if name == "f64":
            return "f64"
        if name in {"i64", "u64", "isize", "usize"}:
            return "i64"
        return "i32"
    if kind == "enum":
        return "i32"
    if kind == "handle":
        return "i64"
    return None


def packed32_type(semantic: dict[str, Any]) -> bool:
    kind = semantic.get("kind")
    if kind == "enum":
        return True
    if kind != "scalar":
        return False
    return semantic.get("name") in {
        "bool",
        "i8",
        "i16",
        "i32",
        "u8",
        "char",
        "u16",
        "u32",
        "f32",
    }


def record_index(model: dict[str, Any]) -> dict[str, dict[str, Any]]:
    records: dict[str, dict[str, Any]] = {}
    for module in model.get("modules", []):
        for record in module.get("records", []):
            records.setdefault(record["name"], record)
    return records


def callin_alias_index(model: dict[str, Any]) -> dict[str, list[str]]:
    result: dict[str, list[str]] = {}
    seen_spellings: set[str] = set()
    for callin in model.get("callins", []):
        canonical = str(callin["name"])
        aliases = [str(alias) for alias in callin.get("aliases", [])]
        for spelling in [canonical, *aliases]:
            if spelling in seen_spellings:
                raise ValueError(f"duplicate generated callin spelling {spelling!r}")
            seen_spellings.add(spelling)
        result[canonical] = aliases
    return result


def flatten_type(
    semantic: dict[str, Any], records: dict[str, dict[str, Any]]
) -> list[str] | None:
    direct = core_type(semantic)
    if direct is not None:
        return [direct]
    kind = semantic.get("kind")
    if kind == "record":
        record = records.get(semantic["name"])
        if record is None:
            return None
        result: list[str] = []
        for field in record.get("fields", []):
            values = flatten_type(field["type"], records)
            if values is None:
                return None
            result.extend(values)
        return result
    if kind == "fixed-array":
        values = flatten_type(semantic["element"], records)
        if values is None:
            return None
        return values * int(semantic["length"])
    return None


def flatten_record(
    name: str, records: dict[str, dict[str, Any]]
) -> list[str] | None:
    record = records.get(name)
    if record is None:
        return None
    result: list[str] = []
    for field in record.get("fields", []):
        values = flatten_type(field["type"], records)
        if values is None:
            return None
        result.extend(values)
    return result


def direct_result_type(
    name: str, records: dict[str, dict[str, Any]]
) -> str | None:
    record = records.get(name)
    if record is None or len(record.get("fields", [])) != 1:
        return None
    return core_type(record["fields"][0]["type"])


def fixed_result_packable(
    name: str, records: dict[str, dict[str, Any]]
) -> bool:
    record = records.get(name)
    if record is None:
        return False

    leaves: list[dict[str, Any]] = []

    def visit(semantic: dict[str, Any]) -> bool:
        if packed32_type(semantic):
            leaves.append(semantic)
            return True
        kind = semantic.get("kind")
        if kind == "record":
            nested = records.get(semantic["name"])
            return nested is not None and all(
                visit(field["type"]) for field in nested["fields"]
            )
        if kind == "fixed-array":
            return all(
                visit(semantic["element"])
                for _ in range(int(semantic["length"]))
            )
        return False

    return (
        all(visit(field["type"]) for field in record.get("fields", []))
        and 0 < len(leaves) <= 2
    )


def callin_signature(
    entry: dict[str, Any],
    plan: dict[str, Any],
    records: dict[str, dict[str, Any]],
) -> tuple[list[str], list[str]]:
    name = entry["name"]
    if name in SPECIALIZED_CALLINS:
        params, result = SPECIALIZED_CALLINS[name]
        return split_signature(params), split_signature(result)

    klass = entry.get("class")
    if klass in SCRATCH_CALLIN_CLASSES:
        if klass == "generated-shared-scratch-ignored-result":
            return ["i32"], []
        if klass == "generated-shared-scratch-packed-result":
            if not fixed_result_packable(entry["result"], records):
                raise ValueError(f"{name}: packed scratch result is not <=2 32-bit leaves")
            return ["i32"], ["i64"]
        result = direct_result_type(entry["result"], records)
        return ["i32"], ([] if result is None else [result])

    params = flatten_record(entry["query"], records)
    if params is None:
        raise ValueError(
            f"{name}: coverage says numeric-executable but query is not flattenable"
        )

    if entry.get("aggregation") == "ignore":
        return params, []

    if klass == "generated-first-non-empty-string":
        # The generated guest returns one packed i64: low 32 bits are the
        # guest-memory pointer and high 32 bits are the byte length. The host
        # validates and copies that range immediately.
        return params, ["i64"]

    result_strategy = plan["result_strategy"]
    if result_strategy == "empty":
        results: list[str] = []
    elif result_strategy == "direct":
        result = direct_result_type(entry["result"], records)
        if result is None:
            raise ValueError(f"{name}: direct result record is not one scalar")
        results = [result]
    elif (
        result_strategy == "fixed-wire"
        and fixed_result_packable(entry["result"], records)
    ):
        results = ["i64"]
    else:
        raise ValueError(
            f"{name}: executable coverage class {klass!r} has no surface signature rule"
        )
    return params, results


def split_signature(value: str) -> list[str]:
    return [part for part in value.split(",") if part]


def wat_params(types: Iterable[str]) -> str:
    values = list(types)
    return "" if not values else " (param " + " ".join(values) + ")"


def wat_results(types: Iterable[str]) -> str:
    values = list(types)
    return "" if not values else " (result " + " ".join(values) + ")"


def zero_body(results: list[str]) -> str:
    instructions = []
    for result in results:
        if result == "i32":
            instructions.append("    i32.const 0")
        elif result == "i64":
            instructions.append("    i64.const 0")
        elif result == "f32":
            instructions.append("    f32.const 0")
        elif result == "f64":
            instructions.append("    f64.const 0")
        else:
            raise ValueError(f"unsupported Core result type {result}")
    return "\n".join(instructions)


def callout_plan_index(
    core_abi: dict[str, Any],
) -> dict[tuple[str, str], dict[str, Any]]:
    return {
        (entry["module"], entry["function"]): entry
        for entry in core_abi.get("functions", [])
    }


def callin_plan_index(callin_plan: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {entry["name"]: entry for entry in callin_plan.get("callins", [])}


def environment_mask_for_callin(entry: dict[str, Any]) -> int:
    mask = 0
    for environment in entry.get("environments", []):
        mask |= ENVIRONMENTS[environment]
    return mask


def emit_callin_export(
    lines: list[str], spelling: str, params: list[str], results: list[str]
) -> None:
    export_name = "spring:callin/" + rust_kebab(spelling)
    lines.append(
        f'  (func (export "{export_name}"){wat_params(params)}{wat_results(results)}'
    )
    body = zero_body(results)
    if body:
        lines.append(body)
    lines.append("  )")


def generate_module(
    environment: str,
    callout_coverage: dict[str, Any],
    callout_plans: dict[tuple[str, str], dict[str, Any]],
    callin_coverage: dict[str, Any],
    callin_plans: dict[str, dict[str, Any]],
    callin_aliases: dict[str, list[str]],
    records: dict[str, dict[str, Any]],
) -> tuple[str, dict[str, Any]]:
    bit = ENVIRONMENTS[environment]
    lines = [
        ";; @generated by test/wasm_api/tools/generate_core_abi_surface.py; do not edit.",
        f";; Instantiate-only complete Core ABI surface for {environment}.",
        "(module",
        '  (memory (export "memory") 2)',
    ]

    imported: list[dict[str, Any]] = []
    for entry in callout_coverage.get("executable", []):
        plan = callout_plans.get((entry["module"], entry["function"]))
        if plan is None or not (int(plan["environment_mask"]) & bit):
            continue
        params, results = parse_core_signature(plan["signature"])
        lines.append(
            f'  (import "{plan["import_module"]}" "{plan["import_name"]}" '
            f'(func $callout_{len(imported)}{wat_params(params)}{wat_results(results)}))'
        )
        imported.append(
            {
                "module": entry["module"],
                "function": entry["function"],
                "class": entry.get("class"),
                "signature": plan["signature"],
            }
        )

    exported: list[dict[str, Any]] = []
    emitted_spellings: set[str] = set()
    scratch_needed = False
    for entry in callin_coverage.get("executable", []):
        if not (environment_mask_for_callin(entry) & bit):
            continue
        canonical = str(entry["name"])
        plan = callin_plans.get(canonical)
        if plan is None:
            raise ValueError(f"missing callin plan for {canonical}")
        params, results = callin_signature(entry, plan, records)
        spellings = [canonical, *callin_aliases.get(canonical, [])]
        for spelling in spellings:
            if spelling in emitted_spellings:
                raise ValueError(
                    f"{environment}: duplicate executable callin spelling {spelling!r}"
                )
            emitted_spellings.add(spelling)
            emit_callin_export(lines, spelling, params, results)
            exported.append(
                {
                    "name": spelling,
                    "canonical": canonical,
                    "alias": spelling != canonical,
                    "class": entry.get("class"),
                    "params": params,
                    "results": results,
                }
            )
        scratch_needed |= (
            entry.get("class") in SCRATCH_CALLIN_CLASSES
            or canonical in {"AddConsoleLine", "CommandNotify"}
        )

    if scratch_needed:
        # 64 KiB scratch starts at byte 0 in the second memory page. The
        # packed result is capacity<<32 | offset.
        packed = (65536 << 32) | 65536
        lines.extend(
            [
                '  (func (export "spring:callin/scratch-info") (result i64)',
                f"    i64.const {packed}",
                "  )",
            ]
        )

    lines.extend(
        [
            '  (func (export "spring:surface-count") (result i32)',
            f"    i32.const {len(imported) + len(exported)}",
            "  )",
            ")",
            "",
        ]
    )
    alias_count = sum(bool(entry["alias"]) for entry in exported)
    return "\n".join(lines), {
        "environment": environment,
        "callouts": imported,
        "callins": exported,
        "callout_count": len(imported),
        "callin_count": len(exported),
        "canonical_callin_count": len(exported) - alias_count,
        "alias_callin_count": alias_count,
        "surface_count": len(imported) + len(exported),
    }


def parse_core_signature(signature: str) -> tuple[list[str], list[str]]:
    left, separator, right = signature.partition("->")
    if not separator:
        raise ValueError(f"invalid Core signature {signature!r}")
    return split_signature(left), split_signature(right)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--generated", type=Path, default=GENERATED)
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    args = parser.parse_args()

    model = load_json(args.generated / "model.json")
    core_abi = load_json(args.generated / "core-abi.json")
    callout_coverage = load_json(args.generated / "core-executable-coverage.json")
    callin_plan = load_json(args.generated / "core-callin-plan.json")
    callin_coverage = load_json(
        args.generated / "core-callin-executable-coverage.json"
    )

    records = record_index(model)
    callout_plans = callout_plan_index(core_abi)
    callin_plans = callin_plan_index(callin_plan)
    callin_aliases = callin_alias_index(model)

    executable_canonical_names = {
        str(entry["name"]) for entry in callin_coverage.get("executable", [])
    }
    executable_spelling_count = sum(
        1 + len(callin_aliases.get(name, []))
        for name in executable_canonical_names
    )

    args.output.mkdir(parents=True, exist_ok=True)
    manifests = []
    for environment in ENVIRONMENTS:
        wat, manifest = generate_module(
            environment,
            callout_coverage,
            callout_plans,
            callin_coverage,
            callin_plans,
            callin_aliases,
            records,
        )
        (args.output / f"{environment}.wat").write_text(wat, encoding="utf-8")
        (args.output / f"{environment}.json").write_text(
            json.dumps(manifest, indent=2, sort_keys=True) + "\n",
            encoding="utf-8",
        )
        manifests.append(manifest)

    aggregate = {
        "version": 2,
        "purpose": "instantiate-only complete Core ABI signature/link coverage",
        "environments": manifests,
        "unique_executable_callouts": len(callout_coverage.get("executable", [])),
        "unique_executable_callins": len(executable_canonical_names),
        "unique_executable_callin_spellings": executable_spelling_count,
        "callouts_verified": int(callout_coverage.get("verified_total", 0)),
        "callins_verified": int(callin_coverage.get("verified_total", 0)),
    }
    (args.output / "manifest.json").write_text(
        json.dumps(aggregate, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


if __name__ == "__main__":
    main()
