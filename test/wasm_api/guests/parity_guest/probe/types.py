"""Semantic type and identifier helpers for the parity probe generator."""

from __future__ import annotations

import re


SCALAR_WIT_TYPES = {
    "bool": "bool",
    "i8": "s8",
    "i16": "s16",
    "i32": "s32",
    "i64": "s64",
    "u8": "u8",
    "u16": "u16",
    "u32": "u32",
    "u64": "u64",
    "f32": "f32",
    "f64": "f64",
}

WIT_KEYWORDS = {
    "use", "type", "func", "u8", "u16", "u32", "u64", "s8", "s16",
    "s32", "s64", "f32", "f64", "char", "own", "borrow", "resource",
    "record", "flags", "variant", "enum", "bool", "string", "option",
    "result", "future", "stream", "error-context", "list", "map", "_",
    "as", "from", "static", "interface", "import", "export", "world",
    "package", "constructor", "include", "with", "async",
}

RUST_KEYWORDS = {
    "as", "async", "await", "become", "box", "break", "const", "continue",
    "crate", "dyn", "else", "enum", "extern", "false", "fn", "for", "if",
    "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait",
    "true", "type", "unsafe", "use", "where", "while", "yield",
}


def snake(value: str) -> str:
    value = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", value)
    value = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", value)
    return value.lower()


def pascal(value: str) -> str:
    return "".join(
        part.capitalize()
        for part in snake(value.replace("-", "_")).split("_")
        if part
    )


def kebab(value: str) -> str:
    return snake(value).strip("_").replace("_", "-")


def wit_identifier(value: str) -> str:
    value = kebab(value)
    return f"%{value}" if value in WIT_KEYWORDS else value


def rust_identifier(value: str) -> str:
    identifier = snake(value)
    return f"{identifier}_" if identifier in RUST_KEYWORDS else identifier


def type_kind(type_info: dict) -> str:
    return type_info["kind"]


def wit_type(type_info: dict) -> str:
    kind = type_kind(type_info)
    if kind == "scalar":
        return SCALAR_WIT_TYPES[type_info["name"]]
    if kind == "enum":
        return kebab(type_info["name"])
    if kind == "string":
        return "string"
    if kind == "bytes":
        return "list<u8>"
    if kind in {"list", "fixed-array"}:
        return f"list<{wit_type(type_info['element'])}>"
    if kind == "option":
        return f"option<{wit_type(type_info.get('inner', type_info.get('element'))) }>"
    if kind in {"callback", "pointer", "opaque"}:
        return "u32"
    if kind == "record":
        return kebab(type_info["name"])
    raise ValueError(f"unsupported probe WIT type: {type_info}")


def simple_type(type_info: dict) -> bool:
    return type_info.get("kind") in {"scalar", "enum", "string"}


def records_field(record: dict, name: str) -> dict | None:
    wanted = snake(name)
    for field in record.get("fields", []):
        candidate = snake(field["name"])
        if candidate == wanted or candidate.replace("_", "") == wanted.replace("_", ""):
            return field
    return None


def vector_component(type_info: dict, index: int, records: dict[str, dict]) -> dict | None:
    if type_info.get("kind") != "record":
        return None
    record = records.get(type_info["name"])
    if record is None or index < 1:
        return None
    fields = record.get("fields", [])
    if index > len(fields):
        return None
    candidate = fields[index - 1]
    return candidate if candidate["type"].get("kind") == "scalar" else None


def semantic_path(
    type_info: dict,
    path: list[str | int],
    records: dict[str, dict],
) -> tuple[dict, list[str | int]] | None:
    current = type_info
    resolved: list[str | int] = []
    for component in path:
        if isinstance(component, str):
            if current.get("kind") != "record":
                return None
            record = records.get(current["name"])
            field = records_field(record, component) if record is not None else None
            if field is None:
                return None
            resolved.append(field["name"])
            current = field["type"]
            continue
        if not isinstance(component, int) or component < 0:
            return None
        if current.get("kind") == "record":
            field = vector_component(current, component, records)
            if field is None:
                return None
            resolved.append(field["name"])
            current = field["type"]
            continue
        if current.get("kind") in {"list", "fixed-array"}:
            index = component if component == 0 else component - 1
            resolved.append(index)
            current = current["element"]
            continue
        if current.get("kind") == "bytes":
            resolved.append(component)
            current = {"kind": "scalar", "name": "u8"}
            continue
        return None
    return current, resolved


def rust_semantic_path(expression: str, path: list[str | int]) -> str:
    for component in path:
        if isinstance(component, int):
            expression += f".get({component}).copied().unwrap_or_default()"
        else:
            expression += f".{rust_identifier(component)}"
    return expression


def type_supported_by_probe(
    type_info: dict,
    records: dict[str, dict],
    seen: set[str] | None = None,
) -> bool:
    kind = type_info.get("kind")
    if kind in {"scalar", "enum", "string", "bytes", "callback", "pointer", "opaque"}:
        return True
    if kind in {"list", "fixed-array"}:
        return type_supported_by_probe(type_info["element"], records, seen)
    if kind == "option":
        return type_supported_by_probe(type_info.get("inner", type_info.get("element")), records, seen)
    if kind != "record":
        return False
    name = type_info["name"]
    seen = set() if seen is None else seen
    if name in seen:
        return True
    record = records.get(name)
    if record is None:
        return False
    seen.add(name)
    return all(type_supported_by_probe(field["type"], records, seen) for field in record["fields"])


__all__ = [
    "kebab", "pascal", "records_field", "rust_identifier", "rust_semantic_path",
    "semantic_path", "simple_type", "snake", "type_kind", "type_supported_by_probe",
    "vector_component", "wit_identifier", "wit_type",
]
