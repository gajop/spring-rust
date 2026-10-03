#!/usr/bin/env python3
"""Independent native C / vendored C / Rust ABI probe; Linux maintainer check."""
from pathlib import Path
import subprocess
import tempfile
root=Path(__file__).resolve().parents[3]
probe=Path(__file__).with_name("abi_probe.c")
with tempfile.TemporaryDirectory(prefix="spring-skirmish-abi-") as folder:
    outputs=[]
    for name,include in (("engine",root/"rts"),("vendored",root/"rust/crates/spring-skirmish-ai-sys/include")):
        binary=Path(folder)/name
        subprocess.run(["cc","-std=c99",f"-I{include}",str(probe),"-o",str(binary)],check=True)
        outputs.append(subprocess.check_output([str(binary)],text=True))
    outputs.append(subprocess.check_output(["cargo","run","--quiet","--manifest-path",str(root/"rust/Cargo.toml"),"-p","spring-skirmish-ai-sys","--example","abi_probe"],text=True))
    if outputs[0]!=outputs[1] or outputs[0]!=outputs[2]:
        raise SystemExit("C and Rust ABI layouts differ:\n"+"\n".join(outputs))
    print("Native engine headers, vendored headers and Rust layouts agree (16 size/offset checks)")
