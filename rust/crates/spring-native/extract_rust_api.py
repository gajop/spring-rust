#!/usr/bin/env python3
"""
Extract all public API functions from the spring-native Rust crate.
Parses the generated code to extract function signatures.
"""

import re
import os
from pathlib import Path
from typing import List, Dict, Tuple

def extract_functions_from_file(filepath: Path) -> List[Dict]:
    """Extract public function signatures from a Rust file."""
    functions = []

    with open(filepath, 'r') as f:
        content = f.read()

    # Pattern to match public functions
    # pub fn function_name(&self, param: Type, ...) -> Result<ReturnType, Error>
    pattern = r'pub\s+fn\s+(?:r#)?([a-z_][a-z0-9_]*)\s*\(([^)]*)\)\s*(?:->\s*([^{;]+))?'

    for match in re.finditer(pattern, content):
        func_name = match.group(1)
        param_str = match.group(2)
        return_type = match.group(3).strip() if match.group(3) else '()'

        params = []
        if param_str.strip():
            # Split top-level commas (simplistic but fine for our generated signatures)
            raw_params = [p.strip() for p in param_str.split(',') if p.strip()]
            for p in raw_params:
                # skip self receivers
                if p in ('&self', 'self', '&mut self'):
                    continue
                if ':' in p:
                    name, ty = p.split(':', 1)
                    params.append({'name': name.strip(), 'type': ty.strip()})
                else:
                    params.append({'name': p.strip(), 'type': ''})

        functions.append({
            'name': func_name,
            'return_type': return_type,
            'file': filepath.name,
            'params': params
        })

    return functions

def extract_struct_methods(out_dir: Path) -> Dict[str, List[Dict]]:
    """Extract methods from all generated API files."""
    api_methods = {}

    # Find all *_generated.rs files
    for gen_file in out_dir.glob('*_generated.rs'):
        struct_name = gen_file.stem.replace('_generated', '').replace('_', ' ').title().replace(' ', '')
        functions = extract_functions_from_file(gen_file)

        if functions:
            api_methods[struct_name] = functions

    return api_methods

def extract_from_source(src_dir: Path) -> Dict[str, List[Dict]]:
    """Extract methods from source .rs files."""
    api_methods = {}

    # Files that contain the main API structs
    api_files = [
        'camera.rs', 'config.rs', 'display.rs', 'feature_defs.rs', 'features.rs',
        'game.rs', 'input.rs', 'los.rs', 'math_extra.rs', 'messages.rs',
        'metal_map.rs', 'move_ctrl.rs', 'path_finder.rs', 'player.rs',
        'projectiles.rs', 'rules_params.rs', 'selection.rs', 'sound.rs',
        'synced_ctrl.rs', 'teams.rs', 'terrain.rs', 'tracing.rs', 'unit_defs.rs',
        'units_commands.rs', 'units_info.rs', 'units_pieces.rs', 'units_query.rs',
        'units_weapons.rs', 'utils.rs', 'vfs.rs', 'weapon_defs.rs', 'memory.rs',
        'unsynced_read.rs', 'unsynced_ctrl.rs', 'lights.rs', 'icons.rs',
        'markers.rs', 'ground_decals.rs', 'system_control.rs', 'profiling.rs',
    ]

    for filename in api_files:
        filepath = src_dir / filename
        if filepath.exists():
            struct_name = filename.replace('.rs', '').replace('_', ' ').title().replace(' ', '')
            functions = extract_functions_from_file(filepath)

            if functions:
                api_methods[struct_name] = functions

    return api_methods

def main():
    # Find the build output directory
    rust_dir = Path(__file__).parent
    target_dir = rust_dir.parent.parent / 'target'

    # Try release first, fall back to debug
    out_dirs = []
    for build_type in ['release', 'debug']:
        build_path = target_dir / build_type / 'build'
        if build_path.exists():
            # Find the spring-native (not spring-native-sys) build directory
            for potential_dir in build_path.glob('spring-native-*/out'):
                if 'spring-native-sys' not in str(potential_dir):
                    out_dirs.append(potential_dir)

    if not out_dirs:
        print("Could not find build output directory")
        return

    # Pick the directory with the most recently modified .rs file
    def get_newest_file_time(d):
        rs_files = list(d.glob('*.rs'))
        if not rs_files:
            return 0
        return max(f.stat().st_mtime for f in rs_files)

    out_dir = max(out_dirs, key=get_newest_file_time)
    print(f"Reading generated code from: {out_dir}")
    print(f"Found {len(list(out_dir.glob('*.rs')))} generated files")

    # Extract from generated files
    generated_methods = extract_struct_methods(out_dir)

    # Extract from source files
    src_dir = rust_dir / 'src'
    source_methods = extract_from_source(src_dir)

    # Combine both
    all_methods = {**source_methods, **generated_methods}

    # Write to markdown
    output_file = rust_dir / 'rust_functions.md'
    with open(output_file, 'w') as f:
        f.write('# Rust API Functions\n\n')
        f.write(f'Total APIs: {len(all_methods)}\n\n')

        total_functions = sum(len(funcs) for funcs in all_methods.values())
        f.write(f'Total Functions: {total_functions}\n\n')

        f.write('---\n\n')

        for struct_name in sorted(all_methods.keys()):
            functions = all_methods[struct_name]
            f.write(f'## {struct_name} ({len(functions)} functions)\n\n')

            for func in sorted(functions, key=lambda x: x['name']):
                param_str = ", ".join(f'{p["name"]}:{p["type"]}' for p in func.get("params", []))
                f.write(f'- `{struct_name}.{func["name"]}` (params: {param_str}) → `{func["return_type"]}`\n')

            f.write('\n')

    print(f"Wrote {total_functions} functions from {len(all_methods)} APIs to {output_file}")

if __name__ == '__main__':
    main()
