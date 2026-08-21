#!/usr/bin/env python3
"""Run the established Lua/native benchmark suite with Core Wasm.

This layers on run_benchmarks.py instead of forking the experiment. Existing
fixture generation, process options, validation, scales and test names remain
authoritative; this file supplies the Core-Wasm backend and report.
"""

from __future__ import annotations

from datetime import datetime
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

import frozen_benchmarks
import run_benchmarks as base


ROOT = base.ROOT
CORE_CRATE = ROOT / "test" / "wasm_api" / "core_benchmark_suite_guest" / "Cargo.toml"
CORE_RAW = (
    ROOT
    / "test"
    / "wasm_api"
    / "core_benchmark_suite_guest"
    / "target"
    / "wasm32-unknown-unknown"
    / "release"
    / "recoil_wasm_core_benchmark_suite_guest.wasm"
)
CORE_DEFAULT = CORE_RAW.with_name("recoil_wasm_core_benchmark_suite_guest.default.wasm")
CORE_EMPTY = CORE_RAW.with_name("recoil_wasm_core_benchmark_suite_guest.empty.wasm")
CORE_GAMEFRAME = CORE_RAW.with_name("recoil_wasm_core_benchmark_suite_guest.gameframe.wasm")
CORE_UNIMPLEMENTED = CORE_RAW.with_name("recoil_wasm_core_benchmark_suite_guest.unimplemented.wasm")
CORE_UPDATE = CORE_RAW.with_name("recoil_wasm_core_benchmark_suite_guest.update.wasm")
CORE_MEMORY = CORE_RAW.with_name("recoil_wasm_core_benchmark_suite_guest.memory.wasm")
CORE_DRAW = CORE_RAW.with_name("recoil_wasm_core_benchmark_suite_guest.draw.wasm")

# The Component and typed transports were retired. Keep the report focused on
# the two native references and the surviving Core transport.
base.BACKENDS = ("lua", "native", "wasm_core")
REPORT_BACKENDS = base.BACKENDS

# Re-running every backend costs far more than re-running the one under work.
# RECOIL_BENCHMARK_BACKENDS restricts the run to a comma-separated subset, and
# because run_suite re-spawns this script per profile, an environment variable
# is what actually reaches those children.
_BACKEND_FILTER = os.environ.get("RECOIL_BENCHMARK_BACKENDS", "").strip()
if _BACKEND_FILTER:
    _selected = tuple(name.strip() for name in _BACKEND_FILTER.split(",") if name.strip())
    _unknown = [name for name in _selected if name not in REPORT_BACKENDS]
    if _unknown:
        raise SystemExit(
            f"unknown backend(s) in RECOIL_BENCHMARK_BACKENDS: {', '.join(_unknown)}; "
            f"known: {', '.join(REPORT_BACKENDS)}"
        )
    base.BACKENDS = _selected

# RECOIL_BENCHMARK_FREEZE=1 records this run's measurements as the new frozen
# baseline for whichever backends it measured.
FREEZE = os.environ.get("RECOIL_BENCHMARK_FREEZE", "").strip().lower() in {"1", "true", "yes", "on"}
FROZEN_ROOT = Path(
    os.environ.get("RECOIL_BENCHMARK_FROZEN_ROOT", str(frozen_benchmarks.DEFAULT_FROZEN_ROOT))
)

_ORIGINAL_BUILD_WASM = base.build_wasm
_ORIGINAL_RUN_BACKEND = base.run_backend


def core_artifact_for_backend(backend_artifact: Path) -> Path:
    mapping = {
        base.BENCHMARK_COMPONENT: CORE_DEFAULT,
        base.BENCHMARK_COMPONENT_EMPTY: CORE_EMPTY,
        base.BENCHMARK_COMPONENT_GAMEFRAME: CORE_GAMEFRAME,
        base.BENCHMARK_COMPONENT_UNIMPLEMENTED: CORE_UNIMPLEMENTED,
        base.BENCHMARK_COMPONENT_UPDATE: CORE_UPDATE,
        base.BENCHMARK_COMPONENT_MEMORY: CORE_MEMORY,
        base.BENCHMARK_COMPONENT_DRAW: CORE_DRAW,
    }
    try:
        return mapping[Path(backend_artifact)]
    except KeyError as error:
        raise RuntimeError(f"no Core benchmark artifact corresponds to {backend_artifact}") from error


def build_core_wasm(destination: Path, context: str = "synced_gadget") -> None:
    # base.main exports the exact selected case/scale/iteration/repeat/callin
    # variables before invoking build_wasm. Copy that environment verbatim and
    # only add the world selector used by the Core guest build.rs.
    build_env = os.environ.copy()
    build_env["SPRING_BENCHMARK_CONTEXT"] = context
    base.run_checked(
        [
            "cargo",
            "build",
            "--manifest-path",
            str(CORE_CRATE),
            "--target",
            "wasm32-unknown-unknown",
            "--release",
        ],
        cwd=ROOT,
        env=build_env,
    )
    if not CORE_RAW.is_file():
        raise RuntimeError(f"Core Wasm benchmark module was not produced: {CORE_RAW}")
    destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(CORE_RAW, destination)
    if not destination.is_file():
        raise RuntimeError(f"Core Wasm benchmark variant was not produced: {destination}")


def build_wasm_and_core(
    backend_artifact: Path = base.BENCHMARK_COMPONENT,
    context: str = "synced_gadget",
) -> None:
    build_core_wasm(core_artifact_for_backend(backend_artifact), context)


def _run_core_backend(
    root_output: Path,
    seed: int,
    timeout: int,
    spring: Path,
    scale: float,
    benchmark_case: str,
    benchmark_iterations: int,
    benchmark_repeats: int,
    expected_tests: tuple[str, ...],
    *,
    callin_variant: str,
    backend_artifact: Path,
    output_name: str | None,
    wasm_context: str,
    wasm_module_count: int,
) -> list[dict]:
    backend = "wasm_core"
    selected_module = core_artifact_for_backend(backend_artifact)
    if not selected_module.is_file():
        raise RuntimeError(
            f"Core Wasm artifact is missing: {selected_module}; rerun without --skip-build"
        )

    backend_output = root_output / (output_name or backend)
    backend_output.mkdir(parents=True, exist_ok=True)
    workdir = Path(tempfile.mkdtemp(prefix="wasm-benchmark-wasm-core-"))
    try:
        datadir, _ = base.prepare_datadir(workdir, None, selected_module, wasm_context)
        if wasm_module_count > 1:
            wasm_directory = datadir / "games" / "native_api_parity.sdd" / "LuaRules" / "wasm"
            manifest_lines = [
                f"module(parity-{index}, LuaRules/wasm/parity.wasm, "
                f"{base.WASM_ENVIRONMENT_NAMES[wasm_context]}, {index}, 1.0.0)"
                for index in range(wasm_module_count)
            ]
            (wasm_directory / "manifest.txt").write_text(
                "\n".join(manifest_lines) + "\n", encoding="utf-8"
            )

        script = backend_output / "script.txt"
        harness_args = base.make_args(seed, timeout, benchmark_case)
        base.write_script(
            script,
            base.blank_map_name(harness_args, "benchmark"),
            "benchmark",
            "benchmark",
            True,
            harness_args,
            seed,
            wasm_context,
            selected_module,
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
        core_result = benchmark_dir / "benchmark_wasm_core.jsonl"
        guest_result = benchmark_dir / "benchmark_wasm.jsonl"
        # The engine-side callin recorder labels Core directly. The fixture's
        # historical Wasm message sink writes guest-produced rows to the generic
        # benchmark_wasm.jsonl file. Both belong to this one Core process.
        core_result.unlink(missing_ok=True)
        guest_result.unlink(missing_ok=True)

        env = os.environ.copy()
        env["SPRING_ENABLE_SYNCED_TIMERS"] = "1"
        data_dirs = [datadir, base.BASE_CONTENT]
        if base.ENGINE_INSTALL.is_dir():
            data_dirs.append(base.ENGINE_INSTALL)
        env["SPRING_DATADIR"] = os.pathsep.join(str(path) for path in data_dirs)
        env["SPRING_ISOLATED"] = str(datadir)
        env["SPRING_NATIVE_PARITY_OUTPUT_DIR"] = str(benchmark_dir)
        env["SPRING_NATIVE_BENCHMARK"] = "1"
        env["SPRING_NATIVE_MODULE"] = ""
        env["SPRING_WASM_CORE_HOST"] = "1"
        env.pop("SPRING_WASM_TYPED_HOST", None)
        env.pop("SPRING_WASM_TYPED_HOST_LIBRARY", None)
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

        result_paths = [core_result, guest_result]
        if not any(path.is_file() for path in result_paths):
            raise RuntimeError(f"{backend} benchmark produced no result files in {benchmark_dir}")

        rows: list[dict] = []
        for path in result_paths:
            if not path.is_file():
                continue
            for line in path.read_text(encoding="utf-8").splitlines():
                if not line.strip():
                    continue
                row = json.loads(line)
                # Guest rows already use wasm_core, while the fixture is still
                # free to preserve legacy "wasm" labels. Normalize both here.
                row["backend"] = backend
                rows.append(row)

        if benchmark_case == "draw":
            expected = set(expected_tests)
            rows = [
                row
                for row in rows
                if row.get("test") in expected or row.get("test") == "complete"
            ]

        # Core-only transport-ceiling rows deliberately have no peer in the
        # historical backends. Keep them in the summary/report, but do not let
        # them weaken or invalidate the established cross-backend row contract.
        validation_rows = [
            row
            for row in rows
            if not str(row.get("test", "")).startswith("core_ceiling_")
        ]
        base.validate_rows(backend, validation_rows, expected_tests)
        return rows
    finally:
        shutil.rmtree(workdir, ignore_errors=True)


def run_backend_with_core(
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
    backend_artifact: Path = base.BENCHMARK_COMPONENT,
    output_name: str | None = None,
    wasm_context: str = "synced_gadget",
    wasm_module_count: int = 1,
    load_native_module: bool = True,
) -> list[dict]:
    if backend != "wasm_core":
        return _ORIGINAL_RUN_BACKEND(
            backend,
            root_output,
            seed,
            timeout,
            spring,
            skip_build,
            scale,
            benchmark_case,
            benchmark_iterations,
            benchmark_repeats,
            expected_tests,
            callin_variant=callin_variant,
            backend_artifact=backend_artifact,
            output_name=output_name,
            wasm_context=wasm_context,
            wasm_module_count=wasm_module_count,
            load_native_module=load_native_module,
        )
    return _run_core_backend(
        root_output,
        seed,
        timeout,
        spring,
        scale,
        benchmark_case,
        benchmark_iterations,
        benchmark_repeats,
        expected_tests,
        callin_variant=callin_variant,
        backend_artifact=backend_artifact,
        output_name=output_name,
        wasm_context=wasm_context,
        wasm_module_count=wasm_module_count,
    )


def render_report(summaries: list[dict]) -> str:
    # A run may measure only the backend under work; the others are filled from
    # the frozen store so the table stays complete.
    resolved = []
    frozen_backends: set[str] = set()
    for summary in summaries:
        merged, from_frozen = frozen_benchmarks.fill_missing(summary, REPORT_BACKENDS, FROZEN_ROOT)
        resolved.append(merged)
        frozen_backends.update(from_frozen)
    summaries = resolved

    lines = [
        "<!-- This file is generated by test/native_api_parity/run_benchmarks_core.py. -->",
        "",
    ]
    if frozen_backends:
        lines += [
            f"<!-- Frozen baselines used for: {', '.join(sorted(frozen_backends))}. "
            "Those columns were not measured in this run; see "
            "test/native_api_parity/frozen_benchmarks/metadata.json. -->",
            "",
            f"> Columns for {', '.join(sorted(frozen_backends))} are frozen baselines from an"
            " earlier run, not measured alongside the rows below.",
            "",
        ]
    lines += [
        "| Profile | Scale | Test | Lua | Native | Core | "
        "Lua vs native | Lua vs Core | Core vs native |",
        "| --- | ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: |",
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
            rows = {backend: indexed[backend][test] for backend in REPORT_BACKENDS}
            if profile == "memory":
                values = [base.format_memory_metric(rows[backend], test) for backend in REPORT_BACKENDS]
            else:
                values = [base.format_timed_metric(rows[backend], test) for backend in REPORT_BACKENDS]
            lines.append(
                f"| `{profile}` | {float(summary['scale']):g} | `{test}` | {values[0]} | "
                f"{values[1]} | {values[2]} | "
                f"{base.ratio_for_test(test, rows['lua'], rows['native'])} | "
                f"{base.ratio_for_test(test, rows['lua'], rows['wasm_core'])} | "
                f"{base.ratio_for_test(test, rows['wasm_core'], rows['native'])} |"
            )

    lines.append("")
    return "\n".join(lines)


def validate_report_shape(report: str) -> None:
    lines = report.splitlines()
    if len(lines) < 4:
        raise RuntimeError("benchmark report is missing its table")
    if not lines[0].startswith("<!-- This file is generated by "):
        raise RuntimeError("benchmark report is missing its generated marker")
    if lines[1] != "":
        raise RuntimeError("benchmark report must separate its marker from the table")

    # A partial run prepends a comment and a blockquote saying which columns came
    # from the frozen store, so the header is not at a fixed index. Everything
    # before it must still be prose, never a table row.
    header_index = next(
        (
            index
            for index, line in enumerate(lines)
            if line.startswith("| Profile | Scale | Test |")
        ),
        None,
    )
    if header_index is None:
        raise RuntimeError("benchmark report has an unexpected table header")
    if any(line.startswith("|") for line in lines[2:header_index]):
        raise RuntimeError("benchmark report has table rows above its header")

    expected_separator = (
        "| --- | ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: |"
    )
    if not lines[header_index + 1].startswith(expected_separator):
        raise RuntimeError("benchmark report has an unexpected Core-aware table separator")
    if any(line and not line.startswith("|") for line in lines[header_index + 2 :]):
        raise RuntimeError("benchmark report may contain only generated table rows")


def run_suite(args) -> int:
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
    profiles = base.BOUNDED_SUITE_PROFILES if args.bounded_suite else base.SUITE_PROFILES
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
        print(f"[suite+core] {profile}: scale={scale:g}, timeout={timeout}s", flush=True)
        subprocess.run(command, cwd=ROOT, check=True)
        if not summary_path.is_file():
            raise RuntimeError(f"benchmark profile did not write its summary: {summary_path}")
        summaries.append(json.loads(summary_path.read_text(encoding="utf-8")))

    base.write_results(args.results, summaries)
    print(f"wrote {args.results}", flush=True)
    print(f"raw suite output: {suite_root}", flush=True)
    return 0


_ORIGINAL_WRITE_RESULTS = base.write_results


def write_results_and_freeze(output_path, summaries: list[dict]) -> None:
    """Record the measured backends as the new baseline, then write the report.

    Freezing happens before rendering because rendering merges frozen rows in;
    only what this run actually measured should be written back to the store.
    """
    if FREEZE:
        for summary in summaries:
            for path in frozen_benchmarks.freeze_summary(FROZEN_ROOT, summary):
                print(f"[frozen] {path}", flush=True)
        frozen_benchmarks.write_metadata(
            FROZEN_ROOT,
            engine_version=engine_version(),
            cpu=base.cpu_model(),
        )
    _ORIGINAL_WRITE_RESULTS(output_path, summaries)


def engine_version() -> str:
    try:
        return subprocess.run(
            ["git", "describe", "--always", "--dirty"],
            cwd=ROOT,
            check=True,
            capture_output=True,
            text=True,
        ).stdout.strip()
    except Exception:
        return "unknown"


base.build_wasm = build_wasm_and_core
# The retired typed transport is not part of the Core benchmark matrix.
base.build_typed_host = lambda: None
base.run_backend = run_backend_with_core
base.render_report = render_report
base.validate_report_shape = validate_report_shape
base.run_suite = run_suite
base.write_results = write_results_and_freeze


if __name__ == "__main__":
    try:
        raise SystemExit(base.main())
    except Exception as error:
        print(f"benchmark+core failed: {error}", file=sys.stderr)
        raise
