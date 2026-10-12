# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
import copy
import json
from pathlib import Path
import tempfile
import unittest
from unittest import mock
import wasm_sync_replay

from wasm_sync_replay import VerificationError, first_difference, load_rows, memory_difference, mount_game


class CompareTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.rows = [dict(frame=f, synccheck=True, engine=3, checksum=4, modules=[
            dict(name='rules', identity='a' * 128, environment='rules-synced',
                 bytes=65536, hash=f'{f:016x}', faulted=False)]) for f in range(4)]

    def write(self, name, rows):
        path = self.root / name
        path.write_text(''.join(json.dumps(row) + '\n' for row in rows))
        return path

    def test_first_guest_desync_even_when_engine_also_differs(self):
        other = copy.deepcopy(self.rows)
        other[2]['modules'][0]['hash'] = 'f' * 16
        other[2]['engine'] = 99
        result = first_difference(self.write('a', self.rows), self.write('b', other))
        self.assertEqual((result['frame'], result['module']), (2, 'rules'))

    def test_engine_only_desync(self):
        other = copy.deepcopy(self.rows)
        other[3]['engine'] = 99
        result = first_difference(self.write('a', self.rows), self.write('b', other))
        self.assertEqual((result['frame'], result['module']), (3, 'engine'))

    def test_truncated_or_missing_frame_fails(self):
        result = first_difference(self.write('a', self.rows), self.write('b', self.rows[:2]))
        self.assertEqual((result['frame'], result['module']), (2, 'stream'))
        with self.assertRaises(VerificationError):
            list(load_rows(self.write('gap', [self.rows[0], self.rows[2]])))

    def test_faulted_empty_and_no_synccheck_fail(self):
        for field, value in [('synccheck', False), ('modules', [])]:
            other = copy.deepcopy(self.rows)
            other[0][field] = value
            with self.assertRaises(VerificationError):
                list(load_rows(self.write('bad', other)))
        other = copy.deepcopy(self.rows)
        other[0]['modules'][0]['faulted'] = True
        with self.assertRaises(VerificationError):
            list(load_rows(self.write('fault', other)))
        with self.assertRaises(VerificationError):
            list(load_rows(self.write('empty', [])))

    def test_equal(self):
        self.assertEqual(first_difference(self.write('a', self.rows), self.write('b', self.rows)),
                         dict(equal=True, frames=4))

    def test_memory_offsets_and_pages(self):
        a, b = self.root / 'a', self.root / 'b'
        a.write_bytes(bytes(65537))
        b.write_bytes(bytes(65536) + b'\1')
        result = memory_difference(a, b)
        self.assertEqual(result['pages'], [1])
        self.assertEqual(result['first_offsets'], [65536])

    def test_fixture_overlay_never_writes_through_to_the_pinned_game(self):
        source = self.root / 'source.sdd'
        wasm = source / 'LuaRules/wasm'
        wasm.mkdir(parents=True)
        original_manifest = 'module(rules, LuaRules/wasm/rules.wasm, rules-synced, 0, 1.0.0)\n'
        (wasm / 'manifest.txt').write_text(original_manifest)
        (wasm / 'rules.wasm').write_bytes(b'original rules')
        (wasm / 'sync-jobs-test.wasm').write_bytes(b'old fixture')
        (wasm / 'sync-jobs-worker.wasm').write_bytes(b'old worker')
        fixture, worker = self.root / 'fixture', self.root / 'worker'
        fixture.write_bytes(b'new fixture')
        worker.write_bytes(b'new worker')
        target = self.root / 'mounted.sdd'
        mount_game(source, target, fixture, worker)
        self.assertEqual((wasm / 'manifest.txt').read_text(), original_manifest)
        self.assertEqual((wasm / 'sync-jobs-test.wasm').read_bytes(), b'old fixture')
        self.assertEqual((wasm / 'sync-jobs-worker.wasm').read_bytes(), b'old worker')
        mounted = target / 'LuaRules/wasm'
        self.assertEqual((mounted / 'rules.wasm').read_bytes(), b'original rules')
        self.assertEqual((mounted / 'sync-jobs-test.wasm').read_bytes(), b'new fixture')
        self.assertEqual((mounted / 'sync-jobs-worker.wasm').read_bytes(), b'new worker')
        self.assertIn('module(sync-jobs-worker,', (mounted / 'manifest.txt').read_text())
        self.assertIn('module(sync-jobs-test,', (mounted / 'manifest.txt').read_text())


if __name__ == '__main__':
    unittest.main()


class RunnerDiagnosticTest(unittest.TestCase):
    setUp = CompareTest.setUp
    def invoke(self, different, record=False):
        engine = self.root / 'spring-headless'
        engine.write_bytes(b'test engine')
        demo = self.root / 'demo.sdfz'
        demo.write_bytes(b'test demo')
        game = self.root / 'game.sdd'
        game.mkdir()
        output = self.root / 'run'
        def fake_replay(args, threads, run, dump_frame=-1, record=False, **kwargs):
            run.mkdir()
            (run / 'write').mkdir()
            (run / 'write/infolog.txt').write_text('' if record else '[DESYNC WARNING] coarse checksum')
            rows = copy.deepcopy(self.rows)
            if different and not record and (args.record_first or threads == 4):
                rows[2]['modules'][0]['hash'] = 'f' * 16
            log = run / 'sync.jsonl'
            log.write_text(''.join(json.dumps(row) + '\n' for row in rows))
            return (log, demo) if record else log
        argv = ['runner', 'run', '--spring', str(engine), '--game', str(game),
                '--demo', str(demo), '--output', str(output), '--threads', '1', '4']
        if record:
            argv.append('--record-first')
        with mock.patch('sys.argv', argv), mock.patch.object(wasm_sync_replay, 'replay', side_effect=fake_replay) as replay:
            result = wasm_sync_replay.main()
            return result, json.loads((output / 'result.json').read_text()), replay.call_count

    def test_coarse_warning_does_not_hide_first_module_difference(self):
        status, result, calls = self.invoke(different=True)
        self.assertEqual((status, result['frame'], result['module'], calls), (1, 2, 'rules', 2))

    def test_equal_streams_with_recorded_warnings_still_fail(self):
        with self.assertRaisesRegex(VerificationError, 'recorded replay desync warning'):
            self.invoke(different=False)

    def test_recording_comparison_reports_all_frame_state_difference(self):
        status, result, calls = self.invoke(different=True, record=True)
        self.assertEqual((status, result['frame'], result['module'], calls), (1, 2, 'rules', 2))
        self.assertEqual(result['comparison'], 'recording versus playback')
