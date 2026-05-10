#!/usr/bin/env python3
"""
Match Lua API functions to Rust API functions and generate comparison report.
"""

import re
from pathlib import Path
from typing import List, Dict, Tuple, Optional
from difflib import SequenceMatcher


def expand_params(params: List[Dict]) -> List[str]:
    """Expand vector-like params (float2/3/4) into scalar slots for count comparison."""
    expanded = []
    for p in params:
        ptype = p.get('type', '').strip()
        lower = ptype.lower()
        mult = 1
        for dim, key in [(4, 'float4'), (3, 'float3'), (2, 'float2')]:
            if key in lower:
                mult = dim
                break
        expanded.extend([ptype] * mult)
    return expanded


def bucket_type(ptype: str) -> str:
    """Group types: treat Lua 'number' as floating-point, keep ints separate."""
    lt = ptype.lower()
    if not lt:
        return ""
    if "number" in lt or "float" in lt or "double" in lt:
        return "float"
    if "int" in lt or "uint" in lt or "size" in lt:
        return "int"
    if "bool" in lt:
        return "bool"
    if "string" in lt:
        return "string"
    return lt


def compare_params(lua_params: List[Dict], rust_params: List[Dict]) -> Tuple[bool, str]:
    lua_expanded = expand_params(lua_params)
    rust_expanded = expand_params(rust_params)

    if len(lua_expanded) != len(rust_expanded):
        return False, f"count mismatch (lua={len(lua_expanded)}, rust={len(rust_expanded)})"

    diffs = []
    for idx, (ltype, rtype) in enumerate(zip(lua_expanded, rust_expanded)):
        ltype = ltype.strip()
        rtype = rtype.strip()
        lb = bucket_type(ltype)
        rb = bucket_type(rtype)

        if lb and rb:
            if lb != rb:
                diffs.append(f"p{idx+1} type {ltype}!={rtype}")
        elif ltype and rtype and ltype != rtype:
            diffs.append(f"p{idx+1} type {ltype}!={rtype}")

    if diffs:
        return False, "; ".join(diffs)

    return True, "match"


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

    output_file = rust_dir / 'api_comparison.md'

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

        f.write('---\n\n')

        f.write('## Matched Functions (Perfect Match)\n\n')
        f.write('Functions with perfect 1.0 confidence match:\n\n')

        for lua_func, rust_func, return_type, confidence, rust_meta in sorted(matched, key=lambda x: x[0]['name']):
            ok, msg = compare_params(lua_func.get('params', []), rust_meta.get('params', []))
            suffix = "" if ok else f" (param mismatch: {msg})"
            f.write(f'- `{lua_func["name"]}` → `{rust_func}`{suffix}\n')

        f.write(f'\n**Total: {len(matched)}**\n\n')

        f.write('---\n\n')

        f.write('## Uncertain Matches\n\n')
        f.write('Imperfect matches with <1.0 confidence (likely incorrect):\n\n')

        for lua_func, rust_func, return_type, confidence, rust_meta in sorted(uncertain, key=lambda x: -x[3]):
            ok, msg = compare_params(lua_func.get('params', []), rust_meta.get('params', []))
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

        f.write(f'\n**Total Rust-only: {len(unmatched_rust)}**\n\n')

    print(f"\n✓ Wrote comparison report to {output_file}")
    print(f"\nSummary:")
    print(f"  Perfect matches (1.0): {len(matched)}/{len(spring_funcs)} ({100*len(matched)/len(spring_funcs):.1f}%)")
    print(f"  Uncertain matches (<1.0): {len(uncertain)} ({100*len(uncertain)/len(spring_funcs):.1f}%)")
    print(f"  Lua-only: {len(unmatched_lua)}")
    print(f"  Rust-only: {len(unmatched_rust)}")


if __name__ == '__main__':
    main()
