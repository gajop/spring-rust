# Native stub regressions

A small rendered integration fixture for the concrete defects in
`rts/wasm/docs/recoil_wasm_no_stubs_plan.md`. It creates four units on an 8×8
blank map and quits after checking the first factory construction. It does not
run a performance benchmark or the full API parity suite.

Build `engine-legacy` first, then run from the repository root (C++ compiler,
SDL2, OpenGL headers, and JsonCpp development packages are required):

```sh
SDL_VIDEODRIVER=offscreen LIBGL_ALWAYS_SOFTWARE=1 LP_NUM_THREADS=1 \
  python3 test/native_stub_regressions/run.py --output /tmp/native-stub-regressions
```

Omit `LIBGL_ALWAYS_SOFTWARE=1` to use the available hardware driver. Use
`--spring /absolute/path/to/spring` for a different build. Output includes the
start script, settings, compiled native module, and engine log. The runner owns
and terminates only its own engine process, with a 30-second timeout.

The fixture requires:

- movement results matching Lua for open and blocked squares and object-test
  options, plus an invalid definition;
- distinct actual shields in slots 2 and 3, a non-shield in slot 1, and Lua's
  designated-shield fallback for nonpositive and out-of-range slots;
- nonzero weapon turn rate, actual piece-projectile model metadata, hovercraft
  classification distinct from aircraft `hoverAttack`, and factory construction
  changing from absent to present;
- nonempty team/player custom options, encoded as JSON in the existing string
  ABI, and absence when team keys are not requested;
- logging to the requested section and level (the one intentional warning is
  `[NativeStubAudit] Warning: audit section/level 100%`);
- framebuffer pixel readback proving the selected READ object was cleared,
  another DRAW object retained its pixels, bindings were restored, the default
  framebuffer was refused, integer clears work, and invalid input is rejected.

Each observation is replayed against a zero-return replacement in a private
copy of the API tables. All ten audited functions must have a detected mutant.
The engine's API tables are never modified. Generated unhandled callin exports
preserve the host's defaults; they are module-loader plumbing, not the callouts
under test.

This is a targeted native gate. It does not implement the planned global static
stub checker, per-field vacuity ledger, success-no-op setter mutants, or the
Wasm environment matrix, and does not mark generated APIs `verified`.
