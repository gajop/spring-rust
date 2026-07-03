# ENGINE-001: blank-map archive checksum assert in debug

## Bug

In a debug or ASAN build, starting the engine with Spring's generated blank-map
path can abort while scanning the generated map archive. The failing path is in
`CArchiveScanner::GetArchiveChecksum`: when `IArchive::FileInfo()` reports an
unhashable file entry, the engine already erases the stale cache entry and
returns `false`, but first hits a hard `assert(false)`.

This is independent of the native binding module. The native parity harness
found it because the harness defaults to generated blank maps, but `--mode lua`
is sufficient to reproduce it.

## Reproduction

Build a debug ASAN engine:

```bash
docker-build-v2/build.sh --configure linux \
  -DUSE_ASAN=ON \
  -DCMAKE_BUILD_TYPE=DEBUG \
  -DMATH_LIBRARY:FILEPATH=/usr/lib/x86_64-linux-gnu/libm.so
docker-build-v2/build.sh --compile -j 8 linux
```

Run the Lua-only harness without passing `--map`, so it uses `InitBlank=1` and
a generated blank map name:

```bash
ASAN_OPTIONS=detect_leaks=0 \
python3 test/native_api_parity/run_harness.py \
  --spring build-amd64-linux/install/spring \
  --spring-headless build-amd64-linux/install/spring-headless \
  --enable-rendering-tests \
  --mode lua \
  --cases 1 \
  --blank-map-seed 197116323
```

Before the fix, this can abort during archive scanning with the debug assertion
inside `CArchiveScanner::GetArchiveChecksum`, before the harness reaches the
RmlUi checks.

## Fix

Remove the hard assertion from the recoverable `FileInfo()` failure branch. The
function still invalidates the cached file info and returns `false`, allowing
the caller to handle the missing checksum without crashing a debug build.
