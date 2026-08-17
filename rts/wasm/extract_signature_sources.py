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


def _strip_cpp_comments(source: str) -> str:
    source = re.sub(r"/\*.*?\*/", "", source, flags=re.DOTALL)
    return re.sub(r"//[^\n]*", "", source)


def _cpp_function_body(source: str, qualified_name: str) -> str | None:
    """Return one C++ function body without needing a C++ parser."""
    source = _strip_cpp_comments(source)
    start = source.find(qualified_name)
    if start < 0:
        return None

    opening = source.find("{", start)
    if opening < 0:
        return None

    depth = 0
    quote: str | None = None
    escaped = False
    for index in range(opening, len(source)):
        character = source[index]
        if quote is not None:
            if escaped:
                escaped = False
            elif character == "\\":
                escaped = True
            elif character == quote:
                quote = None
            continue
        if character in {'"', "'"}:
            quote = character
        elif character == "{":
            depth += 1
        elif character == "}":
            depth -= 1
            if depth == 0:
                return source[opening : index + 1]
    return None


def _lua_provider_source(root: Path, provider: str) -> str | None:
    names = [provider]
    if provider.startswith("C"):
        # Several loader helpers are class methods whose source file follows
        # the historical filename without the leading C (for example
        # CLuaUI::LoadCFunctions lives in LuaUI.cpp).
        names.append(provider[1:])
    candidates = [
        path
        for name in names
        for path in sorted((root / "rts").rglob(f"{name}.cpp"))
    ]
    candidates.extend(
        path
        for name in names
        for path in sorted((root / "rts").rglob(f"{name}.h"))
    )
    for path in candidates:
        try:
            return path.read_text(encoding="utf-8", errors="ignore")
        except OSError:
            continue
    return None


def _registered_provider_functions(
    root: Path,
    provider: str,
    method: str,
    visited: set[tuple[str, str]],
) -> set[str]:
    key = (provider, method)
    if key in visited:
        return set()
    visited.add(key)

    source = _lua_provider_source(root, provider)
    if source is None:
        return set()
    body = _cpp_function_body(source, f"{provider}::{method}")
    if body is None:
        return set()
    body = _strip_cpp_comments(body)

    functions: set[str] = set()
    # A few providers first create a nested table (for example MoveCtrl or
    # UnitScript) and then continue by adding compatibility functions to the
    # parent Spring table.  Registrations before that rawset belong to the
    # nested table, not to Spring itself.
    nested_table_end = body.rfind("lua_rawset") if "lua_createtable" in body else -1

    def is_parent_registration(match: re.Match[str]) -> bool:
        return nested_table_end < 0 or match.start() > nested_table_end

    cfunc = re.compile(r"\bREGISTER_(?:SCOPED_)?LUA_CFUNC\s*\(([^()]*)\)")
    for match in cfunc.finditer(body):
        if not is_parent_registration(match):
            continue
        # REGISTER_SCOPED_LUA_CFUNC has a scope argument; the function name
        # is the final macro argument in both forms.
        function = match.group(1).split(",")[-1].strip()
        if re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", function):
            functions.add(function)

    named = re.compile(r'\bREGISTER_NAMED_LUA_CFUNC\s*\(\s*"([^"]+)"')
    functions.update(
        match.group(1) for match in named.finditer(body) if is_parent_registration(match)
    )

    pushed = re.compile(r'\bLuaPushNamedCFunc\s*\(\s*L\s*,\s*"([^"]+)"')
    functions.update(
        match.group(1) for match in pushed.finditer(body) if is_parent_registration(match)
    )

    # LuaObjectRendering uses a local macro rather than REGISTER_LUA_CFUNC.
    # It is harmless for this parser to understand it when a loader reaches
    # that provider through a nested Push* call.
    pushed_macro = re.compile(r"\bPUSH_FUNCTION\s*\(\s*([A-Za-z_][A-Za-z0-9_]*)\s*\)")
    functions.update(
        match.group(1)
        for match in pushed_macro.finditer(body)
        if is_parent_registration(match)
    )

    nested = re.compile(
        r"\b([A-Za-z_][A-Za-z0-9_]*)(?:<[^>\n]+>)?::(Push[A-Za-z_][A-Za-z0-9_]*)\s*\("
    )
    for match in nested.finditer(body):
        nested_provider = match.group(1)
        nested_method = match.group(2)
        if (nested_provider, nested_method) == key:
            continue
        functions.update(
            _registered_provider_functions(root, nested_provider, nested_method, visited)
        )

    return functions


def collect_registered_spring_loader_functions(root: Path) -> list[str]:
    """Collect Spring.* functions from the active gadget and LuaUI loaders.

    Scanning every ``Lua*.cpp`` file also finds providers that are not loaded
    into the Spring table (and old compatibility providers that are no longer
    part of the current NativeInterface).  Follow the same loader entry points
    used by the runtime, then recursively inspect only their Push* providers.
    """
    lua_dir = root / "rts" / "Lua"
    loader_specs = [
        (lua_dir / "LuaHandleSynced.cpp", "CUnsyncedLuaHandle::Init"),
        (lua_dir / "LuaHandleSynced.cpp", "CSyncedLuaHandle::Init"),
        (lua_dir / "LuaUI.cpp", "CLuaUI::CLuaUI"),
    ]
    provider_pattern = re.compile(
        r'AddEntriesToTable\s*\(\s*L\s*,\s*"Spring"\s*,\s*'
        r"([A-Za-z_][A-Za-z0-9_]*)(?:<[^>\n]+>)?::(Push[A-Za-z_][A-Za-z0-9_]*)"
    )

    providers: set[tuple[str, str]] = set()
    for path, loader in loader_specs:
        try:
            source = path.read_text(encoding="utf-8", errors="ignore")
        except OSError:
            continue
        body = _cpp_function_body(source, loader)
        if body is None:
            continue
        providers.update(
            (match.group(1), match.group(2)) for match in provider_pattern.finditer(body)
        )

    registered: set[str] = set()
    for provider, method in providers:
        registered.update(_registered_provider_functions(root, provider, method, set()))

    # CLuaUI::LoadCFunctions creates the Spring table itself, so it is not an
    # AddEntriesToTable provider even though its registration is active.
    registered.update(
        _registered_provider_functions(root, "CLuaUI", "LoadCFunctions", set())
    )
    return sorted(f"Spring.{function}" for function in registered)


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

    registered = collect_registered_spring_loader_functions(root)
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
    # These registrations are active compatibility aliases or legacy Lua
    # tables, but they have no independent entry in the current
    # NativeInterface model.  Keep them explicit so a newly registered
    # function cannot disappear from the audit silently.
    registered_exclusions = {
        "Spring.GetMyAllyTeamID": "compatibility alias of GetLocalAllyTeamID",
        "Spring.GetMyPlayerID": "compatibility alias of GetLocalPlayerID",
        "Spring.GetMyTeamID": "compatibility alias of GetLocalTeamID",
        "Spring.GetUICommands": "legacy LuaUICommand surface",
        "Spring.GetUnitCOBValue": "legacy UnitScript compatibility surface",
        "Spring.SetUnitCOBValue": "legacy UnitScript compatibility surface",
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
        "registered_exclusions": registered_exclusions,
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
