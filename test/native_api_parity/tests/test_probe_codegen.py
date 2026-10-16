# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
"""Probe generation must consume the supplied model and companion metadata."""

import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[3]
GENERATOR = ROOT / 'test/wasm_api/guests/parity_guest/generate_probe.py'


class ProbeCodegenTests(unittest.TestCase):
    def test_external_model_and_metadata(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            sdk = root / 'sdk'
            sdk.mkdir()
            (sdk / 'core_owned.rs').write_text('')
            (sdk / 'core_owned').mkdir()
            shard = sdk / 'core_owned/game.rs'
            shard.write_text(
                '    pub mod game {\n'
                '        pub fn get_game_frame() {}\n'
                '    }\n'
                '    #[doc(hidden)]\n'
            )
            (sdk / 'core_owned_footer.rs').write_text('')
            model = root / 'model.json'
            model.write_text(json.dumps({'modules': [{
                'name': 'game',
                'functions': [{
                    'name': 'GetGameFrame',
                    'inputs': [],
                    'outputs': [
                        {'name': name, 'type': {'kind': 'scalar', 'name': 'u32'}}
                        for name in ('low16', 'high16')
                    ],
                }],
            }]}))
            coverage = root / 'core-executable-coverage.json'
            entry = {
                'module': 'game', 'function': 'GetGameFrame',
                'production_import_allowed': True,
                'production_process_safe': True,
                'production_environment_mask': 1,
            }
            coverage.write_text(json.dumps({'executable': [entry]}))
            command = [sys.executable, str(GENERATOR), '--model', str(model),
                       '--output-root', str(root / 'probe'),
                       '--lua-output', str(root / 'probe.lua')]

            result = subprocess.run(command, cwd=ROOT, capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            manifest = json.loads((root / 'probe/probe_manifest.json').read_text())
            self.assertEqual(manifest['tests'], ['game_frame'])

            # Both policy and façade changes in the external tree must affect
            # selection even when the checkout advertises this function.
            entry['production_environment_mask'] = 0
            coverage.write_text(json.dumps({'executable': [entry]}))
            result = subprocess.run(command, cwd=ROOT, capture_output=True, text=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn('selected no tests', result.stderr)

            entry['production_environment_mask'] = 1
            coverage.write_text(json.dumps({'executable': [entry]}))
            shard.write_text('')
            result = subprocess.run(command, cwd=ROOT, capture_output=True, text=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn('selected no tests', result.stderr)


if __name__ == '__main__':
    unittest.main()
