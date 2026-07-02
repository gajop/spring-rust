"""
Parse Lua*.cpp files using tree-sitter to extract Lua binding function signatures.

Analyses the actual function bodies to determine:
- Parameter types from luaL_check*/luaL_opt*/lua_to* and ParseUnit/ParseFeature calls
- Return types from lua_push* calls, count from 'return N' statements
- Doc comments (@param/@return) as fallback when AST analysis finds nothing
- Registered API functions via REGISTER_LUA_CFUNC/HSTR_PUSH_CFUNC/PUSH_FUNCTION patterns
- Single-call delegate following: int Fn(L) { return Helper(L, ...); }
"""

from __future__ import annotations
from dataclasses import dataclass, field
from pathlib import Path
from typing import Optional
import re

import tree_sitter_cpp as tscpp
from tree_sitter import Language, Parser, Node

_LANG = Language(tscpp.language())
_PARSER = Parser(_LANG)


# ── Type maps ────────────────────────────────────────────────────────────────

# Callee name -> C type for stack-read calls (arg at position 2 is the stack index)
_CHECK_CALL_TO_CTYPE: dict[str, str] = {
    "luaL_checkint":               "int32_t",
    "luaL_optint":                 "int32_t",
    "lua_toint":                   "int32_t",
    "luaL_checkinteger":           "int32_t",
    "luaL_optinteger":             "int32_t",
    "lua_tointeger":               "int32_t",
    "luaL_checkinteger_noassert":  "int32_t",
    "luaL_optinteger_noassert":    "int32_t",
    "luaL_checknumber":            "float",
    "luaL_optnumber":              "float",
    "luaL_checkfloat":             "float",
    "luaL_optfloat":               "float",
    "lua_tonumber":                "float",
    "luaL_checknumber_noassert":   "float",
    "luaL_optnumber_noassert":     "float",
    "luaL_checkboolean":           "bool",
    "luaL_optboolean":             "bool",
    "lua_toboolean":               "bool",
    "luaL_checkboolean_noassert":  "bool",
    "luaL_optboolean_noassert":    "bool",
    "luaL_checkstring":            "const char*",
    "luaL_optstring":              "const char*",
    "lua_tostring":                "const char*",
    "luaL_checksstring":           "const char*",
    "luaL_optsstring":             "const char*",
    "luaL_checklstring":           "const char*",
    "luaL_optlstring":             "const char*",
    "luaL_checkstring_noassert":   "const char*",
    "luaL_optstring_noassert":     "const char*",
}

# Callee name -> C type for push calls (arg at position 2 is the value)
_PUSH_CALL_TO_CTYPE: dict[str, str] = {
    "lua_pushnumber":        "float",
    "lua_pushinteger":       "int32_t",
    "lua_pushboolean":       "bool",
    "lua_pushstring":        "const char*",
    "lua_pushsstring":       "const char*",
    "lua_pushliteral":       "const char*",
    "lua_pushlstring":       "const char*",
    "lua_pushlightuserdata": "uint64_t",
}

# Parse helpers that extract an ID from the stack — callee -> (field_name, c_type)
# Position is the 3rd argument (index 2 in 0-based args)
_PARSE_HELPERS: dict[str, tuple[str, str]] = {
    "ParseUnit":             ("unitID",    "int32_t"),
    "ParseRawUnit":          ("unitID",    "int32_t"),
    "ParseAllyUnit":         ("unitID",    "int32_t"),
    "ParseInLosUnit":        ("unitID",    "int32_t"),
    "ParseFeature":          ("featureID", "int32_t"),
    "ParseScriptMoveType":   ("unitID",    "int32_t"),
    "ParseDerivedMoveType":  ("unitID",    "int32_t"),
    "ParseSolidObject":      ("objectID",  "int32_t"),
}

# Lua doc-comment type names → C types
_DOC_TYPE_TO_CTYPE: dict[str, str] = {
    "integer":  "int32_t",
    "int":      "int32_t",
    "number":   "float",
    "float":    "float",
    "boolean":  "bool",
    "bool":     "bool",
    "string":   "const char*",
    "GL":       "int32_t",   # GL enum constants
}


# ── Data classes ──────────────────────────────────────────────────────────────

@dataclass
class Param:
    position: int       # 1-based Lua stack position
    c_type: str
    name: str           # suggested C field name
    name_inferred: bool = True  # False when name is a positional fallback (arg{N}, flag{N})
    parse_fn: Optional[str] = None   # e.g. "ParseUnit", "ParseScriptMoveType"
    lua_var: Optional[str] = None    # original Lua local variable name (for expr substitution)


@dataclass
class ReturnField:
    c_type: str
    name: str           # suggested C field name
    expr: str           # source text of the push argument (for comments)
    name_inferred: bool = True  # False when name is a positional fallback (ret{N})
    push_callee: str = ""  # the lua_push* callee (e.g. "lua_pushsstring") for type coercions


@dataclass
class LuaFunction:
    name: str           # e.g. GetUnitHealth
    class_name: str     # e.g. LuaSyncedRead
    source_file: str
    source_line: int    # 1-based line of the function definition
    params: list[Param]
    returns: list[ReturnField]
    skip_reason: Optional[str] = None  # set if AST analysis found an ambiguity
    is_registered: bool = False  # True when found in REGISTER_LUA_CFUNC / HSTR_PUSH_CFUNC etc.
    is_noop: bool = False        # True when body has no meaningful statements (just return 0;)
    setter_stmts: list = field(default_factory=list)  # [(lhs_expr, stack_pos), ...]
    void_stmts: list = field(default_factory=list)    # [void_call_expr_str, ...]

    @property
    def needs_review(self) -> bool:
        """True if any param or return name is a positional fallback, not derived from source."""
        return any(not p.name_inferred for p in self.params) or \
               any(not r.name_inferred for r in self.returns)

    @property
    def supported(self) -> bool:
        return self.skip_reason is None


# ── Helpers ───────────────────────────────────────────────────────────────────

def _named_args(arg_list_node: Node) -> list[Node]:
    """Return the actual argument nodes, stripping '(', ',' and ')'."""
    return [c for c in arg_list_node.children if c.type not in ("(", ")", ",")]


def _int_literal(node: Node) -> Optional[int]:
    """Return the integer value if node is a number_literal that looks like a positive int."""
    if node.type == "number_literal":
        txt = node.text.decode()
        try:
            return int(txt)
        except ValueError:
            return None
    return None


def _string_literal_value(node: Node) -> Optional[str]:
    """Return the unquoted string value from a string_literal node."""
    if node.type == "string_literal":
        text = node.text.decode()
        # Strip surrounding quotes
        if (text.startswith('"') and text.endswith('"')) or \
           (text.startswith("'") and text.endswith("'")):
            return text[1:-1]
    return None


def _handle_name_from_type(type_str: str) -> str:
    """Turn a Lua userdata type name (e.g. 'Font', 'FBO') into a C field name."""
    if not type_str:
        return "handle"
    # Convert to camelCase field: 'Font' -> 'font', 'FBO' -> 'fbo'
    return type_str[0].lower() + type_str[1:]


def _last_identifier(node: Node) -> Optional[str]:
    """
    Extract the rightmost plain identifier from a node.
    Works for:
      identifier          -> 'health'
      field_expression    -> last field name, e.g. 'id' from 'unit->unitDef->id'
    Returns None for complex exprs (binary, call, cast, etc.).
    """
    if node.type == "identifier":
        return node.text.decode()
    if node.type == "field_expression":
        last = node.children[-1]
        if last.type == "field_identifier":
            return last.text.decode()
        if last.type == "identifier":
            return last.text.decode()
    return None


_C_KEYWORDS = frozenset({
    "auto", "break", "case", "char", "const", "continue", "default", "do",
    "double", "else", "enum", "extern", "float", "for", "goto", "if",
    "inline", "int", "long", "register", "return", "short", "signed",
    "sizeof", "static", "struct", "switch", "typedef", "union", "unsigned",
    "void", "volatile", "while", "type", "error",
    # C++ boolean/null literals — invalid as field names
    "true", "false", "nullptr", "NULL",
})

def _safe_name(name: str) -> str:
    if name in _C_KEYWORDS:
        return name + "_"
    # Ensure starts with letter/underscore
    if name and not (name[0].isalpha() or name[0] == '_'):
        name = "v_" + name
    return name


def _unique_name(base: str, seen: set[str]) -> str:
    name = _safe_name(base)
    if name not in seen:
        seen.add(name)
        return name
    i = 2
    while f"{name}_{i}" in seen:
        i += 1
    result = f"{name}_{i}"
    seen.add(result)
    return result


# ── AST walking ───────────────────────────────────────────────────────────────

def _collect_calls(body: Node) -> list[Node]:
    """Recursively collect all call_expression nodes inside body."""
    result: list[Node] = []
    def walk(n: Node) -> None:
        if n.type == "call_expression":
            result.append(n)
        for c in n.children:
            walk(c)
    walk(body)
    return result


def _collect_returns(body: Node) -> list[Node]:
    """Collect all return_statement nodes inside body."""
    result: list[Node] = []
    def walk(n: Node) -> None:
        if n.type == "return_statement":
            result.append(n)
        for c in n.children:
            walk(c)
    walk(body)
    return result


def _callee_name(call: Node) -> Optional[str]:
    """Get the function name from a call_expression node."""
    callee = call.children[0] if call.children else None
    if callee is None:
        return None
    if callee.type == "identifier":
        return callee.text.decode()
    # Could be a member call like foo->bar(...) — ignore
    return None


_VECTOR_CTYPES: dict[str, tuple[str, int]] = {
    "float3": ("Float3", 3),
    "float2": ("Float2", 2),
    "float4": ("Float4", 4),
}


def _collect_vector_groups(body: Node) -> dict[int, tuple[str, str]]:
    """
    Detect float3/float2 patterns that pack multiple consecutive check calls into
    a single vector.  Returns {first_stack_pos: (field_name, NativeInterface_ctype)}.
    """
    groups: dict[int, tuple[str, str]] = {}

    def _positions_from_list(node: Node) -> list[int]:
        result = []
        for c in node.children:
            if c.type == "call_expression":
                callee = _callee_name(c)
                if callee in _CHECK_CALL_TO_CTYPE and len(c.children) >= 2:
                    al = c.children[1]
                    if al.type == "argument_list":
                        args = _named_args(al)
                        if len(args) >= 2:
                            pos = _int_literal(args[1])
                            if pos is not None and pos > 0:
                                result.append(pos)
        return result

    def _consecutive(positions: list[int], expected_count: int) -> bool:
        if len(positions) != expected_count:
            return False
        return all(positions[i] == positions[0] + i for i in range(expected_count))

    def walk(n: Node) -> None:
        # Pattern 1: const float3 varName(check(L,N), check(L,N+1), check(L,N+2))
        if n.type == "declaration":
            type_name: Optional[str] = None
            for c in n.children:
                if c.type == "type_identifier":
                    type_name = c.text.decode()
                    break
            if type_name and type_name in _VECTOR_CTYPES:
                ni_type, count = _VECTOR_CTYPES[type_name]
                for c in n.children:
                    if c.type == "init_declarator":
                        var_name: Optional[str] = None
                        arg_list: Optional[Node] = None
                        for cc in c.children:
                            if cc.type == "identifier":
                                var_name = cc.text.decode()
                            elif cc.type == "argument_list":
                                arg_list = cc
                        if var_name and arg_list:
                            positions = _positions_from_list(arg_list)
                            if _consecutive(positions, count):
                                if positions[0] not in groups:
                                    groups[positions[0]] = (var_name, ni_type)

        # Pattern 2: obj->field = {check(L,N), ..., check(L,N+k)}
        elif n.type == "assignment_expression":
            lhs: Optional[Node] = None
            rhs: Optional[Node] = None
            for c in n.children:
                if c.type == "field_expression":
                    lhs = c
                elif c.type == "initializer_list":
                    rhs = c
            if lhs and rhs:
                field_name = _last_identifier(lhs)
                if field_name:
                    positions = _positions_from_list(rhs)
                    count = len(positions)
                    type_map = {2: "Float2", 3: "Float3", 4: "Float4"}
                    if count in type_map and _consecutive(positions, count):
                        if positions[0] not in groups:
                            groups[positions[0]] = (field_name, type_map[count])

        for c in n.children:
            walk(c)

    walk(body)
    return groups


def _collect_var_name_hints(body: Node) -> dict[int, str]:
    """
    Walk the body for name hints for each stack position:
      1. Variable declarations:   const lua_Number x = luaL_check*(L, N)  -> N -> "x"
      2. Member assignments:      obj->field = luaL_check*(L, N)           -> N -> "field"
    """
    hints: dict[int, str] = {}

    def _try_check_call(call_node: Node) -> Optional[tuple[int, str]]:
        callee = _callee_name(call_node)
        if callee not in _CHECK_CALL_TO_CTYPE:
            return None
        arg_list = call_node.children[1] if len(call_node.children) >= 2 else None
        if arg_list is None or arg_list.type != "argument_list":
            return None
        args = _named_args(arg_list)
        if len(args) < 2:
            return None
        pos = _int_literal(args[1])
        if pos is None or pos <= 0:
            return None
        return pos, callee

    def _unwrap_call(node: Node) -> Optional[Node]:
        if node.type == "call_expression":
            return node
        if node.type == "cast_expression":
            for c in node.children:
                result = _unwrap_call(c)
                if result:
                    return result
        return None

    def walk(n: Node) -> None:
        if n.type == "init_declarator":
            var_name: Optional[str] = None
            call_node: Optional[Node] = None
            for c in n.children:
                if c.type == "identifier" and var_name is None:
                    var_name = c.text.decode()
                elif c.type in ("pointer_declarator", "reference_declarator") and var_name is None:
                    # const char* msg = ... → identifier nested inside pointer_declarator
                    for cc in c.children:
                        if cc.type == "identifier":
                            var_name = cc.text.decode()
                            break
                elif c.type == "argument_list" and call_node is None:
                    # Constructor-style init: string text(luaL_check*(L, N), ...)
                    for ac in c.children:
                        if ac.type == "call_expression":
                            call_node = ac
                            break
                else:
                    candidate = _unwrap_call(c)
                    if candidate and call_node is None:
                        call_node = candidate
            if var_name and call_node:
                result = _try_check_call(call_node)
                if result and result[0] not in hints:
                    hints[result[0]] = var_name

        elif n.type == "assignment_expression":
            children = [c for c in n.children if c.type not in ("=",)]
            if len(children) >= 2:
                lhs, rhs = children[0], children[-1]
                if lhs.type == "field_expression" and rhs.type == "call_expression":
                    field = _last_identifier(lhs)
                    if field:
                        result = _try_check_call(rhs)
                        if result and result[0] not in hints:
                            hints[result[0]] = field

        for c in n.children:
            walk(c)

    walk(body)
    return hints


def _analyze_body(
    body: Node, src: bytes
) -> tuple[dict, list, int, Optional[str], dict, dict]:
    """
    Analyse a function body and return:
      params:     dict[stack_pos -> (c_type, name_hint, name_inferred)]
      returns:    list of (c_type, expr_text, push_callee) in push order
      ret_count:  max non-zero return count found
      skip_reason: string if the signature is genuinely unrepresentable
      parse_fns:  dict[stack_pos -> parse_helper_name] for context params
      var_hints:  dict[stack_pos -> lua_local_var_name] from declarations

    Stack positions are 1-based.
    name_inferred=True  → name derived from source (reliable)
    name_inferred=False → positional fallback (FIXME marker generated)
    """
    calls = _collect_calls(body)
    returns_stmts = _collect_returns(body)

    # Detect variadic patterns — these cannot be expressed as fixed C structs.
    has_lua_gettop = any(_callee_name(c) == "lua_gettop" for c in calls)
    has_lua_isnone = any(_callee_name(c) == "lua_isnone" for c in calls)
    has_nonfixed_reads = False
    for call in calls:
        callee = _callee_name(call)
        if callee in _CHECK_CALL_TO_CTYPE or callee in _PARSE_HELPERS:
            arg_list = call.children[1] if len(call.children) >= 2 else None
            if arg_list and arg_list.type == "argument_list":
                idx = 2 if callee in _PARSE_HELPERS else 1
                args = _named_args(arg_list)
                if len(args) > idx:
                    if _int_literal(args[idx]) is None:
                        has_nonfixed_reads = True

    var_hints = _collect_var_name_hints(body)
    vector_groups = _collect_vector_groups(body)

    # pos -> (ctype, name, name_inferred)
    params: dict[int, tuple[str, str, bool]] = {}
    # pos -> parse helper callee name (for context generation)
    parse_fns: dict[int, str] = {}
    push_list: list[tuple[str, str, str]] = []  # (ctype, expr_text, push_callee)

    for call in calls:
        callee = _callee_name(call)
        if callee is None:
            continue
        if len(call.children) < 2:
            continue
        arg_list = call.children[1]
        if arg_list.type != "argument_list":
            continue
        args = _named_args(arg_list)

        # ── Stack reads → params ────────────────────────────────────────────
        if callee in _CHECK_CALL_TO_CTYPE:
            ctype = _CHECK_CALL_TO_CTYPE[callee]
            if len(args) >= 2:
                pos = _int_literal(args[1])
                if pos is not None and pos > 0:
                    src_name = var_hints.get(pos)
                    if src_name:
                        name_hint, inferred = src_name, True
                    else:
                        name_hint, inferred = _param_name_from_callee(callee, pos), False
                    if pos not in params:
                        params[pos] = (ctype, name_hint, inferred)

        # ── Parse helpers → params (source-derived names) ───────────────────
        elif callee in _PARSE_HELPERS:
            field_name, ctype = _PARSE_HELPERS[callee]
            if len(args) >= 3:
                pos = _int_literal(args[2])
                if pos is not None and pos > 0:
                    if pos not in params:
                        params[pos] = (ctype, field_name, True)
                        parse_fns[pos] = callee

        # ── luaL_checkudata → uint32_t handle ──────────────────────────────
        elif callee == "luaL_checkudata":
            if len(args) >= 2:
                pos = _int_literal(args[1])
                if pos is not None and pos > 0 and pos not in params:
                    type_name = _string_literal_value(args[2]) if len(args) >= 3 else None
                    name = _handle_name_from_type(type_name) if type_name else f"arg{pos}"
                    inferred = type_name is not None
                    params[pos] = ("uint32_t", name, inferred)

        # ── Other unrecognized luaL_check*/luaL_opt* → uint32_t FIXME ──────
        elif (callee.startswith("luaL_check") or callee.startswith("luaL_opt")) \
                and callee != "luaL_checktype":  # checktype validates only, no value
            if len(args) >= 2:
                pos = _int_literal(args[1])
                if pos is not None and pos > 0 and pos not in params:
                    params[pos] = ("uint32_t", f"arg{pos}", False)

        # ── Pushes → returns ────────────────────────────────────────────────
        elif callee in _PUSH_CALL_TO_CTYPE:
            ctype = _PUSH_CALL_TO_CTYPE[callee]
            if len(args) >= 2:
                expr_node = args[1]
                expr_text = src[expr_node.start_byte:expr_node.end_byte].decode(errors="replace")
                push_list.append((ctype, expr_text, callee))
        # lua_pushnil → skip

    # ── Merge float2/float3/float4 groups (source-derived names) ─────────────
    _VEC_WIDTHS = {"Float2": 2, "Float3": 3, "Float4": 4}
    for first_pos, (var_name, ni_type) in vector_groups.items():
        width = _VEC_WIDTHS.get(ni_type, 0)
        component_positions = list(range(first_pos, first_pos + width))
        if all(params.get(p, (None,))[0] == "float" for p in component_positions):
            for p in component_positions:
                del params[p]
            params[first_pos] = (ni_type, var_name, True)

    # ── Return count ─────────────────────────────────────────────────────────
    ret_count = 0
    for stmt in returns_stmts:
        for child in stmt.children:
            n = _int_literal(child)
            if n is not None and n > ret_count:
                ret_count = n

    # ── Variadic check — only skip if no fixed params were detected ───────────
    # If we found at least one fixed-position param, generate with that partial sig.
    # Only truly skip when there's nothing at all to generate.
    if not params and has_lua_isnone:
        return {}, [], 0, "variadic: uses lua_isnone", {}, {}
    if not params and has_lua_gettop and has_nonfixed_reads:
        return {}, [], 0, "variadic: uses lua_gettop with non-literal indices", {}, {}

    return params, push_list, ret_count, None, parse_fns, var_hints


def _param_name_from_callee(callee: str, pos: int) -> str:
    """Derive a sensible default param field name from the check call name and position."""
    if "int" in callee or "integer" in callee:
        return f"arg{pos}"
    if "float" in callee or "number" in callee:
        return f"arg{pos}"
    if "boolean" in callee:
        return f"flag{pos}"
    if "string" in callee:
        return f"str{pos}"
    return f"arg{pos}"


# ── Doc comment parsing ───────────────────────────────────────────────────────

_AT_PARAM_RE  = re.compile(r'@param\s+(\S+)\s+(\S+)')
_AT_RETURN_RE = re.compile(r'@return\s+(\S+)(?:\s+(\w+))?')
_AT_FUNC_RE   = re.compile(r'@function\s')


def _doc_map_lua_type(raw_type: str) -> Optional[str]:
    """
    Map a Lua doc type string to a C type.
    Returns None for types that can't be expressed as a scalar (table, fun, ...).
    """
    # Strip trailing ? (optional marker) and leading nil|
    t = raw_type.strip()
    t = re.sub(r'^nil\s*\|\s*', '', t)
    t = t.rstrip('?')
    # Arrays/tables
    if t.endswith('[]') or t == 'table' or t.startswith('table<'):
        return None
    # Callbacks
    if t.startswith('fun') or t == 'function':
        return None
    # Variadic
    if t == '...':
        return None
    # Known scalar types
    if t in _DOC_TYPE_TO_CTYPE:
        return _DOC_TYPE_TO_CTYPE[t]
    # GL enum names
    if t.startswith('GL') or t == 'integer':
        return "int32_t"
    # Handle types (Font, FBO, RBO, Path, etc.) — map to uint32_t with FIXME
    if t and t[0].isupper():
        return "uint32_t"
    return None


def _parse_doc_comment(src: bytes, func_start_byte: int) -> Optional[dict]:
    """
    Parse the JSDoc-style comment immediately preceding the function definition.
    Returns a dict with:
      'has_function': bool  — @function marker found
      'params': list of (name, c_type, name_inferred)  — None if variadic/callback
      'returns': list of (c_type, name)  — None if table/complex return
      'is_variadic': bool
    Or None if no doc comment found.
    """
    text_before = src[:func_start_byte].decode(errors='replace')
    # Find the LAST /** in the text so we get the closest doc block to the function
    last_start = text_before.rfind('/**')
    if last_start == -1:
        return None
    after = text_before[last_start:]
    m = re.match(r'/\*\*(.*?)\*/\s*$', after, re.DOTALL)
    if not m:
        return None

    content = m.group(1)
    has_function = bool(_AT_FUNC_RE.search(content))

    # Check for variadic @param ...
    is_variadic = bool(re.search(r'@param\s+\.\.\.', content))
    if is_variadic:
        return {'has_function': has_function, 'params': None, 'returns': None, 'is_variadic': True}

    params = []
    for pm in _AT_PARAM_RE.finditer(content):
        name_raw, type_raw = pm.group(1).rstrip('?'), pm.group(2)
        if name_raw == '...':
            return {'has_function': has_function, 'params': None, 'returns': None, 'is_variadic': True}
        ctype = _doc_map_lua_type(type_raw)
        if ctype is None:
            # table/callback/unknown param — mark as FIXME
            params.append((name_raw, "uint32_t", False))
        else:
            inferred = ctype != "uint32_t"  # uint32_t means handle FIXME
            params.append((name_raw, ctype, inferred))

    returns = []
    for rm in _AT_RETURN_RE.finditer(content):
        type_raw = rm.group(1)
        name_raw = rm.group(2)
        if type_raw in ('nil', 'void'):
            continue
        ctype = _doc_map_lua_type(type_raw)
        if ctype is None:
            # table/complex — can't represent
            returns = None
            break
        name = name_raw if name_raw else "ret"
        inferred = ctype != "uint32_t"
        if returns is not None:
            returns.append((ctype, name, inferred))

    return {
        'has_function': has_function,
        'params': params,
        'returns': returns,
        'is_variadic': False,
    }


# ── Registration scanning ─────────────────────────────────────────────────────

_REGISTER_RE = re.compile(
    r'(?:'
    r'REGISTER_LUA_CFUNC\s*\(\s*(\w+)\s*\)'                      # REGISTER_LUA_CFUNC(Name)
    r'|HSTR_PUSH_CFUNC\s*\([^,]+,\s*"[^"]*"\s*,\s*(\w+)'         # HSTR_PUSH_CFUNC(L, "name", Fn)
    r'|PUSH_FUNCTION\s*\(\s*(\w+)\s*\)'                           # PUSH_FUNCTION(Name)
    r'|lua_pushcfunction\s*\([^,]+,\s*(\w+)\s*\)'                 # lua_pushcfunction(L, Name)
    r'|LuaPushNamedCFunc\s*\([^,]+,\s*"[^"]*"\s*,\s*(\w+)'       # LuaPushNamedCFunc(L, "name", Fn)
    r')'
)


def _scan_registered_names(src_text: str, header_paths: list[Path] = []) -> set[str]:
    """Return function names registered as Lua API functions in this file."""
    registered: set[str] = set()
    for m in _REGISTER_RE.finditer(src_text):
        name = m.group(1) or m.group(2) or m.group(3) or m.group(4) or m.group(5)
        if name:
            registered.add(name)
    # Also scan associated headers
    for hp in header_paths:
        if hp.exists():
            try:
                text = hp.read_text(encoding="utf-8", errors="replace")
                for m in _REGISTER_RE.finditer(text):
                    name = m.group(1) or m.group(2) or m.group(3) or m.group(4) or m.group(5)
                    if name:
                        registered.add(name)
            except OSError:
                pass
    return registered


# ── Delegate following ────────────────────────────────────────────────────────

_NOOP_CALL_PREFIXES = ("RECOIL_", "TRACY_", "ZoneScoped", "ZoneNamedN")


def _is_noop_stmt(node: Node) -> bool:
    """True for expression statements that are just profiling/tracing macros."""
    if node.type == "expression_statement":
        for c in node.children:
            if c.type == "call_expression":
                name = _callee_name(c)
                if name and any(name.startswith(p) for p in _NOOP_CALL_PREFIXES):
                    return True
            elif c.type == "identifier":
                # Handles macros without parentheses, e.g. RECOIL_DETAILED_TRACY_ZONE;
                name = c.text.decode()
                if any(name.startswith(p) for p in _NOOP_CALL_PREFIXES):
                    return True
    return False


def _body_has_statements(body: Node) -> bool:
    """
    Return True if the body has any non-trivial statements.
    A body is considered empty (no-op) when it contains only:
      - return 0; (or return;)
      - profiling/tracing macro calls (RECOIL_, TRACY_, etc.)
      - comments
    This distinguishes deprecated/stub functions from ones where parsing failed.
    """
    for n in body.children:
        if n.type in ('{', '}', 'comment'):
            continue
        if _is_noop_stmt(n):
            continue
        if n.type == 'return_statement':
            ret_children = [c for c in n.children if c.type not in ('return', ';')]
            if not ret_children:
                continue  # bare return;
            if len(ret_children) == 1 and ret_children[0].type == 'number_literal' \
                    and ret_children[0].text == b'0':
                continue  # return 0;
        # Any other statement means the body has real content
        return True
    return False


def _collect_all_function_bodies(root: Node) -> dict[str, Node]:
    """
    Map function name -> body (compound_statement) for ALL functions in the file,
    including static helpers like `static int Helper(lua_State* L, bool synced)`.
    Only top-level function definitions (not lambdas) are included.
    """
    bodies: dict[str, Node] = {}

    def walk(node: Node) -> None:
        if node.type == "function_definition":
            body = None
            decl = None
            for c in node.children:
                if c.type == "function_declarator":
                    decl = c
                if c.type == "compound_statement":
                    body = c
            if decl and body:
                # Extract the simple function name (last identifier in declarator)
                fn_name = None
                for c in decl.children:
                    if c.type == "qualified_identifier":
                        for cc in reversed(c.children):
                            if cc.type == "identifier":
                                fn_name = cc.text.decode()
                                break
                        break
                    elif c.type == "identifier":
                        fn_name = c.text.decode()
                if fn_name and fn_name not in bodies:
                    bodies[fn_name] = body
            # Don't recurse into function bodies (avoid nested lambdas)
            return
        for c in node.children:
            walk(c)

    walk(root)
    return bodies


def _get_delegate_body(body: Node, all_bodies: dict[str, Node]) -> Optional[Node]:
    """
    If body is `{ [noops...] return Fn(L, ...); }`, return Fn's body node.
    The delegate must pass L as its first argument.
    """
    stmts = [c for c in body.children
             if c.type not in ('{', '}') and not _is_noop_stmt(c)]
    if len(stmts) != 1 or stmts[0].type != "return_statement":
        return None

    ret_children = [c for c in stmts[0].children if c.type not in ('return', ';')]
    if len(ret_children) != 1:
        return None

    expr = ret_children[0]
    # Unwrap parentheses
    while expr.type == "parenthesized_expression":
        inner = [c for c in expr.children if c.type not in ('(', ')')]
        if len(inner) == 1:
            expr = inner[0]
        else:
            break

    if expr.type != "call_expression":
        return None

    callee = _callee_name(expr)
    if not callee:
        return None

    arg_list_node = expr.children[-1] if expr.children else None
    if arg_list_node is None or arg_list_node.type != "argument_list":
        return None
    args = _named_args(arg_list_node)
    if not args or args[0].text.decode() not in ('L',):
        return None

    return all_bodies.get(callee)


# ── Body action collection ───────────────────────────────────────────────────

def _collect_body_actions(
    body: Node, src: bytes, raw_params: dict[int, tuple]
) -> tuple[list[tuple[str, int]], list[str]]:
    """
    Walk direct-child expression_statements of body and collect:
      setter_stmts: (lhs_field_expr_str, stack_pos) for field = luaL_check*(L, pos) patterns
      void_stmts:   expr_str for object method calls (obj->method(...))
    Only top-level statements (direct children of compound_statement) are examined
    to avoid capturing conditional logic inside if/else branches.
    """
    setter_stmts: list[tuple[str, int]] = []
    void_stmts: list[str] = []

    for n in body.children:
        if n.type != "expression_statement":
            continue

        for child in n.children:
            if child.type == "assignment_expression":
                parts = [c for c in child.children if c.type != "="]
                if len(parts) < 2:
                    continue
                lhs, rhs = parts[0], parts[-1]
                if lhs.type != "field_expression":
                    continue
                lhs_text = src[lhs.start_byte:lhs.end_byte].decode(errors="replace")

                if rhs.type == "call_expression":
                    # Simple: field = luaL_check*(L, pos)
                    callee = _callee_name(rhs)
                    if callee not in _CHECK_CALL_TO_CTYPE:
                        continue
                    al = rhs.children[1] if len(rhs.children) >= 2 else None
                    if al is None or al.type != "argument_list":
                        continue
                    args = _named_args(al)
                    if len(args) < 2:
                        continue
                    pos = _int_literal(args[1])
                    if pos is not None and pos > 0 and pos in raw_params:
                        setter_stmts.append((lhs_text, pos))

                elif rhs.type == "initializer_list":
                    # Aggregate: field = {check(L,N), check(L,N+1), ...}
                    positions = []
                    for ac in rhs.children:
                        if ac.type == "call_expression":
                            cal = _callee_name(ac)
                            if cal in _CHECK_CALL_TO_CTYPE and len(ac.children) >= 2:
                                al_n = ac.children[1]
                                if al_n.type == "argument_list":
                                    ac_args = _named_args(al_n)
                                    if len(ac_args) >= 2:
                                        p_n = _int_literal(ac_args[1])
                                        if p_n is not None and p_n > 0:
                                            positions.append(p_n)
                    if positions:
                        first_pos = min(positions)
                        if first_pos in raw_params:
                            setter_stmts.append((lhs_text, first_pos))

                else:
                    # Other assignment to a field expr — capture as void_stmt for
                    # substitution (e.g. gs->noHelperAIs = !lua_toboolean(L, 1))
                    full_text = src[child.start_byte:child.end_byte].decode(errors="replace")
                    void_stmts.append(full_text)

            elif child.type == "call_expression":
                callee_node = child.children[0] if child.children else None
                if callee_node is None or callee_node.type != "field_expression":
                    continue
                expr_text = src[child.start_byte:child.end_byte].decode(errors="replace")
                void_stmts.append(expr_text)

    return setter_stmts, void_stmts


# ── Function detection ────────────────────────────────────────────────────────

def _is_lua_binding(node: Node) -> bool:
    """
    Return True if node is 'int ClassName::FunctionName(lua_State* L)'.
    """
    if node.type != "function_definition":
        return False

    children = node.children
    if not children or children[0].type != "primitive_type":
        return False
    if children[0].text != b"int":
        return False

    declarator = None
    for c in children:
        if c.type == "function_declarator":
            declarator = c
            break
    if declarator is None:
        return False

    has_qualified = any(c.type == "qualified_identifier" for c in declarator.children)
    if not has_qualified:
        return False

    params = None
    for c in declarator.children:
        if c.type == "parameter_list":
            params = c
            break
    if params is None:
        return False

    # Must have exactly one parameter and it must be lua_State*.
    # Functions with extra C++ args (e.g. ParseFoo(L, int idx)) are internal helpers.
    param_decls = [c for c in params.children if c.type == "parameter_declaration"]
    if len(param_decls) != 1:
        return False
    for cc in param_decls[0].children:
        if cc.type == "type_identifier" and cc.text == b"lua_State":
            return True
    return False


def _extract_qualified_name(declarator: Node) -> tuple[str, str]:
    """Return (class_name, method_name) from a function_declarator."""
    for c in declarator.children:
        if c.type == "qualified_identifier":
            children = c.children
            class_name = ""
            method_name = ""
            for cc in children:
                if cc.type in ("namespace_identifier", "type_identifier"):
                    class_name = cc.text.decode()
                elif cc.type == "identifier":
                    method_name = cc.text.decode()
            return class_name, method_name
    return "", ""


# ── Main entry point ──────────────────────────────────────────────────────────

def parse_file(path: Path) -> list[LuaFunction]:
    """Parse a Lua*.cpp file and return all detected Lua binding functions."""
    src = path.read_bytes()
    src_text = src.decode(errors='replace')
    tree = _PARSER.parse(src)
    root = tree.root_node

    # Collect all function bodies for delegate following
    all_bodies = _collect_all_function_bodies(root)

    # Scan registration macros — check .cpp and associated .h
    header_candidate = path.with_suffix('.h')
    registered = _scan_registered_names(src_text, [header_candidate])

    functions: list[LuaFunction] = []

    def walk(node: Node) -> None:
        if _is_lua_binding(node):
            _process_function(node, src, path, functions, all_bodies, registered)
            return
        for c in node.children:
            walk(c)

    walk(root)
    return functions


def _process_function(
    node: Node,
    src: bytes,
    path: Path,
    out: list[LuaFunction],
    all_bodies: dict[str, Node],
    registered: set[str],
) -> None:
    declarator = None
    body = None
    for c in node.children:
        if c.type == "function_declarator":
            declarator = c
        if c.type == "compound_statement":
            body = c

    if declarator is None or body is None:
        return

    class_name, func_name = _extract_qualified_name(declarator)
    if not func_name:
        return

    line = node.start_point[0] + 1  # 1-based
    is_registered = func_name in registered

    # Detect no-op bodies BEFORE delegate following (delegates may have content)
    is_noop = not _body_has_statements(body)

    # Try delegate following: if body is `return Helper(L, ...)`, use Helper's body
    delegate_body = _get_delegate_body(body, all_bodies)
    analysis_body = delegate_body if delegate_body is not None else body

    raw_params, push_list, ret_count, skip_reason, parse_fns, raw_var_hints = _analyze_body(analysis_body, src)

    # Doc comment: full fallback when AST found nothing, or name-fill when names unresolved
    doc = _parse_doc_comment(src, node.start_byte)
    if doc is not None:
        if doc.get('is_variadic') and not raw_params and not push_list and skip_reason is None:
            skip_reason = "variadic: doc annotation indicates variadic"
        elif doc.get('params') is not None:
            if not raw_params and not push_list and skip_reason is None:
                # Full fallback: AST found nothing — use doc entirely
                for i, (dname, dctype, dinferred) in enumerate(doc['params'], start=1):
                    raw_params[i] = (dctype, dname, dinferred)
            else:
                # Partial: use doc @param names for positions where AST couldn't infer a name
                for i, (dname, dctype, dinferred) in enumerate(doc['params'], start=1):
                    if i in raw_params and not raw_params[i][2]:
                        raw_params[i] = (raw_params[i][0], dname, True)
        if doc.get('returns') is not None and not push_list:
            for i, (dctype, dname, dinferred) in enumerate(doc['returns']):
                push_list.append((dctype, dname, ""))
                if ret_count == 0:
                    ret_count = len(doc['returns'])
        # Mark as registered if doc has @function marker
        if doc.get('has_function'):
            is_registered = True

    # Build ordered params list
    params: list[Param] = []
    seen_names: set[str] = set()
    for pos in sorted(raw_params):
        ctype, name_hint, inferred = raw_params[pos]
        name = _unique_name(name_hint, seen_names)
        params.append(Param(
            position=pos,
            c_type=ctype,
            name=name,
            name_inferred=inferred,
            parse_fn=parse_fns.get(pos),
            lua_var=raw_var_hints.get(pos),
        ))

    # Collect setter and void call statements from body
    setter_stmts, void_stmts = _collect_body_actions(analysis_body, src, raw_params)

    # Build return fields from push list
    doc_returns = doc.get('returns') if doc else None
    returns: list[ReturnField] = []
    if ret_count > 0 and push_list:
        effective = push_list[-ret_count:] if len(push_list) >= ret_count else push_list
        seen_ret: set[str] = set()
        for i, entry in enumerate(effective):
            ctype, expr_text = entry[0], entry[1]
            push_callee = entry[2] if len(entry) > 2 else ""
            ident = _last_identifier_from_text(expr_text)
            if ident is None and doc_returns and i < len(doc_returns):
                ident = doc_returns[i][1]  # doc (ctype, name, inferred)
                inferred = bool(ident)
            else:
                inferred = ident is not None
            name = _unique_name(ident if ident else f"ret{i}", seen_ret)
            returns.append(ReturnField(
                c_type=ctype,
                name=name,
                expr=expr_text,
                name_inferred=inferred,
                push_callee=push_callee,
            ))
    elif ret_count == 0 and not push_list:
        pass  # genuinely returns nothing

    out.append(LuaFunction(
        name=func_name,
        class_name=class_name,
        source_file=path.name,
        source_line=line,
        params=params,
        returns=returns,
        skip_reason=skip_reason,
        is_registered=is_registered,
        is_noop=is_noop,
        setter_stmts=setter_stmts,
        void_stmts=void_stmts,
    ))


_EXPR_NON_NAMES = frozenset({"true", "false", "nullptr", "NULL", "0", "1"})


_GET_PREFIX_RE = re.compile(r'^[Gg]et([A-Z]\w*)$')
_IS_PREFIX_RE  = re.compile(r'^[Ii]s([A-Z]\w*)$')
_LUA_INTERNAL  = re.compile(r'^lua_|^luaL_')


def _to_camel(name: str) -> str:
    """Lower-camelCase: handle leading acronyms (LODCount→lodCount, FileExists→fileExists)."""
    if not name:
        return name
    # Find how many leading uppercase chars to lowercase.
    # If followed by more uppercase then lowercase (acronym + word), lowercase all but last.
    i = 0
    while i < len(name) and name[i].isupper():
        i += 1
    if i == 0:
        return name  # already starts lowercase
    if i == 1:
        return name[0].lower() + name[1:]
    # i >= 2: acronym — lowercase all uppercase except the last if it starts a CamelWord
    if i < len(name):
        # name[i-1] starts the next CamelWord, keep it upper: lowercase name[:i-1]
        return name[:i-1].lower() + name[i-1:]
    # All chars are uppercase — lowercase all
    return name.lower()


def _outermost_method_name(expr: str) -> Optional[str]:
    """Extract the method/function name of the outermost call in expr, or None."""
    expr = expr.strip()
    if not expr.endswith(')'):
        return None
    # Find the matching '(' by counting from the right
    depth = 0
    for i in range(len(expr) - 1, -1, -1):
        if expr[i] == ')':
            depth += 1
        elif expr[i] == '(':
            depth -= 1
            if depth == 0:
                # expr[i] is the open paren of the outermost call
                before = expr[:i].rstrip()
                m = re.search(r'(\w+)\s*$', before)
                return m.group(1) if m else None
    return None


def _last_identifier_from_text(expr: str) -> Optional[str]:
    """
    Extract the rightmost C identifier from an expression string.
    Returns None for boolean/null literals or complex expressions.
    """
    if re.match(r'^\w+$', expr):
        return None if expr in _EXPR_NON_NAMES else expr
    if re.match(r'^\w+(?:->\w+)+$', expr):
        last = expr.rsplit('->', 1)[-1]
        return None if last in _EXPR_NON_NAMES else last
    # Method call: extract the outermost method name and strip Get/Is prefix
    method = _outermost_method_name(expr)
    if method and not _LUA_INTERNAL.match(method) and method not in _EXPR_NON_NAMES:
        gm = _GET_PREFIX_RE.match(method)
        if gm:
            return _to_camel(gm.group(1))
        im = _IS_PREFIX_RE.match(method)
        if im:
            return _to_camel(im.group(1))
        return _to_camel(method)
    return None
