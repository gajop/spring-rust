#!/usr/bin/env python3
"""Embed a checked-in Component Model fixture as a C++ byte array."""

# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

import argparse
from pathlib import Path


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("input", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("symbol")
    parser.add_argument("description")
    args = parser.parse_args()

    data = args.input.read_bytes()
    lines = [
        "/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */",
        "",
        "#pragma once",
        "",
        "#include <cstddef>",
        "#include <cstdint>",
        "",
        f"// Generated from {args.description}.",
        "namespace wasm_component_fixture {",
        f"inline constexpr std::uint8_t {args.symbol}[] = {{",
    ]
    for offset in range(0, len(data), 12):
        chunk = data[offset : offset + 12]
        lines.append("  " + ", ".join(f"0x{byte:02x}" for byte in chunk) + ",")
    lines.extend(
        [
            "};",
            f"inline constexpr std::size_t {args.symbol}Size = sizeof({args.symbol});",
            "} // namespace wasm_component_fixture",
            "",
        ]
    )
    args.output.write_text("\n".join(lines), encoding="utf-8")


if __name__ == "__main__":
    main()
