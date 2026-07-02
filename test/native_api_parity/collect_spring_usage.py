#!/usr/bin/env python3
"""Collect Spring.* function usage from a game tree."""

from __future__ import annotations

import argparse
import re
from collections import defaultdict
from pathlib import Path


SPRING_CALL = re.compile(r"\bSpring\.([A-Za-z_][A-Za-z0-9_]*)\s*\(")
DEFAULT_EXTENSIONS = {".lua", ".h.lua", ".tbl", ".tdf"}


def iter_source_files(root: Path):
    for path in root.rglob("*"):
        if path.is_file() and path.suffix in DEFAULT_EXTENSIONS:
            yield path


def collect(root: Path):
    calls = defaultdict(list)
    for path in iter_source_files(root):
        try:
            text = path.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue

        for lineno, line in enumerate(text.splitlines(), start=1):
            for match in SPRING_CALL.finditer(line):
                calls[match.group(1)].append((path.relative_to(root), lineno))

    return calls


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("root", type=Path, help="game directory to scan")
    parser.add_argument(
        "-o",
        "--output",
        type=Path,
        default=Path(__file__).with_name("spring_usage_inventory.md"),
    )
    args = parser.parse_args()

    calls = collect(args.root)
    lines = [
        "# Spring Usage Inventory",
        "",
        f"Source: `{args.root}`",
        f"Functions: {len(calls)}",
        "",
    ]

    for name in sorted(calls):
        refs = calls[name]
        sample = ", ".join(f"{path}:{line}" for path, line in refs[:5])
        suffix = "" if len(refs) <= 5 else f", ... +{len(refs) - 5}"
        lines.append(f"- Spring.{name}: {len(refs)} uses ({sample}{suffix})")

    args.output.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"wrote {args.output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
