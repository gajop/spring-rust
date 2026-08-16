#!/usr/bin/env python3
"""Extract independent native-Rust and Lua signature representations.

This is intentionally a small bridge around the existing native parity
extractors.  The Wasm generator invokes it during the signature gate; it does
not generate or read the semantic model, so a wrong model cannot make this
side of the comparison agree with itself.
"""

# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path


def camel_to_snake(value: str) -> str:
    value = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", value)
    value = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", value)
    return value.lower()


def signature_key(module: str, name: str) -> tuple[str, str]:
    return module, "".join(character for character in name.lower() if character.isalnum())


def native_signatures(root: Path) -> list[dict]:
    rust_root = root / "rust" / "crates" / "spring-native"
    sys.path.insert(0, str(rust_root))
    import extract_rust_api  # pylint: disable=import-outside-toplevel

    signatures: list[dict] = []
    generated = rust_root / "generated"
    for path in sorted(generated.glob("*_generated.rs")):
        module = path.stem.removesuffix("_generated")
        for function in extract_rust_api.extract_functions_from_file(path):
            signatures.append(
                {
                    "module": module,
                    "name": function["name"],
                    "params": function["params"],
                    "return_type": function["return_type"],
                }
            )

    # Four RmlUi operations have reviewed higher-level wrappers in the
    # hand-written Rust layer.  Include source methods so the gate sees the
    # native public surface rather than only the generated files.
    generated_keys = {
        signature_key(signature["module"], signature["name"]) for signature in signatures
    }
    for struct, functions in extract_rust_api.extract_from_source(rust_root / "src").items():
        module = camel_to_snake(struct)
        for function in functions:
            key = signature_key(module, function["name"])
            # Generated methods are the direct NativeInterface ABI surface.
            # Hand-written methods are added only when they introduce a new
            # public operation; this avoids treating an implementation helper
            # spelling as a second signature for one generated function.
            if key not in generated_keys:
                signatures.append(
                    {
                        "module": module,
                        "name": function["name"],
                        "params": function["params"],
                        "return_type": function["return_type"],
                    }
                )
                generated_keys.add(key)

    unique: dict[str, dict] = {}
    for signature in signatures:
        key = json.dumps(signature, sort_keys=True)
        unique[key] = signature
    signatures = list(unique.values())
    signatures.sort(key=lambda item: (item["module"], item["name"], item["return_type"]))
    return signatures


def lua_signatures(root: Path, native: list[dict]) -> dict:
    rust_root = root / "rust" / "crates" / "spring-native"
    sys.path.insert(0, str(rust_root))
    import extract_lua_api  # pylint: disable=import-outside-toplevel
    import match_apis  # pylint: disable=import-outside-toplevel

    documented_by_namespace = match_apis.parse_lua_functions(
        rust_root / "lua_functions.md"
    )
    documented = documented_by_namespace.get("Spring", [])
    documented = [
        {
            "name": function["name"],
            "params": function.get("params", []),
        }
        for function in documented
    ]
    documented.sort(key=lambda item: item["name"])

    registered = sorted(extract_lua_api.collect_registered_spring_functions(root))
    by_normalized: dict[str, list[dict]] = {}
    for function in native:
        by_normalized.setdefault(match_apis.normalize_name(function["name"]), []).append(
            function
        )

    # This is the one documented Spring callout that deliberately belongs to
    # the Lua-only module loader rather than NativeInterface.
    explicit_exclusions = {
        "Spring.InvokeNativeModule": "Lua-only module loader entry point",
    }
    matches = []
    unmatched = []
    for lua_function in documented:
        lua_name = lua_function["name"]
        normalized = match_apis.normalize_name(lua_name)
        candidates = by_normalized.get(normalized, [])
        if not candidates:
            unmatched.append(lua_name)
            matches.append(
                {
                    "lua_name": lua_name,
                    "native_module": None,
                    "native_name": None,
                    "params_match": False,
                    "detail": "no normalized native candidate",
                }
            )
            continue

        # The current public surface is one-to-one after normalization. Keep
        # the ambiguity visible if a future API adds a colliding spelling.
        candidate = candidates[0]
        if len(candidates) > 1:
            detail = f"ambiguous native candidates: {len(candidates)}"
            params_match = False
        else:
            params_match, detail = match_apis.compare_function_params(
                lua_function, candidate
            )
        matches.append(
            {
                "lua_name": lua_name,
                "native_module": candidate["module"],
                "native_name": candidate["name"],
                "params_match": params_match,
                "detail": detail,
            }
        )

    matches.sort(key=lambda item: item["lua_name"])
    return {
        "documented": documented,
        "registered": registered,
        "matches": matches,
        "unmatched": sorted(unmatched),
        "explicit_exclusions": explicit_exclusions,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[2])
    args = parser.parse_args()
    root = args.root.resolve()
    native = native_signatures(root)
    lua = lua_signatures(root, native)
    json.dump({"native": native, "lua": lua}, sys.stdout, indent=2, sort_keys=True)
    sys.stdout.write("\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
