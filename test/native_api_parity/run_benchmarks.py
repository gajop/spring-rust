#!/usr/bin/env python3
"""Run the Lua/native/Wasm benchmark scenario unattended.

The normal profile runs the prescribed matrix.  The bounded profile is a
real, one-case measurement used while iterating: it never presents a partial
matrix as if it were complete.  Each process creates the same fixture and
emits raw elapsed totals as well as normalized values.
"""

from __future__ import annotations

import argparse
from datetime import datetime, timezone
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

from run_harness import (
    BASE_CONTENT,
    ENGINE_INSTALL,
    GAME_FIXTURE,
    NATIVE_CRATE,
    NATIVE_SO,
    ROOT,
    WASM_ENVIRONMENT_NAMES,
    blank_map_name,
    prepare_datadir,
    write_script,
)


BENCHMARK_CRATE = ROOT / "test" / "wasm_api" / "benchmark_guest" / "Cargo.toml"
BENCHMARK_CORE = (
    ROOT
    / "test"
    / "wasm_api"
    / "benchmark_guest"
    / "target"
    / "wasm32-unknown-unknown"
    / "release"
    / "recoil_wasm_benchmark_guest.wasm"
)
BENCHMARK_COMPONENT = (
    ROOT
    / "test"
    / "wasm_api"
    / "benchmark_guest"
    / "target"
    / "recoil_wasm_benchmark_guest.component.wasm"
)
BENCHMARK_COMPONENT_EMPTY = BENCHMARK_COMPONENT.with_name(
    "recoil_wasm_benchmark_guest.empty.component.wasm"
)
BENCHMARK_COMPONENT_GAMEFRAME = BENCHMARK_COMPONENT.with_name(
    "recoil_wasm_benchmark_guest.gameframe.component.wasm"
)
BENCHMARK_COMPONENT_UNIMPLEMENTED = BENCHMARK_COMPONENT.with_name(
    "recoil_wasm_benchmark_guest.unimplemented.component.wasm"
)
BENCHMARK_COMPONENT_MEMORY = BENCHMARK_COMPONENT.with_name(
    "recoil_wasm_benchmark_guest.memory.component.wasm"
)
BENCHMARK_COMPONENT_DRAW = BENCHMARK_COMPONENT.with_name(
    "recoil_wasm_benchmark_guest.draw.component.wasm"
)
BENCHMARK_STAMP = (
    ROOT
    / "test"
    / "wasm_api"
    / "benchmark_guest"
    / "target"
    / "benchmark-profile.json"
)

BACKENDS = ("lua", "native", "wasm")
CALLOUT_TESTS = (
    "callout_scalar",
    "callout_vec3",
    "callout_string",
    "callout_smalllist",
    "callout_biglist",
    "callout_spatial",
    "callout_mutate",
)
HEIGHTMAP_TESTS = (
    "hm_callback_empty",
    "hm_brush_small",
    "hm_brush_medium",
    "hm_brush_large",
    "hm_region_op",
)
WORKLOAD_TESTS = (
	"wl_unit_scan",
	"wl_area_effect",
	"wl_rules_params",
	"wl_commands",
	"wl_compute",
)
MEMORY_TESTS = (
    "mem_per_call_small",
    "mem_per_call_list",
    "gc_pause",
    "frame_spike",
    "mem_growth",
)
CALLIN_TESTS = (
	"callin_empty",
	"callin_gameframe",
	"callin_update",
	"callin_unitcreated",
	"callin_unitpredamaged",
	"callin_allowunitcreation",
	"callin_unimplemented",
	"callin_4modules",
)
CALLIN_COMMON_TESTS = (
	"callin_update",
	"callin_unitcreated",
	"callin_unitpredamaged",
	"callin_allowunitcreation",
)
DRAW_TESTS = (
    "callin_drawworld",
    "callout_draw",
    "wl_ui_draw",
)
TESTS = (
    "callin_empty",
    "callin_gameframe",
    "callin_update",
    "callin_drawworld",
    "callin_unitcreated",
    "callin_unitpredamaged",
    "callin_allowunitcreation",
    "callin_unimplemented",
    "callin_4modules",
    "callout_scalar",
    "callout_vec3",
    "callout_string",
    "callout_smalllist",
    "callout_biglist",
    "callout_spatial",
    "callout_mutate",
    "callout_draw",
    "hm_callback_empty",
    "hm_brush_small",
    "hm_brush_medium",
    "hm_brush_large",
    "hm_region_op",
    "mem_per_call_small",
    "mem_per_call_list",
    "gc_pause",
    "frame_spike",
    "mem_growth",
    "wl_unit_scan",
    "wl_area_effect",
    "wl_rules_params",
    "wl_commands",
    "wl_ui_draw",
    "wl_compute",
)

# The default suite is the documented, scale-1 experiment.  The explicit
# bounded suite is available for iteration on machines where terrain rebuilds
# or render startup make the nominal run impractical; its scale is preserved
# in the generated table and is never presented as the nominal result.
SUITE_PROFILES = (
	("callins", "--callins", 1.0, 180),
	("callouts", "--callouts", 1.0, 120),
	("heightmap", "--heightmap", 1.0, 120),
	("workloads", "--workloads", 1.0, 300),
	("memory", "--memory", 1.0, 180),
	("draw", "--draw", 1.0, 180),
)
BOUNDED_SUITE_PROFILES = (
	("callins", "--callins", 0.1, 20),
	("callouts", "--callouts", 0.1, 20),
	("heightmap", "--heightmap", 0.01, 20),
	("workloads", "--workloads", 0.1, 20),
	("memory", "--memory", 0.1, 20),
	("draw", "--draw", 0.1, 20),
)


def run_checked(command: list[str], *, cwd: Path, env: dict[str, str] | None = None) -> None:
    print("+", " ".join(command), flush=True)
    subprocess.run(command, cwd=cwd, check=True, env=env)


def build_native() -> None:
    run_checked(
        ["cargo", "build", "--manifest-path", str(NATIVE_CRATE), "--release"],
        cwd=ROOT,
    )
    if not NATIVE_SO.is_file():
        raise RuntimeError(f"native benchmark module was not produced: {NATIVE_SO}")


def prepare_benchmark_context(context: str) -> None:
    if context == "synced_gadget":
        lua_output = GAME_FIXTURE / "LuaRules/Utilities/wasm_api_probe_tests.lua"
        manifest_output = ROOT / "test" / "wasm_api" / "parity_guest" / "probe_manifest.json"
    else:
        lua_output = GAME_FIXTURE / f"LuaRules/Utilities/wasm_api_probe_tests_{context}.lua"
        manifest_output = ROOT / "test" / "wasm_api" / "parity_guest" / f"probe_manifest_{context}.json"
    run_checked(
        [
            "python3",
            str(ROOT / "test" / "wasm_api" / "parity_guest" / "generate_probe.py"),
            "--context",
            context,
            "--lua-output",
            str(lua_output),
            "--manifest-output",
            str(manifest_output),
        ],
        cwd=ROOT,
    )


def build_wasm(component: Path = BENCHMARK_COMPONENT, context: str = "synced_gadget") -> None:
    build_env = os.environ.copy()
    build_env["SPRING_BENCHMARK_CONTEXT"] = context
    prepare_benchmark_context(context)
    target = subprocess.run(
        ["rustup", "target", "list", "--installed"],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    ).stdout.splitlines()
    if "wasm32-unknown-unknown" not in target:
        run_checked(["rustup", "target", "add", "wasm32-unknown-unknown"], cwd=ROOT, env=build_env)
    run_checked(
        [
            "cargo",
            "build",
            "--manifest-path",
            str(BENCHMARK_CRATE),
            "--target",
            "wasm32-unknown-unknown",
            "--release",
        ],
        cwd=ROOT,
        env=build_env,
    )
    run_checked(
        [
            "cargo",
            "run",
            "--manifest-path",
            str(BENCHMARK_CRATE),
            "--bin",
            "componentize",
            "--release",
            "--",
            str(BENCHMARK_CORE),
            str(component),
        ],
        cwd=ROOT,
        env=build_env,
    )
    if not component.is_file():
        raise RuntimeError(f"Wasm benchmark component was not produced: {component}")
    # Context generation reuses the parity guest's build inputs, but the
    # checked-in probe sources are the canonical synced context.  Restore
    # those inputs after a context-specific benchmark build so a UI or
    # unsynced run cannot leave the reproducibility gate comparing against
    # its temporary projection.
    if context != "synced_gadget":
        prepare_benchmark_context("synced_gadget")


def write_build_stamp(
    case: str,
    scale: float,
    iterations: int,
    repeats: int,
    callin_variant: str = "empty",
    benchmark_context: str = "synced_gadget",
) -> None:
    BENCHMARK_STAMP.write_text(
        json.dumps(
            {
                "benchmarkCase": case,
                "scale": scale,
                "iterations": iterations,
                "repeats": repeats,
                "callinVariant": callin_variant,
                "benchmarkContext": benchmark_context,
            },
            sort_keys=True,
        )
        + "\n",
        encoding="utf-8",
    )


def check_build_stamp(
    case: str,
    scale: float,
    iterations: int,
    repeats: int,
    callin_variant: str = "empty",
    benchmark_context: str = "synced_gadget",
) -> None:
    if not BENCHMARK_STAMP.is_file():
        raise RuntimeError("--skip-build requires a benchmark-profile.json build stamp")
    try:
        stamp = json.loads(BENCHMARK_STAMP.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise RuntimeError(f"invalid benchmark build stamp: {error}") from error
    expected = {
        "benchmarkCase": case,
        "scale": scale,
        "iterations": iterations,
        "repeats": repeats,
        "callinVariant": callin_variant,
        "benchmarkContext": benchmark_context,
    }
    if stamp != expected:
        raise RuntimeError(
            "--skip-build artifact profile mismatch: "
            f"built={stamp!r}, requested={expected!r}"
        )


def make_args(seed: int, timeout: int, benchmark_case: str) -> argparse.Namespace:
    return argparse.Namespace(
        blank_map_seed=seed,
        blank_map_x=32,
        blank_map_y=32,
        blank_map_height=96,
        cases=1,
        tests=None,
        test_prefix=None,
        enable_rendering_tests=False,
        process_test=None,
        timeout=timeout,
    )


def run_backend(
    backend: str,
    root_output: Path,
    seed: int,
    timeout: int,
    spring: Path,
    skip_build: bool,
    scale: float,
    benchmark_case: str,
    benchmark_iterations: int,
    benchmark_repeats: int,
    expected_tests: tuple[str, ...],
    callin_variant: str = "empty",
    wasm_component: Path = BENCHMARK_COMPONENT,
    output_name: str | None = None,
    wasm_context: str = "synced_gadget",
    wasm_module_count: int = 1,
    load_native_module: bool = True,
) -> list[dict]:
    backend_output = root_output / (output_name or backend)
    backend_output.mkdir(parents=True, exist_ok=True)
    workdir = Path(tempfile.mkdtemp(prefix=f"wasm-benchmark-{backend}-"))
    try:
        wasm_module = wasm_component if backend == "wasm" else None
        datadir, _ = prepare_datadir(workdir, None, wasm_module, wasm_context)
        if backend == "wasm" and wasm_module_count > 1:
            wasm_directory = datadir / "games" / "native_api_parity.sdd" / "LuaRules" / "wasm"
            manifest_lines = [
                f"module(parity-{index}, LuaRules/wasm/parity.wasm, "
                f"{WASM_ENVIRONMENT_NAMES[wasm_context]}, {index}, 1.0.0)"
                for index in range(wasm_module_count)
            ]
            (wasm_directory / "manifest.txt").write_text(
                "\n".join(manifest_lines) + "\n", encoding="utf-8"
            )
        script = backend_output / "script.txt"
        harness_args = make_args(seed, timeout, benchmark_case)
        write_script(
            script,
            blank_map_name(harness_args, "benchmark"),
            "benchmark",
            "benchmark",
            True,
            harness_args,
            seed,
            wasm_context,
            wasm_module,
            benchmark_backend=backend,
            benchmark_repeats=benchmark_repeats,
            benchmark_scale=scale,
            benchmark_case=benchmark_case,
            benchmark_iterations=benchmark_iterations,
            benchmark_callin_variant=callin_variant,
        )

        write_dir = backend_output / "write-dir"
        benchmark_dir = write_dir / "benchmark"
        benchmark_dir.mkdir(parents=True, exist_ok=True)
        # Never parse rows left by an earlier process that reused this
        # write-directory. A benchmark result is valid only when this process
        # produced every row it reports.
        (benchmark_dir / f"benchmark_{backend}.jsonl").unlink(missing_ok=True)
        env = os.environ.copy()
        env["SPRING_ENABLE_SYNCED_TIMERS"] = "1"
        data_dirs = [datadir, BASE_CONTENT]
        if ENGINE_INSTALL.is_dir():
            data_dirs.append(ENGINE_INSTALL)
        env["SPRING_DATADIR"] = os.pathsep.join(str(path) for path in data_dirs)
        env["SPRING_ISOLATED"] = str(datadir)
        env["SPRING_NATIVE_PARITY_OUTPUT_DIR"] = str(benchmark_dir)
        env["SPRING_NATIVE_BENCHMARK"] = "1"
        env["SPRING_NATIVE_MODULE"] = (
            str(NATIVE_SO) if backend == "native" and load_native_module else ""
        )
        env["SPRING_NATIVE_BENCHMARK_SCALE"] = str(scale)
        env["SPRING_NATIVE_BENCHMARK_CASE"] = benchmark_case
        env["SPRING_NATIVE_BENCHMARK_ITERATIONS"] = str(benchmark_iterations)
        env["SPRING_NATIVE_BENCHMARK_REPEATS"] = str(benchmark_repeats)
        env["SPRING_NATIVE_BENCHMARK_BACKEND"] = backend
        env["SPRING_NATIVE_BENCHMARK_CALLIN_VARIANT"] = callin_variant
        env["SPRING_NATIVE_BENCHMARK_MODULES"] = str(wasm_module_count)
        if benchmark_case in {"callins", "draw"}:
            env["SPRING_NATIVE_BENCHMARK_CALLINS"] = "1"
        else:
            env.pop("SPRING_NATIVE_BENCHMARK_CALLINS", None)
        log_path = backend_output / "spring.log"
        command = [str(spring)]
        if wasm_context == "ui":
            # Do not inherit the user's exclusive-fullscreen settings.  UI
            # benchmark runs need a GL context, but must remain unattended and
            # isolated from the desktop.
            (write_dir / "springsettings.cfg").write_text(
                "Fullscreen=0\n"
                "WindowBorderless=0\n"
                "XResolutionWindowed=1280\n"
                "YResolutionWindowed=720\n"
                "UseFontConfigLib=0\n",
                encoding="utf-8",
            )
            command.append("--window")
        command.extend(["--nocolor", "--write-dir", str(write_dir), str(script)])
        print(f"[{backend}] output: {backend_output}", flush=True)
        with log_path.open("wb") as log:
            process = subprocess.Popen(
                command,
                cwd=ROOT,
                env=env,
                stdout=log,
                stderr=subprocess.STDOUT,
            )
            try:
                return_code = process.wait(timeout=timeout)
            except subprocess.TimeoutExpired as exc:
                process.kill()
                process.wait()
                raise RuntimeError(
                    f"{backend} benchmark timed out after {timeout}s; see {log_path}"
                ) from exc
        if return_code != 0:
            tail = log_path.read_text(encoding="utf-8", errors="replace")[-4000:]
            raise RuntimeError(f"{backend} benchmark exited {return_code}:\n{tail}")

        result_path = benchmark_dir / f"benchmark_{backend}.jsonl"
        if not result_path.is_file():
            raise RuntimeError(f"{backend} benchmark produced no result file: {result_path}")
        rows = [
            json.loads(line)
            for line in result_path.read_text(encoding="utf-8").splitlines()
            if line.strip()
        ]
        if benchmark_case == "draw":
            # The process also runs the fixture's normal LuaRules lifecycle;
            # its engine-side callin recorder therefore emits unrelated
            # GameFrame/unit rows.  Draw validation concerns only this UI
            # profile's declared rows.
            rows = [
                row
                for row in rows
                if row.get("test") in set(expected_tests) or row.get("test") == "complete"
            ]
        validate_rows(backend, rows, expected_tests)
        return rows
    finally:
        if skip_build:
            pass
        shutil.rmtree(workdir, ignore_errors=True)


def validate_rows(backend: str, rows: list[dict], expected_tests: tuple[str, ...]) -> None:
    names = [str(row.get("test", "")) for row in rows if row.get("test") != "complete"]
    expected = set(expected_tests)
    actual = set(names)
    missing = sorted(expected - actual)
    extra = sorted(actual - expected)
    duplicates = sorted(name for name in actual if names.count(name) != 1)
    failures = [row for row in rows if row.get("status") in {"error", "fail"}]
    unavailable = sorted(
        name
        for name in names
        if next(row for row in rows if row.get("test") == name).get("status")
        == "unavailable"
    )
    if missing or extra or duplicates or failures or unavailable:
        raise RuntimeError(
            f"{backend} benchmark row validation failed: missing={missing}, "
            f"extra={extra}, duplicates={duplicates}, unavailable={unavailable}, "
            f"failures={failures}"
        )
    print(f"[{backend}] {len(names)} tests", flush=True)


def numeric_metric(row: dict, test: str) -> tuple[float, str] | None:
    if row.get("status") == "unavailable":
        return None
    if "medianNs" in row:
        value = float(row["medianNs"])
        return (value / 1_000_000.0, "ms") if test.startswith("wl_") else (value, "ns")
    if "meanNs" in row:
        return float(row["meanNs"]), "ns"
    if "medianMs" in row:
        return float(row["medianMs"]), "ms"
    for key, unit in (
        ("totalPauseMs", "ms"),
        ("worstMs", "ms"),
        ("peakBytes", "B"),
        ("bytes", "B"),
        ("gcKB", "KB"),
    ):
        if key in row:
            return float(row[key]), unit
    return None


def format_metric(metric: tuple[float, str] | None) -> str:
    if metric is None:
        return "—"
    value, unit = metric
    if unit == "ms":
        return f"{value:.6f} {unit}"
    return f"{value:.3f} {unit}"


def spread_metric(row: dict, test: str) -> tuple[float, str] | None:
    if "spreadNs" in row:
        value = float(row["spreadNs"])
        return (value / 1_000_000.0, "ms") if test.startswith("wl_") else (value, "ns")
    if "spreadMs" in row:
        return float(row["spreadMs"]), "ms"
    return None


def format_timed_metric(row: dict, test: str) -> str:
    value = format_metric(numeric_metric(row, test))
    spread = format_metric(spread_metric(row, test))
    return value if spread == "—" else f"{value} ± {spread}"


def format_memory_metric(row: dict, test: str) -> str:
    if row.get("status") == "unavailable":
        return "—"
    if test in {"mem_per_call_small", "mem_per_call_list"}:
        bytes_value = float(row.get("bytes", 0.0))
        allocations = float(row.get("allocations", 0.0))
        if "elapsedMs" in row:
            timing = f"{float(row['elapsedMs']):.3f} ms total"
        elif "medianNs" in row:
            timing = f"{float(row['medianNs']):.3f} ns/call"
        else:
            timing = "timing unavailable"
        return f"{bytes_value:.0f} B / {allocations:.0f} alloc / {timing}"
    if test == "gc_pause":
        gc_kb = row.get("gcKB", row.get("gcAfterKB"))
        pause = row.get("totalPauseMs")
        parts = []
        if gc_kb is not None:
            parts.append(f"{float(gc_kb):.0f} KB GC")
        if pause is not None:
            parts.append(f"{float(pause):.3f} ms")
        return " / ".join(parts) or "—"
    if test == "frame_spike":
        p99 = row.get("p99Ms")
        worst = row.get("worstMs")
        parts = []
        if p99 is not None:
            parts.append(f"p99 {float(p99):.6f} ms")
        if worst is not None:
            parts.append(f"worst {float(worst):.6f} ms")
        return " / ".join(parts) or "—"
    if test == "mem_growth":
        peak = row.get("peakBytes")
        steady = row.get("steadyBytes")
        parts = []
        if peak is not None:
            parts.append(f"peak {float(peak):.0f} B")
        if steady is not None:
            parts.append(f"steady {float(steady):.0f} B")
        return " / ".join(parts) or "—"
    return format_metric(numeric_metric(row, test))


def ratio_for_test(
    test: str,
    left: dict,
    right: dict,
) -> str:
    if test in {"mem_per_call_small", "mem_per_call_list"}:
        return "—"
    if test == "gc_pause":
        left_metric = (float(left["totalPauseMs"]), "ms") if "totalPauseMs" in left else None
        right_metric = (float(right["totalPauseMs"]), "ms") if "totalPauseMs" in right else None
    elif test == "frame_spike":
        left_metric = (float(left["p99Ms"]), "ms") if "p99Ms" in left else None
        right_metric = (float(right["p99Ms"]), "ms") if "p99Ms" in right else None
    else:
        left_metric = numeric_metric(left, test)
        right_metric = numeric_metric(right, test)
    return ratio(left_metric, right_metric)


def cpu_model() -> str:
    try:
        return subprocess.run(
            ["lscpu", "-p=CPU,MODELNAME"],
            check=True,
            capture_output=True,
            text=True,
        ).stdout.split("#")[0].strip().splitlines()[0].split(",", 1)[1]
    except (FileNotFoundError, IndexError, subprocess.CalledProcessError):
        return "unknown"


def make_summary(
    run_root: Path,
    rows_by_backend: dict[str, list[dict]],
    seed: int,
    scale: float,
    expected_tests: tuple[str, ...],
    benchmark_case: str,
    benchmark_iterations: int,
    benchmark_repeats: int,
) -> dict:
    return {
        "profile": benchmark_case or "full",
        "runRoot": str(run_root),
        "seed": seed,
        "scale": scale,
        "iterations": benchmark_iterations,
        "repeats": benchmark_repeats,
        "cpu": cpu_model(),
        "tests": list(expected_tests),
        "rows": rows_by_backend,
    }


def write_summary(output_path: Path, summary: dict) -> None:
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(
        json.dumps(summary, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )


def render_report(summaries: list[dict]) -> str:
    lines = [
        "<!-- This file is generated by test/native_api_parity/run_benchmarks.py. -->",
        "",
        "| Profile | Scale | Test | Lua | Native | Wasm | Wasm vs Lua | Wasm vs native |",
        "| --- | ---: | --- | ---: | ---: | ---: | ---: | ---: |",
    ]
    for summary in summaries:
        profile = str(summary["profile"])
        indexed = {
            backend: {
                str(row["test"]): row
                for row in rows
                if row.get("test") != "complete"
            }
            for backend, rows in summary["rows"].items()
        }
        for test in (str(test) for test in summary["tests"]):
            rows = {backend: indexed[backend][test] for backend in BACKENDS}
            if profile == "memory":
                values = [format_memory_metric(rows[backend], test) for backend in BACKENDS]
            else:
                values = [
                    format_timed_metric(rows[backend], test)
                    for backend in BACKENDS
                ]
            lines.append(
                f"| `{profile}` | {float(summary['scale']):g} | `{test}` | {values[0]} | "
                f"{values[1]} | {values[2]} | "
                f"{ratio_for_test(test, rows['wasm'], rows['lua'])} | "
                f"{ratio_for_test(test, rows['wasm'], rows['native'])} |"
            )
    lines.append("")
    return "\n".join(lines)


def validate_report_shape(report: str) -> None:
    """Reject hand-written narrative from the generated results artifact."""
    lines = report.splitlines()
    if len(lines) < 4:
        raise RuntimeError("benchmark report is missing its table")
    if not lines[0].startswith("<!-- This file is generated by "):
        raise RuntimeError("benchmark report is missing its generated marker")
    if lines[1] != "":
        raise RuntimeError("benchmark report must separate its marker from the table")
    if not lines[2].startswith("| Profile | Scale | Test |"):
        raise RuntimeError("benchmark report has an unexpected table header")
    if not lines[3].startswith("| --- | ---: | --- |"):
        raise RuntimeError("benchmark report has an unexpected table separator")
    if any(line and not line.startswith("|") for line in lines[4:]):
        raise RuntimeError("benchmark report may contain only generated table rows")


def write_results(output_path: Path, summaries: list[dict]) -> None:
    output_path.parent.mkdir(parents=True, exist_ok=True)
    report = render_report(summaries)
    validate_report_shape(report)
    output_path.write_text(report, encoding="utf-8")


def ratio(left: tuple[float, str] | None, right: tuple[float, str] | None) -> str:
    if left is None or right is None or left[1] != right[1] or right[0] == 0:
        return "—"
    return f"{left[0] / right[0]:.2f}×"


def run_suite(args: argparse.Namespace) -> int:
    if args.skip_build:
        raise RuntimeError("--suite requires builds; use an individual profile with --skip-build")
    if any(
        getattr(args, name)
        for name in ("quick", "callouts", "heightmap", "workloads", "callins", "memory", "draw")
    ):
        raise RuntimeError("--suite cannot be combined with an individual benchmark profile")
    if args.scale is not None and not 0.0 < args.scale <= 1.0:
        raise RuntimeError(f"--scale must be greater than 0 and no greater than 1: {args.scale}")

    suite_root = args.output_root / f"suite-{datetime.now().strftime('%Y%m%d-%H%M%S-%f')}"
    summaries = []
    profiles = BOUNDED_SUITE_PROFILES if args.bounded_suite else SUITE_PROFILES
    for profile, flag, default_scale, default_timeout in profiles:
        scale = args.scale if args.scale is not None else default_scale
        timeout = args.timeout if args.timeout is not None else default_timeout
        summary_path = suite_root / f"{profile}.json"
        profile_output = suite_root / profile
        command = [
            sys.executable,
            str(Path(__file__).resolve()),
            flag,
            "--seed",
            str(args.seed),
            "--scale",
            str(scale),
            "--timeout",
            str(timeout),
            "--spring-headless",
            str(args.spring_headless),
            "--spring",
            str(args.spring),
            "--output-root",
            str(profile_output),
            "--summary-json",
            str(summary_path),
            "--no-report",
        ]
        print(f"[suite] {profile}: scale={scale:g}, timeout={timeout}s", flush=True)
        subprocess.run(command, cwd=ROOT, check=True)
        if not summary_path.is_file():
            raise RuntimeError(f"benchmark profile did not write its summary: {summary_path}")
        summaries.append(json.loads(summary_path.read_text(encoding="utf-8")))

    write_results(args.results, summaries)
    print(f"wrote {args.results}", flush=True)
    print(f"raw suite output: {suite_root}", flush=True)
    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--spring-headless", type=Path, default=ENGINE_INSTALL / "spring-headless")
    parser.add_argument("--spring", type=Path, default=ENGINE_INSTALL / "spring")
    parser.add_argument("--output-root", type=Path, default=ROOT / "test" / "native_api_parity" / "out" / "benchmark")
    parser.add_argument("--results", type=Path, default=ROOT / "rts" / "wasm" / "docs" / "impl" / "benchmarking_results.md")
    parser.add_argument(
        "--suite",
        action="store_true",
        help="run all nominal profiles and regenerate one consolidated report",
    )
    parser.add_argument(
        "--bounded-suite",
        action="store_true",
        help="use reduced real workloads for development instead of the nominal suite",
    )
    parser.add_argument(
        "--summary-json",
        type=Path,
        help="write the machine-readable profile summary here",
    )
    parser.add_argument(
        "--no-report",
        action="store_true",
        help="run a profile without writing its Markdown report",
    )
    parser.add_argument("--seed", type=int, default=424242)
    parser.add_argument(
        "--quick",
        action="store_true",
        help="run one bounded, real callout case across Lua/native/Wasm",
    )
    parser.add_argument(
        "--callouts",
        action="store_true",
        help="run the seven callout cases only; excludes callins, heightmap, memory, and workloads",
    )
    parser.add_argument(
        "--heightmap",
        action="store_true",
        help="run the five heightmap cases only",
    )
    parser.add_argument(
        "--workloads",
        action="store_true",
        help="run the five non-rendering workload cases only",
    )
    parser.add_argument(
        "--callins",
        action="store_true",
        help="run actual GameFrame/Update and deterministic unit-event engine callins",
    )
    parser.add_argument(
        "--memory",
        action="store_true",
        help="run the five unsynced profiling/memory cases only",
    )
    parser.add_argument(
        "--draw",
        action="store_true",
        help="run the UI DrawWorld, draw-callout, and UI workload cases only",
    )
    parser.add_argument(
        "--scale",
        type=float,
        help="scale operation counts and brush dimensions for the complete matrix (default: 1.0)",
    )
    parser.add_argument(
        "--iterations",
        type=int,
        default=1_000_000,
        help="bounded-profile call count (default: 1,000,000)",
    )
    parser.add_argument("--timeout", type=int)
    parser.add_argument("--skip-build", action="store_true")
    args = parser.parse_args()

    if args.bounded_suite and not args.suite:
        raise RuntimeError("--bounded-suite requires --suite")
    if args.suite:
        return run_suite(args)

    profiles = sum(bool(value) for value in (args.quick, args.callouts, args.heightmap, args.workloads, args.callins, args.memory, args.draw))
    if profiles > 1:
        raise RuntimeError("benchmark profiles are mutually exclusive")
    if args.quick and args.scale is not None:
        raise RuntimeError("--quick and --scale are mutually exclusive")
    if args.iterations <= 0:
        raise RuntimeError("--iterations must be positive")
    benchmark_case = (
        "callout_scalar"
        if args.quick
        else "callouts"
        if args.callouts
        else "heightmap"
        if args.heightmap
        else "workloads"
        if args.workloads
        else "callins"
        if args.callins
        else "memory"
        if args.memory
        else "draw"
        if args.draw
        else ""
    )
    benchmark_iterations = args.iterations if args.quick else 0
    benchmark_repeats = 3 if args.quick else 5
    expected_tests = (
        ("callout_scalar",)
        if args.quick
        else CALLOUT_TESTS
        if args.callouts
        else HEIGHTMAP_TESTS
        if args.heightmap
        else WORKLOAD_TESTS
        if args.workloads
        else CALLIN_TESTS
        if args.callins
        else DRAW_TESTS
        if args.draw
        else MEMORY_TESTS
        if args.memory
        else TESTS
    )
    scale = args.scale if args.scale is not None else (0.01 if args.callins or args.memory or args.draw else 1.0)
    if not 0.0 < scale <= 1.0:
        raise RuntimeError(f"--scale must be greater than 0 and no greater than 1: {scale}")
    if args.timeout is not None:
        timeout = args.timeout
    elif benchmark_case == "heightmap":
        # A non-empty terrain edit can trigger path/terrain rebuild work.  Keep
        # the bounded profile honest and fail quickly instead of silently
        # turning one pathological case into a multi-minute suite.
        timeout = 15
    elif benchmark_case in {"callout_scalar", "callouts", "workloads", "callins", "memory", "draw"}:
        timeout = 20
    else:
        # The unscaled 33-row profile is intentionally not the default fast
        # path.  Its caller must opt into a longer timeout explicitly after
        # implementing the remaining real callin/UI/memory cases.
        timeout = 30
    os.environ["SPRING_BENCHMARK_SCALE"] = str(scale)
    os.environ["SPRING_BENCHMARK_CASE"] = benchmark_case
    os.environ["SPRING_BENCHMARK_ITERATIONS"] = str(benchmark_iterations)
    os.environ["SPRING_BENCHMARK_REPEATS"] = str(benchmark_repeats)
    os.environ["SPRING_BENCHMARK_CALLIN_VARIANT"] = "empty"
    benchmark_context = (
        "unsynced_gadget"
        if benchmark_case == "memory"
        else "ui"
        if benchmark_case == "draw"
        else "synced_gadget"
    )

    if not args.spring_headless.is_file():
        raise RuntimeError(f"spring-headless not found: {args.spring_headless}")
    if benchmark_case == "draw" and not args.spring.is_file():
        raise RuntimeError(f"spring not found: {args.spring}")
    if not args.skip_build:
        build_native()
        if benchmark_case == "callins":
            for variant, component in (
                ("empty", BENCHMARK_COMPONENT_EMPTY),
                ("gameframe", BENCHMARK_COMPONENT_GAMEFRAME),
                ("unimplemented", BENCHMARK_COMPONENT_UNIMPLEMENTED),
            ):
                os.environ["SPRING_BENCHMARK_CALLIN_VARIANT"] = variant
                build_wasm(component)
            write_build_stamp(
                benchmark_case,
                scale,
                benchmark_iterations,
                benchmark_repeats,
                "all",
                benchmark_context,
            )
        elif benchmark_case == "memory":
            build_wasm(BENCHMARK_COMPONENT_MEMORY, benchmark_context)
            write_build_stamp(
                benchmark_case,
                scale,
                benchmark_iterations,
                benchmark_repeats,
                benchmark_context=benchmark_context,
            )
        elif benchmark_case == "draw":
            build_wasm(BENCHMARK_COMPONENT_DRAW, benchmark_context)
            write_build_stamp(
                benchmark_case,
                scale,
                benchmark_iterations,
                benchmark_repeats,
                benchmark_context=benchmark_context,
            )
        else:
            build_wasm(BENCHMARK_COMPONENT, benchmark_context)
            write_build_stamp(
                benchmark_case,
                scale,
                benchmark_iterations,
                benchmark_repeats,
                benchmark_context=benchmark_context,
            )
    elif not NATIVE_SO.is_file() or not (
        (
            BENCHMARK_COMPONENT_EMPTY.is_file()
            and BENCHMARK_COMPONENT_GAMEFRAME.is_file()
            and BENCHMARK_COMPONENT_UNIMPLEMENTED.is_file()
        )
        if benchmark_case == "callins"
        else BENCHMARK_COMPONENT_MEMORY.is_file()
        if benchmark_case == "memory"
        else BENCHMARK_COMPONENT_DRAW.is_file()
        if benchmark_case == "draw"
        else BENCHMARK_COMPONENT.is_file()
    ):
        raise RuntimeError("--skip-build requires both the native and Wasm benchmark artifacts")
    else:
        check_build_stamp(
            benchmark_case,
            scale,
            benchmark_iterations,
            benchmark_repeats,
            "all" if benchmark_case == "callins" else "empty",
            benchmark_context,
        )

    run_root = args.output_root / datetime.now().strftime("%Y%m%d-%H%M%S")
    rows_by_backend = {}
    for backend in BACKENDS:
        if benchmark_case == "draw":
            rows_by_backend[backend] = run_backend(
                backend,
                run_root,
                args.seed,
                timeout,
                args.spring,
                args.skip_build,
                scale,
                benchmark_case,
                benchmark_iterations,
                benchmark_repeats,
                expected_tests,
                wasm_component=BENCHMARK_COMPONENT_DRAW,
                wasm_context=benchmark_context,
            )
            continue

        if benchmark_case not in {"callins", "memory"}:
            rows_by_backend[backend] = run_backend(
                backend,
                run_root,
                args.seed,
                timeout,
                args.spring_headless,
                args.skip_build,
                scale,
                benchmark_case,
                benchmark_iterations,
                benchmark_repeats,
                expected_tests,
                wasm_context=benchmark_context,
            )
            continue

        if benchmark_case == "memory":
            rows_by_backend[backend] = run_backend(
                backend,
                run_root,
                args.seed,
                timeout,
                args.spring_headless,
                args.skip_build,
                scale,
                benchmark_case,
                benchmark_iterations,
                benchmark_repeats,
                expected_tests,
                wasm_component=BENCHMARK_COMPONENT_MEMORY,
                wasm_context=benchmark_context,
            )
            continue

        empty_rows = run_backend(
            backend,
            run_root,
            args.seed,
            timeout,
            args.spring_headless,
            args.skip_build,
            scale,
            benchmark_case,
            benchmark_iterations,
            benchmark_repeats,
            ("callin_empty",) + CALLIN_COMMON_TESTS,
            callin_variant="empty",
            wasm_component=BENCHMARK_COMPONENT_EMPTY,
            output_name=f"{backend}-empty",
            wasm_context=benchmark_context,
        )
        gameframe_rows = run_backend(
            backend,
            run_root,
            args.seed,
            timeout,
            args.spring_headless,
            args.skip_build,
            scale,
            benchmark_case,
            benchmark_iterations,
            benchmark_repeats,
            ("callin_gameframe",) + CALLIN_COMMON_TESTS,
            callin_variant="gameframe",
            wasm_component=BENCHMARK_COMPONENT_GAMEFRAME,
            output_name=f"{backend}-gameframe",
            wasm_context=benchmark_context,
        )
        unimplemented_rows = run_backend(
            backend,
            run_root,
            args.seed,
            timeout,
            args.spring_headless,
            args.skip_build,
            scale,
            benchmark_case,
            benchmark_iterations,
            benchmark_repeats,
            ("callin_unimplemented",),
            callin_variant="unimplemented",
            wasm_component=BENCHMARK_COMPONENT_UNIMPLEMENTED,
            output_name=f"{backend}-unimplemented",
            wasm_context=benchmark_context,
            load_native_module=False,
        )
        four_module_rows = run_backend(
            backend,
            run_root,
            args.seed,
            timeout,
            args.spring_headless,
            args.skip_build,
            scale,
            benchmark_case,
            benchmark_iterations,
            benchmark_repeats,
            ("callin_4modules",),
            callin_variant="fourmodules",
            wasm_component=BENCHMARK_COMPONENT_EMPTY,
            output_name=f"{backend}-fourmodules",
            wasm_context=benchmark_context,
            wasm_module_count=4,
        )
        selected_gameframe = next(
            row for row in gameframe_rows if row.get("test") == "callin_gameframe"
        )
        rows_by_backend[backend] = [
            row for row in empty_rows if row.get("test") != "complete"
        ] + [selected_gameframe]
        rows_by_backend[backend] += [
            row for row in unimplemented_rows if row.get("test") != "complete"
        ]
        rows_by_backend[backend] += [
            row for row in four_module_rows if row.get("test") != "complete"
        ]
    summary = make_summary(
        run_root,
        rows_by_backend,
        args.seed,
        scale,
        expected_tests,
        benchmark_case,
        benchmark_iterations,
        benchmark_repeats,
    )
    if args.summary_json is not None:
        write_summary(args.summary_json, summary)
    if not args.no_report:
        write_results(args.results, [summary])
        print(f"wrote {args.results}", flush=True)
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as error:
        print(f"benchmark failed: {error}", file=sys.stderr)
        raise
