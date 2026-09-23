# Ship Game profiling engine handover (2026-09-24)

## Core-Wasm profiling import ABI

The game's typed Rust SDK calls `spring:profiling.get-timer-micros` with the canonical return-area ABI (`i32,i32->i32`). In this engine checkout, `WasmCoreRegistry.h` still validated the older `->i64` signature. After correcting that descriptor, guest instantiation still failed because `WasmCoreBenchmarkBindings.cpp` registered a second, obsolete `get-timer-micros` import after the normal profiling binding and shadowed it in the Wasmtime linker.

This isolated worktree contains the two engine fixes: align the four profiling descriptors with the generated SDK signatures, and remove the duplicate benchmark registration and its unused callback. A Tracy engine build in `build-amd64-linux-tracy/install` then loaded Ship Game successfully, and a mining scenario completed with Tracy and frame-pointer `perf` captures. These changes are confined to this worktree and have not been applied to the normal engine checkout. The CUS fixture did not exercise this particular profiling import, so it is not sufficient regression coverage for the ABI.

## One load-time SIGSEGV

One of eleven startup attempts in the first focused profile suite failed before gameplay, immediately after `Game::LoadMap`. The engine exited with SIGSEGV while the main thread was in `SDL_PollEvent`, called from `InputHandler::PushEvents` at `rts/System/Input/InputHandler.cpp:26`, via `spring::UnfreezeSpring` and `CLoadScreen::SetLoadMessage` during `CGame::LoadDefs`. The full engine crash log is in Ship Game's `out/profile-focused-fp-20260924/profile-idle-far-render-64x64-n1000-r1/stdout.txt`. A repeat of the same idle/far case completed successfully (`out/idle-far-retry-20260924`). This is a real signal crash, unrelated to LeakSanitizer, but the single stack trace does not establish its cause.

If it recurs, capture a core dump and reproduce with `build-amd64-linux-asan/install` using leak detection disabled (`ASAN_OPTIONS=detect_leaks=0`), while keeping the same startup scenario. Investigate SDL event handling and concurrent load-screen access first; do not attribute the fault to Wasm or the profiler without further evidence.
