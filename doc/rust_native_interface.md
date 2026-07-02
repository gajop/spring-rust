# Rust NativeInterface Generation Plan

This document captures the architecture for creating a fully automatic Rust
interface over `rts/NativeInterface/NativeInterface.h`.

## Goals

1. **Raw ABI Coverage** – Bind all C++ symbols behind `NativeInterface` with a
   reliable Rust crate that mirrors the exact memory layout.
2. **Structured Metadata** – Parse the headers once and build a rich metadata
   representation (queries, results, APIs, enums) that downstream generators can
   reuse.
3. **Ergonomic API Layer** – Emit a safe Rust API where each function takes
   strongly typed parameters (flattened from the `*Query` structs) and returns
   idiomatic `Result<T, Error>` values without exposing pointers.
4. **Deterministic Codegen** – Regenerate the bindings in `build.rs` so they stay
   synchronized with upstream header changes.

## Crate Layout

```
rust/
├── Cargo.toml                 # Workspace root
├── crates/
│   ├── spring-native-sys/     # ✅ Raw bindgen output (already implemented)
│   ├── spring-native-codegen/ # ✅ libclang-based metadata & code generator
│   └── spring-native/         # ✅ Safe façade that consumes generated metadata
```

### `spring-native-sys`
- Uses `bindgen` against `rts/NativeInterface/NativeInterface.h` with C++17
  semantics.
- Emits every `*Api`, `*Query`, `*Result`, and the top-level `NativeInterface`
  struct with correct `#[repr(C)]` layout.
- Consumers can opt-in to low-level access or use it indirectly through the
  higher-level crate.

### `spring-native-codegen`
- Library exposes `pub fn generate_units_query(header: &Path, include_dirs: &[PathBuf])`.
- Internally uses the `clang` crate (libclang via runtime loading) to parse
  `rts/NativeInterface/api/UnitsQuery.h` once and build a structured `ApiSpec`
  (struct fields, pointer relationships, API function signatures).
- The generator flattens query structs into ergonomic Rust parameters
  (automatically merging pointer+count patterns into slices, escaping keywords,
  and keeping naming conventions consistent) and renders the corresponding
  `impl UnitsQuery<'_>` block as Rust source.
- Because it runs inside `spring-native`'s build script, the emitted code always
  matches the current header set without checking generated files into git.

### `spring-native`
- ✅ Wraps the raw bindings with safe entry points (`NativeInterfaceRef`,
  generated `UnitsQuery`/`UnitsInfo` methods, helper traits like
  `RectangleQueryExt`).
- Build script runs `spring-native-codegen`, so adding new headers to the safe
  layer is just `include!(OUT_DIR/...)`.
- Includes a runnable example (`cargo run -p spring-native --example mock_units_query`)
  that wires a mock `UnitsQueryApi` into a `NativeInterface` struct and proves
  the safe wrappers link and execute without any Spring binaries present.

## Flattening Query Parameters

Every call follows the `[Query*] → [Result*]` convention. We can map it to Rust
signatures algorithmically:

1. **Parameter extraction**
   - For each `FooQuery` struct, map every field to a Rust parameter.
   - `const` qualifiers drive whether the parameter is `&T`, `&[T]`, or owned by
     value.
   - Arrays follow the `ptr + count` idiom. If a query contains
     `{ const int32_t* unitIDs; uint32_t count; }`, the generator produces a
     single parameter like `unit_ids: &[i32]` (both fields removed from the
     struct packing step).
   - Nested structs (e.g. `RectangleQuery rect;`) become their direct Rust
     counterparts via `spring-native-sys` types, so callers pass plain values.

2. **Return value synthesis**
   - If the result struct contains fields beyond `const Error*`, we map them to
     the `Ok` payload.
   - Scalar fields → return that scalar (e.g. `Float3 centroid;` → `Result<Float3, Error>`).
   - Pointer + count → copy into a `Vec<T>` to avoid exposing raw pointers. The
     generator will automatically call `memory::Free*` when the API indicates the
     buffer is heap allocated; otherwise it clones the scratch buffer contents.
   - For `bool` and `enum` outputs we translate to Rust equivalents.

3. **Naming**
   - `GetUnitsInRectangle` → `get_units_in_rectangle` (standard snake_case).
   - Query field names become parameter names. When merging pointer+count we use
     plural forms (`units`, `unit_ids`).

4. **Implementation sketch**
   ```rust
   pub fn get_units_in_rectangle(
       &self,
       rect: RectangleQuery,
       filter: UnitFilterParams,
   ) -> Result<Vec<i32>, Error> {
       unsafe {
           let query = GetUnitsInRectangleQuery { rect, filter };
           let mut result = MaybeUninit::<GetUnitsInRectangleResult>::zeroed();
           (self.raw.GetUnitsInRectangle.unwrap())(&query, result.as_mut_ptr());
           let result = result.assume_init();
           Error::from_ptr(result.error)?;
           Ok(slice::from_raw_parts(result.units, result.count as usize).to_vec())
       }
   }
   ```
   - `Error::from_ptr` is a helper that constructs a Rust error type (by copying
     the message into an owned `String`).
   - For functions returning `bool` or `int`, `Ok(result.value)` is used.
   - When a buffer must be freed explicitly, metadata from the generator marks
     which `MemoryApi::Free*` function to call.

5. **Zero-copy opportunities**
   - Once the basic version works (which copies slices into `Vec`), we can allow
     advanced users to opt into `SliceHandle<'a, T>` wrappers that carry the
     lifetime of the scratch buffer. The generator would emit both `*_raw()` and
     safe variants. This is optional and can come later.

## Implementation Roadmap

1. ✅ `spring-native-sys`: ship raw bindgen crate.
2. ✅ Create `spring-native-codegen` with libclang parsing + emitter for
   `UnitsQuery.h`.
3. ✅ Plug the code generator into the `spring-native` crate and emit Rust
   wrappers for `UnitsQueryApi` and `UnitsInfoApi`.
4. ✅ Expand coverage to all API headers, improve heuristics (array detection,
   ownership markers, doc comments → Rust docstrings).
5. ⏳ Add integration tests that link against Spring, call the Rust façade, and
   verify data round-trips.

## Generated Safe APIs

All 32 API modules are now fully implemented with automatic code generation:

- ✅ `UnitsQuery` – full coverage (arrays flattened into `Vec`, scalar returns
  copied by value).
- ✅ `UnitsInfo` – full coverage including string conversions (`Option<String>`,
  tuple returns for multi-field results).
- ✅ `Teams`, `UnitsWeapons`, `UnitsCommands`, `UnitsPieces` – unit and team
  management APIs.
- ✅ `Features`, `Projectiles`, `Los` – game object queries and LOS checks.
- ✅ `UnitDefs`, `FeatureDefs`, `WeaponDefs` – definition data access.
- ✅ `Game`, `Terrain`, `Player` – core game state APIs.
- ✅ `MathExtra`, `MetalMap`, `PathFinder`, `RulesParams` – utility and
  pathfinding APIs.
- ✅ `MoveCtrl`, `SyncedCtrl` – unit movement and synced game control.
- ✅ `Camera`, `Input`, `Display`, `Selection` – UI and rendering APIs.
- ✅ `Vfs`, `Sound`, `Messages`, `Config` – I/O and configuration.
- ✅ `Tracing`, `Utils`, `Memory` – debugging, utilities, and memory management.

## Detailed Execution Plan (for automation/offloading)

1. **Add New Modules**
   - Create `src/<module>.rs` mirroring `units_query.rs`/`units_info.rs`.
   - Update `build.rs` to call `spring_native_codegen::generate_<module>()`
     with the correct header path and wrapper struct name.
   - `include!(concat!(env!(\"OUT_DIR\"), \"/<module>_generated.rs\"));` inside
     the module, re-export it from `lib.rs`, and add an accessor to
     `NativeInterfaceRef`.

2. **Generator Patterns**
   - Detect pointer+count fields (`const T* data; uint32_t count;`) → emit
     `&[T]` parameters (`.as_ptr()`, `.len() as u32`), and convert pointer
     results into `Vec<T>` via `slice::from_raw_parts`.
   - Convert `const char*` results into `Option<String>` using `CStr`.
   - For multiple result fields, emit tuples
     (`(sys::UnitShieldState, bool)`).
   - Escape Rust keywords (`box` → `r#box`, struct fields → `box_`).

3. **Ownership Handling**
   - Extend the generator to mark which result buffers need freeing. Emit calls
     into a future `Memory` helper that wraps `MemoryApi::Free*`.
   - Default behavior: copy scratch buffers into `Vec` so callers own the data.

4. **Testing & Examples**
   - Add module-specific mocks/examples under `examples/`.
   - Create integration tests linking against mocked `NativeInterface`
     instances to exercise each generated API.
   - Longer-term: optional feature to run tests against the real engine build.

5. **Documentation & Distribution**
   - Update this doc as each module is generated (tick list in “Generated Safe
     APIs”).
   - Document how plugin authors use `spring-native` (entry point boilerplate,
     linking instructions).
   - When coverage is broad, publish `spring-native`/`spring-native-sys` on
     crates.io; keep `spring-native-codegen` internal unless external reuse is
     desired.

This setup gives us both automatic generation and a clear place to replace the
query structs with direct Rust parameters.
