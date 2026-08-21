#!/usr/bin/env python3
"""Run the Native API parity fixture through Spring's normal script.txt path."""

from __future__ import annotations

import argparse
from collections import Counter
import json
import math
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
LSAN_SUPPRESSION = HARNESS / "lsan.supp"
WASM_PARITY_CRATE = ROOT / "test" / "wasm_api" / "parity_guest" / "Cargo.toml"
WASM_PARITY_CORE = (
    ROOT
    / "test"
    / "wasm_api"
    / "parity_guest"
    / "target"
    / "wasm32-unknown-unknown"
    / "release"
    / "recoil_wasm_parity_guest.wasm"
)
WASM_PARITY_MODULE = (
    ROOT
    / "test"
    / "wasm_api"
    / "parity_guest"
    / "target"
    / "recoil_wasm_parity_guest.component.wasm"
)
WASM_PARITY_CORE_MODULE = (
    ROOT
    / "test"
    / "wasm_api"
    / "parity_guest"
    / "target"
    / "wasm32-unknown-unknown"
    / "release"
    / "recoil_wasm_parity_guest.core.wasm"
)
WASM_PROBE_GENERATOR = ROOT / "test" / "wasm_api" / "parity_guest" / "generate_probe.py"
WASM_CONTEXTS = ("synced_gadget", "unsynced_gadget", "gaia_synced", "gaia_unsynced", "ui")
WASM_CONTEXT_MODULES = {
	"synced_gadget": WASM_PARITY_MODULE,
	"unsynced_gadget": ROOT / "test" / "wasm_api" / "parity_guest" / "target" / "recoil_wasm_parity_guest.unsynced_gadget.component.wasm",
	"gaia_synced": ROOT / "test" / "wasm_api" / "parity_guest" / "target" / "recoil_wasm_parity_guest.gaia_synced.component.wasm",
	"gaia_unsynced": ROOT / "test" / "wasm_api" / "parity_guest" / "target" / "recoil_wasm_parity_guest.gaia_unsynced.component.wasm",
	"ui": ROOT / "test" / "wasm_api" / "parity_guest" / "target" / "recoil_wasm_parity_guest.ui.component.wasm",
}
WASM_CONTEXT_CORE_MODULES = {
	context: WASM_PARITY_CORE_MODULE.with_name(
		f"recoil_wasm_parity_guest.{context}.core.wasm"
	)
	for context in WASM_CONTEXTS
}
WASM_CONTEXT_MANIFESTS = {
	context: ROOT / "test" / "wasm_api" / "parity_guest" / f"probe_manifest_{context}.json"
	for context in WASM_CONTEXTS
}
WASM_CONTEXT_CORE_MANIFESTS = {
	context: ROOT / "test" / "wasm_api" / "parity_guest" / f"probe_manifest_{context}.core.json"
	for context in WASM_CONTEXTS
}
WASM_CONTEXT_LUA_OUTPUTS = {
	context: GAME_FIXTURE / "LuaRules/Utilities" / (
		"wasm_api_probe_tests.lua"
		if context == "synced_gadget"
		else f"wasm_api_probe_tests_{context}.lua"
	)
	for context in WASM_CONTEXTS
}
WASM_CONTEXT_CORE_LUA_OUTPUTS = {
	context: GAME_FIXTURE / "LuaRules/Utilities" / (
		"wasm_api_probe_tests_core.lua"
		if context == "synced_gadget"
		else f"wasm_api_probe_tests_{context}_core.lua"
	)
	for context in WASM_CONTEXTS
}
WASM_CONTEXT_MAP_MANIFESTS = {
	context: ROOT / "test" / "wasm_api" / "parity_guest" / f"map_manifest_{context}.txt"
	for context in WASM_CONTEXTS
}
WASM_ENVIRONMENT_NAMES = {
	"synced_gadget": "rules-synced",
	"unsynced_gadget": "rules-unsynced",
	"gaia_synced": "gaia-synced",
	"gaia_unsynced": "gaia-unsynced",
	"ui": "ui",
}
LUA_API_DOC = ROOT / "rust" / "crates" / "spring-native" / "lua_functions.md"
RUST_API_DOC = ROOT / "rust" / "crates" / "spring-native" / "rust_functions.md"
API_SURFACE_AUDIT_DOC = ROOT / "rust" / "crates" / "spring-native" / "api_surface_audit.md"
CALLIN_SOURCE = ROOT / "rts" / "NativeInterface" / "NativeInterfaceEventClient.cpp"
EVENTS_DEF = ROOT / "rts" / "System" / "Events.def"
GADGET_ROUTER_SOURCE = ROOT / "cont" / "base" / "springcontent" / "LuaGadgets" / "gadgets.lua"
GAME_NAME = "Native API Parity 0.1"
RESULT_STREAMS = (
    "synced_gadget.jsonl",
    "unsynced_gadget.jsonl",
    "widget.jsonl",
)
CALLIN_LUA_STREAM = "callin_lua.jsonl"
CALLIN_NATIVE_STREAM = "callin_native.jsonl"
WASM_PARITY_STREAM = "wasm.jsonl"
WASM_PROBE_MANIFEST = ROOT / "test" / "wasm_api" / "parity_guest" / "probe_manifest.json"
WASM_VACUITY_THRESHOLD = 0.05

# Gaia parity injects these two map options only to tell the blank-map
# generator which manifest and component to load.  They are harness controls,
# not public map options: Spring.GetMapOptions already hides them, while the
# native Game.get_map_options view sees the raw map-generator options.
GAIA_HARNESS_MAP_OPTION_KEYS = frozenset({
    "blank_map_gaia_manifest",
    "blank_map_gaia_module",
})

# The Lua and native symbols named Shutdown are separate lifecycle hooks: Lua
# shuts down each Lua handle, while native shuts down the loaded module.  They
# are documented under the same label but are not corresponding event payloads.
LIFECYCLE_ONLY_CALLINS = {"Shutdown"}
# The save/load lifecycle is routed through LuaRules' gadget handler; LuaUI
# intentionally does not receive Save, and the full-read LuaRules handle is
# the corresponding public surface for Load.
LUA_RULES_ONLY_CALLINS = {"Load", "Save"}
# These control callins are exposed by the stock LuaRules router, but this
# engine delivers the control event to LuaUI rather than the unsynced gadget
# handle. Keep selection based on observed engine routing, not just a method
# name in the router.
LUA_UI_CONTROL_CALLINS = {"GameSetup", "GetTooltip"}
# Some general callins are delivered to both LuaRules and LuaUI, but the
# LuaRules router deliberately adapts their payload before dispatching it.
# Native receives the full event payload, which is the LuaUI-facing contract
# for these callbacks.  Select that handle for value-level comparison.
LUA_UI_NATIVE_SIGNATURE_CALLINS = {
    "KeyPress",
    "KeyRelease",
    "DownloadQueued",
    "DownloadStarted",
    "DownloadProgress",
    "DownloadFinished",
    "DownloadFailed",
    "ViewResize",
    "DrawWorldPreParticles",
}
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
SURFACE_TESTS_PATH = HARNESS / "surface_tests.json"
SURFACE_TESTS = json.loads(SURFACE_TESTS_PATH.read_text(encoding="utf-8"))["tests"]
SURFACE_TEST_BY_ID = {test["id"]: test for test in SURFACE_TESTS}


def documented_callin_names() -> tuple[set[str], set[str], set[str]]:
    """Return the three documented Lua callin namespaces.

    The generated markdown intentionally keeps the namespace in each entry,
    but the runtime callback name is only the suffix.  Keeping the three sets
    separate lets the fixture choose the one Lua handle that corresponds to a
    native event without double-counting a general callin delivered to both
    LuaUI and LuaRules.
    """
    text = LUA_API_DOC.read_text(encoding="utf-8")
    general = set(
        re.findall(r"- `Callins\.([A-Za-z0-9_]+)`", text)
    )
    synced = set(
        re.findall(r"- `SyncedCallins\.([A-Za-z0-9_]+)`", text)
    )
    unsynced = set(
        re.findall(r"- `UnsyncedCallins\.([A-Za-z0-9_]+)`", text)
    )
    return general, synced, unsynced


def event_properties() -> dict[str, str]:
    text = EVENTS_DEF.read_text(encoding="utf-8")
    return {
        name: properties
        for name, properties in re.findall(
            r"SETUP_EVENT\((\w+),\s*([^\n)]+)\)", text
        )
    }


def general_callin_contexts(general_names: set[str]) -> tuple[set[str], set[str]]:
    """Return managed general callins for synced and unsynced Lua handles."""
    properties = event_properties()
    managed = general_names & set(properties)
    synced = {
        name for name in managed if "UNSYNCED_BIT" not in properties[name]
    }
    unsynced = {
        name for name in managed if "UNSYNCED_BIT" in properties[name]
    }
    return synced, unsynced


def gadget_router_callins() -> set[str]:
    """Return callins with an actual stock LuaRules gadget-router method."""
    text = GADGET_ROUTER_SOURCE.read_text(encoding="utf-8")
    return set(re.findall(r"^function gadgetHandler:(\w+)\(", text, re.MULTILINE))


def native_event_callin_names() -> set[str]:
    """Return callback symbols exposed by the engine event client.

    This is deliberately read from the same LOAD_SYMBOL list that controls
    the ABI, rather than copied into the test.  InitializeNativeModule is a
    module lifecycle entry point, not an engine-to-Lua/native callin.
    """
    text = CALLIN_SOURCE.read_text(encoding="utf-8")
    body = text.split("void NativeInterfaceEventClient::LoadSymbols()", 1)[1].split(
        "void* NativeInterfaceEventClient::Initialize()", 1
    )[0]
    return set(re.findall(r"^\s*LOAD_SYMBOL\((\w+)\)", body, re.MULTILINE)) - {
        "InitializeNativeModule"
    }


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


def native_comparable(test: dict) -> bool:
    """Whether the direct native module shares the Lua test's read role.

    NativeInterfaceEventClient is an all-access event client.  LuaUI/widget
    tests intentionally use the local player's restricted visibility context,
    so comparing those observations directly would turn a role difference into
    a false parity failure.  A spec may override this default explicitly when
    a widget-shaped API has role-independent semantics.
    """
    return bool(test.get("native_compare", test.get("context") != "widget"))


def native_comparison_exclusions() -> set[str]:
    return {
        test_id for test_id, test in API_TEST_BY_ID.items() if not native_comparable(test)
    }


def native_comparable_row(row: dict) -> bool:
    test_id = canonical_row_test_id(row_test_name(row))
    test = API_TEST_BY_ID.get(test_id or "")
    return test is None or native_comparable(test)



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
        "--skip-callin-compare",
        action="store_true",
        help="skip the engine callin trace gate for a focused non-rendering diagnostic",
    )
    parser.add_argument(
        "--test-prefix",
        type=int,
        help="also run the first N generated synced tests (for focused diagnostics)",
    )
    parser.add_argument(
        "--mode",
        choices=("lua", "native", "wasm", "both", "compare"),
        default="both",
    )
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
    parser.add_argument("--skip-wasm-build", action="store_true")
    parser.add_argument(
        "--wasm-context",
        choices=WASM_CONTEXTS,
        default="synced_gadget",
        help="Wasm execution environment used by --mode wasm",
    )
    parser.add_argument(
        "--wasm-transport",
        choices=("component", "core"),
        default="component",
        help="Wasm transport used by --mode wasm",
    )
    return parser.parse_args()


def ensure_native_built(skip: bool) -> None:
    if skip:
        return

    subprocess.run(
        ["cargo", "build", "--manifest-path", str(NATIVE_CRATE), "--release"],
        cwd=ROOT,
        check=True,
    )


CORE_CONTEXT_FEATURES = {
	"synced_gadget": "core_rules_synced",
	"unsynced_gadget": "core_rules_unsynced",
	"gaia_synced": "core_gaia_synced",
	"gaia_unsynced": "core_gaia_unsynced",
	"ui": "core_ui",
}


def ensure_wasm_built(
	skip: bool, transport: str = "component", context: str = "synced_gadget"
) -> None:
	if skip:
		modules = WASM_CONTEXT_CORE_MODULES if transport == "core" else WASM_CONTEXT_MODULES
		manifests = WASM_CONTEXT_CORE_MANIFESTS if transport == "core" else WASM_CONTEXT_MANIFESTS
		missing = [modules[context], manifests[context]]
		if context.startswith("gaia_"):
			missing.append(WASM_CONTEXT_MAP_MANIFESTS[context])
		if transport == "core":
			missing.append(WASM_CONTEXT_CORE_LUA_OUTPUTS[context])
		missing = [path for path in missing if not path.is_file()]
		if missing:
			raise RuntimeError("missing Wasm parity components: " + ", ".join(map(str, missing)))
		return

	installed_targets = subprocess.run(
		["rustup", "target", "list", "--installed"],
		cwd=ROOT,
		check=True,
		capture_output=True,
		text=True,
	).stdout.splitlines()
	if "wasm32-unknown-unknown" not in installed_targets:
		subprocess.run(["rustup", "target", "add", "wasm32-unknown-unknown"], cwd=ROOT, check=True)
	build_context = context
	generator_args = ["python3", str(WASM_PROBE_GENERATOR), "--context", build_context]
	if transport == "core":
		generator_args.extend(["--transport", "core"])
	lua_output = (
		WASM_CONTEXT_CORE_LUA_OUTPUTS[build_context]
		if transport == "core"
		else WASM_CONTEXT_LUA_OUTPUTS[build_context]
	)
	generator_args.extend(["--lua-output", str(lua_output)])
	manifests = WASM_CONTEXT_CORE_MANIFESTS if transport == "core" else WASM_CONTEXT_MANIFESTS
	generator_args.extend(["--manifest-output", str(manifests[build_context])])
	subprocess.run(generator_args, cwd=ROOT, check=True)
	build_args = [
		"cargo",
		"build",
		"--manifest-path",
		str(WASM_PARITY_CRATE),
		"--target",
		"wasm32-unknown-unknown",
		"--release",
	]
	if transport == "core":
		build_args.extend(["--features", f"core,{CORE_CONTEXT_FEATURES[build_context]}"])
	subprocess.run(build_args, cwd=ROOT, check=True)
	if transport == "core":
		core_module = WASM_CONTEXT_CORE_MODULES[build_context]
		core_module.parent.mkdir(parents=True, exist_ok=True)
		shutil.copy2(WASM_PARITY_CORE, core_module)
		return
	component = WASM_CONTEXT_MODULES[build_context]
	component.parent.mkdir(parents=True, exist_ok=True)
	subprocess.run(
		[
			"cargo",
			"run",
			"--manifest-path",
			str(WASM_PARITY_CRATE),
			"--bin",
			"componentize",
			"--release",
			"--",
			str(WASM_PARITY_CORE),
			str(component),
		],
		cwd=ROOT,
		check=True,
	)
	if not component.is_file():
		raise RuntimeError(f"Wasm parity component was not produced: {component}")
	if build_context.startswith("gaia_"):
		WASM_CONTEXT_MAP_MANIFESTS[build_context].write_text(
			f"module(parity, LuaGaia/wasm/parity.wasm, {WASM_ENVIRONMENT_NAMES[build_context]}, 0, 1.0.0)\n",
			encoding="utf-8",
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


def prepare_datadir(
    workdir: Path,
    map_arg: str | None,
    wasm_module: Path | None = None,
    wasm_context: str = "synced_gadget",
    wasm_transport: str = "component",
) -> tuple[Path, str | None]:
    datadir = workdir / "data"
    games = datadir / "games"
    maps = datadir / "maps"
    games.mkdir(parents=True)
    maps.mkdir(parents=True)

    game_destination = games / "native_api_parity.sdd"
    if wasm_module is None or wasm_context.startswith("gaia_"):
        link_or_copy(GAME_FIXTURE, game_destination)
    else:
        shutil.copytree(GAME_FIXTURE, game_destination, symlinks=True)
        wasm_owner = "LuaUI" if wasm_context == "ui" else "LuaRules"
        wasm_directory = game_destination / wasm_owner / "wasm"
        wasm_directory.mkdir(parents=True, exist_ok=True)
        shutil.copy2(wasm_module, wasm_directory / "parity.wasm")
        (wasm_directory / "manifest.txt").write_text(
            f"module(parity, {wasm_owner}/wasm/parity.wasm, {WASM_ENVIRONMENT_NAMES[wasm_context]}, 0, 1.0.0)\n",
            encoding="utf-8",
        )
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
    wasm_context: str,
	wasm_module: Path | None,
	wasm_role: str = "combined",
	wasm_transport: str = "component",
	benchmark_backend: str = "",
    benchmark_repeats: int = 5,
    benchmark_scale: float = 1.0,
    benchmark_case: str = "",
    benchmark_iterations: int = 0,
    benchmark_callin_variant: str = "empty",
) -> None:
    init_blank = "1" if use_blank_map else "0"
    host_port = random.SystemRandom().randint(20_000, 50_000)
    # Benchmark workloads are measured inside the callin and must not be
    # dominated by the normal real-time draw loop. A high simulation speed
    # keeps the 5,000 logical frames reproducible while allowing Spring to
    # advance and run its normal per-frame GC between samples.
    speed_limit = "100" if run_mode == "benchmark" else "1"
    # The parity fixture is the test harness.  Its Lua handles perform the
    # assertions and issue the in-engine quit command; the launcher only
    # waits for that natural exit.  Keep LuaUI enabled because it is the
    # result bridge for unsynced gadget and Gaia probes, but its expensive
    # ordinary parity/UI cases are gated by native_api_parity_mode=wasm.
    map_options = ""
    if use_blank_map:
        gaia_options = ""
        if wasm_module is not None and wasm_context.startswith("gaia_"):
            gaia_options = f"""
        blank_map_gaia_manifest={WASM_CONTEXT_MAP_MANIFESTS[wasm_context]};
        blank_map_gaia_module={wasm_module};"""
        map_options = f"""
    [MAPOPTIONS]
    {{
        blank_map_x={args.blank_map_x};
        blank_map_y={args.blank_map_y};
        blank_map_height={args.blank_map_height};
        blank_map_color_r=64;
        blank_map_color_g=128;
        blank_map_color_b=64;
{gaia_options}
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
    MaxSpeed={speed_limit};
    MinSpeed={speed_limit};
    NumPlayers=1;
    NumTeams=2;
    NumAllyTeams=2;

    [MODOPTIONS]
    {{
        LuaRules=1;
        LuaGaia=1;
        native_api_parity_mode={run_mode};
        native_api_parity_output_dir={output_dir};
        native_api_parity_seed={test_seed};
        native_api_parity_cases={args.cases};
        native_api_parity_tests={args.tests or ''};
        native_api_parity_test_prefix={args.test_prefix if args.test_prefix is not None else ''};
        native_api_parity_enable_rendering_tests={1 if args.enable_rendering_tests else 0};
        native_api_parity_process_test={args.process_test or ''};
        native_api_parity_process_stage=initial;
        native_api_parity_wasm_context={wasm_context};
        native_api_parity_wasm_transport={wasm_transport};
        native_api_parity_wasm_role={wasm_role};
        native_api_parity_benchmark_backend={benchmark_backend};
        native_api_parity_benchmark_repeats={benchmark_repeats};
        native_api_parity_benchmark_scale={benchmark_scale};
        native_api_parity_benchmark_case={benchmark_case};
        native_api_parity_benchmark_iterations={benchmark_iterations};
        native_api_parity_benchmark_callin_variant={benchmark_callin_variant};
        wasm_instruction_fuel=0;
        wasm_host_work_limit=0;
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

    [TEAM1]
    {{
        TeamLeader=0;
        AllyTeam=1;
        RGBColor=1 0 0;
        Side=Arm;
    }}

    [ALLYTEAM0]
    {{
        NumAllies=0;
    }}

    [ALLYTEAM1]
    {{
        NumAllies=0;
    }}
}}
""",
        encoding="utf-8",
    )


def run_spring(
    args: argparse.Namespace,
    datadir: Path,
    script: Path,
    output_dir: Path,
    run_mode: str,
    wasm_context: str = "synced_gadget",
    wasm_transport: str = "component",
) -> int:
    env = os.environ.copy()
    result_dir = output_dir / "write-dir" / "native_api_parity"
    result_dir.mkdir(parents=True, exist_ok=True)
    needs_graphics = args.enable_rendering_tests or wasm_context in {
        "unsynced_gadget",
        "gaia_unsynced",
        "ui",
    }
    if needs_graphics:
        # The installed settings file defaults to exclusive fullscreen.  The
        # engine's `--window` flag cannot override that persisted value on
        # this branch, so provide an isolated windowed config explicitly.
        (output_dir / "write-dir" / "springsettings.cfg").write_text(
            "Fullscreen=0\n"
            "WindowBorderless=0\n"
            "XResolutionWindowed=1280\n"
            "YResolutionWindowed=720\n",
            encoding="utf-8",
        )
    data_dirs = [datadir, BASE_CONTENT]
    if ENGINE_INSTALL.is_dir():
        data_dirs.append(ENGINE_INSTALL)
    if SOURCE_CONTENT.is_dir():
        data_dirs.append(SOURCE_CONTENT)
    env["SPRING_DATADIR"] = os.pathsep.join(str(path) for path in data_dirs)
    env["SPRING_ISOLATED"] = str(datadir)
    env["SPRING_NATIVE_PARITY_OUTPUT_DIR"] = str(result_dir)
    if wasm_transport == "core" and run_mode in {"wasm", "wasm_guest"}:
        env["SPRING_WASM_CORE_HOST"] = "1"
    if needs_graphics and LSAN_SUPPRESSION.is_file():
        # Keep LeakSanitizer enabled for ASAN rendering runs while ignoring
        # only the documented third-party/process-lifetime shutdown stacks.
        # Preserve a caller-provided suppression file and all other options.
        lsan_options = env.get("LSAN_OPTIONS", "")
        if "suppressions=" not in lsan_options:
            suppression_option = f"suppressions={LSAN_SUPPRESSION}"
            env["LSAN_OPTIONS"] = (
                f"{lsan_options}:{suppression_option}" if lsan_options else suppression_option
            )

    if run_mode == "native" or (run_mode == "lua" and args.load_native_module_for_lua):
        env["SPRING_NATIVE_MODULE"] = str(NATIVE_SO)
    else:
        env["SPRING_NATIVE_MODULE"] = ""

    if run_mode == "native":
        # Keep executing widget tests in the native process, but do not ask a
        # full-read native event client to prove LuaUI's restricted visibility
        # contract.  The report names this role boundary; the UI Wasm run is
        # the corresponding restricted-read parity gate.
        automatic_skip = sorted(native_comparison_exclusions())
        configured_skip = env.get("SPRING_NATIVE_PARITY_SKIP", "")
        skip_values = [value for value in (configured_skip, *automatic_skip) if value]
        if skip_values:
            env["SPRING_NATIVE_PARITY_SKIP"] = ",".join(skip_values)

    spring_binary = args.spring if needs_graphics else args.spring_headless
    cmd = [
        str(spring_binary),
        "--nocolor",
        "--write-dir",
        str(output_dir / "write-dir"),
        str(script),
    ]
    if needs_graphics:
        # Unsynced/Gaia-unsynced callins are driven by the normal graphics
        # update loop.  This is still an unattended harness run: no SDL
        # events are injected and the fixture quits itself at its completion
        # frame.  The window is isolated and windowed so it cannot take over
        # the user's desktop or switch the physical monitor into exclusive
        # mode.
        cmd.insert(1, "--window")

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


def compare(lua_dir: Path, native_dir: Path, compare_callins: bool = True) -> bool:
    return compare_details(lua_dir, native_dir, compare_callins)["ok"]


def compare_details(lua_dir: Path, native_dir: Path, compare_callins: bool = True) -> dict:
    ok = True
    streams = []
    for name in RESULT_STREAMS:
        lua_rows = load_jsonl(lua_dir / name)
        native_rows = load_jsonl(native_dir / name)
        comparable_lua_rows = [row for row in lua_rows if native_comparable_row(row)]
        comparable_native_rows = [row for row in native_rows if native_comparable_row(row)]
        matches = comparable_rows(comparable_lua_rows) == comparable_rows(comparable_native_rows)
        streams.append({
            "name": name,
            "lua_rows": len(lua_rows),
            "native_rows": len(native_rows),
            "excluded_rows": len(lua_rows) - len(comparable_lua_rows),
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

    # The Lua baseline is the authoritative Lua-side observation.  In the
    # native run, a native event client can consume an event before Lua sees
    # it, so comparing native-run Lua rows against native rows makes valid
    # native ownership look like missing Lua coverage.  Compare the two
    # independent consumers instead: baseline Lua versus native-run native.
    lua_callin_rows = load_jsonl(lua_dir / CALLIN_LUA_STREAM)
    native_callin_rows = load_jsonl(native_dir / CALLIN_NATIVE_STREAM)
    callin_trace = compare_callin_traces(lua_callin_rows, native_callin_rows)
    if not callin_trace["matches"] and compare_callins:
        print("mismatch: engine callin Lua/native trace")
        if not callin_trace["coverage_matches"]:
            print(
                "  callin coverage: "
                f"lua={callin_trace['lua_covered_count']}/{callin_trace['expected_count']}, "
                f"native={callin_trace['native_covered_count']}/{callin_trace['expected_count']}"
            )
            for name in callin_trace["missing_lua_names"]:
                print(f"  missing Lua fixture coverage: {name}")
            for name in callin_trace["missing_native_names"]:
                print(f"  missing native fixture coverage: {name}")
        if not callin_trace["driver_markers_seen"]:
            print("  deterministic callin driver markers were not found on both sides")
        if not callin_trace["driver_coverage_matches"]:
            print(
                "  deterministic driver coverage: "
                f"lua={callin_trace['driver_lua_rows']}, "
                f"native={callin_trace['driver_native_rows']}"
            )
            for name in callin_trace["driver_lua_missing_names"]:
                print(f"  missing Lua driver callback: {name}")
            for name in callin_trace["driver_native_missing_names"]:
                print(f"  missing native driver callback: {name}")
        for row in callin_trace.get("missing_arguments", [])[:10]:
            print(f"  missing native equivalent: {json.dumps(row, sort_keys=True)}")
        for row in callin_trace.get("unexpected_arguments", [])[:10]:
            print(f"  unexpected native event: {json.dumps(row, sort_keys=True)}")
        for row in callin_trace.get("argument_mismatches", [])[:10]:
            print(f"  argument mismatch: {json.dumps(row, sort_keys=True)}")
        for row in callin_trace.get("result_mismatches", [])[:10]:
            print(f"  result mismatch: {json.dumps(row, sort_keys=True)}")
        for row in callin_trace.get("missing_results", [])[:10]:
            print(f"  missing result trace: {json.dumps(row, sort_keys=True)}")
        ok = False
    callin_trace["comparison_skipped"] = not compare_callins

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


def compare_wasm_details(
    lua_dir: Path,
    wasm_dir: Path,
    wasm_context: str = "synced_gadget",
    args: argparse.Namespace | None = None,
) -> dict:
    """Compare the Component Model fixture's host observation to Lua.

    The fixture deliberately uses the same synced query on both sides: Lua
    records the reference value, while the Wasm component obtains it through
    the generated units-query import and reports it through the normal
    LuaRules message boundary.  The stable semantic fields include explicit
    floating-point edge-case and deterministic RNG signatures; the player ID
    is an engine transport detail.
    """
    lua_rows = load_jsonl(lua_dir / WASM_PARITY_STREAM)
    wasm_rows = load_jsonl(wasm_dir / WASM_PARITY_STREAM)

    def semantic(rows: list[dict], source: str) -> list[dict]:
        return sorted(
            [
                {
                    "source": source,
                    "frame": row.get("frame"),
                    "teamUnitCount": row.get("teamUnitCount"),
                    "fpEdgeSignature": row.get("fpEdgeSignature"),
                    "rngSignature": row.get("rngSignature"),
                }
                for row in rows
                if row.get("source") == source
            ],
            key=lambda row: json.dumps(row, sort_keys=True),
        )

    expected = semantic(lua_rows, "lua")
    observed = semantic(wasm_rows, "wasm")
    reference_in_wasm = semantic(wasm_rows, "lua")
    expected_observation = [{**row, "source": "wasm"} for row in expected]
    deterministic_ok = bool(expected) and expected == reference_in_wasm and expected_observation == observed
    if not expected:
        print(f"missing Lua Wasm-parity reference rows: {lua_dir / WASM_PARITY_STREAM}")
    if not observed:
        print(f"missing Wasm observation rows: {wasm_dir / WASM_PARITY_STREAM}")
    if expected != reference_in_wasm:
        print("Wasm run did not reproduce the Lua reference rows")
    if expected_observation != observed:
        print("Wasm host observation does not match the Lua reference")

    manifest_path = (
        WASM_PROBE_MANIFEST
        if args is None or args.wasm_transport == "component"
        else WASM_CONTEXT_CORE_MANIFESTS[wasm_context]
    )
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    manifest_ids = list(manifest["tests"])
    manifest_selected_ids = set(manifest_ids)
    expected_ids = set(manifest_selected_ids)
    filtered_comparison = False
    unknown_requested_ids: set[str] = set()
    if args is not None and args.tests:
        requested_ids = {
            value.strip() for value in args.tests.split(",") if value.strip()
        }
        unknown_requested_ids = requested_ids - expected_ids
        expected_ids &= requested_ids
        filtered_comparison = True
    elif args is not None and args.test_prefix is not None:
        expected_ids = set(manifest_ids[: max(args.test_prefix, 0)])
        filtered_comparison = True
    if unknown_requested_ids:
        print(
            "requested Wasm probe tests are not in the selected manifest: "
            + ", ".join(sorted(unknown_requested_ids))
        )
    coverage = manifest.get("coverage", {})
    if not isinstance(coverage, dict):
        coverage = {}
    source_context = manifest.get("source_context")
    source_ids = {
        test["id"] for test in API_TESTS if test.get("context") == source_context
    }
    selected_ids = coverage.get("selected_ids", [])
    excluded_by_reason = coverage.get("excluded", {})
    excluded_ids = [test_id for ids in excluded_by_reason.values() for test_id in ids]
    covered_ids = set(selected_ids) | set(excluded_ids)
    coverage_ok = (
        manifest.get("version", 0) >= 2
        and coverage.get("source_test_count") == len(source_ids)
        and coverage.get("selected_count") == len(manifest_selected_ids)
        and set(selected_ids) == manifest_selected_ids
        and len(selected_ids) == len(set(selected_ids))
        and len(excluded_ids) == len(set(excluded_ids))
        and covered_ids == source_ids
        and not (set(selected_ids) & set(excluded_ids))
    )
    if not coverage_ok:
        print(
            "Wasm probe manifest does not account for the complete canonical "
            f"{source_context} selection: "
            f"source={len(source_ids)}, selected={len(selected_ids)}, "
            f"excluded={len(excluded_ids)}"
        )
    lua_api_rows = {
        str(row.get("testName")): row
        for row in lua_rows
        if row.get("source") == "lua-api" and row.get("testName")
    }
    wasm_api_rows = {
        str(row.get("testName")): row
        for row in wasm_rows
        if row.get("source") == "wasm-api" and row.get("testName")
    }

    def normalize_wasm_api_row(test_id: str, row: dict) -> dict:
        if wasm_context not in {"gaia_synced", "gaia_unsynced"} or test_id != "get_map_options":
            return row
        fields = row.get("fields")
        if not isinstance(fields, dict) or not isinstance(fields.get("keys"), list):
            return row
        normalized_fields = dict(fields)
        normalized_fields["keys"] = [
            key for key in fields["keys"] if key not in GAIA_HARNESS_MAP_OPTION_KEYS
        ]
        normalized_row = dict(row)
        normalized_row["fields"] = normalized_fields
        return normalized_row

    wasm_status_rows = [
        row for row in wasm_rows if row.get("source") == "wasm-status"
    ]
    fixture_status_rows = [
        row for row in wasm_status_rows if row.get("testName") == "fixture"
    ]
    fixture_status_ok = (
        len(fixture_status_rows) == 1
        and fixture_status_rows[0].get("status") == "pass"
    )
    if not fixture_status_ok:
        print(
            "Wasm probe did not report exactly one successful real-fixture discovery: "
            f"{json.dumps(fixture_status_rows, sort_keys=True)}"
        )

    def values_match(test_id: str, expected_value, actual_value) -> bool:
        if isinstance(expected_value, bool) or isinstance(actual_value, bool):
            return expected_value == actual_value
        if isinstance(expected_value, (int, float)) and isinstance(actual_value, (int, float)):
            test = API_TEST_BY_ID.get(test_id, {})
            epsilon = float(test.get("compare", {}).get("epsilon", 0.00001))
            return math.isclose(float(expected_value), float(actual_value), rel_tol=0.0, abs_tol=epsilon)
        return expected_value == actual_value

    api_missing_lua = sorted(expected_ids - set(lua_api_rows))
    api_missing_wasm = sorted(expected_ids - set(wasm_api_rows))
    api_unexpected_lua = (
        [] if filtered_comparison else sorted(set(lua_api_rows) - expected_ids)
    )
    api_unexpected_wasm = (
        [] if filtered_comparison else sorted(set(wasm_api_rows) - expected_ids)
    )
    api_mismatches = []
    api_error_matches = []
    for test_id in sorted(expected_ids & set(lua_api_rows) & set(wasm_api_rows)):
        expected_row = lua_api_rows[test_id]
        actual_row = normalize_wasm_api_row(test_id, wasm_api_rows[test_id])
        if expected_row.get("status") != "pass" or actual_row.get("status") != "pass":
            test = API_TEST_BY_ID.get(test_id, {})
            if (
                test.get("wasm_expected_error")
                and expected_row.get("status") in {"error", "fail"}
                and actual_row.get("status") == "error"
            ):
                api_error_matches.append(test_id)
                continue
            api_mismatches.append({"testName": test_id, "expected": expected_row, "actual": actual_row})
            continue
        expected_fields = expected_row.get("fields", {})
        actual_fields = actual_row.get("fields", {})
        order_insensitive_fields = set(
            API_TEST_BY_ID.get(test_id, {}).get("order_insensitive_fields", [])
        )

        def comparable_field(field: str, value):
            if field in order_insensitive_fields and isinstance(value, list):
                return sorted(value, key=lambda item: json.dumps(item, sort_keys=True))
            test = API_TEST_BY_ID.get(test_id, {})
            compare = test.get("compare", {})
            if compare.get("stream") == "shape":
                if isinstance(value, bool):
                    return "boolean"
                if isinstance(value, (int, float)):
                    return "number"
                if value is None:
                    return "nil"
                return type(value).__name__
            return value

        if set(expected_fields) != set(actual_fields) or any(
            not values_match(
                test_id,
                comparable_field(field, expected_fields.get(field)),
                comparable_field(field, actual_fields.get(field)),
            )
            for field in set(expected_fields) | set(actual_fields)
        ):
            api_mismatches.append({"testName": test_id, "expected": expected_row, "actual": actual_row})

    accepted_error_ids = set(api_error_matches)
    vacuous_ids = sorted(
        test_id
        for test_id in expected_ids
        for row in [wasm_api_rows.get(test_id, {})]
        if row.get("status") != "pass" and test_id not in accepted_error_ids
    )
    vacuous_fraction = len(vacuous_ids) / max(len(expected_ids), 1)
    vacuity_ok = vacuous_fraction <= WASM_VACUITY_THRESHOLD
    if not vacuity_ok:
        print(
            "Wasm probe vacuity exceeds threshold: "
            f"{len(vacuous_ids)}/{len(expected_ids)} = {vacuous_fraction:.2%} "
            f"> {WASM_VACUITY_THRESHOLD:.2%}"
        )
        for test_id in vacuous_ids[:20]:
            print(f"  vacuous Wasm result: {test_id}")

    api_ok = coverage_ok and fixture_status_ok and vacuity_ok and not (
        api_missing_lua
        or api_missing_wasm
        or api_unexpected_lua
        or api_unexpected_wasm
        or api_mismatches
    )
    ok = deterministic_ok and api_ok and not unknown_requested_ids
    if not api_ok:
        print(
            "Wasm typed API probe mismatch: "
            f"expected={len(expected_ids)}, lua={len(lua_api_rows)}, wasm={len(wasm_api_rows)}"
        )
        for test_id in api_missing_lua:
            print(f"  missing Lua probe reference: {test_id}")
        for test_id in api_missing_wasm:
            print(f"  missing Wasm probe result: {test_id}")
        for mismatch in api_mismatches[:10]:
            print(f"  probe mismatch: {json.dumps(mismatch, sort_keys=True)}")

    return {
        "kind": "wasm",
        "wasm_context": wasm_context,
        "ok": ok,
        "streams": [
            {
                "name": WASM_PARITY_STREAM,
                "lua_rows": len(lua_rows),
                "native_rows": len(wasm_rows),
                "matches": ok,
            }
        ],
        "lua_rows": [],
        "native_rows": [],
        "native_failures": [],
        "native_complete": True,
        "callin_trace": None,
        "expected": expected,
        "observed": observed,
        "deterministic_ok": deterministic_ok,
        "api_expected_count": len(expected_ids),
        "api_lua_count": len(lua_api_rows),
        "api_wasm_count": len(wasm_api_rows),
        "api_ok": api_ok,
        "coverage_ok": coverage_ok,
        "coverage_source_count": len(source_ids),
        "coverage_selected_count": len(selected_ids),
        "coverage_excluded_count": len(excluded_ids),
        "coverage_excluded": excluded_by_reason,
        "fixture_status_ok": fixture_status_ok,
        "fixture_status_rows": fixture_status_rows,
        "vacuity_threshold": WASM_VACUITY_THRESHOLD,
        "vacuous_count": len(vacuous_ids),
        "vacuous_fraction": vacuous_fraction,
        "vacuous_ids": vacuous_ids,
        "vacuity_ok": vacuity_ok,
        "api_error_matches": api_error_matches,
        "api_missing_lua": api_missing_lua,
        "api_missing_wasm": api_missing_wasm,
        "api_unexpected_lua": api_unexpected_lua,
        "api_unexpected_wasm": api_unexpected_wasm,
        "api_mismatches": api_mismatches,
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
    """Compare shared engine-to-Lua/native callins, including argument values."""
    general_names, synced_names, unsynced_names = documented_callin_names()
    synced_general_names, unsynced_general_names = general_callin_contexts(general_names)
    gadget_router_names = gadget_router_callins()
    documented_names = general_names | synced_names | unsynced_names
    expected_names = (
        documented_names & native_event_callin_names()
    ) - LIFECYCLE_ONLY_CALLINS

    def before_complete(rows: list[dict]) -> tuple[list[dict], bool]:
        for index, row in enumerate(rows):
            if row.get("context") == "callin_phase" and row.get("name") == "complete":
                return rows[:index], True
        return rows, False

    lua_rows, lua_phase_seen = before_complete(lua_rows)
    native_rows, native_phase_seen = before_complete(native_rows)

    def driver_window(rows: list[dict]) -> tuple[list[dict], bool]:
        """Return callbacks between the deterministic driver markers.

        The two Spring processes naturally produce different background
        callbacks (render frames, console messages, and shutdown activity).
        The parity driver brackets one deterministic sequence with unique
        GameFrame integer sentinels.  Console callins are deliberately not
        used as delimiters because parity result logging also emits them and
        the engine may decorate or coalesce console messages.
        """
        start_value = -1001
        end_value = -1002
        start = next(
            (
                index
                for index, row in enumerate(rows)
                if row.get("name") == "GameFrame"
                and (row.get("args") or [None])[0] == start_value
            ),
            None,
        )
        if start is None:
            return [], False
        end = next(
            (
                index
                for index, row in enumerate(rows[start + 1 :], start=start + 1)
                if row.get("name") == "GameFrame"
                and (row.get("args") or [None])[0] == end_value
            ),
            None,
        )
        if end is None:
            return [], False
        return rows[start + 1 : end], True

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
        if name in synced_names:
            return context == "synced_gadget"
        if name in general_names:
            if name in LUA_RULES_ONLY_CALLINS:
                expected_context = (
                    "synced_gadget" if name == "Load" else "unsynced_gadget"
                )
            elif name in LUA_UI_CONTROL_CALLINS:
                expected_context = "lua_ui"
            elif name in LUA_UI_NATIVE_SIGNATURE_CALLINS:
                expected_context = "lua_ui"
            elif name in gadget_router_names:
                # Prefer the stock LuaRules dispatcher whenever it actually
                # exposes the callback.  This preserves MouseMove/
                # MouseRelease and other input callins that LuaUI may not see
                # when a gadget owns the event.
                expected_context = (
                    "unsynced_gadget"
                    if name in unsynced_general_names
                    else "synced_gadget"
                )
            elif name in unsynced_general_names:
                # The unsynced LuaRules router has no implementation for
                # every documented callback; the direct LuaUI handle is the
                # canonical public surface for those remaining callbacks.
                expected_context = "lua_ui"
            else:
                # Keep documented callbacks whose stock gadget router has no
                # dispatch method testable through CLuaUI.
                expected_context = "lua_ui"
            return context == expected_context
        if name in synced_general_names:
            return context == "synced_gadget"
        return context == "lua_ui"

    selected_lua_rows = [
        row for row in lua_rows if row.get("name") and selected_lua_row(row)
    ]
    selected_native_rows = [row for row in native_rows if row.get("name")]
    lua_counts = Counter(str(row.get("name")) for row in selected_lua_rows)
    native_counts = Counter(str(row.get("name")) for row in selected_native_rows)
    lua_covered_names = set(lua_counts) & expected_names
    native_covered_names = set(native_counts) & expected_names
    missing_lua_names = sorted(expected_names - lua_covered_names)
    missing_native_names = sorted(expected_names - native_covered_names)
    unexpected_lua_names = sorted(set(lua_counts) - expected_names)
    unexpected_native_names = sorted(set(native_counts) - expected_names)
    coverage_matches = not missing_lua_names and not missing_native_names

    def canonicalize_value(value):
        # Lua's JSON writer prints an integer-valued number without a decimal
        # point, while serde_json may retain the native f32 as `123.0`.
        # They are the same Lua number and must compare numerically.
        if isinstance(value, bool):
            return value
        if isinstance(value, float) and value.is_integer():
            return int(value)
        if isinstance(value, list):
            return [canonicalize_value(item) for item in value]
        if isinstance(value, dict):
            return {key: canonicalize_value(item) for key, item in value.items()}
        return value

    def semantic_row(row: dict) -> dict:
        args = row.get("args", [])
        # Update is driven by the unsynchronised render clock.  Its contract is
        # a numeric delta, while the exact value is necessarily different when
        # the Lua baseline and native process are scheduled independently.
        if row.get("name") == "Update":
            args = ["number"]
        return {
            "name": str(row.get("name")),
            "arity": len(args) if isinstance(args, list) else row.get("arity", 0),
            "args": canonicalize_value(args),
            "resultArity": (
                len(row.get("results", []))
                if isinstance(row.get("results", []), list)
                else row.get("resultArity", 0)
            ),
            "results": canonicalize_value(row.get("results", [])),
            "result_trace_present": "results" in row and "resultArity" in row,
        }

    def values_equal(left, right) -> bool:
        if isinstance(left, bool) or isinstance(right, bool):
            return left == right
        if isinstance(left, (int, float)) and isinstance(right, (int, float)):
            # Lua's float-number build and the native trace serialize the same
            # f32 with slightly different decimal spellings.  A relative
            # tolerance of one f32 ulp at the observed magnitude keeps that
            # representation noise out without hiding unit/pixel changes.
            return math.isclose(float(left), float(right), rel_tol=1e-6, abs_tol=1e-5)
        if type(left) is not type(right):
            return False
        if isinstance(left, list):
            return len(left) == len(right) and all(
                values_equal(item_left, item_right)
                for item_left, item_right in zip(left, right)
            )
        if isinstance(left, dict):
            return left.keys() == right.keys() and all(
                values_equal(left[key], right[key]) for key in left
            )
        return left == right

    lua_driver_rows, lua_driver_markers = driver_window(lua_rows)
    native_driver_rows, native_driver_markers = driver_window(native_rows)
    selected_lua_driver_rows = [
        row for row in lua_driver_rows if row.get("name") and selected_lua_row(row)
    ]
    selected_native_driver_rows = [row for row in native_driver_rows if row.get("name")]

    def driver_rows_once(rows: list[dict]) -> tuple[list[dict], dict[str, int]]:
        """Collapse only the known IsAbove/GetTooltip re-entrant callback."""
        counts = Counter(str(row.get("name")) for row in rows)
        result = []
        seen: set[str] = set()
        for row in rows:
            name = str(row.get("name"))
            if name == "IsAbove" and name in seen:
                continue
            result.append(row)
            seen.add(name)
        return result, {name: count for name, count in counts.items() if count > 1}

    selected_lua_driver_rows, lua_driver_duplicates = driver_rows_once(
        selected_lua_driver_rows
    )
    selected_native_driver_rows, native_driver_duplicates = driver_rows_once(
        selected_native_driver_rows
    )
    lua_driver_semantic = [semantic_row(row) for row in selected_lua_driver_rows]
    native_driver_semantic = [semantic_row(row) for row in selected_native_driver_rows]
    lua_driver_names = {row["name"] for row in lua_driver_semantic}
    native_driver_names = {row["name"] for row in native_driver_semantic}
    lua_driver_missing = sorted(expected_names - lua_driver_names)
    native_driver_missing = sorted(expected_names - native_driver_names)
    driver_coverage_matches = not lua_driver_missing and not native_driver_missing

    lua_by_name = {row["name"]: row for row in lua_driver_semantic}
    native_by_name = {row["name"]: row for row in native_driver_semantic}
    missing_arguments = []
    unexpected_arguments = []
    argument_mismatches = []
    result_mismatches = []
    missing_results = []
    unexpected_results = []
    for name in sorted(expected_names):
        lua_row = lua_by_name.get(name)
        native_row = native_by_name.get(name)
        if lua_row is None:
            missing_arguments.append({"name": name})
            continue
        if native_row is None:
            unexpected_arguments.append({"name": name})
            continue
        if lua_row["arity"] != native_row["arity"] or not values_equal(
            lua_row["args"], native_row["args"]
        ):
            argument_mismatches.append({"lua": lua_row, "native": native_row})
        if not lua_row["result_trace_present"]:
            missing_results.append({"side": "lua", "row": lua_row})
        if not native_row["result_trace_present"]:
            missing_results.append({"side": "native", "row": native_row})
        if (
            lua_row["resultArity"] != native_row["resultArity"]
            or not values_equal(lua_row["results"], native_row["results"])
        ):
            result_mismatches.append({"lua": lua_row, "native": native_row})

    unexpected_driver_names = sorted(
        (lua_driver_names | native_driver_names) - expected_names
    )
    argument_matches = (
        not missing_arguments
        and not unexpected_arguments
        and not argument_mismatches
    )
    result_trace_complete = not missing_results
    result_matches = not result_mismatches and result_trace_complete
    driver_process_complete = (
        lua_driver_markers
        and native_driver_markers
        and driver_coverage_matches
        and not unexpected_driver_names
        and set(lua_driver_duplicates) <= {"IsAbove"}
        and not native_driver_duplicates
    )
    return {
        # A value-level match over only the callbacks that happened to fire is
        # not full parity.  Keep coverage in the gate so a green report cannot
        # silently mean “48 of 150 callbacks matched”.
        "matches": (
            driver_process_complete
            and argument_matches
            and result_matches
            and lua_phase_seen
            and native_phase_seen
            and coverage_matches
        ),
        "argument_matches": argument_matches,
        "result_matches": result_matches,
        "result_trace_complete": result_trace_complete,
        "driver_markers_seen": lua_driver_markers and native_driver_markers,
        "driver_coverage_matches": driver_coverage_matches,
        "driver_lua_rows": len(selected_lua_driver_rows),
        "driver_native_rows": len(selected_native_driver_rows),
        "driver_lua_missing_names": lua_driver_missing,
        "driver_native_missing_names": native_driver_missing,
        "driver_unexpected_names": unexpected_driver_names,
        "lua_driver_duplicates": lua_driver_duplicates,
        "native_driver_duplicates": native_driver_duplicates,
        "coverage_matches": coverage_matches,
        "expected_count": len(expected_names),
        "expected_names": sorted(expected_names),
        "lua_covered_count": len(lua_covered_names),
        "native_covered_count": len(native_covered_names),
        "lua_covered_names": sorted(lua_covered_names),
        "native_covered_names": sorted(native_covered_names),
        "missing_lua_names": missing_lua_names,
        "missing_native_names": missing_native_names,
        "unexpected_lua_names": unexpected_lua_names,
        "unexpected_native_names": unexpected_native_names,
        "lua_rows": len(lua_rows),
        "native_rows": len(native_rows),
        "lua_selected_rows": len(selected_lua_rows),
        "native_selected_rows": len(selected_native_rows),
        "lua_phase_seen": lua_phase_seen,
        "native_phase_seen": native_phase_seen,
        "lua_counts": dict(lua_counts),
        "native_counts": dict(native_counts),
        "missing_arguments": missing_arguments,
        "unexpected_arguments": unexpected_arguments,
        "argument_mismatches": argument_mismatches,
        "result_mismatches": result_mismatches,
        "missing_results": missing_results,
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


LUA_DOCUMENTED_SURFACE_NAMESPACES = (
    "Global", "Spring", "RmlUi", "gl", "VFS", "Script",
    "Encoding", "math", "debug", "table",
)


def read_lua_surface_functions() -> dict[str, set[str]]:
    """Read documented callouts by their real Lua namespace."""
    surfaces = {namespace: set() for namespace in LUA_DOCUMENTED_SURFACE_NAMESPACES}
    if not LUA_API_DOC.exists():
        return surfaces
    text = LUA_API_DOC.read_text(encoding="utf-8", errors="replace")
    pattern = re.compile(
        r"^- `((Global|Spring|RmlUi|gl|VFS|Script|Encoding|math|debug|table)\.[A-Za-z_][A-Za-z0-9_.]*)`",
        re.MULTILINE,
    )
    for full_name, namespace in pattern.findall(text):
        surfaces[namespace].add(full_name)
    return surfaces


def read_rust_api_functions() -> set[str]:
    if not RUST_API_DOC.exists():
        return set()
    text = RUST_API_DOC.read_text(encoding="utf-8", errors="replace")
    return set(re.findall(r"^- `([A-Za-z_][A-Za-z0-9_]*\.[A-Za-z_][A-Za-z0-9_]*)`", text, re.MULTILINE))


def read_native_only_functions() -> set[str]:
    """Read the source-backed Rust-only classifications from the audit."""
    if not API_SURFACE_AUDIT_DOC.exists():
        return set()
    native_only = set()
    category = ""
    for line in API_SURFACE_AUDIT_DOC.read_text(encoding="utf-8", errors="replace").splitlines():
        if line.startswith("### "):
            category = line[4:].split(" (", 1)[0]
            continue
        if category.startswith("native-only ") and line.startswith("- `"):
            native_only.add(line[3:].split("`", 1)[0])
    return native_only


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


def surface_test_ids_from_names(names: set[str]) -> set[str]:
    """Return surface checks whose explicit recorder row was observed."""
    return {test_id for test_id in SURFACE_TEST_BY_ID if test_id in names}


def tested_lua_functions(
    recorded_ids: set[str] | None = None,
    surface_recorded_ids: set[str] | None = None,
) -> set[str]:
    """Return recorded Spring.* callouts, including explicit surface checks."""
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
    for test_id, test in SURFACE_TEST_BY_ID.items():
        if surface_recorded_ids is not None:
            if test_id not in surface_recorded_ids:
                continue
        elif recorded_ids is not None and test_id not in recorded_ids:
            continue
        for name in test.get("lua", []):
            match = re.fullmatch(r"Spring\.([A-Za-z_][A-Za-z0-9_]*)", str(name))
            if match:
                functions.add(match.group(1))
    return functions


def tested_lua_surface_functions(
    recorded_ids: set[str] | None = None,
    surface_recorded_ids: set[str] | None = None,
) -> set[str]:
    """Return qualified Lua surface labels referenced by recorded checks."""
    functions = set()
    items = CHECK_COVERAGE.items()
    if recorded_ids is not None:
        items = ((name, coverage) for name, coverage in items if name in recorded_ids)
    for _, coverage in items:
        for key in ("lua_setter", "lua_getter"):
            for name in markdown_api_names(coverage.get(key, "")):
                if any(name.startswith(f"{namespace}.") for namespace in LUA_DOCUMENTED_SURFACE_NAMESPACES):
                    functions.add(name)
    for test_id, test in SURFACE_TEST_BY_ID.items():
        if surface_recorded_ids is not None:
            if test_id not in surface_recorded_ids:
                continue
        elif recorded_ids is not None and test_id not in recorded_ids:
            continue
        functions.update(
            name for name in test.get("lua", [])
            if any(name.startswith(f"{namespace}.") for namespace in LUA_DOCUMENTED_SURFACE_NAMESPACES)
        )
    return functions


def tested_rust_functions(
    recorded_ids: set[str] | None = None,
    surface_recorded_ids: set[str] | None = None,
) -> set[str]:
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
    for test_id, test in SURFACE_TEST_BY_ID.items():
        if surface_recorded_ids is not None:
            if test_id not in surface_recorded_ids:
                continue
        elif recorded_ids is not None and test_id not in recorded_ids:
            continue
        functions.update(test.get("native", []))
    return functions


def lua_api_labels(coverage: dict) -> list[str]:
    labels = []
    for key in ("lua_setter", "lua_getter"):
        for name in markdown_api_names(coverage.get(key, "")):
            if any(
                re.fullmatch(rf"{namespace}\.[A-Za-z_][A-Za-z0-9_.]*", name)
                for namespace in LUA_DOCUMENTED_SURFACE_NAMESPACES
            ):
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


def coverage_summary(
    checked_names: set[str] | None = None,
    surface_recorded_ids: set[str] | None = None,
) -> dict:
    lua_total = read_lua_api_functions() | known_issue_lua_functions("inventory_missing_docstring")
    lua_surfaces = read_lua_surface_functions()
    lua_surface_total = set().union(*lua_surfaces.values())
    rust_total = read_rust_api_functions()
    rust_native_only = rust_total & read_native_only_functions()
    recorded_ids = recorded_test_ids(checked_names) if checked_names is not None else None
    lua_tested = tested_lua_functions(recorded_ids, surface_recorded_ids)
    lua_surface_tested = tested_lua_surface_functions(recorded_ids, surface_recorded_ids)
    rust_tested = tested_rust_functions(recorded_ids, surface_recorded_ids)
    return {
        "lua_total": lua_total,
        "lua_tested": lua_tested,
        "lua_tested_known": lua_tested & lua_total,
        "lua_tested_unknown": lua_tested - lua_total,
        "lua_untested": lua_total - lua_tested,
        "lua_surfaces": lua_surfaces,
        "lua_surface_total": lua_surface_total,
        "lua_surface_tested": lua_surface_tested,
        "lua_surface_tested_known": lua_surface_tested & lua_surface_total,
        "lua_surface_tested_unknown": lua_surface_tested - lua_surface_total,
        "lua_surface_untested": lua_surface_total - lua_surface_tested,
        "rust_total": rust_total,
        "rust_tested": rust_tested,
        "rust_tested_known": rust_tested & rust_total,
        "rust_tested_unknown": rust_tested - rust_total,
        "rust_untested": rust_total - rust_tested,
        "rust_native_only": rust_native_only,
        "rust_native_only_tested": rust_tested & rust_native_only,
        "rust_native_only_untested": rust_native_only - rust_tested,
        "rust_counterpart": rust_total - rust_native_only,
        "rust_counterpart_tested": rust_tested & (rust_total - rust_native_only),
        "rust_counterpart_untested": (rust_total - rust_native_only) - rust_tested,
        "surface_tested_ids": surface_recorded_ids or set(),
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


def read_surface_test_ids(base_output: Path) -> set[str]:
    """Require both process Lua observations, plus native validation when matched."""
    def run_names(run_name: str) -> set[str]:
        run_dir = base_output / run_name
        names = set()
        for stream_name in RESULT_STREAMS:
            names.update(row_test_name(row) for row in load_jsonl(run_dir / stream_name))
        return names

    lua_names = run_names("lua")
    native_lua_names = run_names("native")
    native_names = {
        row_test_name(row)
        for row in load_jsonl(base_output / "native" / "native.jsonl")
    }
    recorded = set()
    for test_id, test in SURFACE_TEST_BY_ID.items():
        if test_id not in lua_names or test_id not in native_lua_names:
            continue
        if test.get("mode") == "lua_only" or test_id in native_names:
            recorded.add(test_id)
    return recorded


def requested_surface_test_ids(args: argparse.Namespace) -> set[str]:
    """Return explicitly selected surface tests that must be observed."""
    if not args.tests:
        return set()
    requested = {value.strip() for value in args.tests.split(",") if value.strip()}
    return requested & set(SURFACE_TEST_BY_ID)


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
        (
            f"| Native Rust counterparts | {len(summary['rust_counterpart'])} | "
            f"{len(summary['rust_counterpart_tested'])} | "
            f"{pct(len(summary['rust_counterpart_tested']), len(summary['rust_counterpart']))} | 0 |"
        ),
        (
            f"| Native-only Rust surfaces | {len(summary['rust_native_only'])} | "
            f"{len(summary['rust_native_only_tested'])} | "
            f"{pct(len(summary['rust_native_only_tested']), len(summary['rust_native_only']))} | 0 |"
        ),
    ]
    for namespace in ("Global", "RmlUi", "gl", "VFS", "Script", "Encoding", "math", "debug", "table"):
        total = summary["lua_surfaces"].get(namespace, set())
        tested = summary["lua_surface_tested_known"] & total
        unknown = summary["lua_surface_tested_unknown"] & summary["lua_surface_tested"]
        unknown = {name for name in unknown if name.startswith(f"{namespace}.")}
        lines.append(
            f"| Lua `{namespace}.*` | {len(total)} | {len(tested)} | "
            f"{pct(len(tested), len(total))} | {len(unknown)} |"
        )
    lines.extend([
        "",
        "## Context Summary",
        "",
        "| Context | Runtime Spring APIs | Spec Checks | Recorded Checks | Lua APIs Tested / Runtime | Known Mismatch Issues | Affected Runtime APIs | Native APIs Recorded |",
        "| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |",
    ])
    for row in context_coverage(checked_names, inventory, recorded_by_context):
        lines.append(
            f"| `{row['context']}` | {row['runtime_lua']} | {row['spec_checks']} | {row['recorded_checks']} | "
            f"{row['lua_recorded_runtime']} / {row['runtime_lua']} | "
            f"{row['known_mismatch_count']} | {row['known_mismatch_lua']} | "
            f"{row['native_recorded']} |"
        )

    lines.extend([
        "## Additional Lua-facing Surface Checks",
        "",
        "| Check | Context | Mode | Status | Lua APIs | Native APIs | Reason |",
        "| --- | --- | --- | --- | --- | --- | --- |",
    ])
    for test in SURFACE_TESTS:
        test_id = test["id"]
        lines.append(
            f"| `{test_id}` | `{test.get('context', 'unknown')}` | `{test.get('mode', 'matched')}` | "
            f"{'tested' if test_id in summary.get('surface_tested_ids', set()) else 'untested'} | "
            f"{', '.join(f'`{name}`' for name in test.get('lua', [])) or 'n/a'} | "
            f"{', '.join(f'`{name}`' for name in test.get('native', [])) or 'n/a'} | "
            f"{test.get('reason', '')} |"
        )

    lines.extend([
        "",
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

    if summary["lua_surface_untested"]:
        lines.extend(["", "## Untested Documented Lua Surface Functions", ""])
        lines.extend(f"- `{name}`" for name in sorted(summary["lua_surface_untested"]))

    lines.extend([
        "",
        "## Untested Native Functions",
        "",
    ])
    if summary["rust_untested"]:
        lines.extend(f"- `{name}`" for name in sorted(summary["rust_untested"]))
    else:
        lines.append("- none")

    lines.extend([
        "",
        "## Untested Native Counterparts",
        "",
    ])
    if summary["rust_counterpart_untested"]:
        lines.extend(f"- `{name}`" for name in sorted(summary["rust_counterpart_untested"]))
    else:
        lines.append("- none")

    lines.extend([
        "",
        "## Untested Native-only Surfaces",
        "",
    ])
    if summary["rust_native_only_untested"]:
        lines.extend(f"- `{name}`" for name in sorted(summary["rust_native_only_untested"]))
    else:
        lines.append("- none")

    if summary["rust_tested_unknown"]:
        lines.extend(["", "## Tested Native Names Missing From Inventory", ""])
        lines.extend(f"- `{name}`" for name in sorted(summary["rust_tested_unknown"]))

    detail_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return detail_path


def write_report(base_output: Path, args: argparse.Namespace, compare_info: dict | None) -> None:
    # --mode wasm uses two isolated in-engine harness runs.  That directory
    # contains source="lua-api" reference and source="wasm-api" observation
    # runs.  The two runs are separate because setter probes mutate engine
    # state and must not be applied twice to one world.
    lua_dir = base_output / ("wasm_reference" if args.mode == "wasm" else "lua")
    comparison_dir = base_output / ("wasm_guest" if args.mode == "wasm" else "native")
    surface_recorded_ids = read_surface_test_ids(base_output)
    required_surface_ids = requested_surface_test_ids(args)
    missing_required_surface_ids = sorted(required_surface_ids - surface_recorded_ids)
    case_count = read_report_option(lua_dir / "script.txt", "native_api_parity_cases") or "n/a"
    rendering_enabled = (
        read_report_option(lua_dir / "script.txt", "native_api_parity_enable_rendering_tests") == "1"
        or args.enable_rendering_tests
    )
    uses_graphics = rendering_enabled or args.wasm_context in {
        "unsynced_gadget",
        "gaia_unsynced",
    }
    lines = [
        "# Native API Parity Report",
        "",
        f"- Result: {'PASS' if (compare_info and compare_info['ok'] and not missing_required_surface_ids) else 'PARTIAL/UNVERIFIED'}",
        f"- Generated: {time.strftime('%Y-%m-%d %H:%M:%S %z')}",
        f"- Spring binary: {report_link(args.spring if uses_graphics else args.spring_headless)}",
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

    if args.mode == "wasm":
        run_sections = (
            ("Lua Wasm reference", lua_dir),
            ("Wasm guest", comparison_dir),
        )
    else:
        run_sections = (("Lua baseline", lua_dir), ("Native comparison", comparison_dir))
    for label, run_dir in run_sections:
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
        "| Stream | Lua Rows | Native Rows | Native-excluded | Match |",
        "| --- | ---: | ---: | ---: | --- |",
    ])

    if compare_info:
        for stream in compare_info["streams"]:
            lines.append(
                f"| `{stream['name']}` | {stream['lua_rows']} | {stream['native_rows']} | "
                f"{stream.get('excluded_rows', 'n/a')} | "
                f"{'yes' if stream['matches'] else 'NO'} |"
            )
    else:
        stream_names = [WASM_PARITY_STREAM] if args.mode == "wasm" else RESULT_STREAMS
        for name in stream_names:
            lines.append(f"| `{name}` | {len(load_jsonl(lua_dir / name))} | n/a | n/a | n/a |")

    if compare_info and compare_info.get("kind") != "wasm":
        lines.extend([
            "",
            "Widget-context rows are executed in both processes but excluded from the direct",
            "Lua/native comparison because the native event client is full-read; LuaUI↔Wasm",
            "uses the restricted visibility context and remains the UI parity comparison.",
        ])

    if compare_info and compare_info.get("kind") == "wasm":
        lines.extend([
            "",
            "## Wasm Typed API Probe",
            "",
            f"- Deterministic Component observation: `{'pass' if compare_info['deterministic_ok'] else 'FAIL'}`",
            f"- Generated {compare_info['wasm_context']} read-only probes: `{compare_info['api_expected_count']}`",
            f"- Lua reference rows: `{compare_info['api_lua_count']}`",
            f"- Wasm result rows: `{compare_info['api_wasm_count']}`",
            f"- Fixture discovery: `{'pass' if compare_info['fixture_status_ok'] else 'FAIL'}`",
            f"- Vacuous Wasm rows: `{compare_info['vacuous_count']}/{compare_info['api_expected_count']}` "
            f"({compare_info['vacuous_fraction']:.2%}; threshold {compare_info['vacuity_threshold']:.2%})",
            f"- Typed API result parity: `{'pass' if compare_info['api_ok'] else 'FAIL'}`",
        ])
        if compare_info["api_missing_lua"] or compare_info["api_missing_wasm"]:
            lines.extend([
                "",
                "### Missing typed probe rows",
                "",
                f"- Lua: `{', '.join(compare_info['api_missing_lua']) or 'none'}`",
                f"- Wasm: `{', '.join(compare_info['api_missing_wasm']) or 'none'}`",
            ])
        if compare_info["api_mismatches"]:
            lines.extend(["", "### Typed probe mismatches", ""])
            for mismatch in compare_info["api_mismatches"][:20]:
                lines.append(f"- `{mismatch['testName']}`: `{json.dumps(mismatch, sort_keys=True)}`")

    if compare_info and compare_info.get("kind") != "wasm":
        trace = compare_info["callin_trace"]
        baseline_lua_trace_rows = len(load_jsonl(lua_dir / CALLIN_LUA_STREAM))
        native_trace_rows = len(load_jsonl(comparison_dir / CALLIN_NATIVE_STREAM))
        lines.extend([
            "",
            "## Engine Callin Trace",
            "",
            f"- Lua trace rows: baseline `{baseline_lua_trace_rows}`",
            f"- Native trace rows: `{native_trace_rows}`",
            "- Process-wide callback counts: informational only (the two processes have different render/input lifetimes).",
            f"- Deterministic driver callbacks: Lua `{trace['driver_lua_rows']}`, native `{trace['driver_native_rows']}`, expected `{trace['expected_count']}`",
            f"- Comparison: `{'skipped by focused-run option' if trace.get('comparison_skipped') else 'enabled'}`",
            f"- Deterministic driver markers: `{'present' if trace['driver_markers_seen'] else 'MISSING'}`",
            f"- Shared argument values: `{'match' if trace['argument_matches'] else 'MISMATCH'}`",
            f"- Shared return values: `{'match' if trace['result_matches'] else 'MISMATCH'}`",
            f"- Return-value trace coverage: `{'complete' if trace['result_trace_complete'] else 'INCOMPLETE'}`",
            f"- Expected shared callins: `{trace['expected_count']}`",
            f"- Lua callins covered: `{trace['lua_covered_count']}/{trace['expected_count']}`",
            f"- Native callins covered: `{trace['native_covered_count']}/{trace['expected_count']}`",
            f"- Callin coverage: `{'complete' if trace['coverage_matches'] else 'INCOMPLETE'}`",
            "- `Update` delta values are checked as numeric because separate processes have independent render clocks.",
        ])
        if trace["lua_driver_duplicates"] or trace["native_driver_duplicates"]:
            lines.extend([
                "",
                "### Deterministic driver duplicate callbacks",
                "",
                f"- Lua: `{trace['lua_driver_duplicates'] or 'none'}`",
                f"- Native: `{trace['native_driver_duplicates'] or 'none'}`",
                "- `IsAbove` is allowed once extra because the `GetTooltip` fixture query re-enters it.",
            ])
        if not trace["driver_coverage_matches"]:
            lines.extend([
                "",
                "### Missing deterministic driver coverage",
                "",
                "The return-value gate remains unverified until every shared callback is",
                "triggered inside the deterministic marker window on both sides.",
                "",
                "| Callback | Lua driver | Native driver |",
                "| --- | --- | --- |",
            ])
            driver_missing_names = sorted(
                set(trace["driver_lua_missing_names"])
                | set(trace["driver_native_missing_names"])
            )
            for name in driver_missing_names:
                lines.append(
                    f"| `{name}` | "
                    f"{'covered' if name not in trace['driver_lua_missing_names'] else 'MISSING'} | "
                    f"{'covered' if name not in trace['driver_native_missing_names'] else 'MISSING'} |"
                )
        if trace["argument_mismatches"] or trace["result_mismatches"]:
            lines.extend(["", "### Deterministic driver mismatches", ""])
            for row in trace["argument_mismatches"][:20]:
                lines.append(f"- Argument: `{row['lua']['name']}` — `{json.dumps(row, sort_keys=True)}`")
            for row in trace["result_mismatches"][:20]:
                lines.append(f"- Result: `{row['lua']['name']}` — `{json.dumps(row, sort_keys=True)}`")
        if not trace["coverage_matches"]:
            lines.extend([
                "",
                "### Missing shared callin coverage",
                "",
                "The parity gate remains unverified until every documented shared callback has",
                "been triggered in both the Lua baseline and native run.",
                "",
                "| Callback | Lua baseline | Native run |",
                "| --- | --- | --- |",
            ])
            missing_names = sorted(
                set(trace["missing_lua_names"]) | set(trace["missing_native_names"])
            )
            for name in missing_names:
                lines.append(
                    f"| `{name}` | "
                    f"{'covered' if name not in trace['missing_lua_names'] else 'MISSING'} | "
                    f"{'covered' if name not in trace['missing_native_names'] else 'MISSING'} |"
                )

    lines.extend([
        "",
        "## Native Checks",
        "",
        "| Check | Pass | Skip | Fail |",
        "| --- | ---: | ---: | ---: |",
    ])

    native_rows = (
        []
        if args.mode == "wasm"
        else compare_info["native_rows"]
        if compare_info
        else load_jsonl(comparison_dir / "native.jsonl")
    )
    if native_rows:
        native_counts: dict[str, dict[str, int]] = {}
        native_failures = []
        for row in native_rows:
            name = row_test_name(row) or "<unnamed>"
            status = str(row.get("status", ""))
            native_counts.setdefault(name, {"pass": 0, "skip": 0, "fail": 0})
            if status == "pass":
                native_counts[name]["pass"] += 1
            elif status == "skip":
                native_counts[name]["skip"] += 1
            elif status == "fail":
                native_counts[name]["fail"] += 1
                native_failures.append(row)

        for name, counts in native_counts.items():
            lines.append(
                f"| `{name}` | {counts['pass']} | {counts['skip']} | {counts['fail']} |"
            )

        if native_failures:
            lines.extend(["", "### Native Failures", ""])
            for row in native_failures:
                lines.append(f"- `{row_test_name(row) or '<unnamed>'}`: {row.get('message', '')}")
    else:
        lines.append("| n/a | 0 | 0 | 0 |")

    checked_names = {
        row_test_name(row)
        for row in native_rows
        if row.get("status") != "skip"
    }
    summary = coverage_summary(checked_names, surface_recorded_ids)
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
        (
            f"| Native Rust counterparts | {len(summary['rust_counterpart'])} | "
            f"{len(summary['rust_counterpart_tested'])} | "
            f"{pct(len(summary['rust_counterpart_tested']), len(summary['rust_counterpart']))} |"
        ),
        (
            f"| Native-only Rust surfaces | {len(summary['rust_native_only'])} | "
            f"{len(summary['rust_native_only_tested'])} | "
            f"{pct(len(summary['rust_native_only_tested']), len(summary['rust_native_only']))} |"
        ),
    ])
    if required_surface_ids:
        lines.extend([
            "",
            "## Requested Surface Coverage",
            "",
            f"- Required surface checks: `{len(required_surface_ids)}`",
            f"- Observed surface checks: `{len(required_surface_ids) - len(missing_required_surface_ids)}`",
            f"- Missing requested checks: `{', '.join(f'`{name}`' for name in missing_required_surface_ids) or 'none'}`",
        ])
    for namespace in ("Global", "RmlUi", "gl", "VFS", "Script", "Encoding", "math", "debug", "table"):
        total = summary["lua_surfaces"].get(namespace, set())
        tested = summary["lua_surface_tested_known"] & total
        lines.append(
            f"| Lua `{namespace}.*` | {len(total)} | {len(tested)} | "
            f"{pct(len(tested), len(total))} |"
        )
    lines.extend([
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
    for run_dir in (lua_dir, comparison_dir):
        if not run_dir.is_dir():
            continue
        for path in sorted(run_dir.glob("*.jsonl")):
            lines.append(f"- {report_link(path)}")

    (base_output / "report.md").write_text("\n".join(lines) + "\n", encoding="utf-8")


def run_one(
    args: argparse.Namespace,
    base_output: Path,
    run_mode: str,
    test_seed: int,
    wasm_module: Path | None = None,
    wasm_context: str = "synced_gadget",
    wasm_role: str = "combined",
    wasm_transport: str = "component",
) -> Path:
    run_output = base_output / run_mode
    run_output.mkdir(parents=True, exist_ok=True)
    result_dir = run_output / "write-dir" / "native_api_parity"
    result_dir.mkdir(parents=True, exist_ok=True)

    with tempfile.TemporaryDirectory(prefix="native-api-parity-", dir=base_output) as tmp:
        workdir = Path(tmp)
        datadir, map_name = prepare_datadir(
            workdir, args.map, wasm_module, wasm_context, wasm_transport
        )
        use_blank_map = map_name is None
        if use_blank_map:
            map_name = blank_map_name(args, run_mode)
        script = run_output / "script.txt"
        write_script(
            script,
            map_name,
            "native_api_parity",
            run_mode,
            use_blank_map,
            args,
            test_seed,
            wasm_context,
            wasm_module,
            wasm_role,
            wasm_transport,
        )

        exit_code = run_spring(
            args, datadir, script, run_output, run_mode, wasm_context, wasm_transport
        )
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
    if args.mode == "wasm":
        ensure_wasm_built(args.skip_wasm_build, args.wasm_transport, args.wasm_context)

    if args.mode == "compare":
        if not args.output_dir.is_dir():
            raise SystemExit(
                f"comparison output directory does not exist: {args.output_dir}; "
                "run with --mode both or --mode wasm first"
            )
        runs = sorted(
            path
            for path in args.output_dir.iterdir()
            if path.is_dir() and (path / "lua").is_dir() and (path / "native").is_dir()
        )
        if not runs:
            raise SystemExit(f"no harness runs found in {args.output_dir}")
        latest = runs[-1]
        compare_info = compare_details(
            latest / "lua", latest / "native", not args.skip_callin_compare
        )
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

    lua_dir = None
    native_dir = None
    wasm_dir = None
    if args.mode == "wasm":
        # Setter/getter probes intentionally mutate global engine state.  Run
        # the Lua reference and Wasm guest in isolated scripted processes so
        # the second side cannot observe the first side's mutations (for
        # example an additive resource setter would otherwise be applied
        # twice).  Both processes use the fixture's automatic quit path.
        reference_dir = run_one(
            args,
            base_output,
            "wasm_reference",
            test_seed,
            None,
            args.wasm_context,
            "reference",
            args.wasm_transport,
        )
        wasm_dir = run_one(
            args,
            base_output,
            "wasm_guest",
            test_seed,
            (
                WASM_CONTEXT_CORE_MODULES[args.wasm_context]
                if args.wasm_transport == "core"
                else WASM_CONTEXT_MODULES[args.wasm_context]
            ),
            args.wasm_context,
            "guest",
            args.wasm_transport,
        )
        lua_dir = reference_dir
    else:
        lua_dir = (
            run_one(args, base_output, "lua", test_seed, wasm_context=args.wasm_context)
            if args.mode in ("lua", "both")
            else None
        )
        native_dir = (
            run_one(args, base_output, "native", test_seed)
            if args.mode in ("native", "both")
            else None
        )

    if lua_dir and native_dir:
        compare_info = compare_details(lua_dir, native_dir, not args.skip_callin_compare)
    elif lua_dir and wasm_dir:
        compare_info = compare_wasm_details(lua_dir, wasm_dir, args.wasm_context, args)
    else:
        compare_info = None
    write_report(base_output, args, compare_info)

    missing_required_surface_ids = sorted(
        requested_surface_test_ids(args) - read_surface_test_ids(base_output)
    )
    if missing_required_surface_ids:
        print(
            "requested surface checks were not recorded: "
            + ", ".join(missing_required_surface_ids)
        )
        return 1

    if compare_info and not compare_info["ok"]:
        return 1

    print(base_output)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
