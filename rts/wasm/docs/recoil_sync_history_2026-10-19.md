# What past desyncs tell us to test

Review date: 2026-10-19. This is a source/history review, not a new gameplay
acceptance run. The follow-up work is in the [sync test plan](recoil_wasm_sync_plan.md).

The recurring problem is **local activity changing shared simulation work**:
selection changes callback order, graphics settings change projectile IDs,
unsynced path queries change caches, garbage collection changes path lifetime,
and worker scheduling changes which mutable state a search observes. There are
also ordinary uninitialized values, unstable ordering and numeric conversions.
Deterministic Wasm arithmetic alone cannot protect a module from those host bugs.

## Scope and branch status

The reviewed local engine is `6cedf0cde7f9e6f542bfb80ad7c6935fe5ff77b9`.
Public upstream master was fetched and pinned at
[`57a2de17c87078027b345f5557f06dd330108110`](https://github.com/beyond-all-reason/RecoilEngine/commit/57a2de17c87078027b345f5557f06dd330108110).
Their common ancestor is `a863df099b943f2c485d52f20a7d48277e7f9027`:
391 local-only commits and 107 upstream-only commits. The cached remote ref was
stale; this review uses the freshly fetched commit. No upstream changes were
merged or applied.

I searched the available Spring/Recoil history for desync, determinism and race
fixes, screened the 107 extra upstream commit subjects, and inspected selected
full patches and current source. I also read selected public Recoil, Zero-K
and BAR issue/PR bodies, comments and relevant PR diffs. This is a targeted
review, not an audit of every historical commit or a reproduction of every
public report. The [source inventory](data/sync_history_2026-10-19/review.json)
records exact commits, ancestry and public source states at review time.

In the sections below, **historical fix** means an inspected patch with an
identified mechanism, already present in both reviewed branches unless stated
otherwise. **Reported** means public evidence or a proposed explanation that
has not been independently reproduced here. An open PR is not a merged fix.

## Historical fixes and the tests they imply

### 1. Local UI and graphics changed simulation

- **Selection changed destruction callbacks.**
  [`192c9dd028`](https://github.com/beyond-all-reason/RecoilEngine/commit/192c9dd028a0ce7683ae4f7c6f9ba52fee4c7e1c)
  ([#1174](https://github.com/beyond-all-reason/RecoilEngine/pull/1174), 2024)
  replaced iteration over the dependency container with a defined dependence-type
  order. A local selection dependency could change the iteration order and hence
  `DependentDied` callbacks. Test two peers selecting different units while those
  units and their dependants die. Review mixed synced/unsynced dependency lists.
- **Particle settings changed synced projectile IDs.**
  [`e00938fb4f`](https://github.com/beyond-all-reason/RecoilEngine/commit/e00938fb4f1730ac893250312cfc97284fb37bb7)
  (2022) removed `MaxParticles` from initialization of the synced projectile
  free-ID pool. Different graphics settings must still produce the same IDs,
  creation order and damage.
- **Unsynced particles determined synced projectile death.**
  [`ed20e0c68d`](https://github.com/beyond-all-reason/RecoilEngine/commit/ed20e0c68d8720b352f90e158a3e1cd426c88203)
  (2015) replaced a condition involving remaining visual subparticles with a
  lifetime calculation. Particle limits could prevent those subparticles from
  existing. Test persistent fire and explosions with low/high limits and headless
  playback, checking simulation state rather than screenshots.

A related **open report**, [Recoil #2223](https://github.com/beyond-all-reason/RecoilEngine/issues/2223),
shows why callin context matters: transport-permission checks can originate from
local cursor/command checks. A synced callback that mutates state during such a
check can make hovering affect the sim. This is an API misuse/dispatch hazard
to reproduce and inspect, not proof that ordinary transport games currently
desync. Vary local hover activity without adding different network orders.

### 2. Unsynced path queries, caches and garbage collection

[`0b153ad93b`](https://github.com/beyond-all-reason/RecoilEngine/commit/0b153ad93b6737c6b78a9a884010a1a58130c9c5)
([#2279](https://github.com/beyond-all-reason/RecoilEngine/pull/2279), 2025)
fixes two distinct mechanisms. Unsynced HAPFS requests could populate the synced
result cache. In QTPFS, inserting/removing unsynced paths changed component
iteration order, changing which dirty paths received a budgeted update that
frame. It also separates immediate results from synced execution: an immediate
Lua/AI request is not necessarily unsynced.

Follow-up [`a134a1f73d`](https://github.com/beyond-all-reason/RecoilEngine/commit/a134a1f73d4138aa5073b1ba315b2e4778b722e9)
([#2370](https://github.com/beyond-all-reason/RecoilEngine/pull/2370)) deals with
external/ownerless paths and requeue behavior. Later,
[`81fc6fa28d`](https://github.com/beyond-all-reason/RecoilEngine/commit/81fc6fa28d7096f88412d9feda8067a14a773bc6)
and [`385c6eb819`](https://github.com/beyond-all-reason/RecoilEngine/commit/385c6eb819eccfe149d60b5a52785a59d9ada0b0)
([#2691](https://github.com/beyond-all-reason/RecoilEngine/pull/2691),
[#2695](https://github.com/beyond-all-reason/RecoilEngine/pull/2695), December 2025)
separate externally managed synced paths and prevent their automatic requeueing.
Lua GC timing had made external path deletion influence ordinary unit paths.

Test sustained unsynced requests on only one peer, enough dirty paths to exceed
a frame's update budget, explicit deletion versus GC, allocation pressure and
owner destruction. Run HAPFS and QTPFS as separate scenarios. The same rules
apply to Rust wrappers: local drop/finalizer timing must not silently schedule
shared simulation changes.

[Zero-K #2434](https://github.com/ZeroK-RTS/Zero-K/issues/2434) records disabling
`Spring.RequestPath` after QTPFS desyncs. Later reports show that some ZK setups
stub it unless an option or feature enables it. A test must prove the actual
host request ran and produced a usable path; calling a disabled wrapper is not
coverage. The separate current crash is covered below.

### 3. Background path work read changing live state

The canonical reachable fixes are
[`ae1db84a3d`](https://github.com/beyond-all-reason/RecoilEngine/commit/ae1db84a3ddd9b6750ec46b933ccd3c7d256c99e)
and [`10ff7b0523`](https://github.com/beyond-all-reason/RecoilEngine/commit/10ff7b0523e1254d27004f32e9cb21f4c6c98b83)
(June 2026). Repair work uses the search's path copy rather than looking up the
live path during worker execution. Sharing heads are captured before execution,
and movement definitions come from the fixed node layer rather than a changing
owner. The patches also correct repath-trigger bookkeeping.

[Recoil #2982](https://github.com/beyond-all-reason/RecoilEngine/issues/2982)
reported the first different waypoint at frame **55,801**, although the visible
checksum warning arrived at 55,860. Its closure links the sharing fix; that is
historical report evidence, not a rerun performed in this review.

Test long-lived shared/repaired paths while units die, change ownership or
movement state, terrain changes and new requests arrive. Capture waypoints and
path queues before the first warning. A 900-frame test cannot exercise the
reported late failure simply by having more CPUs.

### 4. Ordering and random draws differed

| Historical fix | Mechanism | Regression to retain |
|---|---|---|
| [`ddf580c329`](https://github.com/beyond-all-reason/RecoilEngine/commit/ddf580c3294df8784af64d82aa751d58d6b147af), 2017 | Multiple RNG calls in expressions/argument evaluation; visual smoke also consumed synced RNG | Explicit draw sequence; compare RNG state around simulation phases and local visual activity |
| [`98a5a80ca8`](https://github.com/beyond-all-reason/RecoilEngine/commit/98a5a80ca83a923288ed54c2f73f0cb19710e8cb), 2016; [`e4d0e47d45`](https://github.com/beyond-all-reason/RecoilEngine/commit/e4d0e47d45f5f7ef2f77e042621f57588c35b127), 2024 | Compiler-dependent equal-key sorting; path priority queues lacked a final node-ID tie breaker | Symmetric maps, equal-cost paths/targets, stable input order and explicit final IDs |
| [`6f83700821`](https://github.com/beyond-all-reason/RecoilEngine/commit/6f8370082151a34275eac18f0f635a8da70fb6a7), 2021 | Lua used locale-sensitive `strcoll` for string ordering | Different process locales with mixed-case/non-ASCII names; define byte ordering where required |
| [`939ead8181`](https://github.com/beyond-all-reason/RecoilEngine/commit/939ead8181afc50d4d733664c360d3a900c1b13b), 2010 | LUS sleeping threads woke in nondeterministic order | Same-frame wakeups/completions, cancellation and callbacks submitting more work |

Stable sorting is insufficient if the input order itself is unstable. Rust
`HashMap` iteration and pointer addresses are not suitable simulation ordering
rules. Check list order and callback order through the SDK, not only the final
set of returned IDs.

The actual merged diff of [BAR #352](https://github.com/beyond-all-reason/Beyond-All-Reason/pull/352)
changes **`pairs` to `ipairs`** when constructing a random-event list, despite its
title saying the reverse. Read the patch: arbitrary table iteration followed
by an RNG index is a different problem from different RNG arithmetic.

[BAR #4645](https://github.com/beyond-all-reason/Beyond-All-Reason/issues/4645)
reports replay desync when enabling a Lua profiler; the maintainer points out
that its hook calls `math.random`. Test instrumentation itself for simulation
neutrality. A diagnostic run must not consume shared RNG or add synced work.

### 5. Undefined behavior and uninitialized inputs

[`c19c879a4a`](https://github.com/beyond-all-reason/RecoilEngine/commit/c19c879a4adc195c905edd1406d4231635b74c5c)
(2025) initializes an omitted `AimShieldWeapon` COB argument. An old example,
[`91a728c24a`](https://github.com/beyond-all-reason/RecoilEngine/commit/91a728c24aa075c4bf3cf8706ba76b3ae2a8e802)
(2011), fixes UnitDef defaults read from other uninitialized members. Wasm's
zero-initialized linear memory does not initialize native host objects or
callback argument buffers.

[`7104ab1f76`](https://github.com/beyond-all-reason/RecoilEngine/commit/7104ab1f76d9fe6823016dcb5561e179174e39a2)
([#3075](https://github.com/beyond-all-reason/RecoilEngine/pull/3075), July 2026)
fixes float-to-short angle conversion at the +/-pi boundary. The out-of-range
conversion behaved differently on ARM and x86. This fix is already local.
An **additional upstream fix is missing locally**; see the branch-gap section.

Test angle wrap boundaries, negative zero, subnormals, NaNs/infinities and large
numeric arguments through real host calls. Use targeted UBSan and initialization
checks where feasible. [Zero-K #1496](https://github.com/ZeroK-RTS/Zero-K/issues/1496)
records NaN terrain propagation from terraform code: malformed inputs can poison
much more state than the immediate call result. Define deterministic rejection
before mutating terrain; do not merely normalize a later checksum.

### 6. Content and caches were part of the problem

- [`b57b40cda1`](https://github.com/beyond-all-reason/RecoilEngine/commit/b57b40cda165234ec24d430ed58d60feed761726)
  (2023) fixes model caching by basename when different paths can contain the
  same filename. Load order then determines which model is reused. Test duplicate
  basenames, cache warm/cold runs and canonical path handling.
- [`871896dc7b`](https://github.com/beyond-all-reason/RecoilEngine/commit/871896dc7bbbf3e1a675ee3996e0a6746d141efd)
  (2015) brings path-cache contents into initial sync checking. Reports in
  [ZK #838](https://github.com/ZeroK-RTS/Zero-K/issues/838) and
  [#1075](https://github.com/ZeroK-RTS/Zero-K/issues/1075) discuss stale caches and
  forced rebuilds. A user's cache-clearing workaround is not an isolated cause.
- [Recoil #399](https://github.com/beyond-all-reason/RecoilEngine/issues/399) /
  [#480](https://github.com/beyond-all-reason/RecoilEngine/issues/480) report
  immediate headless/rendered differences with image-based map typemaps.
  The exact fixing commit was not established here. Test map-derived terrain
  and typemap data under both builds, not just SMF metadata or archive hashes.
- [ZK #1069](https://github.com/ZeroK-RTS/Zero-K/issues/1069) reports different
  Windows/Linux metal-spot tables; the exact root cause remains uncertain.
  [#3185](https://github.com/ZeroK-RTS/Zero-K/issues/3185) and
  [#2588](https://github.com/ZeroK-RTS/Zero-K/issues/2588) describe missing resources
  and archive/download lifetime problems. These motivate content-loading tests;
  they do not establish one common arithmetic or engine cause.

Use isolated test caches, save their provenance, and verify bytes actually read.
Do not clear the user's normal caches. Matching archive names are insufficient.

## Findings in current code and extra upstream commits

### Confirmed upstream gap: heading conversions

Upstream [`da974d379b`](https://github.com/beyond-all-reason/RecoilEngine/commit/da974d379b613f024859292da0e000d9d2a4280c)
([#3342](https://github.com/beyond-all-reason/RecoilEngine/pull/3342), September)
fixes more float-to-short heading conversions in `IPathController.cpp` and
`GroundMoveType.cpp`. The local source still has the old expressions, including
`oldHeading + (turnBrakeDist * Sign(curTurnSpeed) * brakeDistFactor)` implicitly
converted to `short`. This is distinct from the already-local COB fix #3075.
Review and reproduce it before claiming ARM acceptance. No patch applied here.

Upstream [`37911da24d`](https://github.com/beyond-all-reason/RecoilEngine/commit/37911da24dc5587bd7900d9f88eae5a5148ab6af)
([#3297](https://github.com/beyond-all-reason/RecoilEngine/pull/3297)) also is absent:
`ITaskGroup::GetDeltaTime` reads an atomic timestamp twice. One snapshot avoids
inconsistent duration arithmetic. This is **not a non-atomic data race** and
inspection finds its uses in task statistics; no simulation desync is established.

### Present locally: HAPFS scratch-slot mismatch, not yet reproduced

[Open PR #3530](https://github.com/beyond-all-reason/RecoilEngine/pull/3530)
proposes a cause for [#3524](https://github.com/beyond-all-reason/RecoilEngine/issues/3524).
The author explicitly says the patch is untested. Source inspection confirms
the relevant mismatch locally:

1. `HAPFS/PathingState.cpp`, `UpdateVertexPathCosts`, passes a `for_mt` **iteration
   index** into `CalcVertexPathCosts` as `threadNum`.
2. `ThreadPool::ForTaskGroup::ExecuteStep` distributes those indices through an
   atomic counter. The index need not equal the executing worker's ID.
3. Nested HAPFS `PathFinder` calls use the **actual** `GetThreadNum()`;
   `MoveMath` blocker checks use `CSolidObject::mtTempNum[threadNum]`.

That makes scratch-slot aliasing across workers a credible race candidate.
The same mismatch is in the pinned upstream tree. It is not a demonstrated local
reproduction, and #3530 must not be described as the established cause of #3524.
The report's replay can diverge from itself on one machine with the same worker
count, so changing only OS/architecture is an inadequate test.

[Open PR #3531](https://github.com/beyond-all-reason/RecoilEngine/pull/3531)
separately proposes initializing default `MultiPath::searchResult` and avoiding
reuse of a pending path-extension request. The local default constructor indeed
leaves `searchResult` uninitialized, and `NextWayPoint` reads it before checking
`moveDef`. Reachability and a reproducer still need testing. The PR does not
claim these explain #3524.

### Present locally: QTPFS background registry access, separately a crash

[Open PR #3448](https://github.com/beyond-all-reason/RecoilEngine/pull/3448)
reports replay failures with two workers and describes workers reading the
registry while main-thread path creation/deletion modifies it. Current local
`ExecuteQueuedSearches` launches background work; `SyncUpdatedPathsSystem`
waits later, while `QueueSearch` and `DeletePath` can mutate the registry in
between. `!IsInMultiThreadedSection()` on the main thread does not establish that
those background workers have finished. The PR author reports before/after
replays; those results have not been reproduced here. Review a barrier versus
deferred mutation, including stalls, rather than adopting the patch on trust.

[Open PR #3459](https://github.com/beyond-all-reason/RecoilEngine/pull/3459)
addresses a **separate crash**: an immediate external synced request has no
`SearchModeIPath`, but `InitializeSearch` copies through `GetSearchPath()` anyway.
The relevant null-write path is present locally. The author supplies Windows
reproduction and regression results. This is not fixed by the #3448 proposal,
and a crash on all peers is not a desync. Test both explicitly.

### New upstream threading/rendering work needs targeted review

Upstream [`854eeec45a`](https://github.com/beyond-all-reason/RecoilEngine/commit/854eeec45a5703c938c0dad57a537ce7d4efcafa)
moves unit-state calculation into the parallel weapon-vector pass, then emits
events serially in unit order. It snapshots the active unit count, deferring
new units to the next pass. This is not locally present and is not a claimed
sync fix. Review cross-unit reads, births/deaths and callback-visible state
when integrating it. Nearby load-balancing changes increase the need to test
schedule independence. Likewise, the quad-list copy change
[#3452](https://github.com/beyond-all-reason/RecoilEngine/pull/3452) is a
storage/performance change, not evidence of a fixed desync.

[Issue #3373](https://github.com/beyond-all-reason/RecoilEngine/issues/3373)
questions renderer-dependent nano-piece polling and a synced build-power getter
after [#3191](https://github.com/beyond-all-reason/RecoilEngine/pull/3191).
The pinned upstream emitter **does call `GetNanoPiece` before local-model and
emission-count early exits**, in both builder and factory paths. Local predates
the new renderer branch. The report alone therefore does not prove that current
branches skip the call or desync. Keep a renderer-toggle/RNG comparison in the
plan and inspect the complete entry paths; do not list it as a confirmed bug.

### Existing local thread/network changes also need coverage

Local `c24a0727aa` changes the **generic** ThreadPool, in addition to Wasm jobs:
acquire reads for task completion, tracking outstanding queue pointers, and
waiting before reuse/deletion. Its effect extends to pathfinding and other pool
users. A Wasm job digest matching is insufficient coverage for that shared code.
Review publication, queue drainage, nested work, sleeping workers and shutdown
under sanitizer/stress tests.

Local `c43c5ce56c` fixes unlocked ping-count reads versus locked queue writes in
`CLocalConnection`; `5689978a8c` fixes deletion/flush count bookkeeping;
`c61f2a18bc` fixes headless backlog pacing. These are already committed local
history, not changes made by this review. The first is a real data race; the
others concern queue correctness/catch-up. Do not count all three as proven
simulation desync fixes. Live backlog/rejoin tests remain separate from replay.

## Evidence quality and remaining blind spots

- [`1f5323a2c9`](https://github.com/beyond-all-reason/RecoilEngine/commit/1f5323a2c9294d84655f407c1f52fb79708844a2)
  ([#3124](https://github.com/beyond-all-reason/RecoilEngine/pull/3124)) disables
  LuaUI in synctest because widgets issue orders at wall-clock-dependent frames.
  Equal seeds do not mean equal inputs. Record once and replay the exact stream;
  vary read-only local UI separately from widgets allowed to add commands.
- Engine checksums cover instrumented operations, not a complete serialization
  of all simulation/cache state. HAPFS load-time checks of vertex costs do not
  automatically cover every later cost update. Add targeted canonical state
  comparisons near suspected blind spots; never hash raw pointers/padding.
- [Recoil #2415](https://github.com/beyond-all-reason/RecoilEngine/issues/2415)
  illustrates incomplete reports and first-warning-only dumps.
  [BAR #7444](https://github.com/beyond-all-reason/Beyond-All-Reason/issues/7444)
  asks for hardware information. Keep both peers' logs, exact builds/configs,
  full command inputs, worker counts and the earliest state difference.
- [ZK #2387](https://github.com/ZeroK-RTS/Zero-K/issues/2387) discusses Lua OOM;
  its exact causal chain is unresolved. Test failure behavior and require every
  expected Wasm module to remain loaded. Silently dropping a failed module is
  not a successful sync run.
- [BAR #3869](https://github.com/beyond-all-reason/Beyond-All-Reason/pull/3869)
  only fixes the UI pattern recognizing a desync warning. A broad rollback such
  as [#3855](https://github.com/beyond-all-reason/Beyond-All-Reason/pull/3855)
  does not isolate a root cause. [ZK #2118](https://github.com/ZeroK-RTS/Zero-K/issues/2118)
  is another unresolved user-specific report. None is counted as a proven causal
  engine fix in this review.

The existing short Linux/Wine Wasm tests remain useful smoke evidence. They do
not settle these host-engine candidates, long-duration behavior, native Windows
hardware or ARM. The [plan](recoil_wasm_sync_plan.md) separates those gaps from
already implemented tooling and from fixes that need their own reproduction.
