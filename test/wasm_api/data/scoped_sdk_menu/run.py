#!/usr/bin/env python3
# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
"""Build one menu source for native/Wasm and exercise real Rml and game transitions."""

import argparse
import fcntl
import hashlib
import json
import os
import shutil
import signal
import subprocess
import tempfile
from pathlib import Path

from PIL import Image

FIXTURE = Path(__file__).resolve().parent
ROOT = FIXTURE.parents[3]
LEGACY = FIXTURE.parent / "rml_menu_fixture/transition"


def run(engine: Path, transport: str, output: Path, fail_init: bool = False) -> dict:
    target = ["--target", "wasm32-unknown-unknown"] if transport == "wasm" else []
    if fail_init:
        target += ["--features", "fail-init"]
    subprocess.run(
        [
            "cargo",
            "build",
            "--manifest-path",
            str(FIXTURE / "Cargo.toml"),
            "--release",
            *target,
        ],
        cwd=ROOT,
        check=True,
    )
    helper = "probe" if fail_init else "game"
    subprocess.run(
        [
            "cargo",
            "build",
            "--manifest-path",
            str(FIXTURE / helper / "Cargo.toml"),
            "--release",
            "--target",
            "wasm32-unknown-unknown",
        ],
        cwd=ROOT,
        check=True,
    )
    if output.exists():
        raise RuntimeError(f"evidence directory already exists: {output}")
    menu = output / "games/fixture.sdd"
    menu.mkdir(parents=True)
    for name in ["modinfo.lua", "LuaMenu", "WasmMenu"]:
        src = FIXTURE / name
        shutil.copytree(src, menu / name) if src.is_dir() else shutil.copy2(
            src, menu / name
        )
    shutil.copy2(LEGACY / "startscript.txt", menu / "startscript.txt")
    game = output / "games/RmlUi Menu Transition Game 0.1.sdd"
    shutil.copytree(LEGACY / "game.sdd", game)
    if not fail_init:
        wasm = game / "LuaRules/wasm"
        wasm.mkdir(parents=True, exist_ok=True)
        shutil.copy2(
            FIXTURE / "game/target/wasm32-unknown-unknown/release/scoped_sdk_game.wasm",
            wasm / "scoped_sdk_game.wasm",
        )
        (wasm / "manifest.txt").write_text(
            "module(scoped-sdk-game, LuaRules/wasm/scoped_sdk_game.wasm, rules-synced, 0, 1.0.0)\n"
        )
    for name in ["base", "fonts"]:
        shutil.copytree(engine.parent / name, output / name)
    if transport == "native":
        shutil.copy2(
            FIXTURE / "target/release/libscoped_sdk_menu.so",
            menu / "WasmMenu/libNativeMenu.so",
        )
        if not fail_init:
            (menu / "WasmMenu/manifest.json").unlink()
    else:
        shutil.copy2(
            FIXTURE / "target/wasm32-unknown-unknown/release/scoped_sdk_menu.wasm",
            menu / "WasmMenu/scoped_sdk_menu.wasm",
        )
    if fail_init:
        shutil.copy2(
            FIXTURE
            / "probe/target/wasm32-unknown-unknown/release/scoped_sdk_probe.wasm",
            menu / "WasmMenu/probe.wasm",
        )
        manifest = "module(sdk-cleanup-probe, WasmMenu/probe.wasm, menu, 0, 1.0.0)\n"
        if transport == "wasm":
            manifest += "module(scoped-sdk-menu, WasmMenu/scoped_sdk_menu.wasm, menu, 1, 1.0.0)\n"
        (menu / "WasmMenu/manifest.json").write_text(manifest)
    write = output / "write"
    write.mkdir()
    (write / "springsettings.cfg").write_text(
        "Sound = 0\nFullscreen = 0\nXResolutionWindowed = 960\nYResolutionWindowed = 640\nWindowBorderless = 1\n"
    )
    command = [
        "xvfb-run",
        "-a",
        str(engine),
        "--isolation",
        f"--isolation-dir={output}",
        f"--write-dir={write}",
        "--config",
        str(write / "springsettings.cfg"),
        "--window",
        "--menu=Scoped SDK Menu 1",
        "--hidden",
    ]
    (output / "fonts.conf").write_text(
        '<fontconfig><dir prefix="relative">fonts</dir><cachedir prefix="relative">fontcache</cachedir></fontconfig>'
    )
    env = os.environ.copy()
    env["LIBGL_ALWAYS_SOFTWARE"] = "1"
    env["LP_NUM_THREADS"] = "2"
    env["RAYON_NUM_THREADS"] = "2"
    env["FONTCONFIG_FILE"] = str(output / "fonts.conf")
    with (output / "stdout.log").open("w") as stdout:
        process = subprocess.Popen(
            command,
            cwd=ROOT,
            env=env,
            stdout=stdout,
            stderr=subprocess.STDOUT,
            start_new_session=True,
        )
        try:
            exit_code = process.wait(timeout=180)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGTERM)
            try:
                process.wait(timeout=10)
            except subprocess.TimeoutExpired:
                os.killpg(process.pid, signal.SIGKILL)
                process.wait()
            exit_code = 124
    log = (write / "infolog.txt").read_text(errors="replace")
    marker = (
        "Successfully loaded native module WasmMenu/libNativeMenu.so"
        if transport == "native"
        else "Wasm manifest loaded from WasmMenu/manifest.json"
    )
    expected = [
        marker,
        "[ScopedSdkProof] activation:1",
        "[ScopedSdkProof] activation:2",
        "[ScopedSdkProof] activation:3",
        "[ScopedSdkProof] typed-event:7",
        "[RmlUiMenuTransitionGame] returning to menu",
        "[ScopedSdkGame] initialized",
        "[ScopedSdkGame] callin",
        "[ScopedSdkGame] shutdown",
    ]
    settings = (write / "springsettings.cfg").read_text(errors="replace")
    missing = [s for s in expected if s not in log]
    if "shutdown:bindings-invalidated" not in settings:
        missing.append("shutdown:bindings-invalidated in saved config")
    if fail_init:
        rejection = (
            "Native menu module failed to initialize"
            if transport == "native"
            else "Core Wasm SDK initialization failed"
        )
        expected = [rejection, "[ScopedSdkProof] init-failure-cleaned"]
        missing = [s for s in expected if s not in log]
        if "[ScopedSdkProof] activation:" in log:
            missing.append("failed module must receive no activation callin")
        result = {
            "transport": transport,
            "exit": exit_code,
            "missing": missing,
            "evidence": str(output),
            "ok": exit_code == 0 and not missing,
        }
        (output / "result.json").write_text(json.dumps(result, indent=2) + "\n")
        return result
    image = write / "scoped-sdk-menu.png"
    pixel_hash = None
    if not image.is_file():
        missing.append("rendered menu screenshot")
    else:
        with Image.open(image) as rendered:
            rendered = rendered.convert("RGB")
            colors = {
                color: count
                for count, color in rendered.getcolors(rendered.width * rendered.height)
            }
            if (
                colors.get((23, 40, 56), 0) < 10000
                or colors.get((84, 190, 220), 0) < 100
            ):
                missing.append("visible Rml panel and title colors")
            pixel_hash = hashlib.sha256(rendered.tobytes()).hexdigest()
    failures = [
        line
        for line in log.splitlines()
        if any(
            s in line.lower()
            for s in [
                "panicked",
                "module fault",
                "callin failed",
                "trap:",
                "failed to initialize",
                "failed to instantiate",
            ]
        )
    ]
    result = {
        "transport": transport,
        "exit": exit_code,
        "missing": missing,
        "failures": failures,
        "evidence": str(output),
        "pixel_sha256": pixel_hash,
        "ok": exit_code == 0 and not missing and not failures,
    }
    (output / "result.json").write_text(json.dumps(result, indent=2) + "\n")
    return result


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--engine", type=Path, default=ROOT / "build-amd64-linux/install/spring"
    )
    parser.add_argument(
        "--transport", choices=["native", "wasm", "both"], default="both"
    )
    parser.add_argument(
        "--fail-init",
        action="store_true",
        help="reject an intentional initialization panic and verify owner cleanup",
    )
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    transports = ["native", "wasm"] if args.transport == "both" else [args.transport]
    # Cargo switches the same artifact between normal and fail-init features.
    # Hold the fixture lock through staging so concurrent invocations cannot
    # copy a different invocation's artifact after their build returns.
    key = hashlib.sha256(str(FIXTURE).encode()).hexdigest()[:16]
    lock_path = Path(tempfile.gettempdir()) / f"scoped-sdk-{os.getuid()}-{key}.lock"
    with lock_path.open("a") as lock:
        fcntl.flock(lock, fcntl.LOCK_EX)
        results = [
            run(
                args.engine.resolve(),
                name,
                args.output.resolve() / name,
                args.fail_init,
            )
            for name in transports
        ]
    if (
        len(results) == 2
        and not args.fail_init
        and results[0]["pixel_sha256"] != results[1]["pixel_sha256"]
    ):
        raise RuntimeError("native and Wasm menus rendered different pixels")
    print(json.dumps(results, indent=2))
    return 0 if all(r["ok"] for r in results) else 1


if __name__ == "__main__":
    raise SystemExit(main())
