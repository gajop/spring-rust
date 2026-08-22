"""Canonical parity metadata loading and native-call resolution."""

from __future__ import annotations

import json
import re
from pathlib import Path

from . import core
from .types import snake


def load_tests(
    manifest_path: Path = core.API_MANIFEST,
    api_root: Path = core.API_ROOT,
) -> list[dict]:
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    tests: list[dict] = []
    for relative in manifest["includes"]:
        source = json.loads((api_root / relative).read_text(encoding="utf-8"))
        tests.extend(source.get("tests", source if isinstance(source, list) else []))
    return tests


def load_model(
    model_path: Path = core.MODEL_PATH,
) -> tuple[
    dict[tuple[str, str], dict],
    dict[str, dict],
    dict[str, dict],
    dict[str, dict],
]:
    model = json.loads(model_path.read_text(encoding="utf-8"))
    functions = {
        (module["name"], snake(function["name"])): function
        for module in model["modules"]
        for function in module["functions"]
    }
    records = {
        record["name"]: record
        for module in model["modules"]
        for record in module.get("records", [])
    }
    enums = {
        enum["name"]: enum
        for module in model["modules"]
        for enum in module.get("enums", [])
    }
    modules = {module["name"]: module for module in model["modules"]}
    return functions, records, modules, enums


def native_function(test: dict, functions: dict[tuple[str, str], dict]) -> tuple[str, str, dict] | None:
    override = core.NATIVE_TEST_FUNCTION_OVERRIDES.get(test.get("id"))
    if override is not None:
        module, function_name = override
        function = functions.get((module, function_name))
        if function is not None:
            return module, function_name, function

    native_get = test.get("native", {}).get("get", [])
    if not native_get:
        sequence = test.get("wasm_sequence") or []
        if sequence:
            target = core.runtime_call_target(sequence[0].get("call", ""), functions)
            if target is not None:
                return target
        return None

    match = re.fullmatch(r"([^.]+?)(?:\(\))?\.([^.]+)", native_get[0])
    if not match:
        return None
    class_name = match.group(1)
    module = core.MODULE_BY_NATIVE_CLASS.get(class_name)
    if module is None:
        module = core.MODULE_BY_NATIVE_CLASS.get(class_name[:1].upper() + class_name[1:])
    if module is None:
        module = next(
            (
                candidate_module
                for candidate_class, candidate_module in core.MODULE_BY_NATIVE_CLASS.items()
                if snake(candidate_class) == snake(class_name)
            ),
            None,
        )
    if module is None:
        return None
    function_name = match.group(2)
    function = functions.get((module, function_name))
    if function is None:
        compact_name = function_name.replace("_", "").lower()
        matches = [
            (candidate_name, candidate)
            for (candidate_module, candidate_name), candidate in functions.items()
            if candidate_module == module and candidate_name.replace("_", "").lower() == compact_name
        ]
        if len(matches) == 1:
            function_name, function = matches[0]
    return (module, function_name, function) if function is not None else None


__all__ = ["load_model", "load_tests", "native_function"]
