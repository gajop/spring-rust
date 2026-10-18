# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
import tempfile
import unittest
from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from sdk_benchmark_fixture import configure


class SdkBenchmarkFixtureTests(unittest.TestCase):
    def test_native_symlink_does_not_change_the_source_fixture(self):
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            source = root / "source"
            relative = Path("LuaRules/Gadgets/wasm_benchmark.lua")
            (source / relative).parent.mkdir(parents=True)
            original = 'elseif backend == "native" and benchmarkCase ~= "heightmap"\n'
            (source / relative).write_text(original)
            game = root / "staged.sdd"
            game.symlink_to(source, target_is_directory=True)
            configure(game)
            self.assertFalse(game.is_symlink())
            self.assertEqual((source / relative).read_text(), original)
            self.assertIn('elseif false and backend == "native"', (game / relative).read_text())


if __name__ == "__main__":
    unittest.main()
