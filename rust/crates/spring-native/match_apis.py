#!/usr/bin/env python3
"""
Match Lua API functions to Rust API functions and generate comparison report.
"""

import re
from pathlib import Path
from typing import List, Dict, Set, Tuple, Optional
from difflib import SequenceMatcher

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

    # Focus on Spring.* callouts (the main API)
    spring_funcs = lua_funcs.get('Spring', [])
    print(f"\nFound {len(spring_funcs)} Spring.* functions in Lua")

    total_rust = sum(len(funcs) for funcs in rust_funcs.values())
    print(f"Found {total_rust} functions in Rust across {len(rust_funcs)} modules")

    # Match Spring functions to Rust functions (1-to-1 mapping)
    print("\nMatching functions...")

    # First, find all potential matches
    potential_matches = []
    for lua_func in spring_funcs:
        match = find_best_match(lua_func, rust_funcs)
        if match:
            rust_full, return_type, confidence = match
            potential_matches.append((lua_func, rust_full, return_type, confidence))

    # Sort by confidence (highest first) to resolve conflicts
    potential_matches.sort(key=lambda x: -x[3])

    # Enforce 1-to-1 mapping
    matched = []
    uncertain = []
    unmatched_lua = []
    used_rust = set()

    for lua_func, rust_full, return_type, confidence in potential_matches:
        if rust_full not in used_rust:
            used_rust.add(rust_full)
            if confidence == 1.0:  # Only perfect matches
                matched.append((lua_func, rust_full, return_type, confidence))
            else:  # Imperfect matches (likely wrong)
                uncertain.append((lua_func, rust_full, return_type, confidence))
        else:
            # This Rust function already matched to another Lua function
            unmatched_lua.append(lua_func)

    # Add Lua functions that didn't match anything
    matched_lua = set(lua for lua, _, _, _ in matched) | set(lua for lua, _, _, _ in uncertain)
    for lua_func in spring_funcs:
        if lua_func not in matched_lua and lua_func not in unmatched_lua:
            unmatched_lua.append(lua_func)

    # Find Rust functions with no Lua equivalent
    matched_rust = set(rust_full for _, rust_full, _, _ in matched)
    all_rust = set()
    for funcs in rust_funcs.values():
        for rust_full, _ in funcs:
            all_rust.add(rust_full)

    unmatched_rust = all_rust - matched_rust

    # Generate report
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

        for lua_func, rust_func, return_type, confidence in sorted(matched, key=lambda x: x[0]):
            f.write(f'- `{lua_func}` → `{rust_func}`\n')

        f.write(f'\n**Total: {len(matched)}**\n\n')

        f.write('---\n\n')

        f.write('## Uncertain Matches\n\n')
        f.write('Imperfect matches with <1.0 confidence (likely incorrect):\n\n')

        for lua_func, rust_func, return_type, confidence in sorted(uncertain, key=lambda x: -x[3]):
            f.write(f'- `{lua_func}` → `{rust_func}` (confidence: {confidence:.2f})\n')

        f.write(f'\n**Total: {len(uncertain)}**\n\n')

        f.write('---\n\n')

        f.write('## Unmatched Lua Functions\n\n')
        f.write('Functions in Lua API with no Rust equivalent:\n\n')

        for lua_func in sorted(unmatched_lua):
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

def parse_lua_functions(filepath: Path) -> Dict[str, List[str]]:
    """Parse lua_functions.md and extract functions by namespace."""
    functions = {}
    current_namespace = None

    with open(filepath, 'r') as f:
        for line in f:
            line = line.strip()

            # Match namespace headers: ### Spring (761 functions)
            ns_match = re.match(r'###\s+(\w+)\s+\((\d+)\s+functions\)', line)
            if ns_match:
                current_namespace = ns_match.group(1)
                functions[current_namespace] = []
                continue

            # Match function: - `Spring.FunctionName`
            func_match = re.match(r'-\s+`([^`]+)`', line)
            if func_match and current_namespace:
                functions[current_namespace].append(func_match.group(1))

    return functions

def parse_rust_functions(filepath: Path) -> Dict[str, List[Tuple[str, str]]]:
    """Parse rust_functions.md and extract functions by module."""
    functions = {}
    current_module = None

    with open(filepath, 'r') as f:
        for line in f:
            line = line.strip()

            # Match module headers: ## Camera (10 functions)
            mod_match = re.match(r'##\s+(\w+)\s+\((\d+)\s+functions\)', line)
            if mod_match:
                current_module = mod_match.group(1)
                functions[current_module] = []
                continue

            # Match function: - `Camera.get_camera_direction` → `Result<sys::Float3, Error>`
            func_match = re.match(r'-\s+`([^`]+)`\s+→\s+`([^`]+)`', line)
            if func_match and current_module:
                full_name = func_match.group(1)
                return_type = func_match.group(2)
                functions[current_module].append((full_name, return_type))

    return functions

def find_best_match(lua_func: str, rust_funcs: Dict[str, List[Tuple[str, str]]]) -> Optional[Tuple[str, str, float]]:
    """
    Find the best matching Rust function for a Lua function.
    Returns (rust_full_name, return_type, confidence) or None.
    """
    lua_normalized = normalize_name(lua_func)

    best_match = None
    best_score = 0.0

    for module, funcs in rust_funcs.items():
        for rust_full, return_type in funcs:
            rust_normalized = normalize_name(rust_full)

            # Calculate similarity
            similarity = SequenceMatcher(None, lua_normalized, rust_normalized).ratio()

            if similarity > best_score:
                best_score = similarity
                best_match = (rust_full, return_type, similarity)

    # Return matches above 0.8 threshold (will be categorized as certain/uncertain later)
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
    # Strip namespace
    if '.' in name:
        name = name.split('.')[-1]

    # Convert camelCase to snake_case
    # Insert underscore before uppercase letters
    result = re.sub('([a-z0-9])([A-Z])', r'\1_\2', name)
    result = result.lower()

    return result

if __name__ == '__main__':
    main()
