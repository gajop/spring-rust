#!/usr/bin/env python3
"""Audit the relationship between documented Lua and native API surfaces.

The Rust API is intentionally larger than the canonical ``Spring.*`` callout
surface.  This report keeps those different namespaces separate so a Rust
method that mirrors ``gl.*``, ``VFS.*``, an RmlUi userdata method, or a
deliberately global Lua helper is not mistaken for a missing Spring binding.
"""

from __future__ import annotations

import argparse
import re
from collections import Counter, defaultdict
from pathlib import Path
from typing import Iterable

import match_apis


# ``__file__`` is ``<repo>/rust/crates/spring-native/audit_api_surfaces.py``;
# the repository root is therefore parents[3], not parents[2] (which is the
# ``rust`` directory).  Using the wrong level silently turned every source
# registration audit into an empty audit.
ROOT = Path(__file__).resolve().parents[3]
LUA_DOC = Path(__file__).with_name("lua_functions.md")
RUST_DOC = Path(__file__).with_name("rust_functions.md")


LUA_ONLY_SURFACE_REASONS = {
    "VFS.Include": "Executes arbitrary Lua code and returns arbitrary Lua values; a typed native counterpart would not preserve the contract.",
}

EXTRA_LUA_SURFACE_NAMESPACES = ("Encoding", "math", "debug", "table")


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

    # CallAsTeam is installed in the Lua global table by LuaHandleSynced.cpp;
    # the old source comment incorrectly called it Spring.CallAsTeam.  Keep
    # this explicit because the native API intentionally exposes a validated
    # callback boundary rather than Lua's arbitrary return-value stack.
    maps["Global"] = {
        "Global.CallAsTeam": ["SystemControl.call_as_team"]
        if any(
            method_name(function) == "call_as_team"
            for function in rust_by_module.get("SystemControl", [])
        )
        else []
        for function in lua_functions.get("Global", [])
    }

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

    encoding_by_compact = defaultdict(list)
    for function in rust_by_module.get("Encoding", []):
        encoding_by_compact[compact(method_name(function))].append(rust_label("Encoding", function))
    maps["Encoding"] = {
        str(function["name"]): encoding_by_compact.get(
            compact(str(function["name"]).split(".", 1)[1]), []
        )
        for function in lua_functions.get("Encoding", [])
    }

    math_by_compact = defaultdict(list)
    for function in rust_by_module.get("MathExtra", []):
        math_by_compact[compact(method_name(function))].append(rust_label("MathExtra", function))
    maps["math"] = {
        str(function["name"]): math_by_compact.get(
            compact(str(function["name"]).split(".", 1)[1]), []
        )
        for function in lua_functions.get("math", [])
    }

    # These tables are embedded Lua/runtime helpers.  Their semantics depend
    # on the Lua state or allocator and are not NativeInterface callouts.
    maps["debug"] = {str(function["name"]): [] for function in lua_functions.get("debug", [])}
    maps["table"] = {str(function["name"]): [] for function in lua_functions.get("table", [])}
    return maps


def lua_only_surface_reasons(lua_functions: dict[str, list[dict]]) -> dict[str, str]:
    reasons = dict(LUA_ONLY_SURFACE_REASONS)
    reasons.update({
        str(function["name"]): "Lua-handle introspection, lifecycle, watcher, and callin-registration state; native modules use a separate ABI and cannot expose the same handle state."
        for function in lua_functions.get("Script", [])
    })
    reasons.update({
        str(function["name"]): "Engine debug/test input injection; native modules receive the resulting event callbacks rather than owning this Lua-only test control table."
        for function in lua_functions.get("debug", [])
    })
    reasons.update({
        str(function["name"]): "Lua table allocation hint; it changes the embedded Lua allocator/table shape and has no native ABI counterpart."
        for function in lua_functions.get("table", [])
    })
    return reasons


def source_registered_names(paths: Iterable[Path]) -> set[str]:
    names: set[str] = set()
    for path in paths:
        try:
            text = path.read_text(encoding="utf-8", errors="ignore")
        except OSError:
            continue
        # Registration macros in comments are historical notes, not active
        # Lua surface.  Remove comments before scanning so commented-out
        # MapArchive/UnmapArchive entries do not become false gaps.
        text = re.sub(r'/\*.*?\*/', '', text, flags=re.DOTALL)
        text = re.sub(r'//[^\n]*', '', text)
        names.update(re.findall(r'REGISTER_LUA_CFUNC\(\s*(?:[^,()]+,\s*)?([A-Za-z][A-Za-z0-9_]*)\s*\)', text))
        names.update(re.findall(r'LuaPushNamedCFunc\(L,\s*"([A-Za-z][A-Za-z0-9_]*)"', text))
    return names


def source_script_registered_names(paths: Iterable[Path]) -> set[str]:
    """Find functions inserted into the embedded ``Script`` table only.

    ``LuaHandle.cpp`` also installs global helpers such as ``loadstring`` and
    ``next``.  Treating every function in those files as ``Script.*`` creates
    false undocumented entries, so restrict the scan to the table-construction
    and ``lua_getglobal(L, "Script")`` blocks.
    """
    names: set[str] = set()
    function_pattern = re.compile(
        r'(?:LuaPushNamedCFunc|LuaPushNamedNil)\(L,\s*"([A-Za-z][A-Za-z0-9_]*)"'
    )
    for path in paths:
        try:
            text = path.read_text(encoding="utf-8", errors="ignore")
        except OSError:
            continue
        text = re.sub(r'/\*.*?\*/', '', text, flags=re.DOTALL)
        text = re.sub(r'//[^\n]*', '', text)
        blocks = re.findall(
            r'LuaPushString\(L,\s*"Script"\);.*?lua_rawset\(L,\s*-3\);',
            text,
            flags=re.DOTALL,
        )
        blocks.extend(re.findall(
            r'lua_getglobal\(L,\s*"Script"\);.*?lua_pop\(L,\s*1\)',
            text,
            flags=re.DOTALL,
        ))
        for block in blocks:
            names.update(function_pattern.findall(block))
    return names


def source_global_registered_names(paths: Iterable[Path]) -> set[str]:
    """Find direct functions installed in the Lua global table."""
    names: set[str] = set()
    direct_pattern = re.compile(
        r'LuaPushNamedCFunc\(L,\s*"([A-Za-z][A-Za-z0-9_]*)"'
    )
    global_push = re.compile(r'lua_pushvalue\(L,\s*LUA_GLOBALSINDEX\)')
    named_table_push = re.compile(r'lua_getglobal\(L,\s*"([A-Za-z][A-Za-z0-9_]*)"\)')
    one_pop = re.compile(r'lua_pop\(L,\s*1\)')
    for path in paths:
        try:
            text = path.read_text(encoding="utf-8", errors="ignore")
        except OSError:
            continue
        text = re.sub(r'/\*.*?\*/', '', text, flags=re.DOTALL)
        text = re.sub(r'//[^\n]*', '', text)
        table_stack: list[str] = []
        for line in text.splitlines():
            if global_push.search(line):
                table_stack.append("Global")
            table_match = named_table_push.search(line)
            if table_match:
                table_stack.append(table_match.group(1))
            if table_stack and table_stack[-1] == "Global":
                names.update(f"Global.{name}" for name in direct_pattern.findall(line))
            if one_pop.search(line) and table_stack:
                table_stack.pop()
    return names


def native_classification(
    lua_functions: dict[str, list[dict]], rust_functions: dict[str, list[dict]], maps: dict[str, dict[str, list[str]]]
) -> dict[str, list[str]]:
    spring_labels = {label for values in maps["Spring"].values() for label in values}
    global_labels = {label for values in maps["Global"].values() for label in values}
    rml_labels = {label for values in maps["RmlUi"].values() for label in values}
    gl_labels = {label for values in maps["gl"].values() for label in values}
    vfs_labels = {label for values in maps["VFS"].values() for label in values}
    encoding_labels = {label for values in maps["Encoding"].values() for label in values}
    math_labels = {label for values in maps["math"].values() for label in values}

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
            elif label in global_labels:
                category = "Global Lua counterpart (semantic boundary)"
            elif label in rml_labels:
                category = "RmlUi callout counterpart"
            elif label in gl_labels:
                category = "gl counterpart or explicit overload"
            elif label in vfs_labels:
                category = "VFS counterpart"
            elif label in encoding_labels:
                category = "Encoding counterpart"
            elif label in math_labels:
                category = "global math.* counterpart (non-Spring namespace)"
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
    lua_only_reasons = lua_only_surface_reasons(lua_functions)
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
    source_script = source_script_registered_names(
        [ROOT / "rts/Lua/LuaHandle.cpp", ROOT / "rts/Lua/LuaHandleSynced.cpp", ROOT / "rts/Lua/LuaUI.cpp"]
    )
    source_global = source_global_registered_names(
        [ROOT / "rts/Lua/LuaHandleSynced.cpp", ROOT / "rts/Lua/LuaUI.cpp"]
    )
    documented_script = {
        str(function["name"]).split(".", 1)[1]
        for function in lua_functions.get("Script", [])
    }
    documented_script_casefold = {name.casefold() for name in documented_script}
    source_only_script = sorted(
        f"Script.{name}"
        for name in source_script
        if name.casefold() not in documented_script_casefold
    )

    lines = [
        "# Lua / Native API Surface Audit",
        "",
        "This report is generated from `lua_functions.md`, `rust_functions.md`, and the local Lua registration sites.",
        "It distinguishes canonical Spring callouts from the separate `Global`, `gl`, `VFS`, `RmlUi`, `Script`, `Encoding`, `math`, `debug`, and `table` surfaces.",
        "",
        "## Lua callout mapping",
        "",
        "| Lua namespace | Documented callouts | Mapped to native Rust | Lua-only by design | Unresolved |",
        "| --- | ---: | ---: | ---: | ---: |",
    ]
    for namespace in ("Global", "Spring", "RmlUi", "gl", "VFS", "Script", *EXTRA_LUA_SURFACE_NAMESPACES):
        functions = lua_functions.get(namespace, [])
        mapped = sum(bool(values) for values in maps[namespace].values())
        intentional = sum(
            1 for name in maps[namespace] if name in lua_only_reasons
        )
        unresolved = len(functions) - mapped - intentional
        lines.append(f"| `{namespace}` | {len(functions)} | {mapped} | {intentional} | {unresolved} |")

    spring_mapped = sum(bool(values) for values in maps["Spring"].values())
    lines.extend([
        "",
        f"`Spring` uses the same one-to-one matcher as `match_apis.py`; its {spring_mapped} mapped callouts are the canonical parity set.",
        "`Global.CallAsTeam` is the actual `_G.CallAsTeam` registration; it is mapped explicitly to `SystemControl.call_as_team` and its callback/return-stack difference is recorded as a semantic boundary.",
        "`RmlUi` is mapped by userdata path (`Context.CreateDocument` → `context_create_document`).",
        "`gl` and `VFS` use case/acronym-insensitive names plus explicit aliases for `gl.Texture` and `gl.UniformArray`.",
        "`VFS.Include`, every documented `Script.*`, `debug.*`, and `table.new` callout are explicitly Lua-only by design; they are still required to have Lua-side signature and behavior tests.",
        "",
        "## Intentional Lua-only surfaces",
        "",
        "These are not name-matching failures. They have no native counterpart by contract, but remain part of the executable Lua-surface coverage gate.",
        "",
    ])
    for name, reason in sorted(lua_only_reasons.items()):
        lines.append(f"- `{name}` — {reason}")
    lines.extend(["", "## Unmapped documented Lua callouts", ""])
    for namespace in ("Global", "Spring", "RmlUi", "gl", "VFS", "Script", *EXTRA_LUA_SURFACE_NAMESPACES):
        missing = sorted(
            name
            for name, values in maps[namespace].items()
            if not values and name not in lua_only_reasons
        )
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
    lines.extend([
        "## Local Script registrations absent from the generated Lua inventory",
        "",
        "These names are registered by the current engine source but are not present in the generated `Script` documentation section.",
        "They remain unresolved until documented and tested.",
    ])
    lines.extend(bullet_list(source_only_script) or ["- None"])
    lines.append("")
    documented_global = {
        str(function["name"]) for function in lua_functions.get("Global", [])
    }
    source_only_global = sorted(source_global - documented_global)
    lines.extend([
        "## Local global registrations absent from the generated Lua inventory",
        "",
        "These direct `_G` registrations are separate from `Spring.*`; undocumented names remain unresolved until classified and tested.",
    ])
    lines.extend(bullet_list(source_only_global) or ["- None"])
    lines.append("")

    args.output.write_text("\n".join(lines), encoding="utf-8")
    print(args.output)
    print(
        "Lua mapping: "
        + ", ".join(
            f"{namespace}={sum(bool(values) for values in maps[namespace].values())}/{len(maps[namespace])}"
            for namespace in ("Global", "Spring", "RmlUi", "gl", "VFS", "Script", *EXTRA_LUA_SURFACE_NAMESPACES)
        )
    )
    print(f"Rust rows={rust_rows}, unique labels={len(rust_unique_labels)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
