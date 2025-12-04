#!/usr/bin/env python3
"""
Extract all Lua API functions from the Recoil Engine documentation.
Scrapes https://recoilengine.org/docs/lua-api/
"""

import urllib.request
import urllib.parse
import re
import time
import json
from pathlib import Path
from typing import List, Dict, Set
from datetime import datetime, timedelta

def fetch_page(url: str, cache_file: Path, cache_days: int = 1) -> str:
    """Fetch a webpage and return its content, using cache if available."""
    # Check if cache exists and is fresh
    if cache_file.exists():
        cache_age = time.time() - cache_file.stat().st_mtime
        if cache_age < cache_days * 86400:  # 86400 seconds in a day
            print(f"Using cached page (age: {cache_age/3600:.1f} hours)")
            with open(cache_file, 'r', encoding='utf-8') as f:
                return f.read()
        else:
            print(f"Cache expired (age: {cache_age/86400:.1f} days)")

    # Fetch from web
    print(f"Fetching {url}...")
    try:
        req = urllib.request.Request(url, headers={'User-Agent': 'Mozilla/5.0'})
        with urllib.request.urlopen(req, timeout=10) as response:
            content = response.read().decode('utf-8')

        # Save to cache
        cache_file.parent.mkdir(parents=True, exist_ok=True)
        with open(cache_file, 'w', encoding='utf-8') as f:
            f.write(content)
        print(f"Cached page to {cache_file}")

        return content
    except Exception as e:
        print(f"Error fetching {url}: {e}")
        return ""

def extract_lua_functions_from_markdown(content: str) -> List[Dict]:
    """
    Extract function signatures from markdown-like content.
    Looking for patterns like:
    - Spring.FunctionName
    - Spring.FunctionName()
    - Spring.FunctionName(param1, param2)
    """
    functions = []
    seen = set()

    # Pattern 1: Function headings/anchors
    # Matches: #Spring.GetGameFrame, #Spring.GetUnitPosition, etc.
    anchor_pattern = r'#(Spring|gl|RmlUi|VFS|Script)\.([A-Za-z][A-Za-z0-9]*)'
    for match in re.finditer(anchor_pattern, content):
        namespace = match.group(1)
        func_name = match.group(2)
        full_name = f"{namespace}.{func_name}"

        if full_name not in seen:
            seen.add(full_name)
            functions.append({
                'namespace': namespace,
                'name': func_name,
                'full_name': full_name
            })

    # Pattern 2: Function definitions with parentheses
    # Matches: Spring.FunctionName(...), gl.DrawGroundCircle(...), etc.
    func_pattern = r'\b(Spring|gl|RmlUi|VFS|Script)\.([A-Za-z][A-Za-z0-9]*)\s*\('
    for match in re.finditer(func_pattern, content):
        namespace = match.group(1)
        func_name = match.group(2)
        full_name = f"{namespace}.{func_name}"

        if full_name not in seen:
            seen.add(full_name)
            functions.append({
                'namespace': namespace,
                'name': func_name,
                'full_name': full_name
            })

    return functions

def extract_callins(content: str) -> List[Dict]:
    """
    Extract call-in functions (callbacks from engine to plugin).
    These don't have a namespace prefix.
    """
    callins = []
    seen = set()

    # Pattern for callins - typically camelCase without namespace
    # Look for function definitions in callin sections
    callin_pattern = r'function\s+([A-Z][A-Za-z0-9]*)\s*\('
    for match in re.finditer(callin_pattern, content):
        func_name = match.group(1)

        if func_name not in seen:
            seen.add(func_name)
            callins.append({
                'namespace': 'Callins',
                'name': func_name,
                'full_name': func_name
            })

    # Also look for anchor-style definitions
    callin_anchor_pattern = r'#([A-Z][A-Za-z0-9]*)\b'
    for match in re.finditer(callin_anchor_pattern, content):
        func_name = match.group(1)

        # Filter out common non-function words
        if func_name not in seen and func_name not in ['Spring', 'RmlUi', 'Script', 'VFS', 'The', 'This', 'For']:
            seen.add(func_name)
            callins.append({
                'namespace': 'Callins',
                'name': func_name,
                'full_name': func_name
            })

    return callins

def main():
    rust_dir = Path(__file__).parent
    cache_file = rust_dir / '.cache' / 'lua_api.html'

    print("Fetching Lua API documentation...")
    url = 'https://recoilengine.org/docs/lua-api/'
    content = fetch_page(url, cache_file, cache_days=1)

    if not content:
        print("Failed to fetch documentation")
        return

    print(f"Processing {len(content)} bytes")

    # Extract callouts (Spring.*, gl.*, etc.)
    print("Extracting callout functions...")
    callouts = extract_lua_functions_from_markdown(content)

    # Extract callins (GameStart, UnitCreated, etc.)
    print("Extracting callin functions...")
    callins = extract_callins(content)

    # Combine all functions
    all_functions = callouts + callins

    # Group by namespace
    by_namespace = {}
    for func in all_functions:
        ns = func['namespace']
        if ns not in by_namespace:
            by_namespace[ns] = []
        by_namespace[ns].append(func)

    # Write to markdown
    output_file = rust_dir / 'lua_functions.md'

    with open(output_file, 'w') as f:
        f.write('# Lua API Functions\n\n')
        f.write(f'Total Namespaces: {len(by_namespace)}\n\n')

        total = sum(len(funcs) for funcs in by_namespace.values())
        f.write(f'Total Functions: {total}\n\n')

        callout_count = len(callouts)
        callin_count = len(callins)
        f.write(f'Callouts (Plugin → Engine): {callout_count}\n')
        f.write(f'Callins (Engine → Plugin): {callin_count}\n\n')

        f.write('---\n\n')

        # Write callouts first
        f.write('## Callouts (Plugin → Engine)\n\n')
        for namespace in sorted(by_namespace.keys()):
            if namespace != 'Callins':
                functions = by_namespace[namespace]
                f.write(f'### {namespace} ({len(functions)} functions)\n\n')

                for func in sorted(functions, key=lambda x: x['name']):
                    f.write(f'- `{func["full_name"]}`\n')

                f.write('\n')

        # Write callins
        if 'Callins' in by_namespace:
            f.write('## Callins (Engine → Plugin)\n\n')
            functions = by_namespace['Callins']
            f.write(f'Total: {len(functions)} functions\n\n')

            for func in sorted(functions, key=lambda x: x['name']):
                f.write(f'- `{func["full_name"]}`\n')

            f.write('\n')

    print(f"\nWrote {total} functions ({callout_count} callouts, {callin_count} callins) to {output_file}")
    print(f"Namespaces: {', '.join(sorted(by_namespace.keys()))}")

if __name__ == '__main__':
    main()
