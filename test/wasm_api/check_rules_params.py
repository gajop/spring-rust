#!/usr/bin/env python3
# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
"""Run typed rules-param getters and the allocation-free game getter in-engine."""
from __future__ import annotations

import argparse
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "test/native_api_parity"))
from run_harness import blank_map_name, prepare_datadir, run_spring, write_script


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--spring", type=Path, default=ROOT / "build-amd64-linux/install/spring-headless")
    parser.add_argument("--output", type=Path, default=ROOT / "test/native_api_parity/out/rules-params")
    options = parser.parse_args()
    guest = ROOT / "test/wasm_api/guests/rules_params_guest"
    subprocess.run(["cargo", "build", "--manifest-path", str(guest / "Cargo.toml"),
                    "--locked", "--release", "--target", "wasm32-unknown-unknown"], cwd=guest, check=True)
    wasm = guest / "target/wasm32-unknown-unknown/release/recoil_rules_params_guest.wasm"
    output = options.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    args = argparse.Namespace(blank_map_x=10, blank_map_y=8, blank_map_height=96,
        blank_map_seed=424242, cases=1, tests=None, test_prefix=None,
        enable_rendering_tests=False, process_test=None, timeout=60,
        spring=options.spring.resolve(), spring_headless=options.spring.resolve())
    with tempfile.TemporaryDirectory(prefix="rules-params-") as directory:
        datadir, _ = prepare_datadir(Path(directory), None, wasm)
        gadgets = datadir / "games/native_api_parity.sdd/LuaRules/Gadgets"
        for path in gadgets.glob("*.lua"):
            path.unlink()
        (gadgets / "rules_params.lua").write_text("""function gadget:GetInfo()
    return {name = "Rules params regression", layer = 0, enabled = true}
end
if gadgetHandler:IsSyncedCode() then
    function gadget:Initialize()
        local unit = assert(Spring.CreateUnit("native_api_test_unit", 128, 96, 128, 0, 0))
        local feature = assert(Spring.CreateFeature("native_api_test_feature", 256, 96, 256))
        Spring.SetGameRulesParam("fixture_unit", unit, {public = true})
        Spring.SetGameRulesParam("fixture_feature", feature, {public = true})
    end
else
    function gadget:GameFrame(frame)
        if frame >= 3 then Spring.SendCommands("quitforce") end
    end
end
""", encoding="utf-8")
        script = output / "script.txt"
        write_script(script, blank_map_name(args, "wasm"), "native_api_parity",
                     "wasm", True, args, 424242, "synced_gadget", None)
        code = run_spring(args, datadir, script, output, "wasm")
    log = (output / "spring.log").read_text(encoding="utf-8", errors="replace")
    if code != 0 or "RULES_PARAMS_TYPED_PASS" not in log or "Wasm trap" in log:
        raise SystemExit(f"rules-param regression failed (exit {code}); see {output / 'spring.log'}")
    print("PASS: five scopes; bool/float/empty and long UTF-8 strings; missing/hidden params; game float errors")


if __name__ == "__main__":
    main()
