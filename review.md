# Review — `doc/rust_cus`

Reviewing the 19 files added by `origin/docs/rust-cus` (1,493 lines) as of `a5c7a53dbd`. Claims below marked *verified* were checked against the engine source in this checkout at `79a08e2c52`; claims marked *unverified* I could not check locally.

**Status:** `64f12cb4ad` ("docs: settle Rust CUS V1 runtime constraints") addresses §2, §3, §4, §5 and §6, and resolves the three items §7 called blocking. §1 and §8 still stand. Kept as a record of the review pass; read the docs at HEAD for current state.

## Verdict

The architecture is sound and the central decision is right. Two things in it are better than the framing I had been working from: making the **same-module requirement** the spine of the design, and treating **native and Wasm as equal backends** rather than wasm-first with native as a test mode.

What it is missing is engine ground truth. Several constraints in `CUnitScript` are already decided by the engine and will shape the API, but the proposal defers them to "implementation questions" — so a reader finishes the doc set believing the API surface is more open than it is. The save/load section in particular defers a decision it could largely settle by stating what COB and LUS do today, which is not what most readers would guess.

It reads as a strong direction document. It is not yet an implementable spec, and §7 lists what stands between the two.

## 1. What the proposal gets right

**The same-module requirement is the load-bearing idea.** `design/module-model.md` and `design/game-api.md` correctly identify that a unit script calling `crate::overkill::should_block(...)` as an ordinary Rust call is what separates this from "a nicer COB". The `BlockShot` example is the right example: it is precisely what LUS can do via `GG` and COB structurally cannot. Stating "one synced module, many script types, many instances — not one Wasm module per script" as a core decision heads off the most expensive wrong turn available here.

**Native/Wasm parity as a first-class goal** (`sdk/portable-spring-rust.md`) is worth the space it gets, and the observation that it is valuable independently of CUS is correct.

**Per-instance routing kept out of the module-wide callin fan-out** (`design/engine-integration.md`) is right, and matches how the engine already separates concerns. Unit scripts have per-instance lifetime and routing; `Callins.def` subscribers do not.

**Reuse `WasmInterfaceSystem` rather than standing up a second Wasmtime runtime** — right, and worth having in writing.

**Deterministic backend selection, no probing/racing** — right instinct (see §3.5 for why it is cheaper than the doc assumes).

**Game evidence across BAR, Zero-K, MCL and Area-17** is better breadth than a single-game proposal would have. The BAR counts (849 `.bos`, 851 `.cob`, 31 `.lua`) match what I counted independently: *verified*.

## 2. Engine constraints the proposal needs and does not have

These are not open questions. The engine has already answered them, and each one shapes the trait in `design/authoring.md`.

### 2.1 `AimWeapon` and `AimShield` return void

*Verified*, `rts/Sim/Units/Scripts/UnitScript.h:214-215`:

```cpp
virtual void  AimWeapon(int weaponNum, float heading, float pitch) = 0;
virtual void  AimShieldWeapon(CPlasmaRepulser* weapon) = 0;
```

Every example in the doc set — README, `authoring.md`, `examples/minimal.md`, `examples/zero-k-amphraid.md` — writes `async fn aim_weapon(...) -> bool`. The engine consumes no bool here. LUS gets the same effect by thread-wrapping the callin and publishing the result afterwards through `SetUnitWeaponState(unitID, weaponNum, "aimReady", ...)` (`unit_script.lua:603`) and `SetUnitShieldState` (`:621`).

So the doc's *shape* is achievable, but the `-> bool` is a runtime convention, not a return value the sim reads, and nothing in the docs says so. An implementer reading `authoring.md` alone would try to wire a synchronous bool through `CUnitScript` and find it has nowhere to go. Worth one sentence in `authoring.md` and one in `engine-integration.md`.

Related signature detail: `AimShieldWeapon` takes a `CPlasmaRepulser*`, not a weapon index, and `BlockShot` is `(int weaponNum, const CUnit* targetUnit, bool userTarget)` — the `userTarget` argument is missing from the `block_shot` examples in `module-model.md` and `zero-k-amphraid.md`.

### 2.2 Which methods may suspend is already determined

`rationale/open-questions.md` says the implementer "should classify the methods in `CUnitScript` explicitly rather than make every method async by default". Correct — and the classification is not a judgement call, it falls out of which virtuals the sim consumes inline. Those are `QueryWeapon`, `AimFromWeapon`, `QueryTransport`, `QueryNanoPiece`, `QueryBuildInfo`, `BlockShot`, `TargetWeight` and `QueryLandingPads`. Everything else is void or fire-and-forget and may suspend.

One of those is not a scalar: `QueryLandingPads` fills a `std::vector<int>` (`UnitScript.h:185`), so the transport needs a capacity/retry contract for list results, not just a value union. That is a real ABI design item and it is currently invisible in the docs.

Listing the eight in `engine-integration.md` would convert an open question into a table.

### 2.3 `usMemBuffer` constrains the C++ object

*Verified*, `rts/Sim/Units/Unit.h:303`:

```cpp
// sufficient for the largest UnitScript (CLuaUnitScript)
uint8_t usMemBuffer[sizeof(CLuaUnitScript)];
```

with `static_assert(sizeof(X) <= sizeof(unit->usMemBuffer))` at `UnitScriptFactory.cpp:52,59`. Scripts are placement-new'd into a fixed inline buffer on every `CUnit`.

So a Rust-backed `CUnitScript` subclass either fits inside `sizeof(CLuaUnitScript)` or the buffer grows for **every unit in every game, including games that never use CUS**. That is a hard design constraint on the C++ side of the adapter — keep it to a module handle, an instance id and a provided-callin mask, with all real state guest-side. `engine-integration.md` should say this; it is the kind of thing that is cheap to honour from the start and expensive to retrofit.

### 2.4 `Killed` must settle from C++

The LUS framework goes out of its way here (`unit_script.lua:635-638`):

```lua
-- It is *very* important the sp_SetDeathScriptFinished is executed, even on error.
SetOnError(sp_SetDeathScriptFinished)
```

For CUS the equivalent has to live in C++, not in Rust, because a Wasmtime trap gives no reliable guest unwind point — `Drop` will not run. If a missing, faulted or silent handler can prevent `KilledScriptFinished`, corpses hang mid-death permanently. Nothing in the doc set mentions this, and it interacts directly with the `Drop`-as-cancellation question in `open-questions.md` (see §6).

### 2.5 Attachment is already solved, and it makes backend selection cheap

`engine-integration.md` says a unit def "should select its unit-script backend deterministically" and leaves the syntax open. The engine's existing answer is worth stating, because it makes this nearly free.

*Verified*: `CUnitScriptFactory::CreateScript` (`UnitScriptFactory.cpp:28`) handles `*.cob` and returns `CNullUnitScript` for everything else — its own comment defers Lua to `LuaUnitScript::CreateScript`. Attachment for a Lua script happens later, from the gadget, when it calls `Spring.UnitScript.CreateScript(unitID, callInTable)` (`LuaUnitScript.cpp:1163`), which does `unit->script = CUnitScriptFactory::CreateLuaScript(unit, L)`.

So today's LUS backend selection is not in the unit def at all: it is "whoever calls the create-script callout at `UnitCreated` wins". CUS attaching the same way means no new file format, no extension sniffing, one factory method beside `CreateLuaScript`, and COB/LUS/CUS coexisting per unit for free. It also means the "no probing and racing" property is structural — there is exactly one owner calling the callout — rather than something a new unit-def field has to enforce.

## 3. Save/load: the missing ground truth

This is the most consequential gap. `design/save-load.md` and `rationale/why-async.md` correctly explain why a rustc-generated future cannot be serialized, and `open-questions.md` asks what real games require. But neither states what the two incumbent backends do today, and that fact changes the shape of the decision.

**COB threads fully serialize.** *Verified*: `CCobThread` is a creg struct (`CobThread.cpp:16-41`) persisting `pc`, `dataStack`, `callStack`, `wakeTime`, `signalMask`, `waitPiece`, `waitAxis` and `state`. The interpreter's entire execution state is plain data, so a COB unit caught mid-walk-cycle resumes mid-walk-cycle.

**LUS threads do not serialize at all.** *Verified*: Spring's Lua save path is cooperative — `CLuaLoadSaveHandler::SaveEventClients` calls `eventHandler.Save(savefile)` and each gadget persists what it chooses. `unit_script.lua` implements no `Save` and no `Load`; its only callins are `GetInfo`, `Initialize`, `UnitCreated` and `GameFrame`. Lua coroutines are not serializable in stock Lua regardless. `CLuaUnitScript::Serialize` (`LuaUnitScript.cpp:68-83`) persists a single bit — whether the handle is LuaRules or LuaGaia — and rebinds `lua_State` in `PostLoad`.

Meanwhile `CUnitScript::anims` **is** creg'd (`UnitScript.cpp:62`). So loading a save of an LUS game restores animations mid-flight with every thread that was waiting on them gone.

What this does to the proposal:

- **Async in V1 is exact parity with LUS.** Not a regression, not a compromise — the same behaviour LUS ships today. `save-load.md` can say this outright and it strengthens the V1 argument considerably.
- **Async is a regression against COB**, for any unit migrating from COB rather than from LUS. Given BAR is ~96% COB by script count, that is the population most affected, and the doc never surfaces the risk.
- **`open-questions.md`'s save/load question becomes answerable.** "What do real games require" is unanswerable in the abstract; "do you accept COB-attached units losing thread state on migration to CUS, when they do not lose it today" is a question a game lead can actually answer.

I would move this ground truth into `save-load.md` as its opening section. It is three paragraphs and it upgrades the whole chapter from deferral to informed deferral.

### 3.1 The forward path is not as free as claimed

The README says the authoring model "must leave a path to proc-macro-generated durable state later without requiring games to rewrite their unit scripts". `save-load.md` then concedes, under *Cross-suspension locals*, that durable lowering likely requires:

> A local that must survive a CUS suspension needs an explicit type and, for a persistent task, that type must satisfy a CUS save-state trait.

Those two statements are in tension. V1 async code that holds a borrow, an iterator, or a non-conforming local across an `.await` will need rework under the durable lowering — the source *shape* survives, the source does not necessarily. `examples/generated-animation.md` shows awareness of exactly this ("stable IDs and explicit indices are better durable state"), and `compatibility/games.md` draws the same lesson from Area-17.

The cheap fix is to write the restriction down in V1 and lint it, rather than discover per-script which code was already compliant. Otherwise "no rewrite required" is a promise the design cannot keep.

## 4. Performance: two items are structural, not tunable

`design/performance.md` takes the position that data-structure choices should follow profiling. Agreed for timer wheels, slab layout and intrusive lists. Two items are not in that category, because they are part of the attachment ABI and cannot be added later without changing it.

**The provided-callin mask.** A script must tell the engine which of the standard entry points it actually implements, so an unimplemented callin costs zero crossings rather than N. `CLuaUnitScript` already does the equivalent with `hasSetSFXOccupy`, `hasRockUnit`, `hasStartBuilding`, `HasBlockShot`, `HasTargetWeight`. At BAR scale this is the difference between zero and tens of thousands of crossings per event type, and the check itself is a shift and a mask. This belongs in the create-script call from day one.

**One frame drain, never a per-unit tick.** `performance.md` says to reuse the existing `GameFrame`/budget model, which implies the right thing without stating the invariant. Worth stating: the scheduler is entered once per frame for the whole module and drains what is due. With 10,000 units and 20 waking tasks that is one crossing, not 10,000.

**Task storage is the perf-relevant consequence of choosing async, and it is unaddressed.** `authoring.md` and `zero-k-amphraid.md` show `ctx.spawn(Self::restore_after_delay)` without saying how tasks are stored. If spawned futures are boxed, that is a heap allocation per spawn per unit — which is the LUS cost profile (`coroutine.create` per `StartThread`) reproduced in Rust. Keeping them inline avoids it but constrains the spawn API to a closed set of concrete future types per script type, which is a real authoring consequence. Whichever way it goes, it should be a stated decision rather than an implementation detail, because it is the main thing that determines whether CUS lands near COB's cost or near LUS's.

**The benchmark suite has no baseline.** The workload list in `performance.md` is good. The target it should be measured against is COB, not LUS — LUS already lost that comparison at BAR scale, which is why 96% of BAR's scripts are `.bos`. "Beats LUS" is not a bar worth clearing.

### 4.1 One thing the docs get right and could state more strongly

`engine-integration.md` says the animation-completion phase is "a natural place to wake waiting CUS tasks". That is correct and more precisely supportable than the doc claims. *Verified*, `UnitScriptEngine.cpp:126-163`: the engine runs two passes over `animating`. The first is `for_mt(...) TickAllAnims`, pure C++ interpolation of piece transforms that never enters script code (`UnitScript.cpp:199-240`). The second is single-threaded `TickAnimFinished`, which `UnitScript.cpp:282-284` documents as deliberately serial to keep `AnimFinished` ordering consistent across the simulation.

So there is already a single-threaded, deterministically-ordered point in the frame designed for exactly this. Worth citing, and worth stating the corollary explicitly, because `for_mt` invites the wrong assumption: **script code never runs multithreaded, in Lua or in Rust, and CUS does not change that.** What CUS gets for free is that a 150-frame turn costs the guest nothing for 149 of those frames.

## 5. Terminology problem

`overview.md`'s glossary defines:

> **CUS** — Spring's engine-side custom unit-script abstraction.

and the README opens with "a Rust implementation of Spring's existing `CUnitScript` abstraction". There is no engine concept named CUS. The class is `CUnitScript`; nothing in `rts/` uses the acronym. Prior use of "CUS" in this project's own earlier proposal meant *Core-wasm Unit Script*, which is a different expansion entirely.

Presenting CUS as pre-existing engine terminology will confuse anyone who then greps for it. Either define it as an alias coined here (`CUS ≡ CUnitScript`, so "Rust CUS" reads naturally), or choose a name that is not already overloaded. This is cosmetic but it is on the first page of every file.

## 6. `Drop`-as-cancellation is partly already decided

`open-questions.md` asks whether "dropping a future and running Rust destructors is part of the public semantic contract". Half of that is forced by the runtime: on a Wasmtime trap there is no reliable guest unwind, so `Drop` cannot be relied on for anything the engine needs to observe. §2.4's `KilledScriptFinished` is the concrete case.

A workable split, worth writing into the doc: `Drop` is fine for guest-internal cleanup and is a clean model for `Signal` cancellation; it is never the mechanism for engine-visible settlement, which must be guaranteed from C++.

## 7. Decided versus deferred

Counting across the set — decided: same-module model, async for V1, `CUnitScript` as the engine contract, native/Wasm parity, reuse of `WasmInterfaceSystem`, per-instance routing kept separate. Deferred: trait shape, sync/async classification, spawn API, signal and cancellation API, custom entry-point registry, backend-selection syntax, crate split, save/load requirements, benchmark thresholds.

That ratio is appropriate for a direction document, and the doc is honest about it. But three of the deferred items block starting implementation rather than merely refining it, and all three are answerable now:

1. **The sync/async classification** (§2.2) — determined by the engine, needs transcribing, not deciding.
2. **The create-script ABI** — must carry the provided-callin mask (§4) and the instance id, or the transport gets redesigned later.
3. **Task storage, boxed or inline** (§4) — sets the cost profile and constrains the spawn API that `authoring.md` leaves open.

Settling those three converts this from a direction document into something someone can build against.

## 8. Smaller notes

- **Repetition.** The async rationale appears in `README.md`, `design/scheduling.md`, `design/save-load.md` and `rationale/why-async.md` with substantial overlap. `why-async.md` and the scheduling doc could merge without loss.
- **`compatibility/cob-lus-cus.md`** is the best single-page summary in the set. Consider promoting it ahead of `overview.md` in the reading order — it orients faster than the goal statement does.
- **COB's "limitation is capability breadth"** is right but undersells the other half: COB is also the fast one, and that is why it persists at scale. The table lists no performance row at all, which is a notable omission given `performance.md` exists.
- **`rationale/rust-to-cob.md`** is a fair treatment and the coexistence conclusion is sound. Its claim that Rust-to-COB "inherits existing save/load semantics" is correct and is, in passing, the strongest available argument for that path — see §3.
- **`examples/generated-animation.md`** is the most useful example in the set for BAR, because it addresses the exporter question rather than hand-authoring. Given BAR is 226k lines of mostly-generated `.bos`, whether the chosen threading model can be *emitted by a generator* deserves to be a first-class requirement rather than an example.

## 9. What I verified

Checked in this checkout: `AimWeapon`/`AimShieldWeapon`/`BlockShot` signatures; the eight synchronous-result virtuals; `usMemBuffer` and its `static_assert`s; the `CreateScript` → `CreateLuaScript` attachment path; `SetOnError(sp_SetDeathScriptFinished)`; `aimReady` and `SetUnitShieldState` publication; `CCobThread` creg membership; the absence of `Save`/`Load` in `unit_script.lua`; `CLuaUnitScript::Serialize`; `CUnitScript::anims` creg; the MT/ST split in `UnitScriptEngine::Tick` and its comment; BAR's script counts.

Not checked: the Zero-K `amphraid.lua` and MCL `Infantry.lua` characterisations, the claim that MCL has no BOS/COB scripts, Area-17's `human.lua`, and the Metal Factions / Tech Annihilation / Spring:1944 remarks. Those repositories are not in this checkout, so `compatibility/games.md` and the two game examples rest on descriptions I could not confirm.
