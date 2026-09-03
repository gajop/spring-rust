#!/bin/bash
# Check every Rust crate in the repo against the current native API headers.
#
# The container build compiles C++ only, so a change to the rts/NativeInterface
# headers can leave the Rust bindings uncompilable and still finish green. The
# crate list is discovered rather than hardcoded, so a new crate is covered the
# moment it is added.
set -e -u -o pipefail

cd "$(dirname "$(readlink -f "$0")")/.."

WASM_TARGET=wasm32-unknown-unknown
HAS_WASM_TARGET=false
if rustup target list --installed 2>/dev/null | grep -qx "$WASM_TARGET"; then
  HAS_WASM_TARGET=true
fi

# Crates in the rust/ workspace are checked together below; everything else is
# a standalone manifest that cargo only sees when named explicitly.
WORKSPACE_MEMBERS="$(cargo metadata --manifest-path rust/Cargo.toml --no-deps --format-version 1 \
  | grep -o '"manifest_path":"[^"]*"' | sed 's/.*:"//; s/"$//')"

# Some crates are wasm guests that do not build for the host. Rather than keep a
# second list in sync, fall back to the wasm target when the host check fails.
check_manifest() {
  local manifest="$1" log
  log="$(mktemp)"
  printf '  %-52s ' "$manifest"
  if cargo check --manifest-path "$manifest" --quiet >"$log" 2>&1; then
    echo "ok"
    rm -f "$log"
    return 0
  fi
  if $HAS_WASM_TARGET && cargo check --manifest-path "$manifest" --target "$WASM_TARGET" --quiet >"$log" 2>&1; then
    echo "ok ($WASM_TARGET)"
    rm -f "$log"
    return 0
  fi
  echo "FAILED"
  cat "$log"
  rm -f "$log"
  if ! $HAS_WASM_TARGET; then
    echo "Note: the $WASM_TARGET target is not installed, so wasm guest crates cannot be checked."
    echo "      Install it with: rustup target add $WASM_TARGET"
  fi
  return 1
}

echo "Checking Rust crates against the current native API headers..."
printf '  %-52s ' "rust/Cargo.toml (workspace)"
cargo check --manifest-path rust/Cargo.toml --workspace --quiet
echo "ok"

while IFS= read -r manifest; do
  manifest="${manifest#./}"
  if grep -qxF "$(readlink -f "$manifest")" <<<"$WORKSPACE_MEMBERS"; then
    continue
  fi
  check_manifest "$manifest"
done < <(find rust test -name Cargo.toml -not -path '*/target/*' -not -path 'rust/Cargo.toml' | sort)
