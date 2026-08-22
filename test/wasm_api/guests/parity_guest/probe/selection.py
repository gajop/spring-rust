"""Selection and coverage accounting for the parity probe."""

from __future__ import annotations

import json
import re
from functools import lru_cache

from . import core
from .model import load_model, load_tests, native_function
from .types import snake

ROOT = core.ROOT
CONTEXT_SOURCE = core.CONTEXT_SOURCE
CORE_CONTEXT_BITS = {
    "synced_gadget": 1 << 0,
    "unsynced_gadget": 1 << 1,
    "gaia_synced": 1 << 2,
    "gaia_unsynced": 1 << 3,
    "ui": 1 << 4,
}
probe_arguments = core.probe_arguments
supported_output = core.supported_output
runtime_call_target = core.runtime_call_target
wasm_sequence_operations = core.wasm_sequence_operations
wasm_set_operations = core.wasm_set_operations

def select_tests(
    functions: dict[tuple[str, str], dict],
    records: dict[str, dict],
    context: str = "synced_gadget",
    include_rendering: bool = False,
    transport: str = "core",
) -> list[tuple[dict, str, str, dict, list[str]]]:
    selected, _coverage = select_tests_with_coverage(
        functions, records, context, include_rendering, transport
    )
    return selected


SELECTION_EXCLUSION_REASONS = {
    "deferred",
    "unsupported_kind",
    "native_only",
    "expect_error",
    "rendering_disabled",
    "no_lua_runtime",
    "no_native_function",
    "mutating_getter_or_unsupported",
    "unresolved_setter",
    "unsupported_output",
    "unresolved_args",
    "unresolved_sequence",
    "core_policy",
    "core_owned_unsupported",
}

CORE_CONTEXT_BITS = {
    "synced_gadget": 1 << 0,
    "unsynced_gadget": 1 << 1,
    "gaia_synced": 1 << 2,
    "gaia_unsynced": 1 << 3,
    "ui": 1 << 4,
}

# These probes require a semantic adapter beyond the raw Core import.  Keep
# them explicit until the corresponding record/option adapters are generated;
# they must not be reported as a passing Core observation.
CORE_OWNED_UNSUPPORTED_TESTS = frozenset(
    {}
)


@lru_cache(maxsize=1)
def core_import_coverage() -> dict[tuple[str, str], dict]:
    coverage_path = ROOT / "rts" / "wasm" / "generated" / "core-executable-coverage.json"
    entries = json.loads(coverage_path.read_text(encoding="utf-8"))
    return {
        (entry["module"], entry["function"]): entry
        for key in ("executable", "pending")
        for entry in entries.get(key, [])
    }


@lru_cache(maxsize=1)
def core_owned_unsupported() -> frozenset[tuple[str, str]]:
    """Return model callouts absent from the generated owned façade.

    The façade intentionally omits shapes without a reviewed lowering.  Keep
    the parity manifest honest by excluding those tests at generation time;
    an absent Rust item must never be turned into a vacuous runtime result.
    """
    owned_dir = ROOT / "rts" / "wasm" / "generated" / "sdk"
    owned_path = owned_dir / "core_owned.rs"
    text = owned_path.read_text(encoding="utf-8")
    shard_dir = owned_dir / "core_owned"
    text += "".join(
        path.read_text(encoding="utf-8")
        for path in sorted(shard_dir.glob("*.rs"))
    )
    text += (owned_dir / "core_owned_footer.rs").read_text(encoding="utf-8")
    modules: dict[str, set[str]] = {}
    module_pattern = re.compile(
        r"^    pub mod ([A-Za-z0-9_]+) \{(?P<body>.*?)(?=^    pub mod |^    #\[doc\(hidden\)\])",
        re.MULTILINE | re.DOTALL,
    )
    function_pattern = re.compile(r"^        pub fn ([A-Za-z0-9_]+)\(", re.MULTILINE)
    for match in module_pattern.finditer(text):
        modules[match.group(1)] = set(function_pattern.findall(match.group("body")))

    model_functions, _records, _modules, _enums = load_model()
    return frozenset(
        (module, snake(function.get("name", "")))
        for (module, _name), function in model_functions.items()
        if snake(function.get("name", "")) not in modules.get(module, set())
    )


def core_import_allowed(module: str, function: dict, context: str) -> bool:
    function_name = snake(function.get("name", ""))
    if (module, function_name) in core_owned_unsupported():
        return False
    entry = core_import_coverage().get((module, function.get("name")))
    if entry is None:
        return False
    return bool(
        entry.get("production_import_allowed")
        and entry.get("production_process_safe")
        and entry.get("production_environment_mask", 0) & CORE_CONTEXT_BITS[context]
    )


def core_test_policy_allows(
    test: dict,
    module: str,
    function: dict,
    context: str,
    functions: dict[tuple[str, str], dict],
    records: dict[str, dict],
) -> bool:
    if test.get("id") in CORE_OWNED_UNSUPPORTED_TESTS:
        return False
    targets = [(module, function)]
    for sequence_module, _name, sequence_function, _args, _bind in (
        wasm_sequence_operations(test, functions, records) or []
    ):
        targets.append((sequence_module, sequence_function))
    for setter_module, _name, setter_function, _args in (
        wasm_set_operations(test, functions, records) or []
    ):
        targets.append((setter_module, setter_function))
    callback = test.get("wasm_callback")
    if callback and callback.get("call"):
        callback_target = runtime_call_target(callback["call"], functions)
        if callback_target is None:
            return False
        targets.append((callback_target[0], callback_target[2]))
    return all(core_import_allowed(target_module, target_function, context) for target_module, target_function in targets)


def select_tests_with_coverage(
    functions: dict[tuple[str, str], dict],
    records: dict[str, dict],
    context: str = "synced_gadget",
    include_rendering: bool = False,
    transport: str = "core",
) -> tuple[
    list[tuple[dict, str, str, dict, list[str]]],
    dict,
]:
    """Select portable probes and account for every canonical test row.

    A Wasm probe is intentionally smaller than the full Lua/native fixture:
    custom hooks, error-only tests, rendering-only tests, and rows without a
    generated semantic counterpart cannot be emitted into this Core probe.
    A row may opt into Wasm error coverage when the API's error result
    is itself the contract under test.  The important property is that every
    other row remains visible in the generated manifest with a reason.  A
    future API row therefore cannot disappear merely because a new selector
    branch forgot to handle it.
    """
    source_context = CONTEXT_SOURCE[context]
    selected: list[tuple[dict, str, str, dict, list[str]]] = []
    entries = []
    seen_ids: set[str] = set()
    for test in load_tests():
        test_id = test.get("id")
        if not test_id or test_id in seen_ids:
            raise ValueError(f"duplicate or missing canonical parity test id: {test_id!r}")
        seen_ids.add(test_id)
        if test.get("context") != source_context:
            continue
        reason = None
        selected_entry = None

        # The Wasm probe deliberately excludes custom/native-only/error-only
        # rows, but it must cover both readonly APIs and portable
        # setter/getter APIs.  Mutability belongs to the setter operation; the
        # getter selected below must itself remain a read operation.
        if test.get("deferred"):
            reason = "deferred"
        elif test.get("kind") not in {"readonly", "setter_getter"}:
            reason = "unsupported_kind"
        elif test.get("native_only") and test.get("wasm_sequence") is None:
            reason = "native_only"
        elif test.get("expect_error") and not test.get("wasm_expected_error"):
            reason = "expect_error"
        elif test.get("requires_rendering") and not include_rendering:
            reason = "rendering_disabled"
        elif test.get("lua_runtime") is None and test.get("wasm_returns") is None:
            reason = "no_lua_runtime"
        else:
            native = native_function(test, functions)
            if native is None:
                reason = "no_native_function"
            else:
                module, function_name, function = native
                if (
                    (
                        function.get("mutating")
                        and not test.get("wasm_expected_error")
                        and test.get("wasm_sequence") is None
                        and not test.get("wasm_mutating")
                    )
                    or function.get("status") == "unsupported"
                ):
                    reason = "mutating_getter_or_unsupported"
                elif (
                    test.get("wasm_sequence") is not None
                    and wasm_sequence_operations(test, functions, records) is None
                ):
                    reason = "unresolved_sequence"
                elif (
                    test.get("kind") == "setter_getter"
                    and test.get("wasm_sequence") is None
                    and wasm_set_operations(test, functions, records) is None
                ):
                    # Resolve this even though render_rust() repeats the
                    # lookup.  A failed lookup is an explicit
                    # non-portable/custom row, not a silently vacuous Wasm
                    # result.
                    reason = "unresolved_setter"
                elif supported_output(test, function, records, functions) is None:
                    reason = "unsupported_output"
                else:
                    arguments = (
                        []
                        if test.get("wasm_sequence") is not None
                        else probe_arguments(test, function, records, module)
                    )
                    if arguments is None:
                        reason = "unresolved_args"
                    else:
                        selected_entry = (test, module, function_name, function, arguments)

        if selected_entry is not None and transport == "core":
            selected_test, selected_module, _, selected_function, _ = selected_entry
            if not core_test_policy_allows(
                selected_test,
                selected_module,
                selected_function,
                context,
                functions,
                records,
            ):
                selected_entry = None
                reason = (
                    "core_owned_unsupported"
                    if selected_test.get("id") in CORE_OWNED_UNSUPPORTED_TESTS
                    else (
                        "core_owned_unsupported"
                        if any(
                            target in core_owned_unsupported()
                            for target in [
                                (selected_module, snake(selected_function.get("name", "")))
                            ]
                        )
                        else "core_policy"
                    )
                )

        if selected_entry is not None:
            selected.append(selected_entry)
            reason = "selected"
        if reason not in SELECTION_EXCLUSION_REASONS and reason != "selected":
            raise ValueError(f"unclassified Wasm probe selection result for {test_id}: {reason}")
        entries.append({"id": test_id, "kind": test.get("kind"), "status": reason})

    context_entries = [entry for entry in entries if entry["id"] in seen_ids]
    excluded = {
        reason: [entry["id"] for entry in context_entries if entry["status"] == reason]
        for reason in sorted(SELECTION_EXCLUSION_REASONS)
        if any(entry["status"] == reason for entry in context_entries)
    }
    coverage = {
        "source_context": source_context,
        "source_test_count": len(context_entries),
        "selected_count": len(selected),
        "selected_kind_counts": {
            kind: sum(
                entry["status"] == "selected" and entry["kind"] == kind
                for entry in context_entries
            )
            for kind in ("readonly", "setter_getter")
        },
        "selected_ids": [entry["id"] for entry in context_entries if entry["status"] == "selected"],
        "excluded": excluded,
    }
    if coverage["source_test_count"] != coverage["selected_count"] + sum(
        len(ids) for ids in excluded.values()
    ):
        raise ValueError(
            f"Wasm probe coverage does not account for every {source_context} test"
        )
    return selected, coverage
