#!/usr/bin/env python3
# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
"""A hostile Core-Wasm menu against the real (headless) engine's game data calls.

The guest (written out here, compiled with wat2wasm) passes pointers outside its memory, wrapped
lengths and paths that try to leave its directory, reports each call's status into a file through
the same call, and must never reach a planted canary or trap the host. A second archive declares no
valid id and must be refused everything.

    python3 run.py --engine build-amd64-linux/install --out DIR
"""

from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import signal
import subprocess
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
PAGE = 65536
REFUSED = "refused: the archive declares no valid gameDataId"


def codes() -> dict[str, int]:
    """The engine's error codes, from its header."""
    text = (ROOT / "rts/NativeInterface/api/CommonTypes.h").read_text()
    found = dict(re.findall(r"\b(ERROR_[A-Z_]+)\s*=\s*(\d+)", text))
    return {name: int(value) for name, value in found.items()}


ERRORS = codes()
OK = 0
OUT_OF_BOUNDS = ERRORS["ERROR_OUT_OF_BOUNDS"]
INVALID = ERRORS["ERROR_INVALID_ARGUMENT"]
DENIED = ERRORS["ERROR_PERMISSION_DENIED"]
NOT_FOUND = ERRORS["ERROR_NOT_FOUND"]
NO_ID = ERRORS["ERROR_INVALID_ID"]

#: (name, call, path bytes, or None for the raw pointers given, expected status)
HOSTILE_PATHS = [
    "../escape.txt",
    "../../escape.txt",
    "a/../../escape.txt",
    "/tmp/game-data-escape.txt",
    "..\\escape.txt",
    "a%2fb.txt",
    "%2e%2e/escape.txt",
    "./a.txt",
    "a//b.txt",
    "a/./b.txt",
    "a/con/b.txt",
    "CON.txt",
    "x.lua",
    "x.so",
    "x.dll",
    "x.exe",
    "x.sdz",
    "x.cfg",
    "x.txt\0/../../escape.txt",
    "a\0.txt",
    ".hidden.txt",
    "x.txt.",
    "x.txt ",
]


def wat_bytes(text: bytes) -> str:
    return "".join(f"\\{b:02x}" for b in text)


def guest() -> tuple[str, list[tuple[str, int]]]:
    """The module's text, and each reported byte's case and expected status."""
    data, body, expected = [], [], []
    at = 4096
    report = 1024  # One status byte for each case.
    descriptor = 512
    output = 600

    def literal(text: bytes) -> tuple[int, int]:
        nonlocal at
        start = at
        data.append(f'(data (i32.const {start}) "{wat_bytes(text)}")')
        at += len(text) + 1
        return start, len(text)

    def write_call(path_ptr: int, path_len: int, data_ptr: int, data_len: int) -> str:
        return (
            f"(i32.store (i32.const {descriptor}) (i32.const {path_ptr & 0xFFFFFFFF}))"
            f"(i32.store (i32.const {descriptor + 4}) (i32.const {path_len & 0xFFFFFFFF}))"
            f"(i32.store (i32.const {descriptor + 8}) (i32.const {data_ptr & 0xFFFFFFFF}))"
            f"(i32.store (i32.const {descriptor + 12}) (i32.const {data_len & 0xFFFFFFFF}))"
            f"(call $write (i32.const {descriptor}))"
        )

    def report_status(call: str) -> None:
        body.append(
            f"(i32.store8 (i32.const {report + len(expected)}) (call $status {call}))"
        )

    good_path = literal(b"ok/ok.txt")
    payload = literal(b"from the guest")
    linked = literal(b"linkdir/planted.txt")
    canary = literal(b"linkdir/canary.txt")
    # A read's path is one length-prefixed blob.
    canary_blob = literal(
        len(b"linkdir/canary.txt").to_bytes(4, "little") + b"linkdir/canary.txt"
    )
    report_path = literal(b"report.bin")

    # Pointers and lengths outside the guest's memory: a status, not a trap and not a host read.
    for name, ptr, length, data_ptr, data_len in [
        ("path pointer past memory", PAGE + 16, 8, payload[0], payload[1]),
        ("path pointer far past memory", 0xFFFFFF00, 8, payload[0], payload[1]),
        ("path length past memory", good_path[0], PAGE, payload[0], payload[1]),
        ("path length wraps", good_path[0], 0xFFFFFFFF, payload[0], payload[1]),
        ("data pointer past memory", good_path[0], good_path[1], PAGE + 16, 8),
        ("data length past memory", good_path[0], good_path[1], payload[0], PAGE),
        ("data length wraps", good_path[0], good_path[1], payload[0], 0xFFFFFFFF),
    ]:
        report_status(write_call(ptr, length, data_ptr, data_len))
        expected.append((name, OUT_OF_BOUNDS))
    report_status("(call $write (i32.const 4294967280))")
    expected.append(("descriptor past memory", OUT_OF_BOUNDS))

    for text in HOSTILE_PATHS:
        path = literal(text.encode())
        report_status(write_call(path[0], path[1], payload[0], payload[1]))
        expected.append((f"write {text!r}", INVALID))

    # Through a link planted beside the game's files: write, read and delete all refused.
    report_status(write_call(linked[0], linked[1], payload[0], payload[1]))
    expected.append(("write through a planted link", DENIED))
    body.append(
        f"(i32.store (i32.const {descriptor}) (i32.const {canary_blob[0]}))"
        f"(i32.store (i32.const {descriptor + 4}) (i32.const {canary_blob[1]}))"
        f"(i32.store (i32.const {output}) (i32.const 8192))"
        f"(i32.store (i32.const {output + 4}) (i32.const 1024))"
    )
    body.append(
        f"(i32.store8 (i32.const {report + len(expected)}) "
        f"(call $read (i32.const {descriptor}) (i32.const {output})))"
    )
    expected.append(("read through a planted link", DENIED))
    body.append(
        f"(i32.store (i32.const {descriptor}) (i32.const {canary[0]}))"
        f"(i32.store (i32.const {descriptor + 4}) (i32.const {canary[1]}))"
    )
    report_status(f"(call $delete (i32.const {descriptor}))")
    expected.append(("delete through a planted link", DENIED))

    # What the guest may do, it can.
    report_status(write_call(good_path[0], good_path[1], payload[0], payload[1]))
    expected.append(("a good write", OK))

    # The report itself, through the same call.
    body.append(
        write_call(report_path[0], report_path[1], report, len(expected)).replace(
            "(call $write", "(drop (call $write"
        )
        + ")"
    )

    module = f"""(module
  (import "spring:vfs" "game-data-write" (func $write (param i32) (result i64)))
  (import "spring:vfs" "game-data-delete" (func $delete (param i32) (result i64)))
  (import "spring:vfs" "game-data-read" (func $read (param i32 i32) (result i32)))
  (func (export "SPRING_ENV_MASK") (result i32) i32.const 32)
  (func (export "spring:callin/scratch-info") (result i64) i64.const 1099511628032)
  (memory (export "memory") 1)
  {" ".join(data)}
  (global $done (mut i32) (i32.const 0))
  (func $status (param i64) (result i32)
    (i32.wrap_i64 (i64.shr_u (local.get 0) (i64.const 32))))
  (func (export "spring:callin/update") (param f32)
    (if (global.get $done) (then return))
    (global.set $done (i32.const 1))
    {" ".join(body)}
  )
)
"""
    return module, expected


def archive(folder: Path, name: str, id_line: str, wat: str) -> None:
    menu = folder / "WasmMenu"
    menu.mkdir(parents=True)
    (folder / "modinfo.lua").write_text(
        f'return {{ name = "{name}", shortname = "x", version = "1", modtype = 5, {id_line} }}\n'
    )
    (menu / "manifest.json").write_text(
        "module(hostile, WasmMenu/hostile.wasm, menu, 0, 1.0.0)\n"
    )
    source = menu / "hostile.wat"
    source.write_text(wat)
    subprocess.run(
        ["wat2wasm", str(source), "-o", str(menu / "hostile.wasm")], check=True
    )


def run_engine(
    engine: Path, run: Path, name: str, report: Path | None, seconds: float
) -> str:
    env = dict(os.environ)
    for directory in ("base", "fonts"):
        link = run / directory
        if not link.exists():
            link.symlink_to((engine / directory).resolve())
    write = run / "write"
    write.mkdir(parents=True, exist_ok=True)
    # Every log line written through at once: the run is watched for them, and then stopped.
    (write / "springsettings.cfg").write_text("LogFlushLevel = 0\n")
    process = subprocess.Popen(
        [
            str((engine / "spring-headless").resolve()),
            "--isolation",
            f"--isolation-dir={run}",
            f"--write-dir={write}",
            f"--menu={name}",
        ],
        env=env,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        start_new_session=True,
    )
    deadline = time.monotonic() + seconds
    log = write / "infolog.txt"
    while time.monotonic() < deadline and process.poll() is None:
        if report is not None and report.is_file():
            time.sleep(1)
            break
        if (
            report is None
            and log.is_file()
            and REFUSED in log.read_text(errors="replace")
        ):
            break
        time.sleep(0.2)
    alive = process.poll() is None
    if alive:
        os.killpg(process.pid, signal.SIGTERM)
        try:
            process.wait(timeout=15)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGKILL)
            process.wait()
    text = log.read_text(errors="replace") if log.is_file() else ""
    if not alive and report is not None and not report.is_file():
        raise SystemExit(f"the engine died ({process.returncode}):\n{text[-2000:]}")
    return text


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--engine", type=Path, default=ROOT / "build-amd64-linux/install"
    )
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--seconds", type=float, default=60)
    args = parser.parse_args()
    out = args.out.resolve()
    shutil.rmtree(out, ignore_errors=True)
    wat, expected = guest()
    failures: list[str] = []

    # An archive with a valid id, a planted link and a canary beside the write directory.
    run = out / "hostile"
    archive(
        run / "games/hostile.sdd",
        "Game Data Hostile",
        'gameDataId = "hostile-fixture"',
        wat,
    )
    game = run / "write/game-data/hostile-fixture"
    game.mkdir(parents=True)
    outside = out / "outside"
    outside.mkdir()
    (outside / "canary.txt").write_text("canary")
    (game / "linkdir").symlink_to(outside)
    report = game / "report.bin"
    log = run_engine(args.engine, run, "Game Data Hostile 1", report, args.seconds)
    if not report.is_file():
        failures.append(f"no report from the guest:\n{log[-1500:]}")
    else:
        got = report.read_bytes()
        statuses = {}
        for (name, want), status in zip(expected, got):
            statuses[name] = dict(want=want, got=status)
            if status != want:
                failures.append(f"{name}: status {status}, expected {want}")
        if len(got) != len(expected):
            failures.append(f"{len(got)} statuses for {len(expected)} cases")
        (out / "statuses.json").write_text(json.dumps(statuses, indent=2))
    if (game / "ok/ok.txt").read_text() != "from the guest":
        failures.append("the good write did not land")
    if (
        sorted(p.name for p in outside.iterdir()) != ["canary.txt"]
        or (outside / "canary.txt").read_text() != "canary"
    ):
        failures.append("something reached the canary directory")
    found = sorted(
        str(p.relative_to(run / "write/game-data"))
        for p in (run / "write/game-data").rglob("*")
    )
    allowed = {"hostile-fixture"} | {
        f"hostile-fixture/{n}" for n in ("linkdir", "ok", "ok/ok.txt", "report.bin")
    }
    if set(found) - allowed:
        failures.append(
            f"unexpected files in the sandbox: {sorted(set(found) - allowed)}"
        )
    for escape in [
        out / "escape.txt",
        run / "escape.txt",
        run / "write/escape.txt",
        Path("/tmp/game-data-escape.txt"),
    ]:
        if escape.exists():
            failures.append(f"escaped to {escape}")
    if "game-data" in "".join(
        line for line in log.splitlines() if "[VFS::GameData" in line
    ):
        failures.append("a refusal's log line names a path")

    # No valid id: refused everything, and no shared directory appears.
    for name, id_line in [
        ("Game Data No Id", ""),
        ("Game Data Bad Id", 'gameDataId = "../up"'),
    ]:
        other = out / name.lower().replace(" ", "-")
        archive(other / "games/fixture.sdd", name, id_line, wat)
        log = run_engine(args.engine, other, f"{name} 1", None, args.seconds)
        if REFUSED not in log:
            failures.append(f"{name}: no refusal in the log:\n{log[-800:]}")
        if (other / "write/game-data").exists():
            failures.append(f"{name}: a directory appeared")

    for folder in [run] + [out / n for n in ("game-data-no-id", "game-data-bad-id")]:
        for link in ("base", "fonts"):
            (folder / link).unlink(missing_ok=True)
    (out / "result.json").write_text(
        json.dumps(dict(passed=not failures, failures=failures), indent=2)
    )
    print(
        "\n".join(failures)
        if failures
        else f"PASS game data hostile guest: {len(expected)} cases"
    )
    return int(bool(failures))


if __name__ == "__main__":
    raise SystemExit(main())
