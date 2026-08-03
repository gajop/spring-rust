#!/usr/bin/env python3
"""
Match Lua API functions to Rust API functions and generate comparison report.
"""

import re
from pathlib import Path
from typing import List, Dict, Tuple, Optional
from difflib import SequenceMatcher


FLATTENED_TYPES = {
    'float2': ['float', 'float'],
    'float3': ['float', 'float', 'float'],
    'float4': ['float', 'float', 'float', 'float'],
    'sys::float2': ['float', 'float'],
    'sys::float3': ['float', 'float', 'float'],
    'sys::float4': ['float', 'float', 'float', 'float'],
    'sys::rectanglequery': ['float', 'float', 'float', 'float'],
    'sys::boxquery': ['float', 'float', 'float', 'float', 'float', 'float'],
    'sys::spherequery': ['float', 'float', 'float', 'float'],
    'sys::cylinderquery': ['float', 'float', 'float', 'float', 'float'],
    'sys::teamcolor': ['float', 'float', 'float', 'float'],
}

CALLBACK_SHAPE_EQUIVALENTS = {
    'Spring.SetHeightMapFunc',
    'Spring.SetOriginalHeightMapFunc',
    'Spring.SetSmoothMeshFunc',
}


def expand_params(params: List[Dict]) -> List[str]:
    """Expand only explicit native aggregate types into normalized scalar slots."""
    expanded = []
    for p in params:
        ptype = p.get('type', '').strip()
        lower = ptype.lower()
        if lower in FLATTENED_TYPES:
            expanded.extend(FLATTENED_TYPES[lower])
        else:
            expanded.append(normalize_param_type(p))
    return expanded


def normalize_param_type(param: Dict) -> str:
    ptype = param.get('type', '').strip()
    normalized = normalize_type(ptype)
    name = param.get('name', '').lower()
    compact_name = re.sub(r'[^a-z0-9]', '', name)
    raw = ptype.lower().rstrip('?')

    integral_name = (
        name.endswith('id') or name.endswith('ids') or re.search(r'id\d*$', compact_name) is not None or
        name.endswith('index') or name.endswith('num') or name.endswith('count') or
        compact_name.endswith('index') or compact_name.endswith('num') or compact_name.endswith('count') or
        name in {
            'button', 'button1', 'key', 'keycode', 'scancode', 'drawmask',
            'facing', 'heading', 'degree', 'level', 'rank', 'sorttype',
            'sortmode', 'packetid', 'piece', 'piecenum', 'weaponnum',
            'cmdid', 'cmdindex', 'team', 'allyteam',
        } or
        compact_name in {
            'button', 'button1', 'key', 'keycode', 'scancode', 'drawmask',
            'facing', 'heading', 'degree', 'level', 'rank', 'sorttype',
            'sortmode', 'packetid', 'piece', 'piecenum', 'weaponnum',
            'cmdid', 'cmdindex', 'team', 'allyteam', 'maxlines',
            'pingtag',
        }
    )
    if raw == 'number' and integral_name:
        return 'int'

    return normalized


def normalize_type(ptype: str) -> str:
    """Normalize unambiguous primitive aliases only; leave unknowns as mismatches."""
    lt = ptype.lower()
    if not lt:
        return ""
    lt = lt.strip()
    if lt.endswith("?"):
        lt = lt[:-1]
    if lt.startswith("option<") and lt.endswith(">"):
        lt = lt[7:-1].strip()
    if lt.startswith("&sys::"):
        lt = lt[6:]
    if lt.startswith("&[sys::") and lt.endswith("]"):
        inner = lt[2:-1]
        if inner == "sys::nativecommand":
            return "array<createcommand>"
        if inner == "sys::commandffi":
            return "array<createcommand>"
        if inner == "sys::float4":
            return "array<float4>"
        return f"array<{inner[5:] if inner.startswith('sys::') else inner}>"
    if lt in ("number", "float", "double", "f32", "f64"):
        return "float"
    if re.fullmatch(r'-?\d+', lt):
        return "int"
    if re.fullmatch(r'-?\d+\.\d+', lt):
        return "float"
    if re.fullmatch(r'".*"', lt):
        return "string"
    if lt in ("integer", "int", "i32", "u32", "i16", "u16", "i8", "u8", "i64", "u64", "usize", "cmd|integer", "(cmd|integer)"):
        return "int"
    if lt in ("boolean", "bool", "true", "false"):
        return "bool"
    if lt in ("string", "&str", "str"):
        return "string"
    if lt in ("integer[]", "&[i32]", "&[u32]", "table<integer,any>", "table<integer, any>"):
        return "array<int>"
    if lt.startswith("table<integer,"):
        return "array<int>"
    if lt in ("number[]", "&[f32]"):
        return "array<float>"
    if lt == "createcommand[]":
        return "array<createcommand>"
    if lt == "controlpoint[]":
        return "array<float4>"
    if lt in ("rgba", "rgba?"):
        return "float4"
    if lt == "plane[]":
        return "planesquery"
    if re.fullmatch(r'\[\s*f32\s*;\s*\d+\s*\]', lt):
        return "array<float>"
    if lt in ("resourcename", "storagename"):
        return "string"
    if lt in ("resourcename|storagename", "(resourcename|storagename)"):
        return "string"
    if lt in ("(loglevel|log)", "1|2", "(1|2)", "losaccess", "losmask", "lostable|losmask|integer", "(lostable|losmask|integer)", "number[bit]", "?string|number", "string|number", "(string|number)"):
        return "int"
    if lt in ("boolean|nil", "nil|boolean"):
        return "bool"
    if lt == '("los"|"airlos"|"radar"|"sonar"|"seismic"|"radarjammer"...)':
        return "string"
    if lt in ("rulesparamvalue",):
        return "rulesparamvalue"
    if lt in ("luafunctionref", "fun(...)", "function"):
        return "luafunctionref"
    if lt in ("nativeluaargs", "any", "...", "...any", "...:any", "...number", "...:number"):
        return "nativeluaargs"
    if lt in ("nativeluavalue",):
        return "nativeluavalue"
    if lt in ("unittargetref",):
        return "unittargetref"
    if lt in ("projectiletargetref",):
        return "projectiletargetref"
    if lt in ("nativeprojectileparams", "projectileparams"):
        return "projectileparams"
    if lt in ("nativeexplosionparams", "explosionparams"):
        return "explosionparams"
    if lt in ("string|integer", "(string|integer)", "defref"):
        return "defref"
    if lt in ("number|boolean", "(number|boolean)", "numberorbool"):
        return "numberorbool"
    if lt in ("unithealthvalue", "(number|setunithealthamounts)", "number|setunithealthamounts"):
        return "unithealthvalue"
    if lt in ("unitcosts", "unitcostoverrides"):
        return "unitcosts"
    if lt in ("atmosphereparams", "sunlightingparams", "waterparams", "maprenderingparams", "soundeffectparams", "rgbcolor"):
        return lt
    if lt in ("facinginteger", "facing", "heading", "cmd", "loglevel", "drawmask", "soundchannel", "createcommandoptions"):
        return "int"
    if lt == "createcommandparams":
        return "array<float>"
    if lt.startswith("sys::"):
        lt = lt[5:]
    named_aliases = {
        "nativecommanddescription": "commanddescription",
        "nativecommand": "createcommand",
        "commandffi": "createcommand",
        "nativecommanddescription": "commanddescription",
        "nativecommanddescriptionffi": "commanddescription",
        "nativecommandffi": "createcommand",
        "float4": "float4",
        "unitcostoverrides": "unitcosts",
        "nativeprojectileparams": "projectileparams",
        "nativeexplosionparams": "explosionparams",
    }
    return named_aliases.get(lt, lt)


def compare_params(lua_params: List[Dict], rust_params: List[Dict]) -> Tuple[bool, str]:
    lua_expanded = expand_params(lua_params)
    rust_expanded = expand_params(rust_params)

    if len(lua_expanded) != len(rust_expanded):
        return False, f"count mismatch (lua={len(lua_expanded)}, rust={len(rust_expanded)})"

    diffs = []
    for idx, (ltype, rtype) in enumerate(zip(lua_expanded, rust_expanded)):
        if ltype and rtype and ltype != rtype:
            diffs.append(f"p{idx+1} type {ltype}!={rtype}")

    if diffs:
        return False, "; ".join(diffs)

    return True, "match"


def compare_function_params(lua_func: Dict, rust_func: Dict) -> Tuple[bool, str]:
    if lua_func.get('name') in CALLBACK_SHAPE_EQUIVALENTS:
        return True, "callback shape equivalent"

    # RequestPath accepts one Lua union (number|string).  The C ABI keeps the
    # two representations explicit so bindgen can expose a typed optional
    # string alongside the numeric path type.  Compare the union as one
    # logical parameter while still checking every remaining argument.
    if lua_func.get('name') == 'Spring.RequestPath':
        lua_params = lua_func.get('params', [])
        rust_params = rust_func.get('params', [])
        if len(lua_params) >= 1 and len(rust_params) >= 2:
            move_id = normalize_type(lua_params[0].get('type', ''))
            move_def_id = normalize_param_type(rust_params[0])
            move_def_name = normalize_param_type(rust_params[1])
            if move_id in {'number|string', 'string|number', 'defref'} and move_def_id == 'int' and move_def_name == 'string':
                return compare_params(lua_params[1:], rust_params[2:])

    return compare_params(lua_func.get('params', []), rust_func.get('params', []))


def param_mismatch_msg(lua_func: Dict, rust_func: Dict) -> Optional[str]:
    ok, msg = compare_function_params(lua_func, rust_func)
    if ok:
        return None
    return f'{lua_func["name"]} param mismatch {msg}'


def split_top_level_commas(param_str: str) -> List[str]:
    parts = []
    start = 0
    depth = 0
    pairs = {'(': ')', '<': '>', '{': '}', '[': ']'}
    closers = set(pairs.values())

    for idx, ch in enumerate(param_str):
        if ch in pairs:
            depth += 1
        elif ch in closers and depth > 0:
            depth -= 1
        elif ch == ',' and depth == 0:
            part = param_str[start:idx].strip()
            if part:
                parts.append(part)
            start = idx + 1

    tail = param_str[start:].strip()
    if tail:
        parts.append(tail)
    return parts


def parse_params(param_str: str) -> List[Dict]:
    if not param_str:
        return []
    parts = split_top_level_commas(param_str)
    params = []
    for part in parts:
        if ':' in part:
            name, ty = part.split(':', 1)
            params.append({'name': name.strip(), 'type': ty.strip()})
        else:
            params.append({'name': part.strip(), 'type': ''})
    return params


def parse_lua_functions(filepath: Path) -> Dict[str, List[Dict]]:
    """Parse lua_functions.md and extract functions by namespace."""
    functions = {}
    current_namespace: Optional[str] = None

    with open(filepath, 'r') as f:
        for line in f:
            line = line.strip()

            if line.startswith('## '):
                current_namespace = None
                continue

            ns_match = re.match(r'###\s+(\w+)\s+\((\d+)\s+functions\)', line)
            if ns_match:
                current_namespace = ns_match.group(1)
                functions[current_namespace] = []
                continue

            func_match = re.match(r'-\s+`([^`]+)`\s+\(params:\s*(.*)\)', line)
            if func_match and current_namespace:
                func_name = func_match.group(1)
                params = parse_params(func_match.group(2))
                functions[current_namespace].append({'name': func_name, 'params': params})

    return functions


def parse_rust_functions(filepath: Path) -> Dict[str, List[Dict]]:
    """Parse rust_functions.md and extract functions by module."""
    functions = {}
    current_module: Optional[str] = None

    with open(filepath, 'r') as f:
        for line in f:
            line = line.strip()

            mod_match = re.match(r'##\s+(\w+)\s+\((\d+)\s+functions\)', line)
            if mod_match:
                current_module = mod_match.group(1)
                functions[current_module] = []
                continue

            func_match = re.match(r'-\s+`([^`]+)`\s+\(params:\s*(.*)\)\s+→\s+`([^`]+)`', line)
            if func_match and current_module:
                func_name = func_match.group(1)
                params = parse_params(func_match.group(2))
                return_type = func_match.group(3)
                functions[current_module].append({'name': func_name, 'params': params, 'return': return_type})

    return functions


def find_best_match(lua_func: Dict, rust_funcs: Dict[str, List[Dict]]) -> Optional[Tuple[Dict, float, str]]:
    """
    Find the best matching Rust function for a Lua function.
    Returns (rust_func_dict, confidence, module) or None.
    """
    lua_normalized = normalize_name(lua_func['name'])

    best_match: Optional[Tuple[Dict, float, str]] = None
    best_score = 0.0

    for module, funcs in rust_funcs.items():
        for rust_func in funcs:
            rust_normalized = normalize_name(rust_func['name'])
            similarity = SequenceMatcher(None, lua_normalized, rust_normalized).ratio()

            if similarity > best_score:
                best_score = similarity
                best_match = (rust_func, similarity, module)

    if best_score >= 0.8:
        return best_match

    return None


def normalize_name(name: str) -> str:
    """
    Normalize a function name for comparison.
    - Convert camelCase to snake_case
    - Remove namespace prefixes
    - Lowercase
    """
    if '.' in name:
        name = name.split('.')[-1]

    result = re.sub('([a-z0-9])([A-Z])', r'\1_\2', name)
    result = result.lower()

    return result


def main():
    rust_dir = Path(__file__).parent

    lua_file = rust_dir / 'lua_functions.md'
    rust_file = rust_dir / 'rust_functions.md'

    if not lua_file.exists() or not rust_file.exists():
        print("Error: Run extract_lua_api.py and extract_rust_api.py first")
        return

    print("Parsing Lua functions...")
    lua_funcs = parse_lua_functions(lua_file)

    print("Parsing Rust functions...")
    rust_funcs = parse_rust_functions(rust_file)

    spring_funcs = lua_funcs.get('Spring', [])
    print(f"\nFound {len(spring_funcs)} Spring.* functions in Lua")

    total_rust = sum(len(funcs) for funcs in rust_funcs.values())
    print(f"Found {total_rust} functions in Rust across {len(rust_funcs)} modules")

    print("\nMatching functions...")

    potential_matches = []
    for lua_func in spring_funcs:
        match = find_best_match(lua_func, rust_funcs)
        if match:
            rust_func, confidence, module = match
            rust_full = f"{module}.{rust_func['name']}"
            potential_matches.append((lua_func, rust_func, rust_full, confidence, module))

    potential_matches.sort(key=lambda x: -x[3])

    matched = []
    uncertain = []
    unmatched_lua = []
    used_rust = set()

    for lua_func, rust_func, rust_full, confidence, module in potential_matches:
        if rust_full in used_rust:
            unmatched_lua.append(lua_func)
            continue

        used_rust.add(rust_full)
        if confidence == 1.0:
            matched.append((lua_func, rust_full, rust_func['return'], confidence, rust_func))
        else:
            uncertain.append((lua_func, rust_full, rust_func['return'], confidence, rust_func))

    matched_lua = set(lua['name'] for lua, _, _, _, _ in matched) | set(lua['name'] for lua, _, _, _, _ in uncertain)
    unmatched_lua_names = set(lua['name'] if isinstance(lua, dict) else lua for lua in unmatched_lua)
    for lua_func in spring_funcs:
        if lua_func['name'] not in matched_lua and lua_func['name'] not in unmatched_lua_names:
            unmatched_lua.append(lua_func['name'])
            unmatched_lua_names.add(lua_func['name'])

    matched_rust = set(rust_full for _, rust_full, _, _, _ in matched)
    all_rust = set()
    for module, funcs in rust_funcs.items():
        for func in funcs:
            all_rust.add(f"{module}.{func['name']}")

    unmatched_rust = all_rust - matched_rust
    param_results = []
    for lua_func, rust_full, _, _, rust_meta in matched + uncertain:
        ok, msg = compare_function_params(lua_func, rust_meta)
        param_results.append((ok, msg))
    param_ok = sum(1 for ok, _ in param_results if ok)
    count_mismatches = sum(1 for ok, msg in param_results if not ok and msg.startswith('count mismatch'))
    type_mismatches = sum(1 for ok, msg in param_results if not ok and not msg.startswith(('shape mismatch', 'count mismatch')))

    output_file = rust_dir / 'api_comparison.md'
    todo_file = rust_dir / 'PORTING_TODO.txt'

    with open(output_file, 'w') as f:
        f.write('# Lua ↔ Rust API Comparison Report\n\n')
        f.write('## Summary Statistics\n\n')

        f.write(f'**Lua API (Spring.* callouts only):**\n')
        f.write(f'- Total: {len(spring_funcs)} functions\n')
        f.write(f'- Matched (perfect 1.0): {len(matched)} ({100*len(matched)/len(spring_funcs):.1f}%)\n')
        f.write(f'- Uncertain matches (<1.0): {len(uncertain)} ({100*len(uncertain)/len(spring_funcs):.1f}%)\n')
        f.write(f'- Unmatched: {len(unmatched_lua)} ({100*len(unmatched_lua)/len(spring_funcs):.1f}%)\n\n')

        f.write(f'**Rust API:**\n')
        f.write(f'- Total: {total_rust} functions across {len(rust_funcs)} modules\n')
        f.write(f'- Matched to Lua: {len(matched)} ({100*len(matched)/total_rust:.1f}%)\n')
        f.write(f'- Rust-only: {len(unmatched_rust)} ({100*len(unmatched_rust)/total_rust:.1f}%)\n\n')

        f.write(f'**Parameter comparison (matched functions):**\n')
        f.write(f'- Parameter matches: {param_ok}\n')
        f.write(f'- Count mismatches: {count_mismatches}\n')
        f.write(f'- Type mismatches: {type_mismatches}\n\n')

        f.write('---\n\n')

        f.write('## Matched Functions (Perfect Match)\n\n')
        f.write('Functions with perfect 1.0 confidence match:\n\n')

        for lua_func, rust_func, return_type, confidence, rust_meta in sorted(matched, key=lambda x: x[0]['name']):
            ok, msg = compare_function_params(lua_func, rust_meta)
            suffix = "" if ok else f" (param mismatch: {msg})"
            f.write(f'- `{lua_func["name"]}` → `{rust_func}`{suffix}\n')

        f.write(f'\n**Total: {len(matched)}**\n\n')

        f.write('---\n\n')

        f.write('## Uncertain Matches\n\n')
        f.write('Imperfect matches with <1.0 confidence (likely incorrect):\n\n')

        for lua_func, rust_func, return_type, confidence, rust_meta in sorted(uncertain, key=lambda x: -x[3]):
            ok, msg = compare_function_params(lua_func, rust_meta)
            suffix = "" if ok else f"; params: {msg}"
            f.write(f'- `{lua_func["name"]}` → `{rust_func}` (confidence: {confidence:.2f}{suffix})\n')

        f.write(f'\n**Total: {len(uncertain)}**\n\n')

        f.write('---\n\n')

        f.write('## Unmatched Lua Functions\n\n')
        f.write('Functions in Lua API with no Rust equivalent:\n\n')

        for lua_func in sorted(unmatched_lua_names):
            f.write(f'- `{lua_func}`\n')

        f.write(f'\n**Total unmatched: {len(unmatched_lua)}**\n\n')

        f.write('---\n\n')

        f.write('## Rust-Only Functions\n\n')
        f.write('Functions in Rust API with no Lua equivalent:\n\n')

        for rust_func in sorted(unmatched_rust):
            f.write(f'- `{rust_func}`\n')

        f.write(f'\n**Total Rust-only: {len(unmatched_rust)}**\n')

    todo_lines = ['Current API port TODO (one function per line; generated from api_comparison.md)']
    for lua_func, _, _, _, rust_meta in sorted(matched, key=lambda x: x[0]['name']):
        line = param_mismatch_msg(lua_func, rust_meta)
        if line:
            todo_lines.append(line)
    for lua_func, _, _, _, rust_meta in sorted(uncertain, key=lambda x: x[0]['name']):
        line = param_mismatch_msg(lua_func, rust_meta)
        if line:
            todo_lines.append(line)
    todo_file.write_text('\n'.join(todo_lines) + '\n')

    print(f"\n✓ Wrote comparison report to {output_file}")
    print(f"✓ Wrote parameter TODO to {todo_file}")
    print(f"\nSummary:")
    print(f"  Perfect matches (1.0): {len(matched)}/{len(spring_funcs)} ({100*len(matched)/len(spring_funcs):.1f}%)")
    print(f"  Uncertain matches (<1.0): {len(uncertain)} ({100*len(uncertain)/len(spring_funcs):.1f}%)")
    print(f"  Lua-only: {len(unmatched_lua)}")
    print(f"  Rust-only: {len(unmatched_rust)}")


if __name__ == '__main__':
    main()
