#!/usr/bin/env python3
"""Generate the engine-to-Lua/native callin trace implementations.

The callin list and the Rust callback signatures are deliberately read from
the source tree instead of being maintained as a second hand-written list.
The generated Rust methods prepend tracing to the trait's existing default
method bodies, so the parity fixture observes callbacks without changing the
native module's default engine behavior.
"""

from __future__ import annotations

import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CALLBACKS = ROOT / "rust/crates/spring-native/src/callbacks.rs"
NATIVE_EVENTS = ROOT / "rts/NativeInterface/NativeInterfaceEventClient.cpp"
LUA_DOC = ROOT / "rust/crates/spring-native/lua_functions.md"
EVENTS_DEF = ROOT / "rts/System/Events.def"
RUST_OUT = ROOT / "test/native_api_parity/native/src/generated_callin_trace.rs"
LUA_OUT = ROOT / "test/native_api_parity/fixtures/game.sdd/LuaRules/Gadgets/callin_parity.lua"
LUA_UI_OUT = ROOT / "test/native_api_parity/fixtures/game.sdd/LuaUI/callin_ui_trace.lua"

def matching(text: str, start: int, opening: str, closing: str) -> int:
    depth = 0
    for index in range(start, len(text)):
        if text[index] == opening:
            depth += 1
        elif text[index] == closing:
            depth -= 1
            if depth == 0:
                return index
    raise ValueError(f"unclosed {opening!r} at {start}")


def split_top_level(text: str) -> list[str]:
    result: list[str] = []
    start = 0
    depth = 0
    pairs = {"(": ")", "[": "]", "{": "}"}
    closing = set(pairs.values())
    for index, char in enumerate(text):
        if char in pairs:
            depth += 1
        elif char in closing:
            depth -= 1
        elif char == "," and depth == 0:
            result.append(text[start:index].strip())
            start = index + 1
    tail = text[start:].strip()
    if tail:
        result.append(tail)
    return result


def camel_to_snake(value: str) -> str:
    value = value.replace("MiniMap", "Minimap")
    value = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", value)
    return re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", value).lower()


def native_symbols() -> set[str]:
    text = NATIVE_EVENTS.read_text(encoding="utf-8")
    return set(re.findall(r"^\s*LOAD_SYMBOL\((\w+)\)", text, re.MULTILINE))


def documented_callins() -> dict[str, set[str]]:
    text = LUA_DOC.read_text(encoding="utf-8")
    result: dict[str, set[str]] = {"Callins": set(), "SyncedCallins": set(), "UnsyncedCallins": set()}
    for namespace, name in re.findall(
        r"- `((?:Callins|SyncedCallins|UnsyncedCallins)\.([A-Za-z0-9_]+))`", text
    ):
        prefix, _ = namespace.split(".", 1)
        result[prefix].add(name)
    return result


def event_properties() -> dict[str, str]:
    """Return the engine event property expression for each managed event."""
    text = EVENTS_DEF.read_text(encoding="utf-8")
    return {
        name: properties
        for name, properties in re.findall(
            r"SETUP_EVENT\((\w+),\s*([^\n)]+)\)", text
        )
    }


def managed_callin_contexts(documented: dict[str, set[str]]) -> tuple[set[str], set[str]]:
    """Split general Callins according to Events.def's delivery bit.

    The generated parity gadget is loaded into both halves of LuaRules.  A
    synced-only definition must therefore be guarded so that the unsynced
    half does not register a second, restricted copy of the same callback.
    General unsynced callins are traced by the LuaUI wrapper instead, since
    that is the public unsynced handle used by the engine.
    """
    properties = event_properties()
    general = documented["Callins"]
    managed = set(properties)
    synced_general = {
        name
        for name in general & managed
        if "UNSYNCED_BIT" not in properties[name]
    }
    unsynced_general = {
        name
        for name in general & managed
        if "UNSYNCED_BIT" in properties[name]
    }
    return synced_general, unsynced_general


def trait_methods() -> list[tuple[str, str, list[tuple[str, str]]]]:
    text = CALLBACKS.read_text(encoding="utf-8")
    methods: list[tuple[str, str, list[tuple[str, str]]]] = []
    for match in re.finditer(r"^    fn\s+([A-Za-z0-9_]+)\s*\(", text, re.MULTILINE):
        name = match.group(1)
        open_paren = text.find("(", match.start())
        close_paren = matching(text, open_paren, "(", ")")
        brace = text.find("{", close_paren)
        if brace < 0:
            continue
        end = matching(text, brace, "{", "}")
        signature = text[match.start() : brace].rstrip()
        params = [
            item
            for item in split_top_level(text[open_paren + 1 : close_paren])
            if item.strip() not in {"self", "&mut self", "&self"}
        ]
        parsed_params = []
        for item in params:
            param_name, param_type = item.split(":", 1)
            parsed_params.append((param_name.strip(), param_type.strip()))
        methods.append((name, signature, parsed_params))
    return methods


def _push(param_name: str, param_type: str) -> list[str]:
    """Return trace statements for one semantically scalar Rust parameter."""
    if param_type == "crate::sys::NativeCallinCommand":
        return [f"        trace_args.extend(self.trace_command(&{param_name}));"]
    if param_type == "crate::sys::Float3":
        return [f"        trace_args.extend(self.trace_float3(&{param_name}));"]
    if param_type == "Option<crate::sys::Float3>":
        return [
            f"        if let Some(value) = {param_name}.as_ref() {{",
            "            trace_args.extend(self.trace_float3(value));",
            "        } else {",
            "            trace_args.push(self.trace_nil());",
            "        }",
        ]
    if param_type == "Option<i32>":
        return [f"        trace_args.push(self.trace_optional_i32({param_name}));"]
    if param_type == "Option<f32>":
        return [f"        trace_args.push(self.trace_optional_f32({param_name}));"]
    if param_type == "Option<&str>":
        return [f"        trace_args.push(self.trace_optional_str({param_name}));"]
    if param_type == "&[u8]":
        return [f"        trace_args.push(self.trace_byte_table({param_name}));"]
    if param_type == "&[crate::sys::ResourceExcessEntry]":
        return [f"        trace_args.push(self.trace_resource_excess({param_name}));"]
    if param_type == "&[KeyAction<'_>]":
        return [f"        trace_args.push(self.trace_actions({param_name}));"]
    if param_type == "&[GameSetupPlayerState<'_>]":
        return [f"        trace_args.push(self.trace_game_setup_states({param_name}));"]
    if param_type == "ViewGeometry":
        return [f"        trace_args.push(self.trace_geometry(&{param_name}));"]
    if param_type == "&str":
        return [f"        trace_args.push(self.trace_str({param_name}));"]
    if param_type == "i32":
        return [f"        trace_args.push(self.trace_i32({param_name}));"]
    if param_type == "i64":
        return [f"        trace_args.push(self.trace_i64({param_name}));"]
    if param_type == "u32":
        return [f"        trace_args.push(self.trace_u32({param_name}));"]
    if param_type == "u8":
        return [f"        trace_args.push(self.trace_u8({param_name}));"]
    if param_type == "f32":
        return [f"        trace_args.push(self.trace_f32({param_name}));"]
    if param_type == "bool":
        return [f"        trace_args.push(self.trace_bool({param_name}));"]
    if param_type.startswith("*mut ") or param_type.startswith("*const "):
        return [f"        trace_args.push(self.trace_opaque());"]
    raise ValueError(f"no callin trace mapping for {param_name}: {param_type}")


def trace_statements(name: str, params: list[tuple[str, str]]) -> list[str]:
    names = [param_name for param_name, _ in params]
    by_name = dict(params)
    lines = [
        "        let record_trace = record_callins_enabled();",
        "        let mut trace_args = Vec::new();",
        "        if record_trace {",
    ]

    if name == "unit_created":
        for param_name, param_type in params[:3]:
            lines.extend(_push(param_name, param_type))
        lines.extend([
            "        if builder_id >= 0 {",
            "            trace_args.push(self.trace_i32(builder_id));",
            "        } else {",
            "            trace_args.push(self.trace_nil());",
            "        }",
        ])
    elif name == "unit_destroyed":
        for param_name, param_type in params[:3]:
            lines.extend(_push(param_name, param_type))
        lines.extend([
            "        trace_args.push(attacker_id.map_or_else(|| self.trace_nil(), |value| self.trace_i32(value)));",
            "        trace_args.push(attacker_def_id.map_or_else(|| self.trace_nil(), |value| self.trace_i32(value)));",
            "        trace_args.push(attacker_team.map_or_else(|| self.trace_nil(), |value| self.trace_i32(value)));",
            "        trace_args.push(self.trace_i32(weapon_def_id));",
        ])
    elif name == "game_id":
        lines.append("        trace_args.push(self.trace_game_id(game_id));")
    elif name == "game_over":
        lines.append("        trace_args.push(self.trace_byte_table(winning_ally_teams));")
    elif name == "command_notify":
        lines.append("        trace_args.extend(self.trace_command_without_tag(&command));")
    elif name == "view_resize":
        lines.append("        trace_args.push(self.trace_geometry(&geometry));")
    elif name == "key_press":
        lines.extend([
            "        trace_args.push(self.trace_i32(key_code));",
            "        trace_args.push(self.trace_table(vec![",
            '            (self.trace_str("alt"), self.trace_bool(alt)),',
            '            (self.trace_str("ctrl"), self.trace_bool(ctrl)),',
            '            (self.trace_str("meta"), self.trace_bool(meta)),',
            '            (self.trace_str("shift"), self.trace_bool(shift)),',
            "        ]));",
            "        trace_args.push(self.trace_bool(is_repeat));",
            "        trace_args.push(self.trace_str(label));",
            "        trace_args.push(self.trace_i32(utf32_char));",
            "        trace_args.push(self.trace_i32(scan_code));",
            "        trace_args.push(self.trace_actions(actions));",
        ])
    elif name == "key_release":
        lines.extend([
            "        trace_args.push(self.trace_i32(key_code));",
            "        trace_args.push(self.trace_table(vec![",
            '            (self.trace_str("alt"), self.trace_bool(alt)),',
            '            (self.trace_str("ctrl"), self.trace_bool(ctrl)),',
            '            (self.trace_str("meta"), self.trace_bool(meta)),',
            '            (self.trace_str("shift"), self.trace_bool(shift)),',
            "        ]));",
            "        trace_args.push(self.trace_str(label));",
            "        trace_args.push(self.trace_i32(utf32_char));",
            "        trace_args.push(self.trace_i32(scan_code));",
            "        trace_args.push(self.trace_actions(actions));",
        ])
    elif name == "allow_unit_creation":
        for param_name, param_type in params[:3]:
            lines.extend(_push(param_name, param_type))
        lines.extend([
            "        if let Some(value) = build_pos.as_ref() {",
            "            trace_args.extend(self.trace_float3(value));",
            "            trace_args.push(self.trace_i32(build_facing));",
            "        }",
        ])
    elif name in {"allow_unit_transport_load", "allow_unit_transport_unload"}:
        for param_name, param_type in params[:6]:
            lines.extend(_push(param_name, param_type))
        lines.extend(_push("position", by_name["position"]))
    elif name == "allow_unit_kamikaze":
        for param_name, param_type in params[:2]:
            lines.extend(_push(param_name, param_type))
    elif name in {"unit_pre_damaged", "feature_pre_damaged"}:
        for param_name, param_type in params[:-2]:
            lines.extend(_push(param_name, param_type))
    elif name == "shield_pre_damaged":
        for param_name, param_type in params[:5]:
            lines.extend(_push(param_name, param_type))
        # Lua uses nil for the two beam-only values on regular projectiles;
        # the native ABI uses -1 as its compact absence representation.
        lines.append(
            "        trace_args.push(if beam_emitter_weapon_num < 0 { self.trace_nil() } else { self.trace_i32(beam_emitter_weapon_num) });"
        )
        lines.append(
            "        trace_args.push(if beam_emitter_unit_id < 0 { self.trace_nil() } else { self.trace_i32(beam_emitter_unit_id) });"
        )
        lines.extend(_push("start_pos", by_name["start_pos"]))
        lines.extend(_push("hit_pos", by_name["hit_pos"]))
    elif name == "unit_seismic_ping":
        # The idiomatic native signature keeps the vector together, while Lua
        # exposes its three coordinates first.
        lines.extend(_push("pos", by_name["pos"]))
        for param_name, param_type in params[1:]:
            lines.extend(_push(param_name, param_type))
    elif name == "default_command":
        lines.extend([
            "        if unit_id >= 0 {",
            '            trace_args.push(self.trace_str("unit"));',
            "            trace_args.push(self.trace_i32(unit_id));",
            "        } else if feature_id >= 0 {",
            '            trace_args.push(self.trace_str("feature"));',
            "            trace_args.push(self.trace_i32(feature_id));",
            "        } else {",
            "            trace_args.push(self.trace_nil());",
            "            trace_args.push(self.trace_nil());",
            "        }",
            "        trace_args.push(self.trace_i32(current_command));",
        ])
    elif name == "map_draw_cmd":
        lines.extend([
            "        trace_args.push(self.trace_i32(player_id));",
            "        match draw_type {",
            "            0 => {",
            '                trace_args.push(self.trace_str("point"));',
            "                if let Some(value) = pos0.as_ref() { trace_args.extend(self.trace_float3(value)); } else { trace_args.push(self.trace_nil()); }",
            "                trace_args.push(label.map_or_else(|| self.trace_nil(), |value| self.trace_str(value)));",
            "            }",
            "            2 => {",
            '                trace_args.push(self.trace_str("line"));',
            "                if let Some(value) = pos0.as_ref() { trace_args.extend(self.trace_float3(value)); } else { trace_args.push(self.trace_nil()); }",
            "                if let Some(value) = pos1.as_ref() { trace_args.extend(self.trace_float3(value)); } else { trace_args.push(self.trace_nil()); }",
            "            }",
            "            1 => {",
            '                trace_args.push(self.trace_str("erase"));',
            "                if let Some(value) = pos0.as_ref() { trace_args.extend(self.trace_float3(value)); } else { trace_args.push(self.trace_nil()); }",
            "                trace_args.push(self.trace_f32(100.0));",
            "            }",
            "            _ => {}",
            "        }",
        ])
    elif name == "world_tooltip":
        lines.extend([
            "        match kind {",
            "            1 => { trace_args.push(self.trace_str(\"unit\")); trace_args.push(self.trace_i32(unit_id)); }",
            "            2 => { trace_args.push(self.trace_str(\"feature\")); trace_args.push(self.trace_i32(feature_id)); }",
            "            3 => { trace_args.push(self.trace_str(\"ground\")); trace_args.extend(self.trace_float3(&ground_pos)); }",
            "            _ => trace_args.push(self.trace_str(\"selection\")),",
            "        }",
        ])
    elif name == "active_command_changed":
        lines.extend([
            "        if cmd_id >= 0 {",
            "            trace_args.push(self.trace_i32(cmd_id));",
            "            trace_args.push(self.trace_i32(cmd_type));",
            "        }",
        ])
    elif name == "add_console_line":
        # Lua intentionally exposes only the message and priority.  The
        # native query also carries an engine-side section label for native
        # consumers; it is not part of the Lua callback contract.
        lines.extend([
            "        trace_args.push(self.trace_str(message));",
            "        trace_args.push(self.trace_i32(level));",
        ])
    elif name in {"load", "save"}:
        lines.extend([
            "        if archive.is_null() {",
            "            trace_args.push(self.trace_nil());",
            "        } else {",
            "            trace_args.push(self.trace_opaque());",
            "        }",
        ])
    else:
        for param_name, param_type in params:
            lines.extend(_push(param_name, param_type))

    return lines


def result_trace_statements(name: str, signature: str) -> list[str]:
    """Generate a normalized trace for the callback's actual Rust result."""
    compact_signature = " ".join(signature.split())
    match = re.search(r"-> Result<(.+), Error>$", compact_signature)
    if match is None:
        raise ValueError(f"could not parse callback result type: {signature}")

    result_type = match.group(1).strip()
    if result_type == "()":
        return [
            "        let trace_results = match &callback_result {",
            "            Ok(()) => Vec::new(),",
            "            Err(_) => vec![self.trace_error()],",
            "        };",
        ]

    scalar_traces = {
        "bool": "trace_bool",
        "i32": "trace_i32",
        "f32": "trace_f32",
    }
    if result_type in scalar_traces:
        trace = scalar_traces[result_type]
        return [
            "        let trace_results = match &callback_result {",
            f"            Ok(value) => vec![self.{trace}(*value)],",
            "            Err(_) => vec![self.trace_error()],",
            "        };",
        ]

    option_traces = {
        "Option<i32>": "trace_optional_i32(*value)",
        "Option<String>": "trace_optional_str(value.as_deref())",
    }
    if result_type in option_traces:
        trace = option_traces[result_type]
        return [
            "        let trace_results = match &callback_result {",
            f"            Ok(value) => vec![self.{trace}],",
            "            Err(_) => vec![self.trace_error()],",
            "        };",
        ]

    if result_type == "Option<bool>":
        if name == "game_setup":
            # The native ABI packs Lua's `(handled, ready)` pair into
            # Option<bool>: None means the event was not handled and the
            # input readiness is preserved; Some(value) means handled with
            # the returned readiness value.
            return [
                "        let trace_results = match &callback_result {",
                "            Ok(Some(value)) => vec![self.trace_bool(true), self.trace_bool(*value)],",
                "            Ok(None) => vec![self.trace_bool(false), self.trace_bool(ready)],",
                "            Err(_) => vec![self.trace_error()],",
                "        };",
            ]
        return [
            "        let trace_results = match &callback_result {",
            "            Ok(value) => vec![self.trace_optional_bool(*value)],",
            "            Err(_) => vec![self.trace_error()],",
            "        };",
        ]

    tuple_traces = {
        "(bool, bool)": ("trace_bool", "trace_bool"),
        "(bool, f32)": ("trace_bool", "trace_f32"),
        "(f32, f32)": ("trace_f32", "trace_f32"),
    }
    if result_type in tuple_traces:
        first_trace, second_trace = tuple_traces[result_type]
        return [
            "        let trace_results = match &callback_result {",
            f"            Ok((first, second)) => vec![self.{first_trace}(*first), self.{second_trace}(*second)],",
            "            Err(_) => vec![self.trace_error()],",
            "        };",
        ]

    raise ValueError(f"no callback result trace mapping for {result_type}")


def method_item(name: str, signature: str, params: list[tuple[str, str]], text: str) -> str:
    start = next(
        match.start()
        for match in re.finditer(rf"^    fn\s+{re.escape(name)}\s*\(", text, re.MULTILINE)
    )
    brace = text.find("{", start)
    end = matching(text, brace, "{", "}")
    body = text[brace + 1 : end]
    callback_name = next(symbol for symbol in native_symbols() if camel_to_snake(symbol) == name)
    trace = "\n".join(trace_statements(name, params))
    result_trace = "\n".join(result_trace_statements(name, signature))
    if name == "mouse_press":
        # The deterministic driver needs the native event client to become
        # the CEventHandler mouse owner so MouseMove/MouseRelease can be
        # exercised.  This override exists only in the generated test module;
        # the production NativeModule default remains non-consuming.
        body = "\n        Ok(true)\n"
    elif name == "game_setup":
        # The fixture's LuaUI handler accepts the setup event and keeps the
        # player ready.  Exercise the corresponding native ABI path so the
        # trace compares `(handled, ready)` rather than the trait's neutral
        # no-handler default.
        body = "\n        Ok(Some(true))\n"
    elif name == "game_frame":
        body = (
            "\n        if benchmark_callin_variant_is(\"gameframe\") {\n"
            "            std::hint::black_box(game_frame);\n"
            "        }\n"
            + body
        )
    elif name == "draw_world":
        body = (
            "\n        if benchmark_case_is(\"draw\") {\n"
            "            if let Err(error) = self.benchmark_draw_world() {\n"
            "                return Err(spring_native::Error::new(1, error));\n"
            "            }\n"
            "        }\n"
            + body
        )
    return (
        f"{signature} {{\n"
        f"{trace}\n"
        "        }\n"
        "        let callback_result = {"
        f"{body}"
        "        };\n"
        "        if record_trace {\n"
        f"{result_trace}\n"
        f'            self.record_callin_args_result("{callback_name}", trace_args, trace_results);\n'
        "        }\n"
        "        callback_result\n"
        "    }\n"
    )


def generate_rust() -> str:
    text = CALLBACKS.read_text(encoding="utf-8")
    documented = documented_callins()
    shared = set().union(*documented.values()) & (native_symbols() - {"InitializeNativeModule"})
    lines = [
        "// @generated by test/native_api_parity/generate_callin_trace.py; do not edit.",
        "// Each method retains the NativeModule trait default body after recording.",
        "",
        "macro_rules! generated_callin_trace_methods {",
        "    () => {",
    ]
    for name, signature, params in trait_methods():
        if name in {"new", "draw_screen", "handle_lua_call"}:
            continue
        symbol = next((item for item in native_symbols() if camel_to_snake(item) == name), None)
        if symbol not in shared:
            continue
        lines.append(method_item(name, signature, params, text))
    lines.extend(["    };", "}", ""])
    return "\n".join(lines)


RETURN_DEFAULTS = {
    "AllowBuilderHoldFire": "true",
    "AllowCommand": "true",
    "AllowDirectUnitControl": "true",
    "AllowFeatureBuildStep": "true",
    "AllowFeatureCreation": "true",
    "AllowResourceLevel": "true",
    "AllowResourceTransfer": "true",
    "AllowStartPosition": "true",
    "AllowUnitBuildStep": "true",
    "AllowUnitCaptureStep": "true",
    "AllowUnitCloak": "true",
    "AllowUnitDecloak": "true",
    "AllowUnitKamikaze": "true",
    "AllowUnitTransfer": "true",
    "AllowUnitTransport": "true",
    "AllowUnitTransportLoad": "true",
    "AllowUnitTransportUnload": "true",
    "CommandFallback": "false",
    "ResourceExcess": "false",
    "MoveCtrlNotify": "false",
    "UnitUnitCollision": "false",
    "UnitFeatureCollision": "false",
    "DrawUnit": "false",
    "DrawFeature": "false",
    "DrawShield": "false",
    "DrawProjectile": "false",
    "DrawMaterial": "false",
    "KeyMapChanged": "false",
    "KeyPress": "false",
    "KeyRelease": "false",
    "TextInput": "false",
    "TextEditing": "false",
    "MouseMove": "false",
    "MousePress": "false",
    "MouseWheel": "false",
    "IsAbove": "false",
    "CommandNotify": "false",
    "AddConsoleLine": "false",
    "GroupChanged": "false",
    "MapDrawCmd": "false",
    "Explosion": "false",
    "ShieldPreDamaged": "false",
    "AllowWeaponTargetCheck": "-1",
    "AllowWeaponTarget": "true, nil",
    "AllowWeaponInterceptTarget": "true",
    "AllowUnitCreation": "true, true",
    "UnitPreDamaged": "nil, nil",
    "FeaturePreDamaged": "nil, nil",
    "TerraformComplete": "false",
}


def generate_lua() -> str:
    documented = documented_callins()
    native = native_symbols() - {"InitializeNativeModule"}
    synced_general, unsynced_general = managed_callin_contexts(documented)
    synced = sorted((documented["SyncedCallins"] | synced_general) & native)
    unsynced = sorted(
        (documented["UnsyncedCallins"] | unsynced_general) & native
    )
    lines = [
        "-- @generated by test/native_api_parity/generate_callin_trace.py; do not edit.",
        "-- This gadget records the Lua side of shared engine callins and returns",
        "-- the same neutral/default values as the C++ Lua implementation.",
        "",
        "function gadget:GetInfo()",
        "\tlocal mode = Spring.GetModOptions() or {}",
        "\treturn {",
        '\t\tname = "Native API Callin Parity Trace",',
        '\t\tdesc = "Traces shared engine-to-Lua/native callins",',
        '\t\tauthor = "Spring",',
        "\t\tlayer = 1000000,",
        "\t\tenabled = tostring(mode.native_api_parity_mode or \"\") ~= \"benchmark\",",
        "\t}",
        "end",
        "",
        'local Common = VFS.Include("LuaRules/Utilities/native_api_parity_common.lua")',
        "local synced = gadgetHandler:IsSyncedCode()",
        "local outputStream = \"callin_lua\"",
        "",
        "local function roundNumber(value)",
        "\treturn math.floor(value * 100000 + 0.5) / 100000",
        "end",
        "",
        "local function normalize(value, depth)",
        "\tlocal valueType = type(value)",
        "\tif valueType == \"nil\" then return { type = \"nil\" } end",
        "\tif valueType == \"number\" then return roundNumber(value) end",
        "\tif valueType == \"boolean\" or valueType == \"string\" then return value end",
        "\tif valueType ~= \"table\" then return valueType end",
        "\tif (depth or 0) >= 5 then return { type = \"table\" } end",
        "\tlocal result = {}",
        "\tfor key, item in pairs(value) do",
        "\t\tresult[#result + 1] = { key = normalize(key, (depth or 0) + 1), value = normalize(item, (depth or 0) + 1) }",
        "\tend",
        "\ttable.sort(result, function(a, b) return Common.encode(a.key) .. Common.encode(a.value) < Common.encode(b.key) .. Common.encode(b.value) end)",
        "\treturn result",
        "end",
        "",
        "local function pack(...)",
        "\tlocal result = { n = select(\"#\", ...) }",
        "\tfor index = 1, result.n do",
        "\t\tresult[index] = select(index, ...)",
        "\tend",
        "\treturn result",
        "end",
        "",
        "local function unpackn(values)",
        "\treturn unpack(values, 1, values.n)",
        "end",
        "",
        "local function forward(encoded)",
        "\tif Script.LuaUI and Script.LuaUI.NativeApiParityResult then",
        "\t\tScript.LuaUI.NativeApiParityResult(outputStream, encoded)",
        "\tend",
        "end",
        "",
        "local function trace(name, results, ...)",
        "\tlocal args = pack(...)",
        "\tlocal normalized = {}",
        "\tfor index = 1, args.n do",
        "\t\tnormalized[index] = normalize(args[index], 0)",
        "\tend",
        "\tlocal normalizedResults = {}",
        "\tfor index = 1, results.n do",
        "\t\tnormalizedResults[index] = normalize(results[index], 0)",
        "\tend",
        "\tlocal payload = {",
        "\t\tcontext = synced and \"synced_gadget\" or \"unsynced_gadget\",",
        "\t\tname = name,",
        "\t\tarity = args.n,",
        "\t\targs = normalized,",
        "\t\tresultArity = results.n,",
        "\t\tresults = normalizedResults,",
        "\t}",
        "\tlocal encoded = Common.encode(payload)",
        "\tif synced then",
        "\t\tSendToUnsynced(\"native_api_callin_trace\", encoded)",
        "\telse",
        "\t\tforward(encoded)",
        "\tend",
        "end",
        "",
        "local function traceCallin(name, ...)",
        "\tlocal args = pack(...)",
        "\tlocal results",
        "\tif name == \"AllowUnitCreation\" then",
        "\t\tresults = pack(true, true)",
        # gadgetHandler starts with priority=1.0 and applies math.max() to
        # every gadget result, so a neutral result must remain numeric even
        # when the engine supplied no default priority.
        "\telseif name == \"AllowWeaponTarget\" then",
        "\t\tresults = pack(true, args[5] or 1.0)",
        "\telseif name == \"UnitPreDamaged\" or name == \"FeaturePreDamaged\" then",
        "\t\tresults = pack(args[4], 1.0)",
        # Let the native test process own the deterministic mouse sequence;
        # the Lua-only baseline owns the same sequence through this gadget.
        "\telseif name == \"MousePress\" then",
        "\t\tresults = pack(Common.mode() ~= \"native\")",
        "\telseif name == \"AllowWeaponTargetCheck\" then",
        "\t\tresults = pack(-1)",
        "\telseif name == \"DefaultCommand\" or name == \"GetTooltip\" or name == \"WorldTooltip\" then",
        "\t\tresults = pack(nil)",
        "\telse",
        "\t\tlocal default = {",
    ]
    for name, value in sorted(RETURN_DEFAULTS.items()):
        if name in {"AllowUnitCreation", "UnitPreDamaged", "FeaturePreDamaged", "AllowWeaponTarget", "AllowWeaponTargetCheck"}:
            continue
        lines.append(f'\t\t["{name}"] = {{{value}}},')
    lines.extend([
        "\t\t}",
        "\t\tlocal values = default[name]",
        "\t\tresults = values and pack(unpack(values, 1, #values)) or pack()",
        "\tend",
        "\ttrace(name, results, unpackn(args))",
        "\treturn unpackn(results)",
        "end",
        "",
        "if not synced then",
        "\tfunction gadget:RecvFromSynced(name, encoded)",
        "\t\tif name == \"native_api_callin_trace\" then",
        "\t\t\tforward(encoded)",
        "\t\telseif name == \"native_api_callin_phase\" then",
        "\t\t\tforward(Common.encode({ context = \"callin_phase\", name = encoded }))",
        "\t\tend",
        "\tend",
        "end",
        "",
        "-- General synced Callins and explicit SyncedCallins are defined only",
        "-- in the full-read LuaRules half; otherwise the unsynced half would",
        "-- register a second restricted callback for the same engine event.",
        "if synced then",
    ])
    for name in synced:
        lines.extend([
            f"\tfunction gadget:{name}(...)",
            f'\t\treturn traceCallin("{name}", ...)',
            "\tend",
            "",
        ])
    lines.extend(["end", "", "if not synced then"])
    for name in unsynced:
        lines.extend([
            f"\tfunction gadget:{name}(...)",
            f'\t\treturn traceCallin("{name}", ...)',
            "end",
            "",
        ])
    lines.append("end")
    return "\n".join(lines)


def generate_lua_ui() -> str:
    documented = documented_callins()
    native = native_symbols() - {"InitializeNativeModule"}
    ui_names = sorted(
        (documented["Callins"] & native) - {"Load", "Save", "Shutdown"}
    )
    lines = [
        "-- @generated by test/native_api_parity/generate_callin_trace.py; do not edit.",
        "-- LuaUI-side wrappers trace general Callins without replacing existing",
        "-- fixture behavior or changing any of their return values.",
        "",
        "local Common = VFS.Include(\"LuaRules/Utilities/native_api_parity_common.lua\")",
        "",
        "local function roundNumber(value)",
        "\treturn math.floor(value * 100000 + 0.5) / 100000",
        "end",
        "",
        "local function normalize(value, depth)",
        "\tlocal valueType = type(value)",
        "\tif valueType == \"nil\" then return { type = \"nil\" } end",
        "\tif valueType == \"number\" then return roundNumber(value) end",
        "\tif valueType == \"boolean\" or valueType == \"string\" then return value end",
        "\tif valueType ~= \"table\" then return valueType end",
        "\tif (depth or 0) >= 5 then return { type = \"table\" } end",
        "\tlocal result = {}",
        "\tfor key, item in pairs(value) do",
        "\t\tresult[#result + 1] = { key = normalize(key, (depth or 0) + 1), value = normalize(item, (depth or 0) + 1) }",
        "\tend",
        "\ttable.sort(result, function(a, b) return Common.encode(a.key) .. Common.encode(a.value) < Common.encode(b.key) .. Common.encode(b.value) end)",
        "\treturn result",
        "end",
        "",
        "local function pack(...)",
        "\tlocal result = { n = select(\"#\", ...) }",
        "\tfor index = 1, result.n do",
        "\t\tresult[index] = select(index, ...)",
        "\tend",
        "\treturn result",
        "end",
        "",
        "local function unpackn(values)",
        "\treturn unpack(values, 1, values.n)",
        "end",
        "",
        "local function defaultResult(name)",
        "\tif name == \"AllowWeaponTargetCheck\" then return -1 end",
        "\tif name == \"AllowWeaponTarget\" then return true, 1.0 end",
        "\tif name == \"AllowUnitCreation\" then return true, true end",
        "\tif name == \"DefaultCommand\" or name == \"GetTooltip\" or name == \"WorldTooltip\" then return nil end",
        "\tif name == \"UnitPreDamaged\" or name == \"FeaturePreDamaged\" then return nil, nil end",
        "\tlocal defaults = {",
    ]
    for name, value in sorted(RETURN_DEFAULTS.items()):
        if name in {"AllowUnitCreation", "UnitPreDamaged", "FeaturePreDamaged", "AllowWeaponTarget", "AllowWeaponTargetCheck"}:
            continue
        lines.append(f'\t\t["{name}"] = {{{value}}},')
    lines.extend([
        "\t}",
        "\tlocal values = defaults[name]",
        "\tif values then return unpack(values) end",
        "end",
        "",
        "local function trace(name, results, ...)",
        "\tlocal args = pack(...)",
        "\tlocal normalized = {}",
        "\tfor index = 1, args.n do",
        "\t\tnormalized[index] = normalize(args[index], 0)",
        "\tend",
        "\tlocal normalizedResults = {}",
        "\tfor index = 1, results.n do",
        "\t\tnormalizedResults[index] = normalize(results[index], 0)",
        "\tend",
        "\tCommon.appendJsonLine(Common.outputDir() .. \"/callin_lua.jsonl\", {",
        "\t\tcontext = \"lua_ui\",",
        "\t\tname = name,",
        "\t\tarity = args.n,",
        "\t\targs = normalized,",
        "\t\tresultArity = results.n,",
        "\t\tresults = normalizedResults,",
        "\t})",
        "end",
        "",
    ])
    for name in ui_names:
        lines.extend([
            f'local previous_{name} = _G["{name}"]',
            f'_G["{name}"] = function(...)',
            f'\tlocal result',
            f'\tif previous_{name} then',
            f'\t\tresult = pack(previous_{name}(...))',
            "\telse",
            f'\t\tresult = pack(defaultResult("{name}"))',
            "\tend",
        ])
        if name in {"GetTooltip", "WorldTooltip"}:
            # A Lua function with no explicit return is observed as nil by
            # the engine's string callin adapters.  Preserve that one-value
            # result in the trace even when the previous LuaUI function
            # returned zero values.
            lines.extend([
                "\tif result.n == 0 then",
                "\t\tresult = pack(nil)",
                "\tend",
            ])
        lines.extend([
            f'\ttrace("{name}", result, ...)',
            "\treturn unpackn(result)",
            "end",
            "",
        ])
    lines.extend([
        "-- These wrappers are installed after LuaUI's normal bootstrap.  Register",
        "-- them explicitly so CLuaUI adds the corresponding engine events.",
        "for _, name in ipairs({",
    ])
    for name in ui_names:
        lines.append(f'\t"{name}",')
    lines.extend([
        "}) do",
        "\tScript.UpdateCallIn(name)",
        "end",
    ])
    return "\n".join(lines)


def main() -> int:
    RUST_OUT.write_text(generate_rust(), encoding="utf-8")
    LUA_OUT.write_text(generate_lua(), encoding="utf-8")
    LUA_UI_OUT.write_text(generate_lua_ui(), encoding="utf-8")
    documented = documented_callins()
    shared = set().union(*documented.values()) & (native_symbols() - {"InitializeNativeModule"})
    print(f"generated {len(shared)} shared callin trace entries")
    print(RUST_OUT)
    print(LUA_OUT)
    print(LUA_UI_OUT)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
