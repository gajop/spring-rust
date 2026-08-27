#!/usr/bin/env python3
"""Generate the typed Wasm parity probe from the canonical API test specs.

The probe is deliberately generated from the canonical parity metadata.  Its
projection layer maps native semantic values to the flattened fields and
transforms used by the Lua reference harness, so nested records and list
results do not require a second hand-maintained API inventory.
"""

from __future__ import annotations

import json
import re
import argparse
from functools import lru_cache
from pathlib import Path


ROOT = Path(__file__).resolve().parents[5]
MODEL_PATH = ROOT / "rts" / "wasm" / "generated" / "model.json"
API_MANIFEST = ROOT / "test" / "native_api_parity" / "api_tests.json"
API_ROOT = ROOT / "test" / "native_api_parity"
OUTPUT_ROOT = Path(__file__).resolve().parents[1]
DEFAULT_RUST_PATH = OUTPUT_ROOT / "src" / "probe_generated.rs"
DEFAULT_BINDINGS_PATH = OUTPUT_ROOT / "src" / "probe_bindings.rs"
DEFAULT_CONTEXT_PATH = OUTPUT_ROOT / "src" / "probe_context.rs"
DEFAULT_MANIFEST_PATH = OUTPUT_ROOT / "probe_manifest.json"
DEFAULT_LUA_IDS_PATH = (
    ROOT
    / "test"
    / "native_api_parity"
    / "fixtures"
    / "game.sdd"
    / "LuaRules"
    / "Utilities"
    / "wasm_api_probe_tests.lua"
)

CONTEXT_SOURCE = {
    "synced_gadget": "synced_gadget",
    "unsynced_gadget": "unsynced_gadget",
    "gaia_synced": "synced_gadget",
    "gaia_unsynced": "unsynced_gadget",
    "ui": "widget",
}

CONTEXT_WORLD = {
    "synced_gadget": "rules-synced",
    "unsynced_gadget": "rules-unsynced",
    "gaia_synced": "gaia-synced",
    "gaia_unsynced": "gaia-unsynced",
    "ui": "ui",
}


MODULE_BY_NATIVE_CLASS = {
    "UnitsQuery": "units_query",
    "UnitsInfo": "units_info",
    "UnitsWeapons": "units_weapons",
    "UnitsCommands": "units_commands",
    "UnitsPieces": "units_pieces",
    "Teams": "teams",
    "Input": "input",
    "Selection": "selection",
    "Features": "features",
    "Projectiles": "projectiles",
    "Los": "los",
    "UnitDefs": "unit_defs",
    "FeatureDefs": "feature_defs",
    "WeaponDefs": "weapon_defs",
    "Game": "game",
    "Terrain": "terrain",
    "Player": "player",
    "MathExtra": "math_extra",
    "Encoding": "encoding",
    "MetalMap": "metal_map",
    "PathFinder": "path_finder",
    "Platform": "platform",
    "SystemControl": "system_control",
    "RulesParams": "rules_params",
    "MoveCtrl": "move_ctrl",
    "Messages": "messages",
    "Config": "config",
    "Tracing": "tracing",
    "Utils": "utils",
    "SystemControl": "system_control",
    "Profiling": "profiling",
    "Vfs": "vfs",
    "TeamControl": "team_control",
    "UnitControl": "unit_control",
    "FeatureControl": "feature_control",
    "TerrainControl": "terrain_control",
    "ProjectileControl": "projectile_control",
    "EffectsControl": "effects_control",
    "GameConfig": "game_config",
    "CobScript": "cob_script",
    "UnsyncedRead": "unsynced_read",
    "UnsyncedCtrl": "unsynced_ctrl",
    "UnitRendering": "unit_rendering",
}

# A small number of parity checks describe a compound Lua operation while the
# Wasm surface exposes the individual semantic operation.  Keep these choices
# explicit at the probe boundary instead of making the generic selector guess
# from return-field order.
NATIVE_TEST_FUNCTION_OVERRIDES = {
    "game_rules_info": ("game", "get_game_rules_info"),
    "vfs_zlib_decompress": ("vfs", "zlib_decompress"),
    "give_order_to_unit_synced": ("unit_control", "give_order_to_unit"),
    "give_order_to_unit_array_synced": ("units_commands", "give_order_to_unit_map"),
    "give_order_to_unit_map_synced": ("unit_control", "give_order_array_to_unit_array"),
    "give_order_array_to_unit_synced": ("unit_control", "give_order_array_to_unit"),
    "give_order_array_to_unit_map_synced": ("unit_control", "give_order_array_to_unit_array"),
    "give_order_array_to_unit_array_synced_pairwise": ("unit_control", "give_order_array_to_unit_array"),
    "give_order_array_to_unit_array_synced_broadcast": ("unit_control", "give_order_array_to_unit_array"),
}

SCALAR_WIT_TYPES = {
    "bool": "bool",
    "i8": "s8",
    "i16": "s16",
    "i32": "s32",
    "i64": "s64",
    "u8": "u8",
    "u16": "u16",
    "u32": "u32",
    "u64": "u64",
    "f32": "f32",
    "f64": "f64",
}

WIT_KEYWORDS = {
    "use",
    "type",
    "func",
    "u8",
    "u16",
    "u32",
    "u64",
    "s8",
    "s16",
    "s32",
    "s64",
    "f32",
    "f64",
    "char",
    "own",
    "borrow",
    "resource",
    "record",
    "flags",
    "variant",
    "enum",
    "bool",
    "string",
    "option",
    "result",
    "future",
    "stream",
    "error-context",
    "list",
    "map",
    "_",
    "as",
    "from",
    "static",
    "interface",
    "tuple",
    "import",
    "export",
    "world",
    "package",
    "constructor",
    "include",
    "with",
    "async",
}

RUST_KEYWORDS = {
    "as",
    "async",
    "await",
    "become",
    "box",
    "break",
    "const",
    "continue",
    "crate",
    "dyn",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "self",
    "Self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",
    "yield",
}

FIXTURE_FIELDS = {
    "unitID": "unit_id",
    "extractorUnitID": "extractor_unit_id",
    "featureID": "feature_id",
    "projectileID": "projectile_id",
    "pieceProjectileID": "piece_projectile_id",
    "teamID": "team_id",
    "allyTeamID": "ally_team_id",
    "enemyLosUnitID": "enemy_los_unit_id",
    "enemyRadarUnitID": "enemy_radar_unit_id",
    "unitDefID": "unit_def_id",
    "featureDefID": "feature_def_id",
    "weaponDefID": "weapon_def_id",
    "weaponDefName": "weapon_def_name",
    "playerID": "player_id",
    "groundX": "ground_x",
    "groundZ": "ground_z",
}



def supported_output(
    test: dict,
    function: dict,
    records: dict[str, dict],
    functions: dict[tuple[str, str], dict],
) -> str | None:
    if not all(type_supported_by_probe(output["type"], records) for output in function["outputs"]):
        return None
    projection = output_projection(test, function, records, functions)
    return "projected" if projection is not None else None

def deterministic_param_values(test: dict) -> dict[str, tuple[object, str]]:
    """Return stable probe values for generated Lua parameters.

    The parity fixture normally randomizes ranged/generated values.  A Wasm
    guest cannot observe those Lua-local values, so the generated probe uses a
    deterministic representative and the Lua reference overlays the same
    values before invoking its getter.
    """
    values: dict[str, tuple[object, str]] = {}
    for name, parameter in test.get("params", {}).items():
        declared_type = parameter.get("type", "string")
        if "wasm_fixed" in parameter:
            value = parameter["wasm_fixed"]
        elif "fixed" in parameter:
            value = parameter["fixed"]
        elif "fixture" in parameter or "fixture_list" in parameter or "fixture_map" in parameter:
            continue
        elif "value" in parameter:
            value = parameter["value"]
        elif parameter.get("generator") in {"unit_circle"}:
            value = {"x": 0.0, "z": 1.0}
        elif parameter.get("generator") in {"map_position", "map_point"}:
            value = {"x": 1024.0, "y": 128.0, "z": 1024.0}
        elif parameter.get("generator") in {"unit_velocity", "flat_velocity"}:
            value = {"x": 1.0, "y": 0.0, "z": 0.0}
        elif parameter.get("generator") in {"front_vector"}:
            value = {"x": 0.0, "y": 0.0, "z": 1.0}
        elif parameter.get("generator") in {"right_vector"}:
            value = {"x": 1.0, "y": 0.0, "z": 0.0}
        elif parameter.get("generator") == "unit_orientation":
            value = {"x": 0.0, "y": 1.0, "z": 0.0}
        elif declared_type == "bool":
            value = False
        elif declared_type == "enum":
            enum_values = parameter.get("values", [])
            if not enum_values:
                continue
            value = enum_values[0]
        elif declared_type == "string":
            value = parameter.get("value", "")
        elif declared_type == "i32" and "range" in parameter:
            value = (int(parameter["range"][0]) + int(parameter["range"][1])) // 2
        elif declared_type == "f32" and "range" in parameter:
            value = round((float(parameter["range"][0]) + float(parameter["range"][1])) / 2.0, 3)
        else:
            continue
        values[name] = (value, declared_type)
        if parameter.get("expands_to") and isinstance(value, dict):
            for index, field in enumerate(parameter["expands_to"]):
                source_field = (parameter.get("expands_from") or parameter["expands_to"])[index]
                if source_field in value:
                    values[field] = (value[source_field], "f32")
    return values


def resolve_wasm_metadata_value(
    value: object,
    deterministic_values: dict[str, tuple[object, str]],
) -> tuple[object, bool]:
    """Resolve value references nested inside an explicit Wasm record."""
    if isinstance(value, str):
        if value == "nil":
            return None, True
        if value.startswith("value."):
            value_info = deterministic_values.get(value.removeprefix("value."))
            return (value_info[0], True) if value_info is not None else (None, False)
        return value, True
    if isinstance(value, list):
        resolved = []
        for item in value:
            item_value, ok = resolve_wasm_metadata_value(item, deterministic_values)
            if not ok:
                return None, False
            resolved.append(item_value)
        return resolved, True
    if isinstance(value, dict):
        resolved = {}
        for key, item in value.items():
            item_value, ok = resolve_wasm_metadata_value(item, deterministic_values)
            if not ok:
                return None, False
            resolved[key] = item_value
        return resolved, True
    return value, True


def candidate_expression(
    arg: object,
    test: dict,
    deterministic_values: dict[str, tuple[object, str]],
) -> tuple[str | None, str | None, object | None, str | None]:
    """Return a normalized candidate for a generated Lua argument.

    The semantic type of a runtime argument is known only after it has been
    matched to a generated NativeInterface input.  Keep the original value
    alongside the provisional Rust expression so the caller can recursively
    lower lists, fixed arrays, options, and records with the target type.
    """
    if isinstance(arg, bool):
        return None, "true" if arg else "false", arg, "bool"
    if isinstance(arg, int):
        return None, str(arg), arg, "int"
    if isinstance(arg, float):
        return None, repr(arg), arg, "float"
    if isinstance(arg, list):
        rendered = []
        for item in arg:
            candidate = candidate_expression(item, test, deterministic_values)
            if candidate[1] is None:
                return None, None, None, None
            rendered.append(candidate[1])
        return None, "vec![" + ", ".join(rendered) + "]", arg, "local"
    if isinstance(arg, dict):
        # Lua map spellings are lowered to the native list-of-IDs semantic
        # input for selection controls. The affected comparisons are
        # order-insensitive, so the metadata iteration order is sufficient.
        if all(isinstance(enabled, bool) for enabled in arg.values()):
            rendered = []
            for key, enabled in arg.items():
                if enabled is not True:
                    continue
                candidate = candidate_expression(key, test, deterministic_values)
                if candidate[1] is None:
                    return None, None, None, None
                rendered.append(candidate[1])
            return None, "vec![" + ", ".join(rendered) + "]", arg, "local"

        # Explicit Wasm translations may need a native record while the Lua
        # spelling is a flattened argument list. Resolve its value references
        # now and let candidate_to_type lower it against the target WIT record.
        resolved, ok = resolve_wasm_metadata_value(arg, deterministic_values)
        if not ok:
            return None, None, None, None
        return None, "{}", resolved, "record"
    if not isinstance(arg, str):
        return None, None, None, None
    if arg == "nil":
        return None, "None", None, "nil"
    prefix, separator, key = arg.partition(".")
    if separator and prefix == "fixture":
        field = FIXTURE_FIELDS.get(key)
        return key, f"fixture.{field}" if field else None, None, "fixture"
    if separator and prefix == "local":
        local_name = snake(key)
        return key, f"local_{local_name}", None, "local"
    if separator and prefix == "sequence":
        parts = key.split(".")
        if not parts or any(not part for part in parts):
            return key, None, None, None
        expression = f"sequence_{rust_identifier(parts[0])}"
        for part in parts[1:]:
            expression += f".{rust_identifier(part)}"
        return key, expression, None, "sequence"
    if separator and prefix == "value":
        value_info = deterministic_values.get(key)
        if value_info is None:
            parameter = test.get("params", {}).get(key, {})
            fixture_name = parameter.get("fixture") or parameter.get("fixture_list") or parameter.get("fixture_map")
            fixture_field = FIXTURE_FIELDS.get(fixture_name or "")
            if fixture_field is not None:
                return key, f"fixture.{fixture_field}", None, "fixture"
            return key, None, None, None
        value, declared_type = value_info
        return key, rust_untyped_literal(value), value, declared_type
    if not separator:
        value_info = deterministic_values.get(arg)
        if value_info is not None:
            value, declared_type = value_info
            return arg, rust_untyped_literal(value), value, declared_type
        return None, None, None, None
    return None, None, None, None


def rust_literal(value: object, declared_type: str) -> str:
    if declared_type == "bool":
        return "true" if value else "false"
    if declared_type in {"f32", "float", "float2", "float3", "float4"}:
        if isinstance(value, int):
            return f"{value}.0f32"
        return f"{value!r}f32"
    if declared_type in {"u8", "u16", "u32", "u64"}:
        return f"{int(value)}u{declared_type[1:]}"
    if declared_type in {"i8", "i16", "i32", "i64", "int"}:
        return f"{int(value)}i{declared_type[1:]}" if declared_type.startswith("i") else str(int(value))
    if isinstance(value, str):
        return json.dumps(value)
    return str(value)


def rust_untyped_literal(value: object) -> str:
    """Render a metadata value before its semantic target type is known."""
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, int):
        return str(value)
    if isinstance(value, float):
        return repr(value)
    if isinstance(value, str):
        return json.dumps(value)
    if isinstance(value, list):
        return "vec![" + ", ".join(rust_untyped_literal(item) for item in value) + "]"
    return "{}"


def rust_scalar_expression(value: object, type_info: dict) -> str | None:
    if isinstance(value, str):
        if value.startswith("fixture."):
            fixture_field = FIXTURE_FIELDS.get(value.removeprefix("fixture."))
            if fixture_field is not None:
                return f"fixture.{fixture_field}"
        if value.startswith("value.") or value == "nil":
            return None
    if type_info["kind"] == "string":
        return json.dumps(value) if isinstance(value, str) else None
    if type_info["kind"] != "scalar":
        return None
    name = type_info["name"]
    if name == "bool" and isinstance(value, bool):
        return "true" if value else "false"
    if name.startswith("f") and isinstance(value, (int, float)):
        return f"{value!r}f32" if name == "f32" else f"{value!r}f64"
    if name.startswith("u") and isinstance(value, (int, float)):
        return f"{int(value)}{name}"
    if name.startswith("i") and isinstance(value, (int, float)):
        return f"{int(value)}{name}"
    return None


def rust_typed_expression(
    value: object,
    type_info: dict,
    module: str,
    records: dict[str, dict],
) -> str | None:
    """Render a concrete metadata value for a semantic WIT type."""
    kind = type_info.get("kind")
    if kind == "string":
        expression = rust_scalar_expression(value, type_info)
        return f"{expression}.to_string()" if expression is not None else None
    if kind == "scalar":
        return rust_scalar_expression(value, type_info)
    if kind == "enum":
        if not isinstance(value, str):
            return None
        variant = value
        if type_info.get("name") == "RulesParamType" and variant in {
            "bool",
            "float",
            "string",
        }:
            variant = f"rulesparam-type-{variant}"
        return (
            f"crate::bindings::recoil::spring_api::{module}::"
            f"{pascal(type_info['name'])}::{pascal(variant)}"
        )
    if kind == "bytes":
        if isinstance(value, str):
            return f"({json.dumps(value)}).as_bytes().to_vec()"
        if isinstance(value, list) and all(isinstance(item, (int, float)) for item in value):
            return "vec![" + ", ".join(str(int(item)) for item in value) + "]"
        return None
    if kind in {"list", "fixed-array"}:
        if not isinstance(value, list):
            value = [value]
        element_type = type_info["element"]
        rendered = [rust_typed_expression(item, element_type, module, records) for item in value]
        if any(item is None for item in rendered):
            return None
        if kind == "fixed-array" and len(rendered) != int(type_info["length"]):
            return None
        return "vec![" + ", ".join(item for item in rendered if item is not None) + "]"
    if kind == "option":
        if value is None:
            return "None"
        inner = type_info.get("inner", type_info.get("element"))
        expression = rust_typed_expression(value, inner, module, records)
        return f"Some({expression})" if expression is not None else None
    if kind != "record" or not isinstance(value, (dict, list)):
        return None
    record = records.get(type_info["name"])
    if record is None:
        return None
    fields = []
    for field_index, field in enumerate(record["fields"]):
        if isinstance(value, list):
            field_value = value[field_index] if field_index < len(value) else None
        else:
            field_value = next(
                (
                    candidate
                    for key, candidate in value.items()
                    if snake(key) == snake(field["name"])
                ),
                None,
            )
        expression = rust_typed_expression(field_value, field["type"], module, records)
        if expression is None:
            return None
        fields.append(f"{rust_identifier(field['name'])}: {expression}")
    return f"crate::bindings::recoil::spring_api::{module}::{pascal(type_info['name'])} {{ " + ", ".join(fields) + " }"


def input_parameter(test: dict, input_name: str) -> dict:
    parameter = next(
        (
            value
            for name, value in test.get("params", {}).items()
            if snake(name) == snake(input_name)
        ),
        {},
    )
    if parameter:
        return parameter
    # Lua's VFS unpack helpers expose a one-based byte position, while the
    # native ABI deliberately exposes a zero-based byte offset.  The test
    # metadata keeps the Lua spelling (pos), so allow the semantic native name
    # to inherit its conversion metadata from that parameter.
    if snake(input_name) == "byte_offset":
        return next(
            (
                value
                for name, value in test.get("params", {}).items()
                if snake(name) == "pos"
            ),
            {},
        )
    return {}


def candidate_key_matches(candidate: tuple, keys: set[str]) -> bool:
    key = candidate[0]
    return key is not None and snake(key) in {snake(value) for value in keys}


def field_candidate_keys(field_name: str, parent_name: str | None = None) -> set[str]:
    """Return Lua metadata names that can populate a semantic record field."""
    field = snake(field_name)
    keys = {field}
    # A row may need a distinct Wasm value when the Lua spelling uses a
    # different index base (for example Lua weapon number 1 versus the native
    # zero-based query).  Prefer an explicit `wasm<Field>` parameter over the
    # ordinary deterministic value when the row supplies one.
    keys.add(f"wasm_{field}")
    # A few native names preserve the C ABI's descriptive suffixes while the
    # parity metadata uses the shorter Lua-facing name.  Keep these aliases
    # here so explicit ``wasm_args`` translations remain declarative instead
    # of requiring duplicate parameters solely for record matching.
    if field in {"scales", "offsets"}:
        keys.add(field.removesuffix("s"))
    if field.endswith("_or_mask"):
        keys.add(field.removesuffix("_or_mask"))
    if field.endswith("_or_impulse_mask"):
        keys.add(field.removesuffix("_or_impulse_mask"))
        if field == "acceleration_or_impulse_mask":
            keys.add("accel")
    if field.endswith("_mask"):
        keys.add(field.removesuffix("_mask"))
    if field == "up_dir":
        keys.add("up")
    elif field == "args":
        keys.add("direction")
    elif field == "next_pos_error_update":
        keys.add("next_update")
    elif field == "set_pos_error_bit":
        keys.add("set_bit")
    elif field.startswith("pos_error_"):
        suffix = field.removeprefix("pos_error_")
        keys.add(suffix)
        keys.add(f"error_{suffix}")
    if field.endswith("_ids"):
        keys.add(field[:-1])
    if field == "exclude_weapon_projectiles":
        keys.add("synced")
    elif field == "exclude_piece_projectiles":
        keys.add("weapon")
    elif field == "length":
        keys.add("max_length")
    elif field == "byte_offset":
        keys.add("pos")
    elif field == "time_out":
        keys.add("timeout")

    if parent_name:
        parent = snake(parent_name)
        parent_variants = {
            parent,
            parent.removesuffix("_position"),
            parent.removesuffix("_pos"),
        }
        for variant in parent_variants:
            keys.add(f"{variant}_{field}")
    return keys


def find_candidate(
    candidates: list[tuple],
    used: set[int],
    field_name: str,
    parent_name: str | None = None,
) -> int | None:
    keys = field_candidate_keys(field_name, parent_name)
    for index, candidate in enumerate(candidates):
        if index not in used and candidate_key_matches(candidate, keys):
            return index
    return None


def default_expression(
    type_info: dict,
    module: str,
    records: dict[str, dict],
    *,
    string_borrowed: bool = False,
) -> str | None:
    kind = type_info.get("kind")
    if kind == "string":
        return "\"\"" if string_borrowed else "String::new()"
    if kind == "bytes" or kind == "list":
        return "Vec::new()"
    if kind == "fixed-array":
        element = default_expression(type_info["element"], module, records)
        if element is None:
            return None
        return "vec![" + ", ".join(element for _ in range(int(type_info["length"]))) + "]"
    if kind == "option":
        return "None"
    if kind == "scalar":
        name = type_info["name"]
        if name == "bool":
            return "false"
        if name.startswith("f"):
            return f"0.0{name}"
        return f"0{name}"
    if kind != "record":
        return None
    record = records.get(type_info["name"])
    if record is None:
        return None
    fields = []
    for field in record["fields"]:
        expression = default_expression(field["type"], module, records)
        if expression is None:
            return None
        fields.append(f"{snake(field['name'])}: {expression}")
    return f"crate::bindings::recoil::spring_api::{module}::{pascal(type_info['name'])} {{ " + ", ".join(fields) + " }"


def candidate_to_type(
    candidate: tuple,
    type_info: dict,
    module: str,
    records: dict[str, dict],
    *,
    string_borrowed: bool = False,
    native_transform: str | None = None,
) -> str | None:
    """Convert one provisional runtime candidate to a target semantic type."""
    _key, expression, raw, declared_type = candidate
    if expression is None:
        return None
    kind = type_info.get("kind")
    if kind == "option":
        if declared_type == "nil" or expression == "None":
            return "None"
        inner = type_info.get("inner", type_info.get("element"))
        inner_expression = candidate_to_type(
            candidate,
            inner,
            module,
            records,
            string_borrowed=False,
            native_transform=native_transform,
        )
        return f"Some({inner_expression})" if inner_expression is not None else None

    if kind in {"list", "fixed-array", "bytes"}:
        element = {"kind": "scalar", "name": "u8"} if kind == "bytes" else type_info["element"]
        if isinstance(raw, str) and element.get("kind") == "scalar" and element.get("name") == "u8":
            return f"({expression}).as_bytes().to_vec()"
        # A list of records cannot use the provisional `vec![...]` expression
        # produced by candidate_expression: its elements need the generated
        # WIT record constructor.  Keep the provisional path for scalar lists
        # because those may contain fixture references such as
        # `fixture.unitID`, which are expressions rather than literals.
        if isinstance(raw, list) and element.get("kind") == "record":
            typed = rust_typed_expression(raw, type_info, module, records)
            if typed is not None:
                return typed
        if declared_type == "local" or expression.startswith("local_"):
            return expression
        if isinstance(raw, list):
            return rust_typed_expression(raw, type_info, module, records)
        scalar = candidate_to_type(candidate, element, module, records, string_borrowed=False)
        if scalar is None:
            return None
        return f"vec![{scalar}]"

    if kind == "record":
        if isinstance(raw, dict):
            return rust_typed_expression(raw, type_info, module, records)
        return None

    if kind == "enum":
        return rust_typed_expression(raw, type_info, module, records)

    if kind == "string":
        if raw is not None:
            expression = rust_scalar_expression(raw, type_info)
        if expression is None:
            return None
        if string_borrowed and expression.startswith("fixture."):
            return f"&{expression}"
        if string_borrowed and expression.startswith('"'):
            return expression
        return expression

    if raw is not None:
        expression = rust_scalar_expression(raw, type_info) or expression
    if expression is None:
        return None
    if native_transform == "lua_one_based":
        if kind != "scalar" or not type_info["name"].startswith(("i", "u")):
            return None
        return f"(({expression}) - 1) as {type_info['name']}"
    if native_transform == "lua_one_based_byte_offset":
        if kind != "scalar" or not type_info["name"].startswith(("i", "u")):
            return None
        return f"(({expression}) - 1) as {type_info['name']}"
    if kind == "scalar" and type_info["name"] != "bool":
        return f"({expression}) as {type_info['name']}"
    return expression


def build_input_expression(
    type_info: dict,
    input_name: str,
    candidates: list[tuple],
    used: set[int],
    module: str,
    records: dict[str, dict],
    test: dict,
    *,
    future_inputs: list[dict] | None = None,
    top_level: bool = False,
    parent_name: str | None = None,
    explicit_candidate_count: int | None = None,
    allow_implicit_top_level: bool = False,
) -> str | None:
    kind = type_info.get("kind")
    candidate_index = find_candidate(candidates, used, input_name, parent_name)
    if (
        top_level
        and not allow_implicit_top_level
        and explicit_candidate_count is not None
        and candidate_index is not None
        and candidate_index >= explicit_candidate_count
    ):
        candidate_index = None
    if kind == "record":
        # An explicit ``wasm_args`` record is intentionally anonymous: its
        # keys describe the target WIT fields, not the Lua parameter name.
        # A deterministic scalar with the same metadata name (for example a
        # Lua parameter named `value`) must not hide that record candidate.
        # Prefer a directly named record, then consume an anonymous record
        # before trying field-by-field matching.
        if candidate_index is not None:
            expression = candidate_to_type(candidates[candidate_index], type_info, module, records)
            if expression is not None:
                used.add(candidate_index)
                return expression
        for index, candidate in enumerate(candidates):
            if index in used or not isinstance(candidate[2], dict):
                continue
            expression = candidate_to_type(candidate, type_info, module, records)
            if expression is not None:
                used.add(index)
                return expression
    if candidate_index is not None and kind not in {"record", "option"}:
        candidate = candidates[candidate_index]
        expression = candidate_to_type(
            candidate,
            type_info,
            module,
            records,
            string_borrowed=top_level and kind == "string",
            native_transform=input_parameter(test, input_name).get("native_transform"),
        )
        if expression is not None:
            used.add(candidate_index)
            return expression

    if candidate_index is None and top_level and kind in {
        "scalar",
        "string",
        "callback",
        "pointer",
        "opaque",
    }:
        # Lua permits legacy/positional spellings whose semantic native names
        # are more precise (unitID -> unitID1/unitID2 is the common case).
        # Preserve declaration-order fallback after named matching.
        candidate_range = range(len(candidates))
        if not allow_implicit_top_level and explicit_candidate_count is not None:
            candidate_range = range(min(explicit_candidate_count, len(candidates)))
        for index in candidate_range:
            if index in used:
                continue
            candidate = candidates[index]
            expression = candidate_to_type(
                candidate,
                type_info,
                module,
                records,
                string_borrowed=kind == "string",
                native_transform=input_parameter(test, input_name).get("native_transform"),
            )
            if expression is not None:
                used.add(index)
                return expression

    if kind in {"list", "fixed-array", "bytes"}:
        # A runtime local (VFS pack -> VFS unpack) already has the complete
        # list semantic value.  Prefer it over vararg aggregation.  The same
        # path handles byte-list APIs whose Lua spelling accepts a string.
        for index, candidate in enumerate(candidates):
            if index in used:
                continue
            if candidate[3] != "local" and not isinstance(candidate[2], (list, str)):
                continue
            expression = candidate_to_type(candidate, type_info, module, records)
            if expression is not None:
                used.add(index)
                return expression
        # A few Lua APIs expose varargs while the semantic native ABI uses a
        # list (MathExtra.diag, BitBits, and GetTeamUnitsByDefs).  If no named
        # list value exists, collect the remaining scalar candidates that are
        # not clearly reserved for a later top-level input.
        future_keys = set()
        for future in future_inputs or []:
            future_keys.update(field_candidate_keys(future["name"]))
        element = {"kind": "scalar", "name": "u8"} if kind == "bytes" else type_info["element"]
        collected = []
        for index, candidate in enumerate(candidates):
            if index in used or (
                candidate[0] is not None
                and snake(candidate[0]) in {snake(key) for key in future_keys}
            ):
                continue
            expression = candidate_to_type(candidate, element, module, records)
            if expression is not None:
                collected.append((index, expression))
        if collected:
            for index, _ in collected:
                used.add(index)
            return "vec![" + ", ".join(expression for _, expression in collected) + "]"

    if kind == "record":
        record = records.get(type_info["name"])
        if record is None:
            return None
        allow_defaults = "Options" in type_info["name"] or type_info["name"] == "PlanesQuery"
        fields = []
        built_fields: dict[str, str] = {}
        for field in record["fields"]:
            field_name = field["name"]
            index = find_candidate(candidates, used, field_name, input_name)
            expression = None
            if index is not None:
                expression = candidate_to_type(
                    candidates[index],
                    field["type"],
                    module,
                    records,
                )
                if expression is not None:
                    used.add(index)
            if expression is None and field["type"].get("kind") == "record":
                expression = build_input_expression(
                    field["type"],
                    field_name,
                    candidates,
                    used,
                    module,
                    records,
                    test,
                    future_inputs=future_inputs,
                    parent_name=input_name,
                    explicit_candidate_count=explicit_candidate_count,
                    allow_implicit_top_level=allow_implicit_top_level,
                )
            if expression is None and field["name"] == "planeCount" and "planes" in built_fields:
                expression = f"({built_fields['planes']}.len() as u32)"
            if expression is None and allow_defaults:
                # Lua's positional option arguments are represented as
                # unnamed candidates (for example false,false for
                # GetUnitSeparation).  Consume them in record-field order
                # before applying the semantic default.
                for candidate_index, candidate in enumerate(candidates):
                    if candidate_index in used:
                        continue
                    positional = candidate_to_type(candidate, field["type"], module, records)
                    if positional is not None:
                        expression = positional
                        used.add(candidate_index)
                        break
            if expression is None and allow_defaults:
                expression = default_expression(field["type"], module, records)
            if expression is None:
                return None
            built_fields[field_name] = expression
            fields.append(f"{snake(field_name)}: {expression}")
        return f"crate::bindings::recoil::spring_api::{module}::{pascal(type_info['name'])} {{ " + ", ".join(fields) + " }"

    if kind == "option":
        if candidate_index is not None:
            expression = candidate_to_type(candidates[candidate_index], type_info, module, records)
            if expression is not None:
                used.add(candidate_index)
                return expression
        return "None" if top_level or parent_name else None

    if top_level:
        return None
    return default_expression(type_info, module, records)


def probe_arguments(
    test: dict,
    function: dict,
    records: dict[str, dict] | None = None,
    module: str = "",
    arg_specs: list | None = None,
) -> list[str] | None:
    records = records or {}
    deterministic_values = deterministic_param_values(test)
    explicit_arg_specs = arg_specs is not None
    if arg_specs is None:
        # Most rows share the Lua call's positional values. A small number of
        # APIs deliberately expose a more structured semantic Wasm input than
        # their legacy Lua spelling (for example a Float3 record versus three
        # goal coordinates). Keep that translation explicit in the parity row
        # instead of weakening the generic record matcher.
        arg_specs = test.get(
            "wasm_args",
            test.get("wasm_get_args", runtime_args(test)),
        )
    candidates: list[tuple] = [
        candidate_expression(arg, test, deterministic_values)
        for arg in arg_specs
    ]
    explicit_candidate_count = len(candidates)
    existing_keys = {snake(candidate[0]) for candidate in candidates if candidate[0] is not None}
    # Native semantic inputs sometimes make Lua defaults explicit (for
    # example stripPadding), or name a value only in the parity metadata
    # while the Lua call relies on the API's default.  Keep those deterministic
    # metadata values available as candidates without duplicating values that
    # are already present in runtime.args.
    for name, (value, declared_type) in deterministic_values.items():
        if (
            snake(name) == "default_value"
            and test.get("params", {}).get("hasDefault", {}).get("fixed") is False
        ):
            # The Lua metadata intentionally omitted an optional default.  Do
            # not turn the fixture's representative `defaultValue` into an
            # ABI-level Some(...) value merely because it is listed in params.
            continue
        if snake(name) in existing_keys or isinstance(value, dict):
            continue
        candidates.append((name, rust_untyped_literal(value), value, declared_type))
        existing_keys.add(snake(name))
    used: set[int] = set()
    result: list[str] = []
    for input_index, input_info in enumerate(function["inputs"]):
        type_info = input_info["type"]
        input_name = input_info["name"]
        if input_name == "_unused":
            result.append(rust_literal(0, type_info.get("name", "u8")))
            continue

        expression = build_input_expression(
            type_info,
            input_name,
            candidates,
            used,
            module,
            records,
            test,
            future_inputs=function["inputs"][input_index + 1 :],
            top_level=True,
            explicit_candidate_count=explicit_candidate_count,
            allow_implicit_top_level=not explicit_arg_specs,
        )
        if expression is None and type_info.get("kind") not in {"record", "option", "list", "fixed-array", "bytes"}:
            # A missing trailing semantic argument is an API default rather
            # than a missing probe value when Lua omitted it.  Only take this
            # path after all concrete candidates have been consumed.
            if all(index in used or candidate[1] is None for index, candidate in enumerate(candidates)):
                expression = default_expression(type_info, module, records, string_borrowed=True)
        if expression is None:
            return None
        result.append(expression)
    return result




def render_type_record(record: dict) -> list[str]:
    lines = [f"  record {kebab(record['name'])} {{"]
    for field in record["fields"]:
        lines.append(f"    {wit_identifier(field['name'])}: {wit_type(field['type'])},")
    lines.append("  }")
    return lines


def render_type_enum(enum: dict) -> list[str]:
    lines = [f"  enum {kebab(enum['name'])} {{"]
    for variant in enum.get("variants", {}):
        lines.append(f"    {wit_identifier(variant)},")
    lines.append("  }")
    return lines


def render_wit(
    selected: list[tuple[dict, str, str, dict, list[str]]],
    records: dict[str, dict],
    functions: dict[tuple[str, str], dict],
    enums: dict[str, dict],
    context: str = "synced_gadget",
) -> str:
    world = CONTEXT_WORLD[context]
    by_module: dict[str, list[dict]] = {}
    for test, module, _, function, _ in selected:
        by_module.setdefault(module, []).append(function)
        for (
            sequence_module,
            _sequence_name,
            sequence_function,
            _sequence_arguments,
            _sequence_bind,
        ) in wasm_sequence_operations(test, functions, records) or []:
            by_module.setdefault(sequence_module, []).append(sequence_function)
        for setter_module, _setter_name, setter_function, _setter_arguments in wasm_set_operations(
            test, functions, records
        ) or []:
            by_module.setdefault(setter_module, []).append(setter_function)
        callback = test.get("wasm_callback")
        if callback is not None and callback.get("call"):
            callback_target = runtime_call_target(callback.get("call", ""), functions)
            if callback_target is None:
                raise ValueError(
                    f"cannot resolve Wasm callback target {callback.get('call')!r} for {test['id']}"
                )
            callback_module, _callback_name, callback_function = callback_target
            by_module.setdefault(callback_module, []).append(callback_function)

    setup = {
        ("messages", "SendLuaRulesMsg"),
        ("messages", "SendToUnsynced"),
        # Unsynced Update is render-rate driven. The guest uses the
        # authoritative simulation frame as its readiness boundary instead of
        # assuming that a fixed number of render updates equals a game frame.
        ("game", "GetGameFrame"),
        ("units_query", "GetAllUnits"),
        ("units_query", "GetTeamUnits"),
        ("units_query", "GetTeamUnitCount"),
        ("units_info", "GetUnitDefID"),
        ("units_info", "GetUnitTeam"),
        ("units_info", "GetUnitAllyTeam"),
        # Fixture discovery also needs the exact persistent unit position for
        # the LOS/radar probes whose Lua metadata uses groundX/groundZ.
        ("units_info", "GetUnitPosition"),
        ("features", "GetAllFeatures"),
        ("features", "GetFeatureDefID"),
        ("features", "GetFeatureLuaDraw"),
        ("features", "GetFeatureDrawFlag"),
        ("feature_defs", "GetFeatureDefIDByName"),
        ("feature_defs", "GetFeatureDefName"),
        ("unit_defs", "GetUnitDefIDByName"),
        ("unit_defs", "GetUnitDefName"),
        ("unit_rendering", "GetVisibleFeatures"),
        ("unit_rendering", "GetVisibleUnits"),
        ("weapon_defs", "GetWeaponDefName"),
        ("projectiles", "GetAllProjectiles"),
        ("projectiles", "GetProjectileDefID"),
        ("projectiles", "GetProjectileOwnerID"),
        ("projectiles", "GetProjectileType"),
        ("teams", "GetPlayerListInTeam"),
    }
    for module, function_name in setup:
        function = next(
            function
            for (candidate_module, candidate_name), function in functions.items()
            if candidate_module == module and candidate_name == snake(function_name)
        )
        if world not in function.get("environments", []):
            continue
        by_module.setdefault(module, []).append(function)

    lines = [
        "package recoil:spring-api@1.0.0;",
        "",
        "// @generated by generate_probe.py; do not edit.",
        "",
    ]
    for module in sorted(by_module):
        lines.extend([f"interface {kebab(module)} {{", "  record spring-error { code: s32 }", ""])
        module_functions = {function["name"]: function for function in by_module[module]}
        record_names: set[str] = set()
        enum_names: set[str] = set()

        def add_record_type(type_info: dict) -> None:
            kind = type_info.get("kind")
            if kind == "record":
                name = type_info["name"]
                if name in record_names:
                    return
                record_names.add(name)
                record = records.get(name)
                if record is not None:
                    for field in record["fields"]:
                        add_record_type(field["type"])
            elif kind == "enum":
                enum_names.add(type_info["name"])
            elif kind in {"list", "fixed-array", "option"}:
                add_record_type(type_info.get("element", type_info.get("inner", {})))

        for function in module_functions.values():
            for input_info in function["inputs"]:
                add_record_type(input_info["type"])
            for output_info in function["outputs"]:
                add_record_type(output_info["type"])
            if len(function["outputs"]) > 1:
                record_names.add(f"{function['name']}Value")
        for enum_name in sorted(enum_names):
            enum = enums.get(enum_name)
            if enum is None:
                raise ValueError(f"missing generated probe enum {enum_name}")
            lines.extend(render_type_enum(enum))
            lines.append("")
        for record_name in sorted(record_names):
            record = records.get(record_name)
            if record is None:
                function_name = record_name.removesuffix("Value")
                function = module_functions[function_name]
                record = {
                    "name": record_name,
                    "fields": function["outputs"],
                }
            lines.extend(render_type_record(record))
            lines.append("")
        for function in sorted(module_functions.values(), key=lambda item: item["name"]):
            params = ", ".join(
                f"{wit_identifier(input_info['name'])}: {wit_type(input_info['type'])}"
                for input_info in function["inputs"]
            )
            outputs = function["outputs"]
            if len(outputs) == 0:
                # WIT's `_` result means a successful unit value.  There is
                # no named `unit` type in WIT.
                result_type = "_"
            elif len(outputs) == 1:
                result_type = wit_type(outputs[0]["type"])
            else:
                result_type = f"{kebab(function['name'])}-value"
                if f"{function['name']}Value" not in record_names:
                    raise ValueError(f"missing generated result record for {function['name']}")
            lines.append(
                f"  {kebab(function['name'])}: func({params}) -> result<{result_type}, spring-error>;"
            )
        lines.extend(["}", ""])

    lines.extend(
        [
            f"interface callins-{world} {{",
            "  record spring-error { code: s32 }",
            "",
            "  record game-frame-query {",
            "    game-frame: s32,",
            "  }",
            "",
            "  record game-frame-result {",
            "    unused: u8,",
            "  }",
            "",
            "  record update-query {",
            "    delta-seconds: f32,",
            "  }",
            "",
            "  record update-result {",
            "    unused: u8,",
            "  }",
            "",
            "  game-frame: func(query: game-frame-query) -> result<game-frame-result, spring-error>;",
            "  game-frame-post: func(query: game-frame-query) -> result<game-frame-result, spring-error>;",
            "  update: func(query: update-query) -> result<update-result, spring-error>;",
        ]
    )
    if context in {"unsynced_gadget", "gaia_unsynced"}:
        lines.extend(
            [
                "",
                "  record recv-from-synced-query {",
                "    message: string,",
                "    message-length: u32,",
                "  }",
                "",
                "  recv-from-synced: func(query: recv-from-synced-query) -> result<_, spring-error>;",
            ]
        )
    lines.extend(
        [
            "}",
            "",
            f"world {world} {{",
        ]
    )
    for module in sorted(by_module):
        lines.append(f"  import {kebab(module)};")
    lines.extend(
        [
            f"  export callins-{world};",
            "  export callback-1: func(user-data: u32);",
            "}",
            "",
        ]
    )
    return "\n".join(lines)


def encode_function(type_info: dict, expression: str, field: str) -> str:
    if type_info["kind"] == "string":
        return f'encode_string("{field}", &{expression})'
    if type_info["kind"] == "bytes":
        return f'encode_bytes("{field}", &{expression})'
    name = type_info["name"]
    return f'encode_{name}("{field}", {expression})'


def encode_list_function(
    type_info: dict,
    expression: str,
    field: str,
    module: str,
    transform: str | None = None,
) -> str:
    if type_info["kind"] == "fixed-array":
        type_info = {"kind": "list", "element": type_info["element"]}
    element = type_info["element"]
    if element["kind"] == "string":
        return f'encode_string_list("{field}", &{expression})'
    if element["kind"] == "scalar":
        name = element["name"]
        if name == "bool":
            return f'encode_bool_list("{field}", &{expression})'
        if name == "u8" and transform != "table_values":
            return f'encode_bytes("{field}", &{expression})'
        return f'encode_numeric_list("{field}", &{expression})'
    if element["kind"] == "record":
        helper = f"encode_{snake(module)}_{snake(element['name'])}_list"
        if transform == "flat_float3_list":
            if element["name"] != "Float3":
                raise ValueError("flat_float3_list requires a Float3 list")
            helper = f"encode_{snake(module)}_{snake(element['name'])}_flat_list"
        return f'{helper}("{field}", &{expression})'
    raise ValueError(f"unsupported probe list element: {type_info}")


def encode_projection(projection: dict, module: str) -> str:
    type_info = projection["type"]
    if projection.get("optional"):
        return encode_optional_function(
            type_info, projection["expression"], projection["field"]
        )
    if type_info.get("kind") == "hex-bytes":
        field = projection["field"]
        expression = projection["expression"]
        return f'encode_hex_bytes("{field}", &{expression})'
    if type_info.get("kind") in {"list", "fixed-array", "bytes"}:
        return encode_list_function(
            type_info,
            projection["expression"],
            projection["field"],
            module,
            projection.get("transform"),
        )
    return encode_function(type_info, projection["expression"], projection["field"])


def projection_condition(condition: dict, projections: list[dict]) -> str:
    """Render a presence condition declared by a Lua return specification.

    Lua APIs commonly omit the rest of a multi-return shape when a presence
    flag is false.  The condition is metadata on the omitted fields so the
    generated probe preserves that distinction instead of serializing native
    zero-initialized storage as if Lua had returned it.
    """
    if not isinstance(condition, dict):
        raise ValueError(f"invalid probe output condition: {condition!r}")
    field = condition.get("field")
    marker = next((projection for projection in projections if projection["field"] == field), None)
    if marker is None:
        raise ValueError(f"probe output condition refers to missing field: {field}")
    expression = marker["expression"]
    if condition.get("truthy", False):
        if marker["type"] != {"kind": "scalar", "name": "bool"}:
            raise ValueError(f"truthy probe output condition is not boolean: {field}")
        return expression
    if set(condition) == {"field"}:
        return expression
    raise ValueError(f"unsupported probe output condition: {condition!r}")


def encode_optional_function(type_info: dict, expression: str, field: str) -> str:
    if type_info["kind"] == "string":
        return f'encode_optional_string("{field}", {expression}.as_deref())'
    if type_info["kind"] != "scalar":
        raise ValueError(f"unsupported optional probe output type: {type_info}")
    return f'encode_optional_{type_info["name"]}("{field}", {expression})'


def runtime_call_target(
    call: str,
    functions: dict[tuple[str, str], dict],
    fallback_module: str | None = None,
) -> tuple[str, str, dict] | None:
    match = re.fullmatch(r"([^.]+)\.([^.]+)", call or "")
    if match is None:
        return None
    class_name, function_name = match.groups()
    module = next(
        (
            candidate_module
            for candidate_class, candidate_module in MODULE_BY_NATIVE_CLASS.items()
            if candidate_class.lower() == class_name.lower()
            or candidate_module == snake(class_name)
        ),
        None,
    )
    if module is None:
        module = fallback_module
    if module is None:
        return None
    function = functions.get((module, snake(function_name)))
    if function is None:
        compact_name = function_name.replace("_", "").lower()
        matches = [
            (candidate_name, candidate)
            for (candidate_module, candidate_name), candidate in functions.items()
            if candidate_module == module
            and candidate_name.replace("_", "").lower() == compact_name
        ]
        if len(matches) == 1:
            function_name, function = matches[0]
    return (module, snake(function_name), function) if function is not None else None


def wasm_sequence_operations(
    test: dict,
    functions: dict[tuple[str, str], dict],
    records: dict[str, dict],
) -> list[tuple[str, str, dict, list[str], str]] | None:
    """Resolve a declarative multi-call Wasm parity operation.

    Some Lua APIs are intentionally exposed as several typed NativeInterface
    calls even though the parity row is one logical operation.  The sequence
    metadata keeps that ordering explicit and gives each successful result a
    stable binding used by the result projection.
    """
    sequence = test.get("wasm_sequence")
    if sequence is None:
        return []
    if not isinstance(sequence, list) or not sequence:
        return None

    operations = []
    bindings: set[str] = set()
    for step in sequence:
        if not isinstance(step, dict):
            return None
        bind = step.get("bind")
        if not isinstance(bind, str) or not bind or bind in bindings:
            return None
        condition = step.get("when")
        if condition is not None:
            if (
                not isinstance(condition, str)
                or not condition.startswith("sequence.")
                or condition.removeprefix("sequence.") not in bindings
            ):
                return None
        target = runtime_call_target(step.get("call", ""), functions)
        if target is None:
            return None
        module, function_name, function = target
        arguments = probe_arguments(
            test,
            function,
            records,
            module,
            arg_specs=step.get("args", []),
        )
        if arguments is None or not function.get("outputs"):
            return None
        bindings.add(bind)
        operations.append((module, function_name, function, arguments, bind))
    return operations


def sequence_output_projection(
    test: dict,
    sequence: list[tuple[str, str, dict, list[str], str]],
    records: dict[str, dict],
) -> list[dict] | None:
    """Project flattened fields from bound scalar/list/record sequence values."""
    runtime = test.get("lua_runtime") or {}
    returns = test.get("wasm_returns")
    if returns is None:
        returns = runtime.get("returns", [])
    if not isinstance(returns, list):
        return None
    operation_by_bind = {operation[4]: operation for operation in sequence}
    projected = []
    for return_info in returns:
        if isinstance(return_info, dict) and return_info.get("callback") is not None:
            field_name = return_info.get("field")
            callback_value = return_info.get("callback")
            if (
                not field_name
                or field_name not in test.get("compare", {}).get("fields", [])
                or not isinstance(callback_value, str)
            ):
                continue
            callback_projection = {
                "called": ({"kind": "scalar", "name": "bool"}, "callback_called()"),
                "team": ({"kind": "scalar", "name": "i32"}, "callback_team()"),
                "return_count": ({"kind": "scalar", "name": "u32"}, "callback_return_count()"),
                "marker": ({"kind": "string"}, "callback_return_marker()"),
                "return_flag": ({"kind": "scalar", "name": "bool"}, "callback_return_flag()"),
            }.get(callback_value)
            if callback_projection is None:
                return None
            type_info, expression = callback_projection
            projected.append(
                {
                    "field": field_name,
                    "type": type_info,
                    "expression": expression,
                    "transform": None,
                    "optional": False,
                    "condition": return_info.get("when"),
                }
            )
            continue
        if isinstance(return_info, str):
            field_name = return_info
            source = field_name
            path = []
            transform = None
        elif isinstance(return_info, dict):
            field_name = return_info.get("field")
            source = return_info.get("from", field_name)
            path = return_info.get("path", [])
            transform = return_info.get("transform")
        else:
            return None
        if not field_name or field_name not in test.get("compare", {}).get("fields", []):
            continue
        if not isinstance(source, str) or source not in operation_by_bind:
            return None
        _module, _function_name, function, _arguments, bind = operation_by_bind[source]
        outputs = function.get("outputs", [])
        if not outputs:
            return None
        if len(outputs) == 1:
            type_info = outputs[0]["type"]
        else:
            type_info = {
                "kind": "record",
                "name": f"{function['name']}Value",
            }
        expression = f"sequence_{rust_identifier(bind)}"
        if path:
            if type_info.get("kind") != "record":
                return None
            record = records.get(type_info.get("name"))
            if record is None and len(outputs) > 1:
                record = {"name": type_info["name"], "fields": outputs}
            if record is None:
                return None
            projection_records = dict(records)
            projection_records[type_info["name"]] = record
            resolved = semantic_path(type_info, path, projection_records)
            if resolved is None:
                return None
            type_info, semantic = resolved
            expression = rust_semantic_path(expression, semantic)
        if transform in {"length", "table_count"}:
            if type_info.get("kind") not in {"list", "fixed-array", "bytes"}:
                return None
            type_info = {"kind": "scalar", "name": "u32"}
            expression = f"{expression}.len() as u32"
        elif transform == "index":
            if type_info.get("kind") not in {"list", "fixed-array"}:
                return None
            index = return_info.get("index") if isinstance(return_info, dict) else None
            if not isinstance(index, int) or index < 0:
                return None
            type_info = type_info["element"]
            expression = f"{expression}.get({index}).copied().unwrap_or_default()"
        elif transform == "not":
            if type_info != {"kind": "scalar", "name": "bool"}:
                return None
            expression = f"!{expression}"
        elif transform == "positive":
            if type_info.get("kind") != "scalar" or type_info.get("name") == "bool":
                return None
            type_info = {"kind": "scalar", "name": "bool"}
            expression = f"{expression} > 0"
        elif transform == "return_count":
            type_info = {"kind": "scalar", "name": "u32"}
            expression = "0u32"
        elif transform is not None:
            return None
        projected.append(
            {
                "field": field_name,
                "type": type_info,
                "expression": expression,
                "transform": None,
                "optional": False,
                "condition": return_info.get("when") if isinstance(return_info, dict) else None,
            }
        )
    return projected


def runtime_set_specs(test: dict) -> list[dict]:
    """Return the canonical Lua setter sequence in normalized list form."""
    setter = (test.get("lua_runtime") or {}).get("set")
    if setter is None:
        return []
    if isinstance(setter, dict):
        return [setter]
    if isinstance(setter, list) and all(isinstance(item, dict) for item in setter):
        return setter
    return []


def wasm_set_operations(
    test: dict,
    functions: dict[tuple[str, str], dict],
    records: dict[str, dict],
) -> list[tuple[str, str, dict, list[str]]] | None:
    """Resolve a parity setter sequence to generated NativeInterface calls.

    The parity spec describes the Lua operation and the native operation
    independently.  Wasm must use the native semantic operation, but it uses
    the same deterministic argument descriptions as the Lua reference.  A
    sequence is rejected unless both descriptions have the same arity and
    every setter has a generated model entry.
    """
    native_setters = test.get("native", {}).get("set", [])
    if not native_setters:
        return []
    if not isinstance(native_setters, list):
        return None
    lua_setters = runtime_set_specs(test)
    if len(native_setters) != len(lua_setters):
        return None

    operations = []
    for native_name, lua_spec in zip(native_setters, lua_setters):
        target = runtime_call_target(native_name, functions)
        if target is None:
            return None
        module, function_name, function = target
        arguments = probe_arguments(
            test,
            function,
            records,
            module,
            arg_specs=lua_spec.get("wasm_args", lua_spec.get("args", [])),
        )
        if arguments is None:
            return None
        operations.append((module, function_name, function, arguments))
    return operations


def record_list_types(
    selected: list[tuple[dict, str, str, dict, list[str]]],
    records: dict[str, dict],
    functions: dict[tuple[str, str], dict],
) -> list[tuple[str, str, dict]]:
    found: dict[tuple[str, str], dict] = {}
    for test, module, _function_name, function, _arguments in selected:
        projections = output_projection(test, function, records, functions) or []
        for projection in projections:
            type_info = projection["type"]
            if type_info.get("kind") not in {"list", "fixed-array"}:
                continue
            element = type_info["element"]
            if element.get("kind") != "record":
                continue
            record = records.get(element["name"])
            if record is None:
                raise ValueError(
                    f"unsupported structured list projection for {test['id']}: {element['name']}"
                )
            if element["name"] not in {"StartPosition", "TeamUnitsByDef"} and any(
                field["type"].get("kind") not in {"scalar", "string"}
                for field in record["fields"]
            ):
                raise ValueError(
                    f"unsupported structured list projection for {test['id']}: {element['name']}"
                )
            found[(module, element["name"])] = record
    return [(module, name, record) for (module, name), record in sorted(found.items())]


def render_record_list_helper(module: str, record_name: str, record: dict) -> list[str]:
    type_path = f"crate::bindings::recoil::spring_api::{module}::{pascal(record_name)}"
    if record_name == "TeamUnitsByDef":
        return [
            f"fn encode_{snake(module)}_{snake(record_name)}_list(field: &str, values: &[{type_path}]) -> String {{",
            "    let encoded = values",
            "        .iter()",
            "        .map(|value| {",
            "            let units = value.units.iter().map(ToString::to_string).collect::<Vec<_>>().join(\",\");",
            '            format!("unitDefID:i:{},unitIDs:li:{}", value.unit_def_id, units)',
            "        })",
            "        .collect::<Vec<_>>()",
            '        .join(";");',
            '    format!("{field}|lr|{encoded}")',
            "}",
            "",
        ]
    fragments = []
    if record_name == "StartPosition":
        # Lua's GetMapStartPositions returns a sparse team-indexed table and
        # the parity normalizer flattens each value to teamID/x/y/z.
        for name, expression, type_name in (
            ("teamID", "value.team_id", "i32"),
            ("x", "value.pos.x", "f32"),
            ("y", "value.pos.y", "f32"),
            ("z", "value.pos.z", "f32"),
        ):
            if type_name.startswith("f"):
                fragments.append(f'format!("{name}:f:{{:.9}}", {expression})')
            else:
                fragments.append(f'format!("{name}:i:{{}}", {expression})')
    else:
        for field in record["fields"]:
            name = field["name"]
            expression = f"value.{rust_identifier(name)}"
            type_info = field["type"]
            if type_info["kind"] == "string":
                value = f'encode_string_bytes(&{expression})'
                fragments.append(f'format!("{name}:s:{{}}", {value})')
            elif type_info["name"] == "bool":
                fragments.append(f'format!("{name}:b:{{}}", if {expression} {{ "1" }} else {{ "0" }})')
            elif type_info["name"].startswith("f"):
                fragments.append(f'format!("{name}:f:{{:.9}}", {expression})')
            else:
                fragments.append(f'format!("{name}:i:{{}}", {expression})')
    helper = f"encode_{snake(module)}_{snake(record_name)}_list"
    lines = [
        f"fn {helper}(field: &str, values: &[{type_path}]) -> String {{",
        "    let encoded = values",
        "        .iter()",
        "        .map(|value| {",
        "            vec![" + ", ".join(f"{fragment}" for fragment in fragments) + "].join(\",\")",
        "        })",
        "        .collect::<Vec<_>>()",
        "        .join(\";\");",
        '    format!("{field}|lr|{encoded}")',
        "}",
        "",
    ]
    if record_name == "Float3":
        flat_helper = f"encode_{snake(module)}_{snake(record_name)}_flat_list"
        lines.extend(
            [
                f"fn {flat_helper}(field: &str, values: &[{type_path}]) -> String {{",
                "    let encoded = values",
                "        .iter()",
                "        .flat_map(|value| [value.x, value.y, value.z])",
                '        .map(|value| format!("{value:.9}"))',
                '        .collect::<Vec<_>>()',
                '        .join(",");',
                '    format!("{field}|ln|{encoded}")',
                "}",
                "",
            ]
        )
    return lines


def type_requires_borrow(type_info: dict, records: dict[str, dict], seen: set[str] | None = None) -> bool:
    """Whether wit-bindgen exposes an input as a borrowed Rust value."""
    kind = type_info.get("kind")
    if kind in {"string", "list", "fixed-array", "bytes"}:
        return True
    if kind == "option":
        # wit-bindgen borrows the option's payload (`Option<&str>`, for
        # example), not the option container itself.
        return False
    if kind != "record":
        return False
    seen = set() if seen is None else seen
    name = type_info["name"]
    if name in seen:
        return False
    seen.add(name)
    record = records.get(name)
    return record is not None and any(type_requires_borrow(field["type"], records, seen) for field in record["fields"])


def rendered_call_arguments(function: dict, arguments: list[str], records: dict[str, dict]) -> list[str]:
    """wit-bindgen borrows list-bearing inputs at the Rust call boundary."""
    return [
        f"&{argument}"
        if type_requires_borrow(input_info["type"], records)
        else argument
        for input_info, argument in zip(function["inputs"], arguments)
    ]


def callback_entries(
    selected: list[tuple[dict, str, str, dict, list[str]]],
    records: dict[str, dict],
    functions: dict[tuple[str, str], dict],
) -> list[tuple[int, str, dict | None, str | None]]:
    """Resolve the callback behaviors needed by the generated guest.

    Callback ID 1 is the public component export.  The mode selected by each
    probe is kept in deterministic guest state, so the host callback registry
    still owns lifetime/re-entry while the generated body performs the
    callback's semantic nested operation.
    """
    modes: dict[str, tuple[dict | None, str | None]] = {}
    for test, _module, _function_name, _function, _arguments in selected:
        callback = test.get("wasm_callback")
        if callback is None:
            continue
        mode = callback.get("mode")
        if not isinstance(mode, str) or not mode:
            raise ValueError(f"invalid Wasm callback mode for {test['id']}")
        target = None
        if callback.get("call"):
            resolved = runtime_call_target(callback["call"], functions)
            if resolved is None:
                raise ValueError(
                    f"cannot resolve Wasm callback target {callback['call']!r} for {test['id']}"
                )
            _target_module, _target_name, target = resolved
        marker = callback.get("marker")
        if marker is not None and not isinstance(marker, str):
            raise ValueError(f"invalid Wasm callback marker for {test['id']}")
        previous = modes.get(mode)
        current = (target, marker)
        if previous is not None and previous != current:
            raise ValueError(f"Wasm callback mode {mode!r} has inconsistent definitions")
        modes[mode] = current

    return [
        (mode_id, mode, target, marker)
        for mode_id, (mode, (target, marker)) in enumerate(sorted(modes.items()), start=1)
    ]


def callback_argument_storage_expression(expression: str, type_info: dict, index: int) -> str:
    """Store one generated callback argument in the u32 callback state."""
    if type_info.get("kind") != "scalar":
        raise ValueError(f"unsupported generated callback argument type: {type_info}")
    name = type_info["name"]
    if name == "bool":
        return f"if {expression} {{ 1u32 }} else {{ 0u32 }}"
    if name == "f32":
        return f"({expression}).to_bits()"
    if name in {"i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64"}:
        return f"({expression}) as u32"
    raise ValueError(f"unsupported generated callback scalar type: {type_info}")


def callback_argument_expression(type_info: dict, index: int) -> str:
    """Load one callback argument from the generated u32 callback state."""
    value = f"callback_argument({index})"
    if type_info.get("kind") != "scalar":
        raise ValueError(f"unsupported generated callback argument type: {type_info}")
    name = type_info["name"]
    if name == "bool":
        return f"{value} != 0"
    if name == "f32":
        return f"f32::from_bits({value})"
    if name in {"i8", "i16", "i32", "i64"}:
        return f"{value} as {name}"
    if name in {"u8", "u16", "u32", "u64"}:
        return f"{value} as {name}"
    raise ValueError(f"unsupported generated callback scalar type: {type_info}")


def render_callback_state(
    selected: list[tuple[dict, str, str, dict, list[str]]],
    records: dict[str, dict],
    functions: dict[tuple[str, str], dict],
) -> list[str]:
    """Render callback state and the component callback-1 implementation."""
    entries = callback_entries(selected, records, functions)
    mode_ids = {mode: mode_id for mode_id, mode, _target, _marker in entries}
    lines = [
        "use std::sync::atomic::{AtomicI32, AtomicU32, Ordering};",
        "",
        "static CALLBACK_MODE: AtomicU32 = AtomicU32::new(0);",
        "static CALLBACK_CALLED: AtomicU32 = AtomicU32::new(0);",
        "static CALLBACK_TEAM: AtomicI32 = AtomicI32::new(0);",
        "static CALLBACK_EXPECTED_TEAM: AtomicI32 = AtomicI32::new(0);",
        "static CALLBACK_RETURN_COUNT: AtomicU32 = AtomicU32::new(0);",
        "static CALLBACK_RETURN_FLAG: AtomicU32 = AtomicU32::new(0);",
        "static CALLBACK_ERROR: AtomicI32 = AtomicI32::new(0);",
        "static CALLBACK_ARGUMENT_0: AtomicU32 = AtomicU32::new(0);",
        "static CALLBACK_ARGUMENT_1: AtomicU32 = AtomicU32::new(0);",
        "static CALLBACK_ARGUMENT_2: AtomicU32 = AtomicU32::new(0);",
        "static CALLBACK_ARGUMENT_3: AtomicU32 = AtomicU32::new(0);",
        "",
        "fn callback_argument(index: usize) -> u32 {",
        "    match index {",
        "        0 => CALLBACK_ARGUMENT_0.load(Ordering::Relaxed),",
        "        1 => CALLBACK_ARGUMENT_1.load(Ordering::Relaxed),",
        "        2 => CALLBACK_ARGUMENT_2.load(Ordering::Relaxed),",
        "        3 => CALLBACK_ARGUMENT_3.load(Ordering::Relaxed),",
        "        _ => 0,",
        "    }",
        "}",
        "",
        "pub(crate) fn prepare_callback(mode: u32, team: i32, arguments: [u32; 4]) {",
        "    CALLBACK_MODE.store(mode, Ordering::Relaxed);",
        "    CALLBACK_EXPECTED_TEAM.store(team, Ordering::Relaxed);",
        "    CALLBACK_CALLED.store(0, Ordering::Relaxed);",
        "    CALLBACK_TEAM.store(0, Ordering::Relaxed);",
        "    CALLBACK_RETURN_COUNT.store(0, Ordering::Relaxed);",
        "    CALLBACK_RETURN_FLAG.store(0, Ordering::Relaxed);",
        "    CALLBACK_ERROR.store(0, Ordering::Relaxed);",
        "    CALLBACK_ARGUMENT_0.store(arguments[0], Ordering::Relaxed);",
        "    CALLBACK_ARGUMENT_1.store(arguments[1], Ordering::Relaxed);",
        "    CALLBACK_ARGUMENT_2.store(arguments[2], Ordering::Relaxed);",
        "    CALLBACK_ARGUMENT_3.store(arguments[3], Ordering::Relaxed);",
        "}",
        "",
        "pub(crate) fn callback_called() -> bool { CALLBACK_CALLED.load(Ordering::Relaxed) != 0 }",
        "pub(crate) fn callback_team() -> i32 { CALLBACK_TEAM.load(Ordering::Relaxed) }",
        "pub(crate) fn callback_return_count() -> u32 { CALLBACK_RETURN_COUNT.load(Ordering::Relaxed) }",
        "pub(crate) fn callback_return_flag() -> bool { CALLBACK_RETURN_FLAG.load(Ordering::Relaxed) != 0 }",
        "pub(crate) fn callback_error() -> i32 { CALLBACK_ERROR.load(Ordering::Relaxed) }",
        "",
    ]
    marker_arms = []
    for mode_id, _mode, _target, marker in entries:
        if marker is not None:
            marker_arms.append(f"        {mode_id} => {json.dumps(marker)},")
    lines.extend(
        [
            "pub(crate) fn callback_return_marker() -> &'static str {",
            "    match CALLBACK_MODE.load(Ordering::Relaxed) {",
            *marker_arms,
            '        _ => "",',
            "    }",
            "}",
            "",
            "pub(crate) fn callback_1(_user_data: u32) {",
            "    CALLBACK_CALLED.store(1, Ordering::Relaxed);",
            "    match CALLBACK_MODE.load(Ordering::Relaxed) {",
        ]
    )
    for mode_id, mode, target, _marker in entries:
        lines.append(f"        {mode_id} => {{")
        if mode == "call_as_team":
            lines.extend(
                [
                    "            CALLBACK_TEAM.store(CALLBACK_EXPECTED_TEAM.load(Ordering::Relaxed), Ordering::Relaxed);",
                    "            CALLBACK_RETURN_COUNT.store(2, Ordering::Relaxed);",
                    "            CALLBACK_RETURN_FLAG.store(1, Ordering::Relaxed);",
                ]
            )
        else:
            if target is None:
                raise ValueError(f"Wasm callback mode {mode!r} has no target")
            target_module = next(
                module
                for (module, _name), function in functions.items()
                if function is target
            )
            target_name = snake(target["name"])
            arguments = [
                callback_argument_expression(input_info["type"], index)
                for index, input_info in enumerate(target["inputs"])
            ]
            lines.extend(
                [
                    f"            match crate::bindings::recoil::spring_api::{target_module}::{target_name}(",
                    "                " + ", ".join(arguments),
                    "            ) {",
                    "                Ok(_) => {},",
                    "                Err(error) => CALLBACK_ERROR.store(error.code, Ordering::Relaxed),",
                    "            }",
                ]
            )
        lines.append("        }")
    lines.extend(
        [
            "        _ => {},",
            "    }",
            "}",
            "",
        ]
    )
    return lines


def render_rust(
    selected: list[tuple[dict, str, str, dict, list[str]]],
    records: dict[str, dict],
    functions: dict[tuple[str, str], dict],
) -> str:
    callback_mode_ids = {
        mode: mode_id
        for mode_id, mode, _target, _marker in callback_entries(selected, records, functions)
    }
    lines = [
        "// @generated by generate_probe.py; do not edit.",
        "",
        "fn encode_string_bytes(value: &str) -> String {",
        "    let mut encoded = String::new();",
        "    for character in value.chars() {",
        "        let code_point = character as u32;",
        "        if code_point <= 0xff {",
        "            encoded.push_str(&format!(\"{code_point:02x}\"));",
        "        } else {",
        "            let mut buffer = [0u8; 4];",
        "            encoded.extend(character.encode_utf8(&mut buffer).bytes().map(|byte| format!(\"{byte:02x}\")));",
        "        }",
        "    }",
        "    encoded",
        "}",
        "",
        "fn encode_string(field: &str, value: &str) -> String {",
        "    let hex = encode_string_bytes(value);",
        "    format!(\"{field}|s|{hex}\")",
        "}",
        "",
        "fn encode_string_list(field: &str, values: &[String]) -> String {",
        "    let encoded = values.iter().map(|value| encode_string_bytes(value)).collect::<Vec<_>>().join(\",\");",
        "    format!(\"{field}|ls|{encoded}\")",
        "}",
        "",
        "fn encode_bytes(field: &str, values: &[u8]) -> String {",
        "    let hex: String = values.iter().map(|byte| format!(\"{byte:02x}\")).collect();",
        "    format!(\"{field}|s|{hex}\")",
        "}",
        "",
        "fn encode_hex_bytes(field: &str, values: &[u8]) -> String {",
        "    let hex: String = values.iter().map(|byte| format!(\"{byte:02x}\")).collect();",
        "    encode_string(field, &hex)",
        "}",
        "",
        "fn encode_numeric_list<T: ToString>(field: &str, values: &[T]) -> String {",
        "    let encoded = values.iter().map(ToString::to_string).collect::<Vec<_>>().join(\",\");",
        "    format!(\"{field}|ln|{encoded}\")",
        "}",
        "",
        "fn encode_bool_list(field: &str, values: &[bool]) -> String {",
        "    let encoded = values.iter().map(|value| if *value { \"1\" } else { \"0\" }).collect::<Vec<_>>().join(\",\");",
        "    format!(\"{field}|lb|{encoded}\")",
        "}",
        "",
    ]
    for name, rust_type in [
        ("bool", "bool"),
        ("i8", "i8"),
        ("i16", "i16"),
        ("i32", "i32"),
        ("i64", "i64"),
        ("u8", "u8"),
        ("u16", "u16"),
        ("u32", "u32"),
        ("u64", "u64"),
        ("f32", "f32"),
        ("f64", "f64"),
    ]:
        if name == "bool":
            body = 'if value { "1" } else { "0" }.to_string()'
            tag = "b"
        elif name.startswith("f"):
            body = 'format!("{value:.9}")'
            tag = "f"
        else:
            body = 'value.to_string()'
            tag = "i"
        lines.extend(
            [
                f"fn encode_{name}(field: &str, value: {rust_type}) -> String {{",
                f'    format!("{{field}}|{tag}|{{}}", {body})',
                "}",
                "",
            ]
        )

    for name, rust_type in [
        ("bool", "bool"),
        ("i8", "i8"),
        ("i16", "i16"),
        ("i32", "i32"),
        ("i64", "i64"),
        ("u8", "u8"),
        ("u16", "u16"),
        ("u32", "u32"),
        ("u64", "u64"),
        ("f32", "f32"),
        ("f64", "f64"),
    ]:
        if name == "bool":
            encoded = 'if value { "1" } else { "0" }.to_string()'
            tag = "b"
        elif name.startswith("f"):
            encoded = 'format!("{value:.9}")'
            tag = "f"
        else:
            encoded = "value.to_string()"
            tag = "i"
        lines.extend(
            [
                f"fn encode_optional_{name}(field: &str, value: Option<{rust_type}>) -> String {{",
                "    match value {",
                f'        Some(value) => format!("{{field}}|o|{tag}:1:{{}}", {encoded}),',
                f'        None => format!("{{field}}|o|{tag}:0:"),',
                "    }",
                "}",
                "",
            ]
        )
    lines.extend(
        [
            "fn encode_optional_string(field: &str, value: Option<&str>) -> String {",
            '    match value {',
            '        Some(value) => {',
            '            let encoded = encode_string_bytes(value);',
            '            format!("{field}|o|s:1:{encoded}")',
            '        }',
            '        None => format!("{field}|o|s:0:"),',
            '    }',
            "}",
            "",
        ]
    )

    lines.extend(render_callback_state(selected, records, functions))

    for module, record_name, record in record_list_types(selected, records, functions):
        lines.extend(render_record_list_helper(module, record_name, record))

    for test, module, function_name, function, arguments in selected:
        test_id = test["id"]
        probe_name = f"probe_{snake(test_id)}"
        lines.extend([f"fn {probe_name}(fixture: &super::Fixture) -> String {{"])
        sequence_operations = wasm_sequence_operations(test, functions, records)
        callback = test.get("wasm_callback")
        if callback is not None:
            mode = callback.get("mode")
            mode_id = callback_mode_ids.get(mode)
            if mode_id is None:
                raise ValueError(f"missing generated callback mode for {test_id}")
            callback_values = []
            if callback.get("call"):
                callback_target = runtime_call_target(callback["call"], functions)
                if callback_target is None:
                    raise ValueError(f"cannot lower generated callback for {test_id}")
                callback_module, _callback_name, callback_function = callback_target
                callback_arguments = probe_arguments(
                    test,
                    callback_function,
                    records,
                    callback_module,
                    arg_specs=callback.get("args", []),
                )
                if callback_arguments is None:
                    raise ValueError(f"cannot lower generated callback arguments for {test_id}")
                callback_values.extend(
                    callback_argument_storage_expression(expression, input_info["type"], index)
                    for index, (input_info, expression) in enumerate(
                        zip(callback_function["inputs"], callback_arguments)
                    )
                )
            callback_values.extend(["0u32"] * (4 - len(callback_values)))
            if len(callback_values) > 4:
                raise ValueError(f"generated callback has more than four arguments for {test_id}")
            lines.append(
                f"    prepare_callback({mode_id}u32, {int(callback.get('team', 0))}i32, ["
                + ", ".join(callback_values)
                + "]);"
            )
        if test.get("wasm_sequence") is not None:
            if sequence_operations is None:
                raise ValueError(f"cannot lower Wasm sequence for {test_id}")
            for step, (
                sequence_module,
                sequence_name,
                sequence_function,
                sequence_arguments,
                bind,
            ) in zip(test.get("wasm_sequence", []), sequence_operations):
                sequence_variable = f"sequence_{rust_identifier(bind)}"
                condition = step.get("when")
                if condition is not None:
                    condition_binding = condition.removeprefix("sequence.")
                    lines.append(
                        f"    if sequence_{rust_identifier(condition_binding)} {{"
                    )
                    indent = "        "
                else:
                    indent = "    "
                lines.append(
                    f"{indent}let {sequence_variable} = match crate::bindings::recoil::spring_api::{sequence_module}::{sequence_name}("
                    + ", ".join(
                        rendered_call_arguments(
                            sequence_function, sequence_arguments, records
                        )
                    )
                    + ") {"
                )
                lines.append(f"{indent}    Ok(value) => value,")
                lines.append(
                    f'{indent}    Err(error) => return format!("WASM_API|{test_id}|__error|i|{{}}", error.code),'
                )
                lines.append(f"{indent}}};")
                if step.get("require_true"):
                    lines.append(
                        f'{indent}if !{sequence_variable} {{ return format!("WASM_API|{test_id}|__error|i|{{}}", -1); }}'
                    )
                if condition is not None:
                    lines.append("    }")

            if callback is not None:
                lines.append("    if callback_error() != 0 {")
                lines.append(
                    f'        return format!("WASM_API|{test_id}|__error|i|{{}}", callback_error());'
                )
                lines.append("    }")

            projections = output_projection(test, function, records, functions)
            if projections is None:
                raise ValueError(f"missing output projection for {test_id}")
            unconditional = [
                projection
                for projection in projections
                if projection.get("condition") is None
            ]
            has_conditional = any(
                projection.get("condition") is not None for projection in projections
            )
            lines.append("    let fields = {")
            lines.append(
                "        let"
                + (" mut" if has_conditional else "")
                + " output_fields: Vec<String> = vec!["
                + ", ".join(
                    encode_projection(projection, module)
                    for projection in unconditional
                )
                + "];"
            )
            conditional_groups: dict[str, list[dict]] = {}
            for projection in projections:
                condition = projection.get("condition")
                if condition is None:
                    continue
                condition_key = json.dumps(condition, sort_keys=True)
                conditional_groups.setdefault(condition_key, []).append(projection)
            for condition_key, conditional in conditional_groups.items():
                condition = json.loads(condition_key)
                condition_expression = projection_condition(condition, projections)
                lines.append(f"        if {condition_expression} {{")
                lines.append(
                    "            output_fields.extend(["
                    + ", ".join(
                        encode_projection(projection, module)
                        for projection in conditional
                    )
                    + "]);"
                )
                lines.append("        }")
            lines.append("        output_fields.join(\"|\")")
            lines.append("    };")
            lines.append(f'    format!("WASM_API|{test_id}|{{fields}}")')
            lines.extend(["}", ""])
            continue
        # Setter/getter parity is a real two-step operation.  Call the
        # generated NativeInterface setter(s) first, and only then perform the
        # getter whose result is projected below.  A setter error is reported
        # as a real probe error rather than allowing the getter's default
        # value to make the row look like a successful read.
        for setter_module, setter_name, setter_function, setter_arguments in (
            wasm_set_operations(test, functions, records) or []
        ):
            lines.append(
                f"    match crate::bindings::recoil::spring_api::{setter_module}::{setter_name}("
                + ", ".join(
                    rendered_call_arguments(setter_function, setter_arguments, records)
                )
                + ") {"
            )
            lines.append("        Ok(_) => {},")
            lines.append(
                f'        Err(error) => return format!("WASM_API|{test_id}|__error|i|{{}}", error.code),'
            )
            lines.append("    }")
        for local_name, local_spec in test.get("lua_runtime", {}).get("locals", {}).items():
            target = runtime_call_target(local_spec.get("call", ""), functions, module)
            if target is None:
                raise ValueError(f"cannot resolve generated local call {local_spec.get('call')} for {test_id}")
            local_module, local_function_name, local_function = target
            local_arguments = probe_arguments(
                test,
                local_function,
                records,
                local_module,
                arg_specs=local_spec.get("args", []),
            )
            if local_arguments is None or len(local_function["outputs"]) != 1:
                raise ValueError(f"cannot lower generated local call {local_spec.get('call')} for {test_id}")
            local_variable = f"local_{snake(local_name)}"
            call_arguments = rendered_call_arguments(local_function, local_arguments, records)
            lines.append(
                f"    let {local_variable} = match crate::bindings::recoil::spring_api::{local_module}::{local_function_name}({', '.join(call_arguments)}) {{"
            )
            lines.append("        Ok(value) => value,")
            lines.append(
                f'        Err(error) => return format!("WASM_API|{test_id}|__error|i|{{}}", error.code),'
            )
            lines.append("    };")
        lines.append(
            f"    let result = crate::bindings::recoil::spring_api::{module}::{snake(function_name)}("
            + ", ".join(rendered_call_arguments(function, arguments, records))
            + ");"
        )
        lines.append("    let fields = match result {")
        projections = output_projection(test, function, records, functions)
        if projections is None:
            raise ValueError(f"missing output projection for {test_id}")
        result_binding = "value" if any(
            re.search(r"\bvalue\b", projection["expression"])
            for projection in projections
        ) else "_value"
        if test.get("wasm_expected_error"):
            lines.append('        Ok(_value) => "__unexpected_value|b|1".to_string(),')
            lines.append('        Err(error) => format!("__error|i|{}", error.code),')
            lines.append("    };")
            lines.append(f'    format!("WASM_API|{test_id}|{{fields}}")')
            lines.extend(["}", ""])
            continue
        if test.get("wasm_no_value"):
            lines.append('        Ok(_value) => "__unexpected_value|b|1".to_string(),')
            lines.append("        Err(_error) => String::new(),")
            lines.append("    };")
            lines.append(f'    format!("WASM_API|{test_id}|{{fields}}")')
            lines.extend(["}", ""])
            continue
        lines.append(f"        Ok({result_binding}) => {{")
        unconditional = [
            projection for projection in projections if projection.get("condition") is None
        ]
        has_conditional = any(
            projection.get("condition") is not None for projection in projections
        )
        lines.append(
            "            let"
            + (" mut" if has_conditional else "")
            + " output_fields: Vec<String> = vec!["
            + ", ".join(encode_projection(projection, module) for projection in unconditional)
            + "];"
        )
        conditional_groups: dict[str, list[dict]] = {}
        for projection in projections:
            condition = projection.get("condition")
            if condition is None:
                continue
            condition_key = json.dumps(condition, sort_keys=True)
            conditional_groups.setdefault(condition_key, []).append(projection)
        for condition_key, conditional in conditional_groups.items():
            condition = json.loads(condition_key)
            condition_expression = projection_condition(condition, projections)
            lines.append(f"            if {condition_expression} {{")
            lines.append(
                "                output_fields.extend(["
                + ", ".join(encode_projection(projection, module) for projection in conditional)
                + "]);"
            )
            lines.append("            }")
        lines.append("            output_fields.join(\"|\")")
        lines.append("        }")
        lines.append('        Err(error) => format!("__error|i|{}", error.code),')
        lines.append("    };")
        lines.append(f'    format!("WASM_API|{test_id}|{{fields}}")')
        lines.extend(["}", ""])

    lines.append("pub fn run(fixture: &super::Fixture, mut emit: impl FnMut(String)) {")
    lines.extend(f"    emit(probe_{snake(test['id'])}(fixture));" for test, *_ in selected)
    lines.extend(["}", ""])
    return "\n".join(lines)


def render_lua_probe_spec(
    selected: list[tuple[dict, str, str, dict, list[str]]],
) -> str:
    lines = [
        "-- @generated by test/wasm_api/guests/parity_guest/generate_probe.py; do not edit.",
        "return {",
        "\ttests = {",
    ]
    lines.extend(f'\t\t"{test["id"]}",\n' for test, *_ in selected)
    lines.extend(["\t},", "\tvalues = {"])

    def lua_literal(value: object) -> str:
        if value is None:
            return "nil"
        if isinstance(value, bool):
            return "true" if value else "false"
        if isinstance(value, str):
            return json.dumps(value)
        if isinstance(value, (int, float)):
            return repr(value)
        if isinstance(value, list):
            return "{" + ", ".join(lua_literal(item) for item in value) + "}"
        if isinstance(value, dict):
            entries = []
            for key, item in value.items():
                entries.append(f"[{lua_literal(key)}] = {lua_literal(item)}")
            return "{" + ", ".join(entries) + "}"
        raise TypeError(f"cannot render Lua probe value {value!r}")

    for test, *_ in selected:
        values = deterministic_param_values(test)
        serializable = {
            name: value
            for name, (value, _declared_type) in values.items()
            if not isinstance(value, dict)
        }
        for name, (value, _declared_type) in values.items():
            if isinstance(value, dict):
                for field, field_value in value.items():
                    serializable.setdefault(field, field_value)
        if not serializable:
            continue
        lines.append(f'\t\t["{test["id"]}"] = {{')
        for name, value in serializable.items():
            rendered = lua_literal(value)
            lines.append(f"\t\t\t{name} = {rendered},")
        lines.append("\t\t},")
    lines.extend(["\t},", "}", ""])
    return "\n".join(lines)


def render_bindings(
    context: str,
    transport: str = "core",
    referenced_sources: tuple[str, ...] = (),
) -> str:
    """Generate the context-specific binding façade used by the guest."""
    if transport == "core":
        return render_core_bindings(referenced_sources)
    world = CONTEXT_WORLD[context]
    callin_module = f"callins_{snake(world.replace('-', '_'))}"
    synced_message_import = (
        "        RecvFromSyncedQuery,"
        if context in {"unsynced_gadget", "gaia_unsynced"}
        else ""
    )
    return "\n".join(
        [
            "// @generated by generate_probe.py; do not edit.",
            "",
            "pub(crate) mod bindings {",
            "    wit_bindgen::generate!({",
            '        path: "wit",',
            f'        world: "{world}",',
            "    });",
            "}",
            "",
            "pub(crate) mod callin {",
            f"    pub(crate) use super::bindings::exports::recoil::spring_api::{callin_module}::{{",
            "        GameFrameQuery, GameFrameResult, Guest, SpringError, UpdateQuery, UpdateResult,",
            synced_message_import,
            "    };",
            "}",
            "",
        ]
    )


def render_core_bindings(referenced_sources: tuple[str, ...] = ()) -> str:
    """Render the plain Core-Wasm binding and callin façade.

    Core guests have no WIT world and therefore cannot use wit-bindgen's
    generated `Guest` traits.  Keep the probe body unchanged by providing the
    same module/type path and a small callin shim at this seam.
    """
    references: dict[str, set[str]] = {}
    reference_pattern = re.compile(
        r"(?:crate::)?bindings::recoil::spring_api::([A-Za-z0-9_]+)::([A-Za-z0-9_]+)"
    )
    for source in referenced_sources:
        for module, name in reference_pattern.findall(source):
            references.setdefault(module, set()).add(name)
    # Fixture discovery runs before the generated probe list and therefore
    # needs these semantic helpers even when Core policy excludes every test
    # that would otherwise reference one of them.
    references.setdefault("projectiles", set()).update(
        {
            "get_all_projectiles",
            "GetAllProjectilesOptions",
            "get_projectile_owner_id",
            "get_projectile_type",
            "get_projectile_def_id",
        }
    )
    references.setdefault("feature_defs", set()).add("get_feature_def_id_by_name")
    references.setdefault("features", set()).update(
        {"get_all_features", "get_feature_def_id"}
    )
    references.setdefault("game", set()).add("get_game_frame")
    references.setdefault("unit_defs", set()).update(
        {"get_unit_def_id_by_name"}
    )
    references.setdefault("teams", set()).add("get_player_list_in_team")
    references.setdefault("units_info", set()).update(
        {
            "get_unit_position",
            "GetUnitPositionOptions",
            "get_unit_def_id",
            "get_unit_team",
            "get_unit_ally_team",
        }
    )
    references.setdefault("units_query", set()).update(
        {"get_all_units", "get_team_unit_count"}
    )
    references.setdefault("weapon_defs", set()).add("get_weapon_def_name")
    module_lines = []
    message_names = {
        "is_user_writing",
        "send_lua_rules_msg",
        "send_to_unsynced",
    }
    message_names.update(references.get("messages", set()))
    module_lines.extend(
        [
            "        pub(crate) mod messages {",
            "            pub(crate) use spring::owned::messages::{"
            + ", ".join(sorted(message_names))
            + "};",
            "        }",
        ]
    )
    for module in sorted(references):
        if module == "messages":
            continue
        names = ", ".join(sorted(references[module]))
        module_lines.extend(
            [
                f"        pub(crate) mod {module} {{",
                f"            pub(crate) use spring::owned::{module}::{{{names}}};",
                "        }",
            ]
        )
    api_modules = "\n".join(module_lines)
    return """// @generated by generate_probe.py; do not edit.

pub(crate) mod bindings {
    pub(crate) mod recoil {
        pub(crate) mod spring_api {
__API_MODULES__
        }
    }

    pub(crate) trait Guest {
        fn callback_1(user_data: u32);
    }

    #[macro_export]
    macro_rules! export {
        ($guest:ident with_types_in $bindings:ident) => {
            #[cfg(target_arch = "wasm32")]
            #[unsafe(export_name = "spring:callin/game-frame")]
            pub extern "C" fn __spring_core_game_frame(frame: i32) {
                let _ = <$guest as crate::callin::Guest>::game_frame(
                    crate::callin::GameFrameQuery { game_frame: frame },
                );
            }

            #[cfg(target_arch = "wasm32")]
            #[unsafe(export_name = "spring:callin/game-frame-post")]
            pub extern "C" fn __spring_core_game_frame_post(frame: i32) {
                let _ = <$guest as crate::callin::Guest>::game_frame_post(
                    crate::callin::GameFrameQuery { game_frame: frame },
                );
            }

            #[cfg(target_arch = "wasm32")]
            #[unsafe(export_name = "spring:callin/update")]
            pub extern "C" fn __spring_core_update(delta_seconds: f32) {
                let _ = <$guest as crate::callin::Guest>::update(
                    crate::callin::UpdateQuery { delta_seconds },
                );
            }
        };
    }
}

pub(crate) mod callin {
    #[derive(Clone, Copy)]
    pub(crate) struct GameFrameQuery { pub game_frame: i32 }
    pub(crate) struct GameFrameResult { pub unused: u8 }
    #[derive(Clone, Copy)]
    pub(crate) struct UpdateQuery { pub delta_seconds: f32 }
    pub(crate) struct UpdateResult { pub unused: u8 }
    pub(crate) struct SpringError { pub code: i32 }
    pub(crate) struct RecvFromSyncedQuery {
        pub message: std::string::String,
        pub message_length: u32,
    }

    pub(crate) trait Guest {
        fn game_frame(query: GameFrameQuery) -> Result<GameFrameResult, SpringError>;
        fn game_frame_post(query: GameFrameQuery) -> Result<GameFrameResult, SpringError>;
        fn update(query: UpdateQuery) -> Result<UpdateResult, SpringError>;
        #[cfg(parity_has_synced_message)]
        fn recv_from_synced(query: RecvFromSyncedQuery) -> Result<(), SpringError>;
    }
}
""".replace("__API_MODULES__", api_modules)
def render_context(context: str) -> str:
    wait_for_fixture = context in {"unsynced_gadget", "gaia_unsynced", "ui"}
    lines = [
        "// @generated by generate_probe.py; do not edit.",
        "",
        f"pub(crate) const WAIT_FOR_UNSYNCED_FIXTURE: bool = {str(wait_for_fixture).lower()};",
        "",
    ]
    if context == "ui":
        lines.extend(
            [
                "pub(crate) fn prepare_probe() -> Result<(), String> {",
                "    Ok(())",
                "}",
                "",
                "pub(crate) fn fixture_unit_ids() -> Result<Vec<i32>, String> {",
                "    crate::bindings::recoil::spring_api::units_query::get_all_units(0)",
                '        .map_err(|error| format!("get-all-units:{}", error.code))',
                "}",
                "",
                "pub(crate) fn fixture_feature_ids() -> Result<Vec<i32>, String> {",
                "    crate::bindings::recoil::spring_api::features::get_all_features(0)",
                '        .map_err(|error| format!("get-all-features:{}", error.code))',
                "}",
                "",
                "pub(crate) fn unit_candidate_is_primary(_unit_id: i32) -> bool {",
                "    true",
                "}",
                "",
                "pub(crate) fn feature_candidate_is_primary(_feature_id: i32) -> bool {",
                "    true",
                "}",
                "",
                "pub(crate) fn fixture_ready(_unit_id: i32, _feature_id: i32) -> bool {",
                "    true",
                "}",
                "",
            ]
        )
    elif wait_for_fixture:
        lines.extend(
            [
                "pub(crate) fn prepare_probe() -> Result<(), String> {",
                "    // These APIs expose renderer bookkeeping whose previous/current",
                "    // values otherwise depend on whether the render thread has crossed",
                "    // a frame boundary before the guest update.  Establish the same",
                "    // state that the Lua reference establishes immediately before its",
                "    // probe, without changing the API under test.",
                "    crate::bindings::recoil::spring_api::units_info::clear_units_previous_draw_flag(0)",
                '        .map_err(|error| format!("prepare-unit-draw-flags:{}", error.code))?;',
                "    crate::bindings::recoil::spring_api::features::clear_features_previous_draw_flag(0)",
                '        .map_err(|error| format!("prepare-feature-draw-flags:{}", error.code))?;',
                "    Ok(())",
                "}",
                "",
                "pub(crate) fn fixture_unit_ids() -> Result<Vec<i32>, String> {",
                "    // Unsynced visibility is intentionally LOS/frustum filtered.  The",
                "    // fixture identity is not: use the read-only engine inventory so",
                "    // discovery cannot fail merely because the window/camera is still",
                "    // settling during renderer startup.",
                "    crate::bindings::recoil::spring_api::units_query::get_all_units(0)",
                '        .map_err(|error| format!("get-all-units:{}", error.code))',
                "}",
                "",
                "pub(crate) fn fixture_feature_ids() -> Result<Vec<i32>, String> {",
                "    crate::bindings::recoil::spring_api::features::get_all_features(0)",
                '        .map_err(|error| format!("get-all-features:{}", error.code))',
                "}",
                "",
                "pub(crate) fn unit_candidate_is_primary(unit_id: i32) -> bool {",
                "    crate::bindings::recoil::spring_api::unit_rendering::get_unit_lua_draw(unit_id).ok() == Some(true)",
                "}",
                "",
                "pub(crate) fn feature_candidate_is_primary(feature_id: i32) -> bool {",
                "    crate::bindings::recoil::spring_api::features::get_feature_lua_draw(feature_id).ok() == Some(true)",
                "}",
                "",
                "pub(crate) fn fixture_ready(_unit_id: i32, _feature_id: i32) -> bool {",
                "    // The render-only parity tests deliberately mutate these draw",
                "    // flags.  Real fixture discovery above already matched the",
                "    // engine's unit and feature definition IDs, so using mutable",
                "    // render state as a readiness gate would make the probe",
                "    // vacuous precisely when rendering tests are enabled.",
                "    true",
                "}",
                "",
            ]
        )
    else:
        lines.extend(
            [
                "pub(crate) fn prepare_probe() -> Result<(), String> {",
                "    Ok(())",
                "}",
                "",
                "pub(crate) fn fixture_unit_ids() -> Result<Vec<i32>, String> {",
                "    crate::bindings::recoil::spring_api::units_query::get_all_units(0)",
                '        .map_err(|error| format!("get-all-units:{}", error.code))',
                "}",
                "",
                "pub(crate) fn fixture_feature_ids() -> Result<Vec<i32>, String> {",
                "    crate::bindings::recoil::spring_api::features::get_all_features(0)",
                '        .map_err(|error| format!("get-all-features:{}", error.code))',
                "}",
                "",
                "pub(crate) fn unit_candidate_is_primary(_unit_id: i32) -> bool {",
                "    true",
                "}",
                "",
                "pub(crate) fn feature_candidate_is_primary(_feature_id: i32) -> bool {",
                "    true",
                "}",
                "",
                "pub(crate) fn fixture_ready(_unit_id: i32, _feature_id: i32) -> bool {",
                "    true",
                "}",
                "",
            ]
        )
    if context == "ui":
        lines.extend(
            [
                "pub(crate) fn visibility_enemy_ids(",
                "    unit_ids: &[i32],",
                "    team_id: i32,",
                "    ally_team_id: i32,",
                ") -> Result<Option<(i32, i32)>, String> {",
                "    let mut enemy_los_unit_id = None;",
                "    let mut enemy_radar_unit_id = None;",
                "    for candidate_id in unit_ids.iter().copied() {",
                "        let Ok(candidate_team_id) = crate::bindings::recoil::spring_api::units_info::get_unit_team(candidate_id) else {",
                "            continue;",
                "        };",
                "        if candidate_team_id == team_id {",
                "            continue;",
                "        }",
                "        let Ok(state) = crate::bindings::recoil::spring_api::units_info::get_unit_los_state(candidate_id, ally_team_id, false) else {",
                "            continue;",
                "        };",
                "        if state.los && enemy_los_unit_id.is_none() {",
                "            enemy_los_unit_id = Some(candidate_id);",
                "        } else if state.radar && enemy_radar_unit_id.is_none() {",
                "            enemy_radar_unit_id = Some(candidate_id);",
                "        }",
                "    }",
                "    match (enemy_los_unit_id, enemy_radar_unit_id) {",
                "        (Some(los), Some(radar)) => Ok(Some((los, radar))),",
                '        _ => Err("native-api-test-enemy-visibility-unavailable".to_string()),',
                "    }",
                "}",
                "",
            ]
        )
    else:
        lines.extend(
            [
                "pub(crate) fn visibility_enemy_ids(",
                "    _unit_ids: &[i32],",
                "    _team_id: i32,",
                "    _ally_team_id: i32,",
                ") -> Result<Option<(i32, i32)>, String> {",
                "    Ok(None)",
                "}",
                "",
            ]
        )
    return "\n".join(lines)


# Keep the historical module as the compatibility implementation while the
# semantic helpers live in focused modules. Importing these names at the end
# preserves every existing call site and the generated output byte-for-byte.
from .model import load_model, load_tests, native_function
from .selection import (
    CORE_CONTEXT_BITS,
    CORE_OWNED_UNSUPPORTED_TESTS,
    SELECTION_EXCLUSION_REASONS,
    core_import_allowed,
    core_import_coverage,
    core_owned_unsupported,
    core_test_policy_allows,
    select_tests,
    select_tests_with_coverage,
)
from .projection import (
    first_parameter_value,
    output_index_for_field,
    output_projection,
    projection_record_path,
    record_path,
    runtime_args,
)
from .types import (
    kebab,
    pascal,
    records_field,
    rust_identifier,
    rust_semantic_path,
    semantic_path,
    simple_type,
    snake,
    type_kind,
    type_supported_by_probe,
    vector_component,
    wit_identifier,
    wit_type,
)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--context",
        choices=tuple(CONTEXT_SOURCE),
        default="synced_gadget",
        help="execution environment for the generated Core-Wasm module",
    )
    parser.add_argument(
        "--transport",
        choices=("core",),
        default="core",
        help="guest transport; Core emits a plain Core-Wasm module",
    )
    parser.add_argument(
        "--model",
        type=Path,
        default=MODEL_PATH,
        help="generated model JSON to project into the probe",
    )
    parser.add_argument(
        "--include-rendering",
        action="store_true",
        help="include canonical tests marked as requiring a renderer",
    )
    parser.add_argument("--output-root", type=Path, default=OUTPUT_ROOT)
    parser.add_argument("--lua-output", type=Path)
    parser.add_argument("--manifest-output", type=Path)
    args = parser.parse_args()

    functions, records, _, enums = load_model(args.model.resolve())
    selected, coverage = select_tests_with_coverage(
        functions, records, args.context, args.include_rendering, args.transport
    )
    if not selected:
        raise SystemExit("the Wasm parity probe selected no tests")
    output_root = args.output_root.resolve()
    rust_path = output_root / "src" / "probe_generated.rs"
    bindings_path = output_root / "src" / "probe_bindings.rs"
    context_path = output_root / "src" / "probe_context.rs"
    manifest_path = (
        args.manifest_output.resolve()
        if args.manifest_output is not None
        else output_root / "probe_manifest.json"
    )
    lua_path = (
        args.lua_output.resolve()
        if args.lua_output is not None
        else (
            DEFAULT_LUA_IDS_PATH
            if args.context == "synced_gadget"
            else DEFAULT_LUA_IDS_PATH.with_name(
                f"wasm_api_probe_tests_{args.context}.lua"
            )
        )
    )
    for path in (rust_path, bindings_path, context_path, manifest_path, lua_path):
        path.parent.mkdir(parents=True, exist_ok=True)
    rust_path.write_text(render_rust(selected, records, functions), encoding="utf-8")
    context_source = render_context(args.context)
    lib_source = (ROOT / "test" / "wasm_api" / "guests" / "parity_guest" / "src" / "lib.rs").read_text(
        encoding="utf-8"
    )
    bindings_path.write_text(
        render_bindings(
            args.context,
            args.transport,
            (rust_path.read_text(encoding="utf-8"), context_source, lib_source),
        ),
        encoding="utf-8",
    )
    context_path.write_text(context_source, encoding="utf-8")
    ids = [test["id"] for test, *_ in selected]
    manifest_path.write_text(
        json.dumps(
            {
                "version": 2,
                "context": args.context,
                "source_context": CONTEXT_SOURCE[args.context],
                "tests": ids,
                "coverage": coverage,
            },
            indent=2,
        )
        + "\n",
        encoding="utf-8",
    )
    lua_path.write_text(render_lua_probe_spec(selected), encoding="utf-8")
    print(f"generated {len(ids)} Wasm parity probe tests for {args.context}")


if __name__ == "__main__":
    main()
