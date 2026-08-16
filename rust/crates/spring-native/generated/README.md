/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

# Native Rust generator snapshot

Normal `spring-native` builds copy these checked-in files and do not require
libclang. Regenerate them deliberately after changing NativeInterface headers:

```text
SPRING_NATIVE_REGENERATE=1 cargo build --manifest-path rust/Cargo.toml -p spring-native --lib
python3 rust/crates/spring-native-codegen/snapshot_native.py --root .
```

The manifest records SHA-256 digests for every generated file. CI should run
the same two commands and fail if the snapshot changes unexpectedly.
