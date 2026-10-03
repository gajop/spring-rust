# spring-skirmish-ai-sys

Checked-in bindings for Spring/Recoil's C Skirmish AI events, commands and callback table. No dependencies or build script; consumers need neither engine sources nor bindgen/libclang.

The matching headers are shipped in `include/`. Bindings use `repr(C)` and `extern "system"` to match the interface's platform calling convention. Calls, pointer lifetimes, visibility restrictions and ABI compatibility are unsafe responsibilities. Prefer the `spring-skirmish-ai` façade for dispatch and ordinary observations/orders.

Current validation targets Linux x86-64. Interface version `C/0.1` is not a guarantee that older engine callback tables have identical layouts. Bindgen layout tests describe the vendored 64-bit headers; do not claim untested architectures or arbitrary engine versions are supported.

Maintainer generator: `rust/tools/skirmish-ai-bindings` in the engine repository. The crate package includes all referenced headers and its GPL-2.0-or-later license. It has no dependency on Ship Game or an engine checkout at build time.
