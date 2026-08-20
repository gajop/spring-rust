#!/usr/bin/env python3
"""Frozen benchmark baselines.

Re-measuring every backend costs far more than measuring the one under active
work, and the backends that are not being changed do not move between runs.
This module keeps their last real measurement on disk so a run restricted to,
say, `wasm_core` can still render the complete comparison table.

Layout, one CSV per profile and backend:

    frozen_benchmarks/
        metadata.json           provenance for the whole store
        callouts/lua.csv
        callouts/wasm_core.csv
        callins/lua.csv
        ...

The CSV columns are the union of the row keys the runners emit, so a frozen
row round-trips into exactly the dict `render_report` would have received from
a live run. These are real measurements, never interpolated or hand-written.
"""

from __future__ import annotations

import csv
import json
from datetime import datetime
from pathlib import Path

ROOT = Path(__file__).resolve().parent
DEFAULT_FROZEN_ROOT = ROOT / "frozen_benchmarks"

# Row values that must stay text; everything else is parsed as a number when it
# looks like one, so ratio arithmetic sees floats rather than strings.
TEXT_FIELDS = frozenset({"backend", "test", "status", "measurement", "note"})


def _profile_dir(frozen_root: Path, profile: str) -> Path:
    return frozen_root / profile


def backend_path(frozen_root: Path, profile: str, backend: str) -> Path:
    return _profile_dir(frozen_root, profile) / f"{backend}.csv"


def _coerce(field: str, value: str):
    if value == "":
        return None
    if field in TEXT_FIELDS:
        return value
    try:
        number = float(value)
    except ValueError:
        return value
    return int(number) if number.is_integer() and abs(number) < 2**53 else number


def load_rows(frozen_root: Path, profile: str, backend: str) -> list[dict] | None:
    """Frozen rows for one backend/profile, or None when nothing is stored."""
    path = backend_path(frozen_root, profile, backend)
    if not path.is_file():
        return None
    rows: list[dict] = []
    with path.open(newline="", encoding="utf-8") as handle:
        for raw in csv.DictReader(handle):
            row = {}
            for field, value in raw.items():
                coerced = _coerce(field, value if value is not None else "")
                # A missing key and an empty cell mean the same thing: the
                # runner never emitted that field for this row.
                if coerced is not None:
                    row[field] = coerced
            if row:
                rows.append(row)
    return rows


def save_rows(frozen_root: Path, profile: str, backend: str, rows: list[dict]) -> Path:
    path = backend_path(frozen_root, profile, backend)
    path.parent.mkdir(parents=True, exist_ok=True)

    # Stable column order: identifying fields first, then the rest sorted, so
    # the files stay diffable across runs.
    leading = [name for name in ("backend", "test", "status", "scale", "iterations") if any(name in row for row in rows)]
    remaining = sorted({key for row in rows for key in row} - set(leading))
    fields = leading + remaining

    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields, extrasaction="ignore")
        writer.writeheader()
        for row in rows:
            writer.writerow({field: row.get(field, "") for field in fields})
    return path


def freeze_summary(frozen_root: Path, summary: dict) -> list[Path]:
    """Persist every backend present in one profile summary."""
    profile = str(summary["profile"])
    written = []
    for backend, rows in summary.get("rows", {}).items():
        written.append(save_rows(frozen_root, profile, backend, list(rows)))
    return written


def write_metadata(frozen_root: Path, *, engine_version: str, cpu: str, note: str = "") -> Path:
    frozen_root.mkdir(parents=True, exist_ok=True)
    path = frozen_root / "metadata.json"
    existing = {}
    if path.is_file():
        existing = json.loads(path.read_text(encoding="utf-8"))
    existing.update(
        {
            "updated": datetime.now().astimezone().isoformat(timespec="seconds"),
            "engineVersion": engine_version,
            "cpu": cpu,
        }
    )
    if note:
        existing["note"] = note
    path.write_text(json.dumps(existing, indent=2) + "\n", encoding="utf-8")
    return path


def fill_missing(summary: dict, backends: tuple[str, ...], frozen_root: Path) -> tuple[dict, list[str]]:
    """Return the summary with frozen rows merged in for backends not run.

    The second element lists the backends that came from the frozen store, so
    callers can say so rather than passing stale numbers off as fresh.
    """
    profile = str(summary["profile"])
    rows = dict(summary.get("rows", {}))
    from_frozen = []
    for backend in backends:
        if backend in rows:
            continue
        frozen = load_rows(frozen_root, profile, backend)
        if frozen is None:
            raise RuntimeError(
                f"no frozen baseline for backend '{backend}' in profile '{profile}' "
                f"({backend_path(frozen_root, profile, backend)}); run that backend once "
                f"with RECOIL_BENCHMARK_FREEZE=1 to record it"
            )
        rows[backend] = frozen
        from_frozen.append(backend)
    merged = dict(summary)
    merged["rows"] = rows
    return merged, from_frozen
