#!/usr/bin/env python3
# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
"""Compare real public-SDK DLL entry points within one live engine process."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile

import run_benchmarks as base


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--worker", action="append", required=True, help="label=/absolute/library.so"
    )
    parser.add_argument(
        "--driver",
        type=Path,
        default=base.ROOT
        / "test/native_api_parity/native_dispatch/target/release/libnative_dispatch_driver.so",
    )
    parser.add_argument(
        "--spring", type=Path, default=base.ENGINE_INSTALL / "spring-headless"
    )
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--iterations", type=int, default=20000)
    parser.add_argument("--repeats", type=int, default=31)
    parser.add_argument("--cache", choices=["warm", "cold"], default="warm")
    parser.add_argument("--cpu", type=int)
    args = parser.parse_args()
    if args.iterations < 1 or args.repeats < 3:
        parser.error("positive iterations and at least three repeats required")
    if args.cpu is not None and (
        not hasattr(os, "sched_getaffinity") or args.cpu not in os.sched_getaffinity(0)
    ):
        parser.error("--cpu must select an available Linux CPU")
    libraries = {}
    for item in args.worker:
        label, value = item.split("=", 1)
        if not label or label in libraries or ";" in item:
            parser.error(
                "worker labels must be nonempty and unique; paths cannot contain semicolons"
            )
        libraries[label] = Path(value).resolve(strict=True)
    out = args.out.resolve()
    out.mkdir(parents=True, exist_ok=False)
    write = out / "write-dir"
    benchmark = write / "benchmark"
    benchmark.mkdir(parents=True)
    with tempfile.TemporaryDirectory(prefix="sdk-native-dispatch-") as folder:
        datadir, _ = base.prepare_datadir(Path(folder), None)
        script = out / "script.txt"
        harness_args = base.make_args(424242, 120, "callouts")
        base.write_script(
            script,
            base.blank_map_name(harness_args, "benchmark"),
            "benchmark",
            "benchmark",
            True,
            harness_args,
            424242,
            "synced_gadget",
            None,
            benchmark_backend="native",
            benchmark_repeats=args.repeats,
            benchmark_scale=1,
            benchmark_case="callouts",
            benchmark_iterations=args.iterations,
        )
        env = os.environ.copy()
        env.update(
            SPRING_DATADIR=os.pathsep.join(
                map(str, [datadir, base.BASE_CONTENT, base.ENGINE_INSTALL])
            ),
            SPRING_ISOLATED=str(datadir),
            SPRING_NATIVE_MODULE=str(args.driver.resolve(strict=True)),
            SPRING_NATIVE_PARITY_OUTPUT_DIR=str(benchmark),
            SPRING_NATIVE_BENCHMARK="1",
            SPRING_NATIVE_BENCHMARK_CASE="callouts",
            SPRING_NATIVE_BENCHMARK_BACKEND="native",
            SDK_DISPATCH_LIBRARIES=";".join(
                f"{label}={path}" for label, path in libraries.items()
            ),
            SDK_DISPATCH_ITERATIONS=str(args.iterations),
            SDK_DISPATCH_REPEATS=str(args.repeats),
            SDK_DISPATCH_CACHE=args.cache,
        )
        env.pop("SDK_DISPATCH_CPU", None)
        if args.cpu is not None:
            env["SDK_DISPATCH_CPU"] = str(args.cpu)
        env.pop("SPRING_NATIVE_BENCHMARK_CALLINS", None)
        with (out / "spring.log").open("wb") as log:
            subprocess.run(
                [
                    str(args.spring.resolve()),
                    "--nocolor",
                    "--write-dir",
                    str(write),
                    str(script),
                ],
                cwd=base.ROOT,
                env=env,
                stdout=log,
                stderr=subprocess.STDOUT,
                timeout=120,
                check=True,
            )
    rows = [
        json.loads(line)
        for line in (benchmark / "dispatch_compare.jsonl").read_text().splitlines()
    ]
    expected = {
        "gameframe_0_queries",
        "gameframe_1_queries",
        "gameframe_16_queries",
        "update",
        "unitcreated",
        "creation",
        "predamaged",
        "command_0",
        "command_4",
        "command_64",
        "command_1024",
        "console_0",
        "console_16",
        "console_1024",
        *(
            f"callout_{name}"
            for name in [
                "scalar",
                "vec3",
                "string",
                "smalllist",
                "biglist",
                "spatial",
                "mutate",
            ]
        ),
    }
    for label in libraries:
        selected = [r for r in rows if r["backend"] == label]
        assert (
            len(selected) == len(expected) and {r["test"] for r in selected} == expected
        )
        assert all(
            r["status"] == "pass" and len(r["samplesNs"]) == args.repeats
            for r in selected
        )
    log = (out / "spring.log").read_text()
    assert not any(
        s in log for s in ["panicked at", "Wasm trap", "Native callin error"]
    )
    result = {
        "measurement": "production DLL ABI entry; live engine API; matched handlers; no engine event dispatcher",
        "cache": args.cache,
        "cpu": args.cpu,
        "counterbalanced": True,
        "libraries": {
            label: {
                "path": str(path),
                "sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
            }
            for label, path in libraries.items()
        },
        "rows": rows,
    }
    (out / "summary.json").write_text(json.dumps(result, indent=2) + "\n")
    for row in rows:
        print(f"{row['backend']:12} {row['test']:24} {row['medianNs']:10.2f} ns")


if __name__ == "__main__":
    main()
