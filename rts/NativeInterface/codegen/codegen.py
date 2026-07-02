"""
Generate NativeInterface C header and stub implementation from parsed Lua functions.
"""

from __future__ import annotations
from dataclasses import dataclass, field as dc_field
from typing import Optional
from lua_parser import LuaFunction, Param, ReturnField
import re

_DEFAULT_VALUE: dict[str, str] = {
    "int32_t":      "0",
    "uint32_t":     "0u",
    "float":        "0.0f",
    "bool":         "false",
    "const char*":  '""',
    "Float2":       "{0.0f, 0.0f}",
    "Float3":       "{0.0f, 0.0f, 0.0f}",
    "Float4":       "{0.0f, 0.0f, 0.0f, 0.0f}",
}


def _default(ctype: str) -> str:
    return _DEFAULT_VALUE.get(ctype, "0")


# ── Context setup ─────────────────────────────────────────────────────────────

@dataclass
class _ContextSetup:
    stmts: list[str]        # C++ statements; {id_field} = query param name
    includes: list[str]     # required header paths
    error_consts: list[tuple[str, str, str]] = dc_field(default_factory=list)
    # Each tuple: (const_name, error_code, message)


_UNIT_ERRORS = [
    ("INVALID_UNIT_ERROR", "ERROR_INVALID_ARGUMENT", "Invalid unit ID"),
]
_FEATURE_ERRORS = [
    ("INVALID_FEATURE_ERROR", "ERROR_INVALID_ARGUMENT", "Invalid feature ID"),
]
_MOVE_TYPE_ERRORS = [
    ("INVALID_UNIT_ERROR",      "ERROR_INVALID_ARGUMENT", "Invalid unit ID"),
    ("INVALID_MOVE_TYPE_ERROR", "ERROR_INVALID_STATE",    "Unit does not use script move type"),
]
_OBJECT_ERRORS = [
    ("INVALID_OBJECT_ERROR", "ERROR_INVALID_ARGUMENT", "Invalid object ID"),
]

_UNIT_INCLUDES    = ["Sim/Units/Unit.h", "Sim/Units/UnitHandler.h"]
_FEATURE_INCLUDES = ["Sim/Features/Feature.h", "Sim/Features/FeatureHandler.h"]

_CONTEXT_SETUP: dict[str, _ContextSetup] = {
    "ParseUnit": _ContextSetup(
        stmts=[
            "CUnit* unit = unitHandler.GetUnit(query->{id_field});",
            "if (unit == nullptr) {{ result->error = &INVALID_UNIT_ERROR; return; }}",
        ],
        includes=_UNIT_INCLUDES,
        error_consts=_UNIT_ERRORS,
    ),
    "ParseRawUnit": _ContextSetup(
        stmts=[
            "CUnit* unit = unitHandler.GetUnit(query->{id_field});",
            "if (unit == nullptr) {{ result->error = &INVALID_UNIT_ERROR; return; }}",
        ],
        includes=_UNIT_INCLUDES,
        error_consts=_UNIT_ERRORS,
    ),
    "ParseAllyUnit": _ContextSetup(
        stmts=[
            "CUnit* unit = unitHandler.GetUnit(query->{id_field});",
            "if (unit == nullptr) {{ result->error = &INVALID_UNIT_ERROR; return; }}",
        ],
        includes=_UNIT_INCLUDES,
        error_consts=_UNIT_ERRORS,
    ),
    "ParseInLosUnit": _ContextSetup(
        stmts=[
            "CUnit* unit = unitHandler.GetUnit(query->{id_field});",
            "if (unit == nullptr) {{ result->error = &INVALID_UNIT_ERROR; return; }}",
        ],
        includes=_UNIT_INCLUDES,
        error_consts=_UNIT_ERRORS,
    ),
    "ParseFeature": _ContextSetup(
        stmts=[
            "CFeature* feature = featureHandler.GetFeature(query->{id_field});",
            "if (feature == nullptr) {{ result->error = &INVALID_FEATURE_ERROR; return; }}",
        ],
        includes=_FEATURE_INCLUDES,
        error_consts=_FEATURE_ERRORS,
    ),
    "ParseScriptMoveType": _ContextSetup(
        stmts=[
            "CUnit* unit = unitHandler.GetUnit(query->{id_field});",
            "if (unit == nullptr) {{ result->error = &INVALID_UNIT_ERROR; return; }}",
            "if (!unit->UsingScriptMoveType()) {{ result->error = &INVALID_MOVE_TYPE_ERROR; return; }}",
            "CScriptMoveType* moveType = static_cast<CScriptMoveType*>(unit->moveType);",
        ],
        includes=_UNIT_INCLUDES + ["Sim/MoveTypes/ScriptMoveType.h"],
        error_consts=_MOVE_TYPE_ERRORS,
    ),
    "ParseDerivedMoveType": _ContextSetup(
        stmts=[
            "CUnit* unit = unitHandler.GetUnit(query->{id_field});",
            "if (unit == nullptr) {{ result->error = &INVALID_UNIT_ERROR; return; }}",
            "// TODO: cast unit->moveType to the appropriate derived type",
        ],
        includes=_UNIT_INCLUDES,
        error_consts=_UNIT_ERRORS,
    ),
    "ParseSolidObject": _ContextSetup(
        stmts=[
            "// TODO: ParseSolidObject — look up object by query->{id_field}",
        ],
        includes=[],
        error_consts=_OBJECT_ERRORS,
    ),
}


# ── Expression substitution ───────────────────────────────────────────────────

# Matches luaL_check*/luaL_opt*/lua_to* calls with L as first arg and int literal second
# Note: lua[Ll]?_ matches lua_ (core API) and luaL_ (helper API)
_CHECK_CALL_RE = re.compile(
    r'lua[Ll]?_(?:check|opt|to)\w*\s*'
    r'\(\s*L\s*,\s*(\d+)'
    r'(?:\s*(?:,\s*[^()]*(?:\([^()]*\)[^()]*)*))?\)'
)

_FLOAT_VEC_COMPONENTS: dict[str, tuple[str, ...]] = {
    "Float2": ("x", "y"),
    "Float3": ("x", "y", "z"),
    "Float4": ("x", "y", "z", "w"),
}
_FLOAT_VEC_CPP: dict[str, str] = {
    "Float2": "float2",
    "Float3": "float3",
    "Float4": "float4",
}


def _substitute_expr(expr: str, params: list[Param]) -> str:
    """
    Substitute Lua check calls and local variable names with query->field references.
    - lua_var_name            → query->name  (word-boundary, done first)
    - Float3 var_name         → float3(query->name.x, query->name.y, query->name.z)
    - luaL_check*(L, N, ...) → query->name  for param at position N (done second)
    Context params (parse_fn set) are left as-is — their context vars match by name.
    Word-boundary sub runs BEFORE check-call sub to avoid double-prefixing
    when a check call result is later matched by a var-name pattern.
    """
    pos_to_param: dict[int, Param] = {p.position: p for p in params if p.parse_fn is None}

    # Step 1: Word-boundary substitute known Lua local var names → query->name
    # Float2/Float3/Float4 expand to their C++ counterparts.
    # Done before check-call sub so we don't accidentally re-sub `query->x` as `query->query->x`.
    result = expr
    for p in params:
        if p.parse_fn is not None:
            continue
        lua_name = p.lua_var or (p.name if p.name_inferred else None)
        if not lua_name:
            continue

        if p.c_type in _FLOAT_VEC_COMPONENTS:
            comps = _FLOAT_VEC_COMPONENTS[p.c_type]
            cpp_type = _FLOAT_VEC_CPP[p.c_type]
            replacement = cpp_type + "(" + ", ".join(f"query->{p.name}.{c}" for c in comps) + ")"
        else:
            replacement = f"query->{p.name}"

        # Negative lookbehind prevents substituting member names after -> or .
        result = re.sub(r'(?<!->)(?<!\.)(?<!:)\b' + re.escape(lua_name) + r'\b', replacement, result)

    # Step 2: Replace inline check/opt/to calls by stack position
    def _replace_check(m: re.Match) -> str:
        pos = int(m.group(1))
        p = pos_to_param.get(pos)
        return f"query->{p.name}" if p else m.group(0)

    result = _CHECK_CALL_RE.sub(_replace_check, result)

    return result


def _needs_c_str(push_callee: str, expr: str) -> bool:
    """True if the push was lua_pushsstring (takes std::string, needs .c_str())."""
    return push_callee == "lua_pushsstring" and not expr.endswith(".c_str()")


def _has_unresolved_lua(expr: str) -> bool:
    """True if the expression still contains Lua API calls after substitution."""
    return bool(re.search(r'\blua[Ll]?_(?:check|opt|to)\w*\s*\(', expr))


_LUA_STATE_RE = re.compile(r'(?<!\w)L(?!\w)')


def _uses_lua_state(expr: str) -> bool:
    """True if the expression references L (the Lua state), which is not available in native fns."""
    return bool(_LUA_STATE_RE.search(expr))


_LITERAL_KEYWORDS = frozenset({'true', 'false', 'nullptr', 'NULL'})
# Context variables introduced by parse helpers — safe to use directly in generated code
_CONTEXT_VAR_NAMES = frozenset({
    'unit', 'feature', 'moveType', 'obj', 'lmd', 'solidObject',
    'gs', 'camera', 'unitHandler', 'featureHandler', 'teamHandler',
    'groundDecals', 'projectileHandler', 'readMap', 'pathManager',
    'sqr', 'math',
})
# All lowercase names that are safe in generated C++ (not local Lua variables)
_ALL_SAFE_NAMES = _CONTEXT_VAR_NAMES | _LITERAL_KEYWORDS | frozenset({
    # C/C++ type keywords and casts
    'int', 'float', 'bool', 'char', 'void', 'auto', 'const', 'static',
    'unsigned', 'signed', 'long', 'short', 'double', 'inline',
    'static_cast', 'dynamic_cast', 'reinterpret_cast', 'const_cast',
    'sizeof', 'new', 'delete', 'this', 'return',
    # Common C integer types
    'size_t', 'uint8_t', 'uint16_t', 'uint32_t', 'uint64_t',
    'int8_t', 'int16_t', 'int32_t', 'int64_t',
    # Namespace prefixes (appear as bare names before ::)
    'std', 'spring', 'math',
    # Spring math types (appear as constructors in expressions)
    'float3', 'float2', 'float4',
    # NI function parameters
    'query', 'result',
    # Common Spring engine globals (singletons / global pointers)
    'globalRendering', 'globalRenderingInfo', 'luaUI',
    'archiveScanner', 'vfsHandler', 'fileSystem',
    'eventHandler', 'wordCompletion', 'queueIDCount',
    'downloadQueue', 'soundChannels', 'musicChannels',
    'resourceHandler', 'sky', 'water', 'waterRendering',
    'mapRendering', 'shadowHandler', 'font', 'smallFont',
})

# Matches bare lowercase identifiers not preceded by ->, ::, . or a word char
# (Excludes member accesses, qualified names, and mid-word matches)
_BARE_LOWER_RE = re.compile(r'(?<![>:.\w])([a-z_][a-zA-Z0-9_]*)')


def _is_unresolved_local(expr: str) -> bool:
    """
    True if expr contains a bare lowercase identifier that is likely an unresolved
    Lua local variable (not substituted by _substitute_expr).

    Scans the full expression for bare lowercase identifiers not preceded by
    ->, ::, or . (member/qualified accesses). Known-safe names (context vars,
    C++ keywords, namespace prefixes) are excluded.

    Examples caught:   r, diff, data, hexHash, numAssignedValues, decoded
    Examples not caught: unit->health, gs->simFrames, std::clamp, query->x
    """
    for m in _BARE_LOWER_RE.finditer(expr):
        name = m.group(1)
        if name in _ALL_SAFE_NAMES:
            continue
        # Extra guard: if the two chars before this match are '->' we skip
        # (the lookbehind catches '>' but not the full '->')
        start = m.start()
        if start >= 2 and expr[start - 2:start] == '->':
            continue
        return True
    return False


# ── Include detection ─────────────────────────────────────────────────────────

def _detect_expr_includes(exprs: list[str]) -> set[str]:
    """Scan expressions for patterns that imply additional required headers."""
    includes: set[str] = set()
    combined = " ".join(exprs)
    if re.search(r'\bgs\s*->', combined):
        includes.add("Sim/Misc/GlobalSynced.h")
    if re.search(r'\bteamHandler\b', combined):
        includes.add("Sim/Misc/TeamHandler.h")
    if re.search(r'\bfloat3\s*\(', combined):
        includes.add("System/float3.h")
    if re.search(r'\bmath::', combined) or re.search(r'\bSpring::', combined):
        includes.add("System/SpringMath.h")
    if re.search(r'->unitDef\b', combined) or re.search(r'\bCUnitDef\b', combined):
        includes.add("Sim/Units/UnitDef.h")
    if re.search(r'\bunitDefHandler\b', combined):
        includes.add("Sim/Units/UnitDefHandler.h")
    if re.search(r'\bfeatureDefHandler\b', combined):
        includes.add("Sim/Features/FeatureDefHandler.h")
    if re.search(r'->featureDef\b', combined):
        includes.add("Sim/Features/FeatureDef.h")
    if re.search(r'\bgroundDecals\b', combined):
        includes.add("Rendering/Env/Decals/DecalHandler.h")
    if re.search(r'\breadMap\b', combined):
        includes.add("Map/ReadMap.h")
    if re.search(r'\bpathManager\b', combined):
        includes.add("Sim/Path/IPathManager.h")
    if re.search(r'\bprojectileHandler\b', combined):
        includes.add("Sim/Projectiles/ProjectileHandler.h")
    if re.search(r'\bcamera\b', combined) or re.search(r'\bCCamera\b', combined):
        includes.add("Rendering/Camera.h")
    if re.search(r'\bglobalRendering\b', combined):
        includes.add("Rendering/GlobalRendering.h")
    if re.search(r'\bstd::clamp\b', combined):
        includes.add("<algorithm>")
    if re.search(r'\bstd::min\b|\bstd::max\b', combined):
        includes.add("<algorithm>")
    return includes


# ── Header generation ─────────────────────────────────────────────────────────

def generate_header(module_name: str, functions: list[LuaFunction], source_file: str) -> str:
    api_struct = f"{module_name}Api"
    lines: list[str] = []

    lines += [
        "// AUTO-GENERATED by rts/NativeInterface/codegen/generate.py",
        f"// Source: rts/Lua/{source_file}",
        "// Do not edit — re-run the generator instead.",
        "#pragma once",
        "",
        "#include <stdint.h>",
        '#include "CommonTypes.h"',
        "",
        "#ifdef __cplusplus",
        'extern "C" {',
        "#endif",
        "",
        f"// {'=' * 74}",
        f"// {module_name} — auto-generated NativeInterface API",
        f"// {'=' * 74}",
        "",
    ]

    api_entries: list[str] = []

    for fn in functions:
        query_name  = f"{fn.name}Query"
        result_name = f"{fn.name}Result"

        # Query struct
        if fn.params:
            lines.append(f"struct {query_name} {{")
            for p in fn.params:
                fixme = "  // FIXME: name not derivable from source" if not p.name_inferred else ""
                lines.append(f"    {p.c_type} {p.name};{fixme}")
            lines.append("};")
        else:
            lines.append(f"struct {query_name} {{ uint8_t _unused; }};")

        # Result struct
        lines.append(f"struct {result_name} {{")
        lines.append(f"    const Error* error;")
        for r in fn.returns:
            fixme = "  // FIXME: name not derivable from source" if not r.name_inferred else ""
            lines.append(f"    {r.c_type} {r.name};{fixme}")
        lines.append("};")
        lines.append("")

        api_entries.append(
            f"    void (*{fn.name})(const {query_name}* query, {result_name}* result);"
        )

    # API struct
    lines += [
        f"struct {api_struct} {{",
        *api_entries,
        "};",
        "",
        f"extern const {api_struct} {module_name.upper()}_API;",
        "",
        "#ifdef __cplusplus",
        "}",
        "#endif",
    ]

    return "\n".join(lines) + "\n"


# ── Implementation generation ─────────────────────────────────────────────────

def _collect_module_includes(functions: list[LuaFunction]) -> list[str]:
    """Determine all #include directives needed for this module's implementation."""
    includes: set[str] = set()

    all_exprs: list[str] = []

    for fn in functions:
        # Include from context setups
        for p in fn.params:
            if p.parse_fn and p.parse_fn in _CONTEXT_SETUP:
                for inc in _CONTEXT_SETUP[p.parse_fn].includes:
                    includes.add(inc)

        # Collect all expressions for pattern detection
        for r in fn.returns:
            if r.expr:
                all_exprs.append(r.expr)
        for lhs, _ in fn.setter_stmts:
            all_exprs.append(lhs)
        for vs in fn.void_stmts:
            all_exprs.append(vs)

    includes.update(_detect_expr_includes(all_exprs))

    # Float3/Float2/Float4 params generate float3(...) calls in impl
    for fn in functions:
        for p in fn.params:
            if p.c_type in ("Float3", "Float2", "Float4"):
                includes.add("System/float3.h")
                break

    return sorted(includes)


def _collect_error_consts(functions: list[LuaFunction]) -> list[tuple[str, str, str]]:
    """Collect unique error constants needed by all context lookups in the module."""
    seen: set[str] = set()
    result: list[tuple[str, str, str]] = []
    for fn in functions:
        for p in fn.params:
            if p.parse_fn and p.parse_fn in _CONTEXT_SETUP:
                for ec in _CONTEXT_SETUP[p.parse_fn].error_consts:
                    if ec[0] not in seen:
                        seen.add(ec[0])
                        result.append(ec)
    return result


def _generate_function_body(fn: LuaFunction) -> list[str]:
    """Generate the body lines for a single NativeInterface function implementation."""
    lines: list[str] = []
    lines.append("    result->error = nullptr;")

    # Build position → param map for setter resolution
    param_by_pos: dict[int, Param] = {p.position: p for p in fn.params}

    # Emit context setup for each parse-helper param (in position order)
    context_params = [p for p in fn.params if p.parse_fn is not None]
    emitted_parse_fns: set[str] = set()
    for p in sorted(context_params, key=lambda x: x.position):
        if p.parse_fn in emitted_parse_fns:
            continue
        emitted_parse_fns.add(p.parse_fn)
        setup = _CONTEXT_SETUP.get(p.parse_fn)
        if setup:
            for stmt in setup.stmts:
                lines.append("    " + stmt.format(id_field=p.name))

    if fn.returns:
        # Reader function — emit substituted return expressions
        all_todo = True
        for r in fn.returns:
            expr = _substitute_expr(r.expr, fn.params)
            if _has_unresolved_lua(expr) or _uses_lua_state(expr) or _is_unresolved_local(expr):
                lines.append(f"    result->{r.name} = {_default(r.c_type)};  // TODO: {r.expr}")
            else:
                if _needs_c_str(r.push_callee, expr):
                    expr = expr + ".c_str()"
                lines.append(f"    result->{r.name} = {expr};")
                all_todo = False
        if all_todo and not context_params:
            pass  # keep as-is — all returns are TODO

    elif fn.setter_stmts or fn.void_stmts:
        # Setter / void-call function
        for lhs_expr, pos in fn.setter_stmts:
            p = param_by_pos.get(pos)
            if p is not None:
                if p.c_type in _FLOAT_VEC_COMPONENTS:
                    comps = _FLOAT_VEC_COMPONENTS[p.c_type]
                    cpp_type = _FLOAT_VEC_CPP[p.c_type]
                    rhs = cpp_type + "(" + ", ".join(f"query->{p.name}.{c}" for c in comps) + ")"
                else:
                    rhs = f"query->{p.name}"
                lines.append(f"    {lhs_expr} = {rhs};")
            else:
                lines.append(f"    // TODO: {lhs_expr} = <pos {pos}>;")

        for void_expr in fn.void_stmts:
            substituted = _substitute_expr(void_expr, fn.params)
            if _has_unresolved_lua(substituted) or _uses_lua_state(substituted) or _is_unresolved_local(substituted):
                lines.append(f"    // TODO: {void_expr}")
            else:
                lines.append(f"    {substituted};")

    else:
        # Could not determine body
        if context_params:
            # Context was set up but no setters/voids detected
            lines.append("    // TODO: implement body (setters/calls not detected)")
        elif fn.is_noop:
            pass  # Genuinely empty body — result->error = nullptr is correct
        else:
            lines.append("    // TODO: implement")

    return lines


def generate_impl(module_name: str, functions: list[LuaFunction], source_file: str) -> str:
    api_struct = f"{module_name}Api"
    lines: list[str] = []

    # ── File header ──────────────────────────────────────────────────────────
    lines += [
        "// AUTO-GENERATED by rts/NativeInterface/codegen/generate.py",
        f"// Source: rts/Lua/{source_file}",
        "// Do not edit — re-run the generator instead.",
        f'#include "{module_name}.h"',
        "",
    ]

    # ── Includes ─────────────────────────────────────────────────────────────
    module_includes = _collect_module_includes(functions)
    if module_includes:
        for inc in module_includes:
            if inc.startswith("<"):
                lines.append(f"#include {inc}")
            else:
                lines.append(f'#include "{inc}"')
        lines.append("")

    # ── Namespace open ────────────────────────────────────────────────────────
    lines += ["namespace {", ""]

    # ── Error constants ───────────────────────────────────────────────────────
    error_consts = _collect_error_consts(functions)
    if error_consts:
        for name, code, msg in error_consts:
            lines.append(
                f'static const Error {name} = {{ .code = {code}, .message = "{msg}" }};'
            )
        lines.append("")

    # ── Function implementations ──────────────────────────────────────────────
    impl_names: list[tuple[str, str]] = []

    for fn in functions:
        query_name  = f"{fn.name}Query"
        result_name = f"{fn.name}Result"
        impl_name   = f"Native{fn.name}"
        impl_names.append((fn.name, impl_name))

        lines.append(f"// {fn.source_file}:{fn.source_line} — {fn.class_name}::{fn.name}")
        lines.append(
            f"static void {impl_name}(const {query_name}* query, {result_name}* result) {{"
        )

        for body_line in _generate_function_body(fn):
            lines.append(body_line)

        lines.append("}")
        lines.append("")

    # ── Namespace close + API struct ──────────────────────────────────────────
    lines += [
        "} // namespace",
        "",
        f"const {api_struct} {module_name.upper()}_API = {{",
    ]
    for fn_name, impl in impl_names:
        lines.append(f"    .{fn_name} = {impl},")
    lines += [
        "};",
    ]

    return "\n".join(lines) + "\n"
