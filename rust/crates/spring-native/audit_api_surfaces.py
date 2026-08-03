#!/usr/bin/env python3
"""Audit the relationship between documented Lua and native API surfaces.

The Rust API is intentionally larger than the canonical ``Spring.*`` callout
surface.  This report keeps those different namespaces separate so a Rust
method that mirrors ``gl.*``, ``VFS.*``, or an RmlUi userdata method is not
mistaken for a missing Spring binding.
"""

from __future__ import annotations

import argparse
import re
from collections import Counter, defaultdict
from pathlib import Path
from typing import Iterable

import match_apis


ROOT = Path(__file__).resolve().parents[2]
LUA_DOC = Path(__file__).with_name("lua_functions.md")
RUST_DOC = Path(__file__).with_name("rust_functions.md")


def method_name(function: dict) -> str:
    return str(function["name"]).rsplit(".", 1)[-1]


def compact(value: str) -> str:
    return re.sub(r"[^a-z0-9]", "", value.lower())


def camel_to_snake(value: str) -> str:
    value = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", value)
    return value.lower()


def rust_label(module: str, function: dict) -> str:
    return f"{module}.{method_name(function)}"


def unique_rust_rows(rust_functions: dict[str, list[dict]]) -> dict[str, list[dict]]:
    """Deduplicate only exact module/method/parameter rows for classification."""
    result: dict[str, list[dict]] = {}
    for module, functions in rust_functions.items():
        seen: set[tuple[str, tuple[tuple[str, str], ...]]] = set()
        result[module] = []
        for function in functions:
            key = (
                method_name(function),
                tuple((str(p.get("name", "")), str(p.get("type", ""))) for p in function.get("params", [])),
            )
            if key in seen:
                continue
            seen.add(key)
            result[module].append(function)
    return result


def spring_matches(
    lua_functions: dict[str, list[dict]], rust_functions: dict[str, list[dict]]
) -> dict[str, str]:
    """Use the same one-to-one matcher as the signature report."""
    matches: dict[str, str] = {}
    used: set[str] = set()
    for lua_function in lua_functions.get("Spring", []):
        candidate = match_apis.find_best_match(lua_function, rust_functions)
        if candidate is None:
            continue
        rust_function, _confidence, module = candidate
        label = rust_label(module, rust_function)
        if label in used:
            continue
        used.add(label)
        matches[str(lua_function["name"])] = label
    return matches


def rml_expected(lua_name: str) -> str:
    parts = lua_name.split(".")[1:]
    return "_".join(camel_to_snake(part) for part in parts)


def namespace_maps(
    lua_functions: dict[str, list[dict]], rust_functions: dict[str, list[dict]]
) -> dict[str, dict[str, list[str]]]:
    rust_by_module = unique_rust_rows(rust_functions)
    maps: dict[str, dict[str, list[str]]] = {}

    spring = spring_matches(lua_functions, rust_functions)
    maps["Spring"] = {name: [label] for name, label in spring.items()}

    rml_by_method = defaultdict(list)
    for function in rust_by_module.get("RmlUi", []):
        rml_by_method[method_name(function)].append(rust_label("RmlUi", function))
    maps["RmlUi"] = {
        str(function["name"]): rml_by_method.get(rml_expected(str(function["name"])), [])
        for function in lua_functions.get("RmlUi", [])
    }

    gfx_by_compact = defaultdict(list)
    for function in rust_by_module.get("Gfx", []):
        gfx_by_compact[compact(method_name(function))].append(rust_label("Gfx", function))
    gl_maps: dict[str, list[str]] = {}
    for function in lua_functions.get("gl", []):
        name = str(function["name"])
        tail = name.split(".", 1)[1]
        candidates = gfx_by_compact.get(compact(tail), [])
        if tail == "Texture":
            candidates = gfx_by_compact.get("bindtexture", [])
        elif tail == "UniformArray":
            candidates = gfx_by_compact.get("uniformarrayfloat", []) + gfx_by_compact.get("uniformarrayint", [])
        gl_maps[name] = candidates
    maps["gl"] = gl_maps

    vfs_by_compact = defaultdict(list)
    for function in rust_by_module.get("Vfs", []):
        vfs_by_compact[compact(method_name(function))].append(rust_label("Vfs", function))
    maps["VFS"] = {
        str(function["name"]): vfs_by_compact.get(compact(str(function["name"]).split(".", 1)[1]), [])
        for function in lua_functions.get("VFS", [])
    }

    maps["Script"] = {str(function["name"]): [] for function in lua_functions.get("Script", [])}
    return maps


def source_registered_names(paths: Iterable[Path]) -> set[str]:
    names: set[str] = set()
    for path in paths:
        try:
            text = path.read_text(encoding="utf-8", errors="ignore")
        except OSError:
            continue
        names.update(re.findall(r'REGISTER_LUA_CFUNC\((?:[^,()]+,\s*)?([A-Za-z][A-Za-z0-9_]*)\)', text))
        names.update(re.findall(r'LuaPushNamedCFunc\(L,\s*"([A-Za-z][A-Za-z0-9_]*)"', text))
    return names


def native_classification(
    lua_functions: dict[str, list[dict]], rust_functions: dict[str, list[dict]], maps: dict[str, dict[str, list[str]]]
) -> dict[str, list[str]]:
    spring_labels = {label for values in maps["Spring"].values() for label in values}
    rml_labels = {label for values in maps["RmlUi"].values() for label in values}
    gl_labels = {label for values in maps["gl"].values() for label in values}
    vfs_labels = {label for values in maps["VFS"].values() for label in values}

    # These native methods are deliberately more explicit than their Lua
    # equivalents, or are owned by a different public namespace.
    derived_labels = {
        "Vfs.create_dir",
        "Vfs.extract_mod_archive_file",
        "Vfs.get_map_square_texture",
        "Vfs.set_map_square_texture",
    }

    classified: dict[str, list[str]] = defaultdict(list)
    for module, functions in unique_rust_rows(rust_functions).items():
        for function in functions:
            label = rust_label(module, function)
            if label in spring_labels:
                category = "Spring counterpart"
            elif label in rml_labels:
                category = "RmlUi callout counterpart"
            elif label in gl_labels:
                category = "gl counterpart or explicit overload"
            elif label in vfs_labels:
                category = "VFS counterpart"
            elif label in derived_labels:
                category = "Spring counterpart in Vfs module"
            elif module == "RmlUi":
                category = "RmlUi native helper/property/data-model surface"
            elif module == "Gfx":
                category = "Gfx native-only or undocumented Lua surface"
            elif module == "Vfs":
                category = "VFS native helper or undocumented Lua surface"
            else:
                category = "native-only surface"
            classified[category].append(label)
    return {category: sorted(labels) for category, labels in classified.items()}


def bullet_list(values: Iterable[str]) -> list[str]:
    return [f"- `{value}`" for value in values]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=Path(__file__).with_name("api_surface_audit.md"))
    args = parser.parse_args()

    lua_functions = match_apis.parse_lua_functions(LUA_DOC)
    rust_functions = match_apis.parse_rust_functions(RUST_DOC)
    maps = namespace_maps(lua_functions, rust_functions)
    classifications = native_classification(lua_functions, rust_functions, maps)

    rust_rows = sum(len(functions) for functions in rust_functions.values())
    rust_unique_labels = {
        rust_label(module, function)
        for module, functions in unique_rust_rows(rust_functions).items()
        for function in functions
    }
    source_vfs = source_registered_names(
        [ROOT / "rts/Lua/LuaVFS.cpp", ROOT / "rts/Lua/LuaVFSDownload.cpp", ROOT / "rts/Lua/LuaArchive.cpp"]
    )
    documented_vfs = {
        str(function["name"]).split(".", 1)[1]
        for function in lua_functions.get("VFS", [])
    }
    source_only_vfs = sorted(f"VFS.{name}" for name in source_vfs - documented_vfs)

    lines = [
        "# Lua / Native API Surface Audit",
        "",
        "This report is generated from `lua_functions.md`, `rust_functions.md`, and the local Lua registration sites.",
        "It distinguishes canonical Spring callouts from the separate `gl`, `VFS`, `RmlUi`, and `Script` surfaces.",
        "",
        "## Lua callout mapping",
        "",
        "| Lua namespace | Documented callouts | Deterministically mapped to native Rust | Unmapped |",
        "| --- | ---: | ---: | ---: |",
    ]
    for namespace in ("Spring", "RmlUi", "gl", "VFS", "Script"):
        functions = lua_functions.get(namespace, [])
        mapped = sum(bool(values) for values in maps[namespace].values())
        lines.append(f"| `{namespace}` | {len(functions)} | {mapped} | {len(functions) - mapped} |")

    lines.extend([
        "",
        "`Spring` uses the same one-to-one matcher as `match_apis.py`; its 793 mapped callouts are the canonical parity set.",
        "`RmlUi` is mapped by userdata path (`Context.CreateDocument` → `context_create_document`).",
        "`gl` and `VFS` use case/acronym-insensitive names plus explicit aliases for `gl.Texture` and `gl.UniformArray`.",
        "`Script.*` has no corresponding Rust native module by design.",
        "",
        "## Unmapped documented Lua callouts",
        "",
    ])
    for namespace in ("Spring", "RmlUi", "gl", "VFS", "Script"):
        missing = sorted(name for name, values in maps[namespace].items() if not values)
        if not missing:
            continue
        lines.append(f"### {namespace} ({len(missing)})")
        lines.extend(bullet_list(missing))
        lines.append("")

    lines.extend([
        "## Rust inventory",
        "",
        f"- Rust documentation rows: {rust_rows}",
        f"- Unique `Module.method` labels: {len(rust_unique_labels)}",
        "- The difference is nine overloaded `RmlUi.set` rows that share one label; coverage by label must not be mistaken for overload coverage.",
        "",
        "| Classification | Unique labels |",
        "| --- | ---: |",
    ])
    for category, labels in sorted(classifications.items()):
        lines.append(f"| {category} | {len(labels)} |")

    lines.extend(["", "## Rust labels by classification", ""])
    for category, labels in sorted(classifications.items()):
        lines.append(f"### {category} ({len(labels)})")
        lines.extend(bullet_list(labels))
        lines.append("")

    lines.extend([
        "## Local VFS registrations absent from the generated Lua inventory",
        "",
        "These names are registered by the current engine source but are not present in the generated `VFS` documentation section.",
        "They must be included before claiming complete VFS documentation coverage.",
    ])
    lines.extend(bullet_list(source_only_vfs) or ["- None"])
    lines.append("")

    args.output.write_text("\n".join(lines), encoding="utf-8")
    print(args.output)
    print(
        "Lua mapping: "
        + ", ".join(
            f"{namespace}={sum(bool(values) for values in maps[namespace].values())}/{len(maps[namespace])}"
            for namespace in ("Spring", "RmlUi", "gl", "VFS", "Script")
        )
    )
    print(f"Rust rows={rust_rows}, unique labels={len(rust_unique_labels)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
