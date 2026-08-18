# Considerations

Reference material for the Wasm host boundary: what the choices are, what they
cost, and what has actually been measured. These describe the design space, not
the current state of the tree, so they stay valid whichever direction is taken.

| Document | What it answers |
| --- | --- |
| [measured_costs.md](measured_costs.md) | What every transport costs, measured. The source of numbers for the others. |
| [host_binding_styles.md](host_binding_styles.md) | Checked, unchecked, Rust typed. How the host passes values to Wasmtime. Invisible to guests. |
| [abi_choice.md](abi_choice.md) | Core wasm or the Component Model. Which ABI the call crosses. Visible to guests. |
| [guest_toolchain_impact.md](guest_toolchain_impact.md) | What the ABI choice means for people writing gadgets, and in which languages. |
| [options.md](options.md) | The candidate directions, with costs and what is measured against what is guessed. |

## The short version

There are two independent axes. The **binding style** is how our host hands
values to Wasmtime; it changes nothing a guest can observe. The **ABI** is
which type system the call crosses; it decides what guest authors and their
toolchains have to do.

They interact in one place: the Component Model has no fast binding style
reachable from C, only the dynamic one. Core wasm does. That single gap is why
reaching the Component Model's fast path means introducing Rust, while reaching
core wasm's does not.

Costs per scalar callout, measured:

- 126 ns, Component Model through the dynamic C API
- 37 ns, Component Model through Wasmtime's typed Rust API
- 4 ns, core wasm through the C API's unchecked entry points
- 33 ns, Lua, end to end including real work

Sandboxing, trap containment and synced determinism are unaffected by either
axis. They sit below both.

## Open work

Two prototypes would settle what is still estimated: `impl/handoff.md` for a
Rust-typed Component Model host, `impl/todo.md` for an unchecked core-wasm
host. Both are scoped to reproduce the full benchmark table.
