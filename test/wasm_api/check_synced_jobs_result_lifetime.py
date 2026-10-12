#!/usr/bin/env python3
# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
"""A result handler must accept a callback borrow, not require permanent bytes."""
from pathlib import Path
import subprocess

fixture = Path(__file__).resolve().parent / 'guests/synced_jobs_result_lifetime'
command = ['cargo', 'check', '--locked', '--target', 'wasm32-unknown-unknown']
accepted = subprocess.run(command, cwd=fixture, capture_output=True, text=True)
if accepted.returncode:
    raise SystemExit(f'ordinary borrowed handler failed:\n{accepted.stderr}')
rejected = subprocess.run(command + ['--features', 'retain-borrowed-result'],
                          cwd=fixture, capture_output=True, text=True)
if not rejected.returncode or 'error[E0308]' not in rejected.stderr or "&'static" not in rejected.stderr:
    raise SystemExit(f'permanent-borrow rejection missing or failed for another reason:\n{rejected.stderr}')
print('PASS: ordinary callback borrow accepted; permanent-borrow handler rejected by Rust')
print(rejected.stderr)
