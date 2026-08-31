#!/usr/bin/env python3
# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

from __future__ import annotations

import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile


ROOT = Path(__file__).resolve().parents[3]
ENGINE = Path(os.environ.get("SPRING_CUS_E2E_ENGINE", ROOT / "build-cus" / "spring"))
BASE = ROOT / "cont" / "base"
SOURCE_FIXTURE = ROOT / "test" / "native_api_parity" / "fixtures" / "game.sdd"
NATIVE_MANIFEST = ROOT / "test" / "cus" / "e2e" / "native" / "Cargo.toml"
CORE_MANIFEST = ROOT / "test" / "cus" / "e2e" / "core" / "Cargo.toml"
NATIVE_LIBRARY = NATIVE_MANIFEST.parent / "target" / "release" / "librust_cus_e2e_native.so"
CORE_WASM = (
    CORE_MANIFEST.parent
    / "target"
    / "wasm32-unknown-unknown"
    / "release"
    / "rust_cus_e2e_core.wasm"
)


def run(command: list[str], *, cwd: Path = ROOT, env: dict[str, str] | None = None) -> None:
    print("+", " ".join(command), flush=True)
    subprocess.run(command, cwd=cwd, env=env, check=True)


def build_artifacts() -> None:
    if not ENGINE.is_file():
        raise RuntimeError(f"missing engine: {ENGINE}; build build-cus engine-legacy first")
    run(["cargo", "build", "--manifest-path", str(NATIVE_MANIFEST), "--release"])
    run(
        [
            "cargo",
            "build",
            "--manifest-path",
            "Cargo.toml",
            "--target",
            "wasm32-unknown-unknown",
            "--release",
        ],
        cwd=CORE_MANIFEST.parent,
    )


def link_or_copy(source: Path, destination: Path) -> None:
    try:
        destination.symlink_to(source)
    except OSError:
        if source.is_dir():
            shutil.copytree(source, destination)
        else:
            shutil.copy2(source, destination)


def make_game(workdir: Path, core: bool) -> Path:
    datadir = workdir / "data"
    game = datadir / "games" / "rust_cus_e2e.sdd"
    (datadir / "games").mkdir(parents=True)
    (datadir / "maps").mkdir()
    (game / "gamedata").mkdir(parents=True)
    (game / "units").mkdir()
    (game / "scripts").mkdir()
    (game / "LuaRules" / "Gadgets").mkdir(parents=True)
    if core:
        (game / "LuaRules" / "wasm").mkdir(parents=True)

    for relative in (
        "gamedata/modrules.lua",
        "gamedata/resources.lua",
        "gamedata/sidedata.lua",
        "LuaRules/main.lua",
        "LuaRules/draw.lua",
        "units/native_api_test_unit.lua",
        "scripts/native_api_test_unit.lua",
    ):
        shutil.copy2(SOURCE_FIXTURE / relative, game / relative)

    (game / "modinfo.lua").write_text(
        """return {
    name = "Rust CUS E2E",
    shortName = "RustCUSE2E",
    version = "0.1",
    game = "Rust CUS E2E",
    shortGame = "CUS",
    description = "Rust CUS end-to-end fixture",
    modtype = 1,
    depend = { "Spring content v1" },
}
""",
        encoding="utf-8",
    )
    (game / "LuaRules" / "Gadgets" / "cus_e2e.lua").write_text(
        """function gadget:GetInfo()
    return {
        name = "Rust CUS E2E unit creator",
        desc = "Creates one real unit for the CUS transport fixture",
        author = "Spring",
        layer = 0,
        enabled = true,
    }
end

if not gadgetHandler:IsSyncedCode() then
    return false
end

local created = false

local function createFixtureUnit()
    if created then
        return
    end
    created = true
    Spring.CreateUnit("native_api_test_unit", 128, 0, 128, 0, 0, false, false)
end

function gadget:GameStart()
    createFixtureUnit()
end

function gadget:GameFrame(frame)
    if frame == 1 then
        createFixtureUnit()
    end
end
""",
        encoding="utf-8",
    )
    if core:
        shutil.copy2(CORE_WASM, game / "LuaRules" / "wasm" / "cus_e2e_core.wasm")
        (game / "LuaRules" / "wasm" / "manifest.txt").write_text(
            "module(cus-e2e, LuaRules/wasm/cus_e2e_core.wasm, rules-synced, 0, 1.0.0)\n",
            encoding="utf-8",
        )

    for archive in ("springcontent", "maphelper", "bitmaps", "cursors"):
        link_or_copy(BASE / archive, datadir / "games" / f"{archive}.sdd")
    return datadir


def make_script(workdir: Path) -> Path:
    script = workdir / "cus_e2e_script.txt"
    script.write_text(
        """[GAME]
{
    IsHost=1;
    MyPlayerName=RustCusE2E;
    MapName=rust_cus_e2e_blank;
    GameType=Rust CUS E2E 0.1;
    InitBlank=1;
    StartPosType=0;
    FixedRNGSeed=1;
    OnlyLocal=1;
    HostIP=localhost;
    HostPort=8452;
    MyPlayerNum=0;
    RecordDemo=0;
    GameStartDelay=0;
    MaxSpeed=100;
    MinSpeed=100;
    NumPlayers=1;
    NumTeams=2;
    NumAllyTeams=2;
    [MAPOPTIONS]
    {
        blank_map_x=10;
        blank_map_y=8;
        blank_map_height=0;
        blank_map_color_r=64;
        blank_map_color_g=128;
        blank_map_color_b=64;
    }

    [MODOPTIONS]
    {
        LuaRules=1;
        LuaGaia=1;
    }

    [PLAYER0]
    {
        Name=RustCusE2E;
        Spectator=0;
        Team=0;
    }

    [TEAM0]
    {
        TeamLeader=0;
        AllyTeam=0;
        RGBColor=1 1 1;
        Side=Arm;
    }

    [TEAM1]
    {
        TeamLeader=0;
        AllyTeam=1;
        RGBColor=1 0 0;
        Side=Arm;
    }

    [ALLYTEAM0]
    {
        NumAllies=0;
    }

    [ALLYTEAM1]
    {
        NumAllies=0;
    }
}
""",
        encoding="utf-8",
    )
    return script


def run_engine(workdir: Path, datadir: Path, script: Path, mode: str, core: bool) -> None:
    output = workdir / f"{mode}-events.log"
    write_dir = workdir / f"{mode}-write"
    write_dir.mkdir(parents=True)
    (write_dir / "springsettings.cfg").write_text(
        "Sound=0;\nFullscreen=0;\nWindowBorderless=0;\nVSync=0;\n",
        encoding="utf-8",
    )
    env = os.environ.copy()
    env["SPRING_DATADIR"] = os.pathsep.join((str(datadir), str(BASE), str(ROOT / "cont")))
    env["SPRING_ISOLATED"] = str(datadir)
    env["SPRING_CUS_E2E_MODE"] = mode
    env["SPRING_CUS_E2E_OUTPUT"] = str(output)
    env["SPRING_WASM_CORE_HOST"] = "1" if core else ""
    env["SPRING_NATIVE_MODULE"] = str(NATIVE_LIBRARY)
    log = workdir / f"{mode}-spring.log"
    command = [str(ENGINE), "--window", "--nocolor", "--write-dir", str(write_dir), str(script)]
    print("+", " ".join(command), flush=True)
    with log.open("wb") as stream:
        completed = subprocess.run(
            command,
            cwd=ROOT,
            env=env,
            stdout=stream,
            stderr=subprocess.STDOUT,
            timeout=90,
            check=False,
        )
    if completed.returncode != 0:
        raise RuntimeError(f"{mode} engine exited with {completed.returncode}; see {log}")


def assert_markers(workdir: Path) -> None:
    native = (workdir / "native-events.log").read_text(encoding="utf-8")
    required_native = (
        "native|attached|",
        "native|create",
        "native|named|found=1|success=1|value=4",
        "native|tick|",
        "task_resumed=1",
        "native|detach",
        "native|shutdown",
    )
    for marker in required_native:
        if marker not in native:
            raise AssertionError(f"native marker missing: {marker!r}\n{native}")

    core_log = (workdir / "core-write" / "infolog.txt").read_text(encoding="utf-8")
    core_native = (workdir / "core-events.log").read_text(encoding="utf-8")
    required_core = (
        "CUS_E2E|core|attached|",
        "CUS_E2E|core|create",
        "CUS_E2E|core|named|found=1|success=1|value=4",
        "CUS_E2E|core|tick|",
        "task_resumed=1",
        "CUS_E2E|core|detach",
    )
    for marker in required_core:
        if marker not in core_log:
            raise AssertionError(f"Core-Wasm marker missing: {marker!r}\n{core_log}")
    for marker in (
        "native|core-named|found=1|success=1|value=4",
        "native|shutdown",
    ):
        if marker not in core_native:
            raise AssertionError(f"Core native-host marker missing: {marker!r}\n{core_native}")

    print(native, end="")
    for line in core_log.splitlines():
        if "CUS_E2E|core|" in line:
            print(line)
    for line in core_native.splitlines():
        if line.startswith("native|core-") or line == "native|shutdown":
            print(line)


def main() -> int:
    build_artifacts()
    workdir = Path(tempfile.mkdtemp(prefix="spring-rust-cus-e2e-"))
    try:
        script = make_script(workdir)
        native_data = make_game(workdir / "native", core=False)
        core_data = make_game(workdir / "core", core=True)
        run_engine(workdir, native_data, script, "native", core=False)
        run_engine(workdir, core_data, script, "core", core=True)
        assert_markers(workdir)
    except Exception:
        print(f"fixture workdir retained for diagnosis: {workdir}", file=sys.stderr)
        raise
    else:
        shutil.rmtree(workdir)
    print("Rust CUS native/Core-Wasm end-to-end fixture passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
