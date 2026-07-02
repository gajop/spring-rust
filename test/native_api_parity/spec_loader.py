#!/usr/bin/env python3
"""Load Native API parity test specs from a manifest or legacy single file."""

from __future__ import annotations

import json
from pathlib import Path


HARNESS = Path(__file__).resolve().parent
SPEC = HARNESS / "api_tests.json"
KNOWN_ISSUES = HARNESS / "known_issues.json"


def load_api_tests(spec: Path = SPEC) -> list[dict]:
    with spec.open(encoding="utf-8") as file:
        data = json.load(file)

    if "includes" in data:
        tests: list[dict] = []
        for include in data["includes"]:
            include_path = (spec.parent / include).resolve()
            with include_path.open(encoding="utf-8") as file:
                include_data = json.load(file)
            include_tests = include_data.get("tests")
            if not isinstance(include_tests, list):
                raise ValueError(f"{include_path} must contain a list field named `tests`")
            tests.extend(include_tests)
    else:
        tests = data.get("tests")
        if not isinstance(tests, list):
            raise ValueError(f"{spec} must contain `tests` or `includes`")

    validate_tests(tests, spec)
    return tests


def load_known_issues(spec: Path = KNOWN_ISSUES) -> list[dict]:
    if not spec.exists():
        return []

    with spec.open(encoding="utf-8") as file:
        data = json.load(file)

    issues = data.get("issues", [])
    if not isinstance(issues, list):
        raise ValueError(f"{spec} must contain a list field named `issues`")

    seen: set[str] = set()
    for issue in issues:
        issue_id = issue.get("id")
        if not isinstance(issue_id, str) or not issue_id:
            raise ValueError(f"{spec} contains an issue without a string `id`: {issue!r}")
        if issue_id in seen:
            raise ValueError(f"duplicate known issue id in {spec}: {issue_id}")
        seen.add(issue_id)
        for key in ("lua", "native"):
            if key in issue and not isinstance(issue[key], list):
                raise ValueError(f"{issue_id}.{key} must be a list")
    return issues


def validate_tests(tests: list[dict], source: Path = SPEC) -> None:
    valid_param_types = {"i32", "f32", "bool", "string", "enum", "object", "float2", "float3", "int3"}
    seen: set[str] = set()
    for test in tests:
        test_id = test.get("id")
        if not isinstance(test_id, str) or not test_id:
            raise ValueError(f"{source} contains a test without a string `id`: {test!r}")
        if test_id in seen:
            raise ValueError(f"duplicate test id in {source}: {test_id}")
        seen.add(test_id)
        if "requires_rendering" in test and not isinstance(test["requires_rendering"], bool):
            raise ValueError(f"{test_id}.requires_rendering must be a boolean")

        params = test.get("params", {})
        if not isinstance(params, dict):
            raise ValueError(f"{test_id}.params must be an object")
        for param_name, param in params.items():
            if not isinstance(param, dict):
                raise ValueError(f"{test_id}.params.{param_name} must be an object")
            param_type = param.get("type")
            if param_type not in valid_param_types:
                raise ValueError(f"{test_id}.params.{param_name} has unknown type {param_type!r}")
            if "range" in param and (not isinstance(param["range"], list) or len(param["range"]) != 2):
                raise ValueError(f"{test_id}.params.{param_name}.range must be [min, max]")
