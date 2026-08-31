# Rust CUS end-to-end fixture

This fixture boots the real engine against the shared CUS implementation twice:

1. a native module attaches a CUS script, receives `Create`, makes a synchronous named call,
   advances the task scheduler, and observes detach during unload;
2. a native event module orchestrates the same checks against a rules-synced Core-Wasm module.

Run it from the repository root after configuring and building `build-cus`'s `engine-legacy`
target (the debug headless stub asserts while uploading base-content textures):

```bash
python3 test/cus/e2e/run.py
```

The runner creates its game data and logs under a temporary directory. On failure it prints the
directory path and retains it for diagnosis.
