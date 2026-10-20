# LocalModel transform differential check

Run `check.py --help` for the required baseline, candidate, complete math-source
tree, engine-built streflop archive, and output paths. Compilation is sequential
and takes a shared `/tmp/shipgame-profiling-1000.lock` lock. Use the same compiler
and floating-point options as the engine being evaluated. The default options
match the x86-64 SSE deterministic engine math settings.

The check extracts each tree's real `LocalModelPiece` update methods,
`LocalModel::SetModel`, hierarchy construction and boundary update methods, and
`S3DModelPiece::ComposeTransform`. It links the actual vector, quaternion, matrix
and transform arithmetic. Minimal static geometry and collision-volume storage
replace dependencies on the renderer and game initialization. The fixture exposes
persisted piece fields for snapshots and simulates restoring them before calling
the production `PostLoad`; it does not exercise the creg serializer itself.

Both executables produce hexadecimal float-bit traces. These compare local and
model transforms, matrices, previous transforms, interpolation output, dirty and
upload flags, inputs and bounds. Cases cover root motion, final settle frames,
equal inputs and signed zero, multiple NaN payloads, blocked/unblocked animation,
lazy and explicit updates, both model-space writers, reparenting, post-load and
model remapping, plus 4,096 reproducible interleaved operations. `*.layout` reports
the size, alignment and offsets of all pre-existing fields. Composition counts
show work avoided in the fixture; they are not an engine performance measurement.

When an owner aggregate gate is present, the fixture uses the real
`LocalModel::SavePrevModelSpaceTransforms` and requires that a clear owner flag
implies every piece is clean and has no pending previous-transform copy. Sixty-four
fully settled frames must perform zero piece save visits. Separate broad, deep
and balanced hierarchies exercise both lazy and breadth-first updates.
`*.owner-layout` compiles the real headers, including the actual collision-volume
and Lua-material types, to compare all existing LocalModel fields, size and
alignment independently of the trace's lightweight geometry fixture.

The animation hierarchy pass is extracted directly from each tree's
`CUnitScript::TickAllAnims`, rather than reimplemented in the fixture. Its dirty
piece visit sequence must match exactly. When dirty-subtree flags are present,
every dirty piece and each ancestor must have its conservative flag set. The
animation kernel before the hierarchy pass is hashed and must be byte-identical;
done-animation handling and callbacks are outside the changed traversal. The
fixture records total hierarchy visits separately from transform calculations.

For a compact previous-copy interval, every pending piece must lie inside the
inclusive owner range or the owner must request a full sweep. The trace compares
the exact sequence of actual previous-transform copies, independently from the
number of piece methods visited. Unknown owner membership and a 65,537-piece
array must use the full sweep; physical index 65,535 must remain a valid one-piece
interval. A deliberately incorrect logical piece ID must not affect indexing.
These checks use physical array membership and never assume DFS preorder or
unchanged parent/child topology.

Manual engine validation after a normal dependency-aware rebuild:

1. Run the ordinary idle-ship workload with its original per-frame drift, bob,
   scripts, target scans and reactions. Capture the same native CPU profile and
   wall/CPU measurements as the baseline; preserve all existing engine features.
2. Compare the synced gameplay/transform contract at matching frames, including
   active motion, stopped animations and the following two render uploads.
3. Check animated weapon emitters, script matrix blocking/unblocking, and a saved
   game reload. Exercise animated descendants and interpolation at close range.
4. Treat matching profiles and contracts as validation evidence, not user review
   or acceptance. Do not install, commit or push the engine as part of this check.

To compare an unchanged full checkout with a candidate full checkout:

```sh
python3 test/validation/local_model_transforms/check.py \
  --baseline /path/to/unchanged-engine \
  --candidate /path/to/candidate-engine \
  --math-root /path/to/engine-with-initialized-math-submodules \
  --streflop /path/to/matching-build/rts/lib/streflop/libstreflop.a \
  --out /path/to/fresh-output
```

No particular checkout or build path is embedded in the checker. It reads the
roots supplied on the command line and writes only the requested output folder.
The fixed profiling lock coordinates with the local engine profiling workflow.

An original parent predating previous-copy suppression uses a direct getter
assignment and emits no optional `previous-copy` operation diagnostics. The checker
verifies the actual SavePrev assignment instrumented by its source extractor,
validates each diagnostic's syntax and uint32 range, and requires that declared
diagnostic support agrees with observed output. It keeps raw traces and their
hashes unchanged. If the original baseline lacks this optional diagnostic, only
those validated operation lines are omitted for a separate state trace; every
state snapshot, dirty visit, matrix/float bit, bounds/interpolation/input flag and
other output line must still match exactly. If both source trees support the
diagnostic, complete raw traces including copy-operation order must match. Losing
a baseline diagnostic in the candidate is rejected explicitly. Existing member
layout mismatches also fail the check.
