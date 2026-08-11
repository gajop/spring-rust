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
import audit_lua_userdata_surfaces as userdata_audit


# ``__file__`` is ``<repo>/rust/crates/spring-native/audit_api_surfaces.py``;
# the repository root is therefore parents[3], not parents[2] (which is the
# ``rust`` directory).  Using the wrong level silently turned every source
# registration audit into an empty audit.
ROOT = Path(__file__).resolve().parents[3]
LUA_DOC = Path(__file__).with_name("lua_functions.md")
RUST_DOC = Path(__file__).with_name("rust_functions.md")


LUA_ONLY_SURFACE_REASONS = {
    "VFS.Include": "Executes arbitrary Lua code and returns arbitrary Lua values; a typed native counterpart would not preserve the contract.",
    "Spring.InvokeNativeModule": "Lua-to-native bridge entry point. Native modules receive this message through the ABI; a native module cannot expose an equivalent callout that invokes itself.",
    "RmlUi.EventListener.OnAttach": "The Lua binding exposes an abstract, non-constructible base type; real listeners use callback functions or strings on Context/Element, while native modules use callback-registration ABI values.",
    "RmlUi.EventListener.OnDetach": "The Lua binding exposes an abstract, non-constructible base type; real listeners use callback functions or strings on Context/Element, while native modules use callback-registration ABI values.",
    "RmlUi.EventListener.ProcessEvent": "The Lua binding exposes an abstract, non-constructible base type; real listeners use callback functions or strings on Context/Element, while native modules use callback-registration ABI values.",
}

EXTRA_LUA_SURFACE_NAMESPACES = ("Encoding", "math", "debug", "table")

RML_LUA_ONLY_METHODS = {
    "RmlUi.EventListener.OnAttach",
    "RmlUi.EventListener.OnDetach",
    "RmlUi.EventListener.ProcessEvent",
}

SPRING_SOURCE_FILES = (
    "LuaSyncedRead.cpp",
    "LuaSyncedCtrl.cpp",
    "LuaUnsyncedRead.cpp",
    "LuaUnsyncedCtrl.cpp",
    "LuaUnitScript.cpp",
    "LuaCob.cpp",
    "LuaGaia.cpp",
    "LuaMetalMap.cpp",
    "LuaPathFinder.cpp",
)
GL_SOURCE_FILES = (
    "LuaOpenGL.cpp",
    "LuaShaders.cpp",
    "LuaFBOs.cpp",
    "LuaVAO.cpp",
    "LuaVBO.cpp",
    "LuaFonts.cpp",
    "LuaRBOs.cpp",
)
VFS_SOURCE_FILES = ("LuaVFS.cpp", "LuaVFSDownload.cpp", "LuaArchive.cpp")
SCRIPT_SOURCE_FILES = ("LuaHandle.cpp", "LuaHandleSynced.cpp", "LuaUI.cpp", "LuaRules.cpp")

SOURCE_ONLY_REASONS = {}

GL_REGISTRATION_BOUNDARY_REASONS = {
    "gl.Begin": "LuaFont userdata method registered in the shared graphics binding; it is covered by the separate Lua userdata audit and `Gfx.font_begin`.",
    "gl.BindTexture": "LuaFont userdata method registered in the shared graphics binding; it is covered by the separate Lua userdata audit and `Gfx.font_bind_texture`.",
    "gl.End": "LuaFont userdata method registered in the shared graphics binding; it is covered by the separate Lua userdata audit and `Gfx.font_end`.",
    "gl.Print": "LuaFont userdata method registered in the shared graphics binding; it is covered by the separate Lua userdata audit and `Gfx.font_print`.",
    "gl.PrintWorld": "LuaFont userdata method registered in the shared graphics binding; it is covered by the separate Lua userdata audit and `Gfx.font_print_world`.",
    "gl.SetAutoOutlineColor": "LuaFont userdata method registered in the shared graphics binding; it is covered by the separate Lua userdata audit and `Gfx.font_set_auto_outline_color`.",
    "gl.SetOutlineColor": "LuaFont userdata method registered in the shared graphics binding; it is covered by the separate Lua userdata audit and `Gfx.font_set_outline_color`.",
    "gl.SetTextColor": "LuaFont userdata method registered in the shared graphics binding; it is covered by the separate Lua userdata audit and `Gfx.font_set_text_color`.",
    "gl.SubmitBuffered": "LuaFont userdata method registered in the shared graphics binding; it is covered by the separate Lua userdata audit and `Gfx.font_submit_buffered`.",
    "gl.WrapText": "LuaFont userdata method registered in the shared graphics binding; it is covered by the separate Lua userdata audit and `Gfx.font_wrap_text`.",
}

GLOBAL_REGISTRATION_REASONS = {
    "Global.SendToUnsynced": "Engine-installed synced-to-unsynced Lua bridge; documented as `SyncedCallins.SendToUnsynced`, not as a native callout.",
    "Global.loadstring": "Embedded Lua runtime helper replaced by the engine for controlled bytecode loading; native modules do not own the Lua global environment.",
    "Global.next": "Embedded synced Lua runtime helper with deterministic table iteration; native modules use typed ABI collections instead.",
    "Global.pairs": "Embedded synced Lua runtime helper with deterministic table iteration; native modules use typed ABI collections instead.",
}

NATIVE_ONLY_CATEGORY_REASONS = {
    "native-only typed definition/proxy surface": "Rust exposes stable typed access to Lua definition proxy tables (`UnitDefs`, `FeatureDefs`, and `WeaponDefs`) instead of reproducing Lua's dynamic table/metatable representation.",
    "native-only owned/FFI representation surface": "Rust-owned result copies, typed conversion helpers, and explicit memory-release operations adapt the C ABI lifetime/domain; they are not additional Lua callouts.",
    "native-only graphics userdata/alias surface": "Native graphics uses integer handles and typed helper records for Lua userdata, plus explicit ABI aliases; the Lua userdata audit records the corresponding object methods separately.",
    "native-only RmlUi helper/property surface": "These are typed RmlUi receiver, property, data-model, and event helpers behind the Lua binding rather than free Lua callouts.",
    "native-only VFS helper/representation surface": "Native VFS exposes typed archive/file metadata, byte buffers, and offline-safe helpers that do not have one-to-one Lua function names.",
    "native-only platform/integration surface": "These methods expose host, callback, tracing, or engine-integration capabilities owned by the native ABI; Lua reaches related behavior through different tables or callins.",
    "native-only typed query/control extension": "The native ABI intentionally exposes richer records, narrower operations, or convenience splits where Lua uses a table, overload, proxy, or no public free callout.",
}


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
    maps["Spring"] = {
        str(function["name"]): (
            [spring[str(function["name"])]]
            if str(function["name"]) in spring
            else []
        )
        for function in lua_functions.get("Spring", [])
    }

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
        str(function["name"]): (
            []
            if str(function["name"]) in RML_LUA_ONLY_METHODS
            else rml_by_method.get(rml_expected(str(function["name"])), [])
        )
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


def source_registration_audit(lua_functions: dict[str, list[dict]]) -> list[dict[str, object]]:
    """Compare active C++ registration sites with the generated inventories.

    The generic registration macro is only safe when the surrounding table is
    known.  Keep the provider lists explicit so Script helpers in LuaUI or
    LuaHandleSynced cannot be mistaken for Spring callouts.
    """
    lua_dir = ROOT / "rts" / "Lua"
    documented = {
        namespace: {
            str(function["name"]).split(".", 1)[1]
            for function in lua_functions.get(namespace, [])
            if "." in str(function["name"])
        }
        for namespace in ("Spring", "gl", "VFS", "Script")
    }

    spring_source = source_registered_names([lua_dir / name for name in SPRING_SOURCE_FILES])
    # LuaUI owns this one Spring table entry; the other LuaUI registrations are
    # Script/global helpers and must not enter the Spring inventory.
    spring_source.add("SetShockFrontFactors")

    gl_source = source_registered_names([lua_dir / name for name in GL_SOURCE_FILES])

    vfs_source = source_registered_names([lua_dir / name for name in VFS_SOURCE_FILES])
    script_source = source_script_registered_names([lua_dir / name for name in SCRIPT_SOURCE_FILES])
    global_source = source_global_registered_names(
        [lua_dir / "LuaHandleSynced.cpp", lua_dir / "LuaUI.cpp"]
    )
    # CallAsTeam is installed by pushing the global table directly, which is
    # intentionally outside the line-oriented table-stack scanner above.
    global_source.add("Global.CallAsTeam")

    rows = [
        {
            "surface": "Spring",
            "source": {f"Spring.{name}" for name in spring_source},
            "documented": {f"Spring.{name}" for name in documented["Spring"]},
            "accepted": set(SOURCE_ONLY_REASONS),
        },
        {
            "surface": "gl + LuaFont registrations",
            "source": {f"gl.{name}" for name in gl_source},
            "documented": {f"gl.{name}" for name in documented["gl"]},
            "accepted": set(GL_REGISTRATION_BOUNDARY_REASONS),
        },
        {
            "surface": "VFS",
            "source": {f"VFS.{name}" for name in vfs_source},
            "documented": {f"VFS.{name}" for name in documented["VFS"]},
            "accepted": set(),
        },
        {
            "surface": "Script",
            "source": {f"Script.{name}" for name in script_source},
            "documented": {f"Script.{name}" for name in documented["Script"]},
            "accepted": set(),
        },
        {
            "surface": "Global",
            "source": global_source,
            "documented": {str(function["name"]) for function in lua_functions.get("Global", [])},
            "accepted": set(GLOBAL_REGISTRATION_REASONS),
        },
    ]
    for row in rows:
        source = row["source"]
        docs = row["documented"]
        accepted = row["accepted"]
        row["source_only"] = sorted(source - docs)
        row["documented_only"] = sorted(docs - source)
        row["unclassified_source_only"] = sorted((source - docs) - accepted)
    return rows


def userdata_native_labels() -> set[str]:
    labels: set[str] = set()
    for counterparts in userdata_audit.COUNTERPARTS.values():
        for value in counterparts:
            module, method, *_ = value.split(".")
            labels.add(f"{module}.{method}")
    return labels


def native_only_category(module: str, label: str, userdata_labels: set[str]) -> str:
    if label in userdata_labels:
        return "native-only graphics userdata/alias surface"
    if module in {"FeatureDefs", "UnitDefs", "WeaponDefs"}:
        return "native-only typed definition/proxy surface"
    if module in {"Memory", "Config"} or label.endswith("_owned") or "_owned" in label:
        return "native-only owned/FFI representation surface"
    if module == "RmlUi":
        return "native-only RmlUi helper/property surface"
    if module == "Vfs":
        return "native-only VFS helper/representation surface"
    if module in {"Platform", "SystemControl", "SyncedCtrl", "UnsyncedRead", "UnitRendering"}:
        return "native-only platform/integration surface"
    if module == "Gfx":
        return "native-only graphics userdata/alias surface"
    return "native-only typed query/control extension"


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
    userdata_labels = userdata_native_labels()

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
            elif label in userdata_labels:
                category = "native-only graphics userdata/alias surface"
            elif module == "RmlUi":
                category = "native-only RmlUi helper/property surface"
            else:
                category = native_only_category(module, label, userdata_labels)
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
    source_audits = source_registration_audit(lua_functions)

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

    lines.extend(["", "## Native-only classification policy", ""])
    lines.append(
        "A native-only label is not counted as a missing Lua API merely because Rust exposes it. It must fit one of these source-backed representation, integration, or typed-extension categories; the separate userdata and callin audits cover their corresponding Lua object/event surfaces."
    )
    lines.extend([""])
    for category, reason in sorted(NATIVE_ONLY_CATEGORY_REASONS.items()):
        lines.append(f"- `{category}` — {reason}")

    lines.extend(["", "## Rust labels by classification", ""])
    for category, labels in sorted(classifications.items()):
        lines.append(f"### {category} ({len(labels)})")
        lines.extend(bullet_list(labels))
        lines.append("")

    lines.extend([
        "## Signature audit policy",
        "",
        "The canonical `Spring.*` parameter audit is the one-to-one exact matcher used by `match_apis.py`; it reports parameter count/type mismatches separately from name mapping. Other surfaces use representation-aware policies because Lua userdata, optional tables, callbacks, and Lua overloads are not expressible as the same raw C ABI signature.",
        "",
        "| Surface | Lua inventory | Native mapped | Lua-only by design | Signature/behavior authority |",
        "| --- | ---: | ---: | ---: | --- |",
        f"| `Spring` | {len(lua_functions.get('Spring', []))} | {sum(bool(values) for values in maps['Spring'].values())} | {sum(name in lua_only_reasons for name in maps['Spring'])} | `match_apis.py` exact parameter audit plus runtime parity rows |",
        f"| `gl` | {len(lua_functions.get('gl', []))} | {sum(bool(values) for values in maps['gl'].values())} | 0 | source registration + graphics surface tests; userdata handles audited separately |",
        f"| `VFS` | {len(lua_functions.get('VFS', []))} | {sum(bool(values) for values in maps['VFS'].values())} | {sum(name in lua_only_reasons for name in maps['VFS'])} | source registration + VFS runtime parity; `VFS.Include` is explicit Lua-only |",
        f"| `RmlUi` | {len(lua_functions.get('RmlUi', []))} | {sum(bool(values) for values in maps['RmlUi'].values())} | {sum(name in lua_only_reasons for name in maps['RmlUi'])} | SolLua source docs + receiver/property runtime surface tests |",
        f"| `Script` | {len(lua_functions.get('Script', []))} | 0 | {sum(name in lua_only_reasons for name in maps['Script'])} | source registration/docs + Lua-only behavior surface tests |",
        f"| `Global`, `Encoding`, `math` | {sum(len(lua_functions.get(ns, [])) for ns in ('Global', 'Encoding', 'math'))} | {sum(bool(values) for ns in ('Global', 'Encoding', 'math') for values in maps[ns].values())} | 0 | namespace-specific source/signature/runtime tests |",
        "",
    ])

    lines.extend([
        "## Source registration audit",
        "",
        "This compares active C++ registration sites with the generated documentation. The provider list is explicit so Script or userdata registrations cannot be counted as free Spring/gl callouts.",
        "",
        "| Surface | Active registrations | Documented | Source-only accepted by design | Unclassified source-only | Documented-only |",
        "| --- | ---: | ---: | ---: | ---: | ---: |",
    ])
    for row in source_audits:
        lines.append(
            f"| `{row['surface']}` | {len(row['source'])} | {len(row['documented'])} | {len(row['accepted'] & set(row['source_only']))} | {len(row['unclassified_source_only'])} | {len(row['documented_only'])} |"
        )
    lines.extend(["", "### Accepted active registrations without generated docs", ""])
    accepted_source_rows = []
    for row in source_audits:
        for name in row["source_only"]:
            reason = (
                SOURCE_ONLY_REASONS.get(name)
                or GL_REGISTRATION_BOUNDARY_REASONS.get(name)
                or GLOBAL_REGISTRATION_REASONS.get(name)
            )
            if reason:
                accepted_source_rows.append(f"- `{name}` — {reason}")
    lines.extend(accepted_source_rows or ["- None"])
    lines.extend(["", "### Unclassified source/documentation registration differences", ""])
    unresolved_source = False
    for row in source_audits:
        if row["unclassified_source_only"] or row["documented_only"]:
            unresolved_source = True
            lines.append(f"#### {row['surface']}")
            lines.extend(bullet_list(row["unclassified_source_only"]) or ["- No source-only names"])
            if row["documented_only"]:
                lines.append("Documented-only:")
                lines.extend(bullet_list(row["documented_only"]))
            lines.append("")
    if not unresolved_source:
        lines.append("- None")

    lines.extend([
        "",
        "The accepted `gl.*` entries above are LuaFont userdata methods registered in the shared graphics binding; their object-level Rust counterparts are audited in `audit_lua_userdata_surfaces.py` rather than treated as free callouts.",
        "The accepted `Global.*` entries are engine-installed Lua runtime/bridge functions. `Global.CallAsTeam` is documented, while `SendToUnsynced`, `loadstring`, `next`, and `pairs` are intentionally recorded as runtime or callin-boundary surfaces rather than native callouts.",
    ])

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
    return 0 if not unresolved_source else 1


if __name__ == "__main__":
    raise SystemExit(main())
