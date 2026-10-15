# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
"""Small offline checks: no engine, renderer, cargo build, or benchmark."""
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import parity_ledger as ledger
import run_harness as harness
from spec_loader import required_compare_fields, validate_tests
from sweep_mutants import target_failures


class SpecTests(unittest.TestCase):
    def test_missing_fields_are_required_by_default(self):
        self.assertEqual(required_compare_fields({'compare': {'fields': ['x', 'y']}}), ['x', 'y'])

    def test_optional_field_needs_reason_and_declared_field(self):
        for optional in ({'x': ''}, {'y': 'not returned'}):
            with self.assertRaises(ValueError):
                validate_tests([{'id': 'query', 'compare': {'fields': ['x'], 'optional_fields': optional}}])

    def test_valid_optional_field_is_not_required(self):
        spec = {'id': 'query', 'compare': {'fields': ['x', 'y'], 'optional_fields': {'y': 'nil for invalid IDs'}}}
        validate_tests([spec])
        self.assertEqual(required_compare_fields(spec), ['x'])


class LedgerTests(unittest.TestCase):
    def entry(self):
        return {'level': 'untested', 'grandfathered': True, 'required_environments': ['rules-synced'],
                'tests': ['query'], 'fields': {}, 'runs': [], 'required_returns_mapped': False,
                'required_mutants': ['zero'], 'killed_mutants': []}

    def test_defaults_include_nested_zero_records(self):
        self.assertTrue(ledger.default({'position': [0, 0, 0], 'name': ''}))
        self.assertFalse(ledger.default({'position': [0, 2, 0]}))

    def test_new_identity_cannot_be_grandfathered(self):
        self.assertTrue(ledger.gate({'functions': {'unit.Query': self.entry()}}, {'functions': {}}))

    def test_grandfathering_cannot_be_reintroduced(self):
        entry = self.entry()
        before = dict(entry, grandfathered=False)
        self.assertTrue(ledger.gate({'functions': {'unit.Query': entry}}, {'functions': {'unit.Query': before}}))

    def test_exemption_cannot_hide_missing_evidence_without_reason(self):
        entry = dict(self.entry(), level='exempt')
        self.assertTrue(ledger.gate({'functions': {'unit.Query': entry}}, {'functions': {'unit.Query': self.entry()}}))

    def test_verification_rejects_invariant_fields(self):
        entry = dict(self.entry(), level='verified', required_returns_mapped=True,
                     killed_mutants=['zero'], wasm_compared_environments=['rules-synced'], runs=[{'path': 'test'}],
                     fields={'x': {'observations': 3, 'missing': 0, 'nondefault': 3, 'distinct': 1, 'recorded_comparisons': 3}})
        self.assertTrue(ledger.gate({'functions': {'unit.Query': entry}}, {'functions': {'unit.Query': self.entry()}}))

    def test_passing_default_or_skipped_comparisons_are_only_exercised(self):
        for values, compared in (([0, 0], True), ([1, 2], False)):
            with self.subTest(values=values, compared=compared), tempfile.TemporaryDirectory() as directory:
                run = Path(directory)
                (run / 'lua').mkdir()
                (run / 'native').mkdir()
                (run / 'report.md').write_text('- Result: PASS\n')
                rows = [{'testName': 'query', 'context': 'synced_gadget', 'x': value} for value in values]
                (run / 'lua/synced_gadget.jsonl').write_text(''.join(json.dumps(row) + '\n' for row in rows))
                checks = [{'test': 'query', 'field': 'x', 'kind': 'value', 'compared': compared} for _ in values]
                (run / 'native/comparisons.jsonl').write_text(''.join(json.dumps(row) + '\n' for row in checks))
                entry = dict(self.entry(), required_returns_mapped=True)
                result = ledger.observe({'functions': {'unit.Query': entry}},
                                        [{'id': 'query', 'compare': {'fields': ['x']}}], run)
                self.assertEqual(result['functions']['unit.Query']['level'], 'exercised')


class ProbeTests(unittest.TestCase):
    def test_fingerprint_binds_rendering_sdk_and_selection(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            crate = root / 'probe'
            (crate / 'src').mkdir(parents=True)
            sdk = root / 'rts/wasm/generated/sdk'
            sdk.mkdir(parents=True)
            for path in (crate / 'Cargo.toml', crate / 'Cargo.lock', crate / 'src/probe.rs',
                         root / 'manifest.json', root / 'probe.lua', root / 'rts/wasm/generated/model.json', sdk / 'api.rs'):
                path.write_text('{}')
            with patch.multiple(harness, ROOT=root, WASM_PARITY_CRATE=crate / 'Cargo.toml',
                                WASM_CONTEXT_CORE_MANIFESTS={'ui': root / 'manifest.json'},
                                WASM_CONTEXT_CORE_LUA_OUTPUTS={'ui': root / 'probe.lua'}):
                before = harness.probe_build_fingerprint('ui', False)
                self.assertEqual(before, harness.probe_build_fingerprint('ui', False))
                self.assertNotEqual(before, harness.probe_build_fingerprint('ui', True))
                (sdk / 'api.rs').write_text('changed API')
                self.assertNotEqual(before, harness.probe_build_fingerprint('ui', False))

    def test_wasm_crash_or_missing_values_are_not_mutant_kills(self):
        with tempfile.TemporaryDirectory() as directory:
            run = Path(directory)
            self.assertEqual(target_failures(run, ['query'], 'wasm'), [])
            mismatch = {'testName': 'query', 'expected': {'status': 'pass', 'fields': {'x': 2}},
                        'actual': {'status': 'pass', 'fields': {}}, 'missing_required_fields': ['x']}
            (run / 'comparison.json').write_text(json.dumps({'api_mismatches': [mismatch]}))
            self.assertEqual(target_failures(run, ['query'], 'wasm'), [])
            mismatch['actual']['fields'] = {'x': 0}
            mismatch['differing_fields'] = ['x']
            (run / 'comparison.json').write_text(json.dumps({'api_mismatches': [mismatch]}))
            self.assertEqual(target_failures(run, ['query'], 'wasm'), ['query'])


if __name__ == '__main__':
    unittest.main()
