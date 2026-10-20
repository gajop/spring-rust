#!/usr/bin/env python3
"""Compare LocalModel transform code from two source trees, bit for bit.

This deliberately compiles the methods extracted from each tree, rather than a
reimplementation of their invalidation logic. Renderer and collision-volume
storage are inert fixtures; quaternion, matrix, vector and transform arithmetic,
S3DModelPiece::ComposeTransform and the LocalModel update methods are production
code. Use the engine's streflop library and deterministic compiler options.

Example:
  python3 test/validation/local_model_transforms/check.py \
    --baseline /path/to/baseline --candidate . --math-root /path/to/full/engine \
    --streflop /path/to/build/rts/lib/streflop/libstreflop.a --out /tmp/lmp-check
"""

import argparse
import fcntl
import hashlib
import json
from pathlib import Path
import re
import subprocess


def function(source, signature):
    start = source.index(signature)
    begin = source.index("{", start)
    depth = 1
    end = begin + 1
    while depth:
        depth += (source[end] == "{") - (source[end] == "}")
        end += 1
    return source[start:end]


def generate(root, destination):
    models = root / "rts/Rendering/Models"
    header = (models / "LocalModelPiece.hpp").read_text()
    # Expose persisted fields only in the fixture, to emulate creg restore and
    # compare pre-lazy-read state without accidentally updating dirty pieces.
    header = re.sub(r"^#include.*$|^#pragma once$", "", header, flags=re.M)
    header = header.replace("private:", "public:")
    piece = (models / "LocalModelPiece.cpp").read_text()
    piece = piece[piece.index("LocalModelPiece::LocalModelPiece("):]
    piece = piece.replace("void LocalModelPiece::SavePrevModelSpaceTransform()\n{",
                          "void LocalModelPiece::SavePrevModelSpaceTransform()\n{\n++previousSaveCalls;")
    piece = piece.replace("prevModelSpaceTra = tra;",
                          'if (tracePreviousCopies) std::printf("previous-copy %u\\n", GetLModelPieceIndex());\nprevModelSpaceTra = tra;')
    for name in ("Draw", "DrawLOD"):
        piece = piece.replace(function(piece, f"void LocalModelPiece::{name}("), "")
    local = (models / "LocalModel.cpp").read_text()
    local_signatures = [
        "void LocalModel::SetModel(",
        "LocalModelPiece* LocalModel::CreateLocalModelPieces(",
        "void LocalModel::UpdateBoundingVolume(",
    ]
    aggregate = "void LocalModel::SavePrevModelSpaceTransforms(" in local
    if aggregate:
        local_signatures.append("void LocalModel::SavePrevModelSpaceTransforms(")
    sparse = "void LocalModel::SetPrevModelSpaceNeedsUpdate(const LocalModelPiece*" in local
    if sparse:
        local_signatures.append("void LocalModel::SetPrevModelSpaceNeedsUpdate(const LocalModelPiece*")
    local = "\n".join(function(local, signature) for signature in local_signatures)
    local_header = (models / "LocalModel.hpp").read_text()
    has_model_postload = "void PostLoad(" in local_header
    local_header = re.sub(r"^#include.*$|^#pragma once$", "", local_header, flags=re.M)
    local_header = local_header.replace("private:", "public:")
    compose = function((models / "3DModelPiece.cpp").read_text(),
                       "Transform S3DModelPiece::ComposeTransform(")
    compose = compose.replace("{", "{\n++composeCount;", 1)
    fixture = Path(__file__).with_name("trace.cpp").read_text()
    fixture = fixture.replace("// INSERT_PIECE_HEADER", header)
    fixture = fixture.replace("// INSERT_LOCAL_HEADER", local_header)
    fixture = fixture.replace("// INSERT_SAVE", "model.SavePrevModelSpaceTransforms();" if aggregate else
                              "for (auto& piece: model.pieces) piece.SavePrevModelSpaceTransform();")
    fixture = fixture.replace("// INSERT_MODEL_POSTLOAD", "model.PostLoad();" if has_model_postload else "")
    fixture = fixture.replace("// INSERT_INVARIANT", """
    if (!model.prevModelSpaceNeedsUpdate) {
        for (const auto& piece: model.pieces) {
            if (piece.dirty || piece.prevModelSpaceNeedsUpdate) {
                std::fprintf(stderr, "Aggregate pending invariant failed\\n");
                std::abort();
            }
        }
    }
    """ + ("""
    for (const auto& piece: model.pieces) {
        if (!piece.dirty) continue;
        for (const LocalModelPiece* ancestor = &piece; ancestor != nullptr; ancestor = ancestor->parent) {
            if (!ancestor->dirtySubtree) {
                std::fprintf(stderr, "Dirty subtree invariant failed\\n");
                std::abort();
            }
        }
    }
    """ if "bool dirtySubtree" in header else "") + ("""
    if (model.prevModelSpaceNeedsUpdate && !model.prevModelSpaceFullSweep) {
        for (size_t index = 0; index < model.pieces.size(); ++index) {
            const auto& piece = model.pieces[index];
            if ((piece.dirty || piece.prevModelSpaceNeedsUpdate) &&
                (index < model.prevModelSpaceBegin || index > model.prevModelSpaceEnd)) {
                std::fprintf(stderr, "Pending interval invariant failed\\n");
                std::abort();
            }
        }
    }
    """ if sparse else "") if aggregate else "")
    fixture = fixture.replace("// INSERT_IDLE_CHECK", """
    if (previousSaveCalls != beforeIdle) {
        std::fprintf(stderr, "Settled aggregate still visited pieces\\n");
        std::abort();
    }
    """ if aggregate else "")
    fixture = fixture.replace("// INSERT_BOUNDARY_INTERVAL_CHECK", """
    if (previousSaveCalls - boundaryBegin != 1) {
        std::fprintf(stderr, "Maximum compact interval index did not visit exactly one piece\\n");
        std::abort();
    }
    """ if sparse else "")
    tick_source = (root / "rts/Sim/Units/Scripts/UnitScript.cpp").read_text()
    tick_start = tick_source.index("void CUnitScript::TickAllAnims(")
    walk_start = tick_source.index("#if 1", tick_start) + len("#if 1")
    walk_end = tick_source.index("\n#else", walk_start)
    walk = tick_source[walk_start:walk_end]
    walk = walk.replace("auto [lmp, pTra] = q[head];", "auto [lmp, pTra] = q[head];\n++breadthVisits;")
    walk = walk.replace("auto [lmp, pTra] = q.front();", "auto [lmp, pTra] = q.front();\n++breadthVisits;")
    walk = walk.replace("if (lmp->GetDirty()) {", "if (lmp->GetDirty()) {\nstd::printf(\"dirty-visit %u\\n\", lmp->GetLModelPieceIndex());")
    fixture = fixture.replace("// INSERT_BREADTH_FIRST", walk)
    (destination.parent / f"{destination.stem}.animation-kernel.sha256").write_text(
        hashlib.sha256(tick_source[tick_start:walk_start - len("#if 1")].encode()).hexdigest() + "\n")
    fixture = fixture.replace("// INSERT_METHODS", compose + "\n" + piece + "\n" + local)
    destination.write_text(fixture)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline", type=Path, required=True)
    parser.add_argument("--candidate", type=Path, default=Path.cwd())
    parser.add_argument("--math-root", type=Path, required=True)
    parser.add_argument("--streflop", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--compiler", default="g++-13")
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)
    options = ["-std=c++23", "-O3", "-DNDEBUG", "-DUNIT_TEST", "-DBUILDING_AI",
               "-DNOT_USING_CREG", "-DSTREFLOP_SSE", "-march=x86-64", "-mtune=generic",
               "-msse", "-mno-sse3", "-mno-ssse3", "-mno-sse4", "-mno-avx",
               "-mno-fma", "-mno-avx2", "-mfpmath=sse", "-fsingle-precision-constant",
               "-frounding-math", "-mieee-fp", "-fno-strict-aliasing",
               "-ffunction-sections", "-fdata-sections", "-Wl,--gc-sections"]
    outputs = {}
    metrics = {}
    # Same lock used by the controlled profiling runner. One compiler at a time.
    with open("/tmp/shipgame-profiling-1000.lock", "a") as lock:
        fcntl.flock(lock, fcntl.LOCK_SH)
        for name, root in (("baseline", args.baseline), ("candidate", args.candidate)):
            root = root.resolve()
            source = args.out / f"{name}.cpp"
            binary = args.out / name
            generate(root, source)
            math_sources = [root / "rts/System" / f"{name}.cpp" for name in
                            ("Quaternion", "Transform", "Matrix44f", "float3", "float4")]
            command = [args.compiler, *options, f"-I{root / 'rts'}",
                       f"-I{args.math_root / 'rts'}", str(source),
                       *map(str, math_sources), str(args.streflop), "-o", str(binary)]
            subprocess.run(command, check=True)
            result = subprocess.run([str(binary)], check=True, capture_output=True)
            outputs[name] = result.stdout
            metrics[name] = result.stderr.decode().strip()
            (args.out / f"{name}.trace").write_bytes(result.stdout)
            (args.out / f"{name}.layout").write_bytes(subprocess.run(
                [str(binary), "--layout"], check=True, capture_output=True).stdout)
            # Measure the real LocalModel header, including its real collision
            # and Lua-material members, rather than the trace fixture stubs.
            owner_source = args.out / f"{name}-owner-layout.cpp"
            owner_source.write_text('''
#include <cstdio>
#include <cstddef>
#include "Rendering/Models/LocalModelPiece.hpp"
#include "Lua/LuaObjectMaterial.h"
#define private public
#include "Rendering/Models/LocalModel.hpp"
#undef private
int main() {
    std::printf("LocalModel sizeof %zu alignof %zu\\n", sizeof(LocalModel), alignof(LocalModel));
    std::printf("LocalModelPiece sizeof %zu alignof %zu\\n", sizeof(LocalModelPiece), alignof(LocalModelPiece));
#define OFFSET(field) std::printf(#field " %zu\\n", offsetof(LocalModel, field))
    OFFSET(pieces); OFFSET(boundingVolume); OFFSET(luaMaterialData); OFFSET(needsBoundariesRecalc);
}
''')
            owner_binary = args.out / f"{name}-owner-layout"
            subprocess.run([args.compiler, "-std=c++23", "-DBUILDING_AI", "-DNOT_USING_CREG",
                            "-DNOT_USING_STREFLOP", "-Wno-invalidoffsetof",
                            f"-I{root / 'rts'}", f"-I{args.math_root / 'rts'}",
                            str(owner_source), "-o", str(owner_binary)], check=True)
            (args.out / f"{name}.owner-layout").write_bytes(subprocess.run(
                [str(owner_binary)], check=True, capture_output=True).stdout)
    raw_trace_lines = {name: len(trace.splitlines()) for name, trace in outputs.items()}
    raw_trace_sha256 = {name: hashlib.sha256(trace).hexdigest() for name, trace in outputs.items()}
    copy_diagnostic_support = {}
    copy_diagnostic_lines = {}
    for name, root in (("baseline", args.baseline), ("candidate", args.candidate)):
        save = function((root / "rts/Rendering/Models/LocalModelPiece.cpp").read_text(),
                        "void LocalModelPiece::SavePrevModelSpaceTransform(")
        # This is the exact assignment instrumented by generate(), not a cache
        # field-name guess. Original parents use a direct getter assignment.
        copy_diagnostic_support[name] = "prevModelSpaceTra = tra;" in save
        diagnostics = [line for line in outputs[name].splitlines(keepends=True)
                       if line.startswith(b"previous-copy ")]
        for line in diagnostics:
            match = re.fullmatch(rb"previous-copy ([0-9]+)\n", line)
            if match is None or int(match[1]) > 0xffffffff:
                raise SystemExit(f"Malformed optional copy diagnostic in {name}: {line!r}")
        copy_diagnostic_lines[name] = len(diagnostics)
        if copy_diagnostic_support[name] != bool(diagnostics):
            raise SystemExit(f"Copy diagnostic support/output mismatch in {name}")
    compare_copy_diagnostics = all(copy_diagnostic_support.values())
    if copy_diagnostic_support["baseline"] and not copy_diagnostic_support["candidate"]:
        raise SystemExit("Candidate lost the baseline's copy diagnostics; reverse comparison roots if intentional")
    if not compare_copy_diagnostics:
        # Avoiding the original parent's unconditional redundant copies is the
        # optimization. Raw traces above are retained unchanged. Only validated
        # optional operation diagnostics are omitted; every semantic state line,
        # flag/input/bounds/interpolation snapshot and dirty visit stays exact.
        outputs = {name: b"".join(line for line in trace.splitlines(keepends=True)
                    if not line.startswith(b"previous-copy ")) for name, trace in outputs.items()}
        for name, trace in outputs.items():
            (args.out / f"{name}.state-trace").write_bytes(trace)
    if outputs["baseline"] != outputs["candidate"]:
        a, b = outputs["baseline"].splitlines(), outputs["candidate"].splitlines()
        for index, (before, after) in enumerate(zip(a, b)):
            if before != after:
                raise SystemExit(f"Mismatch line {index + 1}:\n{before.decode()}\n{after.decode()}")
        raise SystemExit(f"Trace length mismatch: {len(a)} vs {len(b)}")
    summary = {"match": True, "raw_trace_lines": raw_trace_lines,
               "raw_trace_sha256": raw_trace_sha256,
               "copy_diagnostic_support": copy_diagnostic_support,
               "copy_diagnostic_lines": copy_diagnostic_lines,
               "previous_copy_diagnostics_compared": compare_copy_diagnostics,
               "trace_lines": len(outputs["baseline"].splitlines()),
               "sha256": hashlib.sha256(outputs["baseline"]).hexdigest(),
               "metrics": metrics, "compiler_options": options}
    summary["existing_layout_matches"] = ((args.out / "baseline.layout").read_bytes()
                                          == (args.out / "candidate.layout").read_bytes())
    summary["existing_owner_layout_matches"] = ((args.out / "baseline.owner-layout").read_bytes()
                                                == (args.out / "candidate.owner-layout").read_bytes())
    summary["animation_kernel_source_matches"] = ((args.out / "baseline.animation-kernel.sha256").read_bytes()
                                                  == (args.out / "candidate.animation-kernel.sha256").read_bytes())
    if not summary["existing_layout_matches"] or not summary["existing_owner_layout_matches"]:
        raise SystemExit("Pre-existing member layout changed")
    if not summary["animation_kernel_source_matches"]:
        raise SystemExit("Animation kernel source changed before hierarchy traversal")
    (args.out / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
