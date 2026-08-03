#!/usr/bin/env python3
"""Audit Lua callins against the engine-to-native callback interface.

This is intentionally an inventory/signature audit, not a name-only parity
claim. Lua callins may expose more context than the native query currently
does, so query-field differences are reported for follow-up rather than
silently treated as equivalent.
"""

from __future__ import annotations

import argparse
import re
from collections import defaultdict
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[3]
LUA_DOC = Path(__file__).with_name("lua_functions.md")
CALLINS_HEADER = REPO_ROOT / "rts/NativeInterface/NativeInterfaceEventClient.h"
CALLINS_SOURCE = REPO_ROOT / "rts/NativeInterface/NativeInterfaceEventClient.cpp"
CALLINS_TYPES = REPO_ROOT / "rts/NativeInterface/api/Callins.h"
LUA_SOURCE_DIR = REPO_ROOT / "rts/Lua"


def parse_documented_callins() -> dict[str, set[str]]:
    text = LUA_DOC.read_text(encoding="utf-8")
    result: dict[str, set[str]] = defaultdict(set)
    for namespace, name in re.findall(
        r"- `((?:Callins|SyncedCallins|UnsyncedCallins)\.([A-Za-z0-9_]+))`", text
    ):
        prefix, _ = namespace.split(".", 1)
        result[prefix].add(name)
    return dict(result)


def parse_lua_source_signatures() -> dict[str, list[dict[str, str]]]:
    signatures: dict[str, list[dict[str, str]]] = defaultdict(list)
    for path in LUA_SOURCE_DIR.rglob("*"):
        if path.suffix not in {".cpp", ".h"}:
            continue
        text = path.read_text(encoding="utf-8", errors="ignore")
        for block_match in re.finditer(r"/\*{2,}(.*?)\*/", text, re.DOTALL):
            block = block_match.group(1)
            function_match = re.search(
                r"@function\s+((?:Callins|SyncedCallins|UnsyncedCallins):([A-Za-z0-9_]+))",
                block,
            )
            if function_match is None:
                continue
            namespace = function_match.group(1).split(":", 1)[0]
            name = function_match.group(2)
            params = []
            for param in re.finditer(
                r"^\s*\*?\s*@param\s+(\S+)\s+(.+)$", block, re.MULTILINE
            ):
                type_text = param.group(2).strip()
                params.append({"name": param.group(1), "type": type_text.split()[0]})
            signature = {"source": str(path.relative_to(REPO_ROOT)), "params": params}
            existing = signatures[f"{namespace}.{name}"]
            if not existing or len(params) > len(existing[0]["params"]):
                signatures[f"{namespace}.{name}"] = [signature]
    return dict(signatures)


def parse_loaded_symbols() -> set[str]:
    text = CALLINS_SOURCE.read_text(encoding="utf-8")
    body = text.split("void NativeInterfaceEventClient::LoadSymbols()", 1)[1].split(
        "void* NativeInterfaceEventClient::Initialize()", 1
    )[0]
    return set(re.findall(r"^\s*LOAD_SYMBOL\((\w+)\)", body, re.MULTILINE))


def parse_trait_methods() -> set[str]:
    text = (REPO_ROOT / "rust/crates/spring-native/src/callbacks.rs").read_text(
        encoding="utf-8"
    )
    return set(re.findall(r"^\s*fn\s+([A-Za-z0-9_]+)\s*\(", text, re.MULTILINE)) - {
        "new"
    }


def camel_to_snake(value: str) -> str:
    value = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", value)
    return re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", value).lower()


def parse_query_structs() -> dict[str, list[str]]:
    text = CALLINS_TYPES.read_text(encoding="utf-8")
    result: dict[str, list[str]] = {}
    for match in re.finditer(
        r"struct\s+(\w+?)(Query)\s*\{(.*?)\};", text, re.DOTALL
    ):
        fields = []
        for line in match.group(3).splitlines():
            line = line.split("//", 1)[0].strip()
            field = re.match(
                r"(?:const\s+)?[\w:<>]+(?:\s*\*)?\s+(\w+)\s*(?:\[.*?\])?\s*;",
                line,
            )
            if field:
                fields.append(field.group(1))
        result[match.group(1)] = fields
    return result


def parse_callback_query_types() -> dict[str, str]:
    text = CALLINS_HEADER.read_text(encoding="utf-8")
    aliases: dict[str, str] = {}
    direct: dict[str, str] = {}
    for match in re.finditer(
        r"using\s+(\w+)FuncPtr\s*=\s*[^;]*?const\s+(\w+Query)\s*\*",
        text,
    ):
        direct[match.group(1)] = match.group(2).removesuffix("Query")
    for match in re.finditer(
        r"using\s+(\w+)FuncPtr\s*=\s*(\w+)FuncPtr\s*;", text
    ):
        aliases[match.group(1)] = match.group(2)

    def resolve(name: str) -> str | None:
        seen: set[str] = set()
        while name not in direct and name in aliases and name not in seen:
            seen.add(name)
            name = aliases[name]
        return direct.get(name)

    return {name: resolved for name in set(direct) | set(aliases) if (resolved := resolve(name))}


def markdown_list(values: list[str]) -> list[str]:
    return [f"- `{value}`" for value in values]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--output",
        type=Path,
        default=Path(__file__).with_name("callin_surface_audit.md"),
    )
    args = parser.parse_args()

    documented = parse_documented_callins()
    source_signatures = parse_lua_source_signatures()
    native_symbols = parse_loaded_symbols()
    trait_methods = parse_trait_methods()
    query_structs = parse_query_structs()
    callback_queries = parse_callback_query_types()

    documented_names = set().union(*(set(values) for values in documented.values()))
    native_event_symbols = native_symbols - {"InitializeNativeModule"}
    matched_names = documented_names & native_event_symbols
    lua_only_names = documented_names - native_event_symbols
    native_only_names = native_event_symbols - documented_names
    trait_missing = {
        symbol
        for symbol in native_event_symbols
        if camel_to_snake(symbol) not in trait_methods
    }

    signature_rows = []
    for name in sorted(matched_names):
        source = next(
            (
                signature
                for namespace in ("Callins", "SyncedCallins", "UnsyncedCallins")
                for signature in source_signatures.get(f"{namespace}.{name}", [])
            ),
            None,
        )
        query_name = callback_queries.get(name)
        query_fields = query_structs.get(query_name or "", [])
        lua_count = len(source["params"]) if source else None
        native_count = len(query_fields) if query_name else None
        if lua_count is None or native_count is None:
            status = "signature_source_missing"
        elif lua_count == native_count:
            status = "same_raw_field_count"
        else:
            status = "field_count_differs"
        signature_rows.append((name, lua_count, native_count, query_name, status))

    lines = [
        "# Engine Callin Surface Audit",
        "",
        "Generated from the Lua documentation/source comments, the C++ native event client,",
        "the native C query structs, and the Rust `NativeModule` trait.",
        "See `api_surface_contract.md` for intentional-difference policy.",
        "",
        "## Inventory",
        "",
        "| Surface | Count |",
        "| --- | ---: |",
    ]
    for namespace in ("Callins", "SyncedCallins", "UnsyncedCallins"):
        lines.append(f"| Lua `{namespace}` | {len(documented.get(namespace, set()))} |")
    lines.extend(
        [
            f"| Lua documented entries | {sum(len(values) for values in documented.values())} |",
            f"| Native C++ callback symbols | {len(native_event_symbols)} |",
            f"| Shared callback names | {len(matched_names)} |",
            f"| Documented Lua names without native callback | {len(lua_only_names)} |",
            f"| Native callback names without documented Lua callin | {len(native_only_names)} |",
            f"| Native callbacks without Rust trait method | {len(trait_missing)} |",
            "",
            "## Lua names without native callback",
            "",
            "These are unresolved until individually classified or ported. They are not treated as intentional.",
            "",
        ]
    )
    lines.extend(markdown_list(sorted(lua_only_names)))
    lines.extend(["", "## Native callback names without documented Lua callin", ""])
    lines.extend(markdown_list(sorted(native_only_names)))
    lines.extend(["", "## Native callbacks without a Rust trait method", ""])
    lines.extend(markdown_list(sorted(trait_missing)) or ["- None"])
    lines.extend(
        [
            "",
            "## Raw signature field-count audit",
            "",
            "A differing count is a diagnostic signal, not automatically a bug:",
            "native queries may currently use compact IDs or pointer/count pairs where Lua",
            "receives expanded definition/team/object fields. Every difference still requires",
            "a source-level decision and, where applicable, a behavior test.",
            "",
            "| Native callback | Lua params | Native query fields | Query struct | Status |",
            "| --- | ---: | ---: | --- | --- |",
        ]
    )
    for name, lua_count, native_count, query_name, status in signature_rows:
        lines.append(
            f"| `{name}` | {lua_count if lua_count is not None else 'n/a'} | "
            f"{native_count if native_count is not None else 'n/a'} | "
            f"`{query_name or 'n/a'}` | `{status}` |"
        )

    args.output.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(args.output)
    print(
        f"Lua={sum(len(values) for values in documented.values())} "
        f"native={len(native_event_symbols)} shared={len(matched_names)} "
        f"lua_only_unresolved={len(lua_only_names)} native_only={len(native_only_names)}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
