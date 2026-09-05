# spring-addons re-entrancy and `delay()`

Spring callouts are synchronous. A callout made from a gadget or widget callin may cause the engine to immediately enter another callin on the same Wasm stack. `spring-addons` therefore does not assume that one addon callin finishes before another starts.

## Classic addon model

The classic model intentionally stays close to Lua semantics:

- gadget/widget objects are dispatched through `&self`, not a handler-wide `&mut self`;
- the game global is shared (`&G`) and is not itself locked for the duration of a callin;
- mutable state is borrowed explicitly from smaller resources;
- incompatible resource borrows fail loudly at runtime;
- callins that need an immediate return value are never silently queued or given a fallback answer;
- `ctx.delay(...)` is the explicit escape hatch when deferral is semantically valid.

`Resources` is the provided typed resource container:

```rust,ignore
use spring_addons::{GadgetHandler, Resources};

#[derive(Default)]
struct Units { /* ... */ }
#[derive(Default)]
struct Economy { /* ... */ }

type Global = Resources;

fn setup(handler: &mut GadgetHandler<Global>) {
    handler.global.insert(Units::default());
    handler.global.insert(Economy::default());
    // handler.add(...)
}
```

A callin borrows only the resource it needs:

```rust,ignore
fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, _frame: i32) {
    let mut units = ctx.global().access_mut::<Units>();
    // mutate Units
}
```

Holding a mutable `Units` borrow does not block a re-entrant callin that only needs `Economy`. A re-entrant callin that also needs `Units` fails at the resource access, exposing the actual re-entrancy conflict instead of changing engine semantics.

Games do not have to use `Resources`. A custom `Global` may expose its own `access()` / `access_mut()` scheme or use `Cell` / `RefCell` fields directly. The important property is that the global container itself should not be one giant exclusive borrow.

## `delay()` semantics

`ctx.delay(...)` appends an owned closure to a FIFO queue. The queue is drained only after the **outermost** active addon callin returns. If a nested callin schedules more delayed work, it joins the same queue and does not run while an outer callin is still active.

The closure receives the shared global:

```rust,ignore
ctx.delay(move |global| {
    // state access, Spring callouts, or arbitrary game logic
});
```

`delay()` is general-purpose. There is no separate command-buffer abstraction.

### Pattern 1: defer callouts

If game logic can compute what should happen before entering Spring, capture the result and issue the callout after the active callin stack unwinds:

```rust,ignore
fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, _frame: i32) {
    let unit_id = {
        let mut units = ctx.global().access_mut::<Units>();
        units.pick_unit_to_destroy()
    }; // the Units borrow ends here

    ctx.delay(move |_global| {
        // issue the DestroyUnit callout here
        destroy_unit(unit_id);
    });
}
```

This avoids making the callout while the callin holds the state borrow that a resulting re-entrant callin may need.

A delayed block is not magically non-re-entrant. If it borrows a resource and then performs a callout while keeping that borrow alive, the same conflict can still occur. Prefer calculating or copying what the callout needs before issuing it.

### Pattern 2: defer work from a re-entrant callin

A nested callin may need to update state that the outer callin is already using. If that work does not contribute to an immediate return value, schedule the state update instead:

```rust,ignore
fn unit_destroyed(
    &self,
    ctx: &spring_addons::AddonContext<'_, Global>,
    event: &spring_addons::UnitDestroyedEvent,
) {
    let unit_id = event.unit_id;

    ctx.delay(move |global| {
        global.access_mut::<Units>().remove(unit_id);
    });
}
```

This is appropriate when the callin's effects may be applied after the current re-entrant stack unwinds.

It is **not** a solution for an immediate-result callin whose answer depends on state that is currently incompatibly borrowed. For example, if `UnitPreDamaged` must inspect a resource to calculate its damage result and that resource is already mutably borrowed, the classic model treats that as a runtime game error. The framework does not pass damage through unchanged, invent an `AllowCommand` result, or queue a result for later.

## Callin exports

The main widget/gadget export macros export the callins represented by their addon traits. The old per-callin opt-in export macros are retained only as migration no-ops; games no longer need them.

`reentrant_unit_pre_damaged:` is also obsolete. Re-entrant `UnitPreDamaged` uses the same gadget chain as a top-level `UnitPreDamaged`.

## Guarantees and non-guarantees

This is the Lua-compatible, runtime-checked model. A game can still hold an incompatible resource borrow across a callout and discover the problem only when the engine actually re-enters a callin that needs that resource.

A stricter API that prevents this class of problem at compile time is intentionally not part of this implementation.
