#!/usr/bin/env python3
# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
"""Small native API regression with meaningful Lua values and GL readback."""
from __future__ import annotations

import argparse
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile

ROOT=Path(__file__).resolve().parents[2]
HERE=Path(__file__).resolve().parent
sys.path.insert(0,str(ROOT/'test/native_api_parity'))
from run_harness import BASE_CONTENT, ENGINE_INSTALL, SOURCE_CONTENT, GAME_FIXTURE, blank_map_name, prepare_datadir, write_script


def build_module(output):
    # Generate exact-signature unhandled callins for the loader. Only callouts
    # exercised by native.cpp are under test; these event exports preserve the
    # host's supplied defaults and never mutate simulation state.
    header=(ROOT/'rts/NativeInterface/NativeInterfaceEventClient.h').read_text()
    signatures=dict(re.findall(r'using (\w+)FuncPtr\s*=\s*void\(\*\)\(([^;]+)\);',header))
    aliases=dict(re.findall(r'using (\w+)FuncPtr\s*=\s*(\w+)FuncPtr;',header))
    loader=(ROOT/'rts/NativeInterface/NativeInterfaceEventClient.cpp').read_text()
    names=set(re.findall(r'LOAD_SYMBOL\((\w+)\)',loader))-{'SymbolName','InitializeNativeModule','HandleLuaCall'}
    exports=['#include "NativeInterface/NativeInterface.h"','extern "C" {']
    for name in sorted(names):
        key=name
        while key in aliases: key=aliases[key]
        args=[arg.strip() for arg in signatures[key].split(',')]
        args[-1]+=' result'
        exports.append(f'void {name}({", ".join(args)}) {{ result->error=nullptr; }}')
    exports.append('}')
    generated=output/'unhandled_callins.cpp'
    generated.write_text('\n'.join(exports)+'\n')
    module=output/'native_audit.so'
    subprocess.run(['c++','-std=c++20','-O1','-fPIC','-shared','-I'+str(ROOT/'rts'),
        str(HERE/'native.cpp'),str(generated),'-ljsoncpp','-lSDL2','-o',str(module)],check=True)
    return module


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--spring',type=Path,default=ENGINE_INSTALL/'spring')
    parser.add_argument('--output',type=Path,required=True)
    options=parser.parse_args()
    output=options.output.resolve();output.mkdir(parents=True,exist_ok=True)
    module=build_module(output)
    args=argparse.Namespace(blank_map_x=8,blank_map_y=8,blank_map_height=96,blank_map_terrain='blank',
        blank_map_seed=424242,cases=1,tests=None,test_prefix=None,enable_rendering_tests=True,
        process_test=None,spring=options.spring.resolve(),spring_headless=options.spring.resolve())
    timed_out=False
    with tempfile.TemporaryDirectory(prefix='native-stub-audit-') as directory:
        datadir,_=prepare_datadir(Path(directory),None)
        game=datadir/'games/native_api_parity.sdd'
        if game.is_symlink():game.unlink()
        else:shutil.rmtree(game)
        shutil.copytree(GAME_FIXTURE,game)
        os.chmod(game/'modinfo.lua',0o600)
        for path in (game/'LuaRules/Gadgets').glob('*.lua'):path.unlink()
        shutil.copy2(HERE/'setup.lua',game/'LuaRules/Gadgets/audit.lua')
        shutil.copy2(HERE/'checks.lua',game/'LuaUI/main.lua')
        shutil.copy2(game/'units/native_api_test_unit.lua',game/'gamedata/audit_base_defs.lua')
        shutil.copy2(HERE/'defs.lua',game/'units/native_api_test_unit.lua')
        (game/'weapons').mkdir(exist_ok=True)
        (game/'weapons/no_weapon.lua').write_text("return {noweapon={name='No explosion',weaponType='Cannon',range=1,reloadtime=1,weaponVelocity=1,damage={default=0}}}\n")
        (game/'gamedata/movedefs.lua').write_text("return {{name='kbot1',dont_remove=true,footprintX=1,footprintZ=1,maxWaterDepth=20,maxSlope=36},{name='hover1',dont_remove=true,footprintX=1,footprintZ=1,maxSlope=36}}\n")
        script=output/'script.txt'
        write_script(script,blank_map_name(args,'lua'),'native_audit','lua',True,args,424242,'ui',None)
        text=script.read_text().replace('        Name=NativeApiParity;','        Name=NativeApiParity;\n        audit_key=player-nonempty;',1)
        text=text.replace('TeamLeader=0;','TeamLeader=0;\n        audit_key=team-nonempty;',1)
        script.write_text(text)
        settings=output/'write/springsettings.cfg';settings.parent.mkdir(exist_ok=True)
        settings.write_text('Fullscreen=0\nXResolutionWindowed=640\nYResolutionWindowed=480\nVSync=1\n'
            'Shadows=0\nSound=0\nWorkerThreadCount=1\nUseFontConfigLib=0\nUseLuaMemPools=0\nTextureMemPoolSize=0\n')
        env=os.environ.copy()
        env.update(SPRING_DATADIR=os.pathsep.join(map(str,[datadir,BASE_CONTENT,ENGINE_INSTALL,SOURCE_CONTENT])),
            SPRING_ISOLATED=str(datadir),SPRING_NATIVE_MODULE=str(module))
        with (output/'spring.log').open('wb') as log:
            process=subprocess.Popen([str(args.spring),'--window','--nocolor','--config',str(settings),
                '--write-dir',str(settings.parent),str(script)],cwd=ROOT,env=env,stdout=log,stderr=subprocess.STDOUT)
            try:code=process.wait(timeout=30)
            except (KeyboardInterrupt,subprocess.TimeoutExpired) as error:
                process.terminate()
                try:process.wait(timeout=5)
                except subprocess.TimeoutExpired:process.kill();process.wait()
                if isinstance(error, KeyboardInterrupt): raise
                code=process.returncode
                timed_out=True
    log=(output/'spring.log').read_text(errors='replace')
    if timed_out or code or 'NO_STUBS_PASS' not in log or 'NO_STUBS_FAIL' in log:
        # The runner's working directory is discarded after CI. Keep the
        # engine failure in the job log, not just a path on that lost runner.
        print("\n".join(log.splitlines()[-120:]), file=sys.stderr)
        reason='timeout' if timed_out else f'exit {code}'
        raise SystemExit(f'Native audit failed ({reason}): {output/"spring.log"}')
    assert re.search(r'\[NativeStubAudit\].*Warning:.*audit section/level 100%',log), 'Log section/level lost'
    print('PASS: non-default native API values, movement rejection/options, shield selection and framebuffer readback')


if __name__=='__main__':main()
