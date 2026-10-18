# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
"""Configure a private benchmark fixture without modifying its source."""
from pathlib import Path
import shutil


def configure(game: Path) -> None:
    if game.is_symlink():
        source = game.resolve(strict=True)
        game.unlink()
        shutil.copytree(source, game, symlinks=True)
    lua = game / "LuaRules/Gadgets/wasm_benchmark.lua"
    text = lua.read_text()
    # SDK native workloads advance through real GameFrame callbacks. The
    # process timeout still applies, and completion uses the existing message.
    text = text.replace('elseif backend == "native" and benchmarkCase ~= "heightmap"',
                        'elseif false and backend == "native" and benchmarkCase ~= "heightmap"')
    lua.write_text(text)
