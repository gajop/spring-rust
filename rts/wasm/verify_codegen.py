#!/usr/bin/env python3
"""Regenerate the Wasm API artifacts and fail if the checked-in copy drifts."""

# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

from __future__ import annotations

import argparse
import filecmp
import subprocess
import sys
import tempfile
from pathlib import Path


IGNORED_ARTIFACTS = {"model.json", "signatures.json"}
PROBE_GENERATOR = "test/wasm_api/parity_guest/generate_probe.py"
PROBE_CONTEXTS = (
    "synced_gadget",
    "unsynced_gadget",
    "gaia_synced",
    "gaia_unsynced",
)
PROBE_ARTIFACTS = (
    "wit/parity.wit",
    "src/probe_generated.rs",
    "src/probe_bindings.rs",
    "src/probe_context.rs",
    "probe_manifest.json",
    "wasm_api_probe_tests.lua",
)


def is_checked_artifact(path: Path) -> bool:
    return path.is_file() and path.name not in IGNORED_ARTIFACTS


def compare_files(expected: Path, actual: Path, label: str) -> list[str]:
    if not expected.exists():
        return [f"missing checked-in {label}: {expected}"]
    if not actual.exists():
        return [f"missing regenerated {label}: {actual}"]
    if not filecmp.cmp(expected, actual, shallow=False):
        return [f"content drift: {label}"]
    return []


def checked_probe_path(root: Path, context: str, relative: str) -> Path:
    if relative == "wasm_api_probe_tests.lua":
        suffix = "" if context == "synced_gadget" else f"_{context}"
        return (
            root
            / "test/native_api_parity/fixtures/game.sdd/LuaRules/Utilities"
            / f"wasm_api_probe_tests{suffix}.lua"
        )
    if relative == "probe_manifest.json":
        suffix = "" if context == "synced_gadget" else f"_{context}"
        return root / "test/wasm_api/parity_guest" / f"probe_manifest{suffix}.json"
    return root / "test/wasm_api/parity_guest" / relative


def generated_probe_path(probe_root: Path, relative: str) -> Path:
    if relative == "wasm_api_probe_tests.lua":
        return probe_root / relative
    return probe_root / relative


def generate_probe(
    root: Path,
    model: Path,
    output_root: Path,
    context: str,
) -> int:
    command = [
        sys.executable,
        str(root / PROBE_GENERATOR),
        "--model",
        str(model),
        "--context",
        context,
        "--output-root",
        str(output_root),
        "--lua-output",
        str(output_root / "wasm_api_probe_tests.lua"),
        "--manifest-output",
        str(output_root / "probe_manifest.json"),
    ]
    completed = subprocess.run(command, cwd=root, check=False)
    return completed.returncode


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--root", type=Path, default=Path(__file__).resolve().parents[2]
    )
    parser.add_argument(
        "--output", type=Path, default=Path("rts/wasm/generated")
    )
    args = parser.parse_args()
    root = args.root.resolve()
    output = (root / args.output).resolve()

    with tempfile.TemporaryDirectory(prefix="recoil-wasm-codegen-") as directory:
        regenerated = Path(directory) / "generated"
        command = [
            "cargo",
            "run",
            "--manifest-path",
            str(root / "rust/Cargo.toml"),
            "-p",
            "spring-native-codegen",
            "--bin",
            "spring-api-codegen",
            "--",
            "--root",
            str(root),
            "--output",
            str(regenerated),
            "--strict",
        ]
        completed = subprocess.run(command, cwd=root, check=False)
        if completed.returncode != 0:
            return completed.returncode

        expected = {
            path.relative_to(output)
            for path in output.rglob("*")
            if is_checked_artifact(path)
        }
        actual = {
            path.relative_to(regenerated)
            for path in regenerated.rglob("*")
            if is_checked_artifact(path)
        }
        differences = []
        for relative in sorted(expected | actual):
            checked = output / relative
            fresh = regenerated / relative
            if not checked.exists():
                differences.append(f"missing checked-in artifact: {relative}")
            elif not fresh.exists():
                differences.append(f"stale checked-in artifact: {relative}")
            elif not filecmp.cmp(checked, fresh, shallow=False):
                differences.append(f"content drift: {relative}")

        if differences:
            print("Wasm generated artifacts are out of date:", file=sys.stderr)
            print("\n".join(differences), file=sys.stderr)
            return 1

        for context in PROBE_CONTEXTS:
            first_root = Path(directory) / f"probe-{context}-first"
            second_root = Path(directory) / f"probe-{context}-second"
            for probe_root in (first_root, second_root):
                if generate_probe(root, regenerated / "model.json", probe_root, context) != 0:
                    return 1

            probe_differences = []
            # Every context is generated twice so the complete probe surface,
            # including context-specific Rust/WIT projections, participates in
            # the reproducibility gate.  Only the historical synced source
            # files are checked in; the other contexts retain their manifests
            # and Lua fixtures as the reviewable artifacts.
            for relative in PROBE_ARTIFACTS:
                first = generated_probe_path(first_root, relative)
                second = generated_probe_path(second_root, relative)
                probe_differences.extend(
                    compare_files(
                        first,
                        second,
                        f"{context} probe reproducibility {relative}",
                    )
                )
                if context == "synced_gadget" or relative in {
                    "probe_manifest.json",
                    "wasm_api_probe_tests.lua",
                }:
                    checked = checked_probe_path(root, context, relative)
                    probe_differences.extend(
                        compare_files(
                            checked,
                            first,
                            f"{context} probe artifact {relative}",
                        )
                    )
            if probe_differences:
                print(
                    f"Wasm {context} parity probe artifacts are out of date:",
                    file=sys.stderr,
                )
                print("\n".join(probe_differences), file=sys.stderr)
                return 1

    print("Wasm generated artifacts are reproducible and up to date.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
