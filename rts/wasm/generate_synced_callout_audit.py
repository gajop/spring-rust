#!/usr/bin/env python3
"""Generate a review list for Core callouts visible to synced environments."""

# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

from __future__ import annotations

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
MODEL = ROOT / "rts/wasm/generated/model.json"
COVERAGE = ROOT / "rts/wasm/generated/core-executable-coverage.json"
OUTPUT = ROOT / "rts/wasm/docs/generated/generated_synced_callout_audit.md"

SUSPICIOUS_MODULES = {
    "camera",
    "config",
    "debug_input",
    "display",
    "gfx",
    "input",
    "profiling",
    "rml_ui",
    "sound",
    "system_control",
    "tracing",
    "unsynced_ctrl",
    "unsynced_read",
    "vfs",
}
SUSPICIOUS_WORDS = (
    "clipboard",
    "directory",
    "file",
    "mouse",
    "path",
    "random",
    "time",
    "timer",
    "window",
)


def review_status(module: str, function: str) -> str:
    lowered = function.lower()
    if module in SUSPICIOUS_MODULES or any(word in lowered for word in SUSPICIOUS_WORDS):
        return "review-required"
    return "candidate"


def main() -> None:
    model = json.loads(MODEL.read_text(encoding="utf-8"))
    coverage = json.loads(COVERAGE.read_text(encoding="utf-8"))
    classes = {
        (entry["module"], entry["function"]): entry.get("class", "not-listed")
        for entry in coverage["executable"]
    }
    rows = []
    for module in model["modules"]:
        for function in module.get("functions", []):
            environments = set(function.get("environments", []))
            if not environments.intersection({"rules-synced", "gaia-synced"}):
                continue
            name = function["name"]
            rows.append(
                (
                    module["name"],
                    name,
                    "x" if function.get("mutating") else "",
                    classes.get((module["name"], name), "not-listed"),
                    review_status(module["name"], name),
                )
            )

    lines = [
        "# Synced callout audit",
        "",
        "- source: `rts/wasm/generated/model.json`",
        "- coverage: `rts/wasm/generated/core-executable-coverage.json`",
        "- status: heuristic inventory; human review required",
        "- candidate: no name or module heuristic matched",
        "- review-required: module or name may depend on unsynced state",
        "",
        "| module | callout | mutating | transport | review |",
        "| --- | --- | ---: | --- | --- |",
    ]
    for row in sorted(rows):
        lines.append("| `{}` | `{}` | {} | `{}` | `{}` |".format(*row))
    lines.append("")
    OUTPUT.write_text("\n".join(lines), encoding="utf-8")
    print(f"generated {len(rows)} synced-visible callout audit rows")


if __name__ == "__main__":
    main()
