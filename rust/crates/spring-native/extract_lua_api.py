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

def extract_lua_functions_from_markdown(content: str, infer_signatures: bool = True) -> List[Dict]:
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
    func_pattern = re.compile(
        r'@function\s+((?:Spring|gl|RmlUi|VFS|Script)(?:[.:][A-Za-z_][A-Za-z0-9_]*)+)',
        re.MULTILINE,
    )
    matches = list(func_pattern.finditer(content))
    for idx, match in enumerate(matches):
        start = match.start()
        end = matches[idx + 1].start() if idx + 1 < len(matches) else len(content)
        full_name = match.group(1).replace(':', '.')
        namespace, func_name = full_name.split('.', 1)
        func_blocks.append((namespace, func_name, content[start:end]))

    params_by_name = {}
    param_regex = re.compile(r'@param\s+([^\s]+)\s+([^\n]+)')
    sig_regex = re.compile(r'(?:Spring|gl|RmlUi|VFS|Script)(?:[.:][A-Za-z0-9_]+)+\(([^)]*)\)')

    def clean_type(t: str) -> str:
        t = re.sub(r'<[^>]+>', '', t)
        return html.unescape(t).strip()

    def split_top_level_commas(text: str) -> List[str]:
        parts = []
        start = 0
        depth = 0
        openers = {'(': ')', '<': '>', '{': '}', '[': ']'}
        closers = set(openers.values())
        for idx, ch in enumerate(text):
            if ch in openers:
                depth += 1
            elif ch in closers and depth > 0:
                depth -= 1
            elif ch == ',' and depth == 0:
                part = text[start:idx].strip()
                if part:
                    parts.append(part)
                start = idx + 1
        tail = text[start:].strip()
        if tail:
            parts.append(tail)
        return parts

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
        if '<' in text:
            depth = 0
            for idx, ch in enumerate(text):
                if ch == '<':
                    depth += 1
                elif ch == '>':
                    depth -= 1
                    if depth == 0:
                        return text[:idx + 1].strip()
        return text.split()[0]

    def extract_html_params(block: str) -> List[Dict]:
        params = []
        for dt in re.findall(r'<dt[^>]*>(.*?)</dt>', block, flags=re.DOTALL):
            code_match = re.search(r'<code[^>]*>(.*?)</code>', dt, flags=re.DOTALL)
            if not code_match:
                continue

            name_part = dt[code_match.end():]
            name_match = re.search(r'href=["\']?#([^"\'\s>]+)["\']?[^>]*>(.*?)</a>', name_part, flags=re.DOTALL)
            if not name_match:
                continue
            anchor = name_match.group(1)
            if '-returns' in anchor or '-params.' not in anchor:
                continue

            params.append({
                'name': clean_type(name_match.group(2)),
                'type': clean_type(code_match.group(1)),
            })
        return params
    for namespace, func_name, block in func_blocks:
        params = []
        for p in param_regex.finditer(block):
            params.append({'name': p.group(1), 'type': clean_type(parse_param_doc_text(p.group(2)))})

        if not params:
            params = extract_html_params(block)

        if infer_signatures and not params:
            sig_match = sig_regex.search(block)
            if sig_match:
                raw = sig_match.group(1).strip()
                if raw:
                    for idx, token in enumerate(split_top_level_commas(raw)):
                        params.append({'name': f'arg{idx+1}', 'type': clean_type(token)})

        params_by_name[f"{namespace}.{func_name}"] = params

    def extract_params_from_section(full_name: str) -> List[Dict]:
        span_match = re.search(rf'\bid=["\']?{re.escape(full_name)}(?=["\'\s>])', content)
        if span_match:
            heading_start = content.rfind("<h4", 0, span_match.start())
            start = heading_start if heading_start != -1 else span_match.start()
        else:
            start = content.find(full_name)
            if start == -1:
                return []

        next_heading = re.search(r'<h4[^>]*>', content[start + 1:], flags=re.DOTALL)
        end = start + 1 + next_heading.start() if next_heading else len(content)
        block = content[start:end]

        params = []
        params = extract_html_params(block)

        if infer_signatures and not params:
            sig_match = re.search(rf'{re.escape(full_name.split(".")[-1])}\(([^)]*)\)', block)
            if sig_match:
                raw = sig_match.group(1).strip()
                if raw:
                    for idx, token in enumerate(split_top_level_commas(raw)):
                        params.append({'name': f'arg{idx+1}', 'type': clean_type(token)})
        return params

    # Scan signatures from code blocks / text
    sig_scan_pattern = re.compile(
        r'\b((?:Spring|gl|RmlUi|VFS|Script)(?:[.:][A-Za-z0-9_]+)+)\(([^)]*)\)'
    )
    function_index = {}
    if infer_signatures:
        scan_iter = sig_scan_pattern.finditer(content)
    else:
        scan_iter = []
    for match in scan_iter:
        full_name = match.group(1).replace(':', '.')
        namespace, func_name = full_name.split('.', 1)
        if func_name.endswith(('_arguments', '_returns')):
            continue

        if full_name in seen:
            raw = html.unescape(re.sub(r'<[^>]+>', '', match.group(2))).strip()
            if raw and full_name in function_index:
                current = function_index[full_name].get('params', [])
                candidate = [
                    {'name': f'arg{idx+1}', 'type': clean_type(token)}
                    for idx, token in enumerate(split_top_level_commas(raw))
                ] if raw else []
                if candidate and raw != 'unknown':
                    if any(p.get('type') == 'unknown' for p in current) or len(candidate) > len(current):
                        function_index[full_name]['params'] = candidate
            continue

        seen.add(full_name)

        params = params_by_name.get(full_name, [])
        if not params:
            params = extract_params_from_section(full_name)
        if not params:
            raw = match.group(2).strip()
            if raw:
                for idx, token in enumerate(split_top_level_commas(raw)):
                    params.append({'name': f'arg{idx+1}', 'type': clean_type(token)})

        func = {
            'namespace': namespace,
            'name': func_name,
            'full_name': full_name,
            'params': params
        }
        functions.append(func)
        function_index[full_name] = func

    for namespace, func_name, _block in func_blocks:
        full_name = f"{namespace}.{func_name}"
        if full_name in seen:
            continue
        if func_name.endswith(('_arguments', '_returns')):
            continue

        seen.add(full_name)
        func = {
            'namespace': namespace,
            'name': func_name,
            'full_name': full_name,
            'params': params_by_name.get(full_name, []),
        }
        functions.append(func)
        function_index[full_name] = func

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

def collect_registered_spring_functions(project_root: Path) -> Set[str]:
    """Collect Spring.* functions actually registered by the local Lua bindings."""
    registered = set()
    lua_dir = project_root / 'rts' / 'Lua'
    if not lua_dir.exists():
        return registered

    register_regex = re.compile(r'REGISTER_(?:SCOPED_)?LUA_CFUNC\s*\((?:\s*[A-Za-z0-9_:]+\s*,)?\s*([A-Za-z][A-Za-z0-9_]*)\s*\)')
    for path in lua_dir.glob('Lua*.cpp'):
        try:
            text = path.read_text(encoding='utf-8', errors='ignore')
        except OSError:
            continue
        for match in register_regex.finditer(text):
            registered.add(f"Spring.{match.group(1)}")
    return registered

def extract_source_doc_functions(project_root: Path) -> List[Dict]:
    """Extract local @function docs so newly fixed docs are visible before publishing."""
    functions = []
    source_paths = []

    lua_dir = project_root / 'rts' / 'Lua'
    if lua_dir.exists():
        source_paths.extend(lua_dir.glob('Lua*.cpp'))

    rml_bind_dir = project_root / 'rts' / 'Rml' / 'SolLua' / 'bind'
    if rml_bind_dir.exists():
        source_paths.extend(rml_bind_dir.glob('*.cpp'))

    for path in source_paths:
        try:
            content = path.read_text(encoding='utf-8', errors='ignore')
        except OSError:
            continue
        functions.extend(extract_lua_functions_from_markdown(content, infer_signatures=False))
    return functions

def extract_source_callins(project_root: Path) -> List[Dict]:
    """Extract call-ins documented in the checked-out engine sources.

    The public documentation can lag behind a local engine change.  Keep the
    generated inventory grounded in both sources so an implemented call-in is
    not silently omitted from parity work.
    """
    callins = []
    seen = set()
    lua_dir = project_root / 'rts' / 'Lua'
    if not lua_dir.exists():
        return callins

    pattern = re.compile(
        r'@function\s+((?:Callins|SyncedCallins|UnsyncedCallins):([A-Za-z0-9_]+))'
    )
    for path in lua_dir.rglob('*'):
        if path.suffix not in {'.cpp', '.h'}:
            continue
        try:
            content = path.read_text(encoding='utf-8', errors='ignore')
        except OSError:
            continue
        for match in pattern.finditer(content):
            full_name = match.group(1).replace(':', '.')
            if full_name in seen:
                continue
            seen.add(full_name)
            callins.append({
                'namespace': 'Callins',
                'name': full_name,
                'full_name': full_name,
            })
    return callins

def main():
    rust_dir = Path(__file__).parent
    project_root = rust_dir.parents[2]
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

    source_callins = extract_source_callins(project_root)
    callins_by_name = {func['full_name']: func for func in callins}
    added_source_callins = 0
    for func in source_callins:
        if func['full_name'] not in callins_by_name:
            callins.append(func)
            callins_by_name[func['full_name']] = func
            added_source_callins += 1
    if added_source_callins:
        print(f"Merged local source callins: added {added_source_callins}")

    # Combine all functions
    all_functions = callouts + callins

    registered_spring = collect_registered_spring_functions(project_root)
    if registered_spring:
        before = len(callouts)
        callouts = [
            func for func in callouts
            if func.get('namespace') != 'Spring' or func.get('full_name') in registered_spring
        ]
        removed = before - len(callouts)
        if removed:
            print(f"Filtered {removed} stale Spring docs not present in local registrations")

    source_functions = extract_source_doc_functions(project_root)
    if source_functions:
        source_rml_names = {
            func.get('full_name') for func in source_functions
            if func.get('namespace') == 'RmlUi'
        }
        if source_rml_names:
            before = len(callouts)
            callouts = [
                func for func in callouts
                if func.get('namespace') != 'RmlUi' or func.get('full_name') in source_rml_names
            ]
            removed = before - len(callouts)
            if removed:
                print(f"Filtered {removed} stale RmlUi docs not present in local SolLua bindings")

        by_full_name = {func['full_name']: func for func in callouts}
        added = 0
        updated = 0
        for func in source_functions:
            if func.get('namespace') not in ('Spring', 'RmlUi'):
                continue
            if func.get('namespace') == 'Spring' and registered_spring and func.get('full_name') not in registered_spring:
                continue
            existing = by_full_name.get(func['full_name'])
            if existing is None:
                callouts.append(func)
                by_full_name[func['full_name']] = func
                added += 1
            elif func.get('namespace') == 'RmlUi' or func.get('params'):
                existing['params'] = func['params']
                updated += 1
        if added or updated:
            print(f"Merged local source docs: added {added}, updated {updated}")

    all_functions = callouts + callins

    source_overrides = {
        "Spring.AddGrass": [("x", "number"), ("z", "number"), ("grassValue", "integer?")],
        "Spring.AddUnitIcon": [("iconName", "string"), ("texFile", "string"), ("size", "number?"), ("dist", "number?"), ("radAdjust", "boolean?"), ("u0", "number?"), ("v0", "number?"), ("u1", "number?"), ("v1", "number?")],
        "Spring.AddLightTrackingTarget": [("lightHandle", "integer"), ("objectID", "integer"), ("trackUnit", "boolean"), ("enableTracking", "boolean")],
        "Spring.BuggerOff": [("x", "number"), ("y", "number"), ("z", "number?"), ("radius", "number"), ("teamID", "integer"), ("spherical", "boolean?"), ("forced", "boolean?"), ("excludeUnitID", "integer?"), ("excludeUnitDefIDs", "integer[]?")],
        "Spring.CallCOBScript": [("unitID", "integer"), ("func", "CobFunctionRef"), ("retArgs", "integer"), ("args", "integer[]")],
        "Spring.ClearUnitGoal": [("unitID", "integer"), ("cancelRaw", "boolean?")],
        "Spring.ClearWatchDogTimer": [("threadName", "string?"), ("keepStopped", "boolean?")],
        "Spring.ClosestBuildPos": [("teamID", "integer"), ("unitDefID", "integer"), ("posX", "number"), ("posY", "number"), ("posZ", "number"), ("searchRadius", "number"), ("minDistance", "integer"), ("buildFacing", "integer")],
        "Spring.DeselectUnitMap": [("unitMap", "integer[]")],
        "Spring.DrawUnitCommands": [("unitIDs", "integer[]"), ("tableOrArray", "boolean?"), ("queueDrawDepth", "integer?")],
        "Spring.GetConfigInt": [("name", "string"), ("default", "integer?")],
        "Spring.GetCEGID": [("cegName", "string")],
        "Spring.GetGroundBlocked": [("x1", "number"), ("z1", "number"), ("x2", "number"), ("z2", "number")],
        "Spring.GetMapSquareTexture": [("texSquareX", "integer"), ("texSquareY", "integer"), ("lodMin", "integer"), ("luaTexName", "string"), ("lodMax", "integer?")],
        "Spring.GetMouseButtonsPressed": [("buttons", "integer[]")],
        "Spring.GetTeamStatsHistory": [("teamID", "integer"), ("startIndex", "integer"), ("endIndex", "integer?")],
        "Spring.GetTeamUnitsByDefs": [("teamID", "integer"), ("unitDefIDs", "integer[]")],
        "Spring.GetTerrainTypeData": [("terrainTypeIndex", "integer")],
        "Spring.GetGameRulesParam": [("paramName", "string")],
        "Spring.GetFeatureRulesParam": [("featureID", "integer"), ("paramName", "string")],
        "Spring.GetFeaturePieceCollisionVolumeData": [("featureID", "integer"), ("pieceNum", "integer")],
        "Spring.GetPlayerRulesParam": [("playerID", "integer"), ("paramName", "string")],
        "Spring.GetTeamRulesParam": [("teamID", "integer"), ("paramName", "string")],
        "Spring.GetUnitRulesParam": [("unitID", "integer"), ("paramName", "string")],
        "Spring.GetUnitsInBox": [("xmin", "number"), ("ymin", "number"), ("zmin", "number"), ("xmax", "number"), ("ymax", "number"), ("zmax", "number"), ("allegiance", "integer?")],
        "Spring.GetUnitsInCylinder": [("x", "number"), ("z", "number"), ("radius", "number"), ("allegiance", "integer?")],
        "Spring.GetUnitsInRectangle": [("xmin", "number"), ("zmin", "number"), ("xmax", "number"), ("zmax", "number"), ("allegiance", "integer?")],
        "Spring.GetUnitsInSphere": [("x", "number"), ("y", "number"), ("z", "number"), ("radius", "number"), ("allegiance", "integer?")],
        "Spring.GetUnitsInScreenRectangle": [("left", "number"), ("top", "number"), ("right", "number"), ("bottom", "number"), ("allegiance", "integer?")],
        "Spring.GetVisibleProjectiles": [("allyTeamID", "integer?"), ("addSyncedProjectiles", "boolean?"), ("addWeaponProjectiles", "boolean?"), ("addPieceProjectiles", "boolean?")],
        "Spring.GiveOrderArrayToUnitMap": [("unitMap", "integer[]"), ("commands", "CreateCommand[]")],
        "Spring.GiveOrderToUnitMap": [("unitMap", "integer[]"), ("cmdID", "CMD|integer"), ("params", "CreateCommandParams?"), ("options", "CreateCommandOptions?"), ("timeout", "integer?")],
        "Spring.InsertUnitCmdDesc": [("unitID", "integer"), ("index", "integer"), ("cmdDesc", "CommandDescription")],
        "Spring.SelectUnitMap": [("unitMap", "integer[]"), ("append", "boolean?")],
        "Spring.SetGroundDecalQuadPosAndHeight": [("decalID", "integer"), ("posTLX", "number?"), ("posTLY", "number?"), ("posTRX", "number?"), ("posTRY", "number?"), ("posBRX", "number?"), ("posBRY", "number?"), ("posBLX", "number?"), ("posBLY", "number?"), ("projCubeHeight", "number?")],
        "Spring.SetGroundDecalTextureParams": [("decalID", "integer"), ("texWrapDistance", "number?"), ("texTraveledDistance", "number?")],
        "Spring.GetUnitFeatureSeparation": [("unitID", "integer"), ("featureID", "integer"), ("surfaceDist", "boolean?")],
        "Spring.GetUnitArrayCentroid": [("units", "integer[]")],
        "Spring.GetUnitMapCentroid": [("units", "integer[]")],
        "Spring.GetUnitNearestAlly": [("unitID", "integer"), ("range", "number?")],
        "Spring.GetUnitNearestEnemy": [("unitID", "integer"), ("range", "number?"), ("useLOS", "boolean?"), ("sphereDistTest", "boolean?"), ("checkSightDist", "boolean?")],
        "Spring.GetUnitWeaponCanFire": [("unitID", "integer"), ("weaponNum", "integer")],
        "Spring.GetUnitWeaponDamages": [("unitID", "integer"), ("weaponNum", "integer")],
        "Spring.GetUnitWeaponState": [("unitID", "integer"), ("weaponNum", "integer"), ("key", "string?")],
        "Spring.GetUnitWeaponTestRange": [("unitID", "integer"), ("weaponNum", "integer"), ("x", "number"), ("y", "number"), ("z", "number")],
        "Spring.GetUnitWeaponHaveFreeLineOfFire": [("unitID", "integer"), ("weaponNum", "integer"), ("targetID", "integer"), ("x", "number"), ("y", "number"), ("z", "number"), ("isGroundTarget", "boolean")],
        "Spring.GetUnitWeaponTestTarget": [("unitID", "integer"), ("weaponNum", "integer"), ("targetID", "integer"), ("x", "number"), ("y", "number"), ("z", "number"), ("isGroundTarget", "boolean")],
        "Spring.GetUnitWeaponTryTarget": [("unitID", "integer"), ("weaponNum", "integer"), ("targetID", "integer"), ("x", "number"), ("y", "number"), ("z", "number"), ("userTarget", "boolean"), ("isGroundTarget", "boolean")],
        "Spring.GetUnitWeaponVectors": [("unitID", "integer"), ("weaponNum", "integer")],
        "Spring.GetUnitLeavesGhost": [("unitID", "integer")],
        "Spring.SetUnitLeavesGhost": [("unitID", "integer"), ("leavesGhost", "boolean"), ("leaveDeadGhost", "boolean?")],
        "Spring.SetUnitMetalExtraction": [("unitID", "integer"), ("depth", "number"), ("range", "number?")],
        "Spring.MarkerAddPoint": [("x", "number"), ("y", "number"), ("z", "number"), ("text", "string?"), ("localOnly", "boolean?"), ("playerID", "integer?")],
        "Spring.SDLSetTextInputRect": [("x", "integer"), ("y", "integer"), ("width", "integer"), ("height", "integer")],
        "Spring.Pos2BuildPos": [("unitDefID", "integer"), ("posX", "number"), ("posY", "number"), ("posZ", "number"), ("buildFacing", "integer?")],
        "Spring.SendSkirmishAIMessage": [("aiTeam", "integer"), ("message", "string")],
        "Spring.SetBuildSpacing": [("spacing", "integer")],
        "Spring.SetFeatureEngineDrawMask": [("featureID", "integer"), ("engineDrawMask", "integer")],
        "Spring.SetFeatureAlwaysUpdateMatrix": [("featureID", "integer"), ("alwaysUpdateMat", "boolean")],
        "Spring.SetFeatureSelectionVolumeData": [("featureID", "integer"), ("scaleX", "number"), ("scaleY", "number"), ("scaleZ", "number"), ("offsetX", "number"), ("offsetY", "number"), ("offsetZ", "number"), ("vType", "integer"), ("tType", "integer"), ("Axis", "boolean")],
        "Spring.SetFeatureCollisionVolumeData": [("featureID", "integer"), ("scaleX", "number"), ("scaleY", "number"), ("scaleZ", "number"), ("offsetX", "number"), ("offsetY", "number"), ("offsetZ", "number"), ("vType", "integer"), ("tType", "integer"), ("Axis", "integer")],
        "Spring.SetFeaturePieceCollisionVolumeData": [("featureID", "integer"), ("pieceIndex", "integer"), ("enable", "boolean"), ("scaleX", "number"), ("scaleY", "number"), ("scaleZ", "number"), ("offsetX", "number"), ("offsetY", "number"), ("offsetZ", "number"), ("volumeType", "integer"), ("primaryAxis", "integer")],
        "Spring.SetCustomCommandDrawData": [("cmdID", "integer"), ("cmdReference", "DefRef"), ("colorR", "number?"), ("colorG", "number?"), ("colorB", "number?"), ("colorA", "number?"), ("showArea", "boolean?")],
        "Spring.SetUnitCollisionVolumeData": [("unitID", "integer"), ("scaleX", "number"), ("scaleY", "number"), ("scaleZ", "number"), ("offsetX", "number"), ("offsetY", "number"), ("offsetZ", "number"), ("vType", "integer"), ("tType", "integer"), ("Axis", "integer")],
        "Spring.SetDollyCameraCurve": [("curveType", "integer"), ("controlPoints", "ControlPoint[]"), ("knots", "number[]")],
        "Spring.SetDollyCameraLookCurve": [("curveType", "integer"), ("controlPoints", "ControlPoint[]"), ("knots", "number[]")],
        "Spring.SetFeatureRulesParam": [("featureID", "integer"), ("paramName", "string"), ("paramValue", "RulesParamValue"), ("losAccess", "integer?")],
        "Spring.SetGameRulesParam": [("paramName", "string"), ("paramValue", "RulesParamValue"), ("losAccess", "integer?")],
        "Spring.SetMapLightTrackingState": [("lightHandle", "integer"), ("unitOrProjectileID", "integer"), ("enableTracking", "boolean"), ("unitOrProjectile", "boolean")],
        "Spring.SetMapShadingTexture": [("texType", "string"), ("texName", "string"), ("num", "integer?")],
        "Spring.SetMapSquareTerrainType": [("x", "integer"), ("z", "integer"), ("newType", "integer")],
        "Spring.SetMapSquareTexture": [("texSqrX", "integer"), ("texSqrY", "integer"), ("luaTexName", "string")],
        "Spring.SetModelLightTrackingState": [("lightHandle", "integer"), ("unitOrProjectileID", "integer"), ("enableTracking", "boolean"), ("unitOrProjectile", "boolean")],
        "Spring.SetPlayerRulesParam": [("playerID", "integer"), ("paramName", "string"), ("paramValue", "RulesParamValue"), ("losAccess", "integer?")],
        "Spring.SetDollyCameraRelativeMode": [("relativeMode", "integer")],
        "Spring.SetSoundEffectParams": [("params", "table")],
        "Spring.SetSquareBuildingMask": [("x", "integer"), ("z", "integer"), ("mask", "integer")],
        "Spring.SetTerrainTypeData": [("typeIndex", "integer"), ("speedTanks", "number?"), ("speedKBOts", "number?"), ("speedHovers", "number?"), ("speedShips", "number?"), ("hardness", "number?"), ("receiveTracks", "boolean?"), ("name", "string?")],
        "Spring.SetTeamRulesParam": [("teamID", "integer"), ("paramName", "string"), ("paramValue", "RulesParamValue"), ("losAccess", "integer?")],
        "Spring.SetUnitRulesParam": [("unitID", "integer"), ("paramName", "string"), ("paramValue", "RulesParamValue"), ("losAccess", "integer?")],
        "Spring.SetUnitBuildSpeed": [("builderID", "integer"), ("buildSpeed", "number"), ("repairSpeed", "number?"), ("reclaimSpeed", "number?"), ("resurrectSpeed", "number?"), ("captureSpeed", "number?"), ("terraformSpeed", "number?")],
        "Spring.SetUnitCloak": [("unitID", "integer"), ("cloak", "NumberOrBool"), ("cloakArg", "NumberOrBool")],
        "Spring.SetUnitCosts": [("unitID", "integer"), ("costs", "UnitCostOverrides")],
        "Spring.SetUnitFlanking": [("unitID", "integer"), ("type", "string"), ("arg1", "number"), ("y", "number?"), ("z", "number?")],
        "Spring.SetUnitHarvestStorage": [("unitID", "integer"), ("storedMetal", "number?"), ("maxStoredMetal", "number?"), ("storedEnergy", "number?"), ("maxStoredEnergy", "number?")],
        "Spring.SetUnitHealth": [("unitID", "integer"), ("health", "UnitHealthValue")],
        "Spring.SetUnitPosErrorParams": [("unitID", "integer"), ("posErrorVectorX", "number"), ("posErrorVectorY", "number"), ("posErrorVectorZ", "number"), ("posErrorDeltaX", "number"), ("posErrorDeltaY", "number"), ("posErrorDeltaZ", "number"), ("nextPosErrorUpdate", "integer?"), ("allyTeamID", "integer?"), ("setPosErrorBit", "boolean?")],
        "Spring.SetUnitResourcing": [("unitID", "integer"), ("resourceType", "string"), ("amount", "number")],
        "Spring.SetUnitSelectionVolumeData": [("unitID", "integer"), ("scaleX", "number"), ("scaleY", "number"), ("scaleZ", "number"), ("offsetX", "number"), ("offsetY", "number"), ("offsetZ", "number"), ("vType", "integer"), ("tType", "integer"), ("Axis", "integer")],
        "Spring.SetUnitStorage": [("unitID", "integer"), ("resource", "string"), ("amount", "number")],
        "Spring.SetUnitWeaponDamages": [("unitID", "integer"), ("weaponNum", "integer"), ("key", "string"), ("value", "number")],
        "Spring.SetUnitUseWeapons": [("unitID", "integer"), ("forceUseWeapons", "boolean?"), ("allowUseWeapons", "boolean?")],
        "Spring.SetAtmosphere": [("params", "AtmosphereParams")],
        "Spring.SetLosViewColors": [("always", "RgbColor"), ("los", "RgbColor"), ("radar", "RgbColor"), ("jam", "RgbColor"), ("radar2", "RgbColor")],
        "Spring.SetMapRenderingParams": [("params", "MapRenderingParams")],
        "Spring.SetSoundEffectParams": [("params", "SoundEffectParams")],
        "Spring.SetSunLighting": [("params", "SunLightingParams")],
        "Spring.SetWaterParams": [("waterParams", "WaterParams")],
        "Spring.SetUnitNanoPieces": [("unitID", "integer"), ("pieceNums", "integer[]")],
        "Spring.SetUnitSensorRadius": [("unitID", "integer"), ("type", "string"), ("radius", "integer")],
        "Spring.SetUnitStockpile": [("unitID", "integer"), ("stockpile", "integer?"), ("buildPercent", "number?")],
        "Spring.SetUnitPieceCollisionVolumeData": [("unitID", "integer"), ("pieceIndex", "integer"), ("enable", "boolean"), ("scaleX", "number"), ("scaleY", "number"), ("scaleZ", "number"), ("offsetX", "number"), ("offsetY", "number"), ("offsetZ", "number"), ("volumeType", "integer?"), ("primaryAxis", "integer?")],
        "Spring.SetUnitPieceParent": [("unitID", "integer"), ("AlteredPiece", "integer"), ("ParentPiece", "integer")],
        "Spring.SetUnitPosition": [("unitID", "integer"), ("x", "number"), ("y", "number"), ("z", "number")],
        "Spring.SetUnitWeaponState": [("unitID", "integer"), ("weaponNum", "integer"), ("key", "string"), ("value", "number")],
        "Spring.CallAsTeam": [("teamID", "integer"), ("func", "function"), ("args", "any[]")],
        "Spring.Echo": [("arg", "string"), ("rest", "string")],
        "Spring.MarkerErasePosition": [("x", "number"), ("y", "number"), ("z", "number"), ("unused", "number"), ("localOnly", "boolean?"), ("playerID", "integer?"), ("alwaysErase", "boolean?")],
        "Spring.SendCommands": [("command", "string"), ("rest", "string")],
        "Spring.SendMessageToSpectators": [("message", "string")],
        "Spring.SetActiveCommand": [("cmdIndex", "integer"), ("button", "integer"), ("leftClick", "boolean?"), ("rightClick", "boolean?"), ("alt", "boolean?"), ("ctrl", "boolean?"), ("meta", "boolean?"), ("shift", "boolean?")],
        "Spring.SetHeightMapFunc": [("luaFunction", "function"), ("args", "number[]")],
        "Spring.SetOriginalHeightMapFunc": [("heightMapFunc", "function"), ("args", "any[]")],
        "Spring.SetProjectileTarget": [("projectileID", "integer"), ("target", "ProjectileTargetRef")],
        "Spring.SetSmoothMeshFunc": [("luaFunction", "function"), ("args", "any[]")],
        "Spring.SetUnitTarget": [("unitID", "integer"), ("target", "UnitTargetRef"), ("dgun", "boolean?"), ("userTarget", "boolean?"), ("weaponNum", "number?")],
        "Spring.SpawnCEG": [("ceg", "DefRef"), ("posX", "number?"), ("posY", "number?"), ("posZ", "number?"), ("dirX", "number?"), ("dirY", "number?"), ("dirZ", "number?"), ("radius", "number?"), ("damage", "number?"), ("dmgMod", "number?")],
        "Spring.SpawnExplosion": [("posX", "number?"), ("posY", "number?"), ("posZ", "number?"), ("dirX", "number?"), ("dirY", "number?"), ("dirZ", "number?"), ("explosionParams", "NativeExplosionParams")],
        "Spring.SpawnProjectile": [("weaponDefID", "integer"), ("projectileParams", "NativeProjectileParams")],
        "Spring.SetProjectileTimeToLive": [("projectileID", "integer"), ("ttl", "integer")],
        "Spring.SetPieceProjectileParams": [("projectileID", "integer"), ("explosionFlags", "integer?"), ("spinAngle", "number?"), ("spinSpeed", "number?"), ("spinVectorX", "number?"), ("spinVectorY", "number?"), ("spinVectorZ", "number?")],
        "Spring.SetProjectileCollision": [("projectileID", "integer")],
        "Spring.SetProjectileDamages": [("projectileID", "integer"), ("unused", "integer"), ("key", "string"), ("value", "number")],
        "Spring.SetProjectileIsIntercepted": [("projectileID", "integer"), ("intercepted", "boolean")],
        "Spring.SetVideoCapturingTimeOffset": [("timeOffset", "number")],
        "Spring.SetWMCaption": [("title", "string"), ("titleShort", "string?")],
        "Spring.SetWindowGeometry": [("displayIndex", "integer"), ("winRelPosX", "integer"), ("winRelPosY", "integer"), ("winSizeX", "integer"), ("winSizeY", "integer"), ("fullScreen", "boolean"), ("borderless", "boolean")],
        "Spring.SetTeamColor": [("teamID", "integer"), ("r", "number"), ("g", "number"), ("b", "number"), ("a", "number?")],
        "Spring.SetWMIcon": [("iconFileName", "string"), ("forceResolution", "boolean?")],
        "Spring.TraceScreenRay": [("screenX", "number"), ("screenY", "number"), ("onlyCoords", "boolean?"), ("useMinimap", "boolean?"), ("includeSky", "boolean?"), ("ignoreWater", "boolean?"), ("heightOffset", "number?")],
        "Spring.TransferTeamMaxUnits": [("fromTeamID", "integer"), ("newTeamID", "integer"), ("transferAmnt", "integer")],
        "Spring.UnitAttach": [("transporterID", "integer"), ("passengerID", "integer"), ("pieceNum", "integer")],
        "Spring.UnitDetachFromAir": [("passengerID", "integer"), ("x", "number?"), ("y", "number?"), ("z", "number?")],
        "Spring.UpdateMapLight": [("lightHandle", "integer"), ("lightParams", "LightParams")],
        "Spring.UpdateModelLight": [("lightHandle", "integer"), ("lightParams", "LightParams")],
        "Spring.UseTeamResource": [("teamID", "integer"), ("resourceType", "string"), ("amount", "number")],
        "Spring.UseUnitResource": [("unitID", "integer"), ("resourceType", "string"), ("amount", "number")],
        "Spring.WarpMouse": [("x", "integer"), ("y", "integer")],
        "Spring.GetTeamUnitCount": [("teamID", "integer")],
        "Spring.GetUnitBuildParams": [("unitID", "integer"), ("paramName", "string")],
        "Spring.TraceRayGroundInDirection": [("posX", "number"), ("posY", "number"), ("posZ", "number"), ("dirX", "number"), ("dirY", "number"), ("dirZ", "number"), ("maxLength", "number?"), ("testWater", "boolean?")],
        "Spring.SolveNURBSCurve": [("degree", "integer"), ("controlPoints", "ControlPoint[]"), ("knots", "number[]"), ("segments", "integer")],
    }

    for func in all_functions:
        override = source_overrides.get(func.get("full_name", ""))
        if override is not None:
            func["params"] = [{"name": name, "type": ptype} for name, ptype in override]

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

    output_file.write_text(output_file.read_text(encoding='utf-8').rstrip() + '\n', encoding='utf-8')

    print(f"\nWrote {total} functions ({callout_count} callouts, {callin_count} callins) to {output_file}")
    print(f"Namespaces: {', '.join(sorted(by_namespace.keys()))}")

if __name__ == '__main__':
    main()
