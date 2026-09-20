#!/usr/bin/env python3
# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

from __future__ import annotations

import argparse
import os
from pathlib import Path
import shutil
import subprocess
import sys


ROOT = Path(__file__).resolve().parents[5]
FIXTURE = Path(__file__).resolve().parent.parent
DEFAULT_ENGINE = ROOT / "build-amd64-linux" / "install" / "spring"
NATIVE_LIBRARY_NAMES = {
	"linux": "libNativeMenu.so",
	"darwin": "libNativeMenu.dylib",
	"win32": "NativeMenu.dll",
}
NATIVE_OUTPUT_NAMES = {
	"linux": "librml_menu_native.so",
	"darwin": "librml_menu_native.dylib",
	"win32": "rml_menu_native.dll",
}


def copy_fixture(run_dir: Path, install_dir: Path, *, native: bool) -> None:
	menu = run_dir / "games" / "fixture.sdd"
	game = run_dir / "games" / "RmlUi Menu Transition Game 0.1.sdd"
	menu.mkdir(parents=True)
	shutil.copy2(FIXTURE / "transition" / "modinfo.lua", menu / "modinfo.lua")
	shutil.copytree(FIXTURE / "transition" / "LuaMenu", menu / "LuaMenu")
	shutil.copytree(FIXTURE / "WasmMenu", menu / "WasmMenu")
	shutil.copy2(FIXTURE / "transition" / "startscript.txt", menu / "startscript.txt")
	shutil.copytree(FIXTURE / "transition" / "game.sdd", game)
	shutil.copytree(install_dir / "base", run_dir / "base")
	shutil.copytree(install_dir / "fonts", run_dir / "fonts")
	(run_dir / "write").mkdir()
	(run_dir / "write" / "script.txt").write_text("", encoding="utf-8")

	if native:
		library_name = NATIVE_LIBRARY_NAMES[sys.platform]
		manifest = FIXTURE / "native" / "Cargo.toml"
		subprocess.run(
			["cargo", "build", "--manifest-path", str(manifest), "--release"],
			cwd=ROOT,
			check=True,
		)
		library = FIXTURE / "native" / "target" / "release" / NATIVE_OUTPUT_NAMES[sys.platform]
		shutil.copy2(library, menu / "WasmMenu" / library_name)
		(menu / "WasmMenu" / "manifest.json").unlink()


def main() -> int:
	parser = argparse.ArgumentParser(description=__doc__)
	parser.add_argument(
		"--engine",
		type=Path,
		default=Path(os.environ.get("SPRING_ENGINE", DEFAULT_ENGINE)),
		help="engine executable (default: build-amd64-linux/install/spring)",
	)
	parser.add_argument(
		"--run-dir",
		type=Path,
		default=Path("/tmp/rml-menu-transition"),
		help="directory used for the isolated run and retained log",
	)
	parser.add_argument(
		"--native",
		action="store_true",
		help="build and run the native menu transport instead of Core-WASM",
	)
	args = parser.parse_args()

	if not args.engine.is_file():
		parser.error(f"missing engine: {args.engine}")
	if sys.platform not in {"linux", "darwin", "win32"}:
		parser.error(f"unsupported platform: {sys.platform}")

	install_dir = args.engine.resolve().parent
	shutil.rmtree(args.run_dir, ignore_errors=True)
	copy_fixture(args.run_dir, install_dir, native=args.native)

	command = [
		"xvfb-run",
		"-a",
		str(args.engine),
		"--isolation",
		f"--isolation-dir={args.run_dir}",
		f"--write-dir={args.run_dir / 'write'}",
		"--window",
		"--menu=Rml Menu Transition 1",
		"--hidden",
	]
	print("+", " ".join(command), flush=True)
	result = subprocess.run(command, cwd=ROOT)

	log_path = args.run_dir / "write" / "infolog.txt"
	log = log_path.read_text(encoding="utf-8", errors="replace") if log_path.is_file() else ""
	transport_marker = (
		f"Successfully loaded native module WasmMenu/{NATIVE_LIBRARY_NAMES[sys.platform]}"
		if args.native
		else "Wasm manifest loaded from WasmMenu/manifest.json"
	)
	expected = (
		transport_marker,
		"[RmlUiMenuTransition] menu activation 1",
		"[RmlUiMenuTransition] starting game cycle 1",
		"[RmlUiMenuTransitionGame] returning to menu",
		"[RmlUiMenuTransition] menu activation 2",
		"[RmlUiMenuTransition] starting game cycle 2",
		"[RmlUiMenuTransitionGame] returning to menu",
		"[RmlUiMenuTransition] menu activation 3",
		"[RmlUiMenuTransition] completed two menu-game-menu cycles",
		"[MapGen::CBlankMapGenerator] mapOpt<new_map_x,10>",
		"[MapGen::CBlankMapGenerator] mapOpt<new_map_y,8>",
	)
	missing = [marker for marker in expected if marker not in log]
	if missing:
		print("transition validation failed; missing:", file=sys.stderr)
		for marker in missing:
			print(f"  {marker}", file=sys.stderr)
		return 1

	print(f"transition markers passed; log: {log_path}")
	if result.returncode != 0:
		print(
			f"engine exited with status {result.returncode}; inspect ASAN output for the retained run",
			file=sys.stderr,
		)
	return result.returncode


if __name__ == "__main__":
	raise SystemExit(main())
