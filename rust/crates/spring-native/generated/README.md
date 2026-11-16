/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

# Native Rust generator snapshot

Normal `spring-native` builds copy these checked-in files and do not require
libclang. Regenerate them deliberately after changing NativeInterface headers:

```text
python3 rust/crates/spring-native-codegen/snapshot_native.py --root .
```

The script builds `spring-native` with `SPRING_NATIVE_REGENERATE=1` (libclang)
and copies the build script's output. The manifest records SHA-256 digests for
every generated file. CI runs it with `--check` (`tools/ci/wasm-api-check.sh`)
and fails if the snapshot differs from a fresh generation.
