#!/bin/bash
# The "check" job of .github/workflows/wasm.yml, step for step. CI runs this
# script, so a local run and the CI run execute the same commands with the same
# toolchain (rust-toolchain.toml at the repository root).
#
#   tools/ci/wasm-api-check.sh          every step of the job
#   tools/ci/wasm-api-check.sh --fast   formatting and the scoped clippy only
#                                       (what tools/git-hooks/pre-push runs)
set -e -u -o pipefail

cd "$(dirname "$(readlink -f "$0")")/../.."

fast=false
case "${1:-}" in
  --fast) fast=true ;;
  "") ;;
  *) echo "usage: $0 [--fast]" >&2; exit 2 ;;
esac

if [ -z "${LIBCLANG_PATH:-}" ] && [ -d /usr/lib/llvm-18/lib ]; then
  export LIBCLANG_PATH=/usr/lib/llvm-18/lib
fi

# Each step is a collapsible group in the Actions log, and a failure names its step.
current=
step() {
  if [ -n "${GITHUB_ACTIONS:-}" ]; then
    [ -n "$current" ] && echo "::endgroup::"
    echo "::group::$*"
  else
    printf '\n== %s\n' "$*"
  fi
  current="$*"
}
finish() {
  local status=$?
  rm -f "$log"
  [ -n "${GITHUB_ACTIONS:-}" ] && [ -n "$current" ] && echo "::endgroup::"
  if [ "$status" -ne 0 ] && [ -n "$current" ]; then
    if [ -n "${GITHUB_ACTIONS:-}" ]; then
      echo "::error title=Wasm API check::step \"$current\" failed"
    else
      echo "wasm-api-check: step \"$current\" failed" >&2
    fi
  fi
  exit "$status"
}

# cargo with --locked (a check never rewrites a tracked Cargo.lock), failing on
# any warning cargo or rustc prints, cargo's manifest lints such as
# unused_dependencies included.
log="$(mktemp)"
trap finish EXIT
cargo_strict() {
  local sub="$1" status
  shift
  { cargo "$sub" --locked --color never "$@" 2>&1 1>&3 3>&- | tee "$log" >&2; } 3>&1
  status=$?
  [ "$status" -ne 0 ] && return "$status"
  if grep -q '^warning' "$log"; then
    echo "cargo $sub $* printed warnings (above); the check allows none." >&2
    return 1
  fi
}

step "Check Rust formatting"
cargo fmt --manifest-path rust/Cargo.toml --all --check

step "Check scoped SDK Wasm exports"
cargo_strict clippy --manifest-path rust/Cargo.toml -p spring-sdk --target wasm32-unknown-unknown --all-targets -- --deny warnings

if $fast; then
  exit 0
fi

step "Test Rust workspace"
cargo_strict test --manifest-path rust/Cargo.toml --workspace

step "Check generated Wasm artifacts"
python3 rts/wasm/verify_codegen.py

step "Check native generated snapshot"
SPRING_NATIVE_REGENERATE=1 cargo_strict build --manifest-path rust/Cargo.toml -p spring-native --lib
python3 rust/crates/spring-native-codegen/snapshot_native.py --root . --check

# verify_codegen.py already regenerates every Core probe context in a
# temporary tree using fresh metadata; no checkout model.json is needed.
step "Check native parity generated artifacts"
python3 test/native_api_parity/generate_from_spec.py
git diff --exit-code -- \
  test/native_api_parity/native/src/generated_tests.rs \
  test/native_api_parity/fixtures/game.sdd/LuaRules/Utilities/generated_api_tests.lua

step "Build native parity fixture"
cargo_strict build --manifest-path test/native_api_parity/native/Cargo.toml
