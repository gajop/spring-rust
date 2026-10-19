# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
"""Check automation failure/resume behavior without launching an engine."""
import argparse
import json
from pathlib import Path
import tempfile
import unittest
from unittest import mock
import wasm_adversarial
import wasm_sync_suite


class SuiteTest(unittest.TestCase):
    def setUp(self):
        temp = tempfile.TemporaryDirectory()
        self.addCleanup(temp.cleanup)
        self.root = Path(temp.name)
        self.engine = self.root/'spring-headless'
        self.engine.write_bytes(b'engine')
        for name in ['run_harness.py','spec_loader.py']:
            file = self.root/'test/native_api_parity'/name
            file.parent.mkdir(parents=True,exist_ok=True)
            file.write_text('fixture source')
        self.output = self.root/'results'

    def invoke(self, *, resume=False, fail=False, source_change=False):
        def fake_run(command, **kwargs):
            out = Path(command[command.index('--output')+1])
            out.mkdir()
            (out/'result.json').write_text(json.dumps(dict(passed=not fail)))
            (out/'sync.jsonl').write_text('raw evidence')
            if source_change:
                (self.root/'test/native_api_parity/spec_loader.py').write_text('changed')
            return argparse.Namespace(returncode=1 if fail else 0)
        argv = ['suite','--spring',str(self.engine),'--output',str(self.output)]
        if resume:
            argv.append('--resume')
        with mock.patch('sys.argv',argv), mock.patch.object(wasm_sync_suite,'ROOT',self.root), \
             mock.patch.object(wasm_sync_suite.subprocess,'run',side_effect=fake_run) as run:
            status = wasm_sync_suite.main()
            return status, run.call_count

    def test_pass_resume_and_corrupted_evidence_retries_without_deleting(self):
        self.assertEqual(self.invoke(), (0, 4))
        self.assertEqual(self.invoke(resume=True), (0, 0))
        original = next(self.output.glob('rules-*/attempt-0000/sync.jsonl'))
        original.write_text('corrupted')
        self.assertEqual(self.invoke(resume=True), (0, 1))
        self.assertEqual(original.read_text(), 'corrupted')
        self.assertTrue((original.parent.parent/'attempt-0001/result.json').exists())
        self.assertTrue(json.loads((self.output/'result.json').read_text())['complete'])

    def test_failures_are_not_skipped_or_reported_as_passes(self):
        self.assertEqual(self.invoke(fail=True), (1, 4))
        result = json.loads((self.output/'result.json').read_text())
        self.assertFalse(result['passed'])
        self.assertEqual(self.invoke(resume=True), (0, 4))
        self.assertEqual(len(list(self.output.glob('*/attempt-0000/result.json'))), 4)

    def test_resume_refuses_changed_inputs(self):
        self.invoke()
        self.engine.write_bytes(b'different engine')
        with self.assertRaises(SystemExit):
            self.invoke(resume=True)

    def test_source_mutation_during_run_stops_before_next_case(self):
        with self.assertRaisesRegex(RuntimeError,'source changed'):
            self.invoke(source_change=True)
        result = json.loads((self.output/'result.json').read_text())
        self.assertFalse(result['complete'])
        self.assertFalse(result['passed'])


class ValidationTest(unittest.TestCase):
    def test_public_content_hash_is_portable_in_case_and_path_separators(self):
        import hashlib
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            base = root/'test/native_api_parity/fixtures/game.sdd'
            base.mkdir(parents=True)
            for name in ('a.lua','B.lua'):
                (base/name).write_bytes(name.encode())
            expected = hashlib.sha256()
            # This order is case sensitive on Windows too; native Path sorting
            # there folds case and would reverse these two names.
            for name in ('B.lua','a.lua'):
                expected.update(('test/native_api_parity/fixtures/game.sdd/'+name).encode())
                expected.update(hashlib.sha256(name.encode()).digest())
            with mock.patch.object(wasm_adversarial,'ROOT',root):
                self.assertEqual(wasm_adversarial.public_content_hash(),expected.hexdigest())

    def test_all_modules_must_reach_the_last_marker(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root/'console.log').write_text('[SyncAdversarial] initialized\n[SyncAdversarial] frame=60 callbacks=2\n')
            args = argparse.Namespace(rules_modules=2,gaia_modules=0,frames=90)
            with self.assertRaisesRegex(RuntimeError,'expected 2 module markers'):
                wasm_adversarial.validate_run(args,root)
