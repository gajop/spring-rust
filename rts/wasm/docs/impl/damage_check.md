# Damage check: concurrent edits by a second session

Date: 2026-08-22, ~11:50–12:10 JST
Written by the session that caused it. Assume nothing here is safe; verify.

## Why this exists

A human-driven session edited this working tree while an agent was actively
working in it. The human session did not know the agent was running. Two
actions are potentially destructive:

1. `cargo fmt --all` across the whole workspace, not scoped to its own files.
2. `git checkout HEAD --` on 9 files, which discards **all** uncommitted
   changes to those files — including anything the agent had written to them.

The human session cannot distinguish its own edits from the agent's in those
files. **It has no basis for claiming nothing was lost.** Check.

Baseline at the time of the actions: `05ff9d4771`.

**Update:** the agent has since committed `1f96da955f` (`Complete Core Wasm
parity task set`), which already contains all 11 clippy fixes — it applied them
itself from `tasks.md` §0. They are recorded in full below anyway, so the
change can be audited rather than trusted.

## Observed agent activity

Files modified by the agent, observed by mtime while the human session was
working:

- `12:03:03` `rts/wasm/generated/sdk/core_owned.rs`
- `12:04:10` `test/wasm_api/parity_guest/src/probe_generated.rs`
- `12:04:22` `rust/crates/spring-native-codegen/src/render_core_wasm_owned_guest.rs`
  (added a `decode_core_string` helper — hand-written, not generated)

Also in flight, still present: the `rts/WasmInterface/` directory refactor
(~70 renames into `core/bindings/`, `core/host/`, `runtime/`, `system/`).

## Action 1 — clippy edits (intentional, then discarded)

All under `rust/crates/spring-native-codegen/src/`. Purpose: fix the 11
clippy errors failing CI.

| file | edit |
| --- | --- |
| `render_core_wasm_host.rs` | `sed -i 's/, ToLowerCamelCase//; s/ToLowerCamelCase, //'` |
| `render_core_wasm_option_host.rs` | same |
| `render_core_wasm_variable_host.rs` | same |
| `render_core_wasm_variable_output_host.rs` | same |
| `render_core_wasm_borrowed_host.rs` | same |
| `render_core_wasm_dynamic_input_guest.rs` | `sed -i '10s/FieldModel, //'` |
| `render_core_wasm_callins.rs` | line 279, `.map_or(true, \|record\| {` → `.is_none_or(\|record\| {` |
| `render/core/shared/wire.rs` | removed 3 blank lines following `///` doc comments (before lines 19, 89, 131) |
| `render_core_wasm_callin_scratch_guest.rs` | deleted lines 110, 121, 195 — the `field,` arguments and the `field: &FieldModel,` parameter of `render_rust_type` |

These 11 fixes are described in `tasks.md` §0 and are trivially redoable.

## Action 2 — `cargo fmt --all` (NOT scoped, still in the tree)

```sh
cargo fmt --manifest-path rust/Cargo.toml --all
```

Ran at roughly `12:04:22`, the same moment the agent last wrote
`render_core_wasm_owned_guest.rs`.

This reformatted **every** Rust file in the workspace, including files the
agent was mid-edit on. Its effects were **not** reverted and are still in the
working tree.

**Check:** any Rust file the agent had partially written may have been
reformatted underneath it, and `render_core_wasm_owned_guest.rs` specifically
was written by the agent seconds before fmt ran. Confirm its
`decode_core_string` helper and surrounding code are as the agent intended.

## Action 3 — `git checkout HEAD --` on 9 files (DESTRUCTIVE, unrecoverable)

```sh
git checkout HEAD -- \
  rust/crates/spring-native-codegen/src/render/core/shared/wire.rs \
  rust/crates/spring-native-codegen/src/render_core_wasm_borrowed_host.rs \
  rust/crates/spring-native-codegen/src/render_core_wasm_callin_scratch_guest.rs \
  rust/crates/spring-native-codegen/src/render_core_wasm_callins.rs \
  rust/crates/spring-native-codegen/src/render_core_wasm_dynamic_input_guest.rs \
  rust/crates/spring-native-codegen/src/render_core_wasm_host.rs \
  rust/crates/spring-native-codegen/src/render_core_wasm_option_host.rs \
  rust/crates/spring-native-codegen/src/render_core_wasm_variable_host.rs \
  rust/crates/spring-native-codegen/src/render_core_wasm_variable_output_host.rs
```

These 9 files are now byte-identical to `05ff9d4771`. Any uncommitted work the
agent had done in them is gone and is not in reflog, stash, or any object —
uncommitted content discarded by `git checkout` is unrecoverable.

**Check:** whether the agent had pending work in any of these 9. Note
`render_core_wasm_callins.rs`, `render_core_wasm_dynamic_input_host.rs` and
`render_core_wasm_variable_io_host.rs` are neighbours of files the agent *is*
modifying, so this is not hypothetical. If the agent's refactor touched any of
the 9, redo that part.

## Action 4 — build-script regeneration (side effect, still in the tree)

Ran `cargo clippy`, `cargo fmt --check`, `cargo check --workspace`,
`cargo test --workspace --lib`, `python3 rts/wasm/verify_codegen.py`.

Their build scripts regenerate into the working tree. Files changed as a
result, still present:

- `rts/wasm/generated/sdk/core_owned.rs`
- `test/wasm_api/parity_guest/src/probe_generated.rs` (−4,657 lines)
- `test/wasm_api/parity_guest/src/probe_bindings.rs`
- `test/wasm_api/parity_guest/src/probe_context.rs`

The −4,657 line change to `probe_generated.rs` was **not** investigated.
Confirm it is a legitimate regeneration and not a loss.

## Action 5 — documentation edits (human session, deliberate)

Committed nothing. Staged deletions and edits:

- deleted: `next_agent_handoff_2.md`, `unsupported_host_target_removal.md`,
  `complete_owned_surface.md` (completed handoffs)
- earlier deleted then **restored**: `benchmarking_results.md`,
  `core_benchmark_matrix.md` — the restore was correct, they hold the only
  surviving Component Model measurements
- edited: `core_parity_handoff.md` (one reference), `core_benchmark_results.md`
  (added a "Reference floor" section),
  `test/native_api_parity/frozen_benchmarks/README.md` (one reference),
  `rust_sdk_design_notes.md` (references)
- created: `tasks.md`, this file

Also, as a test of the surface gate, `core_environments.rs` had one
`pub use` line removed and was then restored from a `/tmp` copy. It should be
identical to `05ff9d4771`; verify that it is.

## What to do

1. Diff the working tree against `05ff9d4771` and decide, file by file, what
   belongs to the agent's in-flight work and what is contamination.
2. Redo any of the agent's work that fell inside the 9 reverted files.
3. Confirm `render_core_wasm_owned_guest.rs` survived `cargo fmt` intact.
4. Explain the `probe_generated.rs` −4,657 line change or restore it.
5. Confirm `core_environments.rs` matches `05ff9d4771`.
6. Then reapply the 11 clippy fixes (`tasks.md` §0) and get CI green.


## Appendix — the 11 clippy fixes, exactly

Already applied by the agent in `1f96da955f`. Listed so they can be checked,
and re-derived if that commit is reworked. All paths relative to
`rust/crates/spring-native-codegen/src/`.

Failing command (`.github/workflows/spring-native.yml:60`):

```sh
cargo clippy --manifest-path rust/Cargo.toml --package spring-native-codegen \
  --all-targets -- --deny warnings
```

### 1–5. unused import `ToLowerCamelCase` (5 files)

`render_core_wasm_host.rs`, `render_core_wasm_option_host.rs`,
`render_core_wasm_variable_host.rs`, `render_core_wasm_variable_output_host.rs`,
`render_core_wasm_borrowed_host.rs`

Drop `ToLowerCamelCase` from the `heck` import list, leaving the other traits.

### 6. unused import `FieldModel`

`render_core_wasm_dynamic_input_guest.rs`, line 10:

```rust
-use crate::model::{ApiModel, FieldModel, RecordModel, SemanticType};
+use crate::model::{ApiModel, RecordModel, SemanticType};
```

### 7. `clippy::unnecessary_map_or`

`render_core_wasm_callins.rs`, line 279:

```rust
-        SemanticType::Record { name } => records.get(name).map_or(true, |record| {
+        SemanticType::Record { name } => records.get(name).is_none_or(|record| {
```

### 8–10. `clippy::empty_line_after_doc_comments` (3 sites)

`render/core/shared/wire.rs` — delete the blank line between the `///` block
and the item it documents, at the three functions:

- `pub(crate) fn optional_string`
- `pub(crate) fn fixed_wire_field`
- `pub(crate) fn record_has_option`

### 11. `clippy::only_used_in_recursion`

`render_core_wasm_callin_scratch_guest.rs` — `render_rust_type` took a
`field: &FieldModel` parameter that was only ever passed down to itself and
never read. Remove the parameter and both call-site arguments:

```rust
 fn render_rust_type(
     ty: &SemanticType,
-    field: &FieldModel,
     name: &str,
     records: &BTreeMap<String, RecordModel>,
     declarations: &mut String,
     args: &mut Vec<String>,
 ) {
```

and at the two call sites (the top-level loop, and the `SemanticType::FixedArray`
recursion) drop the `field,` argument.

Removing a dead parameter cannot change emitted output; `verify_codegen.py`
should still report the generated tree reproducible. Confirm that it does.
