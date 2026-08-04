#!/usr/bin/env python3
"""Audit Lua callins against the engine-to-native callback interface.

This is intentionally an inventory/signature audit, not a name-only parity
claim. Lua callins and the native ABI often use different representations for
the same semantic values (for example Float3 versus three Lua numbers). Those
representations are recorded explicitly instead of being misreported as
missing parameters.
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


LUA_ONLY_BY_DESIGN = {
    "GotChatMsg": "Lua-handle chat routing; native modules receive a separate integration stream.",
    "Initialize": "Lua-handle lifecycle callback; native modules use InitializeNativeModule.",
    "LoadCode": "Lua-handle code-loading lifecycle callback; native modules are loaded through the native ABI.",
    "RecvFromSynced": "IPC between the engine's synced and unsynced Lua handles; native modules are not Lua handles.",
    "RecvLuaMsg": "Lua-handle message routing; native modules receive the separate HandleLuaMsg hook.",
    "RecvSkirmishAIMessage": "Lua-handle skirmish-AI message routing; no native event-client counterpart exists.",
}

NATIVE_ONLY_BY_DESIGN = {
    "CollectGarbage": "Native event-client garbage-collection scheduling hook; not a script call-in.",
    "DrawAlphaFeaturesLua": "Native renderer phase hook; it is separate from Lua's DrawFeature call-in.",
    "DrawAlphaUnitsLua": "Native renderer phase hook; it is separate from Lua's DrawUnit call-in.",
    "DrawOpaqueFeaturesLua": "Native renderer phase hook; it is separate from Lua's DrawFeature call-in.",
    "DrawOpaqueUnitsLua": "Native renderer phase hook; it is separate from Lua's DrawUnit call-in.",
    "FeatureMoved": "Native engine/rendering movement notification; no script call-in is registered.",
    "HandleLuaCall": "Native-module ingress for Lua-to-native messages; not a Lua call-in.",
    "HandleLuaMsg": "Native-module ingress for network Lua messages; not a Lua call-in.",
    "LastMessagePosition": "Native event for message-position consumers; scripts expose Get/Set callouts instead.",
    "Pong": "Native network timing callback; no script call-in is registered.",
    "UnitMoved": "Native engine/rendering movement notification; no script call-in is registered.",
}

# A C query is an ABI storage shape, not the Lua callin signature.  Keep every
# known arity difference here with the source-level reason it is equivalent.
# Anything absent from this table remains an unresolved representation gap and
# must be fixed or explicitly classified before the audit can be complete.
SEMANTIC_SIGNATURE_NOTES = {
    "ActiveCommandChanged": "Lua receives cmdID/cmdType; native also carries name/action/tooltip for native consumers.",
    "AddConsoleLine": "Lua receives message/level; native section is an engine-side routing field.",
    "AllowCommand": "NativeCallinCommand expands to Lua command ID, params, options, tag and timeout; native also carries ABI flags.",
    "AllowFeatureCreation": "Native Float3 position expands to Lua x,y,z.",
    "AllowStartPosition": "Native clamped/raw Float3 values expand to Lua coordinate arguments; player/ready fields retain their Lua meaning.",
    "AllowUnitCloak": "Native hasEnemy/enemyID presence storage; Lua receives enemyID or nil.",
    "AllowUnitCreation": "Native buildPos Float3 and hasBuildInfo expand to Lua x,y,z and optional build information.",
    "AllowUnitDecloak": "Native hasObject/hasWeapon presence storage expands to Lua optional object/weapon values.",
    "AllowUnitKamikaze": "Native allowed is an engine fallback/result input; Lua receives unitID and targetID.",
    "AllowUnitTransportLoad": "Native position Float3 expands to Lua x,y,z while the nested unit record expands to Lua unit fields.",
    "AllowUnitTransportUnload": "Native position Float3 expands to Lua x,y,z while the nested unit record expands to Lua unit fields.",
    "AllowWeaponTarget": "Native hasTargetPriority/targetPriority is optional-input storage; Lua receives targetPriority or nil semantics.",
    "CameraPositionChanged": "Native Float3 expands to Lua x,y,z.",
    "CameraRotationChanged": "Native Float3 expands to Lua x,y,z.",
    "CommandFallback": "NativeCallinCommand expands to Lua command params/options; the native query omits Lua-only callback routing fields.",
    "CommandNotify": "NativeCallinCommand expands to Lua command ID, params, options and tag.",
    "DrawBuildSquare": "Native status pointer/count expands to Lua's status table.",
    "DrawFeaturesPostDeferred": "Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments.",
    "DrawGenesis": "Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments.",
    "DrawGroundDeferred": "Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments.",
    "DrawGroundPostDeferred": "Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments.",
    "DrawGroundPostForward": "Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments.",
    "DrawGroundPreDeferred": "Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments.",
    "DrawGroundPreForward": "Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments.",
    "DrawPreDecals": "Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments.",
    "DrawShadowFeaturesLua": "Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments.",
    "DrawShadowPassTransparent": "Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments.",
    "DrawShadowUnitsLua": "Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments.",
    "DrawUnitsPostDeferred": "Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments.",
    "DrawWaterPost": "Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments.",
    "DrawWorld": "Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments.",
    "DrawWorldPreUnit": "Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments.",
    "DrawWorldReflection": "Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments.",
    "DrawWorldRefraction": "Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments.",
    "DrawWorldShadow": "Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments.",
    "Explosion": "Native position Float3 expands to Lua x,y,z; optional owner is represented by a presence sentinel in the C query.",
    "GameID": "Native byte pointer/count expands to Lua's game ID string.",
    "GameOver": "Native ally-team pointer/count expands to Lua's winning ally-team table.",
    "FontsChanged": "Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments.",
    "KeyMapChanged": "Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments.",
    "KeyPress": "Native modifier/action arrays and the key label are expanded into Lua's modifiers and actionList tables.",
    "KeyRelease": "Native modifier/action arrays and the key label are expanded into Lua's modifiers and actionList tables.",
    "ResourceExcess": "Native pointer/count entries expand to Lua's resource-excess table.",
    "ShieldPreDamaged": "Native startPos/hitPos Float3 values expand to Lua coordinate arguments.",
    "SunChanged": "Native query retains the new sun state for native consumers; Lua receives no arguments.",
    "UnitCmdDone": "NativeCallinCommand expands to Lua command params/options and tag.",
    "UnitSeismicPing": "Native position Float3 expands to Lua x,y,z.",
    "UnsyncedHeightMapUpdate": "Native rectangle is an engine notification payload; Lua's callin is invoked without arguments.",
    "ViewResize": "Native geometry fields expand to Lua's single geometry table with named fields.",
}


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
    # The Rust trait keeps the engine's `MiniMap` spelling as `Minimap` in
    # method names; normalize that acronym before applying ordinary casing.
    value = value.replace("MiniMap", "Minimap")
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
    lua_only_by_design = set(lua_only_names) & set(LUA_ONLY_BY_DESIGN)
    native_only_by_design = set(native_only_names) & set(NATIVE_ONLY_BY_DESIGN)
    lua_only_unclassified = lua_only_names - lua_only_by_design
    native_only_unclassified = native_only_names - native_only_by_design
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
            note = "Lua source documentation or native query mapping is missing."
        elif name in SEMANTIC_SIGNATURE_NOTES:
            status = "semantically_mapped"
            note = SEMANTIC_SIGNATURE_NOTES[name]
        elif lua_count == native_count:
            status = "same_arity_pending_runtime_check"
            note = "Raw arity agrees; value-level parity still requires the executable harness."
        else:
            status = "unresolved_representation_gap"
            note = "Raw field count differs and has no recorded semantic mapping."
        signature_rows.append((name, lua_count, native_count, query_name, status, note))

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
            f"| Lua documented entries (namespace rows) | {sum(len(values) for values in documented.values())} |",
            f"| Lua unique documented callin names | {len(documented_names)} |",
            f"| Native C++ callback symbols | {len(native_event_symbols)} |",
            f"| Shared callback names | {len(matched_names)} |",
            f"| Documented Lua names without native callback | {len(lua_only_names)} |",
            f"| Native callback names without documented Lua callin | {len(native_only_names)} |",
            f"| Lua-only callins classified by design | {len(lua_only_by_design)} |",
            f"| Native-only callbacks classified by design | {len(native_only_by_design)} |",
            f"| Unclassified Lua-only names | {len(lua_only_unclassified)} |",
            f"| Unclassified native-only names | {len(native_only_unclassified)} |",
            f"| Native callbacks without Rust trait method | {len(trait_missing)} |",
            "",
            "## Lua names without native callback",
            "",
            "Every entry is classified below. Classification is a design decision, not evidence that its runtime behavior has already been tested.",
            "",
        ]
    )
    lines.extend(["| Name | Classification | Reason |", "| --- | --- | --- |"])
    for name in sorted(lua_only_names):
        if name in LUA_ONLY_BY_DESIGN:
            lines.append(f"| `{name}` | `lua_only_by_design` | {LUA_ONLY_BY_DESIGN[name]} |")
        else:
            lines.append(f"| `{name}` | `unresolved_gap` | Requires source-level decision. |")
    lines.extend(["", "## Native callback names without documented Lua callin", ""])
    lines.extend(["| Name | Classification | Reason |", "| --- | --- | --- |"])
    for name in sorted(native_only_names):
        if name in NATIVE_ONLY_BY_DESIGN:
            lines.append(f"| `{name}` | `native_only_by_design` | {NATIVE_ONLY_BY_DESIGN[name]} |")
        else:
            lines.append(f"| `{name}` | `unresolved_gap` | Requires source-level decision. |")
    lines.extend(["", "## Native callbacks without a Rust trait method", ""])
    lines.extend(markdown_list(sorted(trait_missing)) or ["- None"])
    lines.extend(
        [
            "",
            "## Semantic signature audit",
            "",
            "The native query column is an ABI storage shape. `semantically_mapped` means the",
            "representation difference has an explicit source-level explanation; it does not",
            "replace the value-level runtime comparison. `same_arity_pending_runtime_check`",
            "still needs an executable callback test. Any `unresolved_representation_gap` is",
            "an implementation/documentation queue item, not an intentional omission.",
            "",
            "| Native callback | Lua params | Native query fields | Query struct | Status | Notes |",
            "| --- | ---: | ---: | --- | --- | --- |",
        ]
    )
    for name, lua_count, native_count, query_name, status, note in signature_rows:
        lines.append(
            f"| `{name}` | {lua_count if lua_count is not None else 'n/a'} | "
            f"{native_count if native_count is not None else 'n/a'} | "
            f"`{query_name or 'n/a'}` | `{status}` | {note} |"
        )

    args.output.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(args.output)
    print(
        f"Lua_entries={sum(len(values) for values in documented.values())} "
        f"Lua_unique={len(documented_names)} "
        f"native={len(native_event_symbols)} shared={len(matched_names)} "
        f"lua_only_by_design={len(lua_only_by_design)} "
        f"native_only_by_design={len(native_only_by_design)} "
        f"lua_only_unclassified={len(lua_only_unclassified)} "
        f"native_only_unclassified={len(native_only_unclassified)}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
