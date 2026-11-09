#!/usr/bin/env python3
# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
"""A console line longer than a Core-Wasm module's callin scratch, against the real (headless)
engine: the module stays registered, and hears the line cut to fit with a marker saying so.

The guest (written out here, compiled with wat2wasm) has a 256-byte scratch. It logs one short
line and one long one of two-byte characters, saves what it hears through the game data calls
and, many updates later, proves it is still running.

    python3 run.py --engine build-amd64-linux/install --out DIR
"""

from __future__ import annotations

import argparse
import json
import os
import shutil
import signal
import struct
import subprocess
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
FAULT = "was deregistered due to fault"
NAME = "Console Line Fixture"
SCRATCH = 4096
CAPACITY = 256
SHORT = "FIXSHORT is a line that fits"
LONG = "FIXLONG_" + "é" * 150_000
SECTION = "fixture"
PREFIX = f"[{SECTION}] "
MARKER = " ...[cut to fit this module: "
SECTION_AT, SHORT_AT, LONG_AT = 7168, 8192, 16384
LOG_LEVEL = 35  # L_NOTICE
LOG_RECORD_BYTES = 32768  # `log_record_t::msg`, the longest line the log hands on
ALIVE_AFTER_UPDATES = 300


def wat_bytes(text: bytes) -> str:
    return "".join(f"\\{b:02x}" for b in text)


def guest() -> str:
    short, long = SHORT.encode(), LONG.encode()
    pages = (LONG_AT + len(long)) // 65536 + 1
    names = {"short.bin": 512, "long.bin": 528, "alive.bin": 544}
    data = "\n  ".join(
        f'(data (i32.const {at}) "{wat_bytes(text)}")'
        for at, text in [
            *((at, name.encode()) for name, at in names.items()),
            (SECTION_AT, SECTION.encode()),
            (SHORT_AT, short),
            (LONG_AT, long),
        ]
    )

    def save(name: str, at: str, length: str) -> str:
        return (
            f"(i32.store (i32.const 600) (i32.const {names[name]}))"
            f"(i32.store (i32.const 604) (i32.const {len(name)}))"
            f"(i32.store (i32.const 608) {at})"
            f"(i32.store (i32.const 612) {length})"
            "(drop (call $write (i32.const 600)))"
        )

    def log(at: int, length: int) -> str:
        return (
            f"(drop (call $log (i32.const {SECTION_AT}) (i32.const {len(SECTION)}) "
            f"(i32.const {LOG_LEVEL}) (i32.const {at}) (i32.const {length})))"
        )

    # The log record starts a line in a section with `[section] `.
    message_at = (
        f"(i32.add (i32.const {SCRATCH + len(PREFIX)}) (i32.load (i32.const {SCRATCH})))"
    )
    magic = "(i64.load (local.get $at))"
    return f"""(module
  (import "spring:vfs" "game-data-write" (func $write (param i32) (result i64)))
  (import "spring:messages" "log" (func $log (param i32 i32 i32 i32 i32) (result i64)))
  (func (export "SPRING_ENV_MASK") (result i32) i32.const 32)
  (func (export "spring:callin/scratch-info") (result i64)
    (i64.const {SCRATCH | (CAPACITY << 32)}))
  (memory (export "memory") {pages})
  {data}
  (global $updates (mut i32) (i32.const 0))
  (func (export "spring:callin/add-console-line") (param $used i32) (result i64)
    (local $at i32)
    (local.set $at {message_at})
    (if (i64.eq {magic} (i64.const {int.from_bytes(SHORT[:8].encode(), "little")}))
      (then {save("short.bin", f"(i32.const {SCRATCH})", "(local.get $used)")}))
    (if (i64.eq {magic} (i64.const {int.from_bytes(LONG[:8].encode(), "little")}))
      (then {save("long.bin", f"(i32.const {SCRATCH})", "(local.get $used)")}))
    (i64.const 0))
  (func (export "spring:callin/update") (param f32)
    (global.set $updates (i32.add (global.get $updates) (i32.const 1)))
    (if (i32.eq (global.get $updates) (i32.const 1))
      (then
        {log(SHORT_AT, len(short))}
        {log(LONG_AT, len(long))}))
    (if (i32.eq (global.get $updates) (i32.const {ALIVE_AFTER_UPDATES}))
      (then {save("alive.bin", "(i32.const 512)", "(i32.const 1)")})))
)
"""


def line_in(payload: bytes) -> tuple[bytes, bytes, int]:
    """The `AddConsoleLine` wire record: message, section and level."""
    message_at, message_size, section_at, section_size, level = struct.unpack_from(
        "<IIIIi", payload
    )
    return (
        payload[message_at : message_at + message_size],
        payload[section_at : section_at + section_size],
        level,
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--engine", type=Path, default=ROOT / "build-amd64-linux/install"
    )
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--seconds", type=float, default=60)
    args = parser.parse_args()
    out, engine = args.out.resolve(), args.engine.resolve()
    shutil.rmtree(out, ignore_errors=True)
    menu = out / "games/fixture.sdd/WasmMenu"
    menu.mkdir(parents=True)
    (menu.parent / "modinfo.lua").write_text(
        f'return {{ name = "{NAME}", shortname = "x", version = "1", modtype = 5, '
        'gameDataId = "console-line-fixture" }\n'
    )
    (menu / "manifest.json").write_text(
        "module(console-line, WasmMenu/console_line.wasm, menu, 0, 1.0.0)\n"
    )
    source = menu / "console_line.wat"
    source.write_text(guest())
    subprocess.run(
        ["wat2wasm", str(source), "-o", str(menu / "console_line.wasm")], check=True
    )
    for name in ("base", "fonts"):
        (out / name).symlink_to(engine / name)
    write = out / "write"
    write.mkdir()
    (write / "springsettings.cfg").write_text("LogFlushLevel = 0\n")

    process = subprocess.Popen(
        [
            str(engine / "spring-headless"),
            "--isolation",
            f"--isolation-dir={out}",
            f"--write-dir={write}",
            f"--menu={NAME} 1",
        ],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        start_new_session=True,
        env=dict(os.environ),
    )
    saved = write / "game-data/console-line-fixture"
    log = write / "infolog.txt"
    deadline = time.monotonic() + args.seconds
    while time.monotonic() < deadline and process.poll() is None:
        if (saved / "alive.bin").is_file():
            break
        if log.is_file() and FAULT in log.read_text(errors="replace"):
            break
        time.sleep(0.2)
    exited = process.poll()
    if exited is None:
        os.killpg(process.pid, signal.SIGTERM)
        try:
            process.wait(timeout=15)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGKILL)
            process.wait()
    text = log.read_text(errors="replace") if log.is_file() else ""

    failures = []
    if FAULT in text:
        failures.append(next(l for l in text.splitlines() if FAULT in l))
    if exited is not None:
        failures.append(f"the engine exited with {exited}")
    if not (saved / "alive.bin").is_file():
        failures.append("the module did not reach its later updates")
    heard = {}
    if (saved / "short.bin").is_file():
        message, section, level = line_in((saved / "short.bin").read_bytes())
        heard["short"] = dict(bytes=len(message), section=section.decode())
        if (message.decode(), section.decode(), level) != (PREFIX + SHORT, SECTION, LOG_LEVEL):
            failures.append(f"the short line changed: {message!r} {section!r} {level}")
    else:
        failures.append("the module did not hear the short line")
    if (saved / "long.bin").is_file():
        payload = (saved / "long.bin").read_bytes()
        message, section, _ = line_in(payload)
        heard["long"] = dict(
            scratch_bytes=len(payload),
            message_bytes=len(message),
            logged_bytes=LOG_RECORD_BYTES,
            tail=message[-60:].decode(errors="replace"),
        )
        if len(payload) > CAPACITY:
            failures.append(f"{len(payload)} bytes in a {CAPACITY}-byte scratch")
        if section.decode() != SECTION:
            failures.append(f"the long line's section changed: {section!r}")
        try:
            decoded = message.decode()
            if not decoded.startswith(PREFIX + LONG[:9]) or MARKER not in decoded:
                failures.append(f"no marker on the cut line: {decoded[-80:]!r}")
            if not decoded.endswith(f" {LOG_RECORD_BYTES - 1} bytes in the infolog]"):
                failures.append(f"the marker doesn't size the line: {decoded[-80:]!r}")
        except UnicodeDecodeError as error:
            failures.append(f"the cut split a character: {error}")
    else:
        failures.append("the module did not hear the long line")
    if not any(LONG[:9] in line and len(line.encode()) > 32_000 for line in text.splitlines()):
        failures.append("the infolog lost the line")

    for name in ("base", "fonts"):
        (out / name).unlink(missing_ok=True)
    (out / "result.json").write_text(
        json.dumps(dict(passed=not failures, failures=failures, heard=heard), indent=2)
    )
    print("\n".join(failures) if failures else f"PASS console line fixture: {heard}")
    return int(bool(failures))


if __name__ == "__main__":
    raise SystemExit(main())
