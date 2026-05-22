"""
Entry point for the NativeInterface code generator.

Parses Lua*.cpp source files via tree-sitter and emits C header/stub
implementations using the query/result pattern of rts/NativeInterface/api/.

Usage:
    uv run python generate.py <LuaFile.cpp> [LuaFile2.cpp ...]
        --api-dir  PATH     path to existing NativeInterface/api/ (default: auto)
        --output-dir PATH   where to write generated files     (default: ./generated/)
        --module-name NAME  override module name (single input only)
        --only FUNC,...     comma-separated list: only emit these functions
        --list              dry-run: print detected functions, don't write
        --verbose           show skipped functions and reasons
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

from lua_parser import LuaFunction, parse_file
from codegen import generate_header, generate_impl


# ── Helpers ───────────────────────────────────────────────────────────────────

def _scan_existing(api_dir: Path) -> set[str]:
    """Return names of functions already declared in NativeInterface headers."""
    existing: set[str] = set()
    for h in api_dir.glob("*.h"):
        text = h.read_text(encoding="utf-8", errors="replace")
        for m in re.finditer(r"void\s+\(\*(\w+)\)\s*\(", text):
            existing.add(m.group(1))
    return existing


def _module_name(path: Path) -> str:
    stem = path.stem          # e.g. LuaSyncedRead
    if stem.startswith("Lua"):
        stem = stem[3:]       # SyncedRead
    return stem


def _auto_api_dir() -> Path | None:
    here = Path(__file__).parent
    candidate = here.parent / "api"
    return candidate if candidate.is_dir() else None


# ── Main ──────────────────────────────────────────────────────────────────────

def main() -> int:
    ap = argparse.ArgumentParser(
        description="Generate NativeInterface C stubs from Lua*.cpp AST.",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    ap.add_argument("sources", nargs="+", metavar="LuaFile.cpp")
    ap.add_argument("--api-dir",     metavar="PATH", help="path to NativeInterface/api/")
    ap.add_argument("--output-dir",  metavar="PATH", default="generated")
    ap.add_argument("--module-name", metavar="NAME",
                    help="override module name (single source file only)")
    ap.add_argument("--only",        metavar="FUNCS",
                    help="comma-separated list of function names to emit")
    ap.add_argument("--list",    action="store_true", help="dry-run: list functions")
    ap.add_argument("--verbose", action="store_true", help="show skipped functions")
    args = ap.parse_args()

    # Resolve api dir
    if args.api_dir:
        api_dir = Path(args.api_dir)
    else:
        api_dir = _auto_api_dir()
        if api_dir is None:
            print("ERROR: cannot find NativeInterface/api/; use --api-dir", file=sys.stderr)
            return 1

    existing = _scan_existing(api_dir)
    if args.verbose:
        print(f"Found {len(existing)} already-implemented function(s) in {api_dir}")

    if args.module_name and len(args.sources) > 1:
        print("ERROR: --module-name only valid with a single source file", file=sys.stderr)
        return 1

    only_set: set[str] | None = None
    if args.only:
        only_set = {s.strip() for s in args.only.split(",")}

    out_dir = Path(args.output_dir)
    if not args.list:
        out_dir.mkdir(parents=True, exist_ok=True)

    total_written = 0

    for src_str in args.sources:
        src_path = Path(src_str)
        if not src_path.exists():
            print(f"ERROR: {src_path} not found", file=sys.stderr)
            return 1

        module = args.module_name if args.module_name else _module_name(src_path)
        all_fns = parse_file(src_path)

        selected: list[LuaFunction] = []
        n_existing = 0
        n_skipped  = 0

        for fn in all_fns:
            if fn.name in existing:
                n_existing += 1
                if args.verbose:
                    print(f"  skip (exists)   {fn.name}")
                continue
            if only_set is not None and fn.name not in only_set:
                if args.verbose:
                    print(f"  skip (not in --only) {fn.name}")
                continue
            if fn.skip_reason:
                n_skipped += 1
                if args.verbose:
                    print(f"  skip (variadic) {fn.name}: {fn.skip_reason}")
                continue
            if not fn.returns and not fn.params and not fn.is_registered:
                n_skipped += 1
                if args.verbose:
                    print(f"  skip (unregistered, no sig) {fn.name}")
                continue
            selected.append(fn)

        print(f"\n{src_path.name}  →  module '{module}'")
        print(f"  total functions:   {len(all_fns)}")
        print(f"  already ported:    {n_existing}")
        print(f"  skipped (ambig):   {n_skipped}")
        print(f"  to generate:       {len(selected)}")

        if args.list:
            for fn in selected:
                def _fmt_p(p):
                    mark = "?" if not p.name_inferred else ""
                    return f"{p.name}{mark}:{p.c_type}"
                def _fmt_r(r):
                    mark = "?" if not r.name_inferred else ""
                    return f"{r.name}{mark}:{r.c_type}"
                params = ", ".join(_fmt_p(p) for p in fn.params)
                rets   = ", ".join(_fmt_r(r) for r in fn.returns)
                flags = []
                if fn.needs_review:
                    flags.append("[needs-review]")
                if fn.is_registered and not fn.params and not fn.returns:
                    flags.append("[void-void]")
                print(f"    {fn.name}({params}) → ({rets}){' '.join(flags)}")
            continue

        if not selected:
            print("  nothing to write.")
            continue

        header_path = out_dir / f"{module}.h"
        impl_path   = out_dir / f"{module}.cpp"

        header_path.write_text(generate_header(module, selected, src_path.name))
        impl_path.write_text(generate_impl(module, selected, src_path.name))

        print(f"  written: {header_path}")
        print(f"  written: {impl_path}")
        total_written += len(selected)

    print(f"\nTotal functions generated: {total_written}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
