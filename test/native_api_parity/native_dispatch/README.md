# Paired native SDK benchmarks

The driver loads two or more release DLLs in one real engine instance. Workers
compile the same handlers through `spring-native` or `spring-sdk`. Samples
alternate library order on the same engine thread. No mocked API tables or
locally rebuilt engine are required.

From the engine repository root:

```sh
export LIBCLANG_PATH=/usr/lib/x86_64-linux-gnu
mkdir -p .cache/native-dispatch
cargo build --release --manifest-path test/native_api_parity/native_dispatch/Cargo.toml \
  -p native-dispatch-worker --features legacy
cp test/native_api_parity/native_dispatch/target/release/libnative_dispatch_worker.so \
  .cache/native-dispatch/legacy.so
cargo build --release --manifest-path test/native_api_parity/native_dispatch/Cargo.toml \
  -p native-dispatch-worker --features borrowed-command -p native-dispatch-driver
cp test/native_api_parity/native_dispatch/target/release/libnative_dispatch_worker.so \
  .cache/native-dispatch/sdk.so
python3 test/native_api_parity/run_native_dispatch.py \
  --worker legacy=.cache/native-dispatch/legacy.so \
  --worker sdk=.cache/native-dispatch/sdk.so \
  --cpu 14 --repeats 101 --out /tmp/native-dispatch-warm
```

Choose an available CPU on your machine. The runner applies affinity on the
actual engine callback thread, after engine initialization. Output directories
must be fresh. To measure forced eviction, repeat with `--cache cold --repeats 65`
and a different output directory. Use `--spring` for an existing engine binary.

The 14 callback cases time exported C ABI entry, handler and result validation;
they exclude the C++ event dispatcher. Commands use 0/4/64/1024 parameters,
strings 0/16/1024 bytes. Seven callout cases batch real engine operations inside
one callback: scalar, vector, string plus scalar lookup, nonempty command list,
1,000 unit IDs, spatial query and numeric mutation. The fixture already has a
move command with three parameters. Results and cross-worker checksums must
agree. Input/output copies remain part of the measured public API behavior.
Legacy command-list results contain engine pointers; SDK results own nested
parameters. The driver uses the legacy crate only to host the harness; all timed
SDK calls execute inside the separately compiled SDK worker.

Warm samples use 20,000 callback entries. Batched callouts instead use one entry
containing 10,000 operations, or 1,000 for unit ID lists. `operationsPerEntry`
and `iterations` record those counts. Cold samples evict 64 MiB before each
entry, outside the timer. Batched callouts are not cold for every operation.
Cold entry includes clock overhead. No FPS or real frame scheduling is measured.

For a historical SDK comparison, create a separate worktree at the desired
commit and copy this fixture there, excluding `target/`. Build its worker with
`--features unified` for SDK revisions before `CallinCommand` was introduced,
or `--features borrowed-command` for current revisions. Copy the resulting DLL
to a distinct filename and add another `--worker label=path`. The recorded
comparison used SDK commit `4c1202909c`, identical Rust compiler, release fat LTO
and matched handler source. Keep all workers loaded in the same process.

Run at least three processes and rotate initial worker order. Preserve every
run: frequency changes and shared cache contention still affect a busy host.
The JSON retains raw samples, artifact hashes, affinity and cache mode.
