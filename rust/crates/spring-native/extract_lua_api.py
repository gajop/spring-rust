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
import html
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
        if cache_file.exists():
            print("Falling back to cached content despite expiry")
            with open(cache_file, 'r', encoding='utf-8') as f:
                return f.read()
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

    # First, collect parameter info from @function blocks
    func_blocks = []
    func_pattern = re.compile(r'@function\s+(Spring|gl|RmlUi|VFS|Script)\.([A-Za-z][A-Za-z0-9_]*)', re.MULTILINE)
    matches = list(func_pattern.finditer(content))
    for idx, match in enumerate(matches):
        start = match.start()
        end = matches[idx + 1].start() if idx + 1 < len(matches) else len(content)
        func_blocks.append((match.group(1), match.group(2), content[start:end]))

    params_by_name = {}
    param_regex = re.compile(r'@param\s+([^\s]+)\s+([^\n]+)')
    html_param_regex = re.compile(r'<dt><code>([^<]+)</code><a [^>]+href=#([^>]+)>([^<]+)</a>')
    sig_regex = re.compile(r'Spring\.[A-Za-z0-9_]+\(([^)]*)\)')

    def clean_type(t: str) -> str:
        t = re.sub(r'<[^>]+>', '', t)
        return html.unescape(t).strip()

    def parse_param_doc_text(text: str) -> str:
        text = text.strip()
        if not text:
            return ''
        # Most source docs use "@param name type description". Keep the type
        # token, but preserve braced table types that contain spaces.
        if text.startswith('{'):
            depth = 0
            for idx, ch in enumerate(text):
                if ch == '{':
                    depth += 1
                elif ch == '}':
                    depth -= 1
                    if depth == 0:
                        return text[:idx + 1].strip()
        return text.split()[0]

    def extract_html_params(block: str) -> List[Dict]:
        params = []
        for dt in re.findall(r'<dt>(.*?)</dt>', block, flags=re.DOTALL):
            code_match = re.search(r'<code>(.*?)</code>', dt, flags=re.DOTALL)
            if not code_match:
                continue

            name_part = dt[code_match.end():]
            name_match = re.search(r'href=#([^>\s]+)>([^<]*)</a>', name_part, flags=re.DOTALL)
            if not name_match:
                continue
            anchor = name_match.group(1)
            if '-returns' in anchor or '-params.' not in anchor:
                continue

            params.append({
                'name': html.unescape(name_match.group(2)).strip(),
                'type': clean_type(code_match.group(1)),
            })
        return params
    for namespace, func_name, block in func_blocks:
        params = []
        for p in param_regex.finditer(block):
            params.append({'name': p.group(1), 'type': clean_type(parse_param_doc_text(p.group(2)))})

        if not params:
            params = extract_html_params(block)

        if not params:
            sig_match = sig_regex.search(block)
            if sig_match:
                raw = sig_match.group(1).strip()
                if raw:
                    for idx, token in enumerate([t.strip() for t in raw.split(',') if t.strip()]):
                        params.append({'name': f'arg{idx+1}', 'type': clean_type(token)})

        params_by_name[f"{namespace}.{func_name}"] = params

    def extract_params_from_section(full_name: str) -> List[Dict]:
        start = content.find(f"id={full_name}")
        if start == -1:
            start = content.find(full_name)
            if start == -1:
                return []
        next_h4 = content.find("<h4", start + 1)
        block = content[start: next_h4 if next_h4 != -1 else len(content)]

        params = []
        params = extract_html_params(block)

        if not params:
            sig_match = re.search(rf'{re.escape(full_name.split(".")[1])}\(([^)]*)\)', block)
            if sig_match:
                raw = sig_match.group(1).strip()
                if raw:
                    for idx, token in enumerate([t.strip() for t in raw.split(',') if t.strip()]):
                        params.append({'name': f'arg{idx+1}', 'type': token})
        return params

    # Scan signatures from code blocks / text
    sig_scan_pattern = re.compile(r'\b(Spring|gl|RmlUi|VFS|Script)\.([A-Za-z0-9_]+)\(([^)]*)\)')
    for match in sig_scan_pattern.finditer(content):
        namespace = match.group(1)
        func_name = match.group(2)
        if func_name.endswith(('_arguments', '_returns')):
            continue
        full_name = f"{namespace}.{func_name}"

        if full_name in seen:
            continue

        seen.add(full_name)

        params = extract_params_from_section(full_name)
        if not params:
            params = params_by_name.get(full_name, [])
        if not params:
            raw = match.group(3).strip()
            if raw:
                for idx, token in enumerate([t.strip() for t in raw.split(',') if t.strip()]):
                    params.append({'name': f'arg{idx+1}', 'type': clean_type(token)})

        functions.append({
            'namespace': namespace,
            'name': func_name,
            'full_name': full_name,
            'params': params
        })

    return functions

def extract_callins(content: str) -> List[Dict]:
    """
    Extract call-in functions (callbacks from engine to plugin) from the documented
    Callins/SyncedCallins/UnsyncedCallins sections. Avoids picking up anchor noise
    like L50/L500 and table headings.
    """
    callins = []
    seen = set()

    prefixes = [
        'Callins',
        'SyncedCallins',
        'UnsyncedCallins',
        'RulesSyncedCallins',
        'RulesUnsyncedCallins',
    ]

    for prefix in prefixes:
        pattern = rf'{prefix}:([A-Za-z0-9_]+)'
        for name in re.findall(pattern, content):
            # Skip anchors for params/returns and numeric noise
            if name.endswith(('_arguments', '_returns')):
                continue
            if re.match(r'^L\d+$', name):
                continue

            full = f"{prefix}.{name}"
            if full in seen:
                continue

            seen.add(full)
            callins.append({
                'namespace': 'Callins',
                'name': full,
                'full_name': full,
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
                    params = func.get("params", [])
                    param_str = ", ".join(f'{p["name"]}:{p["type"]}' for p in params) if params else ""
                    suffix = f" (params: {param_str})" if param_str else " (params: )"
                    f.write(f'- `{func["full_name"]}`{suffix}\n')

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
