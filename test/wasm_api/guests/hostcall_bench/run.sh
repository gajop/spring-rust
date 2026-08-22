#!/usr/bin/env bash
# Builds the guests and both harnesses, then runs each pinned to one P-core.
# Usage: run.sh [iterations] [samples] [cpu]
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
iterations="${1:-500000}"
samples="${2:-21}"
cpu="${3:-2}"

wat2wasm "$here/core.wat" -o "$here/core.wasm"
cargo build --quiet --manifest-path "$here/guest/Cargo.toml" \
	--target wasm32-unknown-unknown --release
cargo run --quiet --manifest-path "$here/guest/Cargo.toml" --bin componentize -- \
	"$here/guest/target/wasm32-unknown-unknown/release/recoil_hostcall_bench_guest.wasm" \
	"$here/bench_component.wasm"
"$here/host_c/build.sh"
cargo build --quiet --release --manifest-path "$here/host_rust/Cargo.toml"

taskset -c "$cpu" "$here/host_c/bench_hostcall" "$here" "$iterations" "$samples"
echo
taskset -c "$cpu" "$here/host_rust/target/release/recoil-hostcall-bench-host" \
	"$here" "$iterations" "$samples"
