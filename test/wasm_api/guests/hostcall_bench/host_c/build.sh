#!/usr/bin/env bash
# Builds the C API floor harness against the same pinned Wasmtime SDK the engine
# links.  WASMTIME_ROOT may point at any provisioned SDK of that version.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo="$(cd "$here/../../../.." && pwd)"
version="$(tr -d '[:space:]' < "$repo/rts/wasm/wasmtime.version")"
root="${WASMTIME_ROOT:-$repo/build-amd64-linux/_deps/wasmtime-$version/wasmtime-v$version-x86_64-linux-c-api}"
if [[ ! -f "$root/include/wasmtime.h" ]]; then
	echo "no Wasmtime SDK under $root; set WASMTIME_ROOT" >&2
	exit 1
fi
g++ -O2 -std=c++17 -o "$here/bench_hostcall" "$here/bench_hostcall.cpp" \
	-I"$root/include" "$root/lib/libwasmtime.a" -lpthread -ldl -lm
