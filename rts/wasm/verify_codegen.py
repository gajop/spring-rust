#!/usr/bin/env python3
"""Regenerate the Wasm API artifacts and fail if the checked-in copy drifts."""

# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

from __future__ import annotations

import argparse
import filecmp
import re
import subprocess
import sys
import tempfile
from pathlib import Path


IGNORED_ARTIFACTS = {"model.json", "signatures.json"}
PROBE_GENERATOR = "test/wasm_api/parity_guest/generate_probe.py"
CORE_SURFACE_GENERATOR = "test/wasm_api/generate_core_abi_surface.py"
PROBE_CONTEXTS = (
    "synced_gadget",
    "unsynced_gadget",
    "gaia_synced",
    "gaia_unsynced",
    "ui",
)
PROBE_ARTIFACTS = (
    "src/probe_generated.rs",
    "src/probe_bindings.rs",
    "src/probe_context.rs",
    "probe_manifest.json",
    "wasm_api_probe_tests.lua",
)

CALLIN_DESCRIPTOR = re.compile(r'^\s*\{"([^"]+)",', re.MULTILINE)
NUMERIC_CALLIN_BIND = re.compile(
    r'exports\[(\d+)\]\.Resolve\(context, instance,\s*'
    r'"spring:callin/([^"]+)"',
    re.MULTILINE,
)
SCRATCH_CALLIN_BIND = re.compile(
    r'ResolveOptional\(exports\[(\d+)\], context, instance,\s*'
    r'"spring:callin/([^"]+)"',
    re.MULTILINE,
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
        suffix = "_core" if context == "synced_gadget" else f"_{context}_core"
        return (
            root
            / "test/native_api_parity/fixtures/game.sdd/LuaRules/Utilities"
            / f"wasm_api_probe_tests{suffix}.lua"
        )
    if relative == "probe_manifest.json":
        suffix = "_synced_gadget.core" if context == "synced_gadget" else f"_{context}.core"
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
        "--transport",
        "core",
        "--output-root",
        str(output_root),
        "--lua-output",
        str(output_root / "wasm_api_probe_tests.lua"),
        "--manifest-output",
        str(output_root / "probe_manifest.json"),
    ]
    completed = subprocess.run(command, cwd=root, check=False)
    return completed.returncode


def rust_kebab(value: str) -> str:
    """Mirror the callin generators' heck::ToKebabCase for API spellings."""
    output: list[str] = []
    for index, character in enumerate(value):
        if character == "_":
            output.append("-")
            continue
        if character.isupper() and index and value[index - 1] != "_":
            previous = value[index - 1]
            following = value[index + 1] if index + 1 < len(value) else ""
            if previous.islower() or previous.isdigit() or following.islower():
                output.append("-")
        output.append(character.lower())
    return "".join(output)


HANDWRITTEN_MODULE_ALIAS = re.compile(
    r'inline constexpr std::string_view (\w+)Module = "([^"]+)";'
)
HANDWRITTEN_IMPORT = re.compile(r'\{(\w+)Module,\s*"([^"]+)",\s*"([^"]+)"')
GENERATED_IMPORT = re.compile(r'\{"([^"]+)", "([^"]+)", "([^"]+)"')


def verify_core_import_registries(root: Path, regenerated: Path) -> list[str]:
    """Fail when the two Core import registries disagree about a signature.

    `WasmCoreRegistry.h` resolves reviewed handwritten imports before generated
    ones, so when both claim a name the handwritten signature is the only one a
    guest can import. A generated entry advertising a different shape is
    unreachable: any module built against it is rejected by the validator, while
    the coverage report still counts it as an executable generated transport.
    """
    handwritten_path = root / "rts" / "WasmInterface" / "WasmCoreRegistry.h"
    generated_path = regenerated / "WasmCoreGeneratedRegistry.h"
    if not handwritten_path.is_file():
        return [f"missing handwritten Core import registry: {handwritten_path}"]
    if not generated_path.is_file():
        return [f"missing regenerated Core import registry: {generated_path}"]

    handwritten_text = handwritten_path.read_text(encoding="utf-8")
    modules = dict(HANDWRITTEN_MODULE_ALIAS.findall(handwritten_text))
    handwritten: dict[tuple[str, str], str] = {}
    for alias, name, signature in HANDWRITTEN_IMPORT.findall(handwritten_text):
        module = modules.get(alias)
        if module is not None:
            handwritten[(module, name)] = signature

    generated: dict[tuple[str, str], str] = {
        (module, name): signature
        for module, name, signature in GENERATED_IMPORT.findall(
            generated_path.read_text(encoding="utf-8")
        )
    }

    return [
        f"{module}.{name} is generated as {generated[(module, name)]} but the reviewed "
        f"handwritten registry resolves it as {signature}; the generated binding is "
        "unreachable, so the function belongs to handwritten_signature_owner"
        for (module, name), signature in sorted(handwritten.items())
        if (module, name) in generated and generated[(module, name)] != signature
    ]


def verify_core_callin_ordinals(regenerated: Path) -> list[str]:
    registry_path = regenerated / "WasmCallinRegistry.h"
    if not registry_path.is_file():
        return [f"missing regenerated Core callin registry: {registry_path}"]
    registry = CALLIN_DESCRIPTOR.findall(registry_path.read_text(encoding="utf-8"))
    if not registry:
        return ["regenerated WasmCallinRegistry.h contains no callins"]

    findings: list[str] = []
    generated_files = (
        ("numeric", regenerated / "WasmCoreGeneratedCallinBindings.cpp", NUMERIC_CALLIN_BIND),
        (
            "scratch",
            regenerated / "WasmCoreGeneratedScratchCallinBindings.cpp",
            SCRATCH_CALLIN_BIND,
        ),
    )
    for label, path, pattern in generated_files:
        if not path.is_file():
            findings.append(f"missing regenerated {label} Core callin bindings: {path}")
            continue
        matches = pattern.findall(path.read_text(encoding="utf-8"))
        seen: set[int] = set()
        for raw_ordinal, export_name in matches:
            ordinal = int(raw_ordinal)
            if ordinal in seen:
                findings.append(
                    f"{label} Core callin ordinal {ordinal} is generated more than once"
                )
                continue
            seen.add(ordinal)
            if ordinal <= 0 or ordinal > len(registry):
                findings.append(
                    f"{label} Core callin {export_name!r} uses out-of-range ordinal {ordinal}"
                )
                continue
            expected = rust_kebab(registry[ordinal - 1])
            if export_name != expected:
                findings.append(
                    f"{label} Core callin ordinal {ordinal} binds {export_name!r}; "
                    f"WasmCallinRegistry expects {expected!r} ({registry[ordinal - 1]})"
                )
    return findings


def generate_core_surface(root: Path, regenerated: Path, output: Path) -> int:
    command = [
        sys.executable,
        str(root / CORE_SURFACE_GENERATOR),
        "--generated",
        str(regenerated),
        "--output",
        str(output),
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

        ordinal_findings = verify_core_callin_ordinals(regenerated)
        if ordinal_findings:
            print("Generated Core callin ordinals are inconsistent:", file=sys.stderr)
            print("\n".join(ordinal_findings), file=sys.stderr)
            return 1

        registry_findings = verify_core_import_registries(root, regenerated)
        if registry_findings:
            print("Core import registries disagree:", file=sys.stderr)
            print("\n".join(registry_findings), file=sys.stderr)
            return 1

        # This consumes only freshly generated metadata. It is a cheap way to
        # require every advertised executable callout/callin class and alias to
        # have a complete Core signature rule before snapshot comparison or the
        # expensive engine build starts.
        surface_output = Path(directory) / "core-surface"
        if generate_core_surface(root, regenerated, surface_output) != 0:
            return 1
        manifest = surface_output / "manifest.json"
        if not manifest.is_file():
            print("Core ABI surface generator produced no manifest", file=sys.stderr)
            return 1

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
            # including context-specific Rust projections, participates in the
            # reproducibility gate. Only the synced source files are checked in;
            # other contexts retain manifests and Lua fixtures.
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
