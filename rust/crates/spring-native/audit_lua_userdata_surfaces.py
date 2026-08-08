#!/usr/bin/env python3
"""Audit Lua userdata surfaces that are not represented by free callouts.

The generated ``lua_functions.md`` inventory intentionally contains callable
free functions.  VAO/VBO/Font/FBO/RBO userdata methods and properties are
registered through metatables or ``sol::new_usertype`` instead, so they need a
separate source-backed inventory.  This report keeps object identity and
lifecycle differences explicit and fails the audit for an unclassified Lua
surface.
"""

from __future__ import annotations

import argparse
import json
import re
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
LUA_SOURCE = ROOT / "rts" / "Lua"
GFX_HEADER = ROOT / "rts" / "NativeInterface" / "api" / "Gfx.h"
RUST_DOC = Path(__file__).with_name("rust_functions.md")
SURFACE_SPEC = ROOT / "test" / "native_api_parity" / "surface_tests.json"


@dataclass(frozen=True)
class Surface:
    name: str
    source: Path
    registration: str
    doc_class: str
    doc_sources: tuple[Path, ...] = ()


SURFACES = (
    Surface("VAO", LUA_SOURCE / "LuaVAO.cpp", "sol", "VAO", (LUA_SOURCE / "LuaVAOImpl.cpp",)),
    Surface("VBO", LUA_SOURCE / "LuaVBO.cpp", "sol", "VBO", (LUA_SOURCE / "LuaVBOImpl.cpp",)),
    Surface("LuaFont", LUA_SOURCE / "LuaFonts.cpp", "metatable", "LuaFont"),
    Surface("RBO", LUA_SOURCE / "LuaRBOs.cpp", "metatable", "RBO"),
    Surface("FBO", LUA_SOURCE / "LuaFBOs.cpp", "metatable", "FBO"),
)


# Native flat APIs deliberately use integer resource handles instead of Lua
# userdata.  These are the explicit object-method counterparts that already
# exist in Gfx.h.  Anything absent from this table is a real parity gap until
# it is assigned a reason below.
COUNTERPARTS: dict[tuple[str, str], tuple[str, ...]] = {
    ("VAO", "Delete"): ("Gfx.delete_vao",),
    ("VBO", "Delete"): ("Gfx.delete_vbo",),
    ("LuaFont", "Print"): ("Gfx.font_print",),
    ("LuaFont", "PrintWorld"): ("Gfx.font_print_world",),
    ("LuaFont", "Begin"): ("Gfx.font_begin",),
    ("LuaFont", "End"): ("Gfx.font_end",),
    ("LuaFont", "SubmitBuffered"): ("Gfx.font_submit_buffered",),
    ("LuaFont", "WrapText"): ("Gfx.font_wrap_text",),
    ("LuaFont", "GetTextWidth"): ("Gfx.font_get_text_width",),
    ("LuaFont", "GetTextHeight"): ("Gfx.font_get_text_height",),
    ("LuaFont", "SetTextColor"): ("Gfx.font_set_text_color",),
    ("LuaFont", "SetOutlineColor"): ("Gfx.font_set_outline_color",),
    ("LuaFont", "SetAutoOutlineColor"): ("Gfx.font_set_auto_outline_color",),
    ("LuaFont", "BindTexture"): ("Gfx.font_bind_texture",),
    ("RBO", "target"): ("Gfx.get_rbo_info.target",),
    ("RBO", "format"): ("Gfx.get_rbo_info.format",),
    ("RBO", "xsize"): ("Gfx.get_rbo_info.xsize",),
    ("RBO", "ysize"): ("Gfx.get_rbo_info.ysize",),
    ("RBO", "samples"): ("Gfx.get_rbo_info.samples",),
    ("RBO", "valid"): ("Gfx.get_rbo_info.valid",),
    ("LuaFont", "size"): ("Gfx.get_font_info.size",),
    ("LuaFont", "path"): ("Gfx.get_font_info.path",),
    ("LuaFont", "height"): ("Gfx.get_font_info.line_height",),
    ("LuaFont", "lineheight"): ("Gfx.get_font_info.line_height",),
    ("LuaFont", "descender"): ("Gfx.get_font_info.descender",),
    ("LuaFont", "outlinewidth"): ("Gfx.get_font_info.outline_width",),
    ("LuaFont", "outlineweight"): ("Gfx.get_font_info.outline_weight",),
    ("LuaFont", "family"): ("Gfx.get_font_info.family",),
    ("LuaFont", "style"): ("Gfx.get_font_info.style",),
    ("LuaFont", "texturewidth"): ("Gfx.get_font_info.texture_width",),
    ("LuaFont", "textureheight"): ("Gfx.get_font_info.texture_height",),
}


DESIGN_REASONS: dict[tuple[str, str], str] = {
    ("VAO", "__gc"): "Native modules own explicit integer handles and call DeleteVAO; Rust has no Lua garbage collector boundary.",
    ("VBO", "__gc"): "Native modules own explicit integer handles and call DeleteVBO; Rust has no Lua garbage collector boundary.",
    ("LuaFont", "__gc"): "Native modules own explicit integer font handles and call DeleteFont; Rust has no Lua garbage collector boundary.",
    ("RBO", "__gc"): "Native modules own explicit integer handles and call DeleteRBO; Rust has no Lua garbage collector boundary.",
    ("FBO", "__gc"): "Native modules own explicit integer handles and call DeleteFBO; Rust has no Lua garbage collector boundary.",
    ("FBO", "dynamic attachment keys"): "Lua FBO userdata stores arbitrary attachment fields through a Lua table; native uses typed CreateFBO attachment arrays and must expose equivalent typed operations.",
}


def strip_comments(text: str) -> str:
    text = re.sub(r"/\*.*?\*/", "", text, flags=re.DOTALL)
    return re.sub(r"//[^\n]*", "", text)


def registration_block(surface: Surface, text: str) -> str:
    if surface.registration == "sol":
        match = re.search(
            rf'gl\.new_usertype<[^>]+>\("{re.escape(surface.doc_class)}",(?P<body>.*?)\n\s*\);',
            text,
            flags=re.DOTALL,
        )
        return match.group("body") if match else ""
    match = re.search(
        rf'luaL_newmetatable\(L, "{re.escape(surface.doc_class if surface.name != "LuaFont" else "Font")}"\);(?P<body>.*?)lua_pop\(L, 1\);',
        text,
        flags=re.DOTALL,
    )
    return match.group("body") if match else ""


def registered_methods(surface: Surface, text: str) -> list[str]:
    block = registration_block(surface, text)
    if surface.registration == "sol":
        names = re.findall(r'"([A-Za-z][A-Za-z0-9_]*)"\s*,', block)
        return sorted(set(names))
    return sorted(set(re.findall(r'REGISTER_LUA_CFUNC\(([^)]+)\)', block)))


def documented_methods(surface: Surface, text: str) -> list[str]:
    return sorted(
        set(
            re.findall(
                rf'@function\s+{re.escape(surface.doc_class)}:([A-Za-z][A-Za-z0-9_]*)',
                text,
            )
        )
    )


def class_comment(surface: Surface, text: str) -> str:
    match = re.search(
        rf'@class\s+{re.escape(surface.doc_class)}(?P<body>.*?)(?:/\*\*|\Z)',
        text,
        flags=re.DOTALL,
    )
    return match.group("body") if match else ""


def documented_fields(surface: Surface, text: str) -> list[str]:
    body = class_comment(surface, text)
    return sorted(set(re.findall(r'@field\s+([A-Za-z][A-Za-z0-9_]*)', body)))


def switch_properties(surface: Surface, text: str) -> list[str]:
    if surface.name == "RBO":
        body = re.search(r'int LuaRBOs::meta_index\(.*?\n\}', text, flags=re.DOTALL)
    elif surface.name == "LuaFont":
        body = re.search(r'int LuaFonts::meta_index\(.*?\n\}', text, flags=re.DOTALL)
    else:
        body = None
    if not body:
        return []
    return sorted(set(re.findall(r'hashString\("([A-Za-z][A-Za-z0-9_]*)"\)', body.group(0))))


def native_labels() -> set[str]:
    text = GFX_HEADER.read_text(encoding="utf-8")
    names = re.findall(r'void \(\*([A-Za-z][A-Za-z0-9_]*)\)\s*\(', text)

    def snake(value: str) -> str:
        value = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", value)
        return value.lower()

    return {f"Gfx.{snake(name)}" for name in names}


def tested_labels() -> set[str]:
    data = json.loads(SURFACE_SPEC.read_text(encoding="utf-8"))
    labels: set[str] = set()
    for row in data.get("tests", []):
        labels.update(str(value) for value in row.get("lua", []))
        labels.update(str(value) for value in row.get("native", []))
    return labels


def counterpart_tested(counterpart: tuple[str, ...], tested: set[str]) -> bool:
    return any(
        value in tested or value.rsplit(".", 1)[0] in tested
        for value in counterpart
    )


def counterpart_exists(counterpart: tuple[str, ...], native: set[str]) -> bool:
    return all(
        value in native or value.rsplit(".", 1)[0] in native
        for value in counterpart
    )


def signatures(surface: Surface, text: str, method: str) -> tuple[list[str], str]:
    match = re.search(
        rf'/\*\*.*?@function\s+{re.escape(surface.doc_class)}:{re.escape(method)}(?P<body>.*?)(?:\*/|\Z)',
        text,
        flags=re.DOTALL,
    )
    if not match:
        return [], "undocumented"
    params = re.findall(r'@param\s+([^\s]+)(?:\s+([^\n]+))?', match.group("body"))
    returns = re.findall(r'@return\s+([^\n]+)', match.group("body"))
    rendered = ", ".join(
        f"{name}: {description.strip()}" if description else name
        for name, description in params
    )
    if returns:
        rendered += f" -> {'; '.join(value.strip() for value in returns)}"
    return [name for name, _description in params], rendered or "no documented parameters"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=Path(__file__).with_name("lua_userdata_surface_audit.md"))
    args = parser.parse_args()

    native = native_labels()
    tested = tested_labels()
    rows: list[dict[str, object]] = []
    for surface in SURFACES:
        text = surface.source.read_text(encoding="utf-8")
        documentation = "\n".join(
            [text]
            + [path.read_text(encoding="utf-8") for path in surface.doc_sources]
        )
        methods = sorted(set(registered_methods(surface, text)) | set(documented_methods(surface, documentation)))
        if surface.name == "FBO":
            fields = documented_fields(surface, documentation)
            fields += ["dynamic attachment keys"]
        else:
            fields = sorted(set(documented_fields(surface, documentation)) | set(switch_properties(surface, text)))
        for method in methods:
            params, signature = signatures(surface, documentation, method)
            key = (surface.name, method)
            counterpart = COUNTERPARTS.get(key, ())
            if counterpart:
                status = "matched" if all(value in native or value.rsplit(".", 1)[0] == "Gfx.get_font_info" for value in counterpart) else "native-api-missing"
                if status == "matched" and not counterpart_tested(counterpart, tested):
                    status = "matched-but-untested"
            else:
                status = "unclassified-gap"
            rows.append({
                "surface": surface.name,
                "kind": "method",
                "name": method,
                "lua_signature": signature,
                "native": list(counterpart),
                "status": status,
            })
        for field in fields:
            key = (surface.name, field)
            counterpart = COUNTERPARTS.get(key, ())
            status = "matched" if counterpart and counterpart_exists(counterpart, native) else (
                "native-api-missing" if counterpart else "unclassified-gap"
            )
            if status == "matched" and not counterpart_tested(counterpart, tested):
                status = "matched-but-untested"
            rows.append({
                "surface": surface.name,
                "kind": "property",
                "name": field,
                "lua_signature": "property read",
                "native": list(counterpart),
                "status": status,
            })
        rows.append({
            "surface": surface.name,
            "kind": "lifecycle",
            "name": "__gc",
            "lua_signature": "metatable finalizer",
            "native": [],
            "status": "by-design",
        })

    unresolved = [row for row in rows if row["status"] == "unclassified-gap"]
    native_missing = [row for row in rows if row["status"] == "native-api-missing"]
    lines = [
        "# Lua userdata / class surface audit",
        "",
        "Generated from the active registration sites in `rts/Lua/LuaVAO.cpp`, `LuaVBO.cpp`, `LuaFonts.cpp`, `LuaRBOs.cpp`, and `LuaFBOs.cpp` plus their implementation documentation.",
        "This is separate from the free-callout inventory in `lua_functions.md`; native modules use explicit integer handles where Lua uses userdata.",
        "",
        "## Summary",
        "",
        f"- Inventory rows: {len(rows)}",
        f"- Matched rows: {sum(row['status'] == 'matched' for row in rows)}",
        f"- Matched but untested: {sum(row['status'] == 'matched-but-untested' for row in rows)}",
        f"- Unclassified gaps: {len(unresolved)}",
        f"- Native ABI missing for declared counterpart: {len(native_missing)}",
        "",
        "A complete parity claim requires zero unclassified gaps and zero matched-but-untested rows. `by-design` lifecycle rows are explicit exceptions, not coverage omissions.",
        "",
        "## Inventory",
        "",
        "| Surface | Kind | Lua member | Lua signature | Native counterpart | Status |",
        "| --- | --- | --- | --- | --- | --- |",
    ]
    for row in rows:
        native_value = ", ".join(f"`{value}`" for value in row["native"]) or "—"
        lines.append(
            f"| `{row['surface']}` | `{row['kind']}` | `{row['name']}` | {row['lua_signature']} | {native_value} | **{row['status']}** |"
        )
    lines.extend(["", "## Explicit design boundaries", ""])
    for (surface, name), reason in sorted(DESIGN_REASONS.items()):
        lines.append(f"- `{surface}.{name}` — {reason}")
    lines.extend(["", "## Audit interpretation", "", "- A Lua userdata method with no native counterpart is a porting gap, not a harmless naming mismatch.", "- A native integer-handle API is an acceptable representation change only when its lifecycle, result values, and documented parameters are tested against the Lua object.", "- Dynamic FBO fields are deliberately listed as a gap until typed native attachment operations and readback semantics exist.", ""])
    args.output.write_text("\n".join(lines), encoding="utf-8")
    print(f"wrote {args.output}")
    print(f"rows={len(rows)} unresolved={len(unresolved)} native_missing={len(native_missing)}")
    return 0 if not unresolved and not native_missing else 1


if __name__ == "__main__":
    raise SystemExit(main())
