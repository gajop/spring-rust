#!/usr/bin/env python3
"""Compile both trees' actual DoTask methods against deterministic queue fixtures.

The real method bodies are extracted unchanged. Fixture timer construction models
the three existing profiler clock reads (begin, duration, AddTime). Task clocks
and task statistics are tracked separately. No worker scheduling policy is mocked
into the method itself: all queue/notification/execute/reschedule/finish events are
recorded and must match baseline exactly.
"""

import argparse
import fcntl
import hashlib
import json
from pathlib import Path
import subprocess


def do_task(root):
    source = (root / "rts/System/Threading/ThreadPool.cpp").read_text()
    start = source.index("static bool DoTask(")
    # Inactive boost/non-boost branches both open the same if/while scope;
    # counting raw braces before preprocessing would count those twice.
    end = source.index("\n\n__FORCE_ALIGN_STACK__", start)
    return source[start:end]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline", type=Path, required=True)
    parser.add_argument("--candidate", type=Path, default=Path.cwd())
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--compiler", default="g++-13")
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)
    template = Path(__file__).with_name("trace.cpp").read_text()
    source = template.replace("// INSERT_BASELINE", do_task(args.baseline))
    source = source.replace("// INSERT_CANDIDATE", do_task(args.candidate))
    generated = args.out / "threadpool-task-timing.cpp"
    generated.write_text(source)
    options = ["-std=c++23", "-O2", "-Wall", "-Wextra", "-Werror"]
    with open("/tmp/shipgame-profiling-1000.lock", "a") as lock:
        fcntl.flock(lock, fcntl.LOCK_SH)
        binary = args.out / "threadpool-task-timing"
        subprocess.run([args.compiler, *options, str(generated), "-o", str(binary)], check=True)
        result = subprocess.run([str(binary)], capture_output=True, text=True, check=True)
    (args.out / "trace.txt").write_text(result.stdout)
    summary = {"pass": True, "compiler_options": options, "results": result.stdout.splitlines(),
               "source_sha256": hashlib.sha256(generated.read_bytes()).hexdigest()}
    (args.out / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
