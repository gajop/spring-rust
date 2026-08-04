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
RUST_OUT = ROOT / "test/native_api_parity/native/src/generated_callin_trace.rs"
LUA_OUT = ROOT / "test/native_api_parity/fixtures/game.sdd/LuaRules/Gadgets/callin_parity.lua"
LUA_UI_OUT = ROOT / "test/native_api_parity/fixtures/game.sdd/LuaUI/callin_ui_trace.lua"

# These callbacks are documented in the general Callins section because their
# C++ implementation is shared by Lua handles, but only the synced LuaRules
# handle can receive them: the watch masks used by the implementation are not
# initialized for LuaUI.
SYNCED_GENERAL_CALLINS = {"Explosion", "ProjectileCreated", "ProjectileDestroyed"}


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
    lines = ["        let mut trace_args = Vec::new();"]

    if name == "unit_created":
        for param_name, param_type in params[:3]:
            lines.extend(_push(param_name, param_type))
        lines.extend([
            "        if builder_id >= 0 {",
            "            trace_args.push(self.trace_i32(builder_id));",
            "        }",
        ])
    elif name == "unit_destroyed":
        for param_name, param_type in params[:3]:
            lines.extend(_push(param_name, param_type))
        lines.extend([
            "        trace_args.push(if attacker_id < 0 { self.trace_nil() } else { self.trace_i32(attacker_id) });",
            "        trace_args.push(if attacker_id < 0 || attacker_def_id < 0 { self.trace_nil() } else { self.trace_i32(attacker_def_id) });",
            "        trace_args.push(if attacker_id < 0 || attacker_team < 0 { self.trace_nil() } else { self.trace_i32(attacker_team) });",
            "        trace_args.push(self.trace_i32(weapon_def_id));",
        ])
    elif name == "game_id":
        lines.append("        trace_args.push(self.trace_game_id(game_id));")
    elif name == "game_over":
        lines.append("        trace_args.push(self.trace_byte_table(winning_ally_teams));")
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
    else:
        for param_name, param_type in params:
            lines.extend(_push(param_name, param_type))

    lines.append(f'        self.record_callin_args("{next(symbol for symbol in native_symbols() if camel_to_snake(symbol) == name)}", trace_args);')
    return lines


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
    return (
        f"{signature} {{\n"
        f"{trace}\n"
        f"{body}"
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
}


def generate_lua() -> str:
    documented = documented_callins()
    shared = sorted(
        (documented["SyncedCallins"] | documented["UnsyncedCallins"] | SYNCED_GENERAL_CALLINS)
        & (native_symbols() - {"InitializeNativeModule"})
    )
    lines = [
        "-- @generated by test/native_api_parity/generate_callin_trace.py; do not edit.",
        "-- This gadget records the Lua side of shared engine callins and returns",
        "-- the same neutral/default values as the C++ Lua implementation.",
        "",
        "function gadget:GetInfo()",
        "\treturn {",
        '\t\tname = "Native API Callin Parity Trace",',
        '\t\tdesc = "Traces shared engine-to-Lua/native callins",',
        '\t\tauthor = "Spring",',
        "\t\tlayer = 1000000,",
        "\t\tenabled = true,",
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
        "local function forward(encoded)",
        "\tif Script.LuaUI and Script.LuaUI.NativeApiParityResult then",
        "\t\tScript.LuaUI.NativeApiParityResult(outputStream, encoded)",
        "\tend",
        "end",
        "",
        "local function trace(name, ...)",
        "\tlocal args = { ... }",
        "\tlocal normalized = {}",
        "\tfor index = 1, select(\"#\", ...) do",
        "\t\tnormalized[index] = normalize(args[index], 0)",
        "\tend",
        "\tlocal payload = {",
        "\t\tcontext = synced and \"synced_gadget\" or \"unsynced_gadget\",",
        "\t\tname = name,",
        "\t\tarity = select(\"#\", ...),",
        "\t\targs = normalized,",
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
        "\ttrace(name, ...)",
        "\tif name == \"AllowUnitCreation\" then return true, true end",
        "\tif name == \"UnitPreDamaged\" or name == \"FeaturePreDamaged\" then return nil, nil end",
        # gadgetHandler starts with priority=1.0 and applies math.max() to
        # every gadget result, so a neutral result must remain numeric even
        # when the engine supplied no default priority.
        "\tif name == \"AllowWeaponTarget\" then",
        "\t\tlocal args = { ... }",
        "\t\treturn true, args[5] or 1.0",
        "\tend",
        "\tif name == \"AllowWeaponTargetCheck\" then return -1 end",
        "\tlocal default = {",
    ]
    for name, value in sorted(RETURN_DEFAULTS.items()):
        if name in {"AllowUnitCreation", "UnitPreDamaged", "FeaturePreDamaged", "AllowWeaponTarget", "AllowWeaponTargetCheck"}:
            continue
        lines.append(f'\t\t["{name}"] = {{{value}}},')
    lines.extend([
        "\t}",
        "\tlocal values = default[name]",
        "\tif values then return unpack(values) end",
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
    ])
    for name in shared:
        lines.extend([
            f"function gadget:{name}(...)",
            f'\treturn traceCallin("{name}", ...)',
            "end",
            "",
        ])
    return "\n".join(lines)


def generate_lua_ui() -> str:
    documented = documented_callins()
    shared = set().union(*documented.values()) & (native_symbols() - {"InitializeNativeModule"})
    ui_names = sorted(
        shared
        - documented["SyncedCallins"]
        - documented["UnsyncedCallins"]
        - SYNCED_GENERAL_CALLINS
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
        "local function defaultResult(name)",
        "\tif name == \"AllowWeaponTargetCheck\" then return -1 end",
        "\tif name == \"AllowWeaponTarget\" then return true, 1.0 end",
        "\tif name == \"AllowUnitCreation\" then return true, true end",
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
        "local function trace(name, ...)",
        "\tlocal args = { ... }",
        "\tlocal normalized = {}",
        "\tfor index = 1, select(\"#\", ...) do",
        "\t\tnormalized[index] = normalize(args[index], 0)",
        "\tend",
        "\tCommon.appendJsonLine(Common.outputDir() .. \"/callin_lua.jsonl\", {",
        "\t\tcontext = \"lua_ui\",",
        "\t\tname = name,",
        "\t\tarity = select(\"#\", ...),",
        "\t\targs = normalized,",
        "\t})",
        "end",
        "",
    ])
    for name in ui_names:
        lines.extend([
            f'local previous_{name} = _G["{name}"]',
            f'_G["{name}"] = function(...)',
            f'\ttrace("{name}", ...)',
            f'\tif previous_{name} then return previous_{name}(...) end',
            f'\treturn defaultResult("{name}")',
            "end",
            "",
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
