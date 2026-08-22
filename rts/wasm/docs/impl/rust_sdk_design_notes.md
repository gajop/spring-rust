# Design notes: published Rust SDK, docs, distribution, sync proof

Date: 2026-08-22
Status: discussion notes. Decisions marked DECIDED are settled; OPEN items are
deliberately not decided yet. Not a work plan — see `next_agent_handoff_2.md` for
what is actually queued.

## 1. Verified facts (checked in repo, not assumed)

- A wasm guest is a `cdylib` built for `wasm32-unknown-unknown`. One crate, one
  `.wasm`. The crate contains **no environment awareness**.
- Environment is declared externally, per module, in the manifest:
  `module(name, path, rules-synced, 0, 1.0.0)`.
- Enforcement is at runtime by the host. Every callout in
  `rts/wasm/generated/WasmCalloutRegistry.h` carries a 5-bit `environmentMask`
  (`31u` = all five; `26u` = unsynced + gaia-unsynced + UI).
- All five environments have `runtimeEnabled = true`.
- Native is one DLL loaded by `NativeInterfaceSystem`, full API access, **no
  environment concept at all**. It also owns the Wasm system
  (`LoadWasmModule`, `GetWasmInterfaceSystem`).
- The parity oracle is Lua (`run_harness.py:921`), with the native `.so` as a
  second reference.

## 2. What native is for — DECIDED

Native exists for **capability**, not trust:

- tools
- singleplayer
- UI / menus (faster, more possible)
- in-game lobbies

These need real threading, job systems, and the normal Rust crate ecosystem.
Wasm prohibits that. That is the whole distinction.

Consequence, unresolved: if native has threading and wasm does not, the two
APIs **cannot** be identical in the general case. The shared surface is the
engine API; the divergence is everything around it. See §7.

## 3. Environment model for wasm — DECIDED

**One wasm module per environment.** A game with synced logic, unsynced logic
and UI is three `.wasm` files, three crates:

```
module(mygame_sim,  sim.wasm,  rules-synced,   0, 1.0.0)
module(mygame_draw, draw.wasm, rules-unsynced, 0, 1.0.0)
module(mygame_ui,   ui.wasm,   ui,             0, 1.0.0)
```

This is the isolation boundary, not a limitation: separate instances, separate
linear memory, separate import sets. A synced module cannot reach rendering
state because those imports were never granted.

**Env is selected by module path, not by Cargo feature:**

```rust
use spring_api::rules_synced as api;   // sim.wasm
use spring_api::ui as api;             // ui.wasm
```

Callouts invalid for that env do not exist in that module → ordinary compile
error. Shared game logic lives in the author's own crate that does not touch
the API directly.

**Rejected: Cargo features per env.** Cargo unifies features across a build
graph, so a workspace containing both a synced and a UI crate resolves
`spring-api` once with the union. The guarantee disappears exactly when a game
has more than one module. Resolver v2 does not fix this for normal deps across
workspace members.

**Three enforcement layers**, each catching what the previous cannot:

| layer | mechanism | catches |
| --- | --- | --- |
| compile | symbol absent from the env module | calling an illegal callout |
| load | marker export vs manifest declaration | code/manifest env mismatch |
| runtime | host `environmentMask` check | everything; never trust the guest |

The load-time marker is a generated export (e.g. `SPRING_ENV_MASK: u32`) that
the engine compares against the declared environment, failing closed with a
message naming both sides. Runtime checking stays regardless.

## 4. Distribution — DECIDED in shape, OPEN in detail

Goal: **users build games without building the engine.**

- Wasm already needs nothing: `cargo build --target wasm32-unknown-unknown`
  plus a manifest line. Publishing `spring-wasm-core` is sufficient.
- Native likely also needs nothing: native modules are loaded *by* the engine,
  so the guest `.so` needs ABI bindings, not the engine binary — symbols
  resolve at load time from the host. **UNVERIFIED; confirm before promising.**

Crates to publish: a facade (`spring-api`), plus the wasm and native backends.

OPEN: ABI stability policy. `interfaceVersion` already exists in the manifest.
Decide the compatibility promise **before** first publish, not after.

## 5. Docs — DECIDED

Split by who maintains them:

- **Generated:** full API reference, per-callout env/mask table, sync-safety
  annotations. Regenerates from `model.json`; never rots. Precedent already in
  repo: `lua_functions.md`, `rust_functions.md`, `api_surface_audit.md`.
- **Hand-written, small (~5 pages):** install, quickstart, env model, sync
  rules, debugging.

AI writes v1 of the hand-written part as an explicitly labelled placeholder,
to be rewritten by a human later. Constraints for that draft: tables and bullet
fragments only; no sentence longer than one line; no adjectives; no
intro/summary paragraphs; facts only.

## 6. Proving sync — DECIDED as an approach

Do not attempt a general proof. Build a regression gate.

Structural advantages that already exist:

- Wasm `f32`/`f64` are IEEE-754 exact and deterministic by spec — no x87, no
  FMA contraction, no fast-math, no float reassociation. The classic
  cross-platform desync source is gone by construction.
- The environment mask **is** the sync mechanism: a synced module cannot import
  nondeterministic callouts. `spring:desync` is the explicit opt-in escape
  hatch, so the audit surface is exactly that module's import list.

Ladder, cheapest first:

1. Same binary, same replay, N runs, hash sim state per frame. Catches
   intra-binary nondeterminism.
2. Audit the mask: for every synced-visible callout, is it actually
   deterministic? Generated table, reviewed once by a human.
3. Cross-platform CI: headless Linux + Windows, same replay, compare per-frame
   sync checksums. One GitHub Actions workflow; both runners are free.
4. Treat desync as a failing test with a frame number, not as a proof
   obligation.

Note: this reasoning is wasm-specific. It does not transfer to native, which
has none of these structural properties.

## 7. OPEN — not decided

- **Does the env model apply to native?** Not required. Possibly useful. Would
  hurt tools if forced. Leaning: optional, never mandatory. Not decided.
- **Is "identical Rust APIs" still the right goal**, given §2? Native gets
  threads, jobs and the crate ecosystem; wasm gets none. The engine API can
  match; the surrounding programming model cannot. Decide what "identical"
  should mean before publishing.
- ABI stability policy (§4).
- Whether native is in scope for any sync guarantee at all.

## 8. Also worth doing, not yet discussed

- **Debugging story.** What a modder sees when a guest traps mid-frame. A bare
  trap with no symbol, frame, or module name will block adoption harder than
  any missing callout.
- **Iteration loop.** Edit → rebuild → in-game, in seconds. Modders coming from
  editing a `.lua` and reloading will feel any regression here badly.
- **A real example module.** Not a test guest — a small recognizable gadget
  someone can copy.
- **Error ergonomics.** Former missing-wrapper fallbacks are now compile-time
  omissions. Users must be able to tell "not implemented yet" from "wrong
  environment" from "bad input".

## 9. Corrections made during this discussion

Recorded so they are not reintroduced:

- The Component Model was **never** the parity oracle. Lua is. Deleting CM
  costs nothing verification-wise.
- `rules-unsynced` / `gaia-unsynced` / `ui` are **not** runtime-disabled;
  `core_parity_handoff.md` misreads the `synced` column as `runtimeEnabled`.
- Cargo features are the wrong mechanism for env selection (§3).
- Native vs wasm is **not** a trust distinction, and native is not "the
  permissive case". It is a capability distinction (§2).
