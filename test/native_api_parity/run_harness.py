#!/usr/bin/env python3
"""Run the Native API parity fixture through Spring's normal script.txt path."""

from __future__ import annotations

import argparse
from collections import Counter
import json
import os
import random
import re
import shutil
import subprocess
import tempfile
import time
from pathlib import Path

from spec_loader import load_api_tests, load_known_issues

ROOT = Path(__file__).resolve().parents[2]
HARNESS = ROOT / "test" / "native_api_parity"
BASE_CONTENT = ROOT / "cont" / "base"
SOURCE_CONTENT = ROOT / "cont"
def default_engine_install() -> Path:
    for build_dir in ("build-amd64-linux", "build-linux"):
        install = ROOT / build_dir / "install"
        if (install / "spring-headless").exists():
            return install
    return ROOT / "build-linux" / "install"


ENGINE_INSTALL = default_engine_install()
DEFAULT_SPRING = ENGINE_INSTALL / "spring"
DEFAULT_SPRING_HEADLESS = ENGINE_INSTALL / "spring-headless"
GAME_FIXTURE = HARNESS / "fixtures" / "game.sdd"
NATIVE_CRATE = HARNESS / "native" / "Cargo.toml"
NATIVE_SO = HARNESS / "native" / "target" / "release" / "libnative_api_parity.so"
LUA_API_DOC = ROOT / "rust" / "crates" / "spring-native" / "lua_functions.md"
RUST_API_DOC = ROOT / "rust" / "crates" / "spring-native" / "rust_functions.md"
GAME_NAME = "Native API Parity 0.1"
RESULT_STREAMS = (
    "synced_gadget.jsonl",
    "unsynced_gadget.jsonl",
    "widget.jsonl",
)
CALLIN_LUA_STREAM = "callin_lua.jsonl"
CALLIN_NATIVE_STREAM = "callin_native.jsonl"

# C++ documents these in the general Callins section, but the implementation
# only invokes them for the synced LuaRules handle because their watch masks
# are not initialized for LuaUI.
SYNCED_GENERAL_CALLINS = {"Explosion", "ProjectileCreated", "ProjectileDestroyed"}
NATIVE_ACCESSOR_TYPES = {
    "features": "Features",
    "game": "Game",
    "terrain": "Terrain",
    "units_info": "UnitsInfo",
    "units_weapons": "UnitsWeapons",
    "platform": "Platform",
}
API_TESTS = load_api_tests()
KNOWN_ISSUES = load_known_issues()


def documented_callin_names() -> tuple[set[str], set[str]]:
    text = LUA_API_DOC.read_text(encoding="utf-8")
    synced = set(
        re.findall(r"- `SyncedCallins\.([A-Za-z0-9_]+)`", text)
    )
    unsynced = set(
        re.findall(r"- `UnsyncedCallins\.([A-Za-z0-9_]+)`", text)
    )
    return synced, unsynced


def coverage_metadata(test: dict) -> dict:
    """Return coverage labels, deriving them from the canonical spec when needed."""
    report = dict(test.get("report", {}))
    lua = test.get("lua", {})
    native = test.get("native", {})
    if not report.get("lua_setter") and lua.get("set"):
        report["lua_setter"] = ", ".join(f"`{name}`" for name in lua["set"])
    if not report.get("lua_getter") and lua.get("get"):
        report["lua_getter"] = ", ".join(f"`{name}`" for name in lua["get"])
    if not report.get("native_setter") and native.get("set"):
        report["native_setter"] = ", ".join(f"`{name}`" for name in native["set"])
    if not report.get("native_getter") and native.get("get"):
        report["native_getter"] = ", ".join(f"`{name}`" for name in native["get"])
    if not report.get("fields") and test.get("compare", {}).get("fields"):
        report["fields"] = ", ".join(f"`{field}`" for field in test["compare"]["fields"])
    return report


CHECK_COVERAGE = {test["id"]: coverage_metadata(test) for test in API_TESTS}
API_TEST_BY_ID = {test["id"]: test for test in API_TESTS}



def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--spring", type=Path, default=DEFAULT_SPRING)
    parser.add_argument("--spring-headless", type=Path, default=DEFAULT_SPRING_HEADLESS)
    parser.add_argument("--enable-rendering-tests", action="store_true")
    parser.add_argument("--map", help="existing map archive path, archive name, or known map name")
    parser.add_argument("--blank-map-x", type=int, default=10)
    parser.add_argument("--blank-map-y", type=int, default=8)
    parser.add_argument("--blank-map-height", type=int, default=96)
    parser.add_argument("--blank-map-seed", type=int)
    parser.add_argument("--test-seed", type=int)
    parser.add_argument("--cases", type=int, default=3)
    parser.add_argument(
        "--tests",
        help="comma-separated parity test IDs to run (useful for focused diagnostics)",
    )
    parser.add_argument(
        "--test-prefix",
        type=int,
        help="also run the first N generated synced tests (for focused diagnostics)",
    )
    parser.add_argument("--mode", choices=("lua", "native", "both", "compare"), default="both")
    parser.add_argument(
        "--load-native-module-for-lua",
        action="store_true",
        help="load the native module while retaining Lua-mode setter/readback behavior",
    )
    parser.add_argument(
        "--process-test",
        choices=("quit", "reload", "restart", "start"),
        help="run one isolated process-control parity case at the end of the fixture",
    )
    parser.add_argument("--output-dir", type=Path, default=HARNESS / "out")
    parser.add_argument("--timeout", type=int, default=120)
    parser.add_argument("--keep-workdir", action="store_true")
    parser.add_argument("--skip-native-build", action="store_true")
    return parser.parse_args()


def ensure_native_built(skip: bool) -> None:
    if skip:
        return

    subprocess.run(
        ["cargo", "build", "--manifest-path", str(NATIVE_CRATE), "--release"],
        cwd=ROOT,
        check=True,
    )


def link_or_copy(src: Path, dst: Path) -> None:
    if dst.exists() or dst.is_symlink():
        dst.unlink()

    try:
        dst.symlink_to(src)
    except OSError:
        if src.is_dir():
            shutil.copytree(src, dst)
        else:
            shutil.copy2(src, dst)


def prepare_datadir(workdir: Path, map_arg: str | None) -> tuple[Path, str | None]:
    datadir = workdir / "data"
    games = datadir / "games"
    maps = datadir / "maps"
    games.mkdir(parents=True)
    maps.mkdir(parents=True)

    link_or_copy(GAME_FIXTURE, games / "native_api_parity.sdd")
    for archive_name in ("springcontent", "maphelper", "bitmaps", "cursors"):
        link_or_copy(BASE_CONTENT / archive_name, games / f"{archive_name}.sdd")

    if not map_arg:
        return datadir, None

    map_path = Path(map_arg)
    if map_path.exists():
        link_or_copy(map_path.resolve(), maps / map_path.name)
        map_name = map_path.name
    else:
        map_name = map_arg

    return datadir, map_name


def blank_map_name(args: argparse.Namespace, run_mode: str) -> str:
    seed = args.blank_map_seed
    if seed is None:
        seed = random.SystemRandom().randint(1, 1_000_000_000)

    # Lua and native runs are separate Spring processes, but they must load
    # the same generated map for a meaningful parity comparison.  Including
    # the process mode here creates two different map identities and makes
    # the fixture depend on an accidental second RNG draw.
    return f"native_api_parity_blank_{seed}_{args.blank_map_x}x{args.blank_map_y}"


def write_script(
    path: Path,
    map_name: str,
    output_dir: str,
    run_mode: str,
    use_blank_map: bool,
    args: argparse.Namespace,
    test_seed: int,
) -> None:
    init_blank = "1" if use_blank_map else "0"
    host_port = random.SystemRandom().randint(20_000, 50_000)
    map_options = ""
    if use_blank_map:
        map_options = f"""
    [MAPOPTIONS]
    {{
        blank_map_x={args.blank_map_x};
        blank_map_y={args.blank_map_y};
        blank_map_height={args.blank_map_height};
        blank_map_color_r=64;
        blank_map_color_g=128;
        blank_map_color_b=64;
    }}
"""

    path.write_text(
        f"""[GAME]
{{
    IsHost=1;
    MyPlayerName=NativeApiParity;
    MapName={map_name};
    GameType={GAME_NAME};
    InitBlank={init_blank};
    StartPosType=0;
    FixedRNGSeed=1;
    OnlyLocal=1;
    HostIP=localhost;
    HostPort={host_port};
    MyPlayerNum=0;
    RecordDemo=0;
    GameStartDelay=0;
    MaxSpeed=1;
    MinSpeed=1;
    NumPlayers=1;
    NumTeams=1;
    NumAllyTeams=1;

    [MODOPTIONS]
    {{
        LuaRules=1;
        LuaGaia=0;
        native_api_parity_mode={run_mode};
        native_api_parity_output_dir={output_dir};
        native_api_parity_seed={test_seed};
        native_api_parity_cases={args.cases};
        native_api_parity_tests={args.tests or ''};
        native_api_parity_test_prefix={args.test_prefix if args.test_prefix is not None else ''};
        native_api_parity_enable_rendering_tests={1 if args.enable_rendering_tests else 0};
        native_api_parity_process_test={args.process_test or ''};
        native_api_parity_process_stage=initial;
    }}
{map_options}

    [PLAYER0]
    {{
        Name=NativeApiParity;
        Spectator=0;
        Team=0;
    }}

    [TEAM0]
    {{
        TeamLeader=0;
        AllyTeam=0;
        RGBColor=1 1 1;
        Side=Arm;
    }}

    [ALLYTEAM0]
    {{
        NumAllies=0;
    }}
}}
""",
        encoding="utf-8",
    )


def run_spring(args: argparse.Namespace, datadir: Path, script: Path, output_dir: Path, run_mode: str) -> int:
    env = os.environ.copy()
    result_dir = output_dir / "write-dir" / "native_api_parity"
    result_dir.mkdir(parents=True, exist_ok=True)
    data_dirs = [datadir, BASE_CONTENT]
    if ENGINE_INSTALL.is_dir():
        data_dirs.append(ENGINE_INSTALL)
    if SOURCE_CONTENT.is_dir():
        data_dirs.append(SOURCE_CONTENT)
    env["SPRING_DATADIR"] = os.pathsep.join(str(path) for path in data_dirs)
    env["SPRING_ISOLATED"] = str(datadir)
    env["SPRING_NATIVE_PARITY_OUTPUT_DIR"] = str(result_dir)

    if run_mode == "native" or (run_mode == "lua" and args.load_native_module_for_lua):
        env["SPRING_NATIVE_MODULE"] = str(NATIVE_SO)
    else:
        env["SPRING_NATIVE_MODULE"] = ""

    spring_binary = selected_spring_binary(args)
    cmd = [
        str(spring_binary),
        "--nocolor",
        "--write-dir",
        str(output_dir / "write-dir"),
        str(script),
    ]

    with (output_dir / "spring.log").open("wb") as log:
        proc = subprocess.Popen(cmd, cwd=ROOT, env=env, stdout=log, stderr=subprocess.STDOUT)
        (output_dir / "spring.pid").write_text(f"{proc.pid}\n", encoding="utf-8")
        try:
            return proc.wait(timeout=args.timeout)
        except subprocess.TimeoutExpired:
            proc.kill()
            proc.wait()
            raise RuntimeError(f"Spring timed out after {args.timeout}s; see {output_dir / 'spring.log'}")


def load_jsonl(path: Path) -> list[dict]:
    if not path.exists():
        return []
    rows = []
    for line in path.read_text(encoding="utf-8").splitlines():
        if line.strip():
            rows.append(json.loads(line))
    return rows


def row_test_name(row: dict) -> str:
    """Return the parity test identity without confusing API result `name` fields."""
    return str(row.get("testName", row.get("name", "")))


def compare(lua_dir: Path, native_dir: Path) -> bool:
    return compare_details(lua_dir, native_dir)["ok"]


def compare_details(lua_dir: Path, native_dir: Path) -> dict:
    ok = True
    streams = []
    for name in RESULT_STREAMS:
        lua_rows = load_jsonl(lua_dir / name)
        native_rows = load_jsonl(native_dir / name)
        matches = comparable_rows(lua_rows) == comparable_rows(native_rows)
        streams.append({
            "name": name,
            "lua_rows": len(lua_rows),
            "native_rows": len(native_rows),
            "matches": matches,
        })
        if not matches:
            print(f"mismatch: {name}")
            ok = False
        for side, rows in (("lua", lua_rows), ("native", native_rows)):
            failures = [
                row for row in rows
                if row_test_name(row).startswith("lua_rml_") and row.get("status") == "fail"
            ]
            if failures:
                print(f"{side} {name} RmlUi failures: {len(failures)}")
                for row in failures[:10]:
                    print(json.dumps(row, sort_keys=True))
                ok = False

    native_callin_rows = load_jsonl(native_dir / CALLIN_NATIVE_STREAM)
    native_lua_callin_rows = load_jsonl(native_dir / CALLIN_LUA_STREAM)
    callin_trace = compare_callin_traces(native_lua_callin_rows, native_callin_rows)
    if not callin_trace["matches"]:
        print("mismatch: engine callin Lua/native trace")
        for name in sorted(set(callin_trace["lua_counts"]) | set(callin_trace["native_counts"])):
            lua_count = callin_trace["lua_counts"].get(name, 0)
            native_count = callin_trace["native_counts"].get(name, 0)
            if lua_count != native_count:
                print(f"  {name}: lua={lua_count}, native={native_count}")
        ok = False

    native_path = native_dir / "native.jsonl"
    native_jsonl_exists = native_path.exists()
    if not native_jsonl_exists:
        print(f"missing: {native_path}")
        ok = False

    native_rows = load_jsonl(native_path)
    native_failures = [row for row in native_rows if row.get("status") == "fail"]
    native_complete = any(row_test_name(row) == "complete" and row.get("status") == "pass" for row in native_rows)
    if native_failures:
        print(f"native failures: {len(native_failures)}")
        for row in native_failures[:10]:
            print(json.dumps(row, sort_keys=True))
        ok = False
    if native_jsonl_exists and not native_complete:
        print(f"missing native completion marker in {native_path}")
        ok = False

    return {
        "ok": ok,
        "streams": streams,
        "native_jsonl_exists": native_jsonl_exists,
        "native_rows": native_rows,
        "native_failures": native_failures,
        "native_complete": native_complete,
        "callin_trace": callin_trace,
    }


def comparable_rows(rows: list[dict]) -> list[dict]:
    return [comparable_row(row) for row in rows]


def comparable_callin_rows(rows: list[dict]) -> list[dict]:
    """Compare the stable, semantic part of the Lua callin trace.

    The fixture runs in separate Spring processes, and the callin list can be
    delivered in a different order when rendering is active.  Argument object
    IDs are intentionally represented only by their normalized Lua value kind;
    source-level signature auditing handles the compact/native-vs-Lua field
    mapping separately.
    """
    result = []
    for row in rows:
        if not row.get("name"):
            continue
        result.append(
            {
                "context": row.get("context"),
                "name": row.get("name"),
                "arity": row.get("arity"),
                "args": row.get("args", []),
            }
        )
    return sorted(result, key=lambda row: json.dumps(row, sort_keys=True))


def compare_callin_traces(lua_rows: list[dict], native_rows: list[dict]) -> dict:
    """Compare event delivery counts for shared Lua/native engine callins."""
    synced_names, unsynced_names = documented_callin_names()

    def before_complete(rows: list[dict]) -> tuple[list[dict], bool]:
        for index, row in enumerate(rows):
            if row.get("context") == "callin_phase" and row.get("name") == "complete":
                return rows[:index], True
        return rows, False

    lua_rows, lua_phase_seen = before_complete(lua_rows)
    native_rows, native_phase_seen = before_complete(native_rows)

    def selected_lua_row(row: dict) -> bool:
        name = str(row.get("name", ""))
        context = str(row.get("context", ""))
        # Prefer the full-read synced gadget for synced callins, the unsynced
        # gadget for renderer/input callins, and the LuaUI handle for general
        # Callins.  The same general callback can legitimately be delivered to
        # both a gadget and a widget; the native event client receives one
        # engine event, so comparing all Lua handles would double-count it.
        if name in unsynced_names:
            return context == "unsynced_gadget"
        if name in synced_names or name in SYNCED_GENERAL_CALLINS:
            return context == "synced_gadget"
        return context == "lua_ui"

    lua_counts = Counter(
        str(row.get("name")) for row in lua_rows if row.get("name") and selected_lua_row(row)
    )
    native_counts = Counter(str(row.get("name")) for row in native_rows if row.get("name"))
    return {
        "matches": lua_counts == native_counts,
        "lua_rows": len(lua_rows),
        "native_rows": len(native_rows),
        "lua_phase_seen": lua_phase_seen,
        "native_phase_seen": native_phase_seen,
        "lua_counts": dict(lua_counts),
        "native_counts": dict(native_counts),
    }


def comparable_row(row: dict) -> dict:
    test = API_TEST_BY_ID.get(canonical_row_test_id(row_test_name(row)) or "")
    compare = test.get("compare", {}) if test else {}
    test_id = canonical_row_test_id(row_test_name(row))

    # The two sides run in separate engine processes.  Fixture/object IDs and
    # the arguments copied into a result row are transport data, not the API
    # result contract; they can legitimately differ when object allocation or
    # cleanup differs.  The manifest's compare.fields is the authoritative set
    # of values that this test promises to compare.
    if test is None:
        if row_test_name(row) == "context_inventory":
            return {
                "testName": row_test_name(row),
                "context": row.get("context"),
                "functions": row.get("functions", []),
            }
        return {
            "testName": row_test_name(row),
            "context": row.get("context"),
            "case": row.get("case"),
            "status": row.get("status"),
        }

    normalized = {
        "testName": test_id,
        "context": row.get("context"),
        "case": row.get("case"),
    }
    # Error-path rows and native-side failures are part of the observable
    # contract even when a spec has no result fields to compare.
    for field in ("status", "error"):
        if field in row:
            normalized[field] = row[field]

    epsilon = compare.get("epsilon")
    order_insensitive = set(test.get("order_insensitive_fields", []))
    normalizers = compare.get("normalizers", {})
    for field in compare.get("fields", []):
        if field not in row:
            # Preserve missing-vs-null and missing-vs-present distinctions.
            normalized[field] = {"__missing__": True}
            continue

        value = row[field]
        if field in order_insensitive and isinstance(value, list):
            value = sorted(value, key=lambda item: json.dumps(item, sort_keys=True))

        if normalizers.get(field) == "id_list":
            # Object IDs are allocated independently in the Lua baseline and
            # native processes.  Preserve the observable collection shape
            # while the native checker still compares the actual IDs within
            # the same process.
            normalized[field] = {
                "count": len(value) if isinstance(value, list) else None,
            }
        elif compare.get("stream") == "shape":
            normalized[field] = "number" if isinstance(value, (int, float)) else type(value).__name__
        elif isinstance(epsilon, (int, float)) and isinstance(value, (int, float)):
            normalized[field] = round(float(value) / float(epsilon))
        else:
            normalized[field] = value

    return normalized


def canonical_row_test_id(name: str) -> str | None:
    if name in API_TEST_BY_ID:
        return name
    for prefix in ("native_", "set_native_"):
        if name.startswith(prefix):
            test_id = name.removeprefix(prefix)
            if test_id in API_TEST_BY_ID:
                return test_id
    return None


def read_first_log_match(path: Path, needle: str) -> str | None:
    if not path.exists():
        return None
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        if needle in line:
            return line
    return None


def result_names(rows: list[dict]) -> list[str]:
    return [row_test_name(row) or "<unnamed>" for row in rows]


def read_report_option(script: Path, option: str) -> str | None:
    if not script.exists():
        return None
    prefix = f"{option}="
    for line in script.read_text(encoding="utf-8", errors="replace").splitlines():
        stripped = line.strip()
        if stripped.startswith(prefix):
            return stripped.removeprefix(prefix).removesuffix(";")
    return None


def selected_spring_binary(args: argparse.Namespace) -> Path:
    return args.spring if args.enable_rendering_tests else args.spring_headless


def report_link(path: Path) -> str:
    path = path.resolve()
    label = path.relative_to(ROOT) if path.is_relative_to(ROOT) else path
    target = str(path)
    if " " in target:
        target = f"<{target}>"
    return f"[{label}]({target})"


def read_lua_api_functions() -> set[str]:
    if not LUA_API_DOC.exists():
        return set()
    text = LUA_API_DOC.read_text(encoding="utf-8", errors="replace")
    return set(re.findall(r"`Spring\.([A-Za-z_][A-Za-z0-9_]*)`", text))


def read_rust_api_functions() -> set[str]:
    if not RUST_API_DOC.exists():
        return set()
    text = RUST_API_DOC.read_text(encoding="utf-8", errors="replace")
    return set(re.findall(r"^- `([A-Za-z_][A-Za-z0-9_]*\.[A-Za-z_][A-Za-z0-9_]*)`", text, re.MULTILINE))


def markdown_api_names(value: str) -> list[str]:
    if not value or value == "n/a" or value.startswith("n/a "):
        return []
    return re.findall(r"`([^`]+)`", value)


def recorded_test_ids(checked_names: set[str]) -> set[str]:
    return {
        name
        for name in CHECK_COVERAGE
        if name in checked_names or f"native_{name}" in checked_names or f"set_native_{name}" in checked_names
    }


def tested_lua_functions(recorded_ids: set[str] | None = None) -> set[str]:
    functions = set()
    items = CHECK_COVERAGE.items()
    if recorded_ids is not None:
        items = ((name, coverage) for name, coverage in items if name in recorded_ids)
    for _, coverage in items:
        for key in ("lua_setter", "lua_getter"):
            for name in markdown_api_names(coverage.get(key, "")):
                match = re.fullmatch(r"Spring\.([A-Za-z_][A-Za-z0-9_]*)", name)
                if match:
                    functions.add(match.group(1))
    return functions


def tested_rust_functions(recorded_ids: set[str] | None = None) -> set[str]:
    functions = set()
    items = CHECK_COVERAGE.items()
    if recorded_ids is not None:
        items = ((name, coverage) for name, coverage in items if name in recorded_ids)
    for _, coverage in items:
        for key in ("native_setter", "native_getter"):
            for name in markdown_api_names(coverage.get(key, "")):
                direct = re.fullmatch(r"([A-Za-z_][A-Za-z0-9_]*)\.([A-Za-z_][A-Za-z0-9_]*)", name)
                if direct:
                    functions.add(name)
                    continue

                accessor = re.fullmatch(r"([A-Za-z_][A-Za-z0-9_]*)\(\)\.([A-Za-z_][A-Za-z0-9_]*)", name)
                if accessor:
                    type_name = NATIVE_ACCESSOR_TYPES.get(accessor.group(1))
                    if type_name:
                        functions.add(f"{type_name}.{accessor.group(2)}")
    return functions


def lua_api_labels(coverage: dict) -> list[str]:
    labels = []
    for key in ("lua_setter", "lua_getter"):
        for name in markdown_api_names(coverage.get(key, "")):
            if re.fullmatch(r"Spring\.[A-Za-z_][A-Za-z0-9_]*", name):
                labels.append(name)
    return labels


def rust_api_labels(coverage: dict) -> list[str]:
    labels = []
    for key in ("native_setter", "native_getter"):
        for name in markdown_api_names(coverage.get(key, "")):
            if re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*\.[A-Za-z_][A-Za-z0-9_]*", name):
                labels.append(name)
            elif re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*\(\)\.[A-Za-z_][A-Za-z0-9_]*", name):
                labels.append(name)
    return labels


def coverage_summary(checked_names: set[str] | None = None) -> dict:
    lua_total = read_lua_api_functions() | known_issue_lua_functions("inventory_missing_docstring")
    rust_total = read_rust_api_functions()
    recorded_ids = recorded_test_ids(checked_names) if checked_names is not None else None
    lua_tested = tested_lua_functions(recorded_ids)
    rust_tested = tested_rust_functions(recorded_ids)
    return {
        "lua_total": lua_total,
        "lua_tested": lua_tested,
        "lua_tested_known": lua_tested & lua_total,
        "lua_tested_unknown": lua_tested - lua_total,
        "lua_untested": lua_total - lua_tested,
        "rust_total": rust_total,
        "rust_tested": rust_tested,
        "rust_tested_known": rust_tested & rust_total,
        "rust_tested_unknown": rust_tested - rust_total,
        "rust_untested": rust_total - rust_tested,
    }


def pct(part: int, total: int) -> str:
    if total == 0:
        return "n/a"
    return f"{part / total * 100:.1f}%"


def expanded_param_count(test: dict) -> int:
    total = 0
    for param in test.get("params", {}).values():
        param_type = param.get("type")
        if param_type in ("float2",):
            total += 2
        elif param_type in ("float3", "int3"):
            total += 3
        else:
            total += 1
    return total


def context_counts() -> dict[str, int]:
    counts: dict[str, int] = {}
    for test in API_TESTS:
        context = str(test.get("context", "unknown"))
        counts[context] = counts.get(context, 0) + 1
    return counts


def read_context_inventory(base_output: Path) -> dict[str, set[str]]:
    inventory: dict[str, set[str]] = {}
    lua_dir = base_output / "lua"
    for stream_name in RESULT_STREAMS:
        for row in load_jsonl(lua_dir / stream_name):
            if row_test_name(row) != "context_inventory":
                continue
            context = str(row.get("context") or stream_name.removesuffix(".jsonl"))
            functions = {
                str(name)
                for name in row.get("functions", [])
                if re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", str(name))
            }
            inventory.setdefault(context, set()).update(functions)
    return inventory


def canonical_test_id(name: str) -> str | None:
    if name in CHECK_COVERAGE:
        return name
    for prefix in ("native_", "set_native_"):
        if name.startswith(prefix) and name.removeprefix(prefix) in CHECK_COVERAGE:
            return name.removeprefix(prefix)
    return None


def read_recorded_ids_by_context(base_output: Path) -> dict[str, set[str]]:
    recorded: dict[str, set[str]] = {}
    lua_dir = base_output / "lua"
    for stream_name in RESULT_STREAMS:
        stream_context = stream_name.removesuffix(".jsonl")
        for row in load_jsonl(lua_dir / stream_name):
            name = row_test_name(row)
            test_id = canonical_test_id(name)
            if test_id is None:
                continue
            context = str(row.get("context") or stream_context)
            recorded.setdefault(context, set()).add(test_id)
    return recorded


def is_portable_readonly_test(test: dict) -> bool:
    return (
        test.get("kind") == "readonly"
        and test.get("lua_runtime") is not None
        and not test.get("requires_rendering", False)
        and not test.get("requires", [])
    )


def tests_for_context(context: str) -> list[dict]:
    tests = [test for test in API_TESTS if str(test.get("context", "unknown")) == context]
    if context in {"unsynced_gadget", "widget"}:
        seen = {test["id"] for test in tests}
        tests.extend(test for test in API_TESTS if test["id"] not in seen and is_portable_readonly_test(test))
    return tests


def context_coverage(
    checked_names: set[str],
    inventory: dict[str, set[str]] | None = None,
    recorded_by_context: dict[str, set[str]] | None = None,
) -> list[dict]:
    inventory = inventory or {}
    recorded_by_context = recorded_by_context or {}
    recorded_ids = recorded_test_ids(checked_names)
    contexts = sorted(
        {"synced_gadget", "unsynced_gadget", "widget"}
        | {str(test.get("context", "unknown")) for test in API_TESTS}
        | set(inventory)
    )
    rows = []
    for context in contexts:
        tests = tests_for_context(context)
        test_ids = {test["id"] for test in tests}
        recorded_context_ids = recorded_by_context.get(context, test_ids & recorded_ids)
        recorded_tests = [API_TEST_BY_ID[test_id] for test_id in sorted(recorded_context_ids) if test_id in API_TEST_BY_ID]
        lua_total = set()
        lua_recorded = set()
        native_total = set()
        native_recorded = set()
        for test in tests:
            coverage = CHECK_COVERAGE.get(test["id"], {})
            lua_names = {
                match.group(1)
                for api in lua_api_labels(coverage)
                if (match := re.fullmatch(r"Spring\.([A-Za-z_][A-Za-z0-9_]*)", api))
            }
            native_names = set()
            for api in rust_api_labels(coverage):
                direct = re.fullmatch(r"([A-Za-z_][A-Za-z0-9_]*)\.([A-Za-z_][A-Za-z0-9_]*)", api)
                if direct:
                    native_names.add(api)
                    continue
                accessor = re.fullmatch(r"([A-Za-z_][A-Za-z0-9_]*)\(\)\.([A-Za-z_][A-Za-z0-9_]*)", api)
                if accessor:
                    type_name = NATIVE_ACCESSOR_TYPES.get(accessor.group(1))
                    if type_name:
                        native_names.add(f"{type_name}.{accessor.group(2)}")
            lua_total |= lua_names
            native_total |= native_names
        for test in recorded_tests:
            coverage = CHECK_COVERAGE.get(test["id"], {})
            lua_recorded |= {
                match.group(1)
                for api in lua_api_labels(coverage)
                if (match := re.fullmatch(r"Spring\.([A-Za-z_][A-Za-z0-9_]*)", api))
            }
            for api in rust_api_labels(coverage):
                direct = re.fullmatch(r"([A-Za-z_][A-Za-z0-9_]*)\.([A-Za-z_][A-Za-z0-9_]*)", api)
                if direct:
                    native_recorded.add(api)
                    continue
                accessor = re.fullmatch(r"([A-Za-z_][A-Za-z0-9_]*)\(\)\.([A-Za-z_][A-Za-z0-9_]*)", api)
                if accessor:
                    type_name = NATIVE_ACCESSOR_TYPES.get(accessor.group(1))
                    if type_name:
                        native_recorded.add(f"{type_name}.{accessor.group(2)}")
        runtime_lua = inventory.get(context, set())
        lua_denominator = runtime_lua if runtime_lua else lua_total
        issue_counts = context_known_issue_counts(lua_denominator)
        rows.append({
            "context": context,
            "runtime_lua": len(runtime_lua),
            "spec_checks": len(tests),
            "recorded_checks": len(recorded_context_ids),
            "lua_spec": len(lua_total),
            "lua_recorded": len(lua_recorded),
            "lua_recorded_runtime": len(lua_recorded & lua_denominator),
            "lua_runtime_untested": len(lua_denominator - lua_recorded),
            "native_spec": len(native_total),
            "native_recorded": len(native_recorded),
            **issue_counts,
        })
    return rows


def known_issue_api_label(value: str) -> str:
    if value.startswith("Spring."):
        return f"`{value}`"
    return f"`{value}`"


def known_issue_lua_functions(status: str | None = None) -> set[str]:
    functions = set()
    for issue in KNOWN_ISSUES:
        if status is not None and issue.get("status") != status:
            continue
        for name in issue.get("lua", []):
            match = re.fullmatch(r"Spring\.([A-Za-z_][A-Za-z0-9_]*)", str(name))
            if match:
                functions.add(match.group(1))
    return functions


def issue_lua_functions(issue: dict) -> set[str]:
    functions = set()
    for name in issue.get("lua", []):
        match = re.fullmatch(r"Spring\.([A-Za-z_][A-Za-z0-9_]*)", str(name))
        if match:
            functions.add(match.group(1))
    return functions


def issue_native_functions(issue: dict) -> set[str]:
    return {
        str(name)
        for name in issue.get("native", [])
        if re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*\.[A-Za-z_][A-Za-z0-9_]*", str(name))
    }


def known_issue_status_summary() -> list[dict]:
    by_status: dict[str, dict[str, object]] = {}
    for issue in KNOWN_ISSUES:
        status = str(issue.get("status", "known_problem"))
        row = by_status.setdefault(status, {"status": status, "issues": 0, "lua": set(), "native": set()})
        row["issues"] = int(row["issues"]) + 1
        row["lua"].update(issue_lua_functions(issue))
        row["native"].update(issue_native_functions(issue))
    rows = []
    for status in sorted(by_status):
        row = by_status[status]
        rows.append({
            "status": status,
            "issues": row["issues"],
            "lua": len(row["lua"]),
            "native": len(row["native"]),
        })
    return rows


def context_known_issue_counts(context_functions: set[str]) -> dict[str, int]:
    issue_ids = set()
    mismatch_ids = set()
    affected_lua = set()
    affected_mismatch_lua = set()
    for issue in KNOWN_ISSUES:
        lua_names = issue_lua_functions(issue) & context_functions
        if not lua_names:
            continue
        issue_ids.add(issue["id"])
        affected_lua.update(lua_names)
        if issue.get("status") == "known_mismatch":
            mismatch_ids.add(issue["id"])
            affected_mismatch_lua.update(lua_names)
    return {
        "known_issue_count": len(issue_ids),
        "known_issue_lua": len(affected_lua),
        "known_mismatch_count": len(mismatch_ids),
        "known_mismatch_lua": len(affected_mismatch_lua),
    }


def known_issue_rust_functions(status: str | None = None) -> set[str]:
    functions = set()
    for issue in KNOWN_ISSUES:
        if status is not None and issue.get("status") != status:
            continue
        for name in issue.get("native", []):
            if re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*\.[A-Za-z_][A-Za-z0-9_]*", str(name)):
                functions.add(str(name))
    return functions


def write_coverage_details(
    base_output: Path,
    summary: dict,
    checked_names: set[str],
    inventory: dict[str, set[str]],
    recorded_by_context: dict[str, set[str]],
) -> Path:
    detail_path = base_output / "coverage_details.md"
    lines = [
        "# Native API Parity Coverage Details",
        "",
        "## Summary",
        "",
        "| Surface | Total Functions | Tested Functions | Coverage | Unknown Tested Names |",
        "| --- | ---: | ---: | ---: | ---: |",
        (
            f"| Lua `Spring.*` | {len(summary['lua_total'])} | {len(summary['lua_tested_known'])} | "
            f"{pct(len(summary['lua_tested_known']), len(summary['lua_total']))} | {len(summary['lua_tested_unknown'])} |"
        ),
        (
            f"| Native Rust | {len(summary['rust_total'])} | {len(summary['rust_tested_known'])} | "
            f"{pct(len(summary['rust_tested_known']), len(summary['rust_total']))} | {len(summary['rust_tested_unknown'])} |"
        ),
        "",
        "## Context Summary",
        "",
        "| Context | Runtime Spring APIs | Spec Checks | Recorded Checks | Lua APIs Tested / Runtime | Known Mismatch Issues | Affected Runtime APIs | Native APIs Recorded |",
        "| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |",
    ]
    for row in context_coverage(checked_names, inventory, recorded_by_context):
        lines.append(
            f"| `{row['context']}` | {row['runtime_lua']} | {row['spec_checks']} | {row['recorded_checks']} | "
            f"{row['lua_recorded_runtime']} / {row['runtime_lua']} | "
            f"{row['known_mismatch_count']} | {row['known_mismatch_lua']} | "
            f"{row['native_recorded']} |"
        )

    lines.extend([
        "## Tested Checks",
        "",
        "| Check | Context | Kind | Requires | Params | Recorded | Lua APIs | Native APIs |",
        "| --- | --- | --- | --- | ---: | --- | --- | --- |",
    ])

    for name, coverage in CHECK_COVERAGE.items():
        test = API_TEST_BY_ID.get(name, {})
        lua_apis = lua_api_labels(coverage)
        native_apis = rust_api_labels(coverage)
        recorded = name in checked_names or f"native_{name}" in checked_names or f"set_native_{name}" in checked_names
        lines.append(
            f"| `{name}` | `{test.get('context', 'unknown')}` | `{test.get('kind', 'unknown')}` | "
            f"{', '.join(f'`{item}`' for item in test.get('requires', [])) or 'n/a'} | "
            f"{expanded_param_count(test)} | {'yes' if recorded else 'no'} | "
            f"{', '.join(f'`{api}`' for api in lua_apis) or 'n/a'} | "
            f"{', '.join(f'`{api}`' for api in native_apis) or 'n/a'} |"
        )

    lines.extend([
        "",
        "## Known Problems",
        "",
        "| Issue | Status | Lua APIs | Native APIs | Problem |",
        "| --- | --- | --- | --- | --- |",
    ])
    if KNOWN_ISSUES:
        for issue in KNOWN_ISSUES:
            lua_apis = ", ".join(known_issue_api_label(str(name)) for name in issue.get("lua", [])) or "n/a"
            native_apis = ", ".join(known_issue_api_label(str(name)) for name in issue.get("native", [])) or "n/a"
            lines.append(
                f"| `{issue['id']}` | `{issue.get('status', 'known_problem')}` | "
                f"{lua_apis} | {native_apis} | {issue.get('problem', '')} |"
            )
    else:
        lines.append("| n/a | n/a | n/a | n/a | none |")

    lines.extend([
        "",
        "## Untested Lua Functions",
        "",
    ])
    if summary["lua_untested"]:
        lines.extend(f"- `Spring.{name}`" for name in sorted(summary["lua_untested"]))
    else:
        lines.append("- none")

    if summary["lua_tested_unknown"]:
        lines.extend(["", "## Tested Lua Names Missing From Inventory", ""])
        lines.extend(f"- `Spring.{name}`" for name in sorted(summary["lua_tested_unknown"]))

    lines.extend([
        "",
        "## Untested Native Functions",
        "",
    ])
    if summary["rust_untested"]:
        lines.extend(f"- `{name}`" for name in sorted(summary["rust_untested"]))
    else:
        lines.append("- none")

    if summary["rust_tested_unknown"]:
        lines.extend(["", "## Tested Native Names Missing From Inventory", ""])
        lines.extend(f"- `{name}`" for name in sorted(summary["rust_tested_unknown"]))

    detail_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return detail_path


def write_report(base_output: Path, args: argparse.Namespace, compare_info: dict | None) -> None:
    lua_dir = base_output / "lua"
    native_dir = base_output / "native"
    case_count = read_report_option(lua_dir / "script.txt", "native_api_parity_cases") or "n/a"
    rendering_enabled = (
        read_report_option(lua_dir / "script.txt", "native_api_parity_enable_rendering_tests") == "1"
        or args.enable_rendering_tests
    )
    lines = [
        "# Native API Parity Report",
        "",
        f"- Result: {'PASS' if (compare_info and compare_info['ok']) else 'PARTIAL/UNVERIFIED'}",
        f"- Generated: {time.strftime('%Y-%m-%d %H:%M:%S %z')}",
        f"- Spring binary: {report_link(args.spring if rendering_enabled else args.spring_headless)}",
        f"- Rendering tests: `{'enabled' if rendering_enabled else 'disabled'}`",
        f"- Mode: `{args.mode}`",
        f"- Test seed: `{read_report_option(lua_dir / 'script.txt', 'native_api_parity_seed') or 'n/a'}`",
        f"- Cases per check: `{case_count}`",
        f"- Fixture game: {report_link(GAME_FIXTURE)}",
        f"- Native module: {report_link(NATIVE_SO)}",
        "",
        "## What Ran",
        "",
    ]

    for label, run_dir in (("Lua baseline", lua_dir), ("Native comparison", native_dir)):
        if not run_dir.is_dir():
            continue
        log = run_dir / "spring.log"
        lines.extend([
            f"### {label}",
            "",
            f"- Directory: {report_link(run_dir)}",
            f"- Script: {report_link(run_dir / 'script.txt')}",
            f"- Spring log: {report_link(log)}",
        ])
        for needle in (
            "Loaded SYNCED gadget",
            "Loaded UNSYNCED gadget",
            "LuaUI Entry Point",
            "Successfully opened plugin",
            "Native module initialized successfully",
            "Player NativeApiParity finished loading",
            "[QuitAction]",
        ):
            match = read_first_log_match(log, needle)
            if match:
                lines.append(f"- `{match}`")
        lines.append("")

    lines.extend([
        "## Result Streams",
        "",
        "| Stream | Lua Rows | Native Rows | Match |",
        "| --- | ---: | ---: | --- |",
    ])

    if compare_info:
        for stream in compare_info["streams"]:
            lines.append(
                f"| `{stream['name']}` | {stream['lua_rows']} | {stream['native_rows']} | "
                f"{'yes' if stream['matches'] else 'NO'} |"
            )
    else:
        for name in RESULT_STREAMS:
            lines.append(f"| `{name}` | {len(load_jsonl(lua_dir / name))} | n/a | n/a |")

    if compare_info:
        trace = compare_info["callin_trace"]
        lua_trace_rows = len(load_jsonl(lua_dir / CALLIN_LUA_STREAM))
        native_lua_trace_rows = len(load_jsonl(native_dir / CALLIN_LUA_STREAM))
        native_trace_rows = len(load_jsonl(native_dir / CALLIN_NATIVE_STREAM))
        lines.extend([
            "",
            "## Engine Callin Trace",
            "",
            f"- Lua trace rows: baseline `{lua_trace_rows}`, native run `{native_lua_trace_rows}`",
            f"- Native trace rows: `{native_trace_rows}`",
            f"- Shared event counts: `{'match' if trace['matches'] else 'MISMATCH'}`",
        ])

    lines.extend([
        "",
        "## Native Checks",
        "",
        "| Check | Pass | Fail |",
        "| --- | ---: | ---: |",
    ])

    native_rows = compare_info["native_rows"] if compare_info else load_jsonl(native_dir / "native.jsonl")
    if native_rows:
        native_counts: dict[str, dict[str, int]] = {}
        native_failures = []
        for row in native_rows:
            name = row_test_name(row) or "<unnamed>"
            status = str(row.get("status", ""))
            native_counts.setdefault(name, {"pass": 0, "fail": 0})
            if status == "pass":
                native_counts[name]["pass"] += 1
            elif status == "fail":
                native_counts[name]["fail"] += 1
                native_failures.append(row)

        for name, counts in native_counts.items():
            lines.append(f"| `{name}` | {counts['pass']} | {counts['fail']} |")

        if native_failures:
            lines.extend(["", "### Native Failures", ""])
            for row in native_failures:
                lines.append(f"- `{row_test_name(row) or '<unnamed>'}`: {row.get('message', '')}")
    else:
        lines.append("| n/a | 0 | 0 |")

    checked_names = set(result_names(native_rows))
    summary = coverage_summary(checked_names)
    inventory = read_context_inventory(base_output)
    recorded_by_context = read_recorded_ids_by_context(base_output)
    coverage_details = write_coverage_details(base_output, summary, checked_names, inventory, recorded_by_context)

    lines.extend([
        "",
        "## API Coverage",
        "",
        f"- Details: {report_link(coverage_details)}",
        f"- Known problems: `{len(KNOWN_ISSUES)}`",
        "",
        "| Surface | Total Functions | Tested Functions | Coverage |",
        "| --- | ---: | ---: | ---: |",
        (
            f"| Lua `Spring.*` | {len(summary['lua_total'])} | {len(summary['lua_tested_known'])} | "
            f"{pct(len(summary['lua_tested_known']), len(summary['lua_total']))} |"
        ),
        (
            f"| Native Rust | {len(summary['rust_total'])} | {len(summary['rust_tested_known'])} | "
            f"{pct(len(summary['rust_tested_known']), len(summary['rust_total']))} |"
        ),
        "",
        "| Surface | Untested Functions | Unknown Tested Names |",
        "| --- | ---: | ---: |",
        f"| Lua `Spring.*` | {len(summary['lua_untested'])} | {len(summary['lua_tested_unknown'])} |",
        f"| Native Rust | {len(summary['rust_untested'])} | {len(summary['rust_tested_unknown'])} |",
        "",
        "## Known Problem Counts",
        "",
        "| Status | Issues | Lua APIs | Native APIs |",
        "| --- | ---: | ---: | ---: |",
    ])
    for row in known_issue_status_summary():
        lines.append(f"| `{row['status']}` | {row['issues']} | {row['lua']} | {row['native']} |")

    lines.extend([
        "",
        "## Context Coverage",
        "",
        "| Context | Runtime Spring APIs | Recorded Checks | Lua APIs Tested / Runtime | Runtime Untested | Known Mismatch Issues | Affected Runtime APIs | Native APIs Recorded |",
        "| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |",
    ])
    for row in context_coverage(checked_names, inventory, recorded_by_context):
        lines.append(
            f"| `{row['context']}` | {row['runtime_lua']} | {row['recorded_checks']} | "
            f"{row['lua_recorded_runtime']} / {row['runtime_lua']} | {row['lua_runtime_untested']} | "
            f"{row['known_mismatch_count']} | {row['known_mismatch_lua']} | "
            f"{row['native_recorded']} |"
        )

    lines.extend([
        "",
        "## Known Problems",
        "",
        "| Issue | Status | Lua APIs | Native APIs | Problem |",
        "| --- | --- | --- | --- | --- |",
    ])
    if KNOWN_ISSUES:
        for issue in KNOWN_ISSUES:
            lua_apis = ", ".join(known_issue_api_label(str(name)) for name in issue.get("lua", [])) or "n/a"
            native_apis = ", ".join(known_issue_api_label(str(name)) for name in issue.get("native", [])) or "n/a"
            lines.append(
                f"| `{issue['id']}` | `{issue.get('status', 'known_problem')}` | "
                f"{lua_apis} | {native_apis} | {issue.get('problem', '')} |"
            )
    else:
        lines.append("| n/a | n/a | n/a | n/a | none |")

    lines.extend([
        "",
        "## Current Check Coverage",
        "",
        "Methodology: setter rows are tested in both directions when equivalent. Read-only rows compare Lua getter output against native getter output. Full function lists and gaps are in the details file.",
        "",
        "| Check | Context | Kind | Cases | Params | Compared Fields |",
        "| --- | --- | --- | ---: | ---: | --- |",
    ])
    covered_checks = [
        name for name in CHECK_COVERAGE
        if name in checked_names or f"native_{name}" in checked_names or f"set_native_{name}" in checked_names
    ]
    if covered_checks:
        for name in covered_checks:
            coverage = CHECK_COVERAGE.get(name, {})
            test = API_TEST_BY_ID.get(name, {})
            lines.append(
                f"| `{name}` | `{test.get('context', 'unknown')}` | `{test.get('kind', 'unknown')}` | "
                f"{case_count} | {expanded_param_count(test)} | {coverage.get('fields', 'n/a')} |"
            )
    else:
        lines.append("| n/a | n/a | n/a | 0 | 0 | No native parity checks were recorded. |")

    lines.extend([
        "",
        "## Files",
        "",
        f"- {report_link(coverage_details)}",
    ])
    for run_dir in (lua_dir, native_dir):
        if not run_dir.is_dir():
            continue
        for path in sorted(run_dir.glob("*.jsonl")):
            lines.append(f"- {report_link(path)}")

    (base_output / "report.md").write_text("\n".join(lines) + "\n", encoding="utf-8")


def run_one(args: argparse.Namespace, base_output: Path, run_mode: str, test_seed: int) -> Path:
    run_output = base_output / run_mode
    run_output.mkdir(parents=True, exist_ok=True)
    result_dir = run_output / "write-dir" / "native_api_parity"
    result_dir.mkdir(parents=True, exist_ok=True)

    with tempfile.TemporaryDirectory(prefix="native-api-parity-", dir=base_output) as tmp:
        workdir = Path(tmp)
        datadir, map_name = prepare_datadir(workdir, args.map)
        use_blank_map = map_name is None
        if use_blank_map:
            map_name = blank_map_name(args, run_mode)
        script = run_output / "script.txt"
        write_script(script, map_name, "native_api_parity", run_mode, use_blank_map, args, test_seed)

        exit_code = run_spring(args, datadir, script, run_output, run_mode)
        for result_file in result_dir.glob("*.jsonl"):
            shutil.copy2(result_file, run_output / result_file.name)
        if run_mode == "native":
            try:
                spring_pid = int((run_output / "spring.pid").read_text(encoding="utf-8"))
            except (OSError, ValueError) as err:
                raise RuntimeError(f"could not determine native Spring PID: {err}") from err
            parent_result = result_dir / f"native-{spring_pid}.jsonl"
            if not parent_result.is_file():
                raise RuntimeError(f"missing native result stream for Spring PID {spring_pid}: {parent_result}")
            shutil.copy2(parent_result, run_output / "native.jsonl")
        if args.keep_workdir:
            kept = run_output / "workdir"
            if kept.exists():
                shutil.rmtree(kept)
            shutil.copytree(workdir, kept, symlinks=True)

        if exit_code != 0:
            raise RuntimeError(f"Spring exited with {exit_code}; see {run_output / 'spring.log'}")

    return run_output


def main() -> int:
    args = parse_args()
    # Spring requires absolute isolated and write-directory paths.  Resolve
    # user-supplied relative output roots before creating temporary fixtures;
    # this also makes custom diagnostic output roots behave like the default.
    args.output_dir = args.output_dir.resolve()
    if not args.spring.is_file():
        raise SystemExit(f"spring binary not found: {args.spring}")
    if not args.spring_headless.is_file():
        raise SystemExit(f"spring-headless binary not found: {args.spring_headless}")

    if args.mode in ("native", "both"):
        ensure_native_built(args.skip_native_build)

    if args.mode == "compare":
        runs = sorted(
            path
            for path in args.output_dir.iterdir()
            if path.is_dir() and (path / "lua").is_dir() and (path / "native").is_dir()
        )
        if not runs:
            raise SystemExit(f"no harness runs found in {args.output_dir}")
        latest = runs[-1]
        compare_info = compare_details(latest / "lua", latest / "native")
        write_report(latest, args, compare_info)
        return 0 if compare_info["ok"] else 1

    # Include a sub-second suffix so concurrent or rapid diagnostic runs do
    # not write into the same result directory.
    timestamp = f"{time.strftime('%Y%m%d-%H%M%S')}-{time.time_ns() % 1_000_000:06d}"
    test_seed = args.test_seed
    if test_seed is None:
        test_seed = random.SystemRandom().randint(1, 1_000_000_000)
    if args.map is None and args.blank_map_seed is None:
        # Choose the generated map identity once per parity run so both
        # process modes receive identical map metadata and content.
        args.blank_map_seed = random.SystemRandom().randint(1, 1_000_000_000)
    base_output = args.output_dir / timestamp
    base_output.mkdir(parents=True, exist_ok=True)

    lua_dir = run_one(args, base_output, "lua", test_seed) if args.mode in ("lua", "both") else None
    native_dir = run_one(args, base_output, "native", test_seed) if args.mode in ("native", "both") else None

    compare_info = compare_details(lua_dir, native_dir) if lua_dir and native_dir else None
    write_report(base_output, args, compare_info)

    if compare_info and not compare_info["ok"]:
        return 1

    print(base_output)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
