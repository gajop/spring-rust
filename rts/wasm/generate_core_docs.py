#!/usr/bin/env python3
"""Generate the factual Core SDK reference."""

# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

from __future__ import annotations

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
MODEL = ROOT / "rts/wasm/generated/model.json"
COVERAGE = ROOT / "rts/wasm/generated/core-executable-coverage.json"
ABI = ROOT / "rts/wasm/generated/core-abi.json"
OUTPUT = ROOT / "rts/wasm/docs/generated/core_api_reference.md"

ENVIRONMENTS = (
    "rules-synced",
    "rules-unsynced",
    "gaia-synced",
    "gaia-unsynced",
    "ui",
)


def main() -> None:
    model = json.loads(MODEL.read_text())
    coverage = json.loads(COVERAGE.read_text())
    abi = json.loads(ABI.read_text())
    coverage_by_callout = {
        (entry["module"], entry["function"]): entry
        for entry in coverage["executable"]
    }
    signature_by_callout = {
        (entry["module"], entry["function"]): entry["signature"]
        for entry in abi["functions"]
    }
    callout_count = sum(len(module.get("functions", [])) for module in model["modules"])

    lines = [
        "# Core API reference",
        "",
        "- source: `rts/wasm/generated/model.json`",
        "- coverage: `rts/wasm/generated/core-executable-coverage.json`",
        f"- callouts: {callout_count}",
        "",
        "## Modules",
        "",
        "| module | callouts |",
        "| --- | ---: |",
    ]
    for module in model["modules"]:
        functions = module.get("functions", [])
        if functions:
            lines.append(f"| `{module['name']}` | {len(functions)} |")

    for module in model["modules"]:
        functions = module.get("functions", [])
        if not functions:
            continue
        lines.extend(
            [
                "",
                f"## `{module['name']}`",
                "",
                "| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |",
                "| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |",
            ]
        )
        for function in functions:
            key = (module["name"], function["name"])
            entry = coverage_by_callout.get(key, {})
            environments = set(function.get("environments", []))
            sync = "synced-visible" if environments & {"rules-synced", "gaia-synced"} else "unsynced-only"
            transport = entry.get("class", "not-listed")
            values = ["x" if environment in environments else "" for environment in ENVIRONMENTS]
            lines.append(
                "| `{}` | {} | {} | {} | {} | {} | {} | {} | {} |".format(
                    function["name"],
                    signature_by_callout.get(key, "not-listed"),
                    *values,
                    sync,
                    transport,
                    "x" if function.get("mutating") else "",
                )
            )

    lines.append("")
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text("\n".join(lines))


if __name__ == "__main__":
    main()
